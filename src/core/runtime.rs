/*!
 * Módulo: Matter Runtime
 *
 * Función:
 * Inicializar y coordinar la infraestructura de Matter-Base.
 *
 * Responsabilidades:
 * - Crear el contexto compartido.
 * - Inicializar el almacén legal persistente.
 * - Proporcionar acceso a la configuración y los servicios.
 * - Centralizar las comprobaciones de autorización.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use crate::bots::BotRegistry;
use crate::core::config::CoreConfig;
use crate::core::context::MatterContext;
use crate::legal::{LegalPrincipal, LegalStore};
use crate::modules::{CoreModule, ModuleManager};
use anyhow::Result;
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct MatterRuntime {
    pub config: CoreConfig,
    pub context: MatterContext,
    modules: Arc<Mutex<ModuleManager>>,
}

impl MatterRuntime {
    /// Inicializa el Runtime usando el registro legal persistente.
    ///
    /// Ruta predeterminada: data/legal/registry.db
    pub fn new(config: CoreConfig) -> Result<Self> {
        Self::open(config, "data/legal/registry.db")
    }

    /// Inicializa el Runtime usando una ruta legal personalizada.
    pub fn open(config: CoreConfig, legal_database: impl AsRef<Path>) -> Result<Self> {
        let legal = LegalStore::open(legal_database)?;
        Ok(Self {
            config,
            context: MatterContext::new(legal),
            modules: Arc::new(Mutex::new(ModuleManager::new())),
        })
    }

    /// Opens the runtime with a caller-configured bot identity registry.
    ///
    /// Use `BotRegistry::open_sqlite` when identities must survive restarts.
    pub fn open_with_bot_registry(
        config: CoreConfig,
        legal_database: impl AsRef<Path>,
        bots: BotRegistry,
    ) -> Result<Self> {
        let legal = LegalStore::open(legal_database)?;
        Ok(Self {
            config,
            context: MatterContext::with_bot_registry(legal, bots),
            modules: Arc::new(Mutex::new(ModuleManager::new())),
        })
    }

    /// Construye el Runtime con un almacén legal existente.
    ///
    /// Útil para pruebas y para integraciones que administran
    /// explícitamente su almacenamiento.
    pub fn with_legal_store(config: CoreConfig, legal: LegalStore) -> Self {
        Self {
            config,
            context: MatterContext::new(legal),
            modules: Arc::new(Mutex::new(ModuleManager::new())),
        }
    }

    /// Builds the runtime with explicit legal and bot identity stores.
    pub fn with_stores(config: CoreConfig, legal: LegalStore, bots: BotRegistry) -> Self {
        Self {
            config,
            context: MatterContext::with_bot_registry(legal, bots),
            modules: Arc::new(Mutex::new(ModuleManager::new())),
        }
    }

    /// Registers a runtime module before initialization starts.
    pub fn register_module(&self, module: Box<dyn CoreModule>) -> Result<()> {
        self.modules
            .lock()
            .map_err(|_| anyhow::anyhow!("Runtime module manager lock poisoned"))?
            .register(module)
    }

    /// Initializes registered modules and the runtime context.
    pub fn initialize(&self) -> Result<()> {
        self.modules
            .lock()
            .map_err(|_| anyhow::anyhow!("Runtime module manager lock poisoned"))?
            .initialize_all(&self.context)?;
        tracing::info!(
            application = %self.config.name,
            environment = %self.config.environment,
            "Matter runtime initialized"
        );
        Ok(())
    }

    /// Initializes modules if needed, then starts them in registration order.
    pub fn start(&self) -> Result<()> {
        let mut modules = self
            .modules
            .lock()
            .map_err(|_| anyhow::anyhow!("Runtime module manager lock poisoned"))?;
        modules.initialize_all(&self.context)?;
        modules.start_all(&self.context)
    }

    /// Stops runtime modules in reverse registration order.
    pub fn shutdown(&self) -> Result<()> {
        self.modules
            .lock()
            .map_err(|_| anyhow::anyhow!("Runtime module manager lock poisoned"))?
            .stop_all(&self.context)
    }

    pub fn context(&self) -> &MatterContext {
        &self.context
    }

    pub fn config(&self) -> &CoreConfig {
        &self.config
    }

    pub fn is_access_allowed(
        &self,
        user_id: &str,
        scope: &str,
        locale: &str,
        principal: &LegalPrincipal<'_>,
    ) -> Result<bool> {
        self.context
            .is_access_allowed(user_id, scope, locale, principal)
    }

    pub fn ensure_access(
        &self,
        user_id: &str,
        scope: &str,
        locale: &str,
        principal: &LegalPrincipal<'_>,
    ) -> Result<()> {
        self.context
            .ensure_access(user_id, scope, locale, principal)
    }
}
