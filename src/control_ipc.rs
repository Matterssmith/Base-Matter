/*
 * Base-Matter — authenticated loopback transport for Rust WhatsApp controls.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use crate::control::DispatchResult;
use crate::control_style::{ControlTheme, ControlThemes};
use crate::whatsapp_bridge::WhatsAppControlFrame;
use crate::whatsapp_bridge::{WhatsAppBridgeOutcome, WhatsAppControlBridge};
use anyhow::{bail, Result};
use std::{
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
    sync::Arc,
    time::{Duration, Instant},
};
const MAX_FRAME_BYTES: usize = 16 * 1024;

#[derive(Debug, PartialEq, Eq)]
pub enum ControlIpcOutcome {
    Applied,
    Ignored,
    Duplicate,
    Denied,
}
impl ControlIpcOutcome {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Applied => "applied",
            Self::Ignored => "ignored",
            Self::Duplicate => "duplicate",
            Self::Denied => "denied",
        }
    }
    pub fn permitted(&self) -> bool {
        !matches!(self, Self::Denied)
    }
}
pub struct WhatsAppControlIpc {
    bridge: Arc<WhatsAppControlBridge>,
    themes: Arc<ControlThemes>,
    console_rank: bool,
}
impl WhatsAppControlIpc {
    pub fn new(bridge: Arc<WhatsAppControlBridge>) -> Self {
        Self {
            bridge,
            themes: Arc::new(ControlThemes::new(
                ControlTheme::new("Base-Matter", "base-matter", "✦")
                    .expect("Static Matterssmith theme must be valid"),
            )),
            console_rank: false,
        }
    }

    /// Compatibility helper for changing only the fallback style.
    pub fn with_theme(mut self, theme: ControlTheme) -> Self {
        self.themes = Arc::new(ControlThemes::new(theme));
        self
    }

    /// Share the exact scoped registry with the menu command.
    /// This guarantees that status, menu, audit and policy replies use
    /// the same selected theme for their verified chat scope.
    pub fn with_themes(mut self, themes: Arc<ControlThemes>) -> Self {
        self.themes = themes;
        self
    }

    /// Local diagnostic metadata; not part of any chat reply or permission.
    pub fn with_console_rank(mut self, enabled: bool) -> Self {
        self.console_rank = enabled;
        self
    }

    pub fn dispatch(&self, input: &[u8]) -> ControlIpcOutcome {
        self.dispatch_with_message(input).0
    }

    fn dispatch_with_message(&self, input: &[u8]) -> (ControlIpcOutcome, Option<String>) {
        if input.is_empty() || input.len() > MAX_FRAME_BYTES {
            return (ControlIpcOutcome::Denied, None);
        }
        let Ok(text) = std::str::from_utf8(input) else {
            return (ControlIpcOutcome::Denied, None);
        };
        let result = match self.bridge.ingest_json(text) {
            Ok(WhatsAppBridgeOutcome::Dispatched(DispatchResult::Extension {
                output, ..
            })) => (ControlIpcOutcome::Applied, Some(output)),
            Ok(WhatsAppBridgeOutcome::Dispatched(DispatchResult::Status {
                enabled, source, ..
            })) => (
                ControlIpcOutcome::Applied,
                Some(format!(
                    "Estado del chat: BOT {} (origen: {source:?}). {}",
                    if enabled { "ACTIVADO" } else { "APAGADO" },
                    if source == crate::activation::ActivationSource::PoweredOff {
                        "El socket está apagado."
                    } else {
                        "La activación respeta la política de este chat."
                    }
                )),
            ),
            Ok(WhatsAppBridgeOutcome::Dispatched(DispatchResult::Control {
                powered, ..
            })) => (
                ControlIpcOutcome::Applied,
                Some(format!(
                    "Socket {}. La política de comandos es independiente.",
                    if powered { "encendido" } else { "apagado" }
                )),
            ),
            Ok(WhatsAppBridgeOutcome::Dispatched(DispatchResult::BotGlobalStatus {
                socket_powered, default_policy, group_policy,
                private_policy, channel_policy, community_policy,
            })) => (
                ControlIpcOutcome::Applied,
                Some(format!(
                    "Configuración global del socket\nSocket: {}\nPredeterminado: {}\nGrupos: {}\nPrivados externos: {}\nCanales: {}\nAvisos comunitarios: {} (requiere metadatos verificados)\nEstas políticas NO conceden rangos.",
                    if socket_powered { "ENCENDIDO" } else { "APAGADO" },
                    format_policy(default_policy),
                    format_surface_policy(group_policy),
                    format_surface_policy(private_policy),
                    format_surface_policy(channel_policy),
                    format_surface_policy(community_policy),
                )),
            ),
            Ok(WhatsAppBridgeOutcome::Dispatched(DispatchResult::BotPolicyChanged {
                scope, policy,
            })) => (
                ControlIpcOutcome::Applied,
                Some(format!(
                    "Política del bot para {scope}: {}.",
                    match policy {
                        Some(true) => "activada",
                        Some(false) => "desactivada",
                        None => "heredar",
                    }
                )),
            ),
            Ok(WhatsAppBridgeOutcome::Dispatched(DispatchResult::GroupPolicyChanged {
                policy, ..
            })) => (
                ControlIpcOutcome::Applied,
                Some(format!(
                    "Política de grupo {}.",
                    match policy {
                        Some(true) => "habilitada",
                        Some(false) => "deshabilitada",
                        None => "heredada",
                    }
                )),
            ),
            Ok(WhatsAppBridgeOutcome::Dispatched(DispatchResult::GroupStatus {
                enabled, source, ..
            })) => (
                ControlIpcOutcome::Applied,
                Some(format!(
                    "Estado del grupo: BOT {} (origen: {source:?}). {}",
                    if enabled { "ACTIVADO" } else { "APAGADO" },
                    if source == crate::activation::ActivationSource::PoweredOff {
                        "El socket está apagado; una política de grupo no lo enciende."
                    } else {
                        "El bot respeta la política de este chat."
                    }
                )),
            ),
            Ok(WhatsAppBridgeOutcome::Dispatched(DispatchResult::SocketRoleChanged {
                role_id,
                changed,
                ..
            })) => (
                ControlIpcOutcome::Applied,
                Some(format!(
                    "Rango {} {} localmente. No concede permisos operativos todavía; Vault-Matter aún no sincronizado.",
                    role_id,
                    if changed { "actualizado" } else { "sin cambios" }
                )),
            ),
            Ok(WhatsAppBridgeOutcome::Dispatched(DispatchResult::Inactive)) => {
                (ControlIpcOutcome::Ignored, None)
            }
            Ok(WhatsAppBridgeOutcome::Dispatched(_)) => (ControlIpcOutcome::Applied, None),
            Ok(WhatsAppBridgeOutcome::Ignored) => (ControlIpcOutcome::Ignored, None),
            Ok(WhatsAppBridgeOutcome::AntiObserved) => {
                (ControlIpcOutcome::Applied, None)
            }
            Ok(WhatsAppBridgeOutcome::Duplicate) => (ControlIpcOutcome::Duplicate, None),
            Err(_) => (ControlIpcOutcome::Denied, None),
        };
        let (outcome, message) = result;
        // We select a template from the chat identifier ONLY after the
        // bridge has accepted and authenticated the signed frame.
        let authenticated_chat = if matches!(&outcome, ControlIpcOutcome::Applied) {
            serde_json::from_str::<WhatsAppControlFrame>(text)
                .ok()
                .map(|frame| frame.chat_jid)
        } else {
            None
        };
        let authenticated_actor = if matches!(&outcome, ControlIpcOutcome::Applied) {
            serde_json::from_str::<WhatsAppControlFrame>(text)
                .ok()
                .map(|frame| if frame.from_me { frame.owner_jid } else { frame.actor_id })
        } else {
            None
        };
        // The HMAC bridge has already authenticated, replay-checked and
        // authorized the menu before we inspect this frame again.
        // Actor/JID are never derived from a message body or display name.
        let verified_root = if matches!(&outcome, ControlIpcOutcome::Applied) {
            serde_json::from_str::<WhatsAppControlFrame>(text)
                .ok()
                .filter(|frame| {
                    matches!(
                        frame.text.as_str(),
                        "=/>menu"
                            | "=/>menú"
                            | "=/>menu page 1"
                            | "=/>menu page 2"
                            | "=/>menu page 3"
                    )
                })
                .and_then(|frame| {
                    let owner = frame.from_me;
                    let alias = if owner {
                        format!("@{}", frame.actor_id.chars().take(24).collect::<String>())
                    } else {
                        "@usuario".to_owned()
                    };
                    let socket_alias = if owner {
                        frame.socket_id.chars().take(32).collect::<String>()
                    } else {
                        "Privado".to_owned()
                    };
                    self.themes
                        .render_verified_chat_root_for(
                            authenticated_chat.as_deref(),
                            authenticated_actor.as_deref(),
                            crate::menu::MenuViewer {
                                user: &alias,
                                role: if owner { "Owner del socket" } else { "Usuario" },
                                status: "ACTIVADO",
                                socket: &socket_alias,
                                socket_owner: owner,
                            },
                        )
                        .ok()
                })
        } else {
            None
        };
        // All successful replies use the same per-chat renderer.
        // A menu preformatted through that registry is idempotent.
        // Re-render admitted menu/help with the verified caller's preferences.
        // Explicitly unsupported user scopes must retain their denial text.
        let validated = verified_root
            .or_else(|| {
                if !matches!(&outcome, ControlIpcOutcome::Applied) || message.is_none() {
                    return None;
                }
                let frame = serde_json::from_str::<WhatsAppControlFrame>(text).ok()?;
                let mut words = frame.text.split_whitespace();
                let kind = match words.next()? {
                    "=/>menu" | "=/>menú" => "menu",
                    "=/>help" | "=/>ayuda" | "=/>h" => "help",
                    _ => return None,
                };
                let args: Vec<_> = words.collect();
                if args.iter().any(|arg| matches!(*arg, "--user" | "--chat" | "--socket" | "--mod" | "--global" | "--quoted")) {
                    return None;
                }
                let section = format!("{kind} {}", args.join(" "));
                self.themes
                    .render_for(authenticated_chat.as_deref(), authenticated_actor.as_deref(), section.trim_end())
                    .ok()
            })
            .or(message)
            .and_then(|body| {
                let status = serde_json::from_str::<WhatsAppControlFrame>(text)
                    .ok()
                    .is_some_and(|frame| {
                        matches!(
                            frame.text.split_whitespace().next(),
                            Some("=/>status" | "=/>estado")
                        )
                    });
                if status {
                    self.themes
                        .decorate_status_for(authenticated_chat.as_deref(), authenticated_actor.as_deref(), &body)
                        .ok()
                } else {
                    self.themes
                        .decorate_for(authenticated_chat.as_deref(), authenticated_actor.as_deref(), &body)
                        .ok()
                }
            })
            .filter(|body| {
                !body.is_empty()
                    && body.len() <= 3800
                    && !body.chars().any(|c| c.is_control() && c != '\n')
            });
        (outcome, validated)
    }

    /// The provider must close its write half after sending one JSON frame.
    /// No HTTP server or public network listener is ever created.
    pub fn accept(&self, mut stream: TcpStream) -> Result<()> {
        stream.set_write_timeout(Some(Duration::from_secs(3)))?;
        let deadline = Instant::now() + Duration::from_secs(4);
        let mut request = Vec::new();
        let (result, message) = loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                break (ControlIpcOutcome::Denied, None);
            }
            stream.set_read_timeout(Some(remaining.min(Duration::from_secs(1))))?;
            let mut chunk = [0_u8; 2048];
            match stream.read(&mut chunk) {
                Ok(0) => break self.dispatch_with_message(&request),
                Ok(n) => {
                    if request.len() + n > MAX_FRAME_BYTES {
                        break (ControlIpcOutcome::Denied, None);
                    }
                    request.extend_from_slice(&chunk[..n]);
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(_) => break (ControlIpcOutcome::Denied, None),
            }
        };
        // Only the authenticated, replay-checked root menu receives
        // typed presentation metadata. Other IPC outcomes remain unchanged.
        let menu_mode = if matches!(&result, ControlIpcOutcome::Applied) {
            serde_json::from_slice::<WhatsAppControlFrame>(&request)
                .ok()
                .filter(|frame| matches!(frame.text.as_str(), "=/>menu" | "=/>menú"))
                .and_then(|_| self.themes.current_menu_mode().ok())
        } else {
            None
        };
        let menu_items = if matches!(&result, ControlIpcOutcome::Applied) {
            serde_json::from_slice::<WhatsAppControlFrame>(&request)
                .ok()
                .filter(|frame| {
                    matches!(
                        frame.text.split_whitespace().next(),
                        Some("=/>menu" | "=/>menú")
                    )
                })
                .and_then(|frame| {
                    let options = self
                        .themes
                        .effective_preferences(Some(&frame.chat_jid))
                        .ok()?;
                    Some(crate::menu::button_menu_items(
                        &frame.text,
                        options.language,
                    ))
                })
        } else {
            None
        };
        let mut response = if let Some(message) = message {
            if let Some(mode) = menu_mode {
                serde_json::json!({
                    "ok":result.permitted(),"outcome":result.as_str(),
                    "message":message,"menu_mode":mode.as_str(),
                })
            } else {
                serde_json::json!({
                    "ok":result.permitted(),"outcome":result.as_str(),"message":message,
                })
            }
        } else {
            serde_json::json!({
                "ok":result.permitted(),"outcome":result.as_str(),
            })
        };
        if let Some(items) = menu_items {
            if response.get("message").is_some() {
                response["menu_items"] = serde_json::Value::Array(items);
            }
        }
        // Only admitted commands whose presentation is built by the provider
        // need the effective style as typed metadata; it grants no authority.
        if matches!(result, ControlIpcOutcome::Applied) {
            if let Ok(frame) = serde_json::from_slice::<WhatsAppControlFrame>(&request) {
                if matches!(
                    frame.text.split_whitespace().next(),
                    Some("=/>menu" | "=/>menú" | "=/>ping" | "=/>start" | "=/>add++")
                ) {
                    let actor = if frame.from_me {
                        &frame.owner_jid
                    } else {
                        &frame.actor_id
                    };
                    if let Ok(preferences) = self
                        .themes
                        .effective_preferences_for(Some(&frame.chat_jid), Some(actor))
                    {
                        response["response_style"] = preferences.style.as_str().into();
                    }
                }
            }
        }
        // A command denial can still originate from an authenticated global
        // owner. Do not infer rank from the dispatch result. Never check passive
        // observations, ignored chats or duplicate messages.
        if self.console_rank
            && matches!(
                result,
                ControlIpcOutcome::Applied | ControlIpcOutcome::Denied
            )
            && serde_json::from_slice::<WhatsAppControlFrame>(&request)
                .is_ok_and(|frame| self.bridge.console_global_owner(&frame))
        {
            response["global_owner_verified"] = serde_json::Value::Bool(true);
        }
        let encoded = serde_json::to_vec(&response)?;
        stream.write_all(&encoded)?;
        Ok(())
    }

    /// Reject public binds; one sequential worker bounds memory and CPU.
    pub fn serve(&self, addr: SocketAddrV4) -> Result<()> {
        if *addr.ip() != Ipv4Addr::LOCALHOST || addr.port() == 0 {
            bail!("WhatsApp control must bind explicitly to local loopback");
        }
        let listener = TcpListener::bind(addr)?;
        for stream in listener.incoming().flatten() {
            // Refusals never expose frame contents or secret material.
            let _ = self.accept(stream);
        }
        Ok(())
    }
}
fn format_policy(value: Option<bool>) -> &'static str {
    match value {
        Some(true) => "activado",
        Some(false) => "apagado",
        None => "sin predeterminado (denegar si no hay excepción)",
    }
}

fn format_surface_policy(value: Option<bool>) -> &'static str {
    match value {
        Some(true) => "activado explícitamente",
        Some(false) => "apagado explícitamente",
        None => "APAGADO por defecto (requiere habilitación expresa)",
    }
}

// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
