/*
 * Matter-Base — Control dispatch
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use crate::{
    activation::{GlobalPolicy, SocketActivationStore},
    BotRegistry, CommandRouter,
};
use anyhow::{anyhow, bail, Result};
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
/// Global policy comes from trusted configuration or the activation store.
pub struct ControlDispatcher {
    activation: Arc<SocketActivationStore>,
    bots: Arc<BotRegistry>,
    router: Arc<CommandRouter>,
    global_policy: GlobalPolicy,
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
            global_policy: GlobalPolicy::Configured(global_default),
        }
    }

    /// Observe authorized persistent global policy changes at each admission.
    /// Global administrators still cannot power another owner's socket.
    pub fn new_with_persistent_policy(
        activation: Arc<SocketActivationStore>,
        bots: Arc<BotRegistry>,
        router: Arc<CommandRouter>,
    ) -> Self {
        Self {
            activation,
            bots,
            router,
            global_policy: GlobalPolicy::Persisted,
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
            self.activation.set_power(socket, actor, powered)?;
            return Ok(DispatchResult::Control {
                socket_id: socket.to_owned(),
                powered,
            });
        }
        let admission = self
            .activation
            .admission(receiving_socket, group, self.global_policy)?;
        if !admission.decision.enabled {
            return Ok(DispatchResult::Inactive);
        }
        let bot = admission
            .bot_instance_id
            .ok_or_else(|| anyhow!("Socket has no bot binding"))?;
        self.bots.authorize_owner(&bot, &admission.owner_id)?;
        self.router
            .route(&bot, actor, source, message)
            .map(DispatchResult::Message)
    }
}
