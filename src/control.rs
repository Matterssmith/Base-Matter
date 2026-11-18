/*
 * Matter-Base — Control dispatch
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use crate::{
    activation::{ActivationSource, GlobalPolicy, SocketActivationStore},
    BotRegistry, CommandRouter, ControlExtensionContext, ControlExtensionRegistry, RoleStore,
};
use anyhow::{anyhow, bail, Result};
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
pub enum DispatchResult {
    Control {
        socket_id: String,
        powered: bool,
    },
    Status {
        socket_id: String,
        enabled: bool,
        source: ActivationSource,
        bot_instance_id: Option<String>,
    },
    GroupPolicyChanged {
        group_id: String,
        policy: Option<bool>,
    },
    GroupStatus {
        group_id: String,
        enabled: bool,
        source: ActivationSource,
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
    Extension {
        command: String,
        output: String,
    },
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
    extensions: Option<Arc<ControlExtensionRegistry>>,
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
            extensions: None,
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
            extensions: None,
        }
    }

    /// Installs optional role management on reserved control commands.
    pub fn with_roles(mut self, roles: Arc<RoleStore>) -> Self {
        self.roles = Some(roles);
        self
    }

    pub fn with_control_extensions(mut self, registry: Arc<ControlExtensionRegistry>) -> Self {
        self.extensions = Some(registry);
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

    /// Control grammar: `=/>on [SOCKET_ID]`, `=/>off [SOCKET_ID]`,
    /// `=/>status [SOCKET_ID]`; `=` and `/>` remain compatibility aliases.
    /// Unknown control verbs never reach the
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
        let canonical_global_prefix = message.starts_with("=/>");
        if let Some(body) = message
            .strip_prefix("=/>")
            .or_else(|| message.strip_prefix("/>"))
            .or_else(|| message.strip_prefix('='))
        {
            let tokens: Vec<_> = body.split_whitespace().collect();
            match tokens.as_slice() {
                ["group" | "grupo", "on" | "encender"]
                | ["group" | "grupo", "off" | "apagar"]
                | ["group" | "grupo", "inherit" | "heredar"] => {
                    let group_id = group.ok_or_else(|| {
                        anyhow!("Group control requires an authenticated group chat")
                    })?;
                    let policy = match tokens[1] {
                        "on" | "encender" => Some(true),
                        "off" | "apagar" => Some(false),
                        _ => None,
                    };
                    self.activation
                        .set_group_policy(receiving_socket, actor, group_id, policy)?;
                    return Ok(DispatchResult::GroupPolicyChanged {
                        group_id: group_id.to_owned(),
                        policy,
                    });
                }
                ["group" | "grupo", "status" | "estado"] => {
                    let group_id = group.ok_or_else(|| {
                        anyhow!("Group status requires an authenticated group chat")
                    })?;
                    let admission = self
                        .activation
                        .admission(receiving_socket, Some(group_id), self.global_policy)?;
                    if admission.owner_id != actor {
                        bail!("Socket owner authorization denied");
                    }
                    return Ok(DispatchResult::GroupStatus {
                        group_id: group_id.to_owned(),
                        enabled: admission.decision.enabled,
                        source: admission.decision.source,
                    });
                }
                ["on"] | ["off"] | ["on", _] | ["off", _] => {
                    if group.is_some() {
                        bail!("Global socket power is private configuration only");
                    }
                    let powered = tokens[0] == "on";
                    let socket = tokens.get(1).copied().unwrap_or(receiving_socket);
                    self.activation.set_power(socket, actor, powered)?;
                    return Ok(DispatchResult::Control {
                        socket_id: socket.to_owned(),
                        powered,
                    });
                }
                ["status"] | ["status", _] => {
                    if group.is_some() {
                        bail!("Global socket status is private configuration only");
                    }
                    let socket = tokens.get(1).copied().unwrap_or(receiving_socket);
                    // One consistent snapshot: owner, binding, and effective
                    // activation policy. No role or bot binding is required.
                    let admission = self
                        .activation
                        .admission(socket, None, self.global_policy)?;
                    if admission.owner_id != actor {
                        bail!("Socket owner authorization denied");
                    }
                    return Ok(DispatchResult::Status {
                        socket_id: socket.to_owned(),
                        enabled: admission.decision.enabled,
                        source: admission.decision.source,
                        bot_instance_id: admission.bot_instance_id,
                    });
                }
                ["set", "role", user, role] => {
                    if group.is_some() {
                        bail!("Socket-wide roles may be configured only in private");
                    }
                    return self.change_role(receiving_socket, actor, user, role, true);
                }
                ["unset", "role", user, role] => {
                    if group.is_some() {
                        bail!("Socket-wide roles may be configured only in private");
                    }
                    return self.change_role(receiving_socket, actor, user, role, false);
                }
                [command, args @ ..] if canonical_global_prefix => {
                    let admission =
                        self.activation
                            .admission(receiving_socket, None, self.global_policy)?;
                    if admission.owner_id != actor {
                        bail!("Socket owner authorization denied");
                    }
                    let registry = self
                        .extensions
                        .as_ref()
                        .ok_or_else(|| anyhow!("Invalid control command"))?;
                    let context = ControlExtensionContext {
                        socket_id: receiving_socket,
                        actor_id: actor,
                        source_id: source,
                        group_id: group,
                    };
                    let arguments = args.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>();
                    let (command, output) = registry
                        .execute(command, &context, &arguments)?
                        .ok_or_else(|| anyhow!("Invalid control command"))?;
                    return Ok(DispatchResult::Extension { command, output });
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

// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
