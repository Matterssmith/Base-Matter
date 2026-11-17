/*
 * Base-Matter — REGISTER start requires the receiving socket owner.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::Result;
use matter_base::{
    activation::SocketActivationStore,
    control::{ControlDispatcher, DispatchResult},
    BotRegistry, CommandAuthorizer, CommandInvocation, CommandRouter, ControlExtensionRegistry,
};
use std::sync::Arc;

struct DenyBot;
impl CommandAuthorizer for DenyBot {
    fn authorize(&self, _: &CommandInvocation) -> Result<()> {
        anyhow::bail!("Bot-Matter cannot execute")
    }
}

#[test]
fn start_is_read_only_owner_only_and_independent_of_bot_power() {
    let activation = Arc::new(SocketActivationStore::open(":memory:").unwrap());
    activation.provision("socket-a", "alice").unwrap();
    activation.provision("socket-b", "bob").unwrap();
    let router = Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(DenyBot)).unwrap());
    let extensions = Arc::new(ControlExtensionRegistry::new());
    extensions
        .register("start", &[], |_, arguments| {
            if !arguments.is_empty() {
                anyhow::bail!("Changes prohibited")
            }
            Ok("Discovery only".to_owned())
        })
        .unwrap();
    let control = ControlDispatcher::new(
        Arc::clone(&activation),
        Arc::new(BotRegistry::in_memory()),
        router,
        None,
    )
    .with_control_extensions(extensions);
    assert_eq!(
        control
            .dispatch("socket-a", "alice", "private", None, "=/>start")
            .unwrap(),
        DispatchResult::Extension {
            command: "start".to_owned(),
            output: "Discovery only".to_owned()
        }
    );
    for (socket, actor, command) in [
        ("socket-a", "bob", "=/>start"),
        ("socket-b", "alice", "=/>start"),
        ("socket-a", "alice", "=/>start apply unknown"),
        ("socket-a", "alice", "=start"),
    ] {
        assert!(control
            .dispatch(socket, actor, "private", None, command)
            .is_err());
    }
    assert!(!activation.resolve_persisted("socket-a", None).unwrap().enabled);
}
