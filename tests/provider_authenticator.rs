/*
 * Matter-Base — Provider Authenticator Tests
 *
 * Verifica autenticación, expiración y protección contra repetición.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use matter_base::security::{ProviderAssertion, ProviderAuthenticator};
use time::OffsetDateTime;

const VAULT: &str = "matterssmith-primary-vault";
const AUDIENCE: &str = "matter-base-production";

fn assertion() -> ProviderAssertion {
    let now = OffsetDateTime::now_utc().unix_timestamp();

    ProviderAssertion {
        provider_id: "matter-vault-provider".to_owned(),
        vault_id: VAULT.to_owned(),
        audience: AUDIENCE.to_owned(),
        assertion_id: format!("assertion-{now}"),
        issued_at: now,
        expires_at: now + 120,
        permissions: vec![
            "read_key_ids".to_owned(),
            "read_active_key".to_owned(),
            "read_key_material".to_owned(),
        ],
    }
}

fn encrypted_assertion(assertion: &ProviderAssertion, nonce: &[u8; 24], key: &[u8; 32]) -> Vec<u8> {
    ProviderAuthenticator::seal_assertion(assertion, nonce, key).unwrap()
}

fn authenticator() -> ProviderAuthenticator {
    ProviderAuthenticator::new(VAULT.to_owned(), AUDIENCE.to_owned()).unwrap()
}

#[test]
fn authenticates_valid_provider() {
    let key = [7u8; 32];
    let nonce = [1u8; 24];
    let assertion = assertion();

    let ciphertext = encrypted_assertion(&assertion, &nonce, &key);
    let mut verifier = authenticator();

    let identity = verifier.authenticate(&nonce, &ciphertext, &key).unwrap();

    assert_eq!(identity.provider_id(), "matter-vault-provider");
    assert_eq!(identity.vault_id(), VAULT);
}

#[test]
fn rejects_modified_ciphertext() {
    let key = [7u8; 32];
    let nonce = [2u8; 24];
    let assertion = assertion();

    let mut ciphertext = encrypted_assertion(&assertion, &nonce, &key);
    ciphertext[0] ^= 1;

    let mut verifier = authenticator();

    assert!(verifier.authenticate(&nonce, &ciphertext, &key).is_err());
}

#[test]
fn rejects_wrong_key() {
    let key = [7u8; 32];
    let wrong_key = [8u8; 32];
    let nonce = [3u8; 24];
    let assertion = assertion();

    let ciphertext = encrypted_assertion(&assertion, &nonce, &key);
    let mut verifier = authenticator();

    assert!(verifier
        .authenticate(&nonce, &ciphertext, &wrong_key)
        .is_err());
}

#[test]
fn rejects_expired_assertion() {
    let key = [7u8; 32];
    let nonce = [4u8; 24];
    let mut assertion = assertion();

    assertion.issued_at -= 600;
    assertion.expires_at -= 300;

    let ciphertext = encrypted_assertion(&assertion, &nonce, &key);
    let mut verifier = authenticator();

    assert!(verifier.authenticate(&nonce, &ciphertext, &key).is_err());
}

#[test]
fn rejects_extreme_timestamps_without_overflowing() {
    let key = [7u8; 32];
    let mut verifier = authenticator();

    for (index, (issued_at, expires_at)) in [
        (i64::MIN, i64::MAX),
        (i64::MIN, i64::MIN + 120),
        (i64::MAX, i64::MAX),
    ]
    .into_iter()
    .enumerate()
    {
        let nonce = [20 + index as u8; 24];
        let mut assertion = assertion();
        assertion.assertion_id = format!("extreme-timestamp-{index}");
        assertion.issued_at = issued_at;
        assertion.expires_at = expires_at;

        let ciphertext = encrypted_assertion(&assertion, &nonce, &key);
        assert!(verifier.authenticate(&nonce, &ciphertext, &key).is_err());
    }
}

#[test]
fn rejects_oversized_assertions_before_decryption() {
    let nonce = [31u8; 24];
    let ciphertext = vec![0u8; 16 * 1024 + 1];
    let mut verifier = authenticator();

    assert!(verifier
        .authenticate(&nonce, &ciphertext, &[7u8; 32])
        .is_err());
}

#[test]
fn rejects_wrong_vault() {
    let key = [7u8; 32];
    let nonce = [5u8; 24];
    let mut assertion = assertion();

    assertion.vault_id = "another-vault".to_owned();

    let ciphertext = encrypted_assertion(&assertion, &nonce, &key);
    let mut verifier = authenticator();

    assert!(verifier.authenticate(&nonce, &ciphertext, &key).is_err());
}

#[test]
fn rejects_wrong_audience() {
    let key = [7u8; 32];
    let nonce = [6u8; 24];
    let mut assertion = assertion();

    assertion.audience = "another-service".to_owned();

    let ciphertext = encrypted_assertion(&assertion, &nonce, &key);
    let mut verifier = authenticator();

    assert!(verifier.authenticate(&nonce, &ciphertext, &key).is_err());
}

#[test]
fn rejects_replayed_assertion() {
    let key = [7u8; 32];
    let nonce = [7u8; 24];
    let assertion = assertion();

    let ciphertext = encrypted_assertion(&assertion, &nonce, &key);
    let mut verifier = authenticator();

    assert!(verifier.authenticate(&nonce, &ciphertext, &key).is_ok());

    assert!(verifier.authenticate(&nonce, &ciphertext, &key).is_err());
}

#[test]
fn rejects_unknown_permission() {
    let key = [7u8; 32];
    let nonce = [8u8; 24];
    let mut assertion = assertion();

    assertion.permissions.push("admin".to_owned());

    let ciphertext = encrypted_assertion(&assertion, &nonce, &key);
    let mut verifier = authenticator();

    assert!(verifier.authenticate(&nonce, &ciphertext, &key).is_err());
}

#[test]
fn rejects_empty_permissions() {
    let key = [7u8; 32];
    let nonce = [9u8; 24];
    let mut assertion = assertion();

    assertion.permissions.clear();

    let ciphertext = encrypted_assertion(&assertion, &nonce, &key);
    let mut verifier = authenticator();

    assert!(verifier.authenticate(&nonce, &ciphertext, &key).is_err());
}
