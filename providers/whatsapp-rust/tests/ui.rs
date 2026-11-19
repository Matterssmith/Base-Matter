/* 
 * WhatsApp-Matter — native category selection and read-only button reply tests.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use buffa::MessageField;
use whatsapp_matter_rust_provider::ui::{
    native_menu_message, native_navigation_message, readonly_button_command,
    short_html_to_whatsapp,
};
use whatsapp_rust::prelude::wa;

fn native_click(id: &str, name: &str) -> wa::Message {
    wa::Message {
        interactive_response_message: MessageField::some(
            wa::message::InteractiveResponseMessage {
                interactive_response_message: Some(
                    wa::message::interactive_response_message::InteractiveResponseMessage::NativeFlowResponseMessage(
                        Box::new(
                            wa::message::interactive_response_message::NativeFlowResponseMessage {
                                name: Some(name.to_owned()),
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

fn legacy_click(id: &str) -> wa::Message {
    wa::Message {
        buttons_response_message: MessageField::some(
            wa::message::ButtonsResponseMessage {
                selected_button_id: Some(id.to_owned()),
                ..Default::default()
            },
        ),
        ..Default::default()
    }
}

#[test]
fn menu_has_category_selector_and_two_quick_replies() {
    let original = include_str!("../../../templates/matterssmith-menu.txt").trim_end();
    let message = native_menu_message(original).unwrap();
    let interactive = message.interactive_message.as_option().unwrap();
    assert_eq!(interactive.body.as_option().unwrap().text.as_deref(), Some(original));
    let Some(wa::message::interactive_message::InteractiveMessage::NativeFlowMessage(native)) =
        interactive.interactive_message.as_ref()
    else {
        panic!("Expected NativeFlow");
    };
    assert_eq!(native.buttons.len(), 3);
    assert_eq!(native.buttons[0].name.as_deref(), Some("single_select"));
    let categories: serde_json::Value =
        serde_json::from_str(native.buttons[0].button_params_json.as_deref().unwrap()).unwrap();
    let rows = categories["sections"][0]["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 9);
    assert!(rows.iter().any(|row| row["id"] == "matter.read.menu.antis"));
    assert_eq!(native.buttons[1].name.as_deref(), Some("quick_reply"));
    assert_eq!(native.buttons[2].name.as_deref(), Some("quick_reply"));
    let help: serde_json::Value =
        serde_json::from_str(native.buttons[1].button_params_json.as_deref().unwrap()).unwrap();
    assert_eq!(help["id"], "matter.read.help");
    assert!(message.conversation.is_none());
}

#[test]
fn submenus_offer_contextual_help_button() {
    let message = native_navigation_message("Protecciones", "=/>menu antis").unwrap();
    let interactive = message.interactive_message.as_option().unwrap();
    let Some(wa::message::interactive_message::InteractiveMessage::NativeFlowMessage(native)) =
        interactive.interactive_message.as_ref()
    else {
        panic!("Expected NativeFlow");
    };
    let help: serde_json::Value =
        serde_json::from_str(native.buttons[1].button_params_json.as_deref().unwrap()).unwrap();
    assert_eq!(help["id"], "matter.read.help.antis");
    assert!(native_navigation_message("Unsafe", "=/>set role owner").is_err());
}

#[test]
fn both_native_and_legacy_buttons_map_only_readonly_ids() {
    for (id, command) in [
        ("matter.read.status", "=/>status"),
        ("matter.read.menu.all", "=/>menu all"),
        ("matter.read.menu.antis", "=/>menu antis"),
        ("matter.read.help.antis", "=/>help antis"),
    ] {
        assert_eq!(readonly_button_command(&native_click(id, "quick_reply")), Some(command));
        assert_eq!(readonly_button_command(&native_click(id, "single_select")), Some(command));
        assert_eq!(readonly_button_command(&legacy_click(id)), Some(command));
    }
    for id in ["matter.bot.on", "=/>bot on", "matter.read.roles.grant", "matter.read.status --global"] {
        assert_eq!(readonly_button_command(&native_click(id, "quick_reply")), None);
        assert_eq!(readonly_button_command(&legacy_click(id)), None);
    }
    assert_eq!(readonly_button_command(&native_click("matter.read.menu", "admin_mutate")), None);
    assert_eq!(readonly_button_command(&wa::Message::default()), None);
}

#[test]
fn html_style_is_restricted_and_does_not_execute_markup() {
    assert_eq!(
        short_html_to_whatsapp("<b>Matterssmith</b><br><i>Control</i>").unwrap(),
        "*Matterssmith*\n_Control_"
    );
    for value in ["<script>alert(1)</script>", "<a href=x>x</a>", "<b onclick=x>x</b>", "<b>unclosed"] {
        assert!(short_html_to_whatsapp(value).is_err());
    }
    assert!(native_menu_message("").is_err());
    assert!(native_menu_message(&"x".repeat(3400)).is_err());
}
