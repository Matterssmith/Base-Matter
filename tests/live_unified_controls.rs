/*
 * Base-Matter — unified on/off for the live WhatsApp controller.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Result};
use matter_base::{
    activation::{ActivationSource, SocketActivationStore},
    control::{ControlDispatcher, DispatchResult},
    BotRegistry, CommandAuthorizer, CommandInvocation, CommandRouter,
};
use std::sync::Arc;

struct DenyBot;
impl CommandAuthorizer for DenyBot {
    fn authorize(&self, _: &CommandInvocation) -> Result<()> {
        bail!("Bot-Matter must remain unavailable")
    }
}

fn fixture() -> (ControlDispatcher, Arc<SocketActivationStore>) {
    let store = Arc::new(SocketActivationStore::open(":memory:").unwrap());
    store.provision("socket-alpha", "owner").unwrap();
    let router =
        Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(DenyBot)).unwrap());
    let control = ControlDispatcher::new(
        Arc::clone(&store),
        Arc::new(BotRegistry::in_memory()),
        router,
        None,
    )
    .with_unified_live_controls()
    .with_silent_bot_commands();
    (control, store)
}

fn command(
    control: &ControlDispatcher,
    group: Option<&str>,
    text: &str,
) -> Result<DispatchResult> {
    control.dispatch(
        "socket-alpha",
        "owner",
        group.unwrap_or("owner-self"),
        group,
        text,
    )
}

#[test]
fn one_on_command_enables_bot_and_wakes_socket_atomically() {
    let (control, store) = fixture();
    assert_eq!(
        command(&control, None, "=/>status").unwrap(),
        DispatchResult::Status {
            socket_id: "socket-alpha".into(),
            enabled: false,
            source: ActivationSource::PoweredOff,
            bot_instance_id: None,
        },
    );
    assert_eq!(
        command(&control, None, "=/>bot on").unwrap(),
        DispatchResult::BotPolicyChanged {
            scope: "current".into(),
            policy: Some(true),
        },
    );
    assert!(store
        .inspect_bot_defaults("socket-alpha", "owner")
        .unwrap()
        .socket_powered);
    assert!(matches!(
        command(&control, None, "=/>status").unwrap(),
        DispatchResult::Status {
            enabled: true,
            source: ActivationSource::Group,
            ..
        },
    ));
    assert_eq!(
        command(&control, None, "=/>bot on").unwrap(),
        DispatchResult::BotPolicyChanged {
            scope: "current".into(),
            policy: Some(true),
        }
    );
    assert_eq!(
        command(&control, None, "=/>bot off").unwrap(),
        DispatchResult::BotPolicyChanged {
            scope: "current".into(),
            policy: Some(false),
        }
    );
    assert!(store
        .inspect_bot_defaults("socket-alpha", "owner")
        .unwrap()
        .socket_powered);
    assert!(matches!(
        command(&control, None, "=/>status").unwrap(),
        DispatchResult::Status {
            enabled: false,
            source: ActivationSource::Group,
            ..
        }
    ));
    command(&control, None, "=/>bot on").unwrap();
    assert!(matches!(
        command(&control, None, "=/>status").unwrap(),
        DispatchResult::Status { enabled: true, .. },
    ));
}

#[test]
fn group_switch_does_not_disable_another_group() {
    let (control, store) = fixture();
    let a = Some("120363111@g.us");
    let b = Some("120363222@g.us");
    assert_eq!(
        command(&control, a, "=/>bot on").unwrap(),
        DispatchResult::BotPolicyChanged {
            scope: "current".into(),
            policy: Some(true),
        },
    );
    assert!(store
        .inspect_bot_defaults("socket-alpha", "owner")
        .unwrap()
        .socket_powered);
    assert!(matches!(
        command(&control, a, "=/>status").unwrap(),
        DispatchResult::GroupStatus {
            enabled: true,
            source: ActivationSource::Group,
            ..
        },
    ));
    assert!(matches!(
        command(&control, b, "=/>status").unwrap(),
        DispatchResult::GroupStatus {
            enabled: false,
            source: ActivationSource::None,
            ..
        },
    ));
    command(&control, b, "=/>bot on").unwrap();
    command(&control, a, "=/>bot off").unwrap();
    assert!(matches!(
        command(&control, a, "=/>status").unwrap(),
        DispatchResult::GroupStatus { enabled: false, .. },
    ));
    assert!(matches!(
        command(&control, b, "=/>status").unwrap(),
        DispatchResult::GroupStatus { enabled: true, .. },
    ));
    assert!(command(&control, a, "=/>bot --global on").is_err());
    assert!(control
        .dispatch("socket-alpha", "outsider", "owner-self", None, "=/>bot on")
        .is_err());
    assert!(command(&control, None, "=/>on").is_err());
    assert!(command(&control, a, "=/>off").is_err());
}

#[test]
fn socket_wide_bot_on_alone_wakes_the_self_chat_not_all_external_chats() {
    let (control, store) = fixture();
    assert_eq!(
        command(&control, None, "=/>bot --global on").unwrap(),
        DispatchResult::BotPolicyChanged {
            scope: "socket-global".into(),
            policy: Some(true),
        },
    );
    assert!(store.resolve_persisted("socket-alpha", None).unwrap().enabled);
    assert!(matches!(
        command(&control, None, "=/>status").unwrap(),
        DispatchResult::Status { enabled: true, .. },
    ));
    let group = Some("120363333@g.us");
    assert!(matches!(
        command(&control, group, "=/>status").unwrap(),
        DispatchResult::GroupStatus {
            enabled: false,
            source: ActivationSource::None,
            ..
        },
    ));
    command(&control, None, "=/>bot group on").unwrap();
    assert!(matches!(
        command(&control, group, "=/>status").unwrap(),
        DispatchResult::GroupStatus { enabled: true, .. },
    ));
}

#[test]
fn community_announcement_policy_is_configurable_but_requires_verified_platform_type() {
    let (control, store) = fixture();
    assert_eq!(
        command(&control, None, "=/>bot comunidad on").unwrap(),
        DispatchResult::BotPolicyChanged {
            scope: "community".into(),
            policy: Some(true),
        }
    );
    assert!(matches!(
        command(&control, None, "=/>status --global").unwrap(),
        DispatchResult::BotGlobalStatus {
            community_policy: Some(true),
            group_policy: None,
            channel_policy: None,
            ..
        }
    ));
    // A WhatsApp announcement group still arrives as an ordinary group
    // until authenticated community metadata is available. The new
    // policy must not silently activate arbitrary groups.
    assert!(matches!(
        command(&control, Some("120363999@g.us"), "=/>status").unwrap(),
        DispatchResult::GroupStatus {
            enabled: false,
            source: ActivationSource::None,
            ..
        }
    ));
    let defaults = store.inspect_bot_defaults("socket-alpha", "owner").unwrap();
    assert_eq!(defaults.community_policy, Some(true));
    assert_eq!(defaults.group_policy, None);
    assert_eq!(defaults.channel_policy, None);
}
