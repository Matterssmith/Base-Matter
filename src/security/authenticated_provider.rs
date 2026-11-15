
/*
 * Matter-Base — Authenticated Provider
 *
 * Integra autenticación criptográfica y autorización
 * para operaciones de proveedores externos.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use crate::security::{
    ProviderAccessPolicy,
    ProviderAuthenticator,
    ProviderIdentity,
    ProviderPermission,
};
use anyhow::Result;

/// Representa un proveedor cuya identidad fue autenticada
/// y cuyo permiso para una operación fue verificado.
pub struct AuthenticatedProvider {
    identity: ProviderIdentity,
    authorized_permission: ProviderPermission,
}

impl AuthenticatedProvider {
    /// Autentica al proveedor y comprueba el permiso solicitado.
    ///
    /// No devuelve una identidad si falla cualquiera de las
    /// dos verificaciones.
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

    /// Devuelve el permiso comprobado durante la autenticación.
    pub fn authorized_permission(&self) -> ProviderPermission {
        self.authorized_permission
    }

    /// Comprueba si la operación coincide con el permiso
    /// que fue autorizado al crear esta sesión.
    pub fn permits(&self, permission: ProviderPermission) -> bool {
        self.authorized_permission == permission
    }
}
