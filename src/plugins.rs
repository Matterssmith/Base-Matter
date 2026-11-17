/*
 * Matter-Base — Trusted plugin manifest and sandbox contracts.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use crate::commands::{CommandInvocation, CommandRouter};
use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Component, Path};
use std::sync::Arc;

const MAX_MANIFEST_BYTES: usize = 16 * 1024;
const MAX_IDENTIFIER_BYTES: usize = 128;
const MAX_CAPABILITIES: usize = 32;
const MAX_PLUGIN_MANIFESTS: usize = 256;
const MAX_RESPONSE_BYTES: usize = 16 * 1024;

/// A declaration only. Never a trusted executable merely because it parsed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PluginManifest {
    pub plugin_id: String,
    pub version: String,
    pub runtime: String,
    pub entrypoint: String,
    pub command: String,
    pub aliases: Vec<String>,
    pub capabilities: Vec<String>,
}

impl PluginManifest {
    pub fn parse(json: &str) -> Result<Self> {
        if json.len() > MAX_MANIFEST_BYTES {
            bail!("Plugin manifest exceeds size limit");
        }
        let manifest: Self = serde_json::from_str(json)?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<()> {
        validate_identifier(&self.plugin_id)?;
        validate_identifier(&self.version)?;
        validate_identifier(&self.command)?;
        if !matches!(
            self.runtime.as_str(),
            "rust" | "cpp" | "go" | "python" | "node"
        ) {
            bail!("Unsupported plugin runtime");
        }
        let entry = Path::new(&self.entrypoint);
        if self.entrypoint.is_empty()
            || self.entrypoint.len() > MAX_IDENTIFIER_BYTES
            || entry.components().count() == 0
            || !entry
                .components()
                .all(|c| matches!(c, Component::Normal(_)))
            || self.entrypoint.contains('\\')
        {
            bail!("Plugin entrypoint must be relative and traversal-free");
        }
        if self.capabilities.len() > MAX_CAPABILITIES || self.aliases.len() > 64 {
            bail!("Plugin manifest capacity exceeded");
        }
        let mut capabilities = HashSet::new();
        for cap in &self.capabilities {
            validate_identifier(cap)?;
            if !capabilities.insert(cap) {
                bail!("Duplicate plugin capability");
            }
        }
        let mut aliases = HashSet::new();
        aliases.insert(&self.command);
        for alias in &self.aliases {
            validate_identifier(alias)?;
            if !aliases.insert(alias) {
                bail!("Duplicate plugin command or alias");
            }
        }
        Ok(())
    }
}

fn validate_identifier(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > MAX_IDENTIFIER_BYTES
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._:-".contains(&byte))
    {
        bail!("Invalid plugin identifier");
    }
    Ok(())
}

/// Host authentication, publisher validation and per-invocation capability
/// authorization are mandatory. No built-in trust-all implementation exists.
pub trait PluginTrust: Send + Sync {
    fn approve_manifest(&self, manifest: &PluginManifest) -> Result<()>;
    fn authorize_execution(
        &self,
        manifest: &PluginManifest,
        invocation: &CommandInvocation,
    ) -> Result<()>;
}

/// The application installs an independently isolated process/container/WASM
/// sandbox. An in-process closure does NOT provide isolation; Base provides
/// no fallback to executing untrusted code inside the host process.
pub trait PluginSandbox: Send + Sync {
    fn execute(&self, manifest: &PluginManifest, invocation: &CommandInvocation) -> Result<String>;
}

/// Default backend refuses execution until a real sandbox is installed.
pub struct DenyPluginSandbox;

impl PluginSandbox for DenyPluginSandbox {
    fn execute(
        &self,
        _manifest: &PluginManifest,
        _invocation: &CommandInvocation,
    ) -> Result<String> {
        bail!("Plugin sandbox not configured")
    }
}

/// Validates and registers command handlers without executing plugin code.
/// The host's authentication and sandbox decision are checked on EVERY call.
pub struct PluginHost {
    trust: Arc<dyn PluginTrust>,
    sandbox: Arc<dyn PluginSandbox>,
}

impl PluginHost {
    pub fn new(trust: Arc<dyn PluginTrust>, sandbox: Arc<dyn PluginSandbox>) -> Self {
        Self { trust, sandbox }
    }

    pub fn install(&self, router: &CommandRouter, manifest: PluginManifest) -> Result<()> {
        manifest.validate()?;
        self.trust
            .approve_manifest(&manifest)
            .map_err(|_| anyhow!("Plugin manifest not trusted"))?;
        let command = manifest.command.clone();
        let aliases = manifest.aliases.clone();
        let trust = Arc::clone(&self.trust);
        let sandbox = Arc::clone(&self.sandbox);
        router.register(&command, &aliases, move |invocation| {
            trust
                .authorize_execution(&manifest, invocation)
                .map_err(|_| anyhow!("Plugin execution denied"))?;
            let result = catch_unwind(AssertUnwindSafe(|| sandbox.execute(&manifest, invocation)))
                .map_err(|_| anyhow!("Plugin sandbox execution failed"))?
                .map_err(|_| anyhow!("Plugin sandbox execution failed"))?;
            if result.len() > MAX_RESPONSE_BYTES {
                bail!("Plugin response exceeds size limit");
            }
            Ok(result)
        })
    }

    /// Reads only regular JSON files. Pre-validates and authenticates every
    /// manifest before any command registration. Router registration is not
    /// transactional: the host must coordinate concurrent installations.
    pub fn load_manifests(&self, router: &CommandRouter, directory: &Path) -> Result<usize> {
        let mut manifests = Vec::new();
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            if entry
                .path()
                .extension()
                .and_then(|extension| extension.to_str())
                != Some("json")
            {
                continue;
            }
            if !entry.file_type()?.is_file() {
                bail!("Non-regular plugin manifest file");
            }
            if manifests.len() >= MAX_PLUGIN_MANIFESTS {
                bail!("Plugin manifest limit reached");
            }
            if entry.metadata()?.len() > MAX_MANIFEST_BYTES as u64 {
                bail!("Plugin manifest exceeds size limit");
            }
            let json = fs::read_to_string(entry.path())?;
            let manifest = PluginManifest::parse(&json)?;
            self.trust
                .approve_manifest(&manifest)
                .map_err(|_| anyhow!("Plugin manifest not trusted"))?;
            manifests.push(manifest);
        }
        manifests.sort_by(|a, b| a.plugin_id.cmp(&b.plugin_id));
        let mut commands = HashSet::new();
        let mut ids = HashSet::new();
        for manifest in &manifests {
            if !ids.insert(manifest.plugin_id.clone()) {
                bail!("Duplicate plugin ID");
            }
            for name in std::iter::once(&manifest.command).chain(manifest.aliases.iter()) {
                let canonical = name.to_ascii_lowercase();
                if !commands.insert(canonical) {
                    bail!("Duplicate plugin command across manifests");
                }
            }
        }
        let count = manifests.len();
        for manifest in manifests {
            self.install(router, manifest)?;
        }
        Ok(count)
    }
}
