/*
 * WhatsApp-Matter — read-only Matter Register web preview card.
 * The URL is an exact static allowlist item, not user input or a token.
 * This module neither authenticates identities nor writes to Vault.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Result};
use buffa::MessageField;
use whatsapp_rust::prelude::wa;

pub const PUBLIC_PREVIEW_URL: &str = "https://matterssmith.github.io/Base-Matter/";

/// Never accept arbitrary URLs, including same-host links with a query
/// string, fragment, userinfo, redirected host or a different path.
pub fn trusted_preview_url(value: &str) -> Result<&'static str> {
    if value != PUBLIC_PREVIEW_URL {
        bail!("untrusted_register_preview_url");
    }
    Ok(PUBLIC_PREVIEW_URL)
}

/// Build one read-only URL card in the same NativeFlow family as the
/// provider's existing menu. A successful send is NOT proof of phone rendering.
pub fn registration_preview_card(value: &str) -> Result<wa::Message> {
    let url = trusted_preview_url(value)?;
    let button = wa::message::interactive_message::native_flow_message::NativeFlowButton {
        name: Some("cta_url".to_owned()),
        button_params_json: Some(
            serde_json::json!({
                "display_text": "Abrir Matter Register",
                "url": url
            })
            .to_string(),
        ),
    };
    Ok(wa::Message {
        interactive_message: MessageField::some(wa::message::InteractiveMessage {
            body: MessageField::some(wa::message::interactive_message::Body {
                text: Some(
                    "✦ Matter Register\nVisita la interfaz interactiva de prueba.\
                     \nRegistro real y verificaciones de identidad aún no disponibles.\
                     \nNo envíes CURP, documentos ni fotografías por el chat."
                        .to_owned(),
                ),
            }),
            footer: MessageField::some(wa::message::interactive_message::Footer {
                text: Some("Matterssmith · vista previa pública".to_owned()),
                ..Default::default()
            }),
            interactive_message: Some(
                wa::message::interactive_message::InteractiveMessage::NativeFlowMessage(Box::new(
                    wa::message::interactive_message::NativeFlowMessage {
                        buttons: vec![button],
                        message_version: Some(1),
                        ..Default::default()
                    },
                )),
            ),
            ..Default::default()
        }),
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_exact_public_preview_is_accepted() {
        assert_eq!(
            trusted_preview_url(PUBLIC_PREVIEW_URL).unwrap(),
            PUBLIC_PREVIEW_URL
        );
        for rejected in [
            "http://matterssmith.github.io/Base-Matter/",
            "https://evil.example/Base-Matter/",
            "https://matterssmith.github.io/Base-Matter/?auth=secret",
            "https://matterssmith.github.io/Base-Matter/#token",
            "https://matterssmith.github.io@evil.example/Base-Matter/",
            "https://matterssmith.github.io/Base-Matter/../",
            "",
        ] {
            assert!(trusted_preview_url(rejected).is_err());
            assert!(registration_preview_card(rejected).is_err());
        }
    }

    #[test]
    fn card_contains_only_public_url() {
        let card = registration_preview_card(PUBLIC_PREVIEW_URL).unwrap();
        assert!(card.interactive_message.is_some());
    }
}
