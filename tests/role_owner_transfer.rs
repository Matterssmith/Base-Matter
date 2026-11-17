/*
 * Matter-Base — Owner handover into scoped role administration.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use matter_base::{BotRegistry, RoleStore};
use rusqlite::Connection;

struct Db(std::path::PathBuf);
impl Db {
    fn new() -> Self {
        let mut random = [0_u8; 16];
        getrandom::fill(&mut random).unwrap();
        Self(std::env::temp_dir().join(format!("matter-role-handover-{}.db", hex::encode(random))))
    }
    fn store(&self) -> RoleStore {
        RoleStore::open(&self.0).unwrap()
    }
}
impl Drop for Db {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

#[test]
fn transfers_role_administration_only_to_current_bot_owner() {
    let db = Db::new();
    let roles = db.store();
    let bots = BotRegistry::in_memory();
    let bot = bots.register("family", "v1", "bot", "alice").unwrap();
    let scope = bot.instance_id();
    roles.provision_scope(scope, "alice").unwrap();
    roles.create_role(scope, "alice", "staff", None).unwrap();
    assert!(roles
        .transfer_scope_owner(scope, "alice", "bob", &bots)
        .is_err());
    bots.transfer_owner(scope, "alice", "bob").unwrap();
    assert!(roles
        .transfer_scope_owner(scope, "mallory", "bob", &bots)
        .is_err());
    assert!(roles
        .transfer_scope_owner(scope, "alice", "mallory", &bots)
        .is_err());
    assert!(roles
        .transfer_scope_owner(scope, "alice", "bob", &bots)
        .unwrap());
    assert!(!roles
        .transfer_scope_owner(scope, "bob", "bob", &bots)
        .unwrap());
    assert!(roles.create_role(scope, "alice", "blocked", None).is_err());
    roles.create_role(scope, "bob", "new-staff", None).unwrap();
    assert!(roles.audit_history(scope, "alice", 20, 0).is_err());
    let events = roles.audit_history(scope, "bob", 20, 0).unwrap();
    assert!(events
        .iter()
        .any(|record| record.action == "transfer_scope_owner"
            && record.target_id.as_deref() == Some("bob")));
}

#[test]
fn audit_failure_rolls_back_role_scope_owner_change() {
    let db = Db::new();
    let roles = db.store();
    let bots = BotRegistry::in_memory();
    let bot = bots.register("family", "v1", "bot", "alice").unwrap();
    let scope = bot.instance_id();
    roles.provision_scope(scope, "alice").unwrap();
    bots.transfer_owner(scope, "alice", "bob").unwrap();
    let raw = Connection::open(&db.0).unwrap();
    raw.execute_batch(
        "CREATE TRIGGER block_transfer_role_audit BEFORE INSERT ON role_audit
         WHEN NEW.action='transfer_scope_owner'
         BEGIN SELECT RAISE(ABORT,'forced rollback'); END;",
    )
    .unwrap();
    assert!(roles
        .transfer_scope_owner(scope, "alice", "bob", &bots)
        .is_err());
    roles
        .create_role(scope, "alice", "pre-recovery", None)
        .unwrap();
    assert!(roles.audit_history(scope, "bob", 20, 0).is_err());
    raw.execute_batch("DROP TRIGGER block_transfer_role_audit")
        .unwrap();
    assert!(roles
        .transfer_scope_owner(scope, "alice", "bob", &bots)
        .unwrap());
    assert!(roles.audit_history(scope, "alice", 20, 0).is_err());
}
