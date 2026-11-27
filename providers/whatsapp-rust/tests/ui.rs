/*
 * WhatsApp-Matter — native category selection and read-only button reply tests.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use buffa::MessageField;
use whatsapp_matter_rust_provider::ui::{
    legacy_row_probe_message, menu_lab_card, native_menu_lab_card, native_menu_message,
    native_navigation_message, native_ui_probe_message, readonly_button_command,
    short_html_to_whatsapp, MenuCardVariant,
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

fn list_click(id: &str) -> wa::Message {
    wa::Message {
        list_response_message: MessageField::some(wa::message::ListResponseMessage {
            single_select_reply: MessageField::some(
                wa::message::list_response_message::SingleSelectReply {
                    selected_row_id: Some(id.to_owned()),
                },
            ),
            ..Default::default()
        }),
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
fn root_menu_has_only_the_final_menu_format_selector() {
    let original = whatsapp_matter_rust_provider::classic_template::layout("legacy-menu").trim_end();
    let message = native_menu_message(original).unwrap();
    let interactive = message.interactive_message.as_option().unwrap();
    assert_eq!(
        interactive.body.as_option().unwrap().text.as_deref(),
        Some(original)
    );
    assert!(interactive.footer.as_option().is_none());
    assert!(message.conversation.is_none());
    let Some(wa::message::interactive_message::InteractiveMessage::NativeFlowMessage(native)) =
        interactive.interactive_message.as_ref()
    else {
        panic!("Expected NativeFlow menu");
    };
    assert_eq!(native.buttons.len(), 1);
    assert_eq!(native.buttons[0].name.as_deref(), Some("single_select"));
    let data: serde_json::Value =
        serde_json::from_str(native.buttons[0].button_params_json.as_deref().unwrap()).unwrap();
    assert_eq!(data["title"], "Otros menús");
    let sections = data["sections"].as_array().unwrap();
    assert_eq!(sections.len(), 1);
    assert_eq!(sections[0]["title"], "Otros menús");
    let modes = sections[0]["rows"].as_array().unwrap();
    assert_eq!(modes.len(), 3);
    assert_eq!(modes[0]["id"], "matter.read.menu.classic");
    assert_eq!(modes[1]["id"], "matter.read.menu.web");
    assert_eq!(modes[2]["id"], "matter.read.menu.buttons");
}

#[test]
fn each_mode_links_only_to_the_other_two_and_roundtrips() {
    for mode in ["classic", "web", "buttons"] {
        let request = format!("=/>menu {mode}");
        let message = native_navigation_message("Contenido", &request).unwrap();
        let interactive = message.interactive_message.as_option().unwrap();
        let Some(wa::message::interactive_message::InteractiveMessage::NativeFlowMessage(native)) =
            interactive.interactive_message.as_ref()
        else {
            panic!("Expected NativeFlow");
        };
        let params: serde_json::Value =
            serde_json::from_str(native.buttons[0].button_params_json.as_deref().unwrap()).unwrap();
        let sections = params["sections"].as_array().unwrap();
        assert_eq!(sections.len(), 1);
        let rows = sections[0]["rows"].as_array().unwrap();
        assert_eq!(rows.len(), 2);
        for row in rows {
            let id = row["id"].as_str().unwrap();
            let command = readonly_button_command(&native_click(id, "single_select")).unwrap();
            assert_ne!(command, request);
            assert_eq!(readonly_button_command(&list_click(id)), Some(command));
            let next = native_navigation_message("Contenido siguiente", command).unwrap();
            assert!(next.interactive_message.as_option().is_some());
        }
    }
}

#[test]
fn native_selection_accepts_known_id_variants_but_not_mutation() {
    for (name, payload, expected) in [
        (
            Some("single_select"),
            serde_json::json!({"singleSelectReply": {"selectedRowId": "matter.read.menu.web"}}),
            Some("=/>menu web"),
        ),
        (
            None,
            serde_json::json!({"selectedId": "matter.read.menu.classic"}),
            Some("=/>menu classic"),
        ),
        (
            Some("single_select"),
            serde_json::json!({"selectedButtonId": "matter.read.menu.buttons"}),
            Some("=/>menu buttons"),
        ),
        (
            None,
            serde_json::json!({"selectedId": "matter.read.roles.grant"}),
            None,
        ),
    ] {
        let message = wa::Message {
            interactive_response_message: MessageField::some(
                wa::message::InteractiveResponseMessage {
                    interactive_response_message: Some(
                        wa::message::interactive_response_message::InteractiveResponseMessage::NativeFlowResponseMessage(
                            Box::new(
                                wa::message::interactive_response_message::NativeFlowResponseMessage {
                                    name: name.map(str::to_owned),
                                    params_json: Some(payload.to_string()),
                                    ..Default::default()
                                },
                            ),
                        ),
                    ),
                    ..Default::default()
                },
            ),
            ..Default::default()
        };
        assert_eq!(readonly_button_command(&message), expected);
    }
}

#[test]
fn category_menu_keeps_its_text_and_its_one_final_selector() {
    let message = native_navigation_message("Protecciones", "=/>menu anti").unwrap();
    let interactive = message.interactive_message.as_option().unwrap();
    assert_eq!(
        interactive.body.as_option().unwrap().text.as_deref(),
        Some("Protecciones")
    );
    let Some(wa::message::interactive_message::InteractiveMessage::NativeFlowMessage(native)) =
        interactive.interactive_message.as_ref()
    else {
        panic!("Expected NativeFlow");
    };
    assert_eq!(native.buttons.len(), 1);
    assert_eq!(native.buttons[0].name.as_deref(), Some("single_select"));
    assert!(native_navigation_message("Unsafe", "=/>set role owner").is_err());
}

#[test]
fn formats_are_readonly_and_selected_through_menu_picker() {
    for (id, command) in [
        ("matter.read.menu.classic", "=/>menu classic"),
        ("matter.read.menu.web", "=/>menu web"),
        ("matter.read.menu.buttons", "=/>menu buttons"),
        ("matter.read.menu.all", "=/>menu all"),
    ] {
        assert_eq!(
            readonly_button_command(&native_click(id, "single_select")),
            Some(command)
        );
        assert_eq!(readonly_button_command(&list_click(id)), Some(command));
    }
    assert!(
        readonly_button_command(&native_click("matter.read.roles.grant", "single_select"))
            .is_none()
    );
}

#[test]
fn web_navigation_can_show_text_after_definite_html_failure() {
    let message = whatsapp_matter_rust_provider::ui::native_navigation_message_with_web_fallback(
        "Contenido",
        "=/>menu web",
        true,
    )
    .unwrap();
    let interactive = message.interactive_message.as_option().unwrap();
    assert_eq!(
        interactive.body.as_option().unwrap().text.as_deref(),
        Some("Contenido")
    );
}

#[test]
fn web_navigation_has_no_classic_body_or_mode_heading() {
    let original = "MARCO CLASICO\nCategorías\n=/>status";
    let web = native_navigation_message(original, "=/>menu web").unwrap();
    let interactive = web.interactive_message.as_option().unwrap();
    assert!(interactive.body.as_option().is_none());
    let classic = native_navigation_message(original, "=/>menu classic").unwrap();
    assert_eq!(
        classic
            .interactive_message
            .as_option()
            .unwrap()
            .body
            .as_option()
            .unwrap()
            .text
            .as_deref(),
        Some(original)
    );
}

#[test]
fn native_navigation_has_one_visible_message_and_one_last_selector() {
    for (request, expected) in [
        ("=/>menu buttons", "*Menu Buttons*"),
        ("=/>menu classic", "Categorías"),
        ("=/>menu all", "Categorías"),
    ] {
        let message = native_navigation_message("Categorías", request).unwrap();
        let interactive = message.interactive_message.as_option().unwrap();
        assert!(interactive
            .body
            .as_option()
            .unwrap()
            .text
            .as_deref()
            .unwrap()
            .contains(expected));
        assert!(interactive.footer.as_option().is_none());
        let Some(wa::message::interactive_message::InteractiveMessage::NativeFlowMessage(native)) =
            interactive.interactive_message.as_ref()
        else {
            panic!("Expected NativeFlow");
        };
        assert_eq!(native.buttons.len(), 1);
        assert_eq!(native.buttons[0].name.as_deref(), Some("single_select"));
        assert!(message.bot_forwarded_message.as_option().is_none());
        assert!(message.rich_response_message.as_option().is_none());
        assert!(message.buttons_message.as_option().is_none());
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
        assert_eq!(readonly_button_command(&list_click(id)), Some(command));
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
        assert_eq!(readonly_button_command(&list_click(id)), None);
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
        assert_eq!(readonly_button_command(&list_click(id)), Some(command));
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
        assert_eq!(readonly_button_command(&list_click(id)), Some(command));
    }
}

/// Offline TS18/TS19 cards: only allowed, owner-scoped test acknowledgments.
#[test]
fn ts18_ts19_menu_lab_cards_preserve_readonly_click_mapping() {
    for (variant, size) in [(MenuCardVariant::Ts18, 2), (MenuCardVariant::Ts19, 3)] {
        let card = menu_lab_card(variant);
        let buttons = &card.buttons_message.as_option().unwrap().buttons;
        assert_eq!(buttons.len(), size);
        for (index, expected) in ["=/>test ack audio", "=/>test ack mp4", "=/>test ack link"]
            .iter()
            .take(size)
            .enumerate()
        {
            let id = buttons[index].button_id.as_ref().unwrap();
            assert_eq!(readonly_button_command(&legacy_click(id)), Some(*expected));
        }
        assert!(card.rich_response_message.as_option().is_none());
        assert!(card.audio_message.as_option().is_none());
    }
}

#[test]
fn native_ts18_ts19_buttons_use_quick_reply_and_readonly_ack_ids() {
    for (variant, expected_len) in [(MenuCardVariant::Ts18, 2), (MenuCardVariant::Ts19, 3)] {
        let card = native_menu_lab_card(variant);
        let interactive = card.interactive_message.as_option().unwrap();
        let Some(wa::message::interactive_message::InteractiveMessage::NativeFlowMessage(native)) =
            interactive.interactive_message.as_ref()
        else {
            panic!("Expected a NativeFlow card");
        };
        assert_eq!(native.buttons.len(), expected_len);
        for (index, (id, label, expected_command)) in [
            ("matter.test.row.audio", "Audio", "=/>test ack audio"),
            ("matter.test.row.mp4", "MP4", "=/>test ack mp4"),
            ("matter.test.row.link", "Enlace", "=/>test ack link"),
        ]
        .iter()
        .take(expected_len)
        .enumerate()
        {
            let button = &native.buttons[index];
            assert_eq!(button.name.as_deref(), Some("quick_reply"));
            let data: serde_json::Value =
                serde_json::from_str(button.button_params_json.as_deref().unwrap()).unwrap();
            assert_eq!(data["id"], *id);
            assert_eq!(data["display_text"], *label);
            assert_eq!(
                readonly_button_command(&native_click(id, "quick_reply")),
                Some(*expected_command)
            );
        }
        assert!(card.buttons_message.as_option().is_none());
        assert!(card.rich_response_message.as_option().is_none());
    }
}

#[test]
fn buttons_use_catalog_descriptors_and_preserve_category_context() {
    use whatsapp_matter_rust_provider::{ui, MenuItem};
    let items = vec![
        MenuItem {
            id: "matter.read.buttons.menu.ia".to_owned(),
            title: "IA".to_owned(),
        },
        MenuItem {
            id: "matter.read.buttons.menu.anti.2".to_owned(),
            title: "Siguiente".to_owned(),
        },
        MenuItem {
            id: "matter.read.help.command.matter".to_owned(),
            title: "Ayuda: matter".to_owned(),
        },
        MenuItem {
            id: "matter.read.status".to_owned(),
            title: "status".to_owned(),
        },
    ];
    let cards = ui::button_menu_cards(&items, "IA");
    assert_eq!(cards.len(), 2);
    assert_eq!(
        ui::menu_button_selection(&native_click(&items[0].id, "quick_reply")),
        Some(("=/>menu ia".to_owned(), true))
    );
    assert_eq!(
        ui::menu_button_selection(&native_click(&items[1].id, "quick_reply")),
        Some(("=/>menu anti 2".to_owned(), true))
    );
    assert_eq!(
        ui::menu_button_selection(&native_click(&items[2].id, "quick_reply")),
        Some(("=/>help matter".to_owned(), true))
    );
    assert!(ui::menu_button_selection(&native_click(
        "matter.read.buttons.menu.classic",
        "quick_reply"
    ))
    .is_none());
}
