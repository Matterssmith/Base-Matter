/*
 * Matter-Base — Cross-module Flow Tests
 *
 * Verifica el uso conjunto de identidad, runtime, persistencia, eventos,
 * servicios y ciclo de vida de módulos.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::Result;
use matter_base::{
    BotRegistry, CoreConfig, CoreModule, MatterContext, MatterRuntime, ModuleManager,
};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("matter-base-flow-{}-{nonce}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

struct RuntimeService(&'static str);

struct RuntimeModule {
    callbacks: Arc<Mutex<Vec<&'static str>>>,
}

impl CoreModule for RuntimeModule {
    fn name(&self) -> &str {
        "flow-module"
    }

    fn initialize(&self, context: &MatterContext) -> Result<()> {
        context.services.register(RuntimeService("initialized"));
        self.callbacks.lock().unwrap().push("initialized");
        Ok(())
    }

    fn start(&self, context: &MatterContext) -> Result<()> {
        self.callbacks.lock().unwrap().push("started");
        context.events.publish_checked("runtime.started", "ok")?;
        Ok(())
    }

    fn stop(&self, _context: &MatterContext) -> Result<()> {
        self.callbacks.lock().unwrap().push("stopped");
        Ok(())
    }
}

#[test]
fn persistent_identity_runtime_service_events_and_module_lifecycle_work_together() {
    let directory = TemporaryDirectory::new();
    let registry_path = directory.0.join("bots.db");
    let registry = BotRegistry::open_sqlite(&registry_path).unwrap();
    let identity = registry
        .register("shared-family", "release-1", "matter")
        .unwrap();
    let runtime = MatterRuntime::open_with_bot_registry(
        CoreConfig::default(),
        directory.0.join("legal.db"),
        registry,
    )
    .unwrap();
    assert_eq!(
        runtime.context.bots.get(identity.instance_id()).unwrap(),
        Some(identity)
    );

    runtime.context.legal.register_user("flow-user").unwrap();
    let callbacks = Arc::new(Mutex::new(Vec::new()));
    let observed = Arc::new(Mutex::new(Vec::new()));
    let observed_event = Arc::clone(&observed);
    runtime
        .context
        .events
        .subscribe("runtime.started", move |payload| {
            observed_event.lock().unwrap().push(payload.to_owned());
        });

    let mut modules = ModuleManager::new();
    modules
        .register(Box::new(RuntimeModule {
            callbacks: Arc::clone(&callbacks),
        }))
        .unwrap();
    modules.initialize_all(runtime.context()).unwrap();
    modules.start_all(runtime.context()).unwrap();

    let service = runtime.context.services.get::<RuntimeService>().unwrap();
    assert_eq!(service.0, "initialized");
    assert_eq!(*observed.lock().unwrap(), ["ok"]);
    assert!(runtime.context.legal.user_exists("flow-user").unwrap());

    modules.stop_all(runtime.context()).unwrap();
    assert_eq!(
        *callbacks.lock().unwrap(),
        ["initialized", "started", "stopped"]
    );
}
