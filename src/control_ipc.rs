/*
 * Base-Matter — authenticated loopback transport for Rust WhatsApp controls.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use crate::control::DispatchResult;
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
}
impl WhatsAppControlIpc {
    pub fn new(bridge: Arc<WhatsAppControlBridge>) -> Self {
        Self { bridge }
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
                    "Base-Matter: política de comandos: {} (origen: {source:?}). El encendido del socket es independiente.",
                    if enabled { "permitida" } else { "denegada" }
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
                    "Base-Matter: grupo {} (origen: {source:?}). Socket y política se controlan por separado.",
                    if enabled { "habilitado" } else { "deshabilitado" }
                )),
            ),
            Ok(WhatsAppBridgeOutcome::Dispatched(_)) => (ControlIpcOutcome::Applied, None),
            Ok(WhatsAppBridgeOutcome::Ignored) => (ControlIpcOutcome::Ignored, None),
            Ok(WhatsAppBridgeOutcome::Duplicate) => (ControlIpcOutcome::Duplicate, None),
            Err(_) => (ControlIpcOutcome::Denied, None),
        };
        let (outcome, message) = result;
        let validated = message.filter(|text| {
            !text.is_empty()
                && text.len() <= 760
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
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
