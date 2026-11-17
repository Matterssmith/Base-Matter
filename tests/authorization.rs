/*
 * Matter-Base — Persistent role authorization regressions.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use matter_base::{CommandRouter, RoleCommandAuthorizer, RoleStore, RuleEffect};
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Arc;

struct TestDatabase(PathBuf);

impl TestDatabase {
    fn new() -> Self {
        let mut random = [0_u8; 16];
        getrandom::fill(&mut random).unwrap();
        Self(std::env::temp_dir().join(format!("matter-base-roles-{}.db", hex::encode(random))))
    }

    fn store(&self) -> RoleStore {
        RoleStore::open(&self.0).unwrap()
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

#[test]
fn inherited_grants_and_explicit_deny_override_allow() {
    let db = TestDatabase::new();
    let store = db.store();
    store.provision_scope("bot-1", "owner").unwrap();
    store.create_role("bot-1", "owner", "reader", None).unwrap();
    store
        .create_role("bot-1", "owner", "staff", Some("reader"))
        .unwrap();
    store
        .set_rule(
            "bot-1",
            "owner",
            "reader",
            "command:status",
            "execute",
            RuleEffect::Allow,
        )
        .unwrap();
    store.grant("bot-1", "owner", "member", "staff").unwrap();
    assert!(store
        .allowed("bot-1", "member", "command:status", "execute")
        .unwrap());
    store
        .set_rule(
            "bot-1",
            "owner",
            "staff",
            "command:status",
            "execute",
            RuleEffect::Deny,
        )
        .unwrap();
    assert!(!store
        .allowed("bot-1", "member", "command:status", "execute")
        .unwrap());
    assert!(!store
        .allowed("bot-1", "owner", "command:status", "execute")
        .unwrap());
}

#[test]
fn scope_isolation_owner_permissions_and_bounded_audit() {
    let db = TestDatabase::new();
    let store = db.store();
    store.provision_scope("bot-1", "owner-1").unwrap();
    store.provision_scope("bot-2", "owner-2").unwrap();
    assert!(store.provision_scope("bot-1", "owner-2").is_err());
    assert!(store
        .create_role("bot-1", "owner-2", "staff", None)
        .is_err());
    store
        .create_role("bot-1", "owner-1", "staff", None)
        .unwrap();
    store
        .set_rule(
            "bot-1",
            "owner-1",
            "staff",
            "*",
            "execute",
            RuleEffect::Allow,
        )
        .unwrap();
    store.grant("bot-1", "owner-1", "member", "staff").unwrap();
    assert!(store
        .allowed("bot-1", "member", "anything", "execute")
        .unwrap());
    assert!(!store
        .allowed("bot-2", "member", "anything", "execute")
        .unwrap());
    assert!(store.audit_history("bot-1", "owner-2", 10, 0).is_err());
    assert!(store.audit_history("bot-1", "owner-1", 101, 0).is_err());
    assert_eq!(
        store.audit_history("bot-1", "owner-1", 1, 0).unwrap().len(),
        1
    );
}

#[test]
fn durable_revoke_and_single_owner_control() {
    let db = TestDatabase::new();
    let store = db.store();
    store.provision_scope("bot", "owner").unwrap();
    store.create_role("bot", "owner", "staff", None).unwrap();
    store
        .set_rule("bot", "owner", "staff", "*", "*", RuleEffect::Allow)
        .unwrap();
    assert!(store.grant("bot", "owner", "member", "staff").unwrap());
    assert!(!store.grant("bot", "owner", "member", "staff").unwrap());
    drop(store);
    let store = db.store();
    assert!(store.allowed("bot", "member", "x", "y").unwrap());
    assert!(store.revoke("bot", "owner", "member", "staff").unwrap());
    assert!(!store.allowed("bot", "member", "x", "y").unwrap());
    assert!(!store.revoke("bot", "owner", "member", "staff").unwrap());
}

#[test]
fn failed_audit_rolls_back_policy_mutation() {
    let db = TestDatabase::new();
    let store = db.store();
    store.provision_scope("bot", "owner").unwrap();
    store.create_role("bot", "owner", "staff", None).unwrap();
    store.grant("bot", "owner", "member", "staff").unwrap();
    store
        .set_rule(
            "bot",
            "owner",
            "staff",
            "command:ping",
            "execute",
            RuleEffect::Allow,
        )
        .unwrap();
    let raw = Connection::open(&db.0).unwrap();
    raw.execute_batch(
        "CREATE TRIGGER reject_new_role_audit
         BEFORE INSERT ON role_audit BEGIN SELECT RAISE(ABORT,'injected'); END;",
    )
    .unwrap();
    assert!(store
        .set_rule(
            "bot",
            "owner",
            "staff",
            "command:ping",
            "execute",
            RuleEffect::Deny
        )
        .is_err());
    assert!(store
        .allowed("bot", "member", "command:ping", "execute")
        .unwrap());
    assert!(store.revoke("bot", "owner", "member", "staff").is_err());
    assert!(store
        .allowed("bot", "member", "command:ping", "execute")
        .unwrap());
}

#[test]
fn router_authorizes_canonical_aliases_without_owner_bypass() {
    let store = Arc::new(RoleStore::in_memory().unwrap());
    store.provision_scope("bot", "owner").unwrap();
    store.create_role("bot", "owner", "staff", None).unwrap();
    store
        .set_rule(
            "bot",
            "owner",
            "staff",
            "command:status",
            "execute",
            RuleEffect::Allow,
        )
        .unwrap();
    store.grant("bot", "owner", "member", "staff").unwrap();
    let router = CommandRouter::new(
        vec!["#".to_owned()],
        Arc::new(RoleCommandAuthorizer::new(store)),
    )
    .unwrap();
    router
        .register("status", &["st".to_owned()], |_| Ok("ready".to_owned()))
        .unwrap();
    assert_eq!(
        router.route("bot", "member", "room", "#st").unwrap(),
        Some("ready".to_owned())
    );
    assert!(router.route("bot", "outsider", "room", "#st").is_err());
    assert!(router.route("bot", "owner", "room", "#st").is_err());
    assert!(router.route("other-bot", "member", "room", "#st").is_err());
}

#[test]
fn corrupt_parent_chain_fails_closed_and_audit_is_immutable() {
    let db = TestDatabase::new();
    let store = db.store();
    store.provision_scope("bot", "owner").unwrap();
    store.create_role("bot", "owner", "staff", None).unwrap();
    store.grant("bot", "owner", "member", "staff").unwrap();
    let raw = Connection::open(&db.0).unwrap();
    assert!(raw
        .execute("DELETE FROM role_audit WHERE scope_id='bot'", [])
        .is_err());
    raw.execute(
        "UPDATE role_definitions SET parent_role='staff' WHERE role_id='staff'",
        [],
    )
    .unwrap();
    assert!(store.allowed("bot", "member", "any", "execute").is_err());
}
