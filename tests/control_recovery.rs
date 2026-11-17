/*
 * Matter-Base — Encrypted outbox reconciliation regressions.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Result};
use matter_base::security::key_manager::KeyManager;
use matter_base::{
    encryption::{self, EncryptedEnvelope},
    EncryptedControlOutbox, EncryptedEnvelopeReader, EncryptedEnvelopeSink,
};
use std::sync::Mutex;

fn manager() -> KeyManager {
    let mut keys = KeyManager::new();
    keys.rotate("key-a".to_owned(), [0x5A_u8; 32]).unwrap();
    keys
}

#[derive(Default)]
struct Remote {
    record: Mutex<Option<(EncryptedEnvelope, String)>>,
    lost_ack: bool,
}

impl EncryptedEnvelopeSink for Remote {
    fn put(&self, envelope: &EncryptedEnvelope, expected: Option<&str>) -> Result<String> {
        let mut guard = self.record.lock().unwrap();
        if guard.as_ref().map(|(_, r)| r.as_str()) != expected {
            bail!("remote compare-and-swap refused");
        }
        let revision = match &*guard {
            Some((_, old)) if old == "rev-1" => "rev-2",
            _ => "rev-1",
        };
        *guard = Some((envelope.clone(), revision.to_owned()));
        if self.lost_ack {
            bail!("acknowledgement was lost");
        }
        Ok(revision.to_owned())
    }
}

impl EncryptedEnvelopeReader for Remote {
    fn get(&self, _scope: &str, _record_type: &str) -> Result<Option<(EncryptedEnvelope, String)>> {
        Ok(self.record.lock().unwrap().clone())
    }
}

#[test]
fn lost_acknowledgment_reconciles_exact_ciphertext_without_resending() {
    let outbox = EncryptedControlOutbox::in_memory().unwrap();
    let keys = manager();
    assert!(outbox
        .enqueue(&keys, "bot", "roles", b"sensitive-role-members")
        .unwrap());
    let remote = Remote {
        record: Mutex::new(None),
        lost_ack: true,
    };
    assert_eq!(
        outbox
            .flush(&remote, "bot", "roles")
            .unwrap_err()
            .to_string(),
        "Encrypted envelope publication failed"
    );
    assert!(!outbox.status("bot", "roles").unwrap().unwrap().synchronized);
    assert!(outbox.reconcile(&remote, "bot", "roles").unwrap());
    assert!(!outbox.reconcile(&remote, "bot", "roles").unwrap());
    assert!(!outbox.flush(&remote, "bot", "roles").unwrap());
    assert_eq!(
        outbox
            .status("bot", "roles")
            .unwrap()
            .unwrap()
            .remote_revision,
        Some("rev-1".into())
    );
    let stored = remote.record.lock().unwrap().clone().unwrap().0;
    assert_eq!(
        encryption::decrypt_with_manager(&stored, &keys).unwrap(),
        b"sensitive-role-members"
    );
}

#[test]
fn divergence_never_marks_unsent_record_published() {
    let outbox = EncryptedControlOutbox::in_memory().unwrap();
    let keys = manager();
    outbox
        .enqueue(&keys, "bot", "roles", b"expected-payload")
        .unwrap();
    let replacement = encryption::encrypt(
        b"foreign-payload",
        keys.get_active().unwrap().1,
        "foreign-1",
        "bot",
        "roles",
        "key-a",
        "2026-11-17T00:00:00Z",
    )
    .unwrap();
    let remote = Remote {
        record: Mutex::new(Some((replacement, "rev-other".to_owned()))),
        lost_ack: false,
    };
    assert_eq!(
        outbox
            .reconcile(&remote, "bot", "roles")
            .unwrap_err()
            .to_string(),
        "Remote encrypted record diverged"
    );
    assert!(!outbox.status("bot", "roles").unwrap().unwrap().synchronized);
    assert!(!outbox
        .reconcile(&Remote::default(), "bot", "roles")
        .unwrap());
    assert!(outbox.reconcile(&remote, "bot", "configuration").is_err());
}

#[test]
fn remote_envelope_tampering_does_not_acknowledge_local_record() {
    let outbox = EncryptedControlOutbox::in_memory().unwrap();
    let keys = manager();
    outbox
        .enqueue(&keys, "bot", "audit", b"audit-private")
        .unwrap();
    let remote = Remote {
        record: Mutex::new(None),
        lost_ack: true,
    };
    assert!(outbox.flush(&remote, "bot", "audit").is_err());
    {
        let mut guard = remote.record.lock().unwrap();
        guard.as_mut().unwrap().0.ciphertext = "AQID".to_owned();
    }
    assert!(outbox.reconcile(&remote, "bot", "audit").is_err());
    assert!(!outbox.status("bot", "audit").unwrap().unwrap().synchronized);
}
