/*!
Matter-Rust — common, platform-neutral event ingress and connector registry.

WhatsApp, Telegram and Discord have separate actual transports. Social-Matter
is reserved but cannot connect until an explicitly reviewed release.
An event normalized by this module is still UNAUTHENTICATED; the verified
Base-Matter ingress and Vault authorization are separate, mandatory boundaries.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
*/
pub mod web;

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::sync::Arc;

pub const ENGINE_NAME: &str = "matter-rust";
pub const EVENT_TEXT_LIMIT: usize = 4096;
pub const IDENTIFIER_LIMIT: usize = 128;
pub const REPLAY_WINDOW: usize = 4096;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Platform {
    WhatsApp,
    Telegram,
    Discord,
    SocialMatter,
}

impl Platform {
    pub const fn name(self) -> &'static str {
        match self {
            Self::WhatsApp => "whatsapp",
            Self::Telegram => "telegram",
            Self::Discord => "discord",
            Self::SocialMatter => "social-matter",
        }
    }
    pub fn parse(input: &str) -> Result<Self, EngineError> {
        match input {
            "whatsapp" => Ok(Self::WhatsApp),
            "telegram" => Ok(Self::Telegram),
            "discord" => Ok(Self::Discord),
            "social-matter" => Ok(Self::SocialMatter),
            _ => Err(EngineError::UnknownPlatform),
        }
    }
    pub const fn production_enabled(self) -> bool {
        // Production support means connector admission is allowed; it does
        // not mean the connector is implemented, installed or authenticated.
        !matches!(self, Self::SocialMatter)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SessionKey {
    platform: Platform,
    socket_id: String,
}

impl SessionKey {
    pub fn new(platform: Platform, socket_id: impl Into<String>) -> Result<Self, EngineError> {
        let socket_id = socket_id.into();
        if !valid_identifier(&socket_id) || socket_id.contains("..") {
            return Err(EngineError::InvalidIdentifier);
        }
        Ok(Self {
            platform,
            socket_id,
        })
    }
    pub fn platform(&self) -> Platform {
        self.platform
    }
    pub fn socket_id(&self) -> &str {
        &self.socket_id
    }
}

fn valid_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= IDENTIFIER_LIMIT
        && value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"@._:-".contains(&c))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EngineError {
    UnknownPlatform,
    PlatformNotEnabled,
    InvalidIdentifier,
    InvalidContent,
    DriverAlreadyRegistered,
    DriverUnavailable,
    DriverNotReady,
    SessionMismatch,
    ReplayedMessage,
    PermissionRequired,
}

impl fmt::Display for EngineError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{self:?}")
    }
}
impl std::error::Error for EngineError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConnectionState {
    Offline,
    Starting,
    Connected,
    Interrupted,
}

/// The descriptor is reported by the adapter, not an independent verification.
pub trait PlatformAdapter: Send + Sync {
    fn session(&self) -> SessionKey;
    fn connection_state(&self) -> ConnectionState;
    fn family(&self) -> &'static str;
}

/// Inbound events are intentionally public *only* as untrusted observations.
/// Do not use sender_id or text to assign identity, control roles or tokens.
#[derive(Clone, Debug)]
pub struct UntrustedInbound {
    pub session: SessionKey,
    pub chat_id: String,
    pub sender_id: String,
    pub message_id: String,
    pub text: String,
    pub edited: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservedInbound {
    pub platform: Platform,
    pub socket_id: String,
    pub chat_id: String,
    pub sender_id: String,
    pub message_id: String,
    pub text: String,
    pub edited: bool,
}

impl UntrustedInbound {
    fn validate(&self) -> Result<(), EngineError> {
        if !valid_identifier(&self.chat_id)
            || !valid_identifier(&self.sender_id)
            || !valid_identifier(&self.message_id)
            || self.text.len() > EVENT_TEXT_LIMIT
            || self.text.chars().any(|c| c == '\0')
        {
            return Err(EngineError::InvalidContent);
        }
        Ok(())
    }
}

type ReplayKey = (SessionKey, String, String, String, bool);

/// One engine instance serves many independent platforms and sockets.
/// No transport SDK is linked here, so no socket is opened by construction.
pub struct MatterRust {
    drivers: HashMap<SessionKey, Arc<dyn PlatformAdapter>>,
    replay: HashSet<ReplayKey>,
    replay_order: VecDeque<ReplayKey>,
}

impl Default for MatterRust {
    fn default() -> Self {
        Self::new()
    }
}

impl MatterRust {
    pub fn new() -> Self {
        Self {
            drivers: HashMap::new(),
            replay: HashSet::new(),
            replay_order: VecDeque::new(),
        }
    }

    /// Requires a separately provisioned, credential-safe platform adapter.
    /// Will not replace an existing adapter (no silent session hijack).
    pub fn register(&mut self, driver: Arc<dyn PlatformAdapter>) -> Result<(), EngineError> {
        let key = driver.session();
        if !key.platform.production_enabled() {
            return Err(EngineError::PlatformNotEnabled);
        }
        if self.drivers.contains_key(&key) {
            return Err(EngineError::DriverAlreadyRegistered);
        }
        self.drivers.insert(key, driver);
        Ok(())
    }

    pub fn registered_sessions(&self) -> usize {
        self.drivers.len()
    }

    pub fn connection_state(&self, session: &SessionKey) -> Result<ConnectionState, EngineError> {
        let driver = self
            .drivers
            .get(session)
            .ok_or(EngineError::DriverUnavailable)?;
        if driver.session() != *session {
            return Err(EngineError::SessionMismatch);
        }
        Ok(driver.connection_state())
    }

    /// Validate and deduplicate a text observation only. This is NOT an
    /// authorization check, signature validation, or durable replay store.
    /// No privileged commands can execute through this API.
    pub fn observe(&mut self, raw: UntrustedInbound) -> Result<ObservedInbound, EngineError> {
        raw.validate()?;
        let driver = self
            .drivers
            .get(&raw.session)
            .ok_or(EngineError::DriverUnavailable)?;
        if driver.session() != raw.session {
            return Err(EngineError::SessionMismatch);
        }
        if driver.connection_state() != ConnectionState::Connected {
            return Err(EngineError::DriverNotReady);
        }
        let replay_key = (
            raw.session.clone(),
            raw.chat_id.clone(),
            raw.sender_id.clone(),
            raw.message_id.clone(),
            raw.edited,
        );
        if self.replay.contains(&replay_key) {
            return Err(EngineError::ReplayedMessage);
        }
        self.replay.insert(replay_key.clone());
        self.replay_order.push_back(replay_key);
        if self.replay_order.len() > REPLAY_WINDOW {
            if let Some(expired) = self.replay_order.pop_front() {
                self.replay.remove(&expired);
            }
        }
        Ok(ObservedInbound {
            platform: raw.session.platform(),
            socket_id: raw.session.socket_id().to_owned(),
            chat_id: raw.chat_id,
            sender_id: raw.sender_id,
            message_id: raw.message_id,
            text: raw.text,
            edited: raw.edited,
        })
    }

    /// This phase intentionally cannot send a message or grant a role.
    /// Mutating operations will require verified Base ingress + Vault proof.
    pub fn require_mutating_authority(&self, _event: &ObservedInbound) -> Result<(), EngineError> {
        Err(EngineError::PermissionRequired)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SimulatedAdapter {
        key: SessionKey,
        state: ConnectionState,
    }

    impl PlatformAdapter for SimulatedAdapter {
        fn session(&self) -> SessionKey {
            self.key.clone()
        }
        fn connection_state(&self) -> ConnectionState {
            self.state
        }
        fn family(&self) -> &'static str {
            "synthetic-test-only"
        }
    }

    fn adapter(
        platform: Platform,
        socket: &str,
        state: ConnectionState,
    ) -> Arc<dyn PlatformAdapter> {
        Arc::new(SimulatedAdapter {
            key: SessionKey::new(platform, socket).unwrap(),
            state,
        })
    }

    fn event(platform: Platform, socket: &str, id: &str) -> UntrustedInbound {
        UntrustedInbound {
            session: SessionKey::new(platform, socket).unwrap(),
            chat_id: "group_123".into(),
            sender_id: "member_456".into(),
            message_id: id.into(),
            text: "=/>set role owner --global".into(),
            edited: false,
        }
    }

    #[test]
    fn keeps_three_platforms_and_sockets_separate() {
        let mut engine = MatterRust::new();
        for (p, socket) in [
            (Platform::WhatsApp, "wa-test"),
            (Platform::Telegram, "tg-test"),
            (Platform::Discord, "dc-test"),
        ] {
            engine
                .register(adapter(p, socket, ConnectionState::Connected))
                .unwrap();
        }
        assert_eq!(engine.registered_sessions(), 3);
        for (p, socket) in [
            (Platform::WhatsApp, "wa-test"),
            (Platform::Telegram, "tg-test"),
            (Platform::Discord, "dc-test"),
        ] {
            let observation = engine.observe(event(p, socket, "msg-001")).unwrap();
            assert_eq!(observation.platform, p);
            assert_eq!(observation.socket_id, socket);
            assert_eq!(
                engine.require_mutating_authority(&observation),
                Err(EngineError::PermissionRequired)
            );
        }
    }

    #[test]
    fn rejects_unauthorized_social_transport_and_duplicate_sockets() {
        let mut engine = MatterRust::new();
        assert_eq!(
            engine.register(adapter(
                Platform::SocialMatter,
                "social-1",
                ConnectionState::Connected
            )),
            Err(EngineError::PlatformNotEnabled)
        );
        engine
            .register(adapter(
                Platform::WhatsApp,
                "wa-1",
                ConnectionState::Connected,
            ))
            .unwrap();
        assert_eq!(
            engine.register(adapter(
                Platform::WhatsApp,
                "wa-1",
                ConnectionState::Connected
            )),
            Err(EngineError::DriverAlreadyRegistered)
        );
        assert_eq!(Platform::parse("social-matter"), Ok(Platform::SocialMatter));
        assert_eq!(Platform::parse("other"), Err(EngineError::UnknownPlatform));
    }

    #[test]
    fn replay_is_scoped_by_platform_and_socket() {
        let mut engine = MatterRust::new();
        engine
            .register(adapter(
                Platform::WhatsApp,
                "wa-1",
                ConnectionState::Connected,
            ))
            .unwrap();
        engine
            .register(adapter(
                Platform::WhatsApp,
                "wa-2",
                ConnectionState::Connected,
            ))
            .unwrap();
        engine
            .observe(event(Platform::WhatsApp, "wa-1", "id-1"))
            .unwrap();
        assert_eq!(
            engine
                .observe(event(Platform::WhatsApp, "wa-1", "id-1"))
                .unwrap_err(),
            EngineError::ReplayedMessage
        );
        engine
            .observe(event(Platform::WhatsApp, "wa-2", "id-1"))
            .unwrap();
        // The first edited event is different from the original and is not
        // silently discarded; subsequent same-revision duplicate is rejected.
        let mut edit = event(Platform::WhatsApp, "wa-1", "id-1");
        edit.edited = true;
        let observed = engine.observe(edit.clone()).unwrap();
        assert!(observed.edited);
        assert_eq!(
            engine.observe(edit).unwrap_err(),
            EngineError::ReplayedMessage
        );
    }

    #[test]
    fn invalid_content_and_missing_driver_are_rejected() {
        let mut engine = MatterRust::new();
        engine
            .register(adapter(
                Platform::Telegram,
                "tg-1",
                ConnectionState::Offline,
            ))
            .unwrap();
        assert_eq!(
            engine
                .observe(event(Platform::Telegram, "tg-1", "id-1"))
                .unwrap_err(),
            EngineError::DriverNotReady
        );
        assert_eq!(
            engine
                .observe(event(Platform::Discord, "dc-1", "id-1"))
                .unwrap_err(),
            EngineError::DriverUnavailable
        );
        assert!(SessionKey::new(Platform::WhatsApp, "../secret").is_err());
        let mut bad = event(Platform::Telegram, "tg-1", "id-1");
        bad.text = "\0".into();
        assert_eq!(
            engine.observe(bad).unwrap_err(),
            EngineError::InvalidContent
        );
    }
}
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
