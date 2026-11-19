/*
 * WhatsApp-Matter — optional native quick replies and restricted HTML styling.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Result};
use buffa::MessageField;
use wa::message::interactive_response_message::InteractiveResponseMessage as NativeResponse;
use whatsapp_rust::prelude::wa;

const MAX_MENU_BYTES: usize = 3200;

const NAV_CATEGORIES: [(&str, &str, &str); 9] = [
    ("status", "Estado", "matter.read.menu.status"),
    ("bot", "Bot", "matter.read.menu.bot"),
    ("set", "Configuración", "matter.read.menu.set"),
    ("antis", "Protecciones", "matter.read.menu.antis"),
    ("roles", "Roles", "matter.read.menu.roles"),
    ("system", "Sistema", "matter.read.menu.system"),
    ("sandbox", "Sandbox", "matter.read.menu.sandbox"),
    ("ai", "IA", "matter.read.menu.ai"),
    ("navigation", "Navegación", "matter.read.menu.navigation"),
];

fn button_command(id: &str) -> Option<&'static str> {
    match id {
        "matter.read.status" => Some("=/>status"),
        "matter.read.menu" => Some("=/>menu"),
        "matter.read.antis" => Some("=/>menu antis"),
        "matter.read.help" => Some("=/>help"),
        "matter.read.menu.all" => Some("=/>menu all"),
        "matter.read.menu.status" => Some("=/>menu status"),
        "matter.read.menu.bot" => Some("=/>menu bot"),
        "matter.read.menu.set" => Some("=/>menu set"),
        "matter.read.menu.antis" => Some("=/>menu antis"),
        "matter.read.menu.roles" => Some("=/>menu roles"),
        "matter.read.menu.system" => Some("=/>menu system"),
        "matter.read.menu.sandbox" => Some("=/>menu sandbox"),
        "matter.read.menu.ai" => Some("=/>menu ai"),
        "matter.read.menu.navigation" => Some("=/>menu navigation"),
        "matter.read.help.status" => Some("=/>help status"),
        "matter.read.help.bot" => Some("=/>help bot"),
        "matter.read.help.set" => Some("=/>help set"),
        "matter.read.help.antis" => Some("=/>help antis"),
        "matter.read.help.roles" => Some("=/>help roles"),
        "matter.read.help.system" => Some("=/>help system"),
        "matter.read.help.sandbox" => Some("=/>help sandbox"),
        "matter.read.help.ai" => Some("=/>help ai"),
        "matter.read.help.navigation" => Some("=/>help navigation"),
        _ => None,
    }
}

fn context_help_id(command: &str) -> &'static str {
    let category = command
        .strip_prefix("=/>menu ")
        .or_else(|| command.strip_prefix("=/>help "))
        .unwrap_or_default();
    match category {
        "status" => "matter.read.help.status",
        "bot" => "matter.read.help.bot",
        "set" => "matter.read.help.set",
        "antis" => "matter.read.help.antis",
        "roles" => "matter.read.help.roles",
        "system" => "matter.read.help.system",
        "sandbox" => "matter.read.help.sandbox",
        "ai" => "matter.read.help.ai",
        "navigation" => "matter.read.help.navigation",
        _ => "matter.read.help",
    }
}

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

/// Construct a NativeFlow menu with a selectable category list plus
/// two read-only quick replies. The phone must confirm visual support.
pub fn native_menu_message(menu: &str) -> Result<wa::Message> {
    native_navigation_message(menu, "=/>menu")
}

pub fn native_navigation_message(menu: &str, request: &str) -> Result<wa::Message> {
    if menu.is_empty()
        || menu.len() > MAX_MENU_BYTES
        || menu.chars().any(|c| c.is_control() && c != '\n')
    {
        bail!("Invalid authenticated native menu");
    }
    if !(request == "=/>menu"
        || request == "=/>menú"
        || request.starts_with("=/>menu ")
        || request.starts_with("=/>menú ")
        || request == "=/>help"
        || request == "=/>h"
        || request == "=/>ayuda"
        || request.starts_with("=/>help ")
        || request.starts_with("=/>h ")
        || request.starts_with("=/>ayuda "))
    {
        bail!("Native navigation is read-only");
    }
    let rows: Vec<_> = NAV_CATEGORIES
        .iter()
        .map(|(_, label, id)| {
            serde_json::json!({
                "title": label,
                "description": "Abrir categoría",
                "id": id
            })
        })
        .collect();
    let selector = wa::message::interactive_message::native_flow_message::NativeFlowButton {
        name: Some("single_select".to_owned()),
        button_params_json: Some(
            serde_json::json!({
                "title": "Categorías",
                "sections": [{
                    "title": "Explorar menú",
                    "rows": rows
                }]
            })
            .to_string(),
        ),
    };
    let quick_reply = |id: &str, label: &str| {
        wa::message::interactive_message::native_flow_message::NativeFlowButton {
            name: Some("quick_reply".to_owned()),
            button_params_json: Some(
                serde_json::json!({ "display_text": label, "id": id }).to_string(),
            ),
        }
    };
    let buttons = vec![
        selector,
        quick_reply(context_help_id(request), "Ayuda"),
        quick_reply("matter.read.menu.all", "Ver todo"),
    ];
    Ok(wa::Message {
        interactive_message: MessageField::some(wa::message::InteractiveMessage {
            body: MessageField::some(wa::message::interactive_message::Body {
                text: Some(menu.to_owned()),
            }),
            footer: MessageField::some(wa::message::interactive_message::Footer {
                text: Some(short_html_to_whatsapp(
                    "<b>Control</b> · <i>Elige una categoría</i>",
                )?),
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

/// A response is only a request to a read-only, fixed command.
/// The normal WhatsApp identity, HMAC and replay checks run after mapping.
/// Legacy button replies are recognized for Go-style ButtonsRow compatibility.
pub fn readonly_button_command(message: &wa::Message) -> Option<&'static str> {
    let id = if let Some(response) = message.interactive_response_message.as_option() {
        let NativeResponse::NativeFlowResponseMessage(native) =
            response.interactive_response_message.as_ref()?;
        if !matches!(
            native.name.as_deref(),
            Some("quick_reply" | "single_select")
        ) {
            return None;
        }
        let json = native.params_json.as_deref()?;
        if json.len() > 512 {
            return None;
        }
        let value: serde_json::Value = serde_json::from_str(json).ok()?;
        value.get("id")?.as_str().map(str::to_owned)
    } else {
        message
            .buttons_response_message
            .as_option()?
            .selected_button_id
            .clone()
    }?;
    button_command(&id)
}
