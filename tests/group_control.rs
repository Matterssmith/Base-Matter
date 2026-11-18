/*
 * Base-Matter — authenticated group policy scope regressions.
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
        bail!("No Bot-Matter execution during Base-Matter group-control tests")
    }
}

fn fixture() -> (ControlDispatcher, Arc<SocketActivationStore>) {
    let activation = Arc::new(SocketActivationStore::open(":memory:").unwrap());
    activation.provision("wa-alpha", "socket-owner").unwrap();
    let router = Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(DenyBot)).unwrap());
    (
        ControlDispatcher::new(
            Arc::clone(&activation),
            Arc::new(BotRegistry::in_memory()),
            router,
            None,
        ),
        activation,
    )
}

#[test]
fn group_policy_is_independent_from_private_socket_power() {
    let (control, activation) = fixture();
    assert_eq!(
        control
            .dispatch(
                "wa-alpha",
                "socket-owner",
                "120363111@g.us",
                Some("120363111@g.us"),
                "=/>group on",
            )
            .unwrap(),
        DispatchResult::GroupPolicyChanged {
            group_id: "120363111@g.us".into(),
            policy: Some(true),
        }
    );
    // The group override does not power on an inactive socket.
    assert_eq!(
        control
            .dispatch(
                "wa-alpha",
                "socket-owner",
                "120363111@g.us",
                Some("120363111@g.us"),
                "=/>group status",
            )
            .unwrap(),
        DispatchResult::GroupStatus {
            group_id: "120363111@g.us".into(),
            enabled: false,
            source: ActivationSource::PoweredOff,
        }
    );
    control
        .dispatch("wa-alpha", "socket-owner", "self", None, "=/>on")
        .unwrap();
    assert!(
        activation
            .resolve("wa-alpha", Some("120363111@g.us"), None)
            .unwrap()
            .enabled
    );
    assert!(
        !activation
            .resolve("wa-alpha", Some("120363222@g.us"), None)
            .unwrap()
            .enabled
    );
    assert_eq!(
        control
            .dispatch(
                "wa-alpha",
                "socket-owner",
                "120363111@g.us",
                Some("120363111@g.us"),
                "=/>grupo apagar",
            )
            .unwrap(),
        DispatchResult::GroupPolicyChanged {
            group_id: "120363111@g.us".into(),
            policy: Some(false),
        }
    );
    assert!(
        !activation
            .resolve("wa-alpha", Some("120363111@g.us"), None)
            .unwrap()
            .enabled
    );
    assert_eq!(
        control
            .dispatch(
                "wa-alpha",
                "socket-owner",
                "120363111@g.us",
                Some("120363111@g.us"),
                "=/>group inherit",
            )
            .unwrap(),
        DispatchResult::GroupPolicyChanged {
            group_id: "120363111@g.us".into(),
            policy: None,
        }
    );
}

#[test]
fn group_control_rejects_nonowners_and_wrong_chat_scope() {
    let (control, _) = fixture();
    assert!(control
        .dispatch(
            "wa-alpha",
            "intruder",
            "120363111@g.us",
            Some("120363111@g.us"),
            "=/>group on",
        )
        .is_err());
    assert!(control
        .dispatch("wa-alpha", "socket-owner", "self", None, "=/>group on")
        .is_err());
    assert!(control
        .dispatch(
            "wa-alpha",
            "socket-owner",
            "120363111@g.us",
            Some("120363111@g.us"),
            "=/>on",
        )
        .is_err());
    // Status is intentionally visible to the authenticated socket owner
    // in the current group even while the bot/socket is off.
    assert_eq!(
        control
            .dispatch(
                "wa-alpha",
                "socket-owner",
                "120363111@g.us",
                Some("120363111@g.us"),
                "=/>status",
            )
            .unwrap(),
        DispatchResult::GroupStatus {
            group_id: "120363111@g.us".into(),
            enabled: false,
            source: ActivationSource::PoweredOff,
        }
    );
    // Status never grants an outsider access to owner-only dispatch.
    assert!(control
        .dispatch(
            "wa-alpha",
            "intruder",
            "120363111@g.us",
            Some("120363111@g.us"),
            "=/>status",
        )
        .is_err());
    assert!(control
        .dispatch(
            "wa-alpha",
            "socket-owner",
            "120363111@g.us",
            Some("120363111@g.us"),
            "=/>set role foreign admin",
        )
        .is_err());
}
