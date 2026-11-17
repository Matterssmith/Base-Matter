/* Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use matter_base::activation::{ActivationSource, SocketActivationStore};

struct TestDirectory(std::path::PathBuf);
impl TestDirectory {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "matter-activation-{}-{}",
            std::process::id(),
            time::OffsetDateTime::now_utc().unix_timestamp_nanos()
        ));
        std::fs::create_dir(&p).unwrap();
        Self(p)
    }
}
impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn recovery_preserves_power_precedence_and_socket_isolation() {
    let dir = TestDirectory::new();
    let path = dir.0.join("state.db");
    {
        let store = SocketActivationStore::open(&path).unwrap();
        store.provision("a", "alice").unwrap();
        store.provision("b", "bob").unwrap();
        store
            .set_group_policy("a", "alice", "group", Some(true))
            .unwrap();
        assert!(
            !store
                .resolve("a", Some("group"), Some(true))
                .unwrap()
                .enabled
        );
        store.set_power("a", "alice", true).unwrap();
        store.set_socket_policy("a", "alice", Some(false)).unwrap();
    }
    let store = SocketActivationStore::open(&path).unwrap();
    let decision = store.resolve("a", Some("group"), Some(false)).unwrap();
    assert!(decision.enabled);
    assert_eq!(decision.source, ActivationSource::Group);
    assert!(!store.resolve("a", None, Some(true)).unwrap().enabled);
    assert!(
        !store
            .resolve("b", Some("group"), Some(true))
            .unwrap()
            .enabled
    );
    store.set_group_policy("a", "alice", "group", None).unwrap();
    assert_eq!(
        store
            .resolve("a", Some("group"), Some(true))
            .unwrap()
            .source,
        ActivationSource::Socket
    );
    store.set_socket_policy("a", "alice", None).unwrap();
    assert_eq!(
        store.resolve("a", None, Some(true)).unwrap().source,
        ActivationSource::Global
    );
    assert_eq!(
        store.resolve("a", None, None).unwrap().source,
        ActivationSource::None
    );
    store.set_power("a", "alice", false).unwrap();
    assert_eq!(
        store.resolve("a", None, Some(true)).unwrap().source,
        ActivationSource::PoweredOff
    );
    assert_eq!(store.audit_history("a", "alice").unwrap().len(), 7);
}

#[test]
fn denial_and_audit_failure_leave_power_and_policies_unchanged() {
    let dir = TestDirectory::new();
    let path = dir.0.join("state.db");
    let store = SocketActivationStore::open(&path).unwrap();
    store.provision("a", "alice").unwrap();
    assert!(store.set_power("a", "global-owner", true).is_err());
    assert!(store
        .set_group_policy("a", "bob", "group", Some(true))
        .is_err());
    assert!(store.audit_history("a", "bob").is_err());
    assert!(store.provision("a", "bob").is_err());
    let c = rusqlite::Connection::open(&path).unwrap();
    c.execute_batch("CREATE TRIGGER reject_audit BEFORE INSERT ON activation_audit BEGIN SELECT RAISE(ABORT,'injected failure'); END;").unwrap();
    assert!(store.set_power("a", "alice", true).is_err());
    assert!(!store.resolve("a", None, Some(true)).unwrap().enabled);
    assert_eq!(store.audit_history("a", "alice").unwrap().len(), 1);
    c.execute_batch("DROP TRIGGER reject_audit;").unwrap();
    store.set_power("a", "alice", true).unwrap();
    assert!(store.resolve("a", None, Some(true)).unwrap().enabled);
}

#[test]
fn unsupported_schema_is_rejected_without_rewriting_version() {
    let dir = TestDirectory::new();
    let path = dir.0.join("future.db");
    let c = rusqlite::Connection::open(&path).unwrap();
    c.execute_batch("CREATE TABLE matter_schema_versions(component TEXT PRIMARY KEY, version INTEGER NOT NULL); INSERT INTO matter_schema_versions VALUES('socket_activation',99);").unwrap();
    assert!(SocketActivationStore::open(&path).is_err());
    let v: i64 = c
        .query_row(
            "SELECT version FROM matter_schema_versions WHERE component='socket_activation'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(v, 99);
}

#[test]
fn corrupt_persisted_power_does_not_enable_dispatch() {
    let dir = TestDirectory::new();
    let path = dir.0.join("corrupt.db");
    let store = SocketActivationStore::open(&path).unwrap();
    store.provision("a", "alice").unwrap();
    let c = rusqlite::Connection::open(&path).unwrap();
    c.execute_batch("PRAGMA ignore_check_constraints=ON; UPDATE activation_sockets SET powered=2;")
        .unwrap();
    assert!(store.resolve("a", None, Some(true)).is_err());
}

#[test]
fn independent_connections_serialize_power_and_audit_together() {
    let dir = TestDirectory::new();
    let path = dir.0.join("concurrent.db");
    let first = SocketActivationStore::open(&path).unwrap();
    first.provision("a", "alice").unwrap();
    let second = SocketActivationStore::open(&path).unwrap();
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let joins: Vec<_> = [(first, true), (second, false)]
        .into_iter()
        .map(|(store, power)| {
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                store.set_power("a", "alice", power).unwrap();
            })
        })
        .collect();
    for join in joins {
        join.join().unwrap();
    }
    let recovered = SocketActivationStore::open(&path).unwrap();
    let audit = recovered.audit_history("a", "alice").unwrap();
    assert_eq!(audit.len(), 3);
    assert_eq!(audit[1].action, "power");
    assert_eq!(audit[2].action, "power");
    assert_ne!(audit[1].value, audit[2].value);
    assert_eq!(
        Some(recovered.resolve("a", None, Some(true)).unwrap().enabled),
        audit[2].value
    );
}

#[test]
fn binding_audit_failure_rolls_back_and_conflicting_binding_preserves_original() {
    let dir = TestDirectory::new();
    let path = dir.0.join("binding.db");
    let store = SocketActivationStore::open(&path).unwrap();
    let bots = matter_base::BotRegistry::in_memory();
    let first = bots.register("f", "r", "first", "alice").unwrap();
    let second = bots.register("f", "r", "second", "alice").unwrap();
    store.provision("a", "alice").unwrap();
    let c = rusqlite::Connection::open(&path).unwrap();
    c.execute_batch("CREATE TRIGGER reject_binding_audit BEFORE INSERT ON activation_audit WHEN NEW.action='bind_bot' BEGIN SELECT RAISE(ABORT,'injected failure'); END;").unwrap();
    assert!(store
        .bind_bot("a", "alice", first.instance_id(), &bots)
        .is_err());
    let count: i64 = c
        .query_row("SELECT COUNT(*) FROM activation_bindings", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 0);
    assert_eq!(store.audit_history("a", "alice").unwrap().len(), 1);
    c.execute_batch("DROP TRIGGER reject_binding_audit;")
        .unwrap();
    store
        .bind_bot("a", "alice", first.instance_id(), &bots)
        .unwrap();
    store
        .bind_bot("a", "alice", first.instance_id(), &bots)
        .unwrap();
    assert!(store
        .bind_bot("a", "alice", second.instance_id(), &bots)
        .is_err());
    let stored: String = c
        .query_row(
            "SELECT bot_instance_id FROM activation_bindings WHERE socket_id='a'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(stored, first.instance_id());
    assert_eq!(store.audit_history("a", "alice").unwrap().len(), 2);
}

#[test]
fn persistent_global_policy_recovers_precedence_without_socket_authority() {
    let dir = TestDirectory::new();
    let path = dir.0.join("global.db");
    {
        let store = SocketActivationStore::open(&path).unwrap();
        store.provision("a", "alice").unwrap();
        store.provision("b", "bob").unwrap();
        store.set_power("a", "alice", true).unwrap();
        assert_eq!(
            store.resolve_persisted("a", None).unwrap().source,
            ActivationSource::None
        );
        assert!(store.set_global_policy("admin", Some(true)).is_err());
        store.provision_global("admin").unwrap();
        assert!(store.provision_global("alice").is_err());
        assert!(store.set_global_policy("alice", Some(true)).is_err());
        assert!(store.global_audit_history("alice", 0, 10).is_err());
        store.set_global_policy("admin", Some(true)).unwrap();
        assert!(store.set_power("a", "admin", true).is_err());
        assert!(store.set_socket_policy("a", "admin", Some(true)).is_err());
        assert!(!store.resolve_persisted("b", None).unwrap().enabled);
        assert!(store.resolve_persisted("a", None).unwrap().enabled);
        store.set_socket_policy("a", "alice", Some(false)).unwrap();
        store
            .set_group_policy("a", "alice", "room", Some(true))
            .unwrap();
    }
    let store = SocketActivationStore::open(&path).unwrap();
    assert_eq!(
        store.resolve_persisted("a", Some("room")).unwrap().source,
        ActivationSource::Group
    );
    assert!(!store.resolve_persisted("a", None).unwrap().enabled);
    store.set_socket_policy("a", "alice", None).unwrap();
    assert_eq!(
        store.resolve_persisted("a", None).unwrap().source,
        ActivationSource::Global
    );
    store.set_global_policy("admin", None).unwrap();
    assert_eq!(
        store.resolve_persisted("a", None).unwrap().source,
        ActivationSource::None
    );
    assert!(store.resolve_persisted("a", Some("room")).unwrap().enabled);
    // The explicit static-policy API preserves its existing semantics.
    assert!(store.resolve("a", None, Some(true)).unwrap().enabled);
    let records = store.global_audit_history("admin", 0, 1).unwrap();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].actor_id, "admin");
    assert_eq!(records[0].action, "provision");
    assert_eq!(
        store
            .global_audit_history("admin", records[0].id, 10)
            .unwrap()
            .len(),
        2
    );
    for (after, limit) in [(-1, 1), (0, 0), (0, 1001)] {
        assert!(store.global_audit_history("admin", after, limit).is_err());
    }
}

#[test]
fn global_audit_failure_rolls_back_bootstrap_and_policy_changes() {
    let dir = TestDirectory::new();
    let path = dir.0.join("global.db");
    let store = SocketActivationStore::open(&path).unwrap();
    store.provision("a", "alice").unwrap();
    store.set_power("a", "alice", true).unwrap();
    let c = rusqlite::Connection::open(&path).unwrap();
    let trigger = "CREATE TRIGGER reject_global_audit BEFORE INSERT ON activation_global_audit
        BEGIN SELECT RAISE(ABORT,'injected failure'); END;";
    c.execute_batch(trigger).unwrap();
    assert!(store.provision_global("admin").is_err());
    let count: i64 = c
        .query_row("SELECT COUNT(*) FROM activation_global", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 0);
    c.execute_batch("DROP TRIGGER reject_global_audit;")
        .unwrap();
    store.provision_global("admin").unwrap();
    store.set_global_policy("admin", Some(true)).unwrap();
    c.execute_batch(trigger).unwrap();
    assert!(store.set_global_policy("admin", Some(false)).is_err());
    assert!(store.resolve_persisted("a", None).unwrap().enabled);
    assert_eq!(store.global_audit_history("admin", 0, 10).unwrap().len(), 2);
    c.execute_batch(
        "DROP TRIGGER reject_global_audit; PRAGMA ignore_check_constraints=ON;
        UPDATE activation_global SET policy=2;",
    )
    .unwrap();
    assert!(store.resolve_persisted("a", None).is_err());
}

#[test]
fn concurrent_global_changes_commit_in_audit_order() {
    let dir = TestDirectory::new();
    let path = dir.0.join("global.db");
    let first = SocketActivationStore::open(&path).unwrap();
    first.provision_global("admin").unwrap();
    first.provision("a", "alice").unwrap();
    first.set_power("a", "alice", true).unwrap();
    let second = SocketActivationStore::open(&path).unwrap();
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let joins: Vec<_> = [(first, true), (second, false)]
        .into_iter()
        .map(|(store, value)| {
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                store.set_global_policy("admin", Some(value)).unwrap();
            })
        })
        .collect();
    for join in joins {
        join.join().unwrap();
    }
    let recovered = SocketActivationStore::open(&path).unwrap();
    let audit = recovered.global_audit_history("admin", 0, 10).unwrap();
    assert_eq!(audit.len(), 3);
    assert_ne!(audit[1].value, audit[2].value);
    assert_eq!(
        Some(recovered.resolve_persisted("a", None).unwrap().enabled),
        audit[2].value
    );
}

#[test]
fn version_two_global_migration_is_atomic_and_preserves_existing_binding() {
    let dir = TestDirectory::new();
    let path = dir.0.join("legacy.db");
    let c = rusqlite::Connection::open(&path).unwrap();
    c.execute_batch("CREATE TABLE matter_schema_versions(component TEXT PRIMARY KEY,version INTEGER NOT NULL);
        INSERT INTO matter_schema_versions VALUES('socket_activation',2);
        CREATE TABLE activation_sockets(socket_id TEXT PRIMARY KEY,owner_id TEXT NOT NULL,powered INTEGER NOT NULL,policy INTEGER);
        INSERT INTO activation_sockets VALUES('legacy','alice',1,1);
        CREATE TABLE activation_bindings(socket_id TEXT PRIMARY KEY,bot_instance_id TEXT NOT NULL);
        INSERT INTO activation_bindings VALUES('legacy','original-bot');").unwrap();
    // A conflicting object forces the new audit-table DDL to fail.
    c.execute_batch("CREATE INDEX activation_global_audit ON activation_sockets(owner_id);")
        .unwrap();
    assert!(SocketActivationStore::open(&path).is_err());
    let version: i64 = c
        .query_row(
            "SELECT version FROM matter_schema_versions WHERE component='socket_activation'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(version, 2);
    c.execute_batch("DROP INDEX activation_global_audit;")
        .unwrap();
    let store = SocketActivationStore::open(&path).unwrap();
    assert!(store.resolve_persisted("legacy", None).unwrap().enabled);
    let binding: String = c
        .query_row(
            "SELECT bot_instance_id FROM activation_bindings WHERE socket_id='legacy'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(binding, "original-bot");
    assert_eq!(
        store.resolve_persisted("legacy", None).unwrap().source,
        ActivationSource::Socket
    );
}

#[test]
fn ownership_and_binding_release_roll_back_all_state_on_audit_failure() {
    let dir = TestDirectory::new();
    let path = dir.0.join("transfer.db");
    let store = SocketActivationStore::open(&path).unwrap();
    let bots = matter_base::BotRegistry::in_memory();
    let bot = bots.register("f", "r", "a", "alice").unwrap();
    store.provision("a", "alice").unwrap();
    store
        .bind_bot("a", "alice", bot.instance_id(), &bots)
        .unwrap();
    store.set_power("a", "alice", true).unwrap();
    store.set_socket_policy("a", "alice", Some(false)).unwrap();
    store
        .set_group_policy("a", "alice", "room", Some(true))
        .unwrap();
    let c = rusqlite::Connection::open(&path).unwrap();
    c.execute_batch("CREATE TRIGGER reject_transfer_audit BEFORE INSERT ON activation_audit
        WHEN NEW.action IN ('transfer_owner','release_binding') BEGIN SELECT RAISE(ABORT,'injected'); END;").unwrap();
    assert!(store.transfer_owner("a", "alice", "bob").is_err());
    assert!(store.release_binding("a", "alice").is_err());
    assert!(store.resolve("a", Some("room"), None).unwrap().enabled);
    assert_eq!(
        store.resolve("a", None, Some(true)).unwrap().source,
        ActivationSource::Socket
    );
    let binding: String = c
        .query_row(
            "SELECT bot_instance_id FROM activation_bindings WHERE socket_id='a'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(binding, bot.instance_id());
    assert!(store.audit_history("a", "bob").is_err());
    assert_eq!(store.audit_history("a", "alice").unwrap().len(), 5);
    assert!(store.transfer_owner("a", "global-owner", "bob").is_err());
    assert!(store.release_binding("a", "global-owner").is_err());
    c.execute_batch("DROP TRIGGER reject_transfer_audit;")
        .unwrap();
    store.transfer_owner("a", "alice", "alice").unwrap();
    assert_eq!(store.audit_history("a", "alice").unwrap().len(), 5);
    assert!(store.resolve("a", Some("room"), None).unwrap().enabled);
    store.release_binding("a", "alice").unwrap();
    store.release_binding("a", "alice").unwrap();
    assert_eq!(store.audit_history("a", "alice").unwrap().len(), 6);
    assert!(
        !store
            .resolve("a", Some("room"), Some(true))
            .unwrap()
            .enabled
    );
    store.set_power("a", "alice", true).unwrap();
    assert_eq!(
        store.resolve("a", Some("room"), None).unwrap().source,
        ActivationSource::None
    );
}

#[test]
fn competing_socket_transfers_have_one_winner_and_survive_reopening() {
    let dir = TestDirectory::new();
    let path = dir.0.join("transfer.db");
    let first = SocketActivationStore::open(&path).unwrap();
    first.provision("a", "alice").unwrap();
    first.set_power("a", "alice", true).unwrap();
    let second = SocketActivationStore::open(&path).unwrap();
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let joins: Vec<_> = [(first, "bob"), (second, "carol")]
        .into_iter()
        .map(|(store, target)| {
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                (target, store.transfer_owner("a", "alice", target).is_ok())
            })
        })
        .collect();
    let results: Vec<_> = joins.into_iter().map(|join| join.join().unwrap()).collect();
    assert_eq!(results.iter().filter(|(_, success)| *success).count(), 1);
    let owner = results.iter().find(|(_, success)| *success).unwrap().0;
    let store = SocketActivationStore::open(&path).unwrap();
    assert!(store.set_power("a", "alice", true).is_err());
    assert!(store.audit_history("a", "alice").is_err());
    assert!(!store.resolve("a", None, Some(true)).unwrap().enabled);
    let audit = store.audit_history("a", owner).unwrap();
    assert_eq!(audit.len(), 3);
    assert_eq!(audit[2].actor_id, "alice");
    assert_eq!(audit[2].target_id.as_deref(), Some(owner));
    assert_eq!(audit[2].action, "transfer_owner");
    store.set_power("a", owner, true).unwrap();
}

#[test]
fn version_three_audit_upgrade_preserves_legacy_rows_without_inventing_targets() {
    let dir = TestDirectory::new();
    let path = dir.0.join("legacy.db");
    let c = rusqlite::Connection::open(&path).unwrap();
    c.execute_batch("CREATE TABLE matter_schema_versions(component TEXT PRIMARY KEY,version INTEGER NOT NULL);
        INSERT INTO matter_schema_versions VALUES('socket_activation',3);
        CREATE TABLE activation_sockets(socket_id TEXT PRIMARY KEY,owner_id TEXT NOT NULL,powered INTEGER NOT NULL,policy INTEGER);
        INSERT INTO activation_sockets VALUES('legacy','alice',1,1);
        CREATE TABLE activation_audit(id INTEGER PRIMARY KEY AUTOINCREMENT,socket_id TEXT NOT NULL,
        actor_id TEXT NOT NULL,action TEXT NOT NULL,group_id TEXT,value INTEGER,created_at INTEGER NOT NULL);
        INSERT INTO activation_audit(socket_id,actor_id,action,created_at) VALUES('legacy','alice','bind_bot',123);").unwrap();
    let store = SocketActivationStore::open(&path).unwrap();
    let rows = store.audit_history("legacy", "alice").unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].actor_id, "alice");
    assert_eq!(rows[0].created_at, 123);
    assert_eq!(rows[0].target_id, None);
    store.transfer_owner("legacy", "alice", "bob").unwrap();
    let rows = store.audit_history("legacy", "bob").unwrap();
    assert_eq!(rows[1].target_id.as_deref(), Some("bob"));
    assert_eq!(rows[1].value, Some(false));
    let version: i64 = c
        .query_row(
            "SELECT version FROM matter_schema_versions WHERE component='socket_activation'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(version, 4);
}
