use crate::core::context::MatterContext;
use anyhow::{anyhow, bail, Context, Result};

/// Contract for a runtime module.
pub trait CoreModule: Send + Sync {
    fn name(&self) -> &str;

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn initialize(&self, context: &MatterContext) -> Result<()>;

    fn start(&self, _context: &MatterContext) -> Result<()> {
        Ok(())
    }

    fn stop(&self, _context: &MatterContext) -> Result<()> {
        Ok(())
    }
}

struct ModuleEntry {
    module: Box<dyn CoreModule>,
    initialized: bool,
    started: bool,
    cleanup_required: bool,
}

/// Coordinates module initialization and lifecycle callbacks in registration
/// order, with reverse-order shutdown and retryable cleanup after failures.
#[derive(Default)]
pub struct ModuleManager {
    modules: Vec<ModuleEntry>,
}

impl ModuleManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a module before initialization begins.
    pub fn register(&mut self, module: Box<dyn CoreModule>) -> Result<()> {
        let name = module.name();
        if name.trim().is_empty() || name.len() > 128 || name.chars().any(char::is_control) {
            bail!("Invalid module name");
        }
        if module.version().trim().is_empty()
            || module.version().len() > 64
            || module.version().chars().any(char::is_control)
        {
            bail!("Invalid module version");
        }
        if self.modules.iter().any(|entry| entry.module.name() == name) {
            bail!("Module name is already registered");
        }
        if self
            .modules
            .iter()
            .any(|entry| entry.initialized || entry.started || entry.cleanup_required)
        {
            bail!("Modules cannot be registered after initialization begins");
        }

        self.modules.push(ModuleEntry {
            module,
            initialized: false,
            started: false,
            cleanup_required: false,
        });
        Ok(())
    }

    /// Initializes every registered module in registration order.
    ///
    /// A failing module receives `stop` for partial cleanup. Previously
    /// initialized modules remain initialized and can be retried or stopped.
    pub fn initialize_all(&mut self, context: &MatterContext) -> Result<()> {
        for entry in &mut self.modules {
            if entry.initialized {
                continue;
            }
            if entry.started || entry.cleanup_required {
                bail!("Module cleanup must succeed before initialization");
            }

            if let Err(error) = entry.module.initialize(context) {
                let name = entry.module.name().to_owned();
                return match entry.module.stop(context) {
                    Ok(()) => Err(error).with_context(|| format!("Initializing module {name}")),
                    Err(cleanup) => {
                        entry.cleanup_required = true;
                        Err(anyhow!(
                            "Initializing module {name} failed: {error:#}; cleanup failed: {cleanup:#}"
                        ))
                    }
                };
            }
            entry.initialized = true;
        }
        Ok(())
    }

    /// Starts initialized modules in registration order.
    ///
    /// If a start fails, the failing module and all modules started by this
    /// call are stopped in reverse order. Failed cleanup remains marked as
    /// started so `stop_all` can retry it.
    pub fn start_all(&mut self, context: &MatterContext) -> Result<()> {
        if self.modules.iter().any(|entry| !entry.initialized) {
            bail!("All modules must initialize before startup");
        }

        let mut started_this_call: Vec<usize> = Vec::new();
        for index in 0..self.modules.len() {
            if self.modules[index].started {
                continue;
            }

            if let Err(error) = self.modules[index].module.start(context) {
                let name = self.modules[index].module.name().to_owned();
                let mut cleanup_errors = Vec::new();
                if let Err(cleanup) = self.modules[index].module.stop(context) {
                    self.modules[index].cleanup_required = true;
                    cleanup_errors.push(format!("{name}: {cleanup:#}"));
                } else {
                    self.modules[index].initialized = false;
                }
                for started_index in started_this_call.into_iter().rev() {
                    let entry = &mut self.modules[started_index];
                    if let Err(cleanup) = entry.module.stop(context) {
                        cleanup_errors.push(format!("{}: {cleanup:#}", entry.module.name()));
                    } else {
                        entry.initialized = false;
                        entry.started = false;
                        entry.cleanup_required = false;
                    }
                }

                if cleanup_errors.is_empty() {
                    return Err(error).with_context(|| format!("Starting module {name}"));
                }
                return Err(anyhow!(
                    "Starting module {name} failed: {error:#}; cleanup failures: {}",
                    cleanup_errors.join("; ")
                ));
            }

            self.modules[index].started = true;
            started_this_call.push(index);
        }
        Ok(())
    }

    /// Stops initialized or started modules in reverse registration order.
    ///
    /// A successful stop clears lifecycle state. Modules whose stop callback
    /// fails retain cleanup state so callers can retry it.
    pub fn stop_all(&mut self, context: &MatterContext) -> Result<()> {
        let mut failures = Vec::new();
        for entry in self.modules.iter_mut().rev() {
            if !entry.initialized && !entry.started && !entry.cleanup_required {
                continue;
            }
            match entry.module.stop(context) {
                Ok(()) => {
                    entry.initialized = false;
                    entry.started = false;
                    entry.cleanup_required = false;
                }
                Err(error) => {
                    entry.cleanup_required = true;
                    failures.push(format!("{}: {error:#}", entry.module.name()));
                }
            }
        }

        if failures.is_empty() {
            Ok(())
        } else {
            Err(anyhow!("Could not stop module(s): {}", failures.join("; ")))
        }
    }

    pub fn module_names(&self) -> Vec<&str> {
        self.modules
            .iter()
            .map(|entry| entry.module.name())
            .collect()
    }

    pub fn is_initialized(&self, name: &str) -> bool {
        self.modules
            .iter()
            .find(|entry| entry.module.name() == name)
            .is_some_and(|entry| entry.initialized)
    }

    pub fn is_started(&self, name: &str) -> bool {
        self.modules
            .iter()
            .find(|entry| entry.module.name() == name)
            .is_some_and(|entry| entry.started)
    }
}

#[cfg(test)]
mod tests {
    use super::{CoreModule, ModuleManager};
    use crate::core::context::MatterContext;
    use crate::legal::LegalStore;
    use anyhow::{bail, Result};
    use std::sync::{Arc, Mutex};

    struct TestModule {
        name: &'static str,
        calls: Arc<Mutex<Vec<String>>>,
        fail_initialize: bool,
        fail_start: bool,
        fail_stop: bool,
    }

    impl TestModule {
        fn record(&self, action: &str) {
            self.calls
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .push(format!("{}:{action}", self.name));
        }
    }

    impl CoreModule for TestModule {
        fn name(&self) -> &str {
            self.name
        }

        fn initialize(&self, _context: &MatterContext) -> Result<()> {
            self.record("initialize");
            if self.fail_initialize {
                bail!("initialize failed");
            }
            Ok(())
        }

        fn start(&self, _context: &MatterContext) -> Result<()> {
            self.record("start");
            if self.fail_start {
                bail!("start failed");
            }
            Ok(())
        }

        fn stop(&self, _context: &MatterContext) -> Result<()> {
            self.record("stop");
            if self.fail_stop {
                bail!("stop failed");
            }
            Ok(())
        }
    }

    fn module(name: &'static str, calls: &Arc<Mutex<Vec<String>>>) -> Box<dyn CoreModule> {
        module_with_flags(name, calls, false, false, false)
    }

    fn module_with_flags(
        name: &'static str,
        calls: &Arc<Mutex<Vec<String>>>,
        fail_initialize: bool,
        fail_start: bool,
        fail_stop: bool,
    ) -> Box<dyn CoreModule> {
        Box::new(TestModule {
            name,
            calls: Arc::clone(calls),
            fail_initialize,
            fail_start,
            fail_stop,
        })
    }

    fn context() -> MatterContext {
        MatterContext::new(LegalStore::open_in_memory().unwrap())
    }

    #[test]
    fn initializes_starts_and_stops_in_defined_order() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let mut manager = ModuleManager::new();
        manager.register(module("first", &calls)).unwrap();
        manager.register(module("second", &calls)).unwrap();
        let context = context();

        manager.initialize_all(&context).unwrap();
        manager.start_all(&context).unwrap();
        manager.stop_all(&context).unwrap();

        assert_eq!(
            *calls.lock().unwrap(),
            [
                "first:initialize",
                "second:initialize",
                "first:start",
                "second:start",
                "second:stop",
                "first:stop"
            ]
        );
        assert!(!manager.is_started("first"));
        assert!(!manager.is_started("second"));
    }

    #[test]
    fn failed_start_rolls_back_started_modules_in_reverse_order() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let mut manager = ModuleManager::new();
        manager.register(module("first", &calls)).unwrap();
        manager
            .register(module_with_flags("second", &calls, false, true, false))
            .unwrap();
        let context = context();
        manager.initialize_all(&context).unwrap();

        assert!(manager.start_all(&context).is_err());
        assert_eq!(
            *calls.lock().unwrap(),
            [
                "first:initialize",
                "second:initialize",
                "first:start",
                "second:start",
                "second:stop",
                "first:stop"
            ]
        );
        assert!(!manager.is_started("first"));
        assert!(!manager.is_started("second"));
        assert!(!manager.is_initialized("first"));
        assert!(!manager.is_initialized("second"));
    }

    #[test]
    fn stop_cleans_initialized_modules_even_before_start() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let mut manager = ModuleManager::new();
        manager.register(module("first", &calls)).unwrap();
        let context = context();
        manager.initialize_all(&context).unwrap();
        manager.stop_all(&context).unwrap();

        assert_eq!(*calls.lock().unwrap(), ["first:initialize", "first:stop"]);
        assert!(!manager.is_initialized("first"));
    }

    #[test]
    fn start_requires_all_modules_to_be_initialized() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let mut manager = ModuleManager::new();
        manager.register(module("first", &calls)).unwrap();

        assert!(manager.start_all(&context()).is_err());
        assert!(calls.lock().unwrap().is_empty());
    }

    #[test]
    fn duplicate_module_names_are_rejected() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let mut manager = ModuleManager::new();
        manager.register(module("same", &calls)).unwrap();
        assert!(manager.register(module("same", &calls)).is_err());
        assert_eq!(manager.module_names(), ["same"]);
    }
}
