/*
 * Base-Matter — reusable visual layout must not cross community scopes.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use matter_base::control_style::{ControlTheme, ControlThemes};

#[test]
fn matterssmith_global_menu_is_compact_and_uses_real_newlines() {
    let style = ControlTheme::new("Matter", "base-matter", "✦").unwrap();
    let menu = style.render("menu").unwrap();
    assert!(menu.starts_with("*꒦꒷✰"));
    assert!(menu.contains("꧁✪⃢.✪"));
    assert!(menu.contains("=/>on / =/>off"));
    assert!(menu.contains("=/>status"));
    assert!(menu.contains("=/>bot --global"));
    assert!(menu.contains("Lournyx Matterssmith [Lou-Nyx]"));
    assert!(menu.contains('\n'));
    assert!(!menu.contains("\\n"));
    assert!(!menu.contains("Categorías"));
    assert!(!menu.contains("=/>menu group"));
    assert!(!menu.contains("Bot ID:"));
    assert!(style.render("menu group").is_err());
    assert!(style.render("help").is_err());
    assert!(style.render("h").is_err());
    assert!(style.render("start").is_err());

    let status = style.decorate("BOT ACTIVADO").unwrap();
    assert!(status.contains("BOT ACTIVADO"));
    assert_eq!(style.decorate(&menu).unwrap(), menu);
}

#[test]
fn community_styles_are_independent_and_untrusted_fields_fail_closed() {
    let registry = ControlThemes::new(ControlTheme::new("Base", "default", "*").unwrap());
    registry
        .set_community(
            "community-1@g.us",
            ControlTheme::new("Register", "community-one", "✦").unwrap(),
        )
        .unwrap();
    assert!(registry
        .render(Some("community-1@g.us"), "menu")
        .unwrap()
        .contains("community-one"));
    assert!(registry
        .render(Some("community-2@g.us"), "menu")
        .unwrap()
        .contains("Bot ID: default"));
    assert!(ControlTheme::new("Unsafe\ncommand", "id", "*").is_err());
    assert!(registry.render(Some("../foreign"), "menu").is_err());
}
