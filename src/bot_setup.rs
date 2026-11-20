/*
 * Base-Matter — neutral bot setup selection.
 * Provider selection is an opaque name, never a Git repository or file path.
 * No protected implementation, Vault key, or runtime permission is exposed.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

pub const SETUP_SCHEMA_VERSION: u16 = 1;
pub const DEFAULT_PROVIDER: &str = "default";

/// Public intent only. Values do not authorize any resource.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BotSetupRequest {
    pub provider: Option<String>,
    pub language: Option<String>,
    pub system: Option<String>,
}

/// Resolved public selector. The protected authority alone maps a provider
/// selector to an installed implementation, version and permitted operations.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BotSetupSelection {
    pub version: u16,
    pub provider: String,
    pub language: String,
    pub system: String,
}

impl BotSetupRequest {
    pub fn select(&self) -> Result<BotSetupSelection, SetupError> {
        let provider = self.provider.as_deref().unwrap_or(DEFAULT_PROVIDER);
        if !safe_provider_id(provider) {
            return Err(SetupError::InvalidProvider);
        }
        let language = match self.language.as_deref().unwrap_or("rust") {
            "rust" => "rust",
            "python" => "python",
            "cpp" | "c++" => "cpp",
            "c" => "c",
            "go" => "go",
            "typescript" | "ts" => "typescript",
            "javascript" | "js" => "javascript",
            _ => return Err(SetupError::UnsupportedLanguage),
        };
        let system = match self.system.as_deref().unwrap_or("host") {
            "host" => "host",
            "linux" => "linux",
            "windows" => "windows",
            "macos" => "macos",
            "android" => "android",
            _ => return Err(SetupError::UnsupportedSystem),
        };
        Ok(BotSetupSelection {
            version: SETUP_SCHEMA_VERSION,
            provider: provider.to_owned(),
            language: language.to_owned(),
            system: system.to_owned(),
        })
    }
}

fn safe_provider_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && !value.starts_with('-')
        && !value.ends_with('-')
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

/// Descriptive response only; never a bearer token or an executable grant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BotSetupResolution {
    pub selection: BotSetupSelection,
    pub capability_names: Vec<String>,
}

/// Implement ONLY on a trusted host with authenticated admission and a
/// separately authorized, revocable private capability registry.
/// Do not implement it from caller-supplied JSON or a public manifest.
pub trait ProtectedBotSetupCatalog: Send + Sync {
    fn resolve(&self, selection: &BotSetupSelection) -> Result<BotSetupResolution, SetupError>;
}

pub struct UnavailableBotSetupCatalog;
impl ProtectedBotSetupCatalog for UnavailableBotSetupCatalog {
    fn resolve(&self, _: &BotSetupSelection) -> Result<BotSetupResolution, SetupError> {
        Err(SetupError::Unavailable)
    }
}

/// Validates public intent and delegates all provider availability/permission
/// decisions to the trusted authority. No repository URLs are used.
pub fn resolve_setup(
    catalog: &dyn ProtectedBotSetupCatalog,
    request: &BotSetupRequest,
) -> Result<BotSetupResolution, SetupError> {
    let selection = request.select()?;
    let result = catalog.resolve(&selection)?;
    if result.selection != selection
        || result.capability_names.len() > 64
        || result
            .capability_names
            .iter()
            .any(|name| !safe_capability_name(name))
    {
        return Err(SetupError::InvalidResolution);
    }
    Ok(result)
}

fn safe_capability_name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 96
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"._-".contains(&byte)
        })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SetupError {
    InvalidProvider,
    UnsupportedLanguage,
    UnsupportedSystem,
    Unavailable,
    Denied,
    InvalidResolution,
}
impl fmt::Display for SetupError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        output.write_str(match self {
            Self::InvalidProvider => "Invalid provider selector",
            Self::UnsupportedLanguage => "Unsupported bot language",
            Self::UnsupportedSystem => "Unsupported target system",
            Self::Unavailable => "Protected bot setup catalog unavailable",
            Self::Denied => "Bot setup capability denied",
            Self::InvalidResolution => "Invalid protected catalog response",
        })
    }
}
impl Error for SetupError {}

#[cfg(test)]
mod tests {
    use super::*;

    struct Approved;
    impl ProtectedBotSetupCatalog for Approved {
        fn resolve(&self, selection: &BotSetupSelection) -> Result<BotSetupResolution, SetupError> {
            Ok(BotSetupResolution {
                selection: selection.clone(),
                capability_names: vec!["messaging.receive".into()],
            })
        }
    }

    #[test]
    fn defaults_are_neutral_and_have_no_access() {
        let selection = BotSetupRequest::default().select().unwrap();
        assert_eq!(selection.provider, "default");
        assert_eq!(selection.language, "rust");
        assert_eq!(selection.system, "host");
        assert!(matches!(
            resolve_setup(&UnavailableBotSetupCatalog, &BotSetupRequest::default()),
            Err(SetupError::Unavailable)
        ));
    }

    #[test]
    fn independent_target_selection() {
        let request = BotSetupRequest {
            provider: Some("family-example".into()),
            language: Some("go".into()),
            system: Some("windows".into()),
        };
        let resolution = resolve_setup(&Approved, &request).unwrap();
        assert_eq!(resolution.selection.provider, "family-example");
        assert_eq!(resolution.selection.language, "go");
        assert_eq!(resolution.selection.system, "windows");
        assert_eq!(resolution.capability_names, vec!["messaging.receive"]);
    }

    #[test]
    fn deny_repository_paths_and_code_injection() {
        for provider in [
            "../secrets",
            "https://example.com/owner/code",
            "my_repo",
            "--exec",
            "a b",
            "",
        ] {
            assert_eq!(
                BotSetupRequest {
                    provider: Some(provider.into()),
                    ..Default::default()
                }
                .select(),
                Err(SetupError::InvalidProvider)
            );
        }
    }

    #[test]
    fn language_and_system_are_explicit() {
        let request = BotSetupRequest {
            language: Some("ts".into()),
            system: Some("android".into()),
            ..Default::default()
        };
        let selection = request.select().unwrap();
        assert_eq!(selection.language, "typescript");
        assert_eq!(selection.system, "android");
        assert_eq!(
            BotSetupRequest {
                language: Some("shell".into()),
                ..Default::default()
            }
            .select(),
            Err(SetupError::UnsupportedLanguage)
        );
    }

    #[test]
    fn protected_resolution_must_match_the_requested_target() {
        struct Wrong;
        impl ProtectedBotSetupCatalog for Wrong {
            fn resolve(
                &self,
                selection: &BotSetupSelection,
            ) -> Result<BotSetupResolution, SetupError> {
                let mut changed = selection.clone();
                changed.provider = "other".into();
                Ok(BotSetupResolution {
                    selection: changed,
                    capability_names: vec![],
                })
            }
        }
        assert!(matches!(
            resolve_setup(&Wrong, &BotSetupRequest::default()),
            Err(SetupError::InvalidResolution)
        ));
    }
}

// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
