/*
 * WhatsApp-Matter — native category selection and read-only button reply tests.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use buffa::MessageField;
use whatsapp_matter_rust_provider::ui::{
    legacy_row_probe_message, native_menu_message, native_navigation_message,
    native_ui_probe_message, readonly_button_command, short_html_to_whatsapp,
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
        buttons_response_message: MessageField::some(wa::message::ButtonsResponseMessage {
            selected_button_id: Some(id.to_owned()),
            ..Default::default()
        }),
        ..Default::default()
    }
}

#[test]
fn menu_has_exactly_one_categories_selector() {
    let original = include_str!("../../../templates/matterssmith-menu.txt").trim_end();
    let message = native_menu_message(original).unwrap();
    let interactive = message.interactive_message.as_option().unwrap();
    assert_eq!(
        interactive.body.as_option().unwrap().text.as_deref(),
        Some(original)
    );
    let Some(wa::message::interactive_message::InteractiveMessage::NativeFlowMessage(native)) =
        interactive.interactive_message.as_ref()
    else {
        panic!("Expected NativeFlow");
    };
    assert_eq!(native.buttons.len(), 1);
    assert_eq!(native.buttons[0].name.as_deref(), Some("single_select"));
    let categories: serde_json::Value =
        serde_json::from_str(native.buttons[0].button_params_json.as_deref().unwrap()).unwrap();
    let rows = categories["sections"][0]["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 10);
    assert_eq!(rows[0]["id"], "matter.read.menu.ia");
    assert_eq!(rows[1]["id"], "matter.read.menu.anti");
    assert_eq!(rows[2]["id"], "matter.read.menu.all");
    assert_eq!(rows[2]["title"], "✦ Todo");
    assert_eq!(rows[9]["id"], "matter.read.menu.page.2");
    assert!(rows
        .iter()
        .all(|row| row["id"].as_str().unwrap().starts_with("matter.read.menu.")));
    assert!(message.conversation.is_none());
}

#[test]
fn submenus_offer_only_one_categories_selector() {
    let message = native_navigation_message("Protecciones", "=/>menu antis").unwrap();
    let interactive = message.interactive_message.as_option().unwrap();
    let Some(wa::message::interactive_message::InteractiveMessage::NativeFlowMessage(native)) =
        interactive.interactive_message.as_ref()
    else {
        panic!("Expected NativeFlow");
    };
    assert_eq!(native.buttons.len(), 1);
    assert_eq!(native.buttons[0].name.as_deref(), Some("single_select"));
    assert!(native_navigation_message("Unsafe", "=/>set role owner").is_err());
    for (page, expected_len, first_id, last_id) in [
        (2, 10, "matter.read.menu.group", "matter.read.menu.page.3"),
        (3, 5, "matter.read.menu.search", "matter.read.menu.page.2"),
    ] {
        let request = format!("=/>menu page {page}");
        let message = native_navigation_message("Categorias", &request).unwrap();
        let interactive = message.interactive_message.as_option().unwrap();
        let Some(wa::message::interactive_message::InteractiveMessage::NativeFlowMessage(native)) =
            interactive.interactive_message.as_ref()
        else {
            panic!("Expected single category selector");
        };
        assert_eq!(native.buttons.len(), 1);
        let params: serde_json::Value =
            serde_json::from_str(native.buttons[0].button_params_json.as_deref().unwrap()).unwrap();
        let rows = params["sections"][0]["rows"].as_array().unwrap();
        assert_eq!(rows.len(), expected_len);
        assert_eq!(rows[0]["id"], first_id);
        assert_eq!(rows[rows.len() - 1]["id"], last_id);
    }
}

#[test]
fn both_native_and_legacy_buttons_map_only_readonly_ids() {
    for (id, command) in [
        ("matter.read.status", "=/>status"),
        ("matter.read.menu.all", "=/>menu all"),
        ("matter.read.menu.anti", "=/>menu anti"),
        ("matter.read.menu.antis", "=/>menu anti"),
        ("matter.read.menu.gacha", "=/>menu gacha"),
        ("matter.read.menu.page.2", "=/>menu page 2"),
        ("matter.read.help.antis", "=/>help antis"),
    ] {
        assert_eq!(
            readonly_button_command(&native_click(id, "quick_reply")),
            Some(command)
        );
        assert_eq!(
            readonly_button_command(&native_click(id, "single_select")),
            Some(command)
        );
        assert_eq!(readonly_button_command(&legacy_click(id)), Some(command));
    }
    for id in [
        "matter.bot.on",
        "=/>bot on",
        "matter.read.roles.grant",
        "matter.read.status --global",
    ] {
        assert_eq!(
            readonly_button_command(&native_click(id, "quick_reply")),
            None
        );
        assert_eq!(readonly_button_command(&legacy_click(id)), None);
    }
    assert_eq!(
        readonly_button_command(&native_click("matter.read.menu", "admin_mutate")),
        None
    );
    assert_eq!(readonly_button_command(&wa::Message::default()), None);
}

#[test]
fn html_style_is_restricted_and_does_not_execute_markup() {
    assert_eq!(
        short_html_to_whatsapp("<b>Matterssmith</b><br><i>Control</i>").unwrap(),
        "*Matterssmith*\n_Control_"
    );
    for value in [
        "<script>alert(1)</script>",
        "<a href=x>x</a>",
        "<b onclick=x>x</b>",
        "<b>unclosed",
    ] {
        assert!(short_html_to_whatsapp(value).is_err());
    }
    assert!(native_menu_message("").is_err());
    assert!(native_menu_message(&"x".repeat(3900)).is_err());
}

#[test]
fn owner_ui_probe_has_two_quick_replies_and_two_selectable_rows() {
    let message = native_ui_probe_message().unwrap();
    let interactive = message.interactive_message.as_option().unwrap();
    let Some(wa::message::interactive_message::InteractiveMessage::NativeFlowMessage(native)) =
        interactive.interactive_message.as_ref()
    else {
        panic!("Expected NativeFlow UI probe");
    };
    assert_eq!(native.buttons.len(), 3);
    assert_eq!(native.buttons[0].name.as_deref(), Some("single_select"));
    let list: serde_json::Value =
        serde_json::from_str(native.buttons[0].button_params_json.as_deref().unwrap()).unwrap();
    let rows = list["sections"][0]["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0]["id"], "matter.test.select.one");
    assert_eq!(rows[1]["id"], "matter.test.select.two");
    for (id, command) in [
        ("matter.test.button.a", "=/>test ack a"),
        ("matter.test.button.b", "=/>test ack b"),
        ("matter.test.select.one", "=/>test ack one"),
        ("matter.test.select.two", "=/>test ack two"),
    ] {
        assert_eq!(
            readonly_button_command(&native_click(id, "quick_reply")),
            Some(command)
        );
        assert_eq!(
            readonly_button_command(&native_click(id, "single_select")),
            Some(command)
        );
        assert_eq!(readonly_button_command(&legacy_click(id)), Some(command));
    }
    assert_eq!(
        readonly_button_command(&native_click("matter.test.run.code", "quick_reply")),
        None
    );
}

#[test]
fn legacy_row_uses_three_response_buttons_without_network_or_media() {
    let message = legacy_row_probe_message();
    let buttons = message.buttons_message.as_option().unwrap();
    assert_eq!(buttons.buttons.len(), 3);
    assert_eq!(
        buttons.content_text.as_deref(),
        Some("🧪 Tarjeta de prueba (ButtonsRow)\nElige Audio, MP4 o Enlace. No hay descargas.")
    );
    for (button, id, label) in [
        (&buttons.buttons[0], "matter.test.row.audio", "Audio"),
        (&buttons.buttons[1], "matter.test.row.mp4", "MP4"),
        (&buttons.buttons[2], "matter.test.row.link", "Enlace"),
    ] {
        assert_eq!(button.button_id.as_deref(), Some(id));
        assert_eq!(
            button
                .button_text
                .as_option()
                .unwrap()
                .display_text
                .as_deref(),
            Some(label)
        );
        assert_eq!(
            button.r#type,
            Some(wa::message::buttons_message::button::Type::RESPONSE)
        );
    }
    assert!(message.interactive_message.as_option().is_none());
    for (id, command) in [
        ("matter.test.row.audio", "=/>test ack audio"),
        ("matter.test.row.mp4", "=/>test ack mp4"),
        ("matter.test.row.link", "=/>test ack link"),
    ] {
        assert_eq!(readonly_button_command(&legacy_click(id)), Some(command));
    }
}
