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
    dispatcher
        .dispatch("socket-a", "alice", "room", None, "=on")
        .unwrap();
    // Bot ownership revocation blocks normal execution, but cannot prevent the
    // socket owner from switching off its independently owned operational power.
    assert!(dispatcher
        .dispatch("socket-a", "member", "room", None, "#help")
        .is_err());
    assert!(dispatcher
        .dispatch("socket-a", "new-owner", "room", None, "=off")
        .is_err());
    dispatcher
        .dispatch("socket-a", "alice", "room", None, "=off")
        .unwrap();
    assert_eq!(
        dispatcher
            .dispatch("socket-a", "member", "room", None, "#help")
            .unwrap(),
        DispatchResult::Inactive
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[test]
fn power_control_does_not_require_a_bot_binding_or_network_connection() {
    let dir = Temp::new();
    let path = dir.0.join("activation.db");
    let activation = Arc::new(SocketActivationStore::open(&path).unwrap());
    activation.provision("unbound", "alice").unwrap();
    let bots = Arc::new(BotRegistry::in_memory());
    let router = Arc::new(CommandRouter::new(vec!["=".into()], Arc::new(TestPolicy)).unwrap());
    let calls = Arc::new(AtomicUsize::new(0));
    let handler_calls = calls.clone();
    router
        .register("on", &[], move |_| {
            handler_calls.fetch_add(1, Ordering::SeqCst);
            Ok("must not execute".into())
        })
        .unwrap();
    let dispatcher = ControlDispatcher::new(activation.clone(), bots, router, Some(true));
    for prefix in ["=", "/>", "=/>"] {
        for invalid in ["", "unknown", "on unbound extra"] {
            assert!(dispatcher
                .dispatch(
                    "unbound",
                    "alice",
                    "room",
                    None,
                    &format!("{prefix}{invalid}")
                )
                .is_err());
        }
        assert!(dispatcher
            .dispatch(
                "unbound",
                "global-owner",
                "room",
                None,
                &format!("{prefix}on unbound")
            )
            .is_err());
        assert_eq!(
            dispatcher
                .dispatch(
                    "unbound",
                    "alice",
                    "room",
                    None,
                    &format!("{prefix}on unbound")
                )
                .unwrap(),
            DispatchResult::Control {
                socket_id: "unbound".into(),
                powered: true
            }
        );
        assert!(dispatcher
            .dispatch("unbound", "member", "room", None, "normal")
            .is_err());
        dispatcher
            .dispatch("unbound", "alice", "room", None, &format!("{prefix}off"))
            .unwrap();
        assert_eq!(
            dispatcher
                .dispatch("unbound", "member", "room", None, "normal")
                .unwrap(),
            DispatchResult::Inactive
        );
    }
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let recovered = SocketActivationStore::open(&path).unwrap();
    assert!(
        !recovered
            .resolve("unbound", None, Some(true))
            .unwrap()
            .enabled
    );
    assert_eq!(
        recovered.audit_history("unbound", "alice").unwrap().len(),
        7
    );
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
    assert_eq!(version, 4);
    let count: i64 = c
        .query_row("SELECT COUNT(*) FROM activation_bindings", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn persistent_global_changes_affect_admission_but_never_grant_power_authority() {
    let dir = Temp::new();
    let bots = Arc::new(BotRegistry::in_memory());
    let bot = bots.register("f", "r", "bot", "alice").unwrap();
    let path = dir.0.join("activation.db");
    let activation = Arc::new(SocketActivationStore::open(&path).unwrap());
    activation.provision_global("global-admin").unwrap();
    activation.provision("socket", "alice").unwrap();
    activation
        .bind_bot("socket", "alice", bot.instance_id(), &bots)
        .unwrap();
    let router = Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(TestPolicy)).unwrap());
    let calls = Arc::new(AtomicUsize::new(0));
    let handler_calls = calls.clone();
    router
        .register("help", &[], move |_| {
            handler_calls.fetch_add(1, Ordering::SeqCst);
            Ok("help".into())
        })
        .unwrap();
    let dispatcher =
        ControlDispatcher::new_with_persistent_policy(activation.clone(), bots, router);
    let separate = SocketActivationStore::open(&path).unwrap();
    separate
        .set_global_policy("global-admin", Some(true))
        .unwrap();
    assert!(dispatcher
        .dispatch("socket", "global-admin", "room", None, "=on socket")
        .is_err());
    assert_eq!(
        dispatcher
            .dispatch("socket", "member", "room", None, "#help")
            .unwrap(),
        DispatchResult::Inactive
    );
    dispatcher
        .dispatch("socket", "alice", "room", None, "=on")
        .unwrap();
    assert_eq!(
        dispatcher
            .dispatch("socket", "member", "room", None, "#help")
            .unwrap(),
        DispatchResult::Message(Some("help".into()))
    );
    separate
        .set_global_policy("global-admin", Some(false))
        .unwrap();
    assert_eq!(
        dispatcher
            .dispatch("socket", "member", "room", None, "#help")
            .unwrap(),
        DispatchResult::Inactive
    );
    separate
        .set_group_policy("socket", "alice", "room", Some(true))
        .unwrap();
    assert_eq!(
        dispatcher
            .dispatch("socket", "member", "room", Some("room"), "#help")
            .unwrap(),
        DispatchResult::Message(Some("help".into()))
    );
    dispatcher
        .dispatch("socket", "alice", "room", None, "=off")
        .unwrap();
    assert_eq!(
        dispatcher
            .dispatch("socket", "member", "room", Some("room"), "#help")
            .unwrap(),
        DispatchResult::Inactive
    );
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[test]
fn socket_handover_and_revoked_bot_binding_recovery_require_explicit_reactivation() {
    let dir = Temp::new();
    let bots = Arc::new(BotRegistry::open_sqlite(dir.0.join("bots.db")).unwrap());
    let a = bots.register("f", "r", "a", "alice").unwrap();
    let b = bots.register("f", "r", "b", "bob").unwrap();
    let c = bots.register("f", "r", "c", "bob").unwrap();
    let path = dir.0.join("activation.db");
    let activation = Arc::new(SocketActivationStore::open(&path).unwrap());
    for (socket, owner, bot) in [("a", "alice", &a), ("b", "bob", &b)] {
        activation.provision(socket, owner).unwrap();
        activation
            .bind_bot(socket, owner, bot.instance_id(), &bots)
            .unwrap();
        activation.set_power(socket, owner, true).unwrap();
        activation
            .set_socket_policy(socket, owner, Some(true))
            .unwrap();
    }
    activation
        .set_group_policy("a", "alice", "room", Some(true))
        .unwrap();
    let router = Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(TestPolicy)).unwrap());
    router
        .register("who", &[], |inv| Ok(inv.bot_instance_id.clone()))
        .unwrap();
    let dispatcher = ControlDispatcher::new(activation.clone(), bots.clone(), router, None);
    activation.transfer_owner("a", "alice", "bob").unwrap();
    assert_eq!(
        dispatcher
            .dispatch("a", "member", "room", Some("room"), "#who")
            .unwrap(),
        DispatchResult::Inactive
    );
    assert!(dispatcher
        .dispatch("a", "alice", "room", None, "=on")
        .is_err());
    assert!(activation
        .bind_bot("a", "bob", a.instance_id(), &bots)
        .is_err());
    activation
        .bind_bot("a", "bob", b.instance_id(), &bots)
        .unwrap();
    dispatcher
        .dispatch("a", "bob", "room", None, "=on")
        .unwrap();
    assert_eq!(
        dispatcher
            .dispatch("a", "member", "room", Some("room"), "#who")
            .unwrap(),
        DispatchResult::Inactive
    );
    activation
        .set_socket_policy("a", "bob", Some(true))
        .unwrap();
    assert_eq!(
        dispatcher
            .dispatch("a", "member", "room", None, "#who")
            .unwrap(),
        DispatchResult::Message(Some(b.instance_id().into()))
    );
    assert_eq!(
        dispatcher
            .dispatch("b", "member", "room", None, "#who")
            .unwrap(),
        DispatchResult::Message(Some(b.instance_id().into()))
    );
    bots.transfer_owner(b.instance_id(), "bob", "carol")
        .unwrap();
    assert!(dispatcher
        .dispatch("a", "member", "room", None, "#who")
        .is_err());
    assert!(activation.release_binding("a", "carol").is_err());
    activation.release_binding("a", "bob").unwrap();
    activation
        .bind_bot("a", "bob", c.instance_id(), &bots)
        .unwrap();
    assert_eq!(
        dispatcher
            .dispatch("a", "member", "room", None, "#who")
            .unwrap(),
        DispatchResult::Inactive
    );
    activation
        .set_socket_policy("a", "bob", Some(true))
        .unwrap();
    dispatcher
        .dispatch("a", "bob", "room", None, "=on")
        .unwrap();
    assert_eq!(
        dispatcher
            .dispatch("a", "member", "room", None, "#who")
            .unwrap(),
        DispatchResult::Message(Some(c.instance_id().into()))
    );
    let recovered = SocketActivationStore::open(&path).unwrap();
    let rows = recovered.audit_history("a", "bob").unwrap();
    let released = rows
        .iter()
        .find(|row| row.action == "release_binding")
        .unwrap();
    assert_eq!(released.target_id.as_deref(), Some(b.instance_id()));
    assert_eq!(released.actor_id, "bob");
}

#[test]
fn transfer_does_not_cancel_admitted_handler_or_admit_new_binding_while_off() {
    let dir = Temp::new();
    let bots = Arc::new(BotRegistry::in_memory());
    let a = bots.register("f", "r", "a", "alice").unwrap();
    let b = bots.register("f", "r", "b", "bob").unwrap();
    let activation = Arc::new(SocketActivationStore::open(dir.0.join("state.db")).unwrap());
    activation.provision("socket", "alice").unwrap();
    activation
        .bind_bot("socket", "alice", a.instance_id(), &bots)
        .unwrap();
    activation.set_power("socket", "alice", true).unwrap();
    let router = Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(TestPolicy)).unwrap());
    let (started_tx, started_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    let release_rx = std::sync::Mutex::new(release_rx);
    let original = a.instance_id().to_owned();
    router
        .register("who", &[], move |inv| {
            if inv.bot_instance_id == original {
                started_tx.send(()).unwrap();
                release_rx
                    .lock()
                    .unwrap()
                    .recv_timeout(std::time::Duration::from_secs(5))
                    .unwrap();
            }
            Ok(inv.bot_instance_id.clone())
        })
        .unwrap();
    let dispatcher = Arc::new(ControlDispatcher::new(
        activation.clone(),
        bots.clone(),
        router,
        Some(true),
    ));
    let first = dispatcher.clone();
    let handler = std::thread::spawn(move || {
        first
            .dispatch("socket", "member", "room", None, "#who")
            .unwrap()
    });
    started_rx
        .recv_timeout(std::time::Duration::from_secs(5))
        .unwrap();
    activation.transfer_owner("socket", "alice", "bob").unwrap();
    activation
        .bind_bot("socket", "bob", b.instance_id(), &bots)
        .unwrap();
    assert_eq!(
        dispatcher
            .dispatch("socket", "member", "room", None, "#who")
            .unwrap(),
        DispatchResult::Inactive
    );
    release_tx.send(()).unwrap();
    assert_eq!(
        handler.join().unwrap(),
        DispatchResult::Message(Some(a.instance_id().into()))
    );
    dispatcher
        .dispatch("socket", "bob", "room", None, "=on")
        .unwrap();
    assert_eq!(
        dispatcher
            .dispatch("socket", "member", "room", None, "#who")
            .unwrap(),
        DispatchResult::Message(Some(b.instance_id().into()))
    );
}
