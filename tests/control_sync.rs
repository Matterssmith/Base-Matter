/*
 * Matter-Base — Encrypted control outbox regression tests.
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::{bail, Result};
use matter_base::{
    security::{encryption, key_manager::KeyManager},
    EncryptedControlOutbox, EncryptedEnvelopeSink, RoleStore, RuleEffect,
};
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;

struct Db(PathBuf);
impl Db {
    fn new() -> Self {
        let mut bytes = [0_u8; 16];
        getrandom::fill(&mut bytes).unwrap();
        Self(std::env::temp_dir().join(format!("matter-vault-sync-{}.db", hex::encode(bytes))))
    }
}
impl Drop for Db {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

struct Sink {
    revision: Mutex<Option<String>>,
    latest: Mutex<Option<encryption::EncryptedEnvelope>>,
    reject: bool,
}
impl Sink {
    fn new(reject: bool) -> Self {
        Self {
            revision: Mutex::new(None),
            latest: Mutex::new(None),
            reject,
        }
    }
}
impl EncryptedEnvelopeSink for Sink {
    fn put(
        &self,
        envelope: &encryption::EncryptedEnvelope,
        expected: Option<&str>,
    ) -> Result<String> {
        if self.reject {
            bail!("private provider failure");
        }
        let mut revision = self.revision.lock().unwrap();
        if revision.as_deref() != expected {
            bail!("remote compare-and-swap conflict");
        }
        *self.latest.lock().unwrap() = Some(envelope.clone());
        let next = match revision.as_deref() {
            None => "r1".to_owned(),
            Some("r1") => "r2".to_owned(),
            Some("r2") => "r3".to_owned(),
            _ => "r4".to_owned(),
        };
        *revision = Some(next.clone());
        Ok(next)
    }
}

fn manager() -> KeyManager {
    let mut keys = KeyManager::new();
    keys.rotate("key-1".to_owned(), [7_u8; 32]).unwrap();
    keys
}

#[test]
fn encrypts_before_storage_and_only_publishes_changed_content() {
    let db = Db::new();
    let outbox = EncryptedControlOutbox::open(&db.0).unwrap();
    let keys = manager();
    let secret = b"private-role-membership";
    assert!(outbox.enqueue(&keys, "bot-1", "roles", secret).unwrap());
    assert!(!outbox.enqueue(&keys, "bot-1", "roles", secret).unwrap());
    let raw = Connection::open(&db.0).unwrap();
    let stored: String = raw
        .query_row(
            "SELECT envelope_json FROM encrypted_control_outbox",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert!(!stored.contains("private-role-membership"));
    let sink = Sink::new(false);
    assert!(outbox.flush(&sink, "bot-1", "roles").unwrap());
    assert!(!outbox.flush(&sink, "bot-1", "roles").unwrap());
    assert!(
        outbox
            .status("bot-1", "roles")
            .unwrap()
            .unwrap()
            .synchronized
    );
    let envelope = sink.latest.lock().unwrap().clone().unwrap();
    assert_eq!(
        encryption::decrypt_with_manager(&envelope, &keys).unwrap(),
        secret
    );
    assert!(outbox
        .enqueue(&keys, "bot-1", "roles", b"new-policy")
        .unwrap());
    assert!(outbox.flush(&sink, "bot-1", "roles").unwrap());
}

#[test]
fn failed_sink_preserves_pending_ciphertext_for_retry_after_restart() {
    let db = Db::new();
    let keys = manager();
    let outbox = EncryptedControlOutbox::open(&db.0).unwrap();
    assert!(outbox
        .enqueue(&keys, "bot", "configuration", b"powered=off")
        .unwrap());
    assert_eq!(
        outbox
            .flush(&Sink::new(true), "bot", "configuration")
            .unwrap_err()
            .to_string(),
        "Encrypted envelope publication failed"
    );
    drop(outbox);
    let restored = EncryptedControlOutbox::open(&db.0).unwrap();
    assert!(
        !restored
            .status("bot", "configuration")
            .unwrap()
            .unwrap()
            .synchronized
    );
    assert!(restored
        .flush(&Sink::new(false), "bot", "configuration")
        .unwrap());
}

#[test]
fn role_snapshot_is_owner_scoped_canonical_and_encrypted() {
    let roles = RoleStore::in_memory().unwrap();
    roles.provision_scope("bot", "owner").unwrap();
    roles.create_role("bot", "owner", "staff", None).unwrap();
    roles
        .set_rule(
            "bot",
            "owner",
            "staff",
            "command:ping",
            "execute",
            RuleEffect::Allow,
        )
        .unwrap();
    roles.grant("bot", "owner", "member", "staff").unwrap();
    let keys = manager();
    let outbox = EncryptedControlOutbox::in_memory().unwrap();
    assert!(outbox
        .enqueue_role_snapshot(&roles, "owner", &keys, "bot")
        .unwrap());
    assert!(!outbox
        .enqueue_role_snapshot(&roles, "owner", &keys, "bot")
        .unwrap());
    assert!(outbox
        .enqueue_role_snapshot(&roles, "member", &keys, "bot")
        .is_err());
    let sink = Sink::new(false);
    outbox.flush(&sink, "bot", "roles").unwrap();
    let envelope = sink.latest.lock().unwrap().clone().unwrap();
    let decoded = encryption::decrypt_with_manager(&envelope, &keys).unwrap();
    let snapshot: serde_json::Value = serde_json::from_slice(&decoded).unwrap();
    assert_eq!(snapshot["scope"], "bot");
    assert_eq!(snapshot["grants"][0][0], "member");
    assert_eq!(snapshot["roles"][0][0], "staff");
}

#[test]
fn key_rotation_requeues_unchanged_plaintext_under_new_key() {
    let outbox = EncryptedControlOutbox::in_memory().unwrap();
    let mut keys = manager();
    assert!(outbox.enqueue(&keys, "bot", "roles", b"same").unwrap());
    let sink = Sink::new(false);
    outbox.flush(&sink, "bot", "roles").unwrap();
    keys.rotate("key-2".to_owned(), [8_u8; 32]).unwrap();
    assert!(outbox.enqueue(&keys, "bot", "roles", b"same").unwrap());
    outbox.flush(&sink, "bot", "roles").unwrap();
    let envelope = sink.latest.lock().unwrap().clone().unwrap();
    assert_eq!(envelope.key_id, "key-2");
    assert_eq!(
        encryption::decrypt_with_manager(&envelope, &keys).unwrap(),
        b"same"
    );
}
