/*!
 * Módulo: Matter Context
 *
 * Función:
 * Centralizar los servicios compartidos del Runtime.
 *
 * Responsabilidades:
 * - Proporcionar el bus de eventos.
 * - Administrar el registro de servicios.
 * - Exponer el administrador legal.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use crate::bots::BotRegistry;
use crate::events::EventBus;
use crate::legal::LegalStore;
use crate::services::ServiceRegistry;
use std::sync::Arc;

/// Contexto compartido por los componentes del Runtime.
#[derive(Clone)]
pub struct MatterContext {
    pub events: Arc<EventBus>,
    pub services: Arc<ServiceRegistry>,
    pub legal: Arc<LegalStore>,
    pub bots: Arc<BotRegistry>,
}

impl MatterContext {
    /// Crea el contexto con un almacén legal proporcionado.
    pub fn new(legal: LegalStore) -> Self {
        Self::with_bot_registry(legal, BotRegistry::in_memory())
    }

    /// Creates a context with an explicitly configured bot identity registry.
    pub fn with_bot_registry(legal: LegalStore, bots: BotRegistry) -> Self {
        Self {
            events: Arc::new(EventBus::new()),
            services: Arc::new(ServiceRegistry::new()),
            legal: Arc::new(legal),
            bots: Arc::new(bots),
        }
    }

    /// Comprueba si un usuario puede acceder a un alcance.
    pub fn is_access_allowed(
        &self,
        user_id: &str,
        scope: &str,
        locale: &str,
    ) -> anyhow::Result<bool> {
        self.legal.is_access_allowed(user_id, scope, locale)
    }

    /// Rechaza el acceso cuando faltan requisitos legales.
    pub fn ensure_access(&self, user_id: &str, scope: &str, locale: &str) -> anyhow::Result<()> {
        if !self.is_access_allowed(user_id, scope, locale)? {
            anyhow::bail!("Legal requirements not satisfied");
        }

        Ok(())
    }
}
