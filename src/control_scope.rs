/*
 * Base-Matter — uniform command scope contract.
 *
 * Parsing a scope never grants permissions, authorizes a target user, or
 * initializes a global configuration backend. Every handler checks its own
 * capabilities and authenticated permissions before exposing or editing data.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Result};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommandScope {
    /// Exact authenticated chat, group or private conversation.
    Chat,
    /// The currently authenticated socket, never another user's socket.
    Socket,
    /// Shared ecosystem defaults. Requires a separately authenticated Vault
    /// service for persistent writes; reading a built-in fallback is not a
    /// claim that such a service is connected.
    Global,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ScopedArguments<'a> {
    pub scope: CommandScope,
    pub arguments: Vec<&'a str>,
}

/// Extract at most one scope flag. Accept it anywhere among command argument
/// tokens; never run this parser on free-form message, script or quoted data.
/// Unknown flags are intentionally returned untouched to the command handler.
pub fn parse_scope<'a>(arguments: &[&'a str]) -> Result<ScopedArguments<'a>> {
    let mut scope = CommandScope::Chat;
    let mut seen = false;
    let mut result = Vec::with_capacity(arguments.len());
    for &part in arguments {
        let selected = match part {
            "--mod" => Some(CommandScope::Socket),
            "--global" => Some(CommandScope::Global),
            _ => None,
        };
        if let Some(value) = selected {
            if seen {
                bail!("Solo se permite un ámbito: --mod o --global");
            }
            scope = value;
            seen = true;
        } else {
            result.push(part);
        }
    }
    Ok(ScopedArguments {
        scope,
        arguments: result,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UserTargetKind {
    /// The actual reply participant must be verified by the platform adapter.
    Reply,
    /// A mention must resolve to a provider-confirmed participant identity.
    Mention,
    /// A number is only a search hint, never proof of an identity or role.
    Number,
    /// A username must match an authenticated user directory record.
    User,
}

/// This is an intent, not an authenticated identity. User-targeted public
/// commands MUST resolve it against verified platform or account metadata and
/// enforce a public-data policy. Never use these strings for owner decisions.
#[derive(Debug, Eq, PartialEq)]
pub struct UnverifiedUserTarget<'a> {
    pub kind: UserTargetKind,
    pub value: Option<&'a str>,
}

pub fn parse_user_target<'a>(
    tokens: &[&'a str],
    has_verified_reply: bool,
) -> Result<UnverifiedUserTarget<'a>> {
    match tokens {
        [] if has_verified_reply => Ok(UnverifiedUserTarget {
            kind: UserTargetKind::Reply,
            value: None,
        }),
        ["user", identifier] if safe_identifier(identifier) => Ok(UnverifiedUserTarget {
            kind: UserTargetKind::User,
            value: Some(identifier),
        }),
        ["numero" | "number", phone]
            if phone.len() >= 8
                && phone.len() <= 15
                && phone.bytes().all(|c| c.is_ascii_digit()) =>
        {
            Ok(UnverifiedUserTarget {
                kind: UserTargetKind::Number,
                value: Some(phone),
            })
        }
        [mention] if mention.starts_with('@') && safe_identifier(&mention[1..]) => {
            Ok(UnverifiedUserTarget {
                kind: UserTargetKind::Mention,
                value: Some(mention),
            })
        }
        _ => bail!("Usuario no verificable. Responde a un mensaje o usa una mención, número o user válido"),
    }
}

fn safe_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|part| part.is_ascii_alphanumeric() || b"._:-".contains(&part))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope_flags_are_uniform_and_do_not_infer_authority() {
        assert_eq!(
            parse_scope(&["link"]).unwrap(),
            ScopedArguments {
                scope: CommandScope::Chat,
                arguments: vec!["link"]
            }
        );
        for value in [
            vec!["--mod", "link", "on"],
            vec!["link", "on", "--mod"],
        ] {
            let parsed = parse_scope(&value).unwrap();
            assert_eq!(parsed.scope, CommandScope::Socket);
            assert_eq!(parsed.arguments, vec!["link", "on"]);
        }
        let parsed = parse_scope(&["link", "--global"]).unwrap();
        assert_eq!(parsed.scope, CommandScope::Global);
        assert_eq!(parsed.arguments, vec!["link"]);
        for invalid in [
            vec!["--mod", "--global"],
            vec!["--global", "--global"],
            vec!["--mod", "--mod"],
        ] {
            assert!(parse_scope(&invalid).is_err());
        }
    }

    #[test]
    fn user_target_syntax_is_only_an_unverified_query_hint() {
        assert!(parse_user_target(&[], false).is_err());
        assert_eq!(
            parse_user_target(&[], true).unwrap().kind,
            UserTargetKind::Reply
        );
        assert_eq!(
            parse_user_target(&["@120123456"], false).unwrap().kind,
            UserTargetKind::Mention
        );
        assert_eq!(
            parse_user_target(&["numero", "5215512345678"], false).unwrap().kind,
            UserTargetKind::Number
        );
        assert_eq!(
            parse_user_target(&["user", "member-1"], false).unwrap().kind,
            UserTargetKind::User
        );
        assert!(parse_user_target(&["user", "name with spaces"], false).is_err());
        assert!(parse_user_target(&["@a/../../b"], false).is_err());
        assert!(parse_user_target(&["number", "55+1234"], false).is_err());
    }
}
