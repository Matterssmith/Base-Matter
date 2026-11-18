/*
 * Base-Matter — chat-level bot policy and silent owner command routing.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Result};
use matter_base::{
    activation::{ActivationSource, SocketActivationStore},
    control::{ControlDispatcher, DispatchResult},
    BotRegistry, CommandAuthorizer, CommandInvocation, CommandRouter, ControlExtensionRegistry,
};
use std::sync::Arc;

struct DenyBot;
impl CommandAuthorizer for DenyBot {
    fn authorize(&self, _: &CommandInvocation) -> Result<()> {
        bail!("Bot-Matter remains blocked")
    }
}

fn fixture() -> (ControlDispatcher, Arc<SocketActivationStore>) {
    let activation = Arc::new(SocketActivationStore::open(":memory:").unwrap());
    activation.provision("wa-alpha", "owner").unwrap();
    let commands = Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(DenyBot)).unwrap());
    let extensions = Arc::new(ControlExtensionRegistry::new());
    extensions
        .register("menu", &[], |_, _| Ok("menu test".to_owned()))
        .unwrap();
    (
        ControlDispatcher::new(
            Arc::clone(&activation),
            Arc::new(BotRegistry::in_memory()),
            commands,
            None,
        )
        .with_control_extensions(extensions)
        .with_silent_bot_commands(),
        activation,
    )
}

fn dispatch(c: &ControlDispatcher, group: Option<&str>, command: &str) -> Result<DispatchResult> {
    c.dispatch("wa-alpha", "owner", group.unwrap_or("self"), group, command)
}

#[test]
fn status_works_while_off_and_global_defaults_are_not_masked_by_power() {
    let (c, _store) = fixture();
    let group = Some("120363555@g.us");
    assert_eq!(
        dispatch(&c, group, "=/>status").unwrap(),
        DispatchResult::GroupStatus {
            group_id: "120363555@g.us".into(),
            enabled: false,
            source: ActivationSource::PoweredOff,
        },
    );
    assert_eq!(
        dispatch(&c, None, "=/>bot --global on").unwrap(),
        DispatchResult::BotPolicyChanged {
            scope: "socket-global".into(),
            policy: Some(true),
        },
    );
    assert_eq!(
        dispatch(&c, group, "=/>status --global").unwrap(),
        DispatchResult::BotGlobalStatus {
            socket_powered: false,
            default_policy: Some(true),
            group_policy: None,
            private_policy: None,
            channel_policy: None,
            community_policy: None,
        },
    );
    assert_eq!(
        dispatch(&c, group, "=/>menu").unwrap(),
        DispatchResult::Inactive,
    );
    dispatch(&c, None, "=/>on").unwrap();
    // Global fallback does not silently enable all external group chats.
    assert_eq!(
        dispatch(&c, group, "=/>menu").unwrap(),
        DispatchResult::Inactive
    );
    dispatch(&c, None, "=/>bot group on").unwrap();
    assert!(matches!(
        dispatch(&c, group, "=/>menu").unwrap(),
        DispatchResult::Extension { .. }
    ));
    assert!(matches!(
        dispatch(&c, None, "=/>menu").unwrap(),
        DispatchResult::Extension { .. }
    ));
}

#[test]
fn kind_defaults_and_exact_group_overrides_inherit_correctly() {
    let (c, _) = fixture();
    let group = Some("120363555@g.us");
    dispatch(&c, None, "=/>on").unwrap();
    // Owner self-chat inherits the socket global, never external private.
    dispatch(&c, None, "=/>bot --global on").unwrap();
    assert_eq!(
        dispatch(&c, None, "=/>bot group off").unwrap(),
        DispatchResult::BotPolicyChanged {
            scope: "group".into(),
            policy: Some(false),
        }
    );
    assert_eq!(
        dispatch(&c, group, "=/>menu").unwrap(),
        DispatchResult::Inactive
    );
    assert!(matches!(
        dispatch(&c, group, "=/>status").unwrap(),
        DispatchResult::GroupStatus {
            enabled: false,
            source: ActivationSource::Group,
            ..
        }
    ));
    dispatch(&c, None, "=/>bot grupo encender").unwrap();
    assert!(matches!(
        dispatch(&c, group, "=/>menu").unwrap(),
        DispatchResult::Extension { .. }
    ));
    dispatch(&c, group, "=/>bot off").unwrap();
    assert_eq!(
        dispatch(&c, group, "=/>menu").unwrap(),
        DispatchResult::Inactive,
    );
    dispatch(&c, group, "=/>bot inherit").unwrap();
    assert!(matches!(
        dispatch(&c, group, "=/>menu").unwrap(),
        DispatchResult::Extension { .. }
    ));
    dispatch(&c, None, "=/>bot private off").unwrap();
    // Disabling *external* private chats does not disable the
    // owner's own configuration chat.
    assert!(matches!(
        dispatch(&c, None, "=/>menu").unwrap(),
        DispatchResult::Extension { .. }
    ));
    assert!(matches!(
        dispatch(&c, None, "=/>status").unwrap(),
        DispatchResult::Status {
            enabled: true,
            source: ActivationSource::Socket,
            ..
        }
    ));
    assert!(matches!(
        dispatch(&c, None, "=/>status --global").unwrap(),
        DispatchResult::BotGlobalStatus {
            private_policy: Some(false),
            ..
        }
    ));
    dispatch(&c, None, "=/>bot off").unwrap();
    assert_eq!(
        dispatch(&c, None, "=/>menu").unwrap(),
        DispatchResult::Inactive
    );
    dispatch(&c, None, "=/>bot inherit").unwrap();
    assert!(matches!(
        dispatch(&c, None, "=/>menu").unwrap(),
        DispatchResult::Extension { .. }
    ));
    assert!(dispatch(&c, group, "=/>bot group on").is_err());
    assert!(dispatch(&c, None, "=/>bot --global on").is_ok());
}

#[test]
fn debug_cannot_grant_roles_or_disclose_secrets() {
    let (c, _store) = fixture();
    assert!(dispatch(&c, None, "=/>set role pn:5215512345678 admin --debug").is_err());
    let result = dispatch(&c, None, "=/>status --debug").unwrap();
    let DispatchResult::Extension { command, output } = result else {
        panic!("expected opt-in debug result");
    };
    assert_eq!(command, "debug");
    assert!(output.contains("[DEBUG]"));
    assert!(output.contains("secrets") || output.contains("secretos=ocultos"));
    assert!(!output.contains("5215512345678"));
    assert!(!output.contains("MATTER_WA_BRIDGE_KEY_HEX"));
    assert!(dispatch(&c, None, "=/>bot --global off --debug").is_ok());
    assert!(dispatch(&c, None, "=/>bot --global on --debug").is_ok());
    assert!(dispatch(&c, None, "=/>status --global").is_ok());
    assert!(c
        .dispatch("wa-alpha", "stranger", "self", None, "=/>bot on --debug")
        .is_err());
}
