
/*
 * Matter-Base — Authenticated Provider
 *
 * Integra autenticación, autorización y acceso protegido
 * al administrador de claves.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use crate::security::{
    KeyManager,
    ProviderAccessPolicy,
    ProviderAuthenticator,
    ProviderIdentity,
    ProviderPermission,
};
use anyhow::{bail, Result};

/// Proveedor autenticado y autorizado para una operación.
pub struct AuthenticatedProvider {
    identity: ProviderIdentity,
    authorized_permission: ProviderPermission,
}

impl AuthenticatedProvider {
    /// Autentica al proveedor y autoriza el permiso solicitado.
    pub fn authenticate_and_authorize(
        authenticator: &mut ProviderAuthenticator,
        policy: &ProviderAccessPolicy,
        nonce: &[u8; 24],
        ciphertext: &[u8],
        key: &[u8; 32],
        required_permission: ProviderPermission,
    ) -> Result<Self> {
        let identity = authenticator.authenticate(
            nonce,
            ciphertext,
            key,
        )?;

        policy.authorize(&identity, required_permission)?;

        Ok(Self {
            identity,
            authorized_permission: required_permission,
        })
    }

    /// Comprueba que la sesión fue autorizada para el permiso.
    fn require_permission(
        &self,
        permission: ProviderPermission,
    ) -> Result<()> {
        if self.authorized_permission != permission
            || !self.identity.has_permission(permission)
        {
            bail!("Provider operation is not authorized");
        }

        Ok(())
    }

    /// Comprueba permisos adicionales de la identidad.
    fn require_identity_permission(
        &self,
        permission: ProviderPermission,
    ) -> Result<()> {
        if !self.identity.has_permission(permission) {
            bail!("Provider identity lacks required permission");
        }

        Ok(())
    }

    /// Devuelve el identificador autenticado del proveedor.
    pub fn provider_id(&self) -> &str {
        self.identity.provider_id()
    }

    /// Devuelve el identificador del Vault autenticado.
    pub fn vault_id(&self) -> &str {
        self.identity.vault_id()
    }

    /// Devuelve la identidad autenticada.
    pub fn identity(&self) -> &ProviderIdentity {
        &self.identity
    }

    /// Devuelve el permiso autorizado para esta sesión.
    pub fn authorized_permission(&self) -> ProviderPermission {
        self.authorized_permission
    }

    /// Comprueba si la sesión permite el permiso solicitado.
    pub fn permits(&self, permission: ProviderPermission) -> bool {
        self.authorized_permission == permission
            && self.identity.has_permission(permission)
    }

    /// Consulta el identificador de la clave activa.
    pub fn read_active_key_id<'a>(
        &self,
        manager: &'a KeyManager,
    ) -> Result<&'a str> {
        self.require_permission(ProviderPermission::ReadActiveKey)?;

        let (key_id, _) = manager.get_active()?;

        Ok(key_id)
    }

    /// Recupera una clave por su identificador.
    pub fn read_key<'a>(
        &self,
        manager: &'a KeyManager,
        key_id: &str,
    ) -> Result<&'a [u8; 32]> {
        self.require_permission(ProviderPermission::ReadKeyMaterial)?;

        manager.get(key_id)
    }

    /// Recupera la clave activa.
    ///
    /// Exige autorización para leer el material criptográfico
    /// y permiso de identidad para consultar la clave activa.
    pub fn read_active_key<'a>(
        &self,
        manager: &'a KeyManager,
    ) -> Result<(&'a str, &'a [u8; 32])> {
        self.require_permission(ProviderPermission::ReadKeyMaterial)?;
        self.require_identity_permission(ProviderPermission::ReadActiveKey)?;

        manager.get_active()
    }
}
