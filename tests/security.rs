/*
 * Matter-Base — Security Tests
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use matter_base::security::{decrypt, encrypt, validate_envelope, EncryptedEnvelope};

const ENVELOPE_ID: &str = "test-envelope-001";
const SCOPE: &str = "test-application";
const RECORD_TYPE: &str = "configuration";
const KEY_ID: &str = "test-key-001";
const CREATED_AT: &str = "2026-11-15T12:00:00Z";

fn test_key() -> [u8; 32] {
    [42u8; 32]
}

fn test_plaintext() -> Vec<u8> {
    b"Matter-Base security test".to_vec()
}

fn encrypted_envelope() -> EncryptedEnvelope {
    encrypt(
        &test_plaintext(),
        &test_key(),
        ENVELOPE_ID,
        SCOPE,
        RECORD_TYPE,
        KEY_ID,
        CREATED_AT,
    )
    .expect("Encryption should succeed")
}

#[test]
fn encryption_and_decryption_roundtrip() {
    let plaintext = test_plaintext();

    let envelope = encrypt(
        &plaintext,
        &test_key(),
        ENVELOPE_ID,
        SCOPE,
        RECORD_TYPE,
        KEY_ID,
        CREATED_AT,
    )
    .expect("Encryption should succeed");

    let decrypted = decrypt(&envelope, &test_key()).expect("Decryption should succeed");

    assert_eq!(decrypted, plaintext);
}

#[test]
fn encryption_generates_different_nonces() {
    let first = encrypted_envelope();
    let second = encrypted_envelope();

    assert_ne!(first.nonce, second.nonce);
}

#[test]
fn wrong_key_is_rejected() {
    let envelope = encrypted_envelope();
    let wrong_key = [99u8; 32];

    assert!(decrypt(&envelope, &wrong_key).is_err());
}

#[test]
fn modified_ciphertext_is_rejected() {
    let mut envelope = encrypted_envelope();

    let mut ciphertext = URL_SAFE_NO_PAD
        .decode(&envelope.ciphertext)
        .expect("Ciphertext should be valid");

    ciphertext[0] ^= 0x01;
    envelope.ciphertext = URL_SAFE_NO_PAD.encode(ciphertext);

    assert!(decrypt(&envelope, &test_key()).is_err());
}

#[test]
fn modified_nonce_is_rejected() {
    let mut envelope = encrypted_envelope();

    let mut nonce = URL_SAFE_NO_PAD
        .decode(&envelope.nonce)
        .expect("Nonce should be valid");

    nonce[0] ^= 0x01;
    envelope.nonce = URL_SAFE_NO_PAD.encode(nonce);

    assert!(decrypt(&envelope, &test_key()).is_err());
}

#[test]
fn modified_scope_is_rejected() {
    let mut envelope = encrypted_envelope();

    envelope.scope = "another-application".to_owned();

    assert!(decrypt(&envelope, &test_key()).is_err());
}

#[test]
fn modified_record_type_is_rejected() {
    let mut envelope = encrypted_envelope();

    envelope.record_type = "roles".to_owned();

    assert!(decrypt(&envelope, &test_key()).is_err());
}

#[test]
fn modified_envelope_id_is_rejected() {
    let mut envelope = encrypted_envelope();

    envelope.envelope_id = "another-envelope".to_owned();

    assert!(decrypt(&envelope, &test_key()).is_err());
}

#[test]
fn modified_key_id_is_rejected() {
    let mut envelope = encrypted_envelope();

    envelope.key_id = "another-key".to_owned();

    assert!(decrypt(&envelope, &test_key()).is_err());
}

#[test]
fn unsupported_format_version_is_rejected() {
    let mut envelope = encrypted_envelope();

    envelope.format_version = 2;

    assert!(decrypt(&envelope, &test_key()).is_err());
}

#[test]
fn unsupported_algorithm_is_rejected() {
    let mut envelope = encrypted_envelope();

    envelope.algorithm = "AES-256-GCM".to_owned();

    assert!(decrypt(&envelope, &test_key()).is_err());
}

#[test]
fn invalid_nonce_encoding_is_rejected() {
    let mut envelope = encrypted_envelope();

    envelope.nonce = "invalid_nonce".to_owned();

    assert!(decrypt(&envelope, &test_key()).is_err());
}

#[test]
fn invalid_nonce_length_is_rejected() {
    let mut envelope = encrypted_envelope();

    envelope.nonce = URL_SAFE_NO_PAD.encode([1u8; 12]);

    assert!(decrypt(&envelope, &test_key()).is_err());
}

#[test]
fn invalid_ciphertext_encoding_is_rejected() {
    let mut envelope = encrypted_envelope();

    envelope.ciphertext = "invalid*encoding".to_owned();

    assert!(decrypt(&envelope, &test_key()).is_err());
}

#[test]
fn short_ciphertext_is_rejected() {
    let mut envelope = encrypted_envelope();

    envelope.ciphertext = URL_SAFE_NO_PAD.encode([1u8; 8]);

    assert!(decrypt(&envelope, &test_key()).is_err());
}

#[test]
fn empty_envelope_id_is_rejected_during_encryption() {
    let result = encrypt(
        &test_plaintext(),
        &test_key(),
        "",
        SCOPE,
        RECORD_TYPE,
        KEY_ID,
        CREATED_AT,
    );

    assert!(result.is_err());
}

#[test]
fn empty_scope_is_rejected_during_encryption() {
    let result = encrypt(
        &test_plaintext(),
        &test_key(),
        ENVELOPE_ID,
        "",
        RECORD_TYPE,
        KEY_ID,
        CREATED_AT,
    );

    assert!(result.is_err());
}

#[test]
fn empty_key_id_is_rejected_during_encryption() {
    let result = encrypt(
        &test_plaintext(),
        &test_key(),
        ENVELOPE_ID,
        SCOPE,
        RECORD_TYPE,
        "",
        CREATED_AT,
    );

    assert!(result.is_err());
}

#[test]
fn empty_created_at_is_rejected_during_encryption() {
    let result = encrypt(
        &test_plaintext(),
        &test_key(),
        ENVELOPE_ID,
        SCOPE,
        RECORD_TYPE,
        KEY_ID,
        "",
    );

    assert!(result.is_err());
}

#[test]
fn unsupported_record_type_is_rejected_during_encryption() {
    let result = encrypt(
        &test_plaintext(),
        &test_key(),
        ENVELOPE_ID,
        SCOPE,
        "unknown_type",
        KEY_ID,
        CREATED_AT,
    );

    assert!(result.is_err());
}

#[test]
fn invalid_timestamp_is_rejected_during_encryption() {
    let result = encrypt(
        &test_plaintext(),
        &test_key(),
        ENVELOPE_ID,
        SCOPE,
        RECORD_TYPE,
        KEY_ID,
        "not-a-date",
    );

    assert!(result.is_err());
}

#[test]
fn invalid_calendar_date_is_rejected() {
    let result = encrypt(
        &test_plaintext(),
        &test_key(),
        ENVELOPE_ID,
        SCOPE,
        RECORD_TYPE,
        KEY_ID,
        "2026-02-30T12:00:00Z",
    );

    assert!(result.is_err());
}

#[test]
fn invalid_timestamp_without_timezone_is_rejected() {
    let result = encrypt(
        &test_plaintext(),
        &test_key(),
        ENVELOPE_ID,
        SCOPE,
        RECORD_TYPE,
        KEY_ID,
        "2026-11-15T12:00:00",
    );

    assert!(result.is_err());
}

#[test]
fn noncanonical_nonce_encoding_is_rejected() {
    let mut envelope = encrypted_envelope();

    envelope.nonce.push('=');

    assert!(validate_envelope(&envelope).is_err());
}

#[test]
fn noncanonical_ciphertext_encoding_is_rejected() {
    let mut envelope = encrypted_envelope();

    envelope.ciphertext.push('=');

    assert!(validate_envelope(&envelope).is_err());
}

#[test]
fn valid_envelope_passes_validation() {
    let envelope = encrypted_envelope();

    assert!(validate_envelope(&envelope).is_ok());
}

#[test]
fn valid_rfc3339_timestamp_with_offset_is_accepted() {
    let result = encrypt(
        &test_plaintext(),
        &test_key(),
        ENVELOPE_ID,
        SCOPE,
        RECORD_TYPE,
        KEY_ID,
        "2026-11-15T12:00:00+02:00",
    );

    assert!(result.is_ok());
}

#[test]
fn oversized_ciphertext_is_rejected_before_base64_decoding() {
    let mut envelope = encrypted_envelope();
    envelope.ciphertext = "A".repeat((16_usize * 1024 * 1024 * 4).div_ceil(3) + 1);

    assert!(validate_envelope(&envelope).is_err());
}

#[test]
fn oversized_plaintext_is_rejected_before_encryption() {
    let plaintext = vec![0; 16 * 1024 * 1024];

    assert!(encrypt(
        &plaintext,
        &test_key(),
        ENVELOPE_ID,
        SCOPE,
        RECORD_TYPE,
        KEY_ID,
        CREATED_AT,
    )
    .is_err());
}
