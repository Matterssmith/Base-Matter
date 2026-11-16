/*
 * Matter-Base — Authenticated Key Provider Tests
 *
 * Verifica la vinculación criptográfica de proveedores,
 * sus permisos y la carga protegida de claves.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::{bail, Result};
use matter_base::security::{
    create_provider_binding_proof,
    AuthenticatedProvider,
    BoundKeyProvider,
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

const ASSERTION_KEY: [u8; 32] = [7u8; 32];
const BINDING_KEY: [u8; 32] = [9u8; 32];

struct MockKeyProvider {
    vault_id: String,
    keys: HashMap<String, [u8; 32]>,
    active_key_id: Option<String>,
    binding_key: [u8; 32],
}

impl MockKeyProvider {
    fn new(vault_id: &str, binding_key: [u8; 32]) -> Self {
        let mut keys = HashMap::new();

        keys.insert("vault-key-1".to_owned(), [11u8; 32]);
        keys.insert("vault-key-2".to_owned(), [12u8; 32]);

        Self {
            vault_id: vault_id.to_owned(),
            keys,
            active_key_id: Some("vault-key-1".to_owned()),
            binding_key,
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

impl BoundKeyProvider for MockKeyProvider {
    fn binding_proof(&self, message: &[u8]) -> Result<[u8; 32]> {
        create_provider_binding_proof(
            &self.binding_key,
            message,
        )
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
    assertion_key: &[u8; 32],
    binding_key: [u8; 32],
) -> AuthenticatedProvider {
    let assertion = assertion(permissions);

    let ciphertext = ProviderAuthenticator::seal_assertion(
        &assertion,
        nonce,
        assertion_key,
    )
    .unwrap();

    let mut authenticator = ProviderAuthenticator::new(
        VAULT.to_owned(),
        AUDIENCE.to_owned(),
    )
    .unwrap();

    AuthenticatedProvider::authenticate_and_authorize_with_binding_key(
        &mut authenticator,
        &policy(),
        nonce,
        &ciphertext,
        assertion_key,
        Zeroizing::new(binding_key),
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
fn loads_keys_from_cryptographically_bound_provider() {
    let nonce = [1u8; 24];

    let session = authenticated_provider(
        all_permissions(),
        ProviderPermission::ReadKeyIds,
        &nonce,
        &ASSERTION_KEY,
        BINDING_KEY,
    );

    let source = MockKeyProvider::new(VAULT, BINDING_KEY);
    let mut manager = KeyManager::new();

    session
        .load_key_manager(&mut manager, &source)
        .unwrap();

    assert_eq!(manager.len(), 2);
    assert_eq!(manager.active_key_id(), Some("vault-key-1"));
    assert_eq!(manager.get("vault-key-1").unwrap(), &[11u8; 32]);
    assert_eq!(manager.get("vault-key-2").unwrap(), &[12u8; 32]);
    assert_eq!(manager.provider_vault_id(), Some(VAULT));
}

#[test]
fn rejects_loading_without_key_ids_permission() {
    let nonce = [2u8; 24];

    let session = authenticated_provider(
        vec![
            "read_active_key".to_owned(),
            "read_key_material".to_owned(),
        ],
        ProviderPermission::ReadActiveKey,
        &nonce,
        &ASSERTION_KEY,
        BINDING_KEY,
    );

    let source = MockKeyProvider::new(VAULT, BINDING_KEY);
    let mut manager = KeyManager::new();

    assert!(session.load_key_manager(&mut manager, &source).is_err());
    assert!(manager.is_empty());
}

#[test]
fn rejects_loading_without_active_key_permission() {
    let nonce = [3u8; 24];

    let session = authenticated_provider(
        vec![
            "read_key_ids".to_owned(),
            "read_key_material".to_owned(),
        ],
        ProviderPermission::ReadKeyIds,
        &nonce,
        &ASSERTION_KEY,
        BINDING_KEY,
    );

    let source = MockKeyProvider::new(VAULT, BINDING_KEY);
    let mut manager = KeyManager::new();

    assert!(session.load_key_manager(&mut manager, &source).is_err());
    assert!(manager.is_empty());
}

#[test]
fn rejects_loading_without_key_material_permission() {
    let nonce = [4u8; 24];

    let session = authenticated_provider(
        vec![
            "read_key_ids".to_owned(),
            "read_active_key".to_owned(),
        ],
        ProviderPermission::ReadKeyIds,
        &nonce,
        &ASSERTION_KEY,
        BINDING_KEY,
    );

    let source = MockKeyProvider::new(VAULT, BINDING_KEY);
    let mut manager = KeyManager::new();

    assert!(session.load_key_manager(&mut manager, &source).is_err());
    assert!(manager.is_empty());
}

#[test]
fn rejects_provider_from_different_vault() {
    let nonce = [5u8; 24];

    let session = authenticated_provider(
        all_permissions(),
        ProviderPermission::ReadKeyIds,
        &nonce,
        &ASSERTION_KEY,
        BINDING_KEY,
    );

    let source = MockKeyProvider::new("another-vault", BINDING_KEY);
    let mut manager = KeyManager::new();

    assert!(session.load_key_manager(&mut manager, &source).is_err());
    assert!(manager.is_empty());
}

#[test]
fn rejects_provider_with_different_binding_secret() {
    let nonce = [6u8; 24];

    let session = authenticated_provider(
        all_permissions(),
        ProviderPermission::ReadKeyIds,
        &nonce,
        &ASSERTION_KEY,
        BINDING_KEY,
    );

    let source = MockKeyProvider::new(VAULT, [8u8; 32]);
    let mut manager = KeyManager::new();

    assert!(session.load_key_manager(&mut manager, &source).is_err());
    assert!(manager.is_empty());
}

#[test]
fn preserves_existing_keys_when_binding_fails() {
    let nonce = [7u8; 24];

    let session = authenticated_provider(
        all_permissions(),
        ProviderPermission::ReadKeyIds,
        &nonce,
        &ASSERTION_KEY,
        BINDING_KEY,
    );

    let source = MockKeyProvider::new(VAULT, [8u8; 32]);
    let mut manager = KeyManager::new();

    manager
        .insert("existing-key".to_owned(), [99u8; 32])
        .unwrap();

    manager.set_active("existing-key").unwrap();

    assert!(session.load_key_manager(&mut manager, &source).is_err());

    assert_eq!(manager.len(), 1);
    assert_eq!(manager.active_key_id(), Some("existing-key"));
    assert_eq!(manager.get("existing-key").unwrap(), &[99u8; 32]);
}