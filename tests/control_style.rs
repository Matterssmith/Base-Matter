/*
 * Base-Matter — reusable visual layout must not cross community scopes.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use matter_base::control_style::{ControlTheme, ControlThemes};

#[test]
fn menu_help_h_share_a_reusable_renderer() {
    let style = ControlTheme::new("Matter", "base-matter", "✦").unwrap();
    let menu = style.render("menu").unwrap();
    assert!(menu.contains("BOT ID: base-matter"));
    assert!(menu.contains("Matter"));
    assert!(menu.contains("=/>start"));
    assert!(menu.contains("Lournyx Matterssmith [Lou-Nyx]"));
    assert!(style.render("help").unwrap().contains("AYUDA"));
    assert_eq!(style.render("h").unwrap(), style.render("help").unwrap());
    assert!(style.render("start").is_err());
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
        .contains("BOT ID: default"));
    assert!(ControlTheme::new("Unsafe\ncommand", "id", "*").is_err());
    assert!(registry.render(Some("../foreign"), "menu").is_err());
}
