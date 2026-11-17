/*
 * Base-Matter — authenticated loopback transport for Rust WhatsApp controls.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use crate::whatsapp_bridge::{WhatsAppBridgeOutcome, WhatsAppControlBridge};
use anyhow::{bail, Result};
use std::{
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
    sync::Arc,
    time::Duration,
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
        if input.is_empty() || input.len() > MAX_FRAME_BYTES {
            return ControlIpcOutcome::Denied;
        }
        let Ok(text) = std::str::from_utf8(input) else {
            return ControlIpcOutcome::Denied;
        };
        match self.bridge.ingest_json(text) {
            Ok(WhatsAppBridgeOutcome::Dispatched(_)) => ControlIpcOutcome::Applied,
            Ok(WhatsAppBridgeOutcome::Ignored) => ControlIpcOutcome::Ignored,
            Ok(WhatsAppBridgeOutcome::Duplicate) => ControlIpcOutcome::Duplicate,
            Err(_) => ControlIpcOutcome::Denied,
        }
    }

    /// The provider must close its write half after sending one JSON frame.
    /// No HTTP server or public network listener is ever created.
    pub fn accept(&self, mut stream: TcpStream) -> Result<()> {
        stream.set_read_timeout(Some(Duration::from_secs(3)))?;
        stream.set_write_timeout(Some(Duration::from_secs(3)))?;
        let mut request = Vec::new();
        let result = match stream
            .take((MAX_FRAME_BYTES + 1) as u64)
            .read_to_end(&mut request)
        {
            Ok(_) => self.dispatch(&request),
            Err(_) => ControlIpcOutcome::Denied,
        };
        let response = serde_json::json!({
            "ok":result.permitted(),"outcome":result.as_str(),
        });
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
        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                // Refusals never expose frame contents or secret material.
                let _ = self.accept(stream);
            }
        }
        Ok(())
    }
}
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
