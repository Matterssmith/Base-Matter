/*
 * Base-Matter — bot factory v2; intent only, no runtime grants.
 * Credits: Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::{error::Error, fmt};

pub const FACTORY_SCHEMA_VERSION: u16 = 2;
pub const DEFAULT_LANGUAGE: &str = "msel";
pub const DEFAULT_PLATFORM: &str = "social-matter";
pub const DEFAULT_PROVIDER: &str = "default";

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BotProjectRequest {
    pub language: Option<String>,
    pub system: Option<String>,
    pub platforms: Option<Vec<String>>,
    pub provider: Option<String>,
    #[serde(default)]
    pub adapters: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BotProjectSelection {
    pub version: u16,
    pub language: String,
    pub engine: String,
    pub system: String,
    pub platforms: Vec<String>,
    pub provider: String,
    pub adapters: BTreeMap<String, String>,
}

impl BotProjectRequest {
    pub fn select(&self) -> Result<BotProjectSelection, FactoryError> {
        let language = language_id(self.language.as_deref().unwrap_or(DEFAULT_LANGUAGE))
            .ok_or(FactoryError::UnsupportedLanguage)?;
        let engine = match language {
            "msel" => "matter-msel",
            "rust" => "matter-rust",
            _ => "external",
        };
        let system = system_id(self.system.as_deref().unwrap_or("host"))
            .ok_or(FactoryError::UnsupportedSystem)?;
        let inputs = self.platforms.clone().unwrap_or_else(|| vec![DEFAULT_PLATFORM.into()]);
        if inputs.is_empty() || inputs.len() > 12 {
            return Err(FactoryError::InvalidPlatforms);
        }
        let mut platforms = Vec::with_capacity(inputs.len());
        let mut seen = BTreeSet::new();
        for input in inputs {
            let platform = platform_id(&input).ok_or(FactoryError::InvalidPlatform)?;
            if !seen.insert(platform.clone()) {
                return Err(FactoryError::DuplicatePlatform);
            }
            platforms.push(platform);
        }
        let provider = safe_id(self.provider.as_deref().unwrap_or(DEFAULT_PROVIDER))
            .ok_or(FactoryError::InvalidProvider)?;
        if provider != DEFAULT_PROVIDER && platforms.len() > 1 {
            return Err(FactoryError::AmbiguousMultiProvider);
        }
        let mut overrides = BTreeMap::new();
        for (platform, adapter) in &self.adapters {
            let platform = platform_id(platform).ok_or(FactoryError::InvalidPlatform)?;
            let adapter = safe_id(adapter).ok_or(FactoryError::InvalidProvider)?;
            if !seen.contains(&platform) {
                return Err(FactoryError::AdapterOutsideSelection);
            }
            if overrides.insert(platform, adapter).is_some() {
                return Err(FactoryError::DuplicatePlatform);
            }
        }
        let adapters = platforms
            .iter()
            .map(|platform| (
                platform.clone(),
                overrides.get(platform).cloned().unwrap_or_else(|| provider.clone()),
            ))
            .collect();
        Ok(BotProjectSelection {
            version: FACTORY_SCHEMA_VERSION,
            language: language.into(),
            engine: engine.into(),
            system: system.into(),
            platforms,
            provider,
            adapters,
        })
    }
}

fn safe_id(value: &str) -> Option<String> {
    let v = value.to_ascii_lowercase();
    if v.is_empty() || v.len() > 64 || v.starts_with('-') || v.ends_with('-')
        || !v.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    {
        None
    } else {
        Some(v)
    }
}

fn platform_id(value: &str) -> Option<String> {
    let v = safe_id(value)?;
    Some(if v == "social" { DEFAULT_PLATFORM.into() } else { v })
}

fn language_id(value: &str) -> Option<&'static str> {
    match value.to_ascii_lowercase().as_str() {
        "msel" => Some("msel"),
        "rust" => Some("rust"),
        "python" => Some("python"),
        "cpp" | "c++" => Some("cpp"),
        "c" => Some("c"),
        "go" => Some("go"),
        "typescript" | "ts" => Some("typescript"),
        "javascript" | "js" => Some("javascript"),
        "lua" => Some("lua"),
        "kotlin" => Some("kotlin"),
        "swift" => Some("swift"),
        _ => None,
    }
}

fn system_id(value: &str) -> Option<&'static str> {
    match value.to_ascii_lowercase().as_str() {
        "host" => Some("host"),
        "linux" => Some("linux"),
        "windows" => Some("windows"),
        "macos" => Some("macos"),
        "android" => Some("android"),
        "ios" => Some("ios"),
        "web" => Some("web"),
        "embedded" => Some("embedded"),
        _ => None,
    }
}

fn safe_capability(value: &str) -> bool {
    !value.is_empty() && value.len() <= 96
        && value.bytes().all(|b| {
            b.is_ascii_lowercase() || b.is_ascii_digit() || b"._-".contains(&b)
        })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BotProjectResolution {
    pub selection: BotProjectSelection,
    pub capability_names: Vec<String>,
}

/// A trusted host, not input JSON, decides which providers and privileges exist.
pub trait ProtectedBotFactoryCatalog: Send + Sync {
    fn resolve(&self, selection: &BotProjectSelection) -> Result<BotProjectResolution, FactoryError>;
}

pub struct UnavailableBotFactoryCatalog;

impl ProtectedBotFactoryCatalog for UnavailableBotFactoryCatalog {
    fn resolve(&self, _: &BotProjectSelection) -> Result<BotProjectResolution, FactoryError> {
        Err(FactoryError::Unavailable)
    }
}

pub fn resolve_bot_project(
    catalog: &dyn ProtectedBotFactoryCatalog,
    request: &BotProjectRequest,
) -> Result<BotProjectResolution, FactoryError> {
    let selected = request.select()?;
    let result = catalog.resolve(&selected)?;
    if result.selection != selected || result.capability_names.len() > 64
        || result.capability_names.iter().any(|value| !safe_capability(value))
    {
        return Err(FactoryError::InvalidResolution);
    }
    Ok(result)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FactoryError {
    UnsupportedLanguage,
    UnsupportedSystem,
    InvalidPlatform,
    InvalidPlatforms,
    DuplicatePlatform,
    InvalidProvider,
    AdapterOutsideSelection,
    AmbiguousMultiProvider,
    Unavailable,
    Denied,
    InvalidResolution,
}
impl fmt::Display for FactoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::UnsupportedLanguage => "Unsupported bot language",
            Self::UnsupportedSystem => "Unsupported target system",
            Self::InvalidPlatform => "Invalid platform ID",
            Self::InvalidPlatforms => "Invalid platform selection count",
            Self::DuplicatePlatform => "Duplicate platform",
            Self::InvalidProvider => "Invalid provider/library ID",
            Self::AdapterOutsideSelection => "Adapter for unselected platform",
            Self::AmbiguousMultiProvider => "Use per-platform adapters for multi-platform bots",
            Self::Unavailable => "Protected bot catalog is unavailable",
            Self::Denied => "Protected host denied bot project",
            Self::InvalidResolution => "Invalid protected catalog response",
        })
    }
}
impl Error for FactoryError {}

#[cfg(test)]
mod tests {
    use super::*;

    struct Approved;
    impl ProtectedBotFactoryCatalog for Approved {
        fn resolve(&self, selected: &BotProjectSelection) -> Result<BotProjectResolution, FactoryError> {
            Ok(BotProjectResolution {
                selection: selected.clone(),
                capability_names: vec!["messaging.receive".into()],
            })
        }
    }

    #[test]
    fn defaults_to_msel_social_matter_without_permission() {
        let project = BotProjectRequest::default().select().unwrap();
        assert_eq!(project.version, 2);
        assert_eq!(project.language, "msel");
        assert_eq!(project.engine, "matter-msel");
        assert_eq!(project.platforms, vec!["social-matter"]);
        assert_eq!(project.adapters["social-matter"], "default");
        assert!(matches!(
            resolve_bot_project(&UnavailableBotFactoryCatalog, &BotProjectRequest::default()),
            Err(FactoryError::Unavailable)
        ));
    }

    #[test]
    fn choose_matter_rust_and_baileys() {
        let project = BotProjectRequest {
            language: Some("RuSt".into()),
            platforms: Some(vec!["whatsapp".into()]),
            provider: Some("Baileys-Rust".into()),
            ..Default::default()
        }.select().unwrap();
        assert_eq!(project.engine, "matter-rust");
        assert_eq!(project.adapters["whatsapp"], "baileys-rust");
    }

    #[test]
    fn choose_multiple_platforms_and_independent_adapters() {
        let mut adapters = BTreeMap::new();
        adapters.insert("whatsapp".into(), "XYZBailyes".into());
        adapters.insert("telegram".into(), "telegram-api".into());
        let request = BotProjectRequest {
            platforms: Some(vec!["whatsapp".into(), "telegram".into(), "discord".into(), "social".into()]),
            adapters,
            ..Default::default()
        };
        let project = request.select().unwrap();
        assert_eq!(project.platforms.len(), 4);
        assert_eq!(project.adapters["whatsapp"], "xyzbailyes");
        assert_eq!(project.adapters["telegram"], "telegram-api");
        assert_eq!(project.adapters["discord"], "default");
        assert_eq!(project.adapters["social-matter"], "default");
        assert_eq!(resolve_bot_project(&Approved, &request).unwrap().capability_names.len(), 1);
    }

    #[test]
    fn unsafe_adapter_names_are_rejected() {
        for bad in ["../secret", "https://host/repo", "a b", "foo_bar", "--exec", ""] {
            assert_eq!(BotProjectRequest {
                provider: Some(bad.into()), ..Default::default()
            }.select(), Err(FactoryError::InvalidProvider));
        }
    }

    #[test]
    fn duplicate_and_invalid_platforms_are_rejected() {
        assert_eq!(BotProjectRequest {
            platforms: Some(vec!["social".into(), "social-matter".into()]), ..Default::default()
        }.select(), Err(FactoryError::DuplicatePlatform));
        assert_eq!(BotProjectRequest {
            platforms: Some(vec![]), ..Default::default()
        }.select(), Err(FactoryError::InvalidPlatforms));
        assert_eq!(BotProjectRequest {
            platforms: Some(vec!["../wa".into()]), ..Default::default()
        }.select(), Err(FactoryError::InvalidPlatform));
    }

    #[test]
    fn wrong_adapter_bindings_are_rejected() {
        let mut adapters = BTreeMap::new();
        adapters.insert("discord".into(), "unknown".into());
        assert_eq!(BotProjectRequest {
            platforms: Some(vec!["whatsapp".into()]), adapters, ..Default::default()
        }.select(), Err(FactoryError::AdapterOutsideSelection));
        assert_eq!(BotProjectRequest {
            platforms: Some(vec!["whatsapp".into(), "telegram".into()]),
            provider: Some("baileys".into()), ..Default::default()
        }.select(), Err(FactoryError::AmbiguousMultiProvider));
    }

    #[test]
    fn protected_catalog_cannot_swap_selection() {
        struct Wrong;
        impl ProtectedBotFactoryCatalog for Wrong {
            fn resolve(&self, selection: &BotProjectSelection) -> Result<BotProjectResolution, FactoryError> {
                let mut changed = selection.clone();
                changed.language = "rust".into();
                Ok(BotProjectResolution { selection: changed, capability_names: vec![] })
            }
        }
        assert_eq!(resolve_bot_project(&Wrong, &BotProjectRequest::default()),
            Err(FactoryError::InvalidResolution));
    }

    #[test]
    fn schema_rejects_unknown_fields() {
        assert!(serde_json::from_str::<BotProjectRequest>(
            r#"{"language":"msel","root":true}"#
        ).is_err());
    }
}

// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
