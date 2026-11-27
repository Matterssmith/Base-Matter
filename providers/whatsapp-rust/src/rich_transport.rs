/*
 * WhatsApp-Matter — opt-in RichResponse transport experiment.
 *
 * This path serializes original Matter HTML into an experimental WhatsApp
 * RichResponse carried by a BotForwardedMessage container, as used by current
 * HTML-capable clients. No Meta AI identity, fake signature, or bot JID is forged.
 * Sending requires provider opt-in and authenticated command admission.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use crate::rich_preview::experimental_rich_schema_with_style;
use anyhow::{anyhow, bail, Result};
use buffa::MessageField;
use whatsapp_rust::prelude::wa;

const MAX_WIRE_BYTES: usize = 16_384;

/// Build a RichResponse HTML menu for the authorized provider to send.
/// A client can ignore or discard these unsupported UI primitives even when
/// the API reports a successful send. NativeFlow/text remain the fallback.
pub fn experimental_self_chat_rich_message(menu: &str) -> Result<wa::Message> {
    styled_rich_message(menu, false)
}

pub fn styled_rich_message(menu: &str, plain: bool) -> Result<wa::Message> {
    let mut payload = experimental_rich_schema_with_style(menu, plain)?;
    let mut nonce = [0_u8; 16];
    getrandom::fill(&mut nonce).map_err(|_| anyhow!("Rich response ID unavailable"))?;
    payload["response_id"] = format!("matterssmith-{}", hex::encode(nonce)).into();
    let bytes = serde_json::to_vec(&payload)?;
    if bytes.is_empty() || bytes.len() > MAX_WIRE_BYTES {
        bail!("Experimental HTML response exceeds transport limit");
    }
    let rich = wa::AIRichResponseMessage {
        message_type: Some(wa::AIRichResponseMessageType::AiRichResponseTypeStandard),
        submessages: vec![wa::AIRichResponseSubMessage {
            message_type: Some(wa::AIRichResponseSubMessageType::AiRichResponseText),
            message_text: Some("Matterssmith · Base-Matter".to_owned()),
            ..Default::default()
        }],
        unified_response: MessageField::some(wa::AIRichResponseUnifiedResponse {
            data: Some(bytes),
        }),
        ..Default::default()
    };
    // Aura and current Rich UI clients carry the response in a nested
    // BotForwardedMessage (protobuf field 104), rather than at root field 97.
    // This is structural nesting only: do not claim a Meta AI identity,
    // forge verification proofs or attach a fake @bot origin.
    let nested = wa::Message {
        rich_response_message: MessageField::some(rich),
        ..Default::default()
    };
    Ok(wa::Message {
        bot_forwarded_message: MessageField::some(wa::message::FutureProofMessage {
            message: MessageField::some(nested),
        }),
        ..Default::default()
    })
}
