/*
 * Base-Matter — Vault status is owner-only, read-only and disabled by default.
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Result};
use matter_base::{
    activation::SocketActivationStore,
    control::{ControlDispatcher, DispatchResult},
    BotRegistry, CommandAuthorizer, CommandInvocation, CommandRouter, ControlExtensionRegistry,
};
use std::sync::Arc;
struct NoBot;
impl CommandAuthorizer for NoBot {
    fn authorize(&self, _: &CommandInvocation) -> Result<()> {
        bail!("bot_not_active")
    }
}
fn dispatcher() -> ControlDispatcher {
    let activation = Arc::new(SocketActivationStore::open(":memory:").unwrap());
    activation.provision("socket-test", "owner").unwrap();
    let router = Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(NoBot)).unwrap());
    let extensions = Arc::new(ControlExtensionRegistry::new());
    extensions
        .register("vault", &[], |ctx, args| {
            if ctx.group_id.is_some() || ctx.source_id != "self@s.whatsapp.net" {
                bail!("not_self_chat");
            }
            if !args.is_empty() {
                bail!("unexpected_arguments");
            }
            Ok("Vault-Matter: servicio no configurado".to_owned())
        })
        .unwrap();
    ControlDispatcher::new(activation, Arc::new(BotRegistry::in_memory()), router, None)
        .with_control_extensions(extensions)
        .with_unified_live_controls()
        .with_silent_bot_commands()
}
#[test]
fn vault_status_is_private_and_available_when_bot_off() {
    let c = dispatcher();
    let status = c
        .dispatch(
            "socket-test",
            "owner",
            "self@s.whatsapp.net",
            None,
            "=/>vault"
        )
        .unwrap();
    match status {
        DispatchResult::Extension { command, output } => {
            assert_eq!(command, "vault");
            assert!(output.contains("no configurado"));
        }
        other => panic!("unexpected dispatch result: {other:?}"),
    }
    assert!(c
        .dispatch(
            "socket-test",
            "attacker",
            "self@s.whatsapp.net",
            None,
            "=/>vault"
        )
        .is_err());
    assert!(c
        .dispatch(
            "socket-test",
            "owner",
            "other@s.whatsapp.net",
            None,
            "=/>vault"
        )
        .is_err());
    assert!(c
        .dispatch(
            "socket-test",
            "owner",
            "1203635@g.us",
            Some("1203635@g.us"),
            "=/>vault"
        )
        .is_err());
    assert!(c
        .dispatch(
            "socket-test",
            "owner",
            "self@s.whatsapp.net",
            None,
            "=vault"
        )
        .is_err());
}
