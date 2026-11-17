/*
 * Matter-Base — Control dispatch
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use crate::{
    activation::{GlobalPolicy, SocketActivationStore},
    BotRegistry, CommandRouter, RoleStore,
};
use anyhow::{anyhow, bail, Result};
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
pub enum DispatchResult {
    Control {
        socket_id: String,
        powered: bool,
    },
    RoleChanged {
        bot_instance_id: String,
        user_id: String,
        role_id: String,
        granted: bool,
        changed: bool,
    },
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
    roles: Option<Arc<RoleStore>>,
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
            roles: None,
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
            roles: None,
        }
    }

    /// Installs optional role management on reserved control commands.
    pub fn with_roles(mut self, roles: Arc<RoleStore>) -> Self {
        self.roles = Some(roles);
        self
    }

    fn change_role(
        &self,
        socket: &str,
        actor: &str,
        user: &str,
        role: &str,
        grant: bool,
    ) -> Result<DispatchResult> {
        let roles = self
            .roles
            .as_ref()
            .ok_or_else(|| anyhow!("Role control not configured"))?;
        let admission = self
            .activation
            .admission(socket, None, self.global_policy)?;
        if admission.owner_id != actor {
            bail!("Socket owner authorization denied");
        }
        let bot = admission
            .bot_instance_id
            .ok_or_else(|| anyhow!("Socket has no bot binding"))?;
        self.bots.authorize_owner(&bot, actor)?;
        let changed = if grant {
            roles.grant(&bot, actor, user, role)?
        } else {
            roles.revoke(&bot, actor, user, role)?
        };
        Ok(DispatchResult::RoleChanged {
            bot_instance_id: bot,
            user_id: user.to_owned(),
            role_id: role.to_owned(),
            granted: grant,
            changed,
        })
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
            match tokens.as_slice() {
                ["on"] | ["off"] | ["on", _] | ["off", _] => {
                    let powered = tokens[0] == "on";
                    let socket = tokens.get(1).copied().unwrap_or(receiving_socket);
                    self.activation.set_power(socket, actor, powered)?;
                    return Ok(DispatchResult::Control {
                        socket_id: socket.to_owned(),
                        powered,
                    });
                }
                ["set", "role", user, role] => {
                    return self.change_role(receiving_socket, actor, user, role, true);
                }
                ["unset", "role", user, role] => {
                    return self.change_role(receiving_socket, actor, user, role, false);
                }
                _ => bail!("Invalid control command"),
            }
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
