/* Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::Result;
use matter_base::{
    activation::SocketActivationStore,
    control::{ControlDispatcher, DispatchResult},
    BotRegistry, CommandAuthorizer, CommandInvocation, CommandRouter,
};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
struct TestPolicy;
impl CommandAuthorizer for TestPolicy {
    fn authorize(&self, _: &CommandInvocation) -> Result<()> {
        Ok(())
    }
}
struct Temp(std::path::PathBuf);
impl Temp {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "matter-control-{}-{}",
            std::process::id(),
            time::OffsetDateTime::now_utc().unix_timestamp_nanos()
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn control_precedes_router_and_off_blocks_handler_effects_after_recovery() {
    let dir = Temp::new();
    let bots = Arc::new(BotRegistry::open_sqlite(dir.0.join("bots.db")).unwrap());
    let a = bots.register("f", "r", "a", "alice").unwrap();
    let b = bots.register("f", "r", "b", "bob").unwrap();
    let path = dir.0.join("activation.db");
    {
        let activation = SocketActivationStore::open(&path).unwrap();
        activation.provision("socket-a", "alice").unwrap();
        activation.provision("socket-b", "bob").unwrap();
        assert!(activation
            .bind_bot("socket-a", "alice", b.instance_id(), &bots)
            .is_err());
        activation
            .bind_bot("socket-a", "alice", a.instance_id(), &bots)
            .unwrap();
        activation
            .bind_bot("socket-b", "bob", b.instance_id(), &bots)
            .unwrap();
    }
    let activation = Arc::new(SocketActivationStore::open(&path).unwrap());
    let router = Arc::new(
        CommandRouter::new(
            vec!["#".into(), "=".into(), "/>".into()],
            Arc::new(TestPolicy),
        )
        .unwrap(),
    );
    let calls = Arc::new(AtomicUsize::new(0));
    for name in ["help", "menu", "on", "unknown"] {
        let calls = calls.clone();
        router
            .register(name, &[], move |inv| {
                calls.fetch_add(1, Ordering::SeqCst);
                Ok(inv.bot_instance_id.clone())
            })
            .unwrap();
    }
    let dispatcher = ControlDispatcher::new(activation.clone(), bots.clone(), router, Some(true));
    assert_eq!(
        dispatcher
            .dispatch("socket-a", "member", "room", None, "#help")
            .unwrap(),
        DispatchResult::Inactive
    );
    assert!(dispatcher
        .dispatch("socket-a", "bob", "room", None, "=on")
        .is_err());
    assert!(dispatcher
        .dispatch("socket-a", "global-owner", "room", None, "=on socket-b")
        .is_err());
    assert!(dispatcher
        .dispatch("socket-a", "alice", "room", None, "=on socket-b")
        .is_err());
    assert!(dispatcher
        .dispatch("socket-a", "alice", "room", None, "=unknown")
        .is_err());
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert_eq!(
        dispatcher
            .dispatch("socket-a", "alice", "room", None, "=/>on")
            .unwrap(),
        DispatchResult::Control {
            socket_id: "socket-a".into(),
            powered: true
        }
    );
    assert_eq!(
        dispatcher
            .dispatch("socket-a", "member", "room", None, "#menu")
            .unwrap(),
        DispatchResult::Message(Some(a.instance_id().into()))
    );
    dispatcher
        .dispatch("socket-a", "alice", "room", None, "/>off")
        .unwrap();
    assert_eq!(
        dispatcher
            .dispatch("socket-a", "member", "room", None, "#help")
            .unwrap(),
        DispatchResult::Inactive
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(
        !activation
            .resolve("socket-b", None, Some(true))
            .unwrap()
            .enabled
    );
    assert_eq!(
        activation.audit_history("socket-a", "alice").unwrap().len(),
        4
    );
    bots.transfer_owner(a.instance_id(), "alice", "new-owner")
        .unwrap();
    assert!(dispatcher
        .dispatch("socket-a", "alice", "room", None, "=on")
        .is_err());
}

#[test]
fn schema_v1_upgrade_keeps_existing_power_and_requires_explicit_binding() {
    let dir = Temp::new();
    let path = dir.0.join("v1.db");
    let c = rusqlite::Connection::open(&path).unwrap();
    c.execute_batch("CREATE TABLE matter_schema_versions(component TEXT PRIMARY KEY,version INTEGER NOT NULL); INSERT INTO matter_schema_versions VALUES('socket_activation',1); CREATE TABLE activation_sockets(socket_id TEXT PRIMARY KEY,owner_id TEXT NOT NULL,powered INTEGER NOT NULL,policy INTEGER); INSERT INTO activation_sockets VALUES('old','alice',1,1);").unwrap();
    let store = SocketActivationStore::open(&path).unwrap();
    assert!(store.resolve("old", None, None).unwrap().enabled);
    let version: i64 = c
        .query_row(
            "SELECT version FROM matter_schema_versions WHERE component='socket_activation'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(version, 2);
    let count: i64 = c
        .query_row("SELECT COUNT(*) FROM activation_bindings", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 0);
}
