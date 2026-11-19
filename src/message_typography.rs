/*
 * Matterssmith — reusable Unicode alphabet and WhatsApp text formatter.
 * Visual formatting grants no authority and does not execute commands.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{anyhow, bail, Result};
use serde_json::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextFormat {
    Plain,
    Bold,
    Italic,
    Strikethrough,
    InlineCode,
    CodeBlock,
}

/// Formats one trusted text segment. Avoid nested unbalanced WhatsApp markup.
pub fn format_segment(value: &str, format: TextFormat) -> Result<String> {
    if value.is_empty()
        || value.len() > 3200
        || value.chars().any(|ch| ch.is_control() && ch != '\n')
    {
        bail!("Invalid format segment");
    }
    let delimiter = match format {
        TextFormat::Plain => return Ok(value.to_owned()),
        TextFormat::Bold => "*",
        TextFormat::Italic => "_",
        TextFormat::Strikethrough => "~",
        TextFormat::InlineCode => "`",
        TextFormat::CodeBlock => "```",
    };
    if value.contains(delimiter) {
        bail!("Nested markup is not allowed");
    }
    Ok(format!("{delimiter}{value}{delimiter}"))
}

/// Return an alternative alphabet. Characters not in A-Z, a-z or 0-9
/// (including accented letters) remain unchanged, preserving meaning.
pub fn font_text(value: &str, font: &str) -> Result<String> {
    let catalog: Value = serde_json::from_str(include_str!("../templates/message-alphabets.json"))?;
    let table = catalog["fonts"]
        .get(font)
        .ok_or_else(|| anyhow!("Unknown alphabet"))?;
    let keys = ["uppercase", "lowercase", "digits"];
    let originals = [
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "abcdefghijklmnopqrstuvwxyz",
        "0123456789",
    ];
    let mut result = String::new();
    for ch in value.chars() {
        let mut mapped = None;
        for (key, original) in keys.iter().zip(originals) {
            if let Some(index) = original.chars().position(|letter| letter == ch) {
                let glyphs = table[*key]
                    .as_str()
                    .ok_or_else(|| anyhow!("Invalid alphabet"))?;
                mapped = glyphs.chars().nth(index);
                if mapped.is_none() {
                    bail!("Incomplete alphabet");
                }
                break;
            }
        }
        result.push(mapped.unwrap_or(ch));
    }
    Ok(result)
}

/// Apply the shared decorative frame to arbitrary message text.
/// No personalization, credentials or role checks are inferred by this function.
pub fn render_message(title: &str, body: &str, font: &str) -> Result<String> {
    for content in [title, body] {
        if content.is_empty()
            || content.len() > 3200
            || content.chars().any(|ch| ch.is_control() && ch != '\n')
        {
            bail!("Invalid message content");
        }
    }
    let title = format_segment(&font_text(title, font)?, TextFormat::Bold)?;
    let template = include_str!("../templates/message-frame.txt");
    let output = template
        .replace("{{TITLE}}", &title)
        .replace("{{BODY}}", body);
    if output.len() > 3800 {
        bail!("Decorative message exceeds WhatsApp safe length");
    }
    Ok(output.trim_end_matches('\n').to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_fonts_are_complete_and_reusable() {
        for font in [
            "normal",
            "serif_bold",
            "sans_bold",
            "monospace",
            "fullwidth",
        ] {
            let alphabet = font_text(
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
                font,
            )
            .unwrap();
            assert_eq!(alphabet.chars().count(), 62);
        }
        assert_eq!(font_text("abc ñá 123", "normal").unwrap(), "abc ñá 123");
        assert!(font_text("texto", "unknown").is_err());
    }

    #[test]
    fn whatsapp_text_modes_distinguish_content_from_decoration() {
        assert_eq!(
            format_segment("Título", TextFormat::Bold).unwrap(),
            "*Título*"
        );
        assert_eq!(
            format_segment("menu anti", TextFormat::InlineCode).unwrap(),
            "`menu anti`"
        );
        assert_eq!(
            format_segment("codigo", TextFormat::CodeBlock).unwrap(),
            "```codigo```"
        );
        assert!(format_segment("un`safe", TextFormat::InlineCode).is_err());
        let result = render_message("Aviso", "`=/>status` — Estado del chat", "sans_bold").unwrap();
        assert!(result.starts_with("*꒦꒷✰"));
        assert!(result.contains("`=/>status`"));
        assert!(result.ends_with("╰───────────⟨ 🌀"));
    }
}
