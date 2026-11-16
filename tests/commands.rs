/*
 * Matter-Base — Public Command Router API
 *
 * Verifies the documented, platform-neutral consumer path.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::Result;
use matter_base::{CommandAuthorizer, CommandInvocation, CommandRouter};
use std::sync::Arc;

struct Policy;

impl CommandAuthorizer for Policy {
    fn authorize(&self, invocation: &CommandInvocation) -> Result<()> {
        if invocation.bot_instance_id == "known-bot" {
            Ok(())
        } else {
            anyhow::bail!("command denied")
        }
    }
}

#[test]
fn documented_public_api_builds_and_dispatches_a_command() {
    let router = CommandRouter::new(vec!["#".to_owned()], Arc::new(Policy)).unwrap();
    router
        .register("help", &["h".to_owned()], |invocation| {
            Ok(format!(
                "{} {}",
                invocation.command,
                invocation.args.join(" ")
            ))
        })
        .unwrap();

    assert_eq!(
        router
            .route("known-bot", "app-user", "room-1", "#h for me")
            .unwrap(),
        Some("help for me".to_owned())
    );
    assert!(router
        .route("foreign-bot", "app-user", "room-1", "#help")
        .is_err());
}
