/*
 * Matter-Base — Plugin trust and command isolation regressions.
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::{bail, Result};
use matter_base::{
    plugins::{DenyPluginSandbox, PluginHost, PluginManifest, PluginSandbox, PluginTrust},
    CommandAuthorizer, CommandInvocation, CommandRouter,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

struct Permit;
impl CommandAuthorizer for Permit {
    fn authorize(&self, _invocation: &CommandInvocation) -> Result<()> {
        Ok(())
    }
}

struct Trust {
    manifest: bool,
    invoke: bool,
}
impl PluginTrust for Trust {
    fn approve_manifest(&self, _manifest: &PluginManifest) -> Result<()> {
        if self.manifest {
            Ok(())
        } else {
            bail!("private publisher details")
        }
    }
    fn authorize_execution(
        &self,
        _manifest: &PluginManifest,
        _invocation: &CommandInvocation,
    ) -> Result<()> {
        if self.invoke {
            Ok(())
        } else {
            bail!("private authorization details")
        }
    }
}

struct Sandbox(AtomicUsize);
impl PluginSandbox for Sandbox {
    fn execute(
        &self,
        _manifest: &PluginManifest,
        invocation: &CommandInvocation,
    ) -> Result<String> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(format!(
            "{}:{}",
            invocation.command,
            invocation.args.join(",")
        ))
    }
}

fn manifest() -> PluginManifest {
    PluginManifest {
        plugin_id: "plugin-1".into(),
        version: "1.0.0".into(),
        runtime: "rust".into(),
        entrypoint: "bin/runner".into(),
        command: "status".into(),
        aliases: vec!["st".into()],
        capabilities: vec!["message.read".into()],
    }
}

fn router() -> CommandRouter {
    CommandRouter::new(vec!["#".to_owned()], Arc::new(Permit)).unwrap()
}

#[test]
fn rejects_traversal_unknown_runtime_duplicate_caps_and_aliases() {
    let mut m = manifest();
    assert!(m.validate().is_ok());
    for path in ["../escape", "/absolute", "sub/../escape", "sub\\escape"] {
        m.entrypoint = path.into();
        assert!(m.validate().is_err());
    }
    m = manifest();
    m.runtime = "shell".into();
    assert!(m.validate().is_err());
    m = manifest();
    m.capabilities.push("message.read".into());
    assert!(m.validate().is_err());
    m = manifest();
    m.aliases.push("st".into());
    assert!(m.validate().is_err());
    assert!(PluginManifest::parse(&"x".repeat(16 * 1024 + 1)).is_err());
}

#[test]
fn manifest_and_execution_policy_are_both_required() {
    let calls = Arc::new(Sandbox(AtomicUsize::new(0)));
    let router = router();
    let untrusted = PluginHost::new(
        Arc::new(Trust {
            manifest: false,
            invoke: true,
        }),
        calls.clone(),
    );
    assert!(untrusted.install(&router, manifest()).is_err());
    let denied = PluginHost::new(
        Arc::new(Trust {
            manifest: true,
            invoke: false,
        }),
        calls.clone(),
    );
    denied.install(&router, manifest()).unwrap();
    assert_eq!(
        router
            .route("bot", "user", "room", "#st")
            .unwrap_err()
            .to_string(),
        "Command execution failed"
    );
    assert_eq!(calls.0.load(Ordering::SeqCst), 0);
}

#[test]
fn sandbox_adapter_runs_with_canonical_command_and_no_fallback() {
    let calls = Arc::new(Sandbox(AtomicUsize::new(0)));
    let router = router();
    PluginHost::new(
        Arc::new(Trust {
            manifest: true,
            invoke: true,
        }),
        calls.clone(),
    )
    .install(&router, manifest())
    .unwrap();
    assert_eq!(
        router.route("bot", "user", "room", "#st hello").unwrap(),
        Some("status:hello".into())
    );
    assert_eq!(calls.0.load(Ordering::SeqCst), 1);
    let router = router();
    PluginHost::new(
        Arc::new(Trust {
            manifest: true,
            invoke: true,
        }),
        Arc::new(DenyPluginSandbox),
    )
    .install(&router, manifest())
    .unwrap();
    assert!(router.route("bot", "user", "room", "#st").is_err());
}

#[test]
fn verified_manifests_load_but_invalid_batch_installs_none() {
    let mut random = [0_u8; 16];
    getrandom::fill(&mut random).unwrap();
    let dir = std::env::temp_dir().join(format!("matter-plugins-{}", hex::encode(random)));
    std::fs::create_dir(&dir).unwrap();
    std::fs::write(
        dir.join("a.json"),
        serde_json::to_string(&manifest()).unwrap(),
    )
    .unwrap();
    let mut duplicate = manifest();
    duplicate.plugin_id = "plugin-2".into();
    duplicate.command = "status-2".into();
    duplicate.aliases = vec!["st".into()];
    std::fs::write(
        dir.join("b.json"),
        serde_json::to_string(&duplicate).unwrap(),
    )
    .unwrap();
    let router = router();
    let host = PluginHost::new(
        Arc::new(Trust {
            manifest: true,
            invoke: true,
        }),
        Arc::new(DenyPluginSandbox),
    );
    assert!(host.load_manifests(&router, &dir).is_err());
    assert_eq!(router.route("bot", "user", "room", "#st").unwrap(), None);
    std::fs::remove_file(dir.join("b.json")).unwrap();
    assert_eq!(host.load_manifests(&router, &dir).unwrap(), 1);
    assert!(router.route("bot", "user", "room", "#st").is_err());
    let _ = std::fs::remove_dir_all(dir);
}
