/*
 * Base-Matter — no local fallback when Vault role authority is absent.
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
        bail!("Unrelated bot operation is not authorized")
    }
}

#[test]
fn missing_vault_authority_never_grants_socket_or_group_roles() {
    let activation = Arc::new(SocketActivationStore::open(":memory:").unwrap());
    activation.provision("wa-alpha", "trusted-owner").unwrap();
    let router = Arc::new(
        CommandRouter::new(vec!["#".to_owned()], Arc::new(DenyBot)).unwrap(),
    );
    // No RoleStore or Vault writer is installed: the live runtime must
    // never create grants with this state.
    let controller = ControlDispatcher::new(
        Arc::clone(&activation),
        Arc::new(BotRegistry::in_memory()),
        router,
        None,
    );

    for command in [
        "=/>set role 5215512345678 admin",
        "=/>role 5215512345678 moderator",
        "=/>unset role 5215512345678 admin",
    ] {
        assert!(
            controller
                .dispatch("wa-alpha", "trusted-owner", "private", None, command)
                .is_err()
        );
    }
    assert!(
        controller
            .dispatch(
                "wa-alpha",
                "trusted-owner",
                "120363123456789@g.us",
                Some("120363123456789@g.us"),
                "=/>group role 5215512345678 admin",
            )
            .is_err()
    );
    // An authenticated owner can still perform a safety power-off action.
    assert_eq!(
        controller
            .dispatch("wa-alpha", "trusted-owner", "private", None, "=/>off")
            .unwrap(),
        DispatchResult::Control {
            socket_id: "wa-alpha".to_owned(),
            powered: false,
        }
    );
    assert_eq!(
        controller
            .dispatch("wa-alpha", "trusted-owner", "private", None, "=/>status")
            .unwrap(),
        DispatchResult::Status {
            socket_id: "wa-alpha".to_owned(),
            enabled: false,
            source: ActivationSource::PoweredOff,
            bot_instance_id: None,
        }
    );
    // Project ownership labels never grant another socket's privileges.
    assert!(
        controller
            .dispatch("wa-alpha", "project-owner", "private", None, "=/>on")
            .is_err()
    );
}
