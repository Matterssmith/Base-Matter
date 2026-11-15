
/*
 * Matter-Base — Key Manager
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::{bail, Result};
use std::collections::HashMap;
use zeroize::Zeroizing;

pub struct KeyManager {
    keys: HashMap<String, Zeroizing<[u8; 32]>>,
    active_key_id: Option<String>,
}

impl KeyManager {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            active_key_id: None,
        }
    }

    pub fn insert(
        &mut self,
        key_id: String,
        key: [u8; 32],
    ) -> Result<()> {
        Self::validate_key_id(&key_id)?;

        if self.keys.contains_key(&key_id) {
            bail!("Key already registered");
        }

        self.keys.insert(key_id, Zeroizing::new(key));

        Ok(())
    }

    pub fn rotate(
        &mut self,
        key_id: String,
        key: [u8; 32],
    ) -> Result<()> {
        Self::validate_key_id(&key_id)?;

        if self.keys.contains_key(&key_id) {
            bail!("Key already registered");
        }

        self.keys.insert(key_id.clone(), Zeroizing::new(key));
        self.active_key_id = Some(key_id);

        Ok(())
    }

    pub fn set_active(&mut self, key_id: &str) -> Result<()> {
        if !self.keys.contains_key(key_id) {
            bail!("Key not available");
        }

        self.active_key_id = Some(key_id.to_owned());

        Ok(())
    }

    pub fn active_key_id(&self) -> Option<&str> {
        self.active_key_id.as_deref()
    }

    pub fn get_active(&self) -> Result<(&str, &[u8; 32])> {
        let key_id = self
            .active_key_id
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("No active key"))?;

        let key = self.get(key_id)?;

        Ok((key_id, key))
    }

    pub fn get(&self, key_id: &str) -> Result<&[u8; 32]> {
        self.keys
            .get(key_id)
            .map(|key| &**key)
            .ok_or_else(|| anyhow::anyhow!("Key not available"))
    }

    pub fn contains(&self, key_id: &str) -> bool {
        self.keys.contains_key(key_id)
    }

    pub fn remove(&mut self, key_id: &str) -> bool {
        if self.active_key_id.as_deref() == Some(key_id) {
            return false;
        }

        self.keys.remove(key_id).is_some()
    }

    pub fn clear(&mut self) {
        self.keys.clear();
        self.active_key_id = None;
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    fn validate_key_id(key_id: &str) -> Result<()> {
        if key_id.trim().is_empty()
            || key_id.len() > 128
            || key_id.chars().any(char::is_control)
        {
            bail!("Invalid key identifier");
        }

        Ok(())
    }
}

impl Default for KeyManager {
    fn default() -> Self {
        Self::new()
    }
}
