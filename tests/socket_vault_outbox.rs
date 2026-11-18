/*
 * Base-Matter — encrypted-only local role export contract for Vault-Matter.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::Result;
use matter_base::{
    encryption::{decrypt, EncryptedEnvelope},
    security::key_manager::KeyManager,
    EncryptedControlOutbox, EncryptedEnvelopeSink, RoleStore,
};
use std::sync::Mutex;

#[derive(Default)]
struct CapturingEncryptedSink {
    last: Mutex<Option<EncryptedEnvelope>>,
}
impl EncryptedEnvelopeSink for CapturingEncryptedSink {
    fn put(
        &self,
        envelope: &EncryptedEnvelope,
        _expected_revision: Option<&str>,
    ) -> Result<String> {
        self.last.lock().unwrap().replace(envelope.clone());
        Ok("test-revision".to_owned())
    }
}

#[test]
fn changed_nondefault_socket_role_is_encrypted_and_user_revert_removes_grant() {
    let roles = RoleStore::in_memory().unwrap();
    roles.provision_scope("socket:wa-alpha", "owner").unwrap();
    roles
        .ensure_root_role("socket:wa-alpha", "owner", "admin")
        .unwrap();
    let key = [7u8; 32];
    let mut manager = KeyManager::new();
    manager.rotate("ephemeral-test-key".into(), key).unwrap();

    let outbox = EncryptedControlOutbox::in_memory().unwrap();
    let sink = CapturingEncryptedSink::default();
    roles
        .set_exclusive_role(
            "socket:wa-alpha",
            "owner",
            "pn:5215512345678",
            Some("admin"),
        )
        .unwrap();
    assert!(outbox
        .enqueue_role_snapshot(&roles, "owner", &manager, "socket:wa-alpha")
        .unwrap());
    assert!(outbox.flush(&sink, "socket:wa-alpha", "roles").unwrap());
    let encrypted = sink.last.lock().unwrap().clone().unwrap();
    let wire = serde_json::to_string(&encrypted).unwrap();
    assert!(!wire.contains("5215512345678"));
    assert_eq!(encrypted.record_type, "roles");
    let plaintext = decrypt(&encrypted, &key).unwrap();
    let decoded: serde_json::Value = serde_json::from_slice(&plaintext).unwrap();
    let grants = decoded["grants"].as_array().unwrap();
    assert_eq!(grants.len(), 1);
    assert_eq!(grants[0][0], "pn:5215512345678");
    assert_eq!(grants[0][1], "admin");

    assert!(roles
        .set_exclusive_role("socket:wa-alpha", "owner", "pn:5215512345678", None)
        .unwrap());
    assert!(outbox
        .enqueue_role_snapshot(&roles, "owner", &manager, "socket:wa-alpha")
        .unwrap());
    assert!(outbox.flush(&sink, "socket:wa-alpha", "roles").unwrap());
    let after = sink.last.lock().unwrap().clone().unwrap();
    let restored: serde_json::Value =
        serde_json::from_slice(&decrypt(&after, &key).unwrap()).unwrap();
    assert!(restored["grants"].as_array().unwrap().is_empty());
    assert!(!serde_json::to_string(&after)
        .unwrap()
        .contains("5215512345678"));
}
