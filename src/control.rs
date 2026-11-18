/*
 * Matter-Base — Control dispatch
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use crate::{
    activation::{ActivationSource, GlobalPolicy, SocketActivationStore},
    control_style::ControlThemes,
    anti::{AntiStore, RULES as ANTI_RULES},
    BotRegistry, CommandRouter, ControlExtensionContext, ControlExtensionRegistry, RoleStore,
};
use anyhow::{anyhow, bail, Result};
use std::sync::Arc;

const PROJECT_CREDITS: &str = "Créditos:\nHecho por Lournyx Matterssmith [Lou-Nyx].";

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
    BotPolicyChanged {
        scope: String,
        policy: Option<bool>,
    },
    BotGlobalStatus {
        socket_powered: bool,
        default_policy: Option<bool>,
        group_policy: Option<bool>,
        private_policy: Option<bool>,
        channel_policy: Option<bool>,
        community_policy: Option<bool>,
    },
    RoleChanged {
        bot_instance_id: String,
        user_id: String,
        role_id: String,
        granted: bool,
        changed: bool,
    },
    SocketRoleChanged {
        scope_id: String,
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
    socket_roles: Option<Arc<RoleStore>>,
    extensions: Option<Arc<ControlExtensionRegistry>>,
    silent_when_disabled: bool,
    unified_live_controls: bool,
    public_menu: Option<Arc<ControlThemes>>,
    group_antis: Option<Arc<AntiStore>>,
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
            socket_roles: None,
            extensions: None,
            silent_when_disabled: false,
            unified_live_controls: false,
            public_menu: None,
            group_antis: None,
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
            socket_roles: None,
            extensions: None,
            silent_when_disabled: false,
            unified_live_controls: false,
            public_menu: None,
            group_antis: None,
        }
    }

    /// Installs optional role management on reserved control commands.
    pub fn with_roles(mut self, roles: Arc<RoleStore>) -> Self {
        self.roles = Some(roles);
        self
    }

    /// Owner-administered socket/group roles, never tied to BotRegistry.
    /// This is local durable state; encrypted Vault sync needs a separate
    /// authenticated key provider and repository sink.
    pub fn with_socket_roles(mut self, roles: Arc<RoleStore>) -> Self {
        self.socket_roles = Some(roles);
        self
    }

    fn change_socket_role(
        &self,
        socket: &str,
        actor: &str,
        group: Option<&str>,
        subject: &str,
        role: &str,
        grant: bool,
    ) -> Result<DispatchResult> {
        const SOCKET_ROLES: &[&str] = &[
            "co-owner",
            "admin",
            "moderator",
            "operator",
            "trusted",
            "member",
            "restricted",
            "banned",
        ];
        let registry = self
            .socket_roles
            .as_ref()
            .ok_or_else(|| anyhow!("Socket role controller is not configured"))?;
        let admission = self
            .activation
            .admission(socket, None, self.global_policy)?;
        if admission.owner_id != actor {
            bail!("Socket owner authorization denied");
        }
        if role != "user" && !SOCKET_ROLES.contains(&role) {
            bail!("Unknown socket role");
        }
        if !grant && role == "user" {
            bail!("The implicit user rank cannot be explicitly removed");
        }
        // The authenticated socket owner chooses an explicit PN/LID identity.
        // Display aliases and arbitrary WhatsApp names are never principals.
        let (namespace, raw) = if let Some(phone) = subject.strip_suffix("@s.whatsapp.net") {
            ("pn", phone)
        } else if let Some(lid) = subject.strip_suffix("@lid") {
            ("lid", lid)
        } else if let Some(phone) = subject.strip_prefix("pn:") {
            ("pn", phone)
        } else if let Some(lid) = subject.strip_prefix("lid:") {
            ("lid", lid)
        } else {
            ("pn", subject)
        };
        if !(8..=20).contains(&raw.len())
            || raw.starts_with('0')
            || !raw.bytes().all(|b| b.is_ascii_digit())
        {
            bail!("Socket role subject must be a numeric PN/LID identifier");
        }
        let principal = format!("{namespace}:{raw}");
        let scope = if let Some(group) = group {
            use sha2::{Digest, Sha256};
            let mut digest = Sha256::new();
            digest.update(socket.as_bytes());
            digest.update([0]);
            digest.update(group.as_bytes());
            let hash = digest.finalize();
            format!("group:{}", hex::encode(&hash[..16]))
        } else {
            format!("socket:{socket}")
        };
        registry.provision_scope(&scope, actor)?;
        for role_name in SOCKET_ROLES {
            registry.ensure_root_role(&scope, actor, role_name)?;
        }
        let changed = if grant {
            registry.set_exclusive_role(
                &scope,
                actor,
                &principal,
                if role == "user" { None } else { Some(role) },
            )?
        } else {
            registry.revoke(&scope, actor, &principal, role)?
        };
        Ok(DispatchResult::SocketRoleChanged {
            scope_id: scope,
            user_id: principal,
            role_id: role.to_owned(),
            granted: grant,
            changed,
        })
    }

    /// Read one effective bot policy. Exact group-chat overrides win over
    /// the kind default, followed by socket and configured global policy.
    /// Kind keys are private internal identifiers, never trusted chat IDs.
    fn bot_admission(
        &self,
        socket: &str,
        group: Option<&str>,
        kind: &str,
    ) -> Result<crate::activation::SocketAdmission> {
        let direct = self
            .activation
            .admission(socket, group, self.global_policy)?;
        if matches!(
            direct.decision.source,
            ActivationSource::Group | ActivationSource::PoweredOff
        ) {
            return Ok(direct);
        }
        if !matches!(kind, "group" | "self" | "private" | "channel" | "community") {
            bail!("Unsupported bot chat type");
        }
        let key = format!("kind:{kind}");
        let kind_admission = self
            .activation
            .admission(socket, Some(&key), self.global_policy)?;
        if kind_admission.decision.source == ActivationSource::Group {
            Ok(kind_admission)
        } else if matches!(kind, "group" | "private" | "channel" | "community") {
            // External chat surfaces fail closed by default. The socket-wide
            // fallback never silently enables every group, stranger DM,
            // channel or community announcement. A verified owner must permit the kind
            // or the exact group. Only the self-chat inherits socket-wide
            // policy automatically.
            let mut denied = direct;
            denied.decision.enabled = false;
            denied.decision.source = ActivationSource::None;
            Ok(denied)
        } else {
            Ok(direct)
        }
    }

    fn change_bot_policy(
        &self,
        socket: &str,
        actor: &str,
        group: Option<&str>,
        kind: &str,
        scope: &str,
        policy: Option<bool>,
    ) -> Result<DispatchResult> {
        let admission = self
            .activation
            .admission(socket, None, self.global_policy)?;
        if admission.owner_id != actor {
            bail!("Socket owner authorization denied");
        }
        let key = match scope {
            "global" => {
                if self.unified_live_controls && group.is_some() {
                    bail!("Socket-wide defaults require the owner's private chat");
                }
                if self.unified_live_controls {
                    self.activation
                        .set_live_bot_policy(socket, actor, None, policy)?;
                } else {
                    self.activation.set_socket_policy(socket, actor, policy)?;
                }
                return Ok(DispatchResult::BotPolicyChanged {
                    scope: "socket-global".into(),
                    policy,
                });
            }
            "current" => match group {
                Some(group_id) => group_id.to_owned(),
                None => format!("kind:{kind}"),
            },
            "group" | "private" | "channel" | "community" => format!("kind:{scope}"),
            _ => bail!("Unsupported bot policy scope"),
        };
        if self.unified_live_controls {
            self.activation
                .set_live_bot_policy(socket, actor, Some(&key), policy)?;
        } else {
            self.activation
                .set_group_policy(socket, actor, &key, policy)?;
        }
        Ok(DispatchResult::BotPolicyChanged {
            scope: scope.to_owned(),
            policy,
        })
    }

    pub fn with_control_extensions(mut self, registry: Arc<ControlExtensionRegistry>) -> Self {
        self.extensions = Some(registry);
        self
    }
    /// Opt-in live chat policy: when the bot is disabled, do not execute
    /// nonessential extensions or emit a chat acknowledgment.
    /// Core control extensions retain their legacy owner-only behavior
    /// unless a transport explicitly enables this setting.
    pub fn with_silent_bot_commands(mut self) -> Self {
        self.silent_when_disabled = true;
        self
    }

    /// The live WhatsApp API uses only =/>bot on/off, with a verified
    /// chat/type/global scope. The legacy raw power API is kept solely
    /// for internal core compatibility and is refused by live ingress.
    /// Enabling the bot wakes the operational socket atomically.
    pub fn with_unified_live_controls(mut self) -> Self {
        self.unified_live_controls = true;
        self
    }

    /// A separate read-only entry point, not ordinary control dispatch.
    /// The HMAC bridge must verify provider-origin sender and chat identities
    /// before invoking this method. This API never grants roles, runs bot
    /// handlers, invokes arbitrary extensions or edits activation policies.
    pub fn dispatch_public_readonly(
        &self,
        socket: &str,
        verified_sender: &str,
        verified_chat: &str,
        group: Option<&str>,
        message: &str,
    ) -> Result<DispatchResult> {
        if verified_sender.is_empty()
            || verified_sender.len() > 128
            || verified_sender.chars().any(char::is_control)
            || verified_chat.is_empty()
            || verified_chat.len() > 128
            || verified_chat.chars().any(char::is_control)
            || group.is_some_and(|value| value != verified_chat)
        {
            bail!("Unauthenticated chat scope");
        }
        let kind = if group.is_some() { "group" } else { "private" };
        let decision = self.bot_admission(socket, group, kind)?;
        let parts: Vec<_> = message.split_whitespace().collect();
        match parts.as_slice() {
            ["=/>status" | "=/>estado"] => {
                if let Some(group_id) = group {
                    Ok(DispatchResult::GroupStatus {
                        group_id: group_id.to_owned(),
                        enabled: decision.decision.enabled,
                        source: decision.decision.source,
                    })
                } else {
                    Ok(DispatchResult::Status {
                        socket_id: socket.to_owned(),
                        enabled: decision.decision.enabled,
                        source: decision.decision.source,
                        bot_instance_id: None,
                    })
                }
            }
            ["=/>menu" | "=/>menú"] => {
                if !decision.decision.enabled {
                    return Ok(DispatchResult::Inactive);
                }
                let theme = self
                    .public_menu
                    .as_ref()
                    .ok_or_else(|| anyhow!("Read-only menu is unavailable"))?;
                let output = theme.render(group, "menu")?;
                Ok(DispatchResult::Extension {
                    command: "menu".to_owned(),
                    output,
                })
            }
            ["=/>credits" | "=/>creditos" | "=/>créditos"] => {
                if !decision.decision.enabled {
                    return Ok(DispatchResult::Inactive);
                }
                Ok(DispatchResult::Extension {
                    command: "credits".to_owned(),
                    output: PROJECT_CREDITS.to_owned(),
                })
            }
            ["=/>bot"] => {
                if !decision.decision.enabled {
                    return Ok(DispatchResult::Inactive);
                }
                Ok(DispatchResult::Extension {
                    command: "bot".to_owned(),
                    output: "BOT: =/>status consulta este chat; =/>menu muestra los controles disponibles cuando está activo. Solo el propietario verificado administra =/>bot on/off y sus alcances; los roles requieren Vault-Matter.".to_owned(),
                })
            }
            _ => bail!("Public chat cannot invoke administrative commands"),
        }
    }

    /// Installs durable, per-verified-group anti-rule configuration.
    /// The dispatcher accepts only authenticated owner controls for writes.
    /// Detection never grants authority to delete messages or remove users.
    pub fn with_group_antis(mut self, store: Arc<AntiStore>) -> Self {
        self.group_antis = Some(store);
        self
    }

    /// Install trusted, render-only public menu support. Never connect this
    /// entry point to arbitrary executable extensions or bot handlers.
    pub fn with_public_menu(mut self, themes: Arc<ControlThemes>) -> Self {
        self.public_menu = Some(themes);
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
        if self.socket_roles.is_some() {
            return self.change_socket_role(socket, actor, None, user, role, grant);
        }
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

    /// Control grammar: socket power, contextual status, bot policy scopes,
    /// and reserved owner extensions. `=` and `/>` remain legacy aliases.
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
        // Debug is an opt-in *rendering* of an already authorized effect.
        // Strip it before command parsing; never grant permissions via flags.
        let (statement, debug) = if message.starts_with("=/>") {
            match message.strip_suffix(" --debug") {
                Some(statement) => (statement, true),
                None => (message, false),
            }
        } else {
            (message, false)
        };
        let outcome = self.dispatch_inner(receiving_socket, actor, source, group, statement)?;
        if !debug || !statement.starts_with("=/>") || matches!(&outcome, DispatchResult::Inactive) {
            return Ok(outcome);
        }
        let kind = if group.is_some() { "group" } else { "self" };
        let admission = self.bot_admission(receiving_socket, group, kind)?;
        if admission.owner_id != actor {
            bail!("Technical diagnostics require the authenticated socket owner");
        }
        let summary = match &outcome {
            DispatchResult::Status { enabled, source, .. }
            | DispatchResult::GroupStatus { enabled, source, .. } => {
                format!("BOT {} (origen: {source:?})", if *enabled { "ACTIVADO" } else { "APAGADO" })
            }
            DispatchResult::Control { powered, .. } => {
                format!("Socket {}", if *powered { "encendido" } else { "apagado" })
            }
            DispatchResult::BotPolicyChanged { scope, policy } => {
                format!("Política {scope}: {policy:?}")
            }
            DispatchResult::BotGlobalStatus {
                socket_powered, default_policy,
                group_policy, private_policy, channel_policy, community_policy,
            } => format!(
                "Socket={socket_powered}; defecto={default_policy:?}; grupos={group_policy:?}; privados={private_policy:?}; canales={channel_policy:?}; avisos={community_policy:?}"
            ),
            DispatchResult::GroupPolicyChanged { policy, .. } => {
                format!("Política de grupo: {policy:?}")
            }
            DispatchResult::Extension { output, .. } => output.clone(),
            _ => "Control autenticado completado".to_owned(),
        };
        let detail = format!(
            "\n[DEBUG] entrada=firmada; actor=propietario; contexto={kind};              origen_politica={:?}; bot_habilitado={}; vault_roles=solo-lectura/no-conectado;              secretos=ocultos",
            admission.decision.source, admission.decision.enabled
        );
        let output = format!("{summary}{detail}");
        if output.len() > 4096 {
            bail!("Diagnostic response exceeds limit");
        }
        Ok(DispatchResult::Extension {
            command: "debug".to_owned(),
            output,
        })
    }

    fn dispatch_inner(
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
            let kind = if group.is_some() { "group" } else { "self" };
            if self.unified_live_controls {
                // Retired legacy spellings never perform bot activation on
                // WhatsApp. The sole live toggle is =/>bot on/off.
                match tokens.as_slice() {
                    ["on" | "off" | "encender" | "apagar", ..]
                    | ["group" | "grupo", "on" | "off" | "encender" | "apagar" | "inherit" | "heredar"] =>
                    {
                        bail!("Use =/>bot on/off/inherit with an authorized scope");
                    }
                    _ => {}
                }
            }
            match tokens.as_slice() {
                ["antis"] => {
                    let group_id = group.ok_or_else(|| anyhow!("Antis requieren grupo verificado"))?;
                    let admission = self.bot_admission(receiving_socket, group, kind)?;
                    if admission.owner_id != actor {
                        bail!("Solo el propietario del socket configura protecciones");
                    }
                    let store = self.group_antis.as_ref().ok_or_else(|| anyhow!("Protecciones no configuradas"))?;
                    let output = store.command(receiving_socket, group_id, "antis", &[])?;
                    return Ok(DispatchResult::Extension { command: "antis".into(), output });
                }
                ["set", rule, "action" | "accion" | "acciones" | "accions", action]
                    if ANTI_RULES.contains(rule) =>
                {
                    let group_id = group.ok_or_else(|| anyhow!("Configura antis dentro del grupo"))?;
                    let admission = self.bot_admission(receiving_socket, group, kind)?;
                    if admission.owner_id != actor {
                        bail!("Solo el propietario del socket configura protecciones");
                    }
                    let store = self.group_antis.as_ref().ok_or_else(|| anyhow!("Protecciones no configuradas"))?;
                    let args = vec![(*rule).to_owned(), "action".to_owned(), (*action).to_owned()];
                    let output = store.command(receiving_socket, group_id, "set", &args)?;
                    return Ok(DispatchResult::Extension { command: "set".into(), output });
                }
                [rule, args @ ..] if ANTI_RULES.contains(rule) => {
                    let group_id = group.ok_or_else(|| anyhow!("Antis requieren grupo verificado"))?;
                    let admission = self.bot_admission(receiving_socket, group, kind)?;
                    if admission.owner_id != actor {
                        bail!("Solo el propietario del socket configura protecciones");
                    }
                    let store = self.group_antis.as_ref().ok_or_else(|| anyhow!("Protecciones no configuradas"))?;
                    let args = args.iter().map(|value| (*value).to_owned()).collect::<Vec<_>>();
                    let output = store.command(receiving_socket, group_id, rule, &args)?;
                    return Ok(DispatchResult::Extension { command: (*rule).to_owned(), output });
                }
                ["credits" | "creditos" | "créditos"] => {
                    let admission = self.bot_admission(receiving_socket, group, kind)?;
                    if admission.owner_id != actor {
                        bail!("Socket owner authorization denied");
                    }
                    return Ok(DispatchResult::Extension {
                        command: "credits".to_owned(),
                        output: PROJECT_CREDITS.to_owned(),
                    });
                }
                ["status" | "estado"] | ["status" | "estado", "--debug"] => {
                    let admission = self.bot_admission(receiving_socket, group, kind)?;
                    if admission.owner_id != actor {
                        bail!("Socket owner authorization denied");
                    }
                    if let Some(group_id) = group {
                        return Ok(DispatchResult::GroupStatus {
                            group_id: group_id.to_owned(),
                            enabled: admission.decision.enabled,
                            source: admission.decision.source,
                        });
                    }
                    return Ok(DispatchResult::Status {
                        socket_id: receiving_socket.to_owned(),
                        enabled: admission.decision.enabled,
                        source: admission.decision.source,
                        bot_instance_id: admission.bot_instance_id,
                    });
                }
                ["status" | "estado", "--global"]
                | ["status" | "estado", "--global", "--debug"] => {
                    let defaults = self
                        .activation
                        .inspect_bot_defaults(receiving_socket, actor)?;
                    return Ok(DispatchResult::BotGlobalStatus {
                        socket_powered: defaults.socket_powered,
                        default_policy: defaults.default_policy,
                        group_policy: defaults.group_policy,
                        private_policy: defaults.private_policy,
                        channel_policy: defaults.channel_policy,
                        community_policy: defaults.community_policy,
                    });
                }
                ["bot", "on" | "off" | "inherit" | "encender" | "apagar" | "heredar"]
                | ["bot", "--global", "on" | "off" | "inherit" | "encender" | "apagar" | "heredar"]
                | ["bot", "on" | "off" | "inherit" | "encender" | "apagar" | "heredar", "--global"]
                | ["bot", "group" | "private" | "channel" | "community" | "announcements" | "grupo"
                | "privado" | "canal" | "comunidad" | "avisos", "on" | "off" | "inherit" | "encender" | "apagar" | "heredar"] =>
                {
                    let action = if tokens.last() == Some(&"--global") {
                        tokens[1]
                    } else {
                        tokens[tokens.len() - 1]
                    };
                    let policy = match action {
                        "on" | "encender" => Some(true),
                        "off" | "apagar" => Some(false),
                        _ => None,
                    };
                    let scope = if tokens.len() == 2 {
                        "current"
                    } else if tokens[1] == "--global" || tokens.last() == Some(&"--global") {
                        "global"
                    } else {
                        if group.is_some() {
                            bail!("Kind-wide policy changes require private owner chat");
                        }
                        match tokens[1] {
                            "grupo" => "group",
                            "privado" => "private",
                            "canal" => "channel",
                            "comunidad" | "avisos" | "announcements" => "community",
                            value => value,
                        }
                    };
                    return self.change_bot_policy(
                        receiving_socket,
                        actor,
                        group,
                        kind,
                        scope,
                        policy,
                    );
                }
                ["bot"] | ["bot", "--global"] => {
                    let admission = self.bot_admission(receiving_socket, group, kind)?;
                    if admission.owner_id != actor {
                        bail!("Socket owner authorization denied");
                    }
                    if !admission.decision.enabled {
                        return Ok(DispatchResult::Inactive);
                    }
                    return Ok(DispatchResult::Extension {
                        command: "bot".to_owned(),
                        output: "BOT: =/>bot on/off/inherit (chat actual); =/>bot group/private/channel/community on/off/inherit (tipos, desde chat propio); =/>bot --global on/off/inherit (predeterminado del socket). =/>bot community configura avisos, pero requiere metadatos verificados para aplicarse. =/>status consulta el chat, =/>status --global las políticas. Roles exclusivos de Vault-Matter.".to_owned(),
                    });
                }
                ["set"] => {
                    if group.is_some() {
                        bail!("Configuration is private owner control");
                    }
                    let admission =
                        self.activation
                            .admission(receiving_socket, None, self.global_policy)?;
                    if admission.owner_id != actor {
                        bail!("Socket owner authorization denied");
                    }
                    return Ok(DispatchResult::Extension {
                        command: "set".to_owned(),
                        output: "SET: =/>set role <identidad> <rango> requiere Vault-Matter (no disponible). =/>set style/theme: ajuste visual pendiente. Las políticas del bot se configuran con =/>bot. --debug no concede permisos.".to_owned(),
                    });
                }
                ["set", "role"] => {
                    if group.is_some() {
                        bail!("Role administration requires private owner chat");
                    }
                    return Ok(DispatchResult::Extension {
                        command: "set".to_owned(),
                        output: "SET ROLE: administración exclusiva de Vault-Matter; sin conexión autenticada no se permiten modificaciones. Las identidades deben verificarse antes de asignar rangos.".to_owned(),
                    });
                }
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
                    if self.unified_live_controls {
                        self.activation.set_live_bot_policy(
                            receiving_socket,
                            actor,
                            Some(group_id),
                            policy,
                        )?;
                    } else {
                        self.activation.set_group_policy(
                            receiving_socket,
                            actor,
                            group_id,
                            policy,
                        )?;
                    }
                    return Ok(DispatchResult::GroupPolicyChanged {
                        group_id: group_id.to_owned(),
                        policy,
                    });
                }
                ["group" | "grupo", "role" | "rango", user, role] => {
                    let group_id = group.ok_or_else(|| {
                        anyhow!("Group role control requires an authenticated group chat")
                    })?;
                    return self.change_socket_role(
                        receiving_socket,
                        actor,
                        Some(group_id),
                        user,
                        role,
                        true,
                    );
                }
                ["group" | "grupo", "status" | "estado"] => {
                    let group_id = group.ok_or_else(|| {
                        anyhow!("Group status requires an authenticated group chat")
                    })?;
                    let admission = self.activation.admission(
                        receiving_socket,
                        Some(group_id),
                        self.global_policy,
                    )?;
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
                ["status" | "estado", socket] if socket != &"--debug" && socket != &"--global" => {
                    if group.is_some() {
                        bail!("Targeted socket status requires private owner chat");
                    }
                    let admission = self
                        .activation
                        .admission(socket, None, self.global_policy)?;
                    if admission.owner_id != actor {
                        bail!("Socket owner authorization denied");
                    }
                    return Ok(DispatchResult::Status {
                        socket_id: (*socket).to_owned(),
                        enabled: admission.decision.enabled,
                        source: admission.decision.source,
                        bot_instance_id: admission.bot_instance_id,
                    });
                }
                ["role", user, role] => {
                    if group.is_some() {
                        bail!("Socket role assignment requires private owner chat");
                    }
                    return self.change_role(receiving_socket, actor, user, role, true);
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
                    let admission = self.bot_admission(receiving_socket, group, kind)?;
                    if admission.owner_id != actor {
                        bail!("Socket owner authorization denied");
                    }
                    // A disabled bot sends no reply for menu and other
                    // nonessential operations. Status remains available.
                    if self.silent_when_disabled
                        && !admission.decision.enabled
                        && !matches!(*command, "roles" | "audit")
                    {
                        return Ok(DispatchResult::Inactive);
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
