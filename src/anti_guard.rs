/*
 * Base-Matter — shared read-only anti inspection of edited and disguised text.
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

/// These states are metadata from the verified transport, never from text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EditState {
    Original,
    EditedReadable,
    EditedEncryptedUnavailable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContentReview {
    /// Text normalized ONLY for detection. Never dispatch as a command.
    pub inspection_text: Option<String>,
    pub edited: bool,
    pub unreadable: bool,
    pub hidden_or_confusable: bool,
    pub command_camouflage: bool,
    pub link_camouflage: bool,
}

/// Limited security normalization. We deliberately keep the original body
/// unchanged for display and execution; only the detection engine sees this.
fn mapped(character: char) -> Option<char> {
    Some(match character {
        '＝' => '=',
        '／' | '∕' | '⁄' => '/',
        '＞' => '>',
        '．' | '。' | '｡' | '․' => '.',
        '：' => ':',
        '＃' => '#',
        'а' | 'ɑ' => 'a',
        'е' => 'e',
        'о' => 'o',
        'р' => 'p',
        'с' => 'c',
        'х' => 'x',
        'і' => 'i',
        'ѕ' => 's',
        'ԁ' => 'd',
        'ｍ' => 'm',
        'ｅ' => 'e',
        'ｎ' => 'n',
        'ｕ' => 'u',
        _ => return None,
    })
}

fn invisible(character: char) -> bool {
    matches!(
        character,
        '\u{200B}' | '\u{200C}' | '\u{200D}' | '\u{2060}' | '\u{FEFF}'
            | '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}'
            | '\u{FE00}'..='\u{FE0F}' | '\u{00AD}'
    )
}

fn command_like(value: &str) -> bool {
    let start = value.trim_start();
    if start.starts_with("=/>") || start.starts_with('#') {
        return true;
    }
    let compact = start
        .chars()
        .take(20)
        .filter(|character| !character.is_whitespace())
        .collect::<String>();
    compact.starts_with("=/>") || compact.starts_with('#')
}

/// Classify a message from its transport-verified edit state and body.
/// Opaque encrypted edits MUST NOT be declared inspected: decryption requires
/// the original message secret from the provider's protected message store.
pub fn review_content(input: &str, edit: EditState) -> ContentReview {
    if edit == EditState::EditedEncryptedUnavailable {
        return ContentReview {
            inspection_text: None,
            edited: true,
            unreadable: true,
            hidden_or_confusable: false,
            command_camouflage: false,
            link_camouflage: false,
        };
    }

    let mut normalized = String::with_capacity(input.len());
    let mut changed = false;
    for ch in input.chars() {
        if invisible(ch) {
            changed = true;
            continue;
        }
        if let Some(mapped) = mapped(ch) {
            changed = true;
            normalized.push(mapped);
        } else {
            normalized.push(ch);
        }
    }
    let normalized = normalized.to_ascii_lowercase();
    let has_control = command_like(&normalized);
    let original_control = input.trim_start().starts_with("=/>")
        || input.trim_start().starts_with('#');
    let malformed_prefix = has_control && !original_control;
    let command_camouflage =
        malformed_prefix || (has_control && changed && normalized != input);
    let link_camouflage = changed && normalized.contains("://");
    ContentReview {
        inspection_text: Some(normalized),
        edited: edit == EditState::EditedReadable,
        unreadable: false,
        hidden_or_confusable: changed,
        command_camouflage,
        link_camouflage,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_plain_commands_are_not_rewritten_or_rejected() {
        let result = review_content("=/>menu anti", EditState::Original);
        assert_eq!(result.inspection_text.as_deref(), Some("=/>menu anti"));
        assert!(!result.command_camouflage);
        assert!(!result.edited);
    }

    #[test]
    fn edited_content_is_distinguished_from_original() {
        let result = review_content("https://t.me/chat", EditState::EditedReadable);
        assert!(result.edited);
        assert!(!result.unreadable);
        assert_eq!(result.inspection_text.as_deref(), Some("https://t.me/chat"));
    }

    #[test]
    fn opaque_edit_is_never_treated_as_readable() {
        let result = review_content("=/>bot on", EditState::EditedEncryptedUnavailable);
        assert!(result.unreadable);
        assert!(result.inspection_text.is_none());
    }

    #[test]
    fn disguised_command_is_detected_but_not_executed() {
        for text in [
            "＝／＞bot on",
            "=/​>bot on",
            "= / >bot off",
            "＃menu",
            "=/>mеnu",
        ] {
            let result = review_content(text, EditState::Original);
            assert!(result.command_camouflage, "{text}");
            assert!(result.inspection_text.is_some());
        }
    }

    #[test]
    fn zero_width_link_and_unicode_dot_are_visible_to_inspector() {
        let result = review_content("https://t․me/abc", EditState::Original);
        assert!(result.link_camouflage);
        assert_eq!(
            result.inspection_text.as_deref(),
            Some("https://t.me/abc")
        );
    }

    #[test]
    fn plain_text_does_not_raise_camouflage_flag() {
        let result = review_content("Hola, bienvenido", EditState::Original);
        assert!(!result.command_camouflage);
        assert!(!result.hidden_or_confusable);
    }
}
