/*
 * Shared Matter classic presentation for the core and standalone provider.
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

/// Expand trusted bundled template source before interpolating user data.
pub fn expand(source: &str) -> String {
    let mut result = source.replace("\r\n", "\n");
    let theme = theme();
    for (token, key) in [
        ("HEADER", "header"),
        ("EMBLEM", "emblem"),
        ("ROW", "row"),
        ("COMMAND_ROW", "command_row"),
        ("LINE", "line"),
        ("CLOSE", "close"),
        ("JOINED_CLOSE", "joined_close"),
        ("CONNECTOR", "connector"),
        ("CONTROL_CLOSE", "control_close"),
    ] {
        let value = theme["classic_frame"][key]
            .as_str()
            .expect("Bundled classic frame token");
        result = result.replace(&format!("{{{{CLASSIC_{token}}}}}"), value);
    }
    for (token, key) in [("BRANCH", "branch"), ("LAST_BRANCH", "last_branch")] {
        let value = theme["decoration"][key]
            .as_str()
            .expect("Bundled classic branch token");
        result = result.replace(&format!("{{{{CLASSIC_{token}}}}}"), value);
    }
    result
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
    fn shared_connector_matches_two_cell_emblem() {
        let header = expand("{{CLASSIC_HEADER}}");
        let close = expand("{{CLASSIC_JOINED_CLOSE}}");
        let connector = expand("{{CLASSIC_CONNECTOR}}");
        assert_eq!(header.chars().position(|c| c == '💠'), Some(3));
        assert_eq!(close.chars().position(|c| c == '╮'), Some(3));
        assert_eq!(close.chars().position(|c| c == '╭'), Some(4));
        assert_eq!(connector.chars().position(|c| c == '│'), Some(3));
        assert_eq!(
            render("{{CLASSIC_ROW}} {{BODY}}", &[("BODY", "{{CLASSIC_HEADER}}")]),
            "│·♪· {{CLASSIC_HEADER}}"
        );
    }
}
