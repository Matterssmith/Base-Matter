
/*
 * Matter-Base — Authenticated Key Provider Tests
 *
 * Verifica que la carga desde KeyProvider exija permisos
 * y corresponda al Vault autenticado.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::{bail, Result};
use matter_base::security::{
    AuthenticatedProvider,
    KeyManager,
    KeyProvider,
    ProviderAccessPolicy,
    ProviderAssertion,
    ProviderAuthenticator,
    ProviderPermission,
};
use std::collections::{HashMap, HashSet};
use time::OffsetDateTime;
use zeroize::Zeroizing;

const VAULT: &str = "matterssmith-primary-vault";
const AUDIENCE: &str = "matter-base-production";
const PROVIDER: &str = "matter-vault-provider";

struct MockKeyProvider {
    vault_id: String,
    keys: HashMap<String, [u8; 32]>,
    active_key_id: Option<String>,
}

impl MockKeyProvider {
    fn new(vault_id: &str) -> Self {
        let mut keys = HashMap::new();

        keys.insert("vault-key-1".to_owned(), [11u8; 32]);
        keys.insert("vault-key-2".to_owned(), [12u8; 32]);

        Self {
            vault_id: vault_id.to_owned(),
            keys,
            active_key_id: Some("vault-key-1".to_owned()),
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
        match self.keys.get(key_id) {
            Some(key) => Ok(Zeroizing::new(*key)),
            None => bail!("Unknown key"),
        }
    }
}

fn assertion(permissions: Vec<String>) -> ProviderAssertion {
    let now = OffsetDateTime::now_utc().unix_timestamp();

    ProviderAssertion {
        provider_id: PROVIDER.to_owned(),
        vault_id: VAULT.to_owned(),
        audience: AUDIENCE.to_owned(),
        assertion_id: format!("key-provider-{now}"),
        issued_at: now,
        expires_at: now + 120,
        permissions,
    }
}

fn policy() -> ProviderAccessPolicy {
    ProviderAccessPolicy::new(
        VAULT.to_owned(),
        [PROVIDER.to_owned()]
            .into_iter()
            .collect::<HashSet<_>>(),
    )
    .unwrap()
}

fn authenticated_provider(
    permissions: Vec<String>,
    required_permission: ProviderPermission,
    nonce: &[u8; 24],
    key: &[u8; 32],
) -> AuthenticatedProvider {
    let assertion = assertion(permissions);

    let ciphertext = ProviderAuthenticator::seal_assertion(
        &assertion,
        nonce,
        key,
    )
    .unwrap();

    let mut authenticator = ProviderAuthenticator::new(
        VAULT.to_owned(),
        AUDIENCE.to_owned(),
    )
    .unwrap();

    AuthenticatedProvider::authenticate_and_authorize(
        &mut authenticator,
        &policy(),
        nonce,
        &ciphertext,
        key,
        required_permission,
    )
    .unwrap()
}

fn all_permissions() -> Vec<String> {
    vec![
        "read_key_ids".to_owned(),
        "read_active_key".to_owned(),
        "read_key_material".to_owned(),
    ]
}

#[test]
fn loads_keys_from_authenticated_provider() {
    let key = [7u8; 32];
    let nonce = [1u8; 24];

    let provider = authenticated_provider(
        all_permissions(),
        ProviderPermission::ReadKeyIds,
        &nonce,
        &key,
    );

    let source = MockKeyProvider::new(VAULT);
    let mut manager = KeyManager::new();

    provider
        .load_key_manager(&mut manager, &source)
        .unwrap();

    assert_eq!(manager.len(), 2);
    assert_eq!(manager.active_key_id(), Some("vault-key-1"));
    assert_eq!(manager.get("vault-key-1").unwrap(), &[11u8; 32]);
    assert_eq!(manager.get("vault-key-2").unwrap(), &[12u8; 32]);
}

#[test]
fn rejects_loading_without_key_ids_permission() {
    let key = [7u8; 32];
    let nonce = [2u8; 24];

    let provider = authenticated_provider(
        vec![
            "read_active_key".to_owned(),
            "read_key_material".to_owned(),
        ],
        ProviderPermission::ReadActiveKey,
        &nonce,
        &key,
    );

    let source = MockKeyProvider::new(VAULT);
    let mut manager = KeyManager::new();

    assert!(
        provider
            .load_key_manager(&mut manager, &source)
            .is_err()
    );

    assert!(manager.is_empty());
}

#[test]
fn rejects_loading_without_active_key_permission() {
    let key = [7u8; 32];
    let nonce = [3u8; 24];

    let provider = authenticated_provider(
        vec![
            "read_key_ids".to_owned(),
            "read_key_material".to_owned(),
        ],
        ProviderPermission::ReadKeyIds,
        &nonce,
        &key,
    );

    let source = MockKeyProvider::new(VAULT);
    let mut manager = KeyManager::new();

    assert!(
        provider
            .load_key_manager(&mut manager, &source)
            .is_err()
    );

    assert!(manager.is_empty());
}

#[test]
fn rejects_loading_without_key_material_permission() {
    let key = [7u8; 32];
    let nonce = [4u8; 24];

    let provider = authenticated_provider(
        vec![
            "read_key_ids".to_owned(),
            "read_active_key".to_owned(),
        ],
        ProviderPermission::ReadKeyIds,
        &nonce,
        &key,
    );

    let source = MockKeyProvider::new(VAULT);
    let mut manager = KeyManager::new();

    assert!(
        provider
            .load_key_manager(&mut manager, &source)
            .is_err()
    );

    assert!(manager.is_empty());
}

#[test]
fn rejects_provider_from_different_vault() {
    let key = [7u8; 32];
    let nonce = [5u8; 24];

    let provider = authenticated_provider(
        all_permissions(),
        ProviderPermission::ReadKeyIds,
        &nonce,
        &key,
    );

    let source = MockKeyProvider::new("another-vault");
    let mut manager = KeyManager::new();

    assert!(
        provider
            .load_key_manager(&mut manager, &source)
            .is_err()
    );

    assert!(manager.is_empty());
}

#[test]
fn preserves_existing_keys_when_vault_does_not_match() {
    let key = [7u8; 32];
    let nonce = [6u8; 24];

    let provider = authenticated_provider(
        all_permissions(),
        ProviderPermission::ReadKeyIds,
        &nonce,
        &key,
    );

    let source = MockKeyProvider::new("another-vault");
    let mut manager = KeyManager::new();

    manager
        .insert("existing-key".to_owned(), [99u8; 32])
        .unwrap();

    manager.set_active("existing-key").unwrap();

    assert!(
        provider
            .load_key_manager(&mut manager, &source)
            .is_err()
    );

    assert_eq!(manager.len(), 1);
    assert_eq!(manager.active_key_id(), Some("existing-key"));
    assert_eq!(manager.get("existing-key").unwrap(), &[99u8; 32]);
}
