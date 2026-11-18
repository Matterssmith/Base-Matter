/*
 * WhatsApp-Matter — optional native quick replies and restricted HTML styling.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Result};
use buffa::MessageField;
use whatsapp_rust::prelude::wa;
use wa::message::interactive_response_message::InteractiveResponseMessage as NativeResponse;

const MAX_MENU_BYTES: usize = 3200;
const BUTTONS: [(&str, &str, &str); 3] = [
    ("matter.read.status", "Estado", "=/>status"),
    ("matter.read.menu", "Menú", "=/>menu"),
    ("matter.read.credits", "Créditos", "=/>credits"),
];

/// Convert a *small* allowlisted HTML-style fragment to WhatsApp text
/// formatting. WhatsApp does not interpret arbitrary HTML or JavaScript.
/// Reject attributes, scripts, unknown tags and unclosed markup.
pub fn short_html_to_whatsapp(input: &str) -> Result<String> {
    if input.len() > 512 || input.chars().any(|c| c.is_control() && c != '\n') {
        bail!("HTML style fragment exceeds safe bounds");
    }
    let mut remaining = input;
    let mut stack: Vec<&str> = Vec::new();
    let mut output = String::with_capacity(input.len());
    while !remaining.is_empty() {
        let mut found = false;
        for (tag, marker) in [
            ("b", "*"),
            ("strong", "*"),
            ("i", "_"),
            ("em", "_"),
            ("s", "~"),
            ("code", "`"),
        ] {
            let opening = format!("<{tag}>");
            let closing = format!("</{tag}>");
            if remaining.starts_with(&opening) {
                stack.push(tag);
                output.push_str(marker);
                remaining = &remaining[opening.len()..];
                found = true;
                break;
            }
            if remaining.starts_with(&closing) {
                if stack.pop() != Some(tag) {
                    bail!("Mismatched HTML style tag");
                }
                output.push_str(marker);
                remaining = &remaining[closing.len()..];
                found = true;
                break;
            }
        }
        if found {
            continue;
        }
        if remaining.starts_with("<br>") {
            output.push('\n');
            remaining = &remaining[4..];
        } else if remaining.starts_with("&amp;") {
            output.push('&');
            remaining = &remaining[5..];
        } else if remaining.starts_with("&lt;") {
            output.push('<');
            remaining = &remaining[4..];
        } else if remaining.starts_with("&gt;") {
            output.push('>');
            remaining = &remaining[4..];
        } else if remaining.starts_with("&quot;") {
            output.push('"');
            remaining = &remaining[6..];
        } else {
            let ch = remaining.chars().next().expect("nonempty input");
            if ch == '<' || ch == '>' || ch == '&' {
                bail!("Unsupported HTML tag or entity");
            }
            output.push(ch);
            remaining = &remaining[ch.len_utf8()..];
        }
    }
    if !stack.is_empty() || output.len() > 800 {
        bail!("Unclosed or oversized HTML style");
    }
    Ok(output)
}

/// Construct a real WhatsApp NativeFlow message from a signed, already
/// formatted controller menu. The original banner and credit remain intact.
/// Delivery and display still require a real client test.
pub fn native_menu_message(menu: &str) -> Result<wa::Message> {
    if menu.is_empty()
        || menu.len() > MAX_MENU_BYTES
        || menu.chars().any(|c| c.is_control() && c != '\n')
    {
        bail!("Invalid authenticated native menu");
    }
    let buttons = BUTTONS
        .iter()
        .map(|(id, label, _)| {
            wa::message::interactive_message::native_flow_message::NativeFlowButton {
                name: Some("quick_reply".to_owned()),
                button_params_json: Some(
                    serde_json::json!({ "display_text": label, "id": id }).to_string(),
                ),
            }
        })
        .collect();
    Ok(wa::Message {
        interactive_message: MessageField::some(wa::message::InteractiveMessage {
            body: MessageField::some(wa::message::interactive_message::Body {
                text: Some(menu.to_owned()),
            }),
            footer: MessageField::some(wa::message::interactive_message::Footer {
                text: Some(short_html_to_whatsapp("<i>Botones de consulta</i>")?),
                ..Default::default()
            }),
            interactive_message: Some(
                wa::message::interactive_message::InteractiveMessage::NativeFlowMessage(Box::new(
                    wa::message::interactive_message::NativeFlowMessage {
                        buttons,
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

/// A clicked button can request only public read-only commands. Never
/// translate a button payload into a socket mutation, role or debug command.
/// The event still passes through owner/public HMAC and replay verification.
pub fn readonly_button_command(message: &wa::Message) -> Option<&'static str> {
    let response = message.interactive_response_message.as_option()?;
    let NativeResponse::NativeFlowResponseMessage(native) =
        response.interactive_response_message.as_ref()?;
    let params = native.params_json.as_deref()?;
    if params.len() > 512 {
        return None;
    }
    let json: serde_json::Value = serde_json::from_str(params).ok()?;
    let id = json.get("id")?.as_str()?;
    BUTTONS
        .iter()
        .find(|(allowed, _, _)| *allowed == id)
        .map(|(_, _, command)| *command)
}
