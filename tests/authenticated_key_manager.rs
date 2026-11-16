/*
 * Matter-Base — Authenticated Key Manager Tests
 *
 * Verifica el acceso a claves mediante proveedores autenticados.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::Result;
use matter_base::security::{
    AuthenticatedProvider, KeyManager, KeyProvider, ProviderAccessPolicy, ProviderAssertion,
    ProviderAuthenticator, ProviderPermission,
};
use std::collections::HashSet;
use time::OffsetDateTime;
use zeroize::Zeroizing;

const VAULT: &str = "matterssmith-primary-vault";
const AUDIENCE: &str = "matter-base-production";
const PROVIDER: &str = "matter-vault-provider";

fn assertion(permissions: Vec<String>) -> ProviderAssertion {
    let now = OffsetDateTime::now_utc().unix_timestamp();

    ProviderAssertion {
        provider_id: PROVIDER.to_owned(),
        vault_id: VAULT.to_owned(),
        audience: AUDIENCE.to_owned(),
        assertion_id: format!("key-manager-{now}"),
        issued_at: now,
        expires_at: now + 120,
        permissions,
    }
}

fn prepare(permissions: Vec<String>, nonce: &[u8; 24], key: &[u8; 32]) -> Vec<u8> {
    ProviderAuthenticator::seal_assertion(&assertion(permissions), nonce, key).unwrap()
}

fn policy() -> ProviderAccessPolicy {
    ProviderAccessPolicy::new(
        VAULT.to_owned(),
        [PROVIDER.to_owned()].into_iter().collect::<HashSet<_>>(),
    )
    .unwrap()
}

fn manager() -> KeyManager {
    let mut manager = KeyManager::new();

    manager.insert("key-1".to_owned(), [9u8; 32]).unwrap();

    manager.insert("key-2".to_owned(), [8u8; 32]).unwrap();

    manager.set_active("key-1").unwrap();

    manager
}

struct OtherVaultKeyProvider;

impl KeyProvider for OtherVaultKeyProvider {
    fn vault_id(&self) -> Result<String> {
        Ok("unrelated-vault".to_owned())
    }

    fn key_ids(&self) -> Result<Vec<String>> {
        Ok(vec!["foreign-key".to_owned()])
    }

    fn active_key_id(&self) -> Result<Option<String>> {
        Ok(Some("foreign-key".to_owned()))
    }

    fn load_key(&self, _key_id: &str) -> Result<Zeroizing<[u8; 32]>> {
        Ok(Zeroizing::new([6u8; 32]))
    }
}

fn authenticate(
    permissions: Vec<String>,
    nonce: &[u8; 24],
    key: &[u8; 32],
    required: ProviderPermission,
) -> AuthenticatedProvider {
    let ciphertext = prepare(permissions, nonce, key);

    let mut authenticator =
        ProviderAuthenticator::new(VAULT.to_owned(), AUDIENCE.to_owned()).unwrap();

    AuthenticatedProvider::authenticate_and_authorize(
        &mut authenticator,
        &policy(),
        nonce,
        &ciphertext,
        key,
        required,
    )
    .unwrap()
}

#[test]
fn reads_active_key_id_with_permission() {
    let key = [7u8; 32];
    let nonce = [1u8; 24];

    let provider = authenticate(
        vec!["read_active_key".to_owned()],
        &nonce,
        &key,
        ProviderPermission::ReadActiveKey,
    );

    let manager = manager();

    assert_eq!(provider.read_active_key_id(&manager).unwrap(), "key-1");
}

#[test]
fn reads_key_material_with_permission() {
    let key = [7u8; 32];
    let nonce = [2u8; 24];

    let provider = authenticate(
        vec!["read_key_material".to_owned()],
        &nonce,
        &key,
        ProviderPermission::ReadKeyMaterial,
    );

    let manager = manager();

    assert_eq!(provider.read_key(&manager, "key-2").unwrap(), &[8u8; 32]);
}

#[test]
fn reads_active_key_with_both_permissions() {
    let key = [7u8; 32];
    let nonce = [3u8; 24];

    let provider = authenticate(
        vec!["read_active_key".to_owned(), "read_key_material".to_owned()],
        &nonce,
        &key,
        ProviderPermission::ReadKeyMaterial,
    );

    let manager = manager();

    let (key_id, material) = provider.read_active_key(&manager).unwrap();

    assert_eq!(key_id, "key-1");
    assert_eq!(material, &[9u8; 32]);
}

#[test]
fn rejects_key_material_without_permission() {
    let key = [7u8; 32];
    let nonce = [4u8; 24];

    let provider = authenticate(
        vec!["read_key_ids".to_owned()],
        &nonce,
        &key,
        ProviderPermission::ReadKeyIds,
    );

    let manager = manager();

    assert!(provider.read_key(&manager, "key-1").is_err());
}

#[test]
fn rejects_active_key_without_active_permission() {
    let key = [7u8; 32];
    let nonce = [5u8; 24];

    let provider = authenticate(
        vec!["read_key_material".to_owned()],
        &nonce,
        &key,
        ProviderPermission::ReadKeyMaterial,
    );

    let manager = manager();

    assert!(provider.read_active_key(&manager).is_err());
}

#[test]
fn rejects_active_key_without_material_permission() {
    let key = [7u8; 32];
    let nonce = [6u8; 24];

    let provider = authenticate(
        vec!["read_active_key".to_owned()],
        &nonce,
        &key,
        ProviderPermission::ReadActiveKey,
    );

    let manager = manager();

    assert!(provider.read_active_key(&manager).is_err());
}

#[test]
fn rejects_unknown_key_id() {
    let key = [7u8; 32];
    let nonce = [7u8; 24];

    let provider = authenticate(
        vec!["read_key_material".to_owned()],
        &nonce,
        &key,
        ProviderPermission::ReadKeyMaterial,
    );

    let manager = manager();

    assert!(provider.read_key(&manager, "unknown-key").is_err());
}

#[test]
fn authenticated_provider_cannot_read_keys_loaded_from_another_vault() {
    let mut foreign_manager = KeyManager::new();
    foreign_manager
        .load_from_provider(&OtherVaultKeyProvider)
        .unwrap();

    let material_provider = authenticate(
        vec!["read_key_material".to_owned()],
        &[31u8; 24],
        &[7u8; 32],
        ProviderPermission::ReadKeyMaterial,
    );
    assert!(material_provider
        .read_key(&foreign_manager, "foreign-key")
        .is_err());

    let active_provider = authenticate(
        vec!["read_active_key".to_owned()],
        &[32u8; 24],
        &[7u8; 32],
        ProviderPermission::ReadActiveKey,
    );
    assert!(active_provider
        .read_active_key_id(&foreign_manager)
        .is_err());
}
