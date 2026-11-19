use matter_base::control_style::{ControlTheme, ControlThemes};
use matter_base::menu::{Language, MenuPreferences, Preferences, Style};
use std::sync::Arc;

#[test]
fn live_menu_registry_builds_categories_without_personal_brand_or_footer_credits() {
    let prefs = Arc::new(MenuPreferences::open(":memory:").unwrap());
    let control = ControlThemes::new(ControlTheme::new("Base-Matter", "principal", "✦").unwrap())
        .with_dynamic_menu(Arc::clone(&prefs));
    let root = control.render(None, "menu").unwrap();
    assert!(root.starts_with("*꒦꒷✰꒷꒦︶"));
    assert!(root.contains("│·♪·╠[ *ⴵ ·· 𝙸𝙰* `[menu ia]`"));
    assert!(root.contains("╰─╮╭────────⟨🌀"));
    assert!(root.contains("menu config"));
    assert!(root.contains("menu anti"));
    assert!(root.contains("menu mode"));
    assert!(!root.contains("Matterssmith"));
    assert!(!root.contains("Créditos:"));
    assert!(root.matches("=/>").count() >= 1);
    assert_eq!(matter_base::menu::CATEGORIES.len(), 20);
    for forbidden in [
        "[menu bot]",
        "[menu roles]",
        "[menu sandbox]",
        "[menu navigation]",
        "[menu set]",
        "[menu status]",
        "[menu antis]",
        "[menu ai]",
    ] {
        assert!(
            !root.contains(forbidden),
            "Legacy categories must not be visible: {forbidden}"
        );
    }
    assert!(!root.contains("[menu sockets]"));
    assert!(!root.contains("[menu group]"));
    assert!(!root.contains("[menu gacha]"));
    assert!(root.contains("`[menu anti]`"));
    assert!(root.contains("[menu utils]"));
    assert_eq!(control.decorate(None, &root).unwrap(), root);

    let set = control.render(None, "menu set").unwrap();
    assert!(set.contains("set style classic|none"));
    assert!(set.contains("set lang es|en"));
    assert_eq!(control.decorate(None, &set).unwrap(), set);
}

#[test]
fn style_none_removes_artwork_and_language_en_changes_descriptions() {
    let prefs = Arc::new(MenuPreferences::open(":memory:").unwrap());
    let control = ControlThemes::new(ControlTheme::new("Base-Matter", "principal", "✦").unwrap())
        .with_dynamic_menu(Arc::clone(&prefs));
    control.set_menu_style("none").unwrap();
    control.set_menu_language("en").unwrap();
    assert_eq!(
        prefs.get().unwrap(),
        Preferences {
            style: Style::None,
            language: Language::En,
        }
    );
    let menu = control.render(None, "menu set").unwrap();
    assert!(menu.starts_with("Global control menu"));
    assert!(menu.contains("=/>set style classic|none"));
    assert!(!menu.contains("╞"));
    assert!(!menu.contains("│"));
    assert!(!menu.contains("♪"));
    assert_eq!(control.decorate(None, &menu).unwrap(), menu);
    assert!(control.set_menu_style("untrusted").is_err());
    assert!(control.set_menu_language("invalid").is_err());
}

#[test]
fn individual_antis_are_public_metadata_but_never_execution_grants() {
    use matter_base::menu;
    for rule in matter_base::anti::RULES {
        assert!(menu::antis::CATEGORY
            .commands
            .iter()
            .any(|descriptor| descriptor.name == *rule));
    }
    let page = menu::render("menu antis 1", Preferences::default()).unwrap();
    assert!(page.contains("antilink"));
    assert!(!page.contains("📖 Ayuda: Anti"));
    let explanation = menu::render_help("antis 2", Preferences::default()).unwrap();
    assert!(explanation.contains("📖 Ayuda: Anti"));
    assert!(menu::render("menu antis 9", Preferences::default()).is_err());
    assert!(menu::render("menu no-such-category", Preferences::default()).is_err());
}

#[test]
fn help_queries_support_command_metadata_and_category_pages() {
    use matter_base::menu;
    let options = Preferences::default();
    let root_help = menu::render_help("", options).unwrap();
    assert!(root_help.starts_with("📖 Ayuda · Uso"));
    assert!(root_help.contains("=/>help <comando>"));
    assert!(!root_help.contains("╭─💠 𝗖𝗮𝘁𝗲𝗴𝗼𝗿í𝗮𝘀"));
    assert!(!root_help.contains("[menu ia]"));
    assert!(!root_help.contains("╭─💠 𝗔𝘂𝘁𝗼𝗿𝗶𝗱𝗮𝗱 𝗚𝗹𝗼𝗯𝗮𝗹"));
    let root_help_all = menu::render_help("all", options).unwrap();
    assert_eq!(root_help_all, root_help);
    let help = menu::render_help("antilink", options).unwrap();
    assert!(help.contains("Ayuda: antilink"));
    assert!(help.contains("=/>antilink on|off|inherit"));
    let page = menu::render_help("antis 1", options).unwrap();
    assert!(page.contains("antilink"));
    let test = menu::render_help("test", options).unwrap();
    assert!(test.contains("test ui"));
    assert!(test.contains("test html"));
    assert!(menu::render_help("not-registered", options).is_err());
    let anti_menu = menu::render("menu anti", options).unwrap();
    assert!(anti_menu.contains("=/>antilink"));
    assert!(menu::render("menu anti 4", options)
        .unwrap()
        .contains("=/>set anti link"));
    let all_anti = menu::render("menu all", options).unwrap();
    for retired in [
        "antilinkwa",
        "antilinktelegram",
        "antilinkdiscord",
        "antilinkinstagram",
        "antilinkfacebook",
        "antilinktiktok",
        "antilinkyoutube",
        "antilinkx",
        "antilinkshort",
        "antilinkphishing",
    ] {
        assert!(!anti_menu.contains(retired), "{retired}");
        assert!(!all_anti.contains(retired), "{retired}");
    }
    assert!(!anti_menu.contains("Protección link"));
    assert!(!anti_menu.contains("La lista de enlaces"));
    assert!(!anti_menu.contains(" [pendiente]"));
    let all = menu::render("menu all", options).unwrap();
    assert!(!all.contains(" [pendiente]"));
    assert!(!all.contains("console"));
    assert!(!all.contains("Actualización supervisada"));
    let empty_section = menu::render("menu gacha", options).unwrap();
    assert!(!empty_section.contains("Sin comandos"));
    assert!(!empty_section.contains("No commands"));
    assert!(!empty_section.contains("Gacha"));
    let help_none = menu::render_help(
        "",
        Preferences {
            style: Style::None,
            language: Language::Es,
        },
    )
    .unwrap();
    assert_eq!(root_help, help_none);
}

#[test]
fn global_menu_full_frame_and_plain_style_have_verified_safe_fields() {
    use matter_base::menu::{self, MenuViewer};
    let context = MenuViewer {
        user: "@owner_checked",
        role: "Owner del socket",
        status: "ACTIVADO",
        socket: "test-socket",
        socket_owner: true,
    };
    let classic = menu::render_root_with_viewer(Preferences::default(), context).unwrap();
    assert!(classic.starts_with("*꒦꒷✰꒷꒦︶"));
    assert!(classic.contains("💬 𝗦𝗶𝘀𝘁𝗲𝗺𝗮 𝗱𝗲 𝗖𝗼𝗻𝘁𝗿𝗼𝗹 𝗚𝗹𝗼𝗯𝗮𝗹"));
    assert!(classic.contains("╭─💠 𝗔𝘂𝘁𝗼𝗿𝗶𝗱𝗮𝗱 𝗚𝗹𝗼𝗯𝗮𝗹"));
    assert!(classic.contains("╭─💠 𝗜𝗻𝘀𝘁𝗿𝘂𝗰𝗰𝗶𝗼𝗻𝗲𝘀"));
    let quoted_instructions = classic
        .split_once("╭─💠 𝗜𝗻𝘀𝘁𝗿𝘂𝗰𝗰𝗶𝗼𝗻𝗲𝘀")
        .unwrap()
        .1
        .split_once("╰───────────⟨ 🌀")
        .unwrap()
        .0;
    let quote_lines: Vec<_> = quoted_instructions
        .lines()
        .filter(|line| !line.is_empty())
        .collect();
    assert_eq!(quote_lines.len(), 4);
    assert!(quote_lines.iter().all(|line| line.starts_with("> ")));
    assert!(quoted_instructions.contains("> ◉ `status` incluso con bot OFF."));
    assert!(quoted_instructions.contains("> ♛ Admin: identidad verificada."));
    assert!(quoted_instructions.contains("> ♧ Externos: OFF por defecto."));
    assert!(quoted_instructions.contains("> ◈ Sockets aislados · cambios auditados."));
    assert!(!quoted_instructions.contains("│   "));
    assert!(quoted_instructions.len() < 300);
    assert!(classic.contains("*@owner_checked*"));
    assert!(classic.contains("*Owner del socket*"));
    assert!(classic.contains("*ACTIVADO*"));
    assert!(classic.contains("*test-socket*"));
    assert!(!classic.contains("{total}"));
    assert!(!classic.contains("{rol}"));
    assert!(!classic.contains("Sin comandos registrados"));
    assert!(!classic.contains("\\n"));
    assert!(classic.len() <= 3800);
    assert!(classic.ends_with("*꒦꒷✰꒷꒦︶꒦꒷✧꒷꒦︶꒦꒷✰꒷꒦*"));
    let plain = menu::render_root_with_viewer(
        Preferences {
            style: Style::None,
            language: Language::Es,
        },
        context,
    )
    .unwrap();
    assert!(plain.starts_with("Menú de control global"));
    assert!(plain.contains("menu config"));
    assert!(plain.contains("menu utils"));
    assert!(!plain.contains("menu group"));
    assert!(!plain.contains("menu sockets"));
    assert!(plain.contains("Usuario: @owner_checked"));
    assert!(plain.contains("Socket: test-socket"));
    assert!(plain.contains("Admin: identidad verificada."));
    assert!(!plain.contains("> ◉"));
    assert!(!plain.contains('╭'));
    assert!(!plain.contains('♪'));
    assert!(plain.contains("IA: menu ia"));
    assert!(plain.contains("Todo: menu all"));
    assert!(!plain.contains("𝙸𝙰"));
    let public = menu::render_root_with_viewer(
        Preferences::default(),
        MenuViewer {
            user: "@usuario",
            role: "Usuario",
            status: "ACTIVADO",
            socket: "Privado",
            socket_owner: false,
        },
    )
    .unwrap();
    assert!(!public.contains("╭─💠 𝗔𝘂𝘁𝗼𝗿𝗶𝗱𝗮𝗱 𝗚𝗹𝗼𝗯𝗮𝗹"));
    assert!(!public.contains("test-socket"));
}

#[test]
fn quoted_command_help_uses_catalog_and_never_runs_the_original() {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
    let examples = [
        ("=/>anti audit", "anti"),
        ("=/>set anti link add t.me", "set anti link"),
        ("=/>bot on", "bot"),
        ("=/>register", "register"),
        ("=/>set role", "set role"),
    ];
    for (quoted, name) in examples {
        let encoded = URL_SAFE_NO_PAD.encode(quoted);
        let reply = menu::render_help(&format!("--quoted {encoded}"), Preferences::default()).unwrap();
        assert!(reply.contains(&format!("Ayuda completa: {name}")), "{quoted}: {reply}");
        assert!(!reply.contains("Bot activado"));
    }
    assert!(menu::render_help("--quoted !!!", Preferences::default()).is_err());
    let unknown = URL_SAFE_NO_PAD.encode("=/>this-command-does-not-exist");
    assert!(menu::render_help(&format!("--quoted {unknown}"), Preferences::default()).is_err());
    let normal = menu::render_help("set anti link", Preferences::default()).unwrap();
    assert!(normal.contains("Categoría:"));
    assert!(normal.contains("set anti link --global"));
}
