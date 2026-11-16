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
use crate::legal::LegalStore;
use anyhow::Result;
use std::path::Path;

#[derive(Clone)]
pub struct MatterRuntime {
    pub config: CoreConfig,
    pub context: MatterContext,
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
        }
    }

    /// Builds the runtime with explicit legal and bot identity stores.
    pub fn with_stores(config: CoreConfig, legal: LegalStore, bots: BotRegistry) -> Self {
        Self {
            config,
            context: MatterContext::with_bot_registry(legal, bots),
        }
    }

    pub fn initialize(&self) {
        tracing::info!(
            application = %self.config.name,
            environment = %self.config.environment,
            "Matter runtime initialized"
        );
    }

    pub fn context(&self) -> &MatterContext {
        &self.context
    }

    pub fn config(&self) -> &CoreConfig {
        &self.config
    }

    pub fn is_access_allowed(&self, user_id: &str, scope: &str, locale: &str) -> Result<bool> {
        self.context.is_access_allowed(user_id, scope, locale)
    }

    pub fn ensure_access(&self, user_id: &str, scope: &str, locale: &str) -> Result<()> {
        self.context.ensure_access(user_id, scope, locale)
    }
}
