/*
 * Matter-Base — Key Manager
 *
 * Administra claves criptográficas en memoria protegida
 * y permite cargarlas desde proveedores autenticados.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use crate::security::key_provider::KeyProvider;
use anyhow::{bail, Result};
use std::collections::HashMap;
use zeroize::Zeroizing;

const MAX_PROVIDER_KEYS: usize = 4096;

pub struct KeyManager {
    keys: HashMap<String, Zeroizing<[u8; 32]>>,
    active_key_id: Option<String>,
    provider_vault_id: Option<String>,
}

impl KeyManager {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            active_key_id: None,
            provider_vault_id: None,
        }
    }

    /// Registra una clave sin activarla.
    pub fn insert(&mut self, key_id: String, key: [u8; 32]) -> Result<()> {
        self.insert_zeroizing(key_id, Zeroizing::new(key))
    }

    /// Registra una clave transfiriendo su memoria protegida.
    pub fn insert_zeroizing(&mut self, key_id: String, key: Zeroizing<[u8; 32]>) -> Result<()> {
        Self::validate_key_id(&key_id)?;

        if self.keys.contains_key(&key_id) {
            bail!("Key already registered");
        }

        self.keys.insert(key_id, key);

        Ok(())
    }

    /// Registra y activa una clave nueva.
    pub fn rotate(&mut self, key_id: String, key: [u8; 32]) -> Result<()> {
        self.rotate_zeroizing(key_id, Zeroizing::new(key))
    }

    /// Rota la clave activa transfiriendo memoria protegida.
    pub fn rotate_zeroizing(&mut self, key_id: String, key: Zeroizing<[u8; 32]>) -> Result<()> {
        Self::validate_key_id(&key_id)?;

        if self.keys.contains_key(&key_id) {
            bail!("Key already registered");
        }

        self.keys.insert(key_id.clone(), key);
        self.active_key_id = Some(key_id);

        Ok(())
    }

    /// Cambia la clave activa a una clave ya registrada.
    pub fn set_active(&mut self, key_id: &str) -> Result<()> {
        if !self.keys.contains_key(key_id) {
            bail!("Key not available");
        }

        self.active_key_id = Some(key_id.to_owned());

        Ok(())
    }

    /// Carga todas las claves autorizadas desde un proveedor.
    ///
    /// La operación sustituye el conjunto actual únicamente
    /// cuando todas las claves se han recuperado correctamente.
    pub fn load_from_provider(&mut self, provider: &dyn KeyProvider) -> Result<()> {
        self.load_from_provider_with_expected_vault(provider, None)
    }

    /// Carga las claves solo si el proveedor sigue presentando el Vault
    /// asociado a la sesión autenticada. La validación ocurre antes de
    /// recuperar claves o reemplazar el estado actual.
    pub(crate) fn load_from_provider_for_vault(
        &mut self,
        provider: &dyn KeyProvider,
        expected_vault_id: &str,
    ) -> Result<()> {
        self.load_from_provider_with_expected_vault(provider, Some(expected_vault_id))
    }

    fn load_from_provider_with_expected_vault(
        &mut self,
        provider: &dyn KeyProvider,
        expected_vault_id: Option<&str>,
    ) -> Result<()> {
        let vault_id = provider.vault_id()?;
        Self::validate_vault_id(&vault_id)?;

        if expected_vault_id.is_some_and(|expected| expected != vault_id) {
            bail!("Provider Vault does not match authenticated Vault");
        }

        let key_ids = provider.key_ids()?;

        if key_ids.len() > MAX_PROVIDER_KEYS {
            bail!("Provider returned too many keys");
        }

        let active_key_id = provider.active_key_id()?;

        if let Some(active_id) = active_key_id.as_deref() {
            Self::validate_key_id(active_id)?;
        }

        let mut loaded_keys = HashMap::with_capacity(key_ids.len());

        for key_id in key_ids {
            Self::validate_key_id(&key_id)?;

            if loaded_keys.contains_key(&key_id) {
                bail!("Duplicate key identifier from provider");
            }

            let key = provider.load_key(&key_id)?;
            loaded_keys.insert(key_id, key);
        }

        if let Some(active_id) = active_key_id.as_deref() {
            if !loaded_keys.contains_key(active_id) {
                bail!("Provider active key is not available");
            }
        }

        self.keys = loaded_keys;
        self.active_key_id = active_key_id;
        self.provider_vault_id = Some(vault_id);

        Ok(())
    }

    /// Devuelve el identificador del Vault cargado.
    pub fn provider_vault_id(&self) -> Option<&str> {
        self.provider_vault_id.as_deref()
    }

    /// Devuelve el identificador de la clave activa.
    pub fn active_key_id(&self) -> Option<&str> {
        self.active_key_id.as_deref()
    }

    /// Devuelve la clave activa.
    pub fn get_active(&self) -> Result<(&str, &[u8; 32])> {
        let key_id = self
            .active_key_id
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("No active key"))?;

        let key = self.get(key_id)?;

        Ok((key_id, key))
    }

    /// Recupera una clave por identificador.
    pub fn get(&self, key_id: &str) -> Result<&[u8; 32]> {
        self.keys
            .get(key_id)
            .map(|key| &**key)
            .ok_or_else(|| anyhow::anyhow!("Key not available"))
    }

    /// Comprueba si una clave está registrada.
    pub fn contains(&self, key_id: &str) -> bool {
        self.keys.contains_key(key_id)
    }

    /// Elimina una clave que no esté activa.
    pub fn remove(&mut self, key_id: &str) -> bool {
        if self.active_key_id.as_deref() == Some(key_id) {
            return false;
        }

        self.keys.remove(key_id).is_some()
    }

    /// Elimina todas las claves y la referencia al proveedor.
    pub fn clear(&mut self) {
        self.keys.clear();
        self.active_key_id = None;
        self.provider_vault_id = None;
    }

    /// Devuelve el número de claves registradas.
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// Comprueba si no hay claves registradas.
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    fn validate_key_id(key_id: &str) -> Result<()> {
        if key_id.trim().is_empty() || key_id.len() > 128 || key_id.chars().any(char::is_control) {
            bail!("Invalid key identifier");
        }

        Ok(())
    }

    fn validate_vault_id(vault_id: &str) -> Result<()> {
        if vault_id.trim().is_empty()
            || vault_id.len() > 128
            || vault_id.chars().any(char::is_control)
        {
            bail!("Invalid vault identifier");
        }

        Ok(())
    }
}

impl Default for KeyManager {
    fn default() -> Self {
        Self::new()
    }
}
