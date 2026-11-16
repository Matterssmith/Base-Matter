/*
 * Matter-Base — Control dispatch
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use crate::{activation::SocketActivationStore, BotRegistry, CommandRouter};
use anyhow::{bail, Result};
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
pub enum DispatchResult {
    Control { socket_id: String, powered: bool },
    Inactive,
    Message(Option<String>),
}

/// Trusted ingress boundary. The application authenticates `actor`, establishes
/// the receiving socket and source/group IDs, and protects this service from
/// arbitrary in-process callers. A message never supplies the bot instance ID.
/// Global policy is supplied once by trusted configuration.
pub struct ControlDispatcher {
    activation: Arc<SocketActivationStore>,
    bots: Arc<BotRegistry>,
    router: Arc<CommandRouter>,
    global_default: Option<bool>,
}

impl ControlDispatcher {
    pub fn new(
        activation: Arc<SocketActivationStore>,
        bots: Arc<BotRegistry>,
        router: Arc<CommandRouter>,
        global_default: Option<bool>,
    ) -> Self {
        Self {
            activation,
            bots,
            router,
            global_default,
        }
    }

    /// Control grammar: `=on [SOCKET_ID]`, `=off [SOCKET_ID]`; `/>` and
    /// `=/>` are also accepted prefixes. Unknown control verbs never reach the
    /// normal router. Explicit target IDs still require that socket's owner.
    /// Admission is snapshot-based: powering off blocks subsequent admissions,
    /// but does not cancel a handler already admitted before the change.
    pub fn dispatch(
        &self,
        receiving_socket: &str,
        actor: &str,
        source: &str,
        group: Option<&str>,
        message: &str,
    ) -> Result<DispatchResult> {
        if message.len() > 16 * 1024 {
            bail!("Message exceeds size limit");
        }
        let (bot, owner) = self.activation.binding(receiving_socket)?;
        self.bots.authorize_owner(&bot, &owner)?;
        if let Some(body) = message
            .strip_prefix("=/>")
            .or_else(|| message.strip_prefix("/>"))
            .or_else(|| message.strip_prefix('='))
        {
            let tokens: Vec<_> = body.split_whitespace().collect();
            if tokens.is_empty() || tokens.len() > 2 {
                bail!("Invalid control command");
            }
            let powered = match tokens[0] {
                "on" => true,
                "off" => false,
                _ => bail!("Unknown control command"),
            };
            let socket = tokens.get(1).copied().unwrap_or(receiving_socket);
            let (target_bot, target_owner) = self.activation.binding(socket)?;
            self.bots.authorize_owner(&target_bot, &target_owner)?;
            self.activation.set_power(socket, actor, powered)?;
            return Ok(DispatchResult::Control {
                socket_id: socket.to_owned(),
                powered,
            });
        }
        if !self
            .activation
            .resolve(receiving_socket, group, self.global_default)?
            .enabled
        {
            return Ok(DispatchResult::Inactive);
        }
        self.router
            .route(&bot, actor, source, message)
            .map(DispatchResult::Message)
    }
}
