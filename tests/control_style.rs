/*
 * Base-Matter — compact scoped menu and verified control frames.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use matter_base::control_style::{ControlTheme, ControlThemes};

#[test]
fn global_menu_restores_owner_original_frame_and_single_credit_footer() {
    let style = ControlTheme::new("Matter", "base-matter", "✦").unwrap();
    let menu = style.render("menu").unwrap();

    assert!(menu.starts_with("*꒦꒷✰꒷꒦"));
    assert!(menu.contains("꧁✪⃢.✪ 𝐌𝐚𝐭𝐭𝐞𝐫𝐬𝐬𝐦𝐢𝐭𝐡"));
    assert!(menu.contains("*Base-Matter* · ✦"));
    assert!(menu.contains("│·♪·╠[ ◉ =/>status — Estado de este chat"));
    assert!(menu.contains("│·♪·╠[ ⓘ =/>status --mod — Configuración del socket [propietario]"));
    assert!(menu.contains("│·♪·╚[ ⴵ =/>audit — Auditoría [propietario]"));
    assert!(menu.contains("💠 =/>status funciona incluso cuando el bot está apagado."));
    assert!(menu.ends_with("Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx]."));
    assert_eq!(menu.matches("Créditos: Hecho por").count(), 1);
    // Preserve the canonical original menu, not a stale hardcoded frame.
    let canonical = matter_base::classic_template::expand(include_str!("../templates/matterssmith-menu.txt"));
    assert_eq!(menu, canonical.trim_end_matches('\n'));
    assert_eq!(menu.matches("╭─ 💠 𝗖𝗼𝗻𝘁𝗿𝗼𝗹").count(), 1);
    assert!(menu.contains('\n'));
    assert!(!menu.contains("\\n"));
    assert!(!menu.contains("Categorías"));
    assert!(!menu.contains("Bot ID:"));
    assert!(style.render("menu group").is_err());
    assert!(style.render("help").is_err());
    assert!(style.render("h").is_err());

    // Custom frames still cannot add automatic credit footers to statuses.
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
    assert!(second.starts_with("*꒦꒷✰꒷꒦"));
    assert_ne!(first, second);
    assert!(!first.contains("community-one"));
    assert!(second.contains("Créditos: Hecho por Lournyx Matterssmith"));
    let status = registry
        .decorate(Some("community-1@g.us"), "BOT ACTIVADO")
        .unwrap();
    assert!(status.contains("*Register* · ✦"));
    assert!(status.contains("BOT ACTIVADO"));
    assert!(!status.contains("*Base* · *"));
    assert!(ControlTheme::new("Unsafe\ncommand", "id", "*").is_err());
    assert!(registry.render(Some("../foreign"), "menu").is_err());
}
