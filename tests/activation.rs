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
