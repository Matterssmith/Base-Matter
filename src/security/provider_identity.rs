
/*
 * Matter-Base — Provider Identity
 *
 * Define identidades de proveedores y políticas de acceso
 * a material criptográfico.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::{bail, Result};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProviderPermission {
    ReadKeyIds,
    ReadActiveKey,
    ReadKeyMaterial,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderIdentity {
    provider_id: String,
    vault_id: String,
    permissions: HashSet<ProviderPermission>,
}

impl ProviderIdentity {
    /// Construye una identidad a partir de datos previamente
    /// verificados por un autenticador de confianza.
    ///
    /// Este constructor valida los datos, pero no autentica
    /// criptográficamente al proveedor.
    pub fn from_verified_assertion(
        provider_id: String,
        vault_id: String,
        permissions: HashSet<ProviderPermission>,
    ) -> Result<Self> {
        Self::validate_identifier(&provider_id)?;
        Self::validate_identifier(&vault_id)?;

        if permissions.is_empty() {
            bail!("Provider has no permissions");
        }

        Ok(Self {
            provider_id,
            vault_id,
            permissions,
        })
    }

    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }

    pub fn vault_id(&self) -> &str {
        &self.vault_id
    }

    pub fn has_permission(&self, permission: ProviderPermission) -> bool {
        self.permissions.contains(&permission)
    }

    fn validate_identifier(value: &str) -> Result<()> {
        if value.trim().is_empty()
            || value.len() > 128
            || value.chars().any(char::is_control)
        {
            bail!("Invalid provider identity");
        }

        Ok(())
    }
}

pub struct ProviderAccessPolicy {
    expected_vault_id: String,
    allowed_providers: HashSet<String>,
}

impl ProviderAccessPolicy {
    pub fn new(
        expected_vault_id: String,
        allowed_providers: HashSet<String>,
    ) -> Result<Self> {
        ProviderIdentity::validate_identifier(&expected_vault_id)?;

        if allowed_providers.is_empty() {
            bail!("No providers are authorized");
        }

        for provider_id in &allowed_providers {
            ProviderIdentity::validate_identifier(provider_id)?;
        }

        Ok(Self {
            expected_vault_id,
            allowed_providers,
        })
    }

    /// Comprueba que la identidad pertenece al Vault esperado
    /// y que el proveedor está autorizado.
    pub fn authorize_provider(
        &self,
        identity: &ProviderIdentity,
    ) -> Result<()> {
        if identity.vault_id != self.expected_vault_id {
            bail!("Provider Vault mismatch");
        }

        if !self.allowed_providers.contains(&identity.provider_id) {
            bail!("Provider is not authorized");
        }

        Ok(())
    }

    /// Comprueba el permiso específico para una operación.
    pub fn authorize(
        &self,
        identity: &ProviderIdentity,
        permission: ProviderPermission,
    ) -> Result<()> {
        self.authorize_provider(identity)?;

        if !identity.has_permission(permission) {
            bail!("Provider permission denied");
        }

        Ok(())
    }
}
