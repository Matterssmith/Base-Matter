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
}
impl WhatsAppControlIpc {
    pub fn new(bridge: Arc<WhatsAppControlBridge>) -> Self {
        Self {
            bridge,
            themes: Arc::new(ControlThemes::new(
                ControlTheme::new("Base-Matter", "base-matter", "✦")
                    .expect("Static Matterssmith theme must be valid"),
            )),
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
        let authenticated_group = if matches!(&outcome, ControlIpcOutcome::Applied) {
            serde_json::from_str::<WhatsAppControlFrame>(text)
                .ok()
                .map(|frame| frame.chat_jid)
                .filter(|chat| chat.ends_with("@g.us"))
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
                        .render_verified_root(crate::menu::MenuViewer {
                            user: &alias,
                            role: if owner { "Owner del socket" } else { "Usuario" },
                            status: "ACTIVADO",
                            socket: &socket_alias,
                            socket_owner: owner,
                        })
                        .ok()
                })
        } else {
            None
        };
        // All successful replies use the same per-chat renderer.
        // A menu preformatted through that registry is idempotent.
        let validated = verified_root
            .or(message)
            .and_then(|body| {
                self.themes
                    .decorate(authenticated_group.as_deref(), &body)
                    .ok()
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
        let response = if let Some(message) = message {
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
