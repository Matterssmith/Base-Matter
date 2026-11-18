/*
 * Base-Matter — compact scoped menu and verified control frames.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use matter_base::control_style::{ControlTheme, ControlThemes};

#[test]
fn global_menu_uses_one_prefix_grouped_arguments_and_no_footer_credits() {
    let style = ControlTheme::new("Matter", "base-matter", "✦").unwrap();
    let menu = style.render("menu").unwrap();

    assert!(menu.starts_with("╭─💠 𝗖𝗼𝗻𝘁𝗿𝗼𝗹"));
    assert!(menu.contains("*Prefijo global:* `=/>`"));
    assert_eq!(menu.matches("=/>").count(), 1);
    assert!(menu.contains("status [--global]"));
    assert!(menu.contains("bot [ámbito] on|off|inherit"));
    assert!(menu.contains("--global · group · private · channel · community"));
    assert!(menu.contains("credits — Créditos"));
    assert!(!menu.contains("Lournyx Matterssmith"));
    assert!(!menu.contains("Créditos: Hecho por"));
    assert!(!menu.contains("*Base-Matter*"));
    assert!(menu.contains('\n'));
    assert!(!menu.contains("\\n"));
    assert!(!menu.contains("Categorías"));
    assert!(!menu.contains("Bot ID:"));
    assert!(style.render("menu group").is_err());
    assert!(style.render("help").is_err());
    assert!(style.render("h").is_err());

    // Per-chat themes may opt into title/accent fields, but never an
    // automatic credit footer. The caller's credit command is separate.
    let custom = "╭─*{{TITLE}}* · {{ACCENT}}\n{{BODY}}\n╰─☆\n";
    let changed = ControlTheme::new("Matter", "base-matter", "✦")
        .unwrap()
        .with_template(custom)
        .unwrap();
    let custom_response = changed.decorate("BOT ACTIVADO").unwrap();
    assert!(custom_response.contains("*Matter* · ✦"));
    assert!(!custom_response.contains("Créditos:"));
    assert_eq!(changed.decorate(&custom_response).unwrap(), custom_response);
    assert!(changed
        .decorate(&format!("{custom_response}\u{0000}"))
        .is_err());
    assert!(changed
        .decorate(&format!("{custom_response}{}", "X".repeat(3900)))
        .is_err());
    assert!(ControlTheme::new("Matter", "base-matter", "✦")
        .unwrap()
        .with_template("{{BODY}}")
        .is_err());
    assert!(ControlTheme::new("Matter", "base-matter", "✦")
        .unwrap()
        .with_template("╭─\n{{BODY}}\nCréditos: Hecho por Lournyx Matterssmith [Lou-Nyx].")
        .is_err());

    let status = style.decorate("BOT ACTIVADO").unwrap();
    assert!(status.contains("BOT ACTIVADO"));
    assert!(!status.contains("Lournyx"));
    assert_eq!(style.decorate(&menu).unwrap(), menu);
    assert!(style.decorate(&format!("{menu}\u{0000}")).is_err());
    assert!(style
        .decorate(&format!("{menu}{}", "X".repeat(3900)))
        .is_err());
}

#[test]
fn custom_community_themes_remain_isolated_and_fail_closed() {
    let registry = ControlThemes::new(ControlTheme::new("Base", "default", "*").unwrap());
    let custom = ControlTheme::new("Register", "community-one", "✦")
        .unwrap()
        .with_template("╭─*{{TITLE}}* · {{ACCENT}}\n{{BODY}}\n╰─☆\n")
        .unwrap();
    registry.set_community("community-1@g.us", custom).unwrap();

    let first = registry.render(Some("community-1@g.us"), "menu").unwrap();
    let second = registry.render(Some("community-2@g.us"), "menu").unwrap();
    assert!(first.contains("*Register* · ✦"));
    assert!(second.starts_with("╭─💠 𝗖𝗼𝗻𝘁𝗿𝗼𝗹"));
    assert_ne!(first, second);
    assert!(!first.contains("community-one"));
    assert!(!second.contains("Créditos:"));
    let status = registry
        .decorate(Some("community-1@g.us"), "BOT ACTIVADO")
        .unwrap();
    assert!(status.contains("*Register* · ✦"));
    assert!(status.contains("BOT ACTIVADO"));
    assert!(!status.contains("*Base* · *"));
    assert!(ControlTheme::new("Unsafe\ncommand", "id", "*").is_err());
    assert!(registry.render(Some("../foreign"), "menu").is_err());
}
