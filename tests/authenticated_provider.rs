
/*
 * Matter-Base — Authenticated Provider Tests
 *
 * Verifica la integración de autenticación y autorización.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use matter_base::security::{
    AuthenticatedProvider,
    ProviderAccessPolicy,
    ProviderAssertion,
    ProviderAuthenticator,
    ProviderPermission,
};
use std::collections::HashSet;
use time::OffsetDateTime;

const VAULT: &str = "matterssmith-primary-vault";
const AUDIENCE: &str = "matter-base-production";

fn permissions() -> Vec<String> {
    vec![
        "read_key_ids".to_owned(),
        "read_active_key".to_owned(),
        "read_key_material".to_owned(),
    ]
}

fn assertion() -> ProviderAssertion {
    let now = OffsetDateTime::now_utc().unix_timestamp();

    ProviderAssertion {
        provider_id: "matter-vault-provider".to_owned(),
        vault_id: VAULT.to_owned(),
        audience: AUDIENCE.to_owned(),
        assertion_id: format!("authenticated-provider-{now}"),
        issued_at: now,
        expires_at: now + 120,
        permissions: permissions(),
    }
}

fn policy() -> ProviderAccessPolicy {
    ProviderAccessPolicy::new(
        VAULT.to_owned(),
        ["matter-vault-provider".to_owned()]
            .into_iter()
            .collect::<HashSet<_>>(),
    )
    .unwrap()
}

fn prepare_assertion(
    assertion: &ProviderAssertion,
    nonce: &[u8; 24],
    key: &[u8; 32],
) -> Vec<u8> {
    ProviderAuthenticator::seal_assertion(
        assertion,
        nonce,
        key,
    )
    .unwrap()
}

fn authenticator() -> ProviderAuthenticator {
    ProviderAuthenticator::new(
        VAULT.to_owned(),
        AUDIENCE.to_owned(),
    )
    .unwrap()
}

#[test]
fn authenticates_and_authorizes_provider() {
    let key = [7u8; 32];
    let nonce = [1u8; 24];
    let assertion = assertion();

    let ciphertext = prepare_assertion(&assertion, &nonce, &key);
    let mut verifier = authenticator();

    let provider = AuthenticatedProvider::authenticate_and_authorize(
        &mut verifier,
        &policy(),
        &nonce,
        &ciphertext,
        &key,
        ProviderPermission::ReadKeyMaterial,
    )
    .unwrap();

    assert_eq!(provider.provider_id(), "matter-vault-provider");
    assert_eq!(provider.vault_id(), VAULT);
    assert_eq!(
        provider.authorized_permission(),
        ProviderPermission::ReadKeyMaterial
    );
}

#[test]
fn rejects_unknown_provider() {
    let key = [7u8; 32];
    let nonce = [2u8; 24];
    let mut assertion = assertion();

    assertion.provider_id = "unknown-provider".to_owned();

    let ciphertext = prepare_assertion(&assertion, &nonce, &key);
    let mut verifier = authenticator();

    let result = AuthenticatedProvider::authenticate_and_authorize(
        &mut verifier,
        &policy(),
        &nonce,
        &ciphertext,
        &key,
        ProviderPermission::ReadKeyIds,
    );

    assert!(result.is_err());
}

#[test]
fn rejects_provider_without_required_permission() {
    let key = [7u8; 32];
    let nonce = [3u8; 24];
    let mut assertion = assertion();

    assertion.permissions = vec!["read_key_ids".to_owned()];

    let ciphertext = prepare_assertion(&assertion, &nonce, &key);
    let mut verifier = authenticator();

    let result = AuthenticatedProvider::authenticate_and_authorize(
        &mut verifier,
        &policy(),
        &nonce,
        &ciphertext,
        &key,
        ProviderPermission::ReadKeyMaterial,
    );

    assert!(result.is_err());
}

#[test]
fn rejects_invalid_authentication_key() {
    let key = [7u8; 32];
    let wrong_key = [8u8; 32];
    let nonce = [4u8; 24];
    let assertion = assertion();

    let ciphertext = prepare_assertion(&assertion, &nonce, &key);
    let mut verifier = authenticator();

    let result = AuthenticatedProvider::authenticate_and_authorize(
        &mut verifier,
        &policy(),
        &nonce,
        &ciphertext,
        &wrong_key,
        ProviderPermission::ReadKeyIds,
    );

    assert!(result.is_err());
}

#[test]
fn rejects_replayed_provider_assertion() {
    let key = [7u8; 32];
    let nonce = [5u8; 24];
    let assertion = assertion();

    let ciphertext = prepare_assertion(&assertion, &nonce, &key);
    let mut verifier = authenticator();

    let first = AuthenticatedProvider::authenticate_and_authorize(
        &mut verifier,
        &policy(),
        &nonce,
        &ciphertext,
        &key,
        ProviderPermission::ReadKeyIds,
    );

    assert!(first.is_ok());

    let second = AuthenticatedProvider::authenticate_and_authorize(
        &mut verifier,
        &policy(),
        &nonce,
        &ciphertext,
        &key,
        ProviderPermission::ReadKeyIds,
    );

    assert!(second.is_err());
}

#[test]
fn exposes_only_the_authorized_permission() {
    let key = [7u8; 32];
    let nonce = [6u8; 24];
    let assertion = assertion();

    let ciphertext = prepare_assertion(&assertion, &nonce, &key);
    let mut verifier = authenticator();

    let provider = AuthenticatedProvider::authenticate_and_authorize(
        &mut verifier,
        &policy(),
        &nonce,
        &ciphertext,
        &key,
        ProviderPermission::ReadKeyIds,
    )
    .unwrap();

    assert!(provider.permits(ProviderPermission::ReadKeyIds));
    assert!(!provider.permits(ProviderPermission::ReadKeyMaterial));
}
