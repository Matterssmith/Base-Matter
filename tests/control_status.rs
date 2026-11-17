/*
 * Base-Matter — status inspection via reserved global control.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::Result;
use matter_base::{
    activation::{ActivationSource, SocketActivationStore},
    control::{ControlDispatcher, DispatchResult},
    BotRegistry, CommandAuthorizer, CommandInvocation, CommandRouter,
};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

struct Permit;
impl CommandAuthorizer for Permit {
    fn authorize(&self, _: &CommandInvocation) -> Result<()> {
        Ok(())
    }
}

#[test]
fn global_status_works_when_socket_and_bot_are_off_without_binding() {
    let store = Arc::new(SocketActivationStore::open(":memory:").unwrap());
    store.provision("socket-a", "alice").unwrap();
    let bots = Arc::new(BotRegistry::in_memory());
    let calls = Arc::new(AtomicUsize::new(0));
    let commands = Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(Permit)).unwrap());
    let hits = Arc::clone(&calls);
    commands
        .register("status", &[], move |_| {
            hits.fetch_add(1, Ordering::SeqCst);
            Ok("unexpected bot command".into())
        })
        .unwrap();
    let dispatcher = ControlDispatcher::new(store.clone(), bots, commands, Some(true));

    assert_eq!(
        dispatcher
            .dispatch("socket-a", "alice", "room", None, "=/>status")
            .unwrap(),
        DispatchResult::Status {
            socket_id: "socket-a".into(),
            enabled: false,
            source: ActivationSource::PoweredOff,
            bot_instance_id: None,
        }
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);

    dispatcher
        .dispatch("socket-a", "alice", "room", None, "=/>on")
        .unwrap();
    assert_eq!(
        dispatcher
            .dispatch("socket-a", "alice", "room", None, "=/>status")
            .unwrap(),
        DispatchResult::Status {
            socket_id: "socket-a".into(),
            enabled: true,
            source: ActivationSource::Global,
            bot_instance_id: None,
        }
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
}

#[test]
fn only_authenticated_socket_owner_can_inspect_status_or_target_another_socket() {
    let store = Arc::new(SocketActivationStore::open(":memory:").unwrap());
    store.provision("socket-a", "alice").unwrap();
    store.provision("socket-b", "bob").unwrap();
    let bots = Arc::new(BotRegistry::in_memory());
    let commands = Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(Permit)).unwrap());
    let dispatcher = ControlDispatcher::new(store, bots, commands, None);

    for statement in ["=/>status", "=/>status socket-a"] {
        assert!(dispatcher
            .dispatch("socket-a", "bob", "room", None, statement)
            .is_err());
    }
    assert!(dispatcher
        .dispatch("socket-a", "alice", "room", None, "=/>status socket-b")
        .is_err());
    assert!(dispatcher
        .dispatch(
            "socket-a",
            "alice",
            "room",
            None,
            "=/>status socket-b extra"
        )
        .is_err());
    assert_eq!(
        dispatcher
            .dispatch("socket-a", "bob", "room", None, "=/>status socket-b")
            .unwrap(),
        DispatchResult::Status {
            socket_id: "socket-b".into(),
            enabled: false,
            source: ActivationSource::PoweredOff,
            bot_instance_id: None,
        }
    );
}

#[test]
fn status_reports_binding_without_dispatching_the_bot_or_exposing_other_owners() {
    let store = Arc::new(SocketActivationStore::open(":memory:").unwrap());
    store.provision("socket-a", "alice").unwrap();
    let bots = Arc::new(BotRegistry::in_memory());
    let bot = bots.register("family", "v1", "test", "alice").unwrap();
    store
        .bind_bot("socket-a", "alice", bot.instance_id(), &bots)
        .unwrap();
    let commands = Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(Permit)).unwrap());
    let dispatcher = ControlDispatcher::new(store.clone(), bots, commands, None);

    assert_eq!(
        dispatcher
            .dispatch("socket-a", "alice", "room", None, "=/>status")
            .unwrap(),
        DispatchResult::Status {
            socket_id: "socket-a".into(),
            enabled: false,
            source: ActivationSource::PoweredOff,
            bot_instance_id: Some(bot.instance_id().into()),
        }
    );
    assert!(dispatcher
        .dispatch("socket-a", "mallory", "room", None, "=/>status")
        .is_err());
    assert!(dispatcher
        .dispatch("socket-a", "alice", "room", None, "=/>ping")
        .is_err());
}
