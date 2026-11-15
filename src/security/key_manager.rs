
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
}

impl KeyManager {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        key_id: String,
        key: [u8; 32],
    ) -> Result<()> {
        if key_id.is_empty() || key_id.len() > 128 {
            bail!("Invalid key identifier");
        }

        if self.keys.contains_key(&key_id) {
            bail!("Key already registered");
        }

        self.keys.insert(
            key_id,
            Zeroizing::new(key),
        );

        Ok(())
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
        self.keys.remove(key_id).is_some()
    }

    pub fn clear(&mut self) {
        self.keys.clear();
    }
}

impl Default for KeyManager {
    fn default() -> Self {
        Self::new()
    }
}
