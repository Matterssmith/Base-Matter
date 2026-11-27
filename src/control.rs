/*
 * Matter-Base — Control dispatch
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use crate::{
    activation::{ActivationSource, GlobalPolicy, SocketActivationStore},
    anti::{AntiStore, RULES as ANTI_RULES},
    control_style::ControlThemes,
    BotRegistry, CommandRouter, ControlExtensionContext, ControlExtensionRegistry, RoleStore,
};
use anyhow::{anyhow, bail, Result};
use std::sync::Arc;

// Visible only on demand. No GitHub membership or Vault rank is inferred from
// a telephone number, display name or local role record. The collaborator
// directory is empty until signed, scope-bound Vault decisions are available.
const PROJECT_CREDITS: &str = "Créditos:\nHecho por Lournyx Matterssmith [Lou-Nyx].\nFundadora y creadora del proyecto: Lournyx Matterssmith [Lou-Nyx].\nColaboradores: pendientes de registro GitHub y verificación de rangos en Vault-Matter.";

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

    /// Chat ingress policy, independent of caller rank and command permissions.
    /// Only the status command family may bypass disabled chat admission.
    pub(crate) fn chat_command_allowed(
        &self,
        socket: &str,
        group: Option<&str>,
        kind: &str,
        message: &str,
    ) -> Result<bool> {
        let admission = self.bot_admission(socket, group, kind)?;
        let status = matches!(
            message.split_whitespace().next(),
            Some("=/>status" | "=/>estado")
        );
        Ok(status || admission.decision.enabled)
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
    pub(crate) fn scoped_style_reply(
        &self,
        scope: crate::control_scope::CommandScope,
        chat: &str,
        principal: &str,
        value: Option<&str>,
        can_change: bool,
    ) -> Result<DispatchResult> {
        use crate::control_scope::CommandScope;
        let label = match scope {
            CommandScope::Chat => "chat",
            CommandScope::Socket => "socket (--mod)",
            CommandScope::Global => "Matter (--global)",
        };
        let output = (|| -> Result<String> {
            let themes = self.public_menu.as_ref().ok_or_else(|| anyhow!("Vault no disponible"))?;
            if let Some(value) = value {
                if !can_change {
                    return Ok(format!("Permiso denegado para modificar el estilo de {label}."));
                }
                if !matches!(value, "matter" | "none" | "inherit")
                    || (scope == CommandScope::Global && value == "inherit")
                {
                    return Ok("Estilo no válido. Usa matter o none; inherit solo por chat o --mod.".into());
                }
                themes.write_scoped_style(scope, chat, principal, value)?;
                return Ok(format!("Estilo de {label}: {value}. Guardado en Vault-Matter."));
            }
            let (style, offline) = themes.scoped_style_read(scope, chat)?;
            let source = if offline {
                "Copia cifrada de la última lectura confirmada; puede estar desactualizada."
            } else {
                "Lectura autenticada de Vault-Matter."
            };
            Ok(format!("Estilo de {label}: {}. Opciones: matter | none.\n{source}", style.as_str()))
        })().unwrap_or_else(|error| {
            match error.to_string().as_str() {
                "vault_offline_no_snapshot" => "Vault-Matter fuera de línea. No existe una copia cifrada verificada para este ámbito. Estilo de fábrica: matter (no confirma el valor guardado).",
                "vault_unavailable" => "Vault-Matter fuera de línea. Las escrituras requieren el servicio activo. No se confirmó ningún cambio.",
                "vault_global_style_denied" => "Vault-Matter respondió, pero denegó la lectura del estilo global. Comprueba la versión del servicio y el permiso del cliente.",
                "vault_operation_denied" => "Vault-Matter respondió, pero denegó la operación. Comprueba la vigencia del permiso del cliente. No se confirmó ningún cambio.",
                "vault_connection_failed" => "No se pudo conectar a Vault-Matter. Comprueba el socket IPC y sus permisos.",
                _ => "Vault-Matter: configuración no disponible o permiso denegado. No se confirmó ningún cambio.",
            }.into()
        });
        Ok(DispatchResult::Extension {
            command: "set".into(),
            output,
        })
    }

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
        if !self.chat_command_allowed(socket, group, kind, message)? {
            return Ok(DispatchResult::Inactive);
        }
        let parts: Vec<_> = message.split_whitespace().collect();
        match parts.as_slice() {
            ["=/>register"] | ["=/>register", "start"] => {
                let start_requested = parts.len() == 2;
                // Registration is informational and requires an enabled chat.
                Ok(DispatchResult::Extension {
                    command: "register".to_owned(),
                    output: crate::guest::registration_notice(start_requested).to_owned(),
                })
            }
            ["=/>ping"] => Ok(DispatchResult::Extension {
                command: "ping".to_owned(),
                output: "Pong · Latencia IPC: pendiente".to_owned(),
            }),
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
            ["=/>status" | "=/>estado", "--global"] => Ok(DispatchResult::Extension {
                command: "status".to_owned(),
                output: self.global_default_status().to_owned(),
            }),
            ["=/>status" | "=/>estado", "--mod"] => Ok(DispatchResult::Extension {
                command: "status".to_owned(),
                output: "Las políticas internas del socket son privadas. Consulta el chat con =/>status.".to_owned(),
            }),
            ["=/>menu" | "=/>menú", rest @ ..] if rest.len() <= 2 => {
                if !crate::guest::can_use_without_registration(
                    crate::guest::GuestCapability::Menu,
                    decision.decision.enabled,
                ) {
                    return Ok(DispatchResult::Inactive);
                }
                let theme = self
                    .public_menu
                    .as_ref()
                    .ok_or_else(|| anyhow!("Read-only menu is unavailable"))?;
                let section = if rest.is_empty() {
                    "menu".to_owned()
                } else {
                    format!("menu {}", rest.join(" "))
                };
                let output = theme.render(Some(verified_chat), &section)?;
                Ok(DispatchResult::Extension {
                    command: "menu".to_owned(),
                    output,
                })
            }
            ["=/>help" | "=/>ayuda" | "=/>h", rest @ ..] if rest.len() <= 3 => {
                if !decision.decision.enabled { return Ok(DispatchResult::Inactive); }
                let theme = self.public_menu.as_ref()
                    .ok_or_else(|| anyhow!("Read-only help is unavailable"))?;
                let section = if rest.is_empty() { "help".to_owned() }
                    else { format!("help {}", rest.join(" ")) };
                let output = theme.render(Some(verified_chat), &section)?;
                Ok(DispatchResult::Extension { command: "help".to_owned(), output })
            }
            ["=/>test"]
            | ["=/>test", "ui" | "row"]
            | ["=/>test", "ack", "a" | "b" | "one" | "two" | "audio" | "mp4" | "link"] => {
                let output = match parts.as_slice() {
                    ["=/>test", "ui"] => "🧪 Prueba NativeFlow: botones y selector. Si aparecen, púlsalos. No se modifica ningún dato.".to_owned(),
                    ["=/>test", "row"] => "🧪 Prueba ButtonsRow: Audio, MP4 y Enlace. No se ejecutan descargas.".to_owned(),
                    ["=/>test", "ack", selected] => format!(
                        "🧪 ACK {selected}: comando de texto autorizado. Solo una respuesta                          interactiva detectada por el proveedor prueba la pulsación."
                    ),
                    _ => "🧪 Pruebas del chat: =/>test ui (NativeFlow), =/>test row (ButtonsRow), =/>test html (Rich HTML experimental en grupos habilitados).".to_owned(),
                };
                Ok(DispatchResult::Extension {
                    command: "test".to_owned(),
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
            ["=/>anti", words @ ..] if words.len() <= 3 => {
                if !decision.decision.enabled {
                    return Ok(DispatchResult::Inactive);
                }
                let scoped = crate::control_scope::parse_scope(words)?;
                if scoped.arguments.len() > 1
                    && scoped.arguments.as_slice() != ["audit"]
                {
                    bail!("Uso: anti [--mod|--global] [tipo]");
                }
                let store = self
                    .group_antis
                    .as_ref()
                    .ok_or_else(|| anyhow!("Protecciones no configuradas"))?;
                let output = if scoped.scope == crate::control_scope::CommandScope::Global {
                    match scoped.arguments.as_slice() {
                        [] => AntiStore::global_factory_defaults(None)?,
                        [name] => AntiStore::global_factory_defaults(Some(name))?,
                        _ => bail!("Uso: anti --global [tipo]"),
                    }
                } else {
                    let scope = match scoped.scope {
                        crate::control_scope::CommandScope::Socket => None,
                        crate::control_scope::CommandScope::Chat => {
                            Self::anti_chat_scope(group, verified_chat)
                        }
                        crate::control_scope::CommandScope::Global => unreachable!(),
                    };
                    let arguments = scoped
                        .arguments
                        .iter()
                        .map(|word| (*word).to_owned())
                        .collect::<Vec<_>>();
                    store.inspect_command(socket, scope, &arguments)?
                };
                Ok(DispatchResult::Extension {
                    command: "anti".to_owned(),
                    output,
                })
            }
            _ => bail!("Public chat cannot invoke administrative commands"),
        }
    }

    fn global_default_status(&self) -> &'static str {
        match self.global_policy {
            GlobalPolicy::Configured(Some(true)) => {
                "Predeterminado general de este proceso: ON. No representa una política central Vault."
            }
            GlobalPolicy::Configured(Some(false)) => {
                "Predeterminado general de este proceso: OFF. No representa una política central Vault."
            }
            GlobalPolicy::Configured(None) => {
                "Predeterminado general: sin configuración; denegar por defecto."
            }
            GlobalPolicy::Persisted => {
                "Predeterminado general: consulta central no disponible aquí sin autoridad global autenticada."
            }
        }
    }

    fn anti_chat_scope<'a>(group: Option<&'a str>, chat: &'a str) -> Option<&'a str> {
        group.or_else(|| {
            if chat.ends_with("@s.whatsapp.net") || chat.ends_with("@lid") {
                Some(chat)
            } else {
                None
            }
        })
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

    /// Provider-inspected group message, not a control invocation.
    /// No moderation action, bot routing, permissions or outgoing reply.
    pub fn inspect_anti_observation(
        &self,
        socket: &str,
        group: &str,
        text: &str,
        edit: crate::anti_guard::EditState,
    ) -> Result<()> {
        // Passive inspection is scoped to chats the owner has activated.
        // Disabled groups do not accumulate observational metadata.
        if !self
            .bot_admission(socket, Some(group), "group")?
            .decision
            .enabled
        {
            return Ok(());
        }
        let store = self
            .group_antis
            .as_ref()
            .ok_or_else(|| anyhow!("Protecciones no configuradas"))?;
        let _ = store.inspect_message(socket, group, text, edit)?;
        Ok(())
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
                ["vault", ..] if !canonical_global_prefix => {
                    bail!("Usa el prefijo completo =/>vault");
                }
                ["register"] | ["register", "start"] => {
                    return Ok(DispatchResult::Extension {
                        command: "register".to_owned(),
                        output: crate::guest::registration_notice(tokens.len() == 2).to_owned(),
                    });
                }
                ["test"]
                | ["test", "ui" | "html" | "row"]
                | ["test", "ack", "a" | "b" | "one" | "two" | "audio" | "mp4" | "link"] => {
                    let admission =
                        self.activation
                            .admission(receiving_socket, group, self.global_policy)?;
                    if group.is_some() && !admission.decision.enabled {
                        return Ok(DispatchResult::Inactive);
                    }
                    if admission.owner_id != actor {
                        bail!("Solo el propietario verificado puede probar la interfaz");
                    }
                    let output = match tokens.as_slice() {
                        ["test", "ui"] => [
                            "🧪 Prueba de botones WhatsApp",
                            "Esta prueba es solo de lectura y no modifica Vault ni grupos.",
                            "Si ves dos botones y un selector, tócalos para comprobar la respuesta.",
                            "Cada selección debe producir un acuse del controlador.",
                            "Requiere MATTER_WA_NATIVE_BUTTONS=1; si falta, se devuelve este texto.",
                            "Enviar no garantiza que WhatsApp muestre el control.",
                        ]
                        .join("\n"),
                        ["test", "row"] => [
                            "🧪 Prueba de tarjeta ButtonsRow",
                            "Ejemplo de prueba con tres botones: Audio, MP4, Enlace.",
                            "No descarga archivos ni ejecuta código.",
                            "El cliente puede ocultar botones clásicos antiguos.",
                        ]
                        .join("\n"),
                        ["test", "html"] => [
                            "🧪 Prueba HTML experimental",
                            "Disponible en grupos habilitados y en el chat privado del propietario.",
                            "Requiere MATTER_WA_RICH_HTML=1.",
                            "El cliente puede ignorar la visualización aunque acepte el envío.",
                            "No ejecuta scripts ni sonidos; esos recursos no están verificados.",
                        ]
                        .join("\n"),
                        ["test", "ack", selection] => format!(
                            "🧪 ACK {selection}: comando firmado y autorizado.\n\
                             Escribirlo manualmente NO demuestra una pulsación.\n\
                             El proveedor confirmará el evento interactivo si lo recibe."
                        ),
                        _ => [
                            "🧪 Pruebas visuales",
                            "=/>test ui — botones NativeFlow y selector",
                            "=/>test row — botones clásicos Audio/MP4/Enlace",
                            "=/>test html — envío Rich HTML experimental",
                            "Los test de ejecución de código siguen pendientes.",
                        ]
                        .join("\n"),
                    };
                    return Ok(DispatchResult::Extension {
                        command: "test".to_owned(),
                        output,
                    });
                }

                ["anti", values @ ..] => {
                    use crate::control_scope::{parse_scope, CommandScope};
                    let admission = self.bot_admission(receiving_socket, group, kind)?;
                    if admission.owner_id != actor {
                        bail!("Solo el propietario del socket consulta protecciones");
                    }
                    let store = self
                        .group_antis
                        .as_ref()
                        .ok_or_else(|| anyhow!("Protecciones no configuradas"))?;
                    let scoped = parse_scope(values)?;
                    let output = match scoped.scope {
                        CommandScope::Global => {
                            let filter = match scoped.arguments.as_slice() {
                                [] => None,
                                [name] => Some(*name),
                                _ => bail!("Uso: anti --global [tipo]"),
                            };
                            AntiStore::global_factory_defaults(filter)?
                        }
                        scope => {
                            let group_scope = match scope {
                                CommandScope::Socket => None,
                                CommandScope::Chat => Self::anti_chat_scope(group, source),
                                CommandScope::Global => unreachable!("handled above"),
                            };
                            let args = scoped
                                .arguments
                                .iter()
                                .map(|word| (*word).to_owned())
                                .collect::<Vec<_>>();
                            store.inspect_command(receiving_socket, group_scope, &args)?
                        }
                    };
                    return Ok(DispatchResult::Extension {
                        command: "anti".to_owned(),
                        output,
                    });
                }
                ["antis", ..] => {
                    bail!("Comando retirado. Consulta los filtros con =/>anti");
                }
                ["set", "anti", values @ ..] => {
                    let admission = self.bot_admission(receiving_socket, group, kind)?;
                    if admission.owner_id != actor {
                        bail!("Solo el propietario del socket configura protecciones");
                    }
                    let store = self
                        .group_antis
                        .as_ref()
                        .ok_or_else(|| anyhow!("Protecciones no configuradas"))?;
                    use crate::control_scope::{parse_scope, CommandScope};
                    let scoped = parse_scope(values)?;
                    let socket_scope = scoped.scope == CommandScope::Socket;
                    let words = scoped.arguments;
                    if scoped.scope == CommandScope::Global {
                        let filter = match words.as_slice() {
                            [] => None,
                            [name] => Some(*name),
                            _ => bail!("No se admiten escrituras globales sin Vault autenticado"),
                        };
                        let output = AntiStore::global_factory_defaults(filter)?;
                        return Ok(DispatchResult::Extension {
                            command: "set".to_owned(),
                            output,
                        });
                    }
                    let chat_scope = Self::anti_chat_scope(group, source);
                    if words.is_empty() {
                        let output = store.inspect_command(
                            receiving_socket,
                            if socket_scope { None } else { chat_scope },
                            &[],
                        )?;
                        return Ok(DispatchResult::Extension {
                            command: "set".to_owned(),
                            output,
                        });
                    }
                    if words.as_slice() == ["link"] {
                        let filter = store.inspect_command(
                            receiving_socket,
                            if socket_scope { None } else { chat_scope },
                            &["link".to_owned()],
                        )?;
                        let domains = store.edit_link_domains(
                            receiving_socket,
                            if socket_scope { None } else { chat_scope },
                            "list",
                            &[],
                        )?;
                        return Ok(DispatchResult::Extension {
                            command: "set".to_owned(),
                            output: format!("{filter}\n{domains}"),
                        });
                    }
                    // Domain lists may be scoped to a verified private chat as well.
                    if let [name, operation, domains @ ..] = words.as_slice() {
                        if *name == "link"
                            && matches!(
                                *operation,
                                "list"
                                    | "lista"
                                    | "add"
                                    | "agregar"
                                    | "del"
                                    | "delete"
                                    | "remove"
                                    | "quitar"
                            )
                        {
                            let domain_scope = if socket_scope {
                                None
                            } else {
                                Some(group.unwrap_or(source))
                            };
                            let entries = domains
                                .iter()
                                .map(|word| (*word).to_owned())
                                .collect::<Vec<_>>();
                            let output = store.edit_link_domains(
                                receiving_socket,
                                domain_scope,
                                operation,
                                &entries,
                            )?;
                            return Ok(DispatchResult::Extension {
                                command: "set".to_owned(),
                                output,
                            });
                        }
                    }
                    if words.iter().any(|word| word.starts_with("--")) {
                        bail!("Opción de protección no reconocida");
                    }
                    let config_scope =
                        if socket_scope {
                            None
                        } else {
                            chat_scope.ok_or_else(|| {
                            anyhow!("Chat no verificado: utiliza --mod para configurar el socket")
                        }).map(Some)?
                        };
                    let args = words
                        .iter()
                        .map(|word| (*word).to_owned())
                        .collect::<Vec<_>>();
                    let output = store.configure_command(receiving_socket, config_scope, &args)?;
                    return Ok(DispatchResult::Extension {
                        command: "set".to_owned(),
                        output,
                    });
                }
                ["set", "antilink", ..] => {
                    bail!("Comando anterior retirado. Usa =/>set anti link");
                }
                ["set", rule, ..] if ANTI_RULES.contains(rule) => {
                    bail!("Usa =/>set anti <tipo> para configurar protecciones");
                }
                [rule, ..] if ANTI_RULES.contains(rule) => {
                    bail!("Usa =/>anti <tipo> para consultar, o =/>set anti <tipo> para modificar");
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
                ["ping"] if canonical_global_prefix => {
                    let admission = self.bot_admission(receiving_socket, group, kind)?;
                    if admission.owner_id != actor {
                        bail!("Socket owner authorization denied");
                    }
                    if !admission.decision.enabled {
                        bail!("Bot is disabled in this chat");
                    }
                    return Ok(DispatchResult::Extension {
                        command: "ping".to_owned(),
                        output: "Pong · Latencia IPC: pendiente".to_owned(),
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
                ["status" | "estado", "--global"] if self.unified_live_controls => {
                    let admission = self.bot_admission(receiving_socket, group, kind)?;
                    if admission.owner_id != actor {
                        bail!("Socket owner authorization denied");
                    }
                    return Ok(DispatchResult::Extension {
                        command: "status".to_owned(),
                        output: self.global_default_status().to_owned(),
                    });
                }
                ["status" | "estado", "--mod"] => {
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
                ["bot", "--mod", action @ ("on" | "off" | "inherit" | "encender" | "apagar" | "heredar")]
                | ["bot", action @ ("on" | "off" | "inherit" | "encender" | "apagar" | "heredar"), "--mod"] =>
                {
                    let policy = match *action {
                        "on" | "encender" => Some(true),
                        "off" | "apagar" => Some(false),
                        _ => None,
                    };
                    return self.change_bot_policy(
                        receiving_socket,
                        actor,
                        group,
                        kind,
                        "global",
                        policy,
                    );
                }
                ["bot", "--global", "on" | "off" | "inherit" | "encender" | "apagar" | "heredar"]
                | ["bot", "on" | "off" | "inherit" | "encender" | "apagar" | "heredar", "--global"]
                    if self.unified_live_controls =>
                {
                    bail!("--global es una política general, no el socket. Las escrituras globales requieren autoridad Vault.");
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
                ["bot"] => {
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
                ["bot", "--mod"] => {
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
                ["bot", "--global"] if self.unified_live_controls => {
                    let admission = self.bot_admission(receiving_socket, group, kind)?;
                    if admission.owner_id != actor {
                        bail!("Socket owner authorization denied");
                    }
                    return Ok(DispatchResult::Extension {
                        command: "bot".to_owned(),
                        output: self.global_default_status().to_owned(),
                    });
                }
                ["bot", "--global"] => {
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
                ["set"]
                | ["set", "--mod"]
                | ["set", "style" | "lang" | "language"]
                | ["set", "style" | "lang" | "language", "--mod"]
                | ["set", "--mod", "style" | "lang" | "language"] => {
                    if group.is_some() {
                        bail!("La configuración del menú se consulta en el chat privado del propietario");
                    }
                    let admission =
                        self.activation
                            .admission(receiving_socket, None, self.global_policy)?;
                    if admission.owner_id != actor {
                        bail!("Socket owner authorization denied");
                    }
                    let themes = self
                        .public_menu
                        .as_ref()
                        .ok_or_else(|| anyhow!("Menu preferences are unavailable"))?;
                    let preferences = themes.current_menu_preferences()?;
                    let field = tokens
                        .iter()
                        .find(|word| matches!(**word, "style" | "lang" | "language"));
                    let output = match field {
                        Some(&"style") => format!(
                            "Configuración actual · socket\nEstilo: {}",
                            preferences.style.as_str()
                        ),
                        Some(&"lang" | &"language") => format!(
                            "Configuración actual · socket\nIdioma: {}",
                            preferences.language.as_str()
                        ),
                        _ => format!(
                            "Configuración actual · socket\nEstilo: {}\nIdioma: {}\nMenú: {}",
                            preferences.style.as_str(),
                            preferences.language.as_str(),
                            themes.current_menu_mode()?.as_str()
                        ),
                    };
                    return Ok(DispatchResult::Extension {
                        command: "set".to_owned(),
                        output,
                    });
                }
                ["set", "--global"] => {
                    return Ok(DispatchResult::Extension {
                        command: "set".to_owned(),
                        output: "Predeterminado general del menú (fábrica, sin configuración Vault): menú web; alternativas classic y buttons; estilo matter; idioma es.".to_owned(),
                    });
                }
                ["set", "menu"] | ["set", "menu", "--mod"] | ["set", "--mod", "menu"] => {
                    if group.is_some() {
                        bail!("Menu configuration requires the owner's private chat");
                    }
                    let admission =
                        self.activation
                            .admission(receiving_socket, None, self.global_policy)?;
                    if admission.owner_id != actor {
                        bail!("Socket owner authorization denied");
                    }
                    let themes = self
                        .public_menu
                        .as_ref()
                        .ok_or_else(|| anyhow!("Menu preferences are unavailable"))?;
                    return Ok(DispatchResult::Extension {
                        command: "set".to_owned(),
                        output: format!(
                            "Menú predeterminado: {} (web|classic|buttons).",
                            themes.current_menu_mode()?.as_str()
                        ),
                    });
                }
                ["set", "menu", value]
                | ["set", "menu", "--mod", value]
                | ["set", "menu", value, "--mod"]
                | ["set", "--mod", "menu", value] => {
                    if group.is_some() {
                        bail!("Menu configuration requires the owner's private chat");
                    }
                    let admission =
                        self.activation
                            .admission(receiving_socket, None, self.global_policy)?;
                    if admission.owner_id != actor {
                        bail!("Socket owner authorization denied");
                    }
                    let themes = self
                        .public_menu
                        .as_ref()
                        .ok_or_else(|| anyhow!("Menu preferences are unavailable"))?;
                    themes.set_menu_mode(value)?;
                    let value = crate::menu::MenuMode::parse(value)?.as_str();
                    return Ok(DispatchResult::Extension {
                        command: "set".to_owned(),
                        output: format!(
                            "Menú predeterminado: {value}. Abre =/>menu para verificar."
                        ),
                    });
                }
                ["set", "style" | "lang" | "language", value]
                | ["set", "--mod", "style" | "lang" | "language", value]
                | ["set", "style" | "lang" | "language", "--mod", value]
                | ["set", "style" | "lang" | "language", value, "--mod"] => {
                    if group.is_some() {
                        bail!("Menu configuration requires the owner's private chat");
                    }
                    let admission =
                        self.activation
                            .admission(receiving_socket, None, self.global_policy)?;
                    if admission.owner_id != actor {
                        bail!("Socket owner authorization denied");
                    }
                    let themes = self
                        .public_menu
                        .as_ref()
                        .ok_or_else(|| anyhow!("Menu preferences are unavailable"))?;
                    let setting = if tokens.contains(&"style") {
                        "style"
                    } else {
                        "lang"
                    };
                    if setting == "style" {
                        themes.set_menu_style(value)?;
                    } else {
                        themes.set_menu_language(value)?;
                    }
                    return Ok(DispatchResult::Extension {
                        command: "set".to_owned(),
                        output: format!("Menú {setting}: {value}. Abre menu para verificar."),
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
                ["status" | "estado", socket]
                    if !matches!(*socket, "--debug" | "--global" | "--mod") =>
                {
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
                        && !matches!(*command, "roles" | "audit" | "vault")
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
