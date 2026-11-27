/*
 * Shared Matter visual presentation for the core and standalone provider.
 * Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use serde_json::Value;
use std::sync::OnceLock;

fn theme() -> &'static Value {
    static THEME: OnceLock<Value> = OnceLock::new();
    THEME.get_or_init(|| {
        serde_json::from_str(include_str!("../templates/menu/theme.json"))
            .expect("Valid bundled Matter theme")
    })
}

/// Canonical Matter layout, shared by the core and standalone provider.
pub fn layout(name: &str) -> &'static str {
    theme()["matter_templates"][name]
        .as_str()
        .expect("Bundled Matter layout")
}

/// Expanded built-in Matter layout, before inserting dynamic values.
pub fn expanded_layout(name: &str) -> String {
    expand(layout(name))
}

/// Maintenance sections use the shared Matter rows. Only the last frame is closed.
pub fn maintenance_report(kind: &str, body: &str) -> Option<String> {
    maintenance_reports(&[(kind, body)])
}

pub fn maintenance_reports(sections: &[(&str, &str)]) -> Option<String> {
    if sections.is_empty() {
        return None;
    }
    let mut output = String::new();
    for (index, (kind, body)) in sections.iter().enumerate() {
        let name = match *kind {
            "Update" => "update",
            "Restart" => "restart",
            "Cambios" => "update-changes",
            "Verificación" => "update-verification",
            "Set Update" => "update-settings",
            "Reload" => "reload",
            "Codex" => "codex",
            _ => return None,
        };
        if body.trim().is_empty() {
            return None;
        }
        let mut lines = body.lines().peekable();
        let mut rows = String::new();
        while let Some(line) = lines.next() {
            let row = if lines.peek().is_some() {
                "row"
            } else {
                "last-row"
            };
            rows.push_str(&render(console_template(row), &[("BODY", line)]));
        }
        let final_block = index + 1 == sections.len();
        let footer = if final_block {
            expand("{{CLASSIC_CLOSE}}")
        } else {
            expand("{{CLASSIC_JOINED_CLOSE}}")
        };
        output.push_str(&render(
            layout(name),
            &[("ROWS", &rows), ("FOOTER", &footer)],
        ));
        if !final_block {
            output.push('\n');
            output.push_str(&expand("{{CLASSIC_CONNECTOR}}"));
            output.push('\n');
        }
    }
    Some(output)
}

/// Dynamic menu layout; unknown sections are handled by the caller.
pub fn menu_layout(name: &str) -> Option<&'static str> {
    theme()["matter_templates"]["menu"][name].as_str()
}

/// Terminal framing is independent of the WhatsApp frame.
pub fn console_template(name: &str) -> &'static str {
    theme()["console"]["templates"][name]
        .as_str()
        .expect("Bundled console template")
}

/// Expand trusted bundled template source before interpolating user data.
pub fn expand(source: &str) -> String {
    let mut result = source.replace("\r\n", "\n");
    let theme = theme();
    let frame = &theme["matter_frame"];
    let prefix = frame["header_prefix"]
        .as_str()
        .expect("Matter header prefix");
    let emblem = frame["emblem"].as_str().expect("Matter emblem");
    result = result.replace("{{CLASSIC_HEADER}}", &format!("{prefix}{emblem}"));
    for (token, key) in [
        ("EMBLEM", "emblem"),
        ("ROW", "row"),
        ("COMMAND_ROW", "row"),
        ("LINE", "line"),
        ("CLOSE", "close"),
        ("JOINED_CLOSE", "joined_close"),
        ("CONNECTOR", "connector"),
        ("CONTROL_CLOSE", "joined_close"),
    ] {
        let value = theme["matter_frame"][key]
            .as_str()
            .expect("Bundled Matter frame token");
        result = result.replace(&format!("{{{{CLASSIC_{token}}}}}"), value);
    }
    for (token, key) in [("BRANCH", "branch"), ("LAST_BRANCH", "last_branch")] {
        let value = theme["decoration"][key]
            .as_str()
            .expect("Bundled Matter branch token");
        result = result.replace(&format!("{{{{CLASSIC_{token}}}}}"), value);
    }
    result
}

/// Terminal presentation uses the same versioned theme as the Matter menu.
pub fn console_palette(name: &str) -> &'static str {
    theme()["console"]["palette"][name]
        .as_str()
        .expect("Bundled console palette")
}

pub fn console_label(name: &str) -> &'static str {
    theme()["console"]["labels"][name]
        .as_str()
        .expect("Bundled console label")
}

pub fn console_layout(name: &str, fields: &[(&str, &str)]) -> String {
    let source = theme()["console"]["rows"][name]
        .as_str()
        .expect("Bundled console layout");
    render(source, fields)
}

/// Insert fields in one pass; inserted values are never reinterpreted.
pub fn render(source: &str, fields: &[(&str, &str)]) -> String {
    let source = expand(source);
    let mut rest = source.as_str();
    let mut output = String::new();
    while let Some((before, after)) = rest.split_once("{{") {
        output.push_str(before);
        let Some((key, tail)) = after.split_once("}}") else {
            output.push_str("{{");
            output.push_str(after);
            return output;
        };
        if let Some((_, value)) = fields.iter().find(|(name, _)| *name == key) {
            output.push_str(value);
        } else {
            output.push_str("{{");
            output.push_str(key);
            output.push_str("}}");
        }
        rest = tail;
    }
    output.push_str(rest);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn console_layout_is_data_only() {
        assert_eq!(
            console_layout("command", &[("COMMAND", "{{CLASSIC_HEADER}}")]),
            "CMD {{CLASSIC_HEADER}}"
        );
        assert_eq!(console_palette("frame"), "1;96");
    }

    #[test]
    fn maintenance_reports_use_the_compact_matter_design() {
        let update =
            maintenance_report("Update", "Estado: Código actualizado\nBot: sigue activo").unwrap();
        assert_eq!(
            update,
            "╭─ 💠 𝗨𝗽𝗱𝗮𝘁𝗲\n│·♪·╠[ Estado: Código actualizado\n│·♪·╚[ Bot: sigue activo\n╰───────────⟨ 🌀"
        );
        let restart = maintenance_report("Restart", "{{CLASSIC_HEADER}}").unwrap();
        assert_eq!(
            restart,
            "╭─ 💠 𝗥𝗲𝘀𝘁𝗮𝗿𝘁\n│·♪·╚[ {{CLASSIC_HEADER}}\n╰───────────⟨ 🌀"
        );
        let joined = maintenance_reports(&[("Update", "OK"), ("Restart", "Manual")]).unwrap();
        assert!(joined.contains("╰─╮╭────────⟨ 🌀\n   ││\n╭─ 💠 𝗥𝗲𝘀𝘁𝗮𝗿𝘁"));
        assert!(joined.ends_with("╰───────────⟨ 🌀"));
        assert!(maintenance_report("Update", "").is_none());
        assert!(maintenance_report("unknown", "Estado: OK").is_none());
    }

    #[test]
    fn shared_frame_matches_owner_design() {
        let header = expand("{{CLASSIC_HEADER}}");
        let close = expand("{{CLASSIC_JOINED_CLOSE}}");
        let connector = expand("{{CLASSIC_CONNECTOR}}");
        assert_eq!(header, "╭─ 💠");
        assert_eq!(close, "╰─╮╭────────⟨ 🌀");
        assert_eq!(connector, "   ││");
        assert_eq!(expand("{{CLASSIC_CONTROL_CLOSE}}"), close);
        assert_eq!(expand("{{CLASSIC_COMMAND_ROW}}"), expand("{{CLASSIC_ROW}}"));
        assert_eq!(
            render(
                "{{CLASSIC_ROW}} {{BODY}}",
                &[("BODY", "{{CLASSIC_HEADER}}")]
            ),
            "│·♪· {{CLASSIC_HEADER}}"
        );
    }
}
