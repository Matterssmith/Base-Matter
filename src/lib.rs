/*!
 * Matter-Base
 *
 * Función:
 * Biblioteca central de infraestructura universal para bots.
 *
 * Responsabilidades:
 * - Exponer los módulos públicos del núcleo.
 * - Proporcionar interfaces independientes del lenguaje y la plataforma.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

pub mod bots;
pub mod core;
pub mod events;
pub mod i18n;
pub mod legal;
pub mod modules;
pub mod security;
pub mod services;

pub use bots::{
    BotIdentity, BotIdentityStore, BotRegistry, InMemoryBotIdentityStore, SqliteBotIdentityStore,
};
pub use core::config::CoreConfig;
pub use core::context::MatterContext;
pub use core::logger::init_logger;
pub use core::runtime::MatterRuntime;

pub use events::EventBus;
pub use i18n::Translator;

pub use legal::{ConsentDecision, ConsentRecord, LegalDocument, LegalDocumentType, LegalStore};

pub use modules::{CoreModule, ModuleManager};
pub use security::encryption;
pub use services::ServiceRegistry;
