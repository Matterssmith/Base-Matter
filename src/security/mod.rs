
/*
 * Matter-Base — Security Module
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

pub mod encryption;
pub mod key_manager;
pub mod key_provider;
pub mod provider_identity;

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

pub use provider_identity::{
    ProviderAccessPolicy,
    ProviderIdentity,
    ProviderPermission,
};
