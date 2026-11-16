/*
 * Matter-Base — Key Provider Tests
 *
 * Verifica la carga segura de claves desde proveedores.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::{bail, Result};
use matter_base::security::{KeyManager, KeyProvider};
use std::collections::HashMap;
use zeroize::Zeroizing;

struct MockKeyProvider {
    vault_id: String,
    keys: HashMap<String, [u8; 32]>,
    active_key_id: Option<String>,
    failing_key_id: Option<String>,
}

impl MockKeyProvider {
    fn new() -> Self {
        let mut keys = HashMap::new();

        keys.insert("key-1".to_owned(), [1u8; 32]);
        keys.insert("key-2".to_owned(), [2u8; 32]);

        Self {
            vault_id: "matterssmith-primary-vault".to_owned(),
            keys,
            active_key_id: Some("key-2".to_owned()),
            failing_key_id: None,
        }
    }
}

impl KeyProvider for MockKeyProvider {
    fn vault_id(&self) -> Result<String> {
        Ok(self.vault_id.clone())
    }

    fn key_ids(&self) -> Result<Vec<String>> {
        Ok(self.keys.keys().cloned().collect())
    }

    fn active_key_id(&self) -> Result<Option<String>> {
        Ok(self.active_key_id.clone())
    }

    fn load_key(&self, key_id: &str) -> Result<Zeroizing<[u8; 32]>> {
        if self.failing_key_id.as_deref() == Some(key_id) {
            bail!("upstream diagnostic leaked-secret-token");
        }

        let key = self
            .keys
            .get(key_id)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("Key not available"))?;

        Ok(Zeroizing::new(key))
    }
}

struct OversizedKeyListProvider;

impl KeyProvider for OversizedKeyListProvider {
    fn vault_id(&self) -> Result<String> {
        Ok("test-vault".to_owned())
    }

    fn key_ids(&self) -> Result<Vec<String>> {
        Ok((0..4097).map(|index| format!("key-{index}")).collect())
    }

    fn active_key_id(&self) -> Result<Option<String>> {
        Ok(None)
    }

    fn load_key(&self, _key_id: &str) -> Result<Zeroizing<[u8; 32]>> {
        bail!("key loading must not start for an oversized list")
    }
}

#[test]
fn loads_keys_from_provider() {
    let provider = MockKeyProvider::new();
    let mut manager = KeyManager::new();

    manager.load_from_provider(&provider).unwrap();

    assert_eq!(manager.len(), 2);
    assert!(manager.contains("key-1"));
    assert!(manager.contains("key-2"));
    assert_eq!(manager.active_key_id(), Some("key-2"));
}

#[test]
fn records_provider_vault_id() {
    let provider = MockKeyProvider::new();
    let mut manager = KeyManager::new();

    manager.load_from_provider(&provider).unwrap();

    assert_eq!(
        manager.provider_vault_id(),
        Some("matterssmith-primary-vault")
    );
}

#[test]
fn failed_load_preserves_existing_keys() {
    let mut manager = KeyManager::new();

    manager
        .insert("existing-key".to_owned(), [9u8; 32])
        .unwrap();
    manager.set_active("existing-key").unwrap();

    let mut provider = MockKeyProvider::new();
    provider.failing_key_id = Some("key-1".to_owned());

    let error = manager.load_from_provider(&provider).unwrap_err();
    assert!(!error.to_string().contains("leaked-secret-token"));
    assert_eq!(error.to_string(), "Key provider key retrieval failed");

    assert_eq!(manager.len(), 1);
    assert!(manager.contains("existing-key"));
    assert_eq!(manager.active_key_id(), Some("existing-key"));
    assert_eq!(manager.provider_vault_id(), None);
}

#[test]
fn rejects_active_key_missing_from_provider() {
    let mut manager = KeyManager::new();

    manager
        .insert("existing-key".to_owned(), [9u8; 32])
        .unwrap();

    let mut provider = MockKeyProvider::new();
    provider.active_key_id = Some("missing-key".to_owned());

    assert!(manager.load_from_provider(&provider).is_err());

    assert_eq!(manager.len(), 1);
    assert!(manager.contains("existing-key"));
    assert_eq!(manager.provider_vault_id(), None);
}

#[test]
fn loaded_active_key_can_encrypt() {
    let provider = MockKeyProvider::new();
    let mut manager = KeyManager::new();

    manager.load_from_provider(&provider).unwrap();

    let (key_id, key) = manager.get_active().unwrap();

    assert_eq!(key_id, "key-2");
    assert_eq!(key, &[2u8; 32]);
}

#[test]
fn rejects_oversized_provider_key_list_without_replacing_existing_keys() {
    let provider = OversizedKeyListProvider;
    let mut manager = KeyManager::new();
    manager
        .insert("existing-key".to_owned(), [9u8; 32])
        .unwrap();
    manager.set_active("existing-key").unwrap();

    assert!(manager.load_from_provider(&provider).is_err());
    assert_eq!(manager.len(), 1);
    assert!(manager.contains("existing-key"));
    assert!(!manager.contains("key-0"));
    assert_eq!(manager.active_key_id(), Some("existing-key"));
    assert_eq!(manager.provider_vault_id(), None);
}
