
/*
 * Matter-Base — Security Module
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

pub mod authenticated_provider;
pub mod encryption;
pub mod key_manager;
pub mod key_provider;
pub mod provider_authenticator;
pub mod provider_identity;

pub use authenticated_provider::AuthenticatedProvider;

pub use encryption::{
    decrypt,
    decrypt_with_manager,
    encrypt,
    encrypt_with_manager,
    validate_envelope,
    EncryptedEnvelope,
};

pub use key_manager::KeyManager;
pub use key_provider::KeyProvider;

pub use provider_authenticator::{
    ProviderAssertion,
    ProviderAuthenticator,
};

pub use provider_identity::{
    ProviderAccessPolicy,
    ProviderIdentity,
    ProviderPermission,
};
