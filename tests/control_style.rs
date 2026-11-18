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
    assert!(menu.contains("=/>bot on/off"));
    assert!(!menu.contains("=/>on / =/>off"));
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

    // Templates are separately editable and cannot remove required attribution.
    let custom = "*{{TITLE}}* {{ACCENT}}\n{{BODY}}\nCréditos: Hecho por Lournyx Matterssmith [Lou-Nyx].\n";
    let changed = ControlTheme::new("Matter", "base-matter", "✦")
        .unwrap()
        .with_template(custom)
        .unwrap();
    assert!(changed.decorate("BOT ACTIVADO").unwrap().contains("Matter"));
    let custom_response = changed.decorate("BOT ACTIVADO").unwrap();
    assert_eq!(changed.decorate(&custom_response).unwrap(), custom_response);
    assert!(changed.decorate(&format!("{custom_response}\u{0000}")).is_err());
    assert!(changed.decorate(&format!("{custom_response}{}", "X".repeat(3900))).is_err());
    assert!(ControlTheme::new("Matter", "base-matter", "✦")
        .unwrap()
        .with_template("{{BODY}}")
        .is_err());

    let status = style.decorate("BOT ACTIVADO").unwrap();
    assert!(status.contains("BOT ACTIVADO"));
    assert_eq!(style.decorate(&menu).unwrap(), menu);
    assert!(style.decorate(&format!("{menu}\u{0000}")).is_err());
    assert!(style.decorate(&format!("{menu}{}", "X".repeat(3900))).is_err());
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
        .contains("*Base* · *"));
    assert!(ControlTheme::new("Unsafe\ncommand", "id", "*").is_err());
    assert!(registry.render(Some("../foreign"), "menu").is_err());
}
