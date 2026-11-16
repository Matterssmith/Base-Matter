/*
 * Matter-Base — Bot Registry Tests
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use matter_base::{BotAction, BotRegistry, OwnerRecoveryAuthorizer, SqliteBotIdentityStore};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "matter-base-bot-registry-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn in_memory_registry_keeps_identity_independent_and_names_unique() {
    let registry = BotRegistry::in_memory();
    let first = registry
        .register("family-a", "release-1", "matter", "owner-1")
        .unwrap();
    let second = registry
        .register("family-a", "release-2", "matter", "owner-1")
        .unwrap();

    assert_eq!(first.family_id(), second.family_id());
    assert_ne!(first.release_key(), second.release_key());
    assert_ne!(first.instance_id(), second.instance_id());
    assert_eq!(first.display_name(), "matter");
    assert_eq!(second.display_name(), "matter (2)");
    assert!(registry
        .audit_history(first.instance_id(), "intruder")
        .is_err());
    assert_eq!(
        registry
            .audit_history(first.instance_id(), "owner-1")
            .unwrap()
            .len(),
        1
    );
    assert_eq!(registry.get(first.instance_id()).unwrap(), Some(first));
    assert_eq!(registry.list().unwrap().len(), 2);
}

#[test]
fn sqlite_registry_recovers_identity_after_reopening() {
    let directory = TemporaryDirectory::new();
    let database = directory.0.join("bots.db");
    let identity = {
        let registry = BotRegistry::open_sqlite(&database).unwrap();
        registry
            .register("family-a", "release-1", "matter", "owner-1")
            .unwrap()
    };

    let recovered = BotRegistry::open_sqlite(&database).unwrap();
    assert_eq!(
        recovered.get(identity.instance_id()).unwrap(),
        Some(identity.clone())
    );
    assert_eq!(recovered.list().unwrap(), vec![identity]);
}

#[test]
fn sqlite_name_allocation_is_atomic_across_connections() {
    let directory = TemporaryDirectory::new();
    let database = directory.0.join("concurrent-bots.db");
    let first = BotRegistry::new(Arc::new(SqliteBotIdentityStore::open(&database).unwrap()));
    let second = BotRegistry::new(Arc::new(SqliteBotIdentityStore::open(&database).unwrap()));
    let barrier = Arc::new(Barrier::new(3));

    let first_barrier = Arc::clone(&barrier);
    let first_thread = thread::spawn(move || {
        first_barrier.wait();
        first.register("family-a", "release-1", "matter", "owner-1")
    });
    let second_barrier = Arc::clone(&barrier);
    let second_thread = thread::spawn(move || {
        second_barrier.wait();
        second.register("family-a", "release-1", "matter", "owner-1")
    });

    barrier.wait();
    let first = first_thread.join().unwrap().unwrap();
    let second = second_thread.join().unwrap().unwrap();
    let names = [first.display_name(), second.display_name()];
    assert_ne!(names[0], names[1]);
    assert_eq!(names.iter().filter(|name| **name == "matter").count(), 1);
}

#[test]
fn registry_rejects_invalid_identity_fields() {
    let registry = BotRegistry::in_memory();
    assert!(registry
        .register("\n", "release-1", "matter", "owner-1")
        .is_err());
    assert!(registry
        .register("family-a", "", "matter", "owner-1")
        .is_err());
    assert!(registry
        .register("family-a", "release-1", "\u{7f}", "owner-1")
        .is_err());
    assert!(registry.get(" ").is_err());
}

#[test]
fn sqlite_registry_migration_preserves_existing_rows_and_records_version() {
    let directory = TemporaryDirectory::new();
    let database = directory.0.join("existing-bots.db");
    let connection = rusqlite::Connection::open(&database).unwrap();
    connection
        .execute_batch(
            "CREATE TABLE bot_identities (
                 instance_id TEXT PRIMARY KEY NOT NULL,
                 family_id TEXT NOT NULL,
                 release_key TEXT NOT NULL,
                 display_name TEXT NOT NULL UNIQUE,
                 created_at INTEGER NOT NULL
             );
             INSERT INTO bot_identities VALUES (
                 'inst_existing', 'family-a', 'release-1', 'legacy-bot', 1
             );",
        )
        .unwrap();
    drop(connection);

    let registry = BotRegistry::open_sqlite(&database).unwrap();
    let recovered = registry.get("inst_existing").unwrap().unwrap();
    assert_eq!(recovered.family_id(), "family-a");
    assert_eq!(recovered.display_name(), "legacy-bot");

    let connection = rusqlite::Connection::open(database).unwrap();
    let version: i64 = connection
        .query_row(
            "SELECT version FROM matter_schema_versions WHERE component = 'bot_registry'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(version, 3);
    assert_eq!(recovered.owner_id(), None);
}

#[test]
fn sqlite_registry_migration_failure_rolls_back_schema_version_and_columns() {
    let directory = TemporaryDirectory::new();
    let database = directory.0.join("migration-rollback.db");
    let connection = rusqlite::Connection::open(&database).unwrap();
    connection
        .execute_batch(
            "CREATE TABLE matter_schema_versions (
                 component TEXT PRIMARY KEY NOT NULL,
                 version INTEGER NOT NULL
             );
             INSERT INTO matter_schema_versions VALUES ('bot_registry', 1);
             CREATE TABLE bot_identities (
                 instance_id TEXT PRIMARY KEY NOT NULL,
                 family_id TEXT NOT NULL,
                 release_key TEXT NOT NULL,
                 display_name TEXT NOT NULL UNIQUE,
                 created_at INTEGER NOT NULL
             );
             INSERT INTO bot_identities VALUES (
                 'inst_existing', 'family-a', 'release-1', 'legacy-bot', 1
             );
             CREATE TABLE bot_registry_audit (conflicting_column TEXT);",
        )
        .unwrap();
    drop(connection);

    assert!(BotRegistry::open_sqlite(&database).is_err());
    let connection = rusqlite::Connection::open(&database).unwrap();
    let version: i64 = connection
        .query_row(
            "SELECT version FROM matter_schema_versions WHERE component = 'bot_registry'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(version, 1);
    let column_count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM pragma_table_info('bot_identities')
             WHERE name IN ('owner_id', 'co_owner_ids')",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(column_count, 0);
    let conflict_column_count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM pragma_table_info('bot_registry_audit')
             WHERE name = 'conflicting_column'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(conflict_column_count, 1);
    drop(connection);

    let connection = rusqlite::Connection::open(&database).unwrap();
    connection
        .execute_batch("DROP TABLE bot_registry_audit;")
        .unwrap();
    drop(connection);

    let recovered = BotRegistry::open_sqlite(&database).unwrap();
    assert_eq!(
        recovered.get("inst_existing").unwrap().unwrap().family_id(),
        "family-a"
    );
    let connection = rusqlite::Connection::open(database).unwrap();
    let version: i64 = connection
        .query_row(
            "SELECT version FROM matter_schema_versions WHERE component = 'bot_registry'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(version, 3);
}

#[test]
fn sqlite_registry_rejects_newer_schema_without_rewriting_it() {
    let directory = TemporaryDirectory::new();
    let database = directory.0.join("future-schema.db");
    let connection = rusqlite::Connection::open(&database).unwrap();
    connection
        .execute_batch(
            "CREATE TABLE matter_schema_versions (
                 component TEXT PRIMARY KEY NOT NULL,
                 version INTEGER NOT NULL
             );
             INSERT INTO matter_schema_versions VALUES ('bot_registry', 99);",
        )
        .unwrap();
    drop(connection);

    assert!(BotRegistry::open_sqlite(&database).is_err());
    let connection = rusqlite::Connection::open(database).unwrap();
    let version: i64 = connection
        .query_row(
            "SELECT version FROM matter_schema_versions WHERE component = 'bot_registry'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(version, 99);
}

#[test]
fn failed_identity_write_rolls_back_and_name_can_be_reused() {
    let directory = TemporaryDirectory::new();
    let database = directory.0.join("interrupted-bot.db");
    let registry = BotRegistry::open_sqlite(&database).unwrap();
    let connection = rusqlite::Connection::open(&database).unwrap();
    connection
        .execute_batch(
            "CREATE TRIGGER fail_bot_identity_insert
             BEFORE INSERT ON bot_identities
             BEGIN
                 SELECT RAISE(ABORT, 'simulated storage interruption');
             END;",
        )
        .unwrap();

    assert!(registry
        .register("family-a", "release-1", "matter", "owner-1")
        .is_err());
    assert!(registry.list().unwrap().is_empty());

    connection
        .execute_batch("DROP TRIGGER fail_bot_identity_insert;")
        .unwrap();
    let registered = registry
        .register("family-a", "release-1", "matter", "owner-1")
        .unwrap();
    assert_eq!(registered.display_name(), "matter");
    assert_eq!(registry.list().unwrap(), vec![registered]);
}

#[test]
fn registry_rejects_corrupted_persisted_identity_fields() {
    let directory = TemporaryDirectory::new();
    let database = directory.0.join("corrupt-identity.db");
    let registry = BotRegistry::open_sqlite(&database).unwrap();
    let identity = registry
        .register("family-a", "release-1", "matter", "owner-1")
        .unwrap();
    let connection = rusqlite::Connection::open(database).unwrap();
    connection
        .execute(
            "UPDATE bot_identities SET family_id = '' WHERE instance_id = ?1",
            [identity.instance_id()],
        )
        .unwrap();

    assert!(registry.get(identity.instance_id()).is_err());
    assert!(registry.list().is_err());
}

struct TestOwnerRecovery;

impl OwnerRecoveryAuthorizer for TestOwnerRecovery {
    fn authorize_claim(&self, instance_id: &str, actor_id: &str) -> anyhow::Result<()> {
        if instance_id == "inst_existing" && actor_id == "recovered-owner" {
            Ok(())
        } else {
            anyhow::bail!("ownership recovery denied")
        }
    }
}

#[test]
fn owner_operations_are_checked_persisted_and_audited() {
    let directory = TemporaryDirectory::new();
    let database = directory.0.join("owned-bot.db");
    let registry = BotRegistry::open_sqlite(&database).unwrap();
    let identity = registry
        .register("family-a", "release-1", "matter", "owner-1")
        .unwrap();

    assert_eq!(identity.owner_id(), Some("owner-1"));
    assert!(registry
        .authorize_owner(identity.instance_id(), "intruder")
        .is_err());
    registry
        .authorize_owner(identity.instance_id(), "owner-1")
        .unwrap();
    assert!(registry
        .transfer_owner(identity.instance_id(), "intruder", "owner-2")
        .is_err());
    registry
        .transfer_owner(identity.instance_id(), "owner-1", "owner-2")
        .unwrap();

    let reopened = BotRegistry::open_sqlite(&database).unwrap();
    assert_eq!(
        reopened
            .get(identity.instance_id())
            .unwrap()
            .unwrap()
            .owner_id(),
        Some("owner-2")
    );
    assert!(reopened
        .audit_history(identity.instance_id(), "intruder")
        .is_err());
    let audit = reopened
        .audit_history(identity.instance_id(), "owner-2")
        .unwrap();
    assert_eq!(audit.len(), 2);
    assert_eq!(audit[0].action, "register");
    assert_eq!(audit[0].actor_id, "owner-1");
    assert_eq!(audit[1].action, "transfer_owner");
    assert_eq!(audit[1].target_owner_id.as_deref(), Some("owner-2"));

    assert!(reopened
        .remove_owned(identity.instance_id(), "owner-1")
        .is_err());
    assert!(reopened
        .remove_owned(identity.instance_id(), "owner-2")
        .is_ok());
    assert!(reopened.get(identity.instance_id()).unwrap().is_none());
    assert!(reopened
        .audit_history(identity.instance_id(), "owner-2")
        .is_err());
    let connection = rusqlite::Connection::open(database).unwrap();
    let audit_count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM bot_registry_audit WHERE instance_id = ?1",
            [identity.instance_id()],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(audit_count, 3);
}

#[test]
fn legacy_unowned_identity_requires_external_recovery_authorization() {
    let directory = TemporaryDirectory::new();
    let database = directory.0.join("legacy-owned-bot.db");
    let connection = rusqlite::Connection::open(&database).unwrap();
    connection
        .execute_batch(
            "CREATE TABLE bot_identities (
             instance_id TEXT PRIMARY KEY NOT NULL,
             family_id TEXT NOT NULL,
             release_key TEXT NOT NULL,
             display_name TEXT NOT NULL UNIQUE,
             created_at INTEGER NOT NULL
         );
         INSERT INTO bot_identities VALUES (
             'inst_existing', 'family-a', 'release-1', 'legacy-bot', 1
         );",
        )
        .unwrap();
    drop(connection);

    let registry = BotRegistry::open_sqlite(&database).unwrap();
    let authorizer = TestOwnerRecovery;
    assert!(registry
        .claim_unowned("inst_existing", "intruder", &authorizer)
        .is_err());
    assert_eq!(
        registry.get("inst_existing").unwrap().unwrap().owner_id(),
        None
    );
    registry
        .claim_unowned("inst_existing", "recovered-owner", &authorizer)
        .unwrap();
    assert_eq!(
        registry.get("inst_existing").unwrap().unwrap().owner_id(),
        Some("recovered-owner")
    );
    assert_eq!(
        registry
            .audit_history("inst_existing", "recovered-owner")
            .unwrap()[0]
            .action,
        "claim_unowned"
    );
}

#[test]
fn co_owner_scope_cannot_transfer_or_delete_and_is_audited() {
    let directory = TemporaryDirectory::new();
    let database = directory.0.join("co-owner.db");
    let registry = BotRegistry::open_sqlite(&database).unwrap();
    let identity = registry
        .register("family-a", "release-1", "matter", "owner-1")
        .unwrap();

    assert!(registry
        .add_co_owner(identity.instance_id(), "intruder", "co-1")
        .is_err());
    registry
        .add_co_owner(identity.instance_id(), "owner-1", "co-1")
        .unwrap();
    registry
        .authorize(identity.instance_id(), "co-1", BotAction::Manage)
        .unwrap();
    assert!(registry
        .authorize(identity.instance_id(), "co-1", BotAction::ManageOwners)
        .is_err());
    assert!(registry
        .authorize(identity.instance_id(), "co-1", BotAction::Delete)
        .is_err());
    assert!(registry
        .transfer_owner(identity.instance_id(), "co-1", "co-2")
        .is_err());
    assert!(registry
        .remove_owned(identity.instance_id(), "co-1")
        .is_err());

    let recovered = BotRegistry::open_sqlite(&database).unwrap();
    let identity = recovered.get(identity.instance_id()).unwrap().unwrap();
    assert_eq!(identity.co_owner_ids(), &["co-1"]);
    recovered
        .remove_co_owner(identity.instance_id(), "owner-1", "co-1")
        .unwrap();
    assert!(recovered
        .authorize(identity.instance_id(), "co-1", BotAction::Manage)
        .is_err());
    assert!(recovered
        .audit_history(identity.instance_id(), "co-1")
        .is_err());
    let audit = recovered
        .audit_history(identity.instance_id(), "owner-1")
        .unwrap();
    assert_eq!(
        audit
            .iter()
            .map(|record| record.action.as_str())
            .collect::<Vec<_>>(),
        ["register", "add_co_owner", "remove_co_owner"]
    );
}

#[test]
fn concurrent_owner_transfers_from_separate_connections_have_one_winner() {
    use std::sync::Barrier;

    let directory = TemporaryDirectory::new();
    let database = directory.0.join("concurrent-owner-transfer.db");
    let initial = BotRegistry::open_sqlite(&database).unwrap();
    let bot = initial
        .register("family-a", "release-1", "matter", "owner-1")
        .unwrap();
    let first = BotRegistry::new(std::sync::Arc::new(
        SqliteBotIdentityStore::open(&database).unwrap(),
    ));
    let second = BotRegistry::new(std::sync::Arc::new(
        SqliteBotIdentityStore::open(&database).unwrap(),
    ));
    let barrier = Arc::new(Barrier::new(3));
    let first_barrier = Arc::clone(&barrier);
    let bot_id = bot.instance_id().to_owned();
    let first_thread = thread::spawn(move || {
        first_barrier.wait();
        first.transfer_owner(&bot_id, "owner-1", "owner-2")
    });
    let second_barrier = Arc::clone(&barrier);
    let bot_id = bot.instance_id().to_owned();
    let second_thread = thread::spawn(move || {
        second_barrier.wait();
        second.transfer_owner(&bot_id, "owner-1", "owner-3")
    });

    barrier.wait();
    let results = [first_thread.join().unwrap(), second_thread.join().unwrap()];
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);

    let recovered = BotRegistry::open_sqlite(&database).unwrap();
    let persisted = recovered.get(bot.instance_id()).unwrap().unwrap();
    assert!(matches!(persisted.owner_id(), Some("owner-2" | "owner-3")));
    let audit = recovered
        .audit_history(bot.instance_id(), persisted.owner_id().unwrap())
        .unwrap();
    assert_eq!(
        audit
            .iter()
            .filter(|record| record.action == "transfer_owner")
            .count(),
        1
    );
}
