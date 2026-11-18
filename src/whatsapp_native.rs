/*
 * Base-Matter — trusted, in-process whatsapp-rust ingress.
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use crate::{
    control::{ControlDispatcher, DispatchResult},
    ingress::InboundReplayStore,
};
use anyhow::{bail, Result};
use std::sync::Arc;
use time::OffsetDateTime;

pub struct WhatsAppNativeMessage<'a> {
    pub chat_jid: &'a str,
    pub sender_jid: &'a str,
    pub message_id: &'a str,
    pub text: &'a str,
    pub from_me: bool,
    pub is_group: bool,
    pub timestamp: i64,
}
pub struct WhatsAppNativeConfig {
    pub socket_id: String,
    pub actor_id: String,
    pub owner_jid: String,
}
#[derive(Debug, PartialEq, Eq)]
pub enum WhatsAppNativeOutcome {
    Ignored,
    Duplicate,
    Dispatched(DispatchResult),
}
/// Only trusted whatsapp-rust host code can attest is_from_me and sender.
/// This type does NOT authenticate arbitrary network requests.
pub struct WhatsAppNativeIngress {
    config: WhatsAppNativeConfig,
    replay: Arc<InboundReplayStore>,
    dispatcher: Arc<ControlDispatcher>,
}
impl WhatsAppNativeIngress {
    pub fn new(
        config: WhatsAppNativeConfig,
        replay: Arc<InboundReplayStore>,
        dispatcher: Arc<ControlDispatcher>,
    ) -> Result<Self> {
        for value in [&config.socket_id, &config.actor_id] {
            if value.is_empty()
                || value.len() > 128
                || !value
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"._:-".contains(&b))
            {
                bail!("Invalid configured WhatsApp identity");
            }
        }
        let user = config.owner_jid.split('@').next().unwrap_or("");
        if config.owner_jid.len() > 128
            || !(config.owner_jid.ends_with("@s.whatsapp.net")
                || config.owner_jid.ends_with("@lid"))
            || user.is_empty()
            || !user
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"._:-".contains(&b))
        {
            bail!("Invalid configured owner JID");
        }
        Ok(Self {
            config,
            replay,
            dispatcher,
        })
    }
    pub fn ingest(&self, event: &WhatsAppNativeMessage<'_>) -> Result<WhatsAppNativeOutcome> {
        if !event.from_me
            || event.is_group
            || event.chat_jid != self.config.owner_jid
            || event.sender_jid != self.config.owner_jid
        {
            return Ok(WhatsAppNativeOutcome::Ignored);
        }
        if !event.text.starts_with("=/>") {
            return Ok(WhatsAppNativeOutcome::Ignored);
        }
        // Check limits before classifying deprecated commands. Even an
        // ignored alias must not bypass the malformed-message boundary.
        if event.text.len() > 4096
            || event.message_id.is_empty()
            || event.message_id.len() > 256
            || event.message_id.chars().any(char::is_control)
        {
            bail!("Invalid WhatsApp native control message");
        }
        // Retired live toggle aliases are never forwarded, even from the
        // legitimate socket owner. Use =/>bot on/off and explicit scopes.
        if matches!(
            event.text.split_whitespace().next(),
            Some("=/>on" | "=/>off" | "=/>encender" | "=/>apagar")
        ) {
            return Ok(WhatsAppNativeOutcome::Ignored);
        }
        let now = OffsetDateTime::now_utc().unix_timestamp();
        if now.abs_diff(event.timestamp) > 120 {
            bail!("Expired WhatsApp native control message");
        }
        if !self
            .replay
            .claim("whatsapp", &self.config.socket_id, event.message_id)?
        {
            return Ok(WhatsAppNativeOutcome::Duplicate);
        }
        let result = self.dispatcher.dispatch(
            &self.config.socket_id,
            &self.config.actor_id,
            &self.config.owner_jid,
            None,
            event.text,
        )?;
        Ok(WhatsAppNativeOutcome::Dispatched(result))
    }
}
