/*
 * WhatsApp-Matter — native quick replies and bounded HTML-style conversions.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use buffa::MessageField;
use whatsapp_matter_rust_provider::ui::{
    native_menu_message, readonly_button_command, short_html_to_whatsapp,
};
use whatsapp_rust::prelude::wa;

fn incoming_click(id: &str) -> wa::Message {
    wa::Message {
        interactive_response_message: MessageField::some(
            wa::message::InteractiveResponseMessage {
                interactive_response_message: Some(
                    wa::message::interactive_response_message::InteractiveResponseMessage::NativeFlowResponseMessage(
                        Box::new(
                            wa::message::interactive_response_message::NativeFlowResponseMessage {
                                name: Some("quick_reply".to_owned()),
                                params_json: Some(serde_json::json!({"id": id}).to_string()),
                                ..Default::default()
                            },
                        ),
                    ),
                ),
                ..Default::default()
            },
        ),
        ..Default::default()
    }
}

#[test]
fn original_menu_is_carried_in_the_same_native_message_with_three_real_buttons() {
    let menu = include_str!("../../../templates/matterssmith-menu.txt").trim_end();
    let envelope = native_menu_message(menu).unwrap();
    let interactive = envelope.interactive_message.as_option().unwrap();
    assert_eq!(
        interactive.body.as_option().unwrap().text.as_deref(),
        Some(menu)
    );
    let Some(wa::message::interactive_message::InteractiveMessage::NativeFlowMessage(native)) =
        interactive.interactive_message.as_ref()
    else {
        panic!("Expected WhatsApp NativeFlow, not buttons drawn as text");
    };
    assert_eq!(native.buttons.len(), 3);
    for (index, id) in [
        "matter.read.status",
        "matter.read.menu",
        "matter.read.antis",
    ]
    .iter()
    .enumerate()
    {
        let button = &native.buttons[index];
        assert_eq!(button.name.as_deref(), Some("quick_reply"));
        let json: serde_json::Value =
            serde_json::from_str(button.button_params_json.as_deref().unwrap()).unwrap();
        assert_eq!(json["id"], *id);
    }
    assert!(native.buttons.iter().all(|button| {
        let json: serde_json::Value =
            serde_json::from_str(button.button_params_json.as_deref().unwrap()).unwrap();
        !json["id"].as_str().unwrap_or_default().contains("bot on")
    }));
    assert!(envelope.conversation.is_none());
}

#[test]
fn only_readonly_clicks_can_be_mapped_to_control_commands() {
    assert_eq!(
        readonly_button_command(&incoming_click("matter.read.status")),
        Some("=/>status")
    );
    assert_eq!(
        readonly_button_command(&incoming_click("matter.read.menu")),
        Some("=/>menu")
    );
    assert_eq!(
        readonly_button_command(&incoming_click("matter.read.antis")),
        Some("=/>menu antis")
    );
    for unsafe_id in [
        "matter.bot.on",
        "matter.read.status --global",
        "=/>bot on",
        "matter.read.audit",
    ] {
        assert_eq!(readonly_button_command(&incoming_click(unsafe_id)), None);
    }
    assert_eq!(readonly_button_command(&wa::Message::default()), None);
}

#[test]
fn html_is_reduced_to_safe_whatsapp_formatting_never_executable_markup() {
    assert_eq!(
        short_html_to_whatsapp("<b>Matterssmith</b><br><i>Control</i>").unwrap(),
        "*Matterssmith*\n_Control_"
    );
    assert_eq!(
        short_html_to_whatsapp("<strong>A &amp; B</strong>").unwrap(),
        "*A & B*"
    );
    for invalid in [
        "<script>alert(1)</script>",
        "<img src=x>",
        "<a href='https://example.com'>link</a>",
        "<b onclick=x>text</b>",
        "<b>unterminated",
        "<i>x</b>",
        "&unknown;",
    ] {
        assert!(
            short_html_to_whatsapp(invalid).is_err(),
            "unexpected HTML allowance: {invalid}"
        );
    }
    assert!(short_html_to_whatsapp(&"x".repeat(600)).is_err());
    assert!(native_menu_message("").is_err());
    assert!(native_menu_message(&"x".repeat(3400)).is_err());
}
