/*
 * Base-Matter — authenticated WhatsApp owner and public read-only bridge (v1).
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use crate::control::{ControlDispatcher, DispatchResult};
use crate::ingress::InboundReplayStore;
use anyhow::{anyhow, bail, Result};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::sync::Arc;
use time::OffsetDateTime;
use zeroize::Zeroizing;

const VERSION: u8 = 1;
const MAX_FRAME_BYTES: usize = 16 * 1024;
const MAX_TEXT_BYTES: usize = 4096;
const MAX_ID_BYTES: usize = 256;
const MAX_SKEW_SECONDS: u64 = 120;
const DOMAIN: &[u8] = b"Base-Matter:WhatsApp-Control:v1\0";

/// Untrusted bytes received from a provider process. The signature covers
/// every field except signature. The authenticated provider must obtain all
/// routing identifiers from its established WhatsApp connection, not from text.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WhatsAppControlFrame {
    pub version: u8,
    pub socket_id: String,
    pub actor_id: String,
    pub owner_jid: String,
    pub chat_jid: String,
    pub message_id: String,
    pub text: String,
    pub from_me: bool,
    pub timestamp: i64,
    pub signature: String,
}

#[derive(Clone, Debug)]
pub struct WhatsAppBridgeConfig {
    pub socket_id: String,
    pub owner_actor_id: String,
    pub owner_jid: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WhatsAppBridgeOutcome {
    Ignored,
    Duplicate,
    Dispatched(DispatchResult),
}

/// Distinguishes trusted owner-origin administrative controls from signed,
/// read-only incoming status/menu/bot requests in verified groups or external
/// private chats. Never routes public messages to Bot-Matter or role mutation.
pub struct WhatsAppControlBridge {
    config: WhatsAppBridgeConfig,
    key: Zeroizing<[u8; 32]>,
    replay: Arc<InboundReplayStore>,
    control: Arc<ControlDispatcher>,
}

impl WhatsAppControlBridge {
    pub fn new(
        config: WhatsAppBridgeConfig,
        key: [u8; 32],
        replay: Arc<InboundReplayStore>,
        control: Arc<ControlDispatcher>,
    ) -> Result<Self> {
        validate_identifier(&config.socket_id)?;
        validate_identifier(&config.owner_actor_id)?;
        validate_owner_jid(&config.owner_jid)?;
        if key.iter().all(|byte| *byte == 0) {
            bail!("Invalid WhatsApp bridge binding key");
        }
        Ok(Self {
            config,
            key: Zeroizing::new(key),
            replay,
            control,
        })
    }

    pub fn ingest_json(&self, incoming: &str) -> Result<WhatsAppBridgeOutcome> {
        if incoming.len() > MAX_FRAME_BYTES {
            bail!("WhatsApp control frame exceeds limit");
        }
        let frame: WhatsAppControlFrame = serde_json::from_str(incoming)
            .map_err(|_| anyhow!("Invalid WhatsApp control frame"))?;
        self.ingest(&frame)
    }

    pub fn ingest(&self, frame: &WhatsAppControlFrame) -> Result<WhatsAppBridgeOutcome> {
        // Two disjoint identities, both backed by the provider's HMAC:
        // (1) verified socket-owner controls; (2) incoming read-only
        // requests bound to a real WhatsApp group or sender's private chat.
        let is_owner = frame.from_me
            && frame.actor_id == self.config.owner_actor_id
            && (frame.chat_jid == self.config.owner_jid || valid_group_jid(&frame.chat_jid));
        let is_public_reader = !frame.from_me
            && valid_user_jid(&frame.actor_id)
            && (valid_group_jid(&frame.chat_jid) || frame.chat_jid == frame.actor_id);
        if frame.version != VERSION
            || frame.socket_id != self.config.socket_id
            || frame.owner_jid != self.config.owner_jid
            || !(is_owner || is_public_reader)
        {
            bail!("WhatsApp control identity denied");
        }
        if frame.message_id.is_empty()
            || frame.message_id.len() > MAX_ID_BYTES
            || frame.message_id.chars().any(char::is_control)
            || frame.text.is_empty()
            || frame.text.len() > MAX_TEXT_BYTES
        {
            bail!("Invalid WhatsApp control payload");
        }
        let now = OffsetDateTime::now_utc().unix_timestamp();
        if now.abs_diff(frame.timestamp) > MAX_SKEW_SECONDS {
            bail!("Expired WhatsApp control frame");
        }
        let supplied =
            hex::decode(&frame.signature).map_err(|_| anyhow!("Invalid WhatsApp control proof"))?;
        if supplied.len() != 32 {
            bail!("Invalid WhatsApp control proof");
        }
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.key[..])
            .map_err(|_| anyhow!("WhatsApp control verifier unavailable"))?;
        mac.update(DOMAIN);
        let fields = [
            VERSION.to_string(),
            frame.socket_id.clone(),
            frame.actor_id.clone(),
            frame.owner_jid.clone(),
            frame.chat_jid.clone(),
            frame.message_id.clone(),
            frame.text.clone(),
        ];
        for field in &fields {
            let bytes = field.as_bytes();
            mac.update(&(u32::try_from(bytes.len())?).to_be_bytes());
            mac.update(bytes);
        }
        mac.update(&[u8::from(frame.from_me)]);
        mac.update(&frame.timestamp.to_be_bytes());
        mac.verify_slice(&supplied)
            .map_err(|_| anyhow!("Invalid WhatsApp control proof"))?;

        // No bot commands, including historic #ping, are admitted here.
        if !frame.text.starts_with("=/>") {
            return Ok(WhatsAppBridgeOutcome::Ignored);
        }
        let group_id = if valid_group_jid(&frame.chat_jid) {
            Some(frame.chat_jid.as_str())
        } else {
            None
        };
        if is_public_reader {
            // Public commands have a dedicated, read-only policy path.
            // Never pass their actor into the owner dispatcher.
            let outcome = self.control.dispatch_public_readonly(
                &self.config.socket_id,
                &frame.actor_id,
                &frame.chat_jid,
                group_id,
                &frame.text,
            )?;
            if !self
                .replay
                .claim("whatsapp", &self.config.socket_id, &frame.message_id)?
            {
                return Ok(WhatsAppBridgeOutcome::Duplicate);
            }
            return Ok(WhatsAppBridgeOutcome::Dispatched(outcome));
        }
        let verb = frame.text.split_whitespace().next().unwrap_or_default();
        if group_id.is_some() {
            if !matches!(
                verb,
                "=/>group"
                    | "=/>grupo"
                    | "=/>menu"
                    | "=/>menú"
                    | "=/>help"
                    | "=/>h"
                    | "=/>ayuda"
                    | "=/>status"
                    | "=/>estado"
                    | "=/>bot"
                    | "=/>start"
                    | "=/>add"
                    | "=/>add++"
                    | "=/>anti"
                    | "=/>antis"
                    | "=/>set"
                    | "=/>credits"
                    | "=/>creditos"
                    | "=/>créditos"
                    | "=/>test"
            ) {
                bail!("Only group-scoped controls are available in groups");
            }
        } else if !matches!(
            verb,
            "=/>status"
                | "=/>estado"
                | "=/>bot"
                | "=/>menu"
                | "=/>menú"
                | "=/>help"
                | "=/>h"
                | "=/>ayuda"
                | "=/>set"
                | "=/>unset"
                | "=/>role"
                | "=/>roles"
                | "=/>audit"
                | "=/>matter"
                | "=/>test"
                | "=/>anti"
                | "=/>antis"
                | "=/>credits"
                | "=/>creditos"
                | "=/>créditos"
        ) {
            bail!("Private chat is reserved for socket configuration");
        }
        if !self
            .replay
            .claim("whatsapp", &self.config.socket_id, &frame.message_id)?
        {
            return Ok(WhatsAppBridgeOutcome::Duplicate);
        }
        let outcome = self.control.dispatch(
            &self.config.socket_id,
            &self.config.owner_actor_id,
            &frame.chat_jid,
            group_id,
            &frame.text,
        )?;
        Ok(WhatsAppBridgeOutcome::Dispatched(outcome))
    }
}

fn valid_user_jid(value: &str) -> bool {
    let account = value
        .strip_suffix("@s.whatsapp.net")
        .or_else(|| value.strip_suffix("@lid"));
    account.is_some_and(|digits| {
        (8..=20).contains(&digits.len()) && digits.bytes().all(|byte| byte.is_ascii_digit())
    })
}

fn valid_group_jid(value: &str) -> bool {
    value.ends_with("@g.us")
        && !value.starts_with('@')
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"@._:-".contains(&byte))
}

fn validate_identifier(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > 128
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._:-".contains(&byte))
    {
        bail!("Invalid bridge identifier");
    }
    Ok(())
}

fn validate_owner_jid(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > 128
        || !(value.ends_with("@s.whatsapp.net") || value.ends_with("@lid"))
        || !value
            .split('@')
            .next()
            .unwrap_or_default()
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._:-".contains(&byte))
    {
        bail!("Invalid WhatsApp owner JID");
    }
    Ok(())
}
