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

pub mod activation;
pub mod authorization;
pub mod bots;
pub mod command_audit;
pub mod commands;
pub mod connections;
pub mod control;
pub mod control_sync;
pub mod core;
pub mod events;
pub mod i18n;
pub mod legal;
pub mod modules;
pub mod plugins;
pub mod security;
pub mod services;
pub mod sessions;
pub mod users;

pub use authorization::{RoleAuditRecord, RoleCommandAuthorizer, RoleStore, RuleEffect};
pub use bots::{
    BotAction, BotAuditRecord, BotIdentity, BotIdentityStore, BotRegistry,
    InMemoryBotIdentityStore, OwnerRecoveryAuthorizer, SqliteBotIdentityStore,
};
pub use command_audit::{CommandAuditOutcome, CommandAuditRecord, CommandAuditStore};
pub use commands::{CommandAuthorizer, CommandInvocation, CommandRouter};
pub use connections::{
    ConnectionAdmission, ConnectionDriver, ConnectionService, SessionCredentialSource,
};
pub use control_sync::{EncryptedControlOutbox, EncryptedEnvelopeSink, OutboxStatus};
pub use core::config::CoreConfig;
pub use core::context::MatterContext;
pub use core::logger::init_logger;
pub use core::runtime::MatterRuntime;

pub use events::EventBus;
pub use i18n::Translator;

pub use legal::{
    ConsentDecision, ConsentRecord, LegalAction, LegalAuthorizationRequest, LegalAuthorizer,
    LegalDocument, LegalDocumentType, LegalPrincipal, LegalStore,
};

pub use modules::{CoreModule, ModuleManager};
pub use security::encryption;
pub use services::ServiceRegistry;
pub use sessions::{
    ConnectionState, ControlPlaneMessageAdmission, MessageAdmission, MessageService,
    OutboundMessage, SessionAuditRecord, SessionRecord, SessionRegistry, TransportAcceptance,
    TransportDriver,
};

pub use users::{
    ExternalIdentity, UserAction, UserAuthorizationRequest, UserAuthorizer, UserPrincipal,
    UserRegistry, UserStore,
};
