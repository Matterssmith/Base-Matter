/*
 * Matter-Base — Authenticated Provider
 *
 * Integra autenticación, autorización y vinculación
 * criptográfica de proveedores de claves.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use crate::security::{
    derive_provider_binding_key,
    BoundKeyProvider,
    KeyManager,
    ProviderAccessPolicy,
    ProviderAuthenticator,
    ProviderIdentity,
    ProviderPermission,
};

use anyhow::{anyhow, bail, Result};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use subtle::ConstantTimeEq;
use zeroize::Zeroizing;

type HmacSha256 = Hmac<Sha256>;

/// Proveedor autenticado y autorizado para operaciones protegidas.
pub struct AuthenticatedProvider {
    identity: ProviderIdentity,
    authorized_permission: ProviderPermission,
    binding_key: Zeroizing<[u8; 32]>,
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

        let binding_key = derive_provider_binding_key(key)?;

        Ok(Self {
            identity,
            authorized_permission: required_permission,
            binding_key,
        })
    }

    /// Comprueba el permiso autorizado para la sesión.
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

    /// Comprueba un permiso declarado por la identidad autenticada.
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
    pub fn read_active_key<'a>(
        &self,
        manager: &'a KeyManager,
    ) -> Result<(&'a str, &'a [u8; 32])> {
        self.require_permission(ProviderPermission::ReadKeyMaterial)?;
        self.require_identity_permission(ProviderPermission::ReadActiveKey)?;

        manager.get_active()
    }

    /// Carga claves después de demostrar la posesión del secreto.
    ///
    /// La prueba se vincula al proveedor, al Vault y a un
    /// desafío aleatorio nuevo para cada operación.
    pub fn load_key_manager(
        &self,
        manager: &mut KeyManager,
        provider: &dyn BoundKeyProvider,
    ) -> Result<()> {
        self.require_permission(ProviderPermission::ReadKeyIds)?;
        self.require_identity_permission(ProviderPermission::ReadActiveKey)?;
        self.require_identity_permission(ProviderPermission::ReadKeyMaterial)?;

        let provider_vault_id = provider.vault_id()?;

        if provider_vault_id != self.identity.vault_id() {
            bail!("Provider Vault does not match authenticated Vault");
        }

        let mut challenge = [0u8; 32];

        getrandom::fill(&mut challenge)
            .map_err(|_| anyhow!("Could not generate provider challenge"))?;

        let message = Self::binding_message(
            &challenge,
            self.identity.provider_id(),
            self.identity.vault_id(),
        )?;

        let received_proof = provider.binding_proof(&message)?;

        let expected_proof = Self::calculate_binding_proof(
            &self.binding_key,
            &message,
        )?;

        if !bool::from(received_proof.ct_eq(&expected_proof)) {
            bail!("Provider cryptographic binding failed");
        }

        manager.load_from_provider(provider)?;

        Ok(())
    }

    /// Construye un mensaje de vinculación sin ambigüedad.
    fn binding_message(
        challenge: &[u8; 32],
        provider_id: &str,
        vault_id: &str,
    ) -> Result<Vec<u8>> {
        let provider_len = u32::try_from(provider_id.len())
            .map_err(|_| anyhow!("Invalid provider identifier length"))?;

        let vault_len = u32::try_from(vault_id.len())
            .map_err(|_| anyhow!("Invalid Vault identifier length"))?;

        let mut message = Vec::with_capacity(
            64 + provider_id.len() + vault_id.len(),
        );

        message.extend_from_slice(
            b"matter-base:provider-binding:v1\0",
        );
        message.extend_from_slice(challenge);
        message.extend_from_slice(&provider_len.to_be_bytes());
        message.extend_from_slice(provider_id.as_bytes());
        message.extend_from_slice(&vault_len.to_be_bytes());
        message.extend_from_slice(vault_id.as_bytes());

        Ok(message)
    }

    /// Calcula la prueba esperada para la sesión autenticada.
    fn calculate_binding_proof(
        binding_key: &[u8; 32],
        message: &[u8],
    ) -> Result<[u8; 32]> {
        let mut mac = HmacSha256::new_from_slice(binding_key)
            .map_err(|_| anyhow!("Invalid provider binding key"))?;

        mac.update(message);

        let output = mac.finalize().into_bytes();
        let mut proof = [0u8; 32];

        proof.copy_from_slice(&output);

        Ok(proof)
    }
}