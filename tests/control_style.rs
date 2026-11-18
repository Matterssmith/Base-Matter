/*
 * Base-Matter — reusable visual layout must not cross community scopes.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use matter_base::control_style::{ControlTheme, ControlThemes};

#[test]
fn matterssmith_default_menu_has_categories_and_removes_help_h() {
    let style = ControlTheme::new("Matter", "base-matter", "✦").unwrap();
    let menu = style.render("menu").unwrap();
    assert!(menu.contains("Bot ID: base-matter"));
    assert!(menu.contains("꒦꒷✰"));
    assert!(menu.contains("=/>menu group"));
    assert!(menu.contains("=/>menu config"));
    assert!(menu.contains("Lournyx Matterssmith [Lou-Nyx]"));
    assert!(style.render("menu group").unwrap().contains("=/>start"));
    assert!(style.render("menu mode").unwrap().contains("=/>bot"));
    assert!(style.render("menu vault").unwrap().contains("Vault"));
    assert!(style.render("help").is_err());
    assert!(style.render("h").is_err());
    assert!(style.render("menu unsupported").is_err());
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
        .contains("Bot ID: default"));
    assert!(ControlTheme::new("Unsafe\ncommand", "id", "*").is_err());
    assert!(registry.render(Some("../foreign"), "menu").is_err());
}
