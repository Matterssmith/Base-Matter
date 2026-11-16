/*
 * Matter-Base — Provider Authenticator
 *
 * Verifica afirmaciones autenticadas de proveedores mediante
 * XChaCha20-Poly1305, expiración y protección contra repetición.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use crate::security::provider_identity::{ProviderIdentity, ProviderPermission};
use anyhow::{bail, Result};
use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    XChaCha20Poly1305, XNonce,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;

const MAX_ASSERTION_LIFETIME: i64 = 300;
const CLOCK_SKEW_SECONDS: i64 = 30;
const MAX_REPLAY_ENTRIES: usize = 4096;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderAssertion {
    pub provider_id: String,
    pub vault_id: String,
    pub audience: String,
    pub assertion_id: String,
    pub issued_at: i64,
    pub expires_at: i64,
    pub permissions: Vec<String>,
}

pub struct ProviderAuthenticator {
    expected_vault_id: String,
    expected_audience: String,
    used_assertions: HashMap<String, i64>,
}

impl ProviderAuthenticator {
    pub fn new(expected_vault_id: String, expected_audience: String) -> Result<Self> {
        Self::validate_identifier(&expected_vault_id)?;
        Self::validate_identifier(&expected_audience)?;

        Ok(Self {
            expected_vault_id,
            expected_audience,
            used_assertions: HashMap::new(),
        })
    }

    /// Descifra y verifica una afirmación del proveedor.
    ///
    /// La clave debe proceder de un canal de confianza.
    /// El nonce debe ser único para cada cifrado realizado
    /// con la misma clave.
    pub fn authenticate(
        &mut self,
        nonce: &[u8; 24],
        ciphertext: &[u8],
        key: &[u8; 32],
    ) -> Result<ProviderIdentity> {
        let cipher = XChaCha20Poly1305::new_from_slice(key)
            .map_err(|_| anyhow::anyhow!("Invalid authentication key"))?;

        let aad = Self::associated_data(&self.expected_vault_id, &self.expected_audience);

        let plaintext = cipher
            .decrypt(
                XNonce::from_slice(nonce),
                Payload {
                    msg: ciphertext,
                    aad: aad.as_bytes(),
                },
            )
            .map_err(|_| anyhow::anyhow!("Provider assertion authentication failed"))?;

        let assertion: ProviderAssertion = serde_json::from_slice(&plaintext)?;

        self.validate_assertion(&assertion)?;

        let permissions = Self::parse_permissions(&assertion.permissions)?;

        let identity = ProviderIdentity::from_verified_assertion(
            assertion.provider_id,
            assertion.vault_id,
            permissions,
        )?;

        self.used_assertions
            .insert(assertion.assertion_id, assertion.expires_at);

        Ok(identity)
    }

    fn validate_assertion(&mut self, assertion: &ProviderAssertion) -> Result<()> {
        Self::validate_identifier(&assertion.provider_id)?;
        Self::validate_identifier(&assertion.vault_id)?;
        Self::validate_identifier(&assertion.audience)?;
        Self::validate_identifier(&assertion.assertion_id)?;

        if assertion.vault_id != self.expected_vault_id {
            bail!("Provider Vault mismatch");
        }

        if assertion.audience != self.expected_audience {
            bail!("Provider audience mismatch");
        }

        let now = OffsetDateTime::now_utc().unix_timestamp();
        let earliest_issued_at = now.saturating_sub(MAX_ASSERTION_LIFETIME + CLOCK_SKEW_SECONDS);
        let latest_issued_at = now.saturating_add(CLOCK_SKEW_SECONDS);

        if assertion.issued_at > latest_issued_at {
            bail!("Provider assertion issued in the future");
        }

        if assertion.issued_at < earliest_issued_at {
            bail!("Provider assertion is too old");
        }

        if assertion.expires_at <= now {
            bail!("Provider assertion expired");
        }

        if assertion.expires_at <= assertion.issued_at {
            bail!("Invalid provider assertion lifetime");
        }

        let lifetime = assertion
            .expires_at
            .checked_sub(assertion.issued_at)
            .ok_or_else(|| anyhow::anyhow!("Invalid provider assertion lifetime"))?;

        if lifetime > MAX_ASSERTION_LIFETIME {
            bail!("Provider assertion lifetime too long");
        }

        self.used_assertions
            .retain(|_, expires_at| *expires_at > now);

        if self.used_assertions.contains_key(&assertion.assertion_id) {
            bail!("Provider assertion replay detected");
        }

        if self.used_assertions.len() >= MAX_REPLAY_ENTRIES {
            bail!("Provider replay cache is full");
        }

        Ok(())
    }

    fn parse_permissions(
        values: &[String],
    ) -> Result<std::collections::HashSet<ProviderPermission>> {
        let mut permissions = std::collections::HashSet::new();

        for value in values {
            let permission = match value.as_str() {
                "read_key_ids" => ProviderPermission::ReadKeyIds,
                "read_active_key" => ProviderPermission::ReadActiveKey,
                "read_key_material" => ProviderPermission::ReadKeyMaterial,
                _ => bail!("Unknown provider permission"),
            };

            if !permissions.insert(permission) {
                bail!("Duplicate provider permission");
            }
        }

        if permissions.is_empty() {
            bail!("Provider has no permissions");
        }

        Ok(permissions)
    }

    /// Cifra una afirmación para transporte autenticado.
    ///
    /// El nonce debe generarse aleatoriamente y no repetirse
    /// con la misma clave.
    pub fn seal_assertion(
        assertion: &ProviderAssertion,
        nonce: &[u8; 24],
        key: &[u8; 32],
    ) -> Result<Vec<u8>> {
        let cipher = XChaCha20Poly1305::new_from_slice(key)
            .map_err(|_| anyhow::anyhow!("Invalid authentication key"))?;

        let plaintext = serde_json::to_vec(assertion)?;

        let aad = Self::associated_data(&assertion.vault_id, &assertion.audience);

        cipher
            .encrypt(
                XNonce::from_slice(nonce),
                Payload {
                    msg: &plaintext,
                    aad: aad.as_bytes(),
                },
            )
            .map_err(|_| anyhow::anyhow!("Provider assertion encryption failed"))
    }

    fn associated_data(vault_id: &str, audience: &str) -> String {
        format!(
            "matter-base:provider-assertion:v1\0{}\0{}",
            vault_id, audience
        )
    }

    fn validate_identifier(value: &str) -> Result<()> {
        if value.trim().is_empty() || value.len() > 128 || value.chars().any(char::is_control) {
            bail!("Invalid provider assertion identifier");
        }

        Ok(())
    }
}
