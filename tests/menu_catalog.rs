use matter_base::control_style::{ControlTheme, ControlThemes};
use matter_base::menu::{Language, MenuPreferences, Preferences, Style};
use std::sync::Arc;

#[test]
fn live_menu_registry_builds_categories_without_personal_brand_or_footer_credits() {
    let prefs = Arc::new(MenuPreferences::open(":memory:").unwrap());
    let control = ControlThemes::new(ControlTheme::new("Base-Matter", "principal", "✦").unwrap())
        .with_dynamic_menu(Arc::clone(&prefs));
    let root = control.render(None, "menu").unwrap();
    assert!(root.starts_with("╭─💠 𝗖𝗮𝘁𝗲𝗴𝗼𝗿í𝗮𝘀"));
    assert!(root.contains("│·♪·╠[ ⴵ ·· 𝙸𝙰 [menu ai]"));
    assert!(root.contains("╰───────────⟨ 🌀"));
    assert!(root.contains("menu set"));
    assert!(root.contains("menu antis"));
    assert!(root.contains("menu bot"));
    assert!(!root.contains("Matterssmith"));
    assert!(!root.contains("Créditos:"));
    assert_eq!(root.matches("=/>").count(), 1);
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
    assert!(menu.contains("Change menu style"));
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
    let page = menu::render("menu antis 2", Preferences::default()).unwrap();
    assert!(page.contains("antilink"));
    assert!(menu::render("menu antis 9", Preferences::default()).is_err());
    assert!(menu::render("menu no-such-category", Preferences::default()).is_err());
}

#[test]
fn help_queries_support_command_metadata_and_category_pages() {
    use matter_base::menu;
    let options = Preferences::default();
    let help = menu::render_help("antilink", options).unwrap();
    assert!(help.contains("Ayuda: antilink"));
    assert!(help.contains("=/>antilink on|off|inherit"));
    let page = menu::render_help("antis 2", options).unwrap();
    assert!(page.contains("antilink"));
    let test = menu::render_help("test", options).unwrap();
    assert!(test.contains("test ui"));
    assert!(test.contains("test html"));
    assert!(menu::render_help("not-registered", options).is_err());
}
