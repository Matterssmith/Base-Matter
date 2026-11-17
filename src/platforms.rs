/*
 * Base-Matter — shared platform driver routing (WhatsApp/Telegram/Discord).
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use crate::connections::ConnectionDriver;
use crate::sessions::{OutboundMessage, SessionRecord, TransportDriver};
use anyhow::{anyhow, bail, Result};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// The three currently planned native transports. Adding another transport
/// requires an explicit contract change; unknown platform IDs fail closed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MatterPlatform {
    WhatsApp,
    Telegram,
    Discord,
}

impl MatterPlatform {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WhatsApp => "whatsapp",
            Self::Telegram => "telegram",
            Self::Discord => "discord",
        }
    }

    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "whatsapp" => Ok(Self::WhatsApp),
            "telegram" => Ok(Self::Telegram),
            "discord" => Ok(Self::Discord),
            _ => bail!("Unknown platform transport"),
        }
    }
}

/// Immutable after registration. A duplicate may not replace a trusted
/// driver; reconfiguration requires an explicit service lifecycle transition.
/// Registry membership is NOT authentication, authorization or revocation.
#[derive(Default)]
pub struct PlatformConnectionDrivers {
    drivers: RwLock<HashMap<MatterPlatform, Arc<dyn ConnectionDriver>>>,
}

impl PlatformConnectionDrivers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &self,
        platform: MatterPlatform,
        driver: Arc<dyn ConnectionDriver>,
    ) -> Result<()> {
        let mut drivers = self
            .drivers
            .write()
            .map_err(|_| anyhow!("Platform connection registry unavailable"))?;
        if drivers.contains_key(&platform) {
            bail!("Platform connection driver already registered");
        }
        drivers.insert(platform, driver);
        Ok(())
    }

    fn driver_for(&self, session: &SessionRecord) -> Result<Arc<dyn ConnectionDriver>> {
        let platform = MatterPlatform::parse(&session.platform)?;
        let drivers = self
            .drivers
            .read()
            .map_err(|_| anyhow!("Platform connection registry unavailable"))?;
        drivers
            .get(&platform)
            .cloned()
            .ok_or_else(|| anyhow!("Platform connection driver unavailable"))
    }
}

impl ConnectionDriver for PlatformConnectionDrivers {
    fn connect(&self, session: &SessionRecord, credential: &[u8]) -> Result<()> {
        self.driver_for(session)?.connect(session, credential)
    }

    fn disconnect(&self, session: &SessionRecord) -> Result<()> {
        self.driver_for(session)?.disconnect(session)
    }

    fn is_connected(&self, session: &SessionRecord) -> Result<bool> {
        self.driver_for(session)?.is_connected(session)
    }
}

/// Shared MessageService transport selector. One socket's message cannot be
/// passed to a different socket's session. Providers must additionally fence
/// session generations and verify authenticated destinations before send.
#[derive(Default)]
pub struct PlatformTransports {
    drivers: RwLock<HashMap<MatterPlatform, Arc<dyn TransportDriver>>>,
}

impl PlatformTransports {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &self,
        platform: MatterPlatform,
        driver: Arc<dyn TransportDriver>,
    ) -> Result<()> {
        let mut drivers = self
            .drivers
            .write()
            .map_err(|_| anyhow!("Platform transport registry unavailable"))?;
        if drivers.contains_key(&platform) {
            bail!("Platform transport driver already registered");
        }
        drivers.insert(platform, driver);
        Ok(())
    }

    fn driver_for(&self, session: &SessionRecord) -> Result<Arc<dyn TransportDriver>> {
        let platform = MatterPlatform::parse(&session.platform)?;
        let drivers = self
            .drivers
            .read()
            .map_err(|_| anyhow!("Platform transport registry unavailable"))?;
        drivers
            .get(&platform)
            .cloned()
            .ok_or_else(|| anyhow!("Platform transport driver unavailable"))
    }
}

impl TransportDriver for PlatformTransports {
    fn transmit(&self, session: &SessionRecord, message: &OutboundMessage) -> Result<String> {
        if session.socket_id != message.socket_id {
            bail!("Message and session socket mismatch");
        }
        self.driver_for(session)?.transmit(session, message)
    }
}
