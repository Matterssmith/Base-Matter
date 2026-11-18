/*
 * Base-Matter — authenticated loopback transport for Rust WhatsApp controls.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use crate::control::DispatchResult;
use crate::control_style::ControlTheme;
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
    theme: ControlTheme,
}
impl WhatsAppControlIpc {
    pub fn new(bridge: Arc<WhatsAppControlBridge>) -> Self {
        Self {
            bridge,
            theme: ControlTheme::new("Base-Matter", "base-matter", "✦")
                .expect("Static Matterssmith theme must be valid"),
        }
    }

    /// Install the same default theme used by the command menu.
    pub fn with_theme(mut self, theme: ControlTheme) -> Self {
        self.theme = theme;
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
                    "Base-Matter | Estado del chat: BOT {} (origen: {source:?}). {}",
                    if enabled { "ACTIVADO" } else { "APAGADO" },
                    if source == crate::activation::ActivationSource::PoweredOff {
                        "El socket está apagado."
                    } else {
                        "La política del bot y el encendido del socket son independientes."
                    }
                )),
            ),
            Ok(WhatsAppBridgeOutcome::Dispatched(DispatchResult::Control {
                powered, ..
            })) => (
                ControlIpcOutcome::Applied,
                Some(format!(
                    "Base-Matter: socket {}. La política de comandos es independiente.",
                    if powered { "encendido" } else { "apagado" }
                )),
            ),
            Ok(WhatsAppBridgeOutcome::Dispatched(DispatchResult::BotGlobalStatus {
                socket_powered, default_policy, group_policy,
                private_policy, channel_policy, community_policy,
            })) => (
                ControlIpcOutcome::Applied,
                Some(format!(
                    "Base-Matter | Configuración global del socket\nSocket: {}\nPredeterminado: {}\nGrupos: {}\nPrivados externos: {}\nCanales: {}\nAvisos comunitarios: {} (requiere metadatos verificados)\nEstas políticas NO conceden rangos.",
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
                    "Base-Matter: política del bot para {scope}: {}. El socket conserva su encendido independiente.",
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
                    "Base-Matter: política de grupo {}. El encendido del socket es independiente.",
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
                    "Base-Matter | Estado del grupo: BOT {} (origen: {source:?}). {}",
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
                    "Base-Matter: rango {} {} localmente. No concede permisos operativos todavía; Vault-Matter aún no sincronizado.",
                    role_id,
                    if changed { "actualizado" } else { "sin cambios" }
                )),
            ),
            Ok(WhatsAppBridgeOutcome::Dispatched(DispatchResult::Inactive)) => {
                (ControlIpcOutcome::Ignored, None)
            }
            Ok(WhatsAppBridgeOutcome::Dispatched(_)) => (ControlIpcOutcome::Applied, None),
            Ok(WhatsAppBridgeOutcome::Ignored) => (ControlIpcOutcome::Ignored, None),
            Ok(WhatsAppBridgeOutcome::Duplicate) => (ControlIpcOutcome::Duplicate, None),
            Err(_) => (ControlIpcOutcome::Denied, None),
        };
        let (outcome, message) = result;
        // Every reply uses the same Matterssmith layout. The menu is
        // already decorated, so the formatter is intentionally idempotent.
        let validated = message
            .and_then(|text| self.theme.decorate(&text).ok())
            .filter(|text| {
                !text.is_empty()
                    && text.len() <= 3800
                    && !text.chars().any(|c| c.is_control() && c != '\n')
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
        let response = if let Some(message) = message {
            serde_json::json!({
                "ok":result.permitted(),"outcome":result.as_str(),"message":message,
            })
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
