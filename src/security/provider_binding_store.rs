/*
 * Matter-Base — Provider Binding Store
 *
 * Define la recuperación controlada de claves de vínculo
 * asociadas a una identidad de proveedor y un Vault.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use crate::security::ProviderIdentity;
use anyhow::{bail, Result};
use std::sync::Arc;
use zeroize::Zeroizing;

/// Fuente de confianza para recuperar claves de vínculo.
///
/// La implementación debe obtener los secretos desde un gestor
/// externo seguro. No debe leerlos de archivos del repositorio,
/// variables públicas ni datos proporcionados por el solicitante.
pub trait ProviderBindingStore: Send + Sync {
    /// Recupera la clave asociada exactamente al proveedor y Vault.
    fn load_binding_key(&self, provider_id: &str, vault_id: &str) -> Result<Zeroizing<[u8; 32]>>;
}

/// Resuelve claves de vínculo para identidades previamente autenticadas.
pub struct ProviderBindingResolver {
    store: Arc<dyn ProviderBindingStore>,
}

impl ProviderBindingResolver {
    /// Construye un resolvedor con una fuente de confianza.
    ///
    /// La aplicación debe inyectar aquí una implementación controlada
    /// por su configuración de seguridad, nunca una fuente del usuario.
    pub fn new(store: Arc<dyn ProviderBindingStore>) -> Self {
        Self { store }
    }

    /// Recupera el secreto correspondiente a una identidad autenticada.
    ///
    /// Este método es interno al crate para impedir que los módulos
    /// externos soliciten secretos usando identificadores arbitrarios.
    pub(crate) fn resolve_for_identity(
        &self,
        identity: &ProviderIdentity,
    ) -> Result<Zeroizing<[u8; 32]>> {
        let provider_id = identity.provider_id();
        let vault_id = identity.vault_id();

        Self::validate_identifier(provider_id)?;
        Self::validate_identifier(vault_id)?;

        let binding_key = self
            .store
            .load_binding_key(provider_id, vault_id)
            .map_err(|_| anyhow::anyhow!("Provider binding key retrieval failed"))?;

        if binding_key.iter().all(|byte| *byte == 0) {
            bail!("Provider binding key is invalid");
        }

        Ok(binding_key)
    }

    fn validate_identifier(value: &str) -> Result<()> {
        if value.trim().is_empty() || value.len() > 128 || value.chars().any(char::is_control) {
            bail!("Invalid provider binding identifier");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::security::ProviderPermission;
    use std::collections::HashSet;

    struct TestBindingStore {
        expected_provider: String,
        expected_vault: String,
        key: [u8; 32],
    }

    impl ProviderBindingStore for TestBindingStore {
        fn load_binding_key(
            &self,
            provider_id: &str,
            vault_id: &str,
        ) -> Result<Zeroizing<[u8; 32]>> {
            if provider_id != self.expected_provider || vault_id != self.expected_vault {
                bail!("Provider binding not found");
            }

            Ok(Zeroizing::new(self.key))
        }
    }

    fn test_identity() -> ProviderIdentity {
        let permissions = HashSet::from([ProviderPermission::ReadKeyIds]);

        ProviderIdentity::from_verified_assertion(
            "matter-vault".to_owned(),
            "matterssmith-primary-vault".to_owned(),
            permissions,
        )
        .expect("test identity must be valid")
    }

    fn test_resolver(key: [u8; 32]) -> ProviderBindingResolver {
        let store = TestBindingStore {
            expected_provider: "matter-vault".to_owned(),
            expected_vault: "matterssmith-primary-vault".to_owned(),
            key,
        };

        ProviderBindingResolver::new(Arc::new(store))
    }

    #[test]
    fn resolves_key_for_matching_identity() {
        let resolver = test_resolver([9; 32]);
        let identity = test_identity();

        let key = resolver
            .resolve_for_identity(&identity)
            .expect("matching binding must resolve");

        assert_eq!(*key, [9; 32]);
    }

    #[test]
    fn rejects_unknown_provider_or_vault() {
        let resolver = test_resolver([9; 32]);

        let permissions = HashSet::from([ProviderPermission::ReadKeyIds]);

        let identity = ProviderIdentity::from_verified_assertion(
            "unknown-provider".to_owned(),
            "matterssmith-primary-vault".to_owned(),
            permissions,
        )
        .expect("test identity must be valid");

        assert!(resolver.resolve_for_identity(&identity).is_err());
    }

    #[test]
    fn rejects_zero_filled_binding_key() {
        let resolver = test_resolver([0; 32]);
        let identity = test_identity();

        assert!(resolver.resolve_for_identity(&identity).is_err());
    }
}
