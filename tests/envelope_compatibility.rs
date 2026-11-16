/*
 * Matter-Base — Encrypted Envelope Compatibility Tests
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use matter_base::security::{decrypt, encrypt, validate_envelope, EncryptedEnvelope};
use serde_json::{json, Value};
use std::collections::BTreeSet;

const ENVELOPE_ID: &str = "compatibility-test-001";
const SCOPE: &str = "matter-vault";
const RECORD_TYPE: &str = "configuration";
const KEY_ID: &str = "compatibility-key-001";
const CREATED_AT: &str = "2026-11-15T12:00:00Z";

fn test_key() -> [u8; 32] {
    [42u8; 32]
}

fn test_plaintext() -> &'static [u8] {
    b"Matter-Vault compatibility test"
}

fn create_envelope() -> EncryptedEnvelope {
    encrypt(
        test_plaintext(),
        &test_key(),
        ENVELOPE_ID,
        SCOPE,
        RECORD_TYPE,
        KEY_ID,
        CREATED_AT,
    )
    .expect("Envelope creation should succeed")
}

fn serialized_envelope() -> Value {
    serde_json::to_value(create_envelope()).expect("Envelope serialization should succeed")
}

#[test]
fn serialized_envelope_contains_exact_schema_fields() {
    let value = serialized_envelope();

    let object = value
        .as_object()
        .expect("Envelope must serialize as a JSON object");

    let actual: BTreeSet<String> = object.keys().cloned().collect();

    let expected: BTreeSet<String> = [
        "format_version",
        "envelope_id",
        "scope",
        "record_type",
        "key_id",
        "algorithm",
        "nonce",
        "ciphertext",
        "created_at",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();

    assert_eq!(actual, expected);
}

#[test]
fn serialized_envelope_uses_schema_constants() {
    let value = serialized_envelope();

    assert_eq!(value["format_version"], json!(1));
    assert_eq!(value["algorithm"], json!("XChaCha20-Poly1305"));

    assert_eq!(value["record_type"], json!("configuration"));
}

#[test]
fn serialized_nonce_matches_schema_encoding() {
    let value = serialized_envelope();

    let nonce = value["nonce"].as_str().expect("Nonce must be a string");

    assert_eq!(nonce.len(), 32);
    assert!(!nonce.contains('='));

    let decoded = URL_SAFE_NO_PAD
        .decode(nonce)
        .expect("Nonce must use base64url without padding");

    assert_eq!(decoded.len(), 24);
}

#[test]
fn serialized_ciphertext_uses_base64url_without_padding() {
    let value = serialized_envelope();

    let ciphertext = value["ciphertext"]
        .as_str()
        .expect("Ciphertext must be a string");

    assert!(!ciphertext.is_empty());
    assert!(!ciphertext.contains('='));

    let decoded = URL_SAFE_NO_PAD
        .decode(ciphertext)
        .expect("Ciphertext must use base64url without padding");

    assert!(decoded.len() >= 16);
}

#[test]
fn serialized_timestamp_uses_rfc3339() {
    let value = serialized_envelope();

    let timestamp = value["created_at"]
        .as_str()
        .expect("Timestamp must be a string");

    assert_eq!(timestamp, CREATED_AT);
}

#[test]
fn envelope_json_roundtrip_preserves_all_fields() {
    let original = create_envelope();

    let json = serde_json::to_string(&original).expect("Serialization should succeed");

    let restored: EncryptedEnvelope =
        serde_json::from_str(&json).expect("Deserialization should succeed");

    assert_eq!(original.format_version, restored.format_version);
    assert_eq!(original.envelope_id, restored.envelope_id);
    assert_eq!(original.scope, restored.scope);
    assert_eq!(original.record_type, restored.record_type);
    assert_eq!(original.key_id, restored.key_id);
    assert_eq!(original.algorithm, restored.algorithm);
    assert_eq!(original.nonce, restored.nonce);
    assert_eq!(original.ciphertext, restored.ciphertext);
    assert_eq!(original.created_at, restored.created_at);

    assert!(validate_envelope(&restored).is_ok());
}

#[test]
fn restored_envelope_can_decrypt_original_plaintext() {
    let original = create_envelope();

    let json = serde_json::to_string(&original).expect("Serialization should succeed");

    let restored: EncryptedEnvelope =
        serde_json::from_str(&json).expect("Deserialization should succeed");

    let plaintext = decrypt(&restored, &test_key()).expect("Restored envelope should decrypt");

    assert_eq!(plaintext, test_plaintext());
}

#[test]
fn unknown_json_fields_are_rejected() {
    let mut value = serialized_envelope();

    value["unexpected_field"] = json!("not-allowed");

    let result = serde_json::from_value::<EncryptedEnvelope>(value);

    assert!(result.is_err());
}

#[test]
fn missing_required_fields_are_rejected() {
    let required_fields = [
        "format_version",
        "envelope_id",
        "scope",
        "record_type",
        "key_id",
        "algorithm",
        "nonce",
        "ciphertext",
        "created_at",
    ];

    for field in required_fields {
        let mut value = serialized_envelope();

        value
            .as_object_mut()
            .expect("Envelope must be an object")
            .remove(field);

        let result = serde_json::from_value::<EncryptedEnvelope>(value);

        assert!(result.is_err(), "Missing field must be rejected: {field}");
    }
}

#[test]
fn invalid_json_field_types_are_rejected() {
    let mut value = serialized_envelope();
    value["format_version"] = json!("1");

    let result = serde_json::from_value::<EncryptedEnvelope>(value);

    assert!(result.is_err());
}

#[test]
fn unsupported_record_type_is_rejected_after_deserialization() {
    let mut value = serialized_envelope();
    value["record_type"] = json!("unknown");

    let envelope: EncryptedEnvelope =
        serde_json::from_value(value).expect("JSON structure should deserialize");

    assert!(validate_envelope(&envelope).is_err());
}

#[test]
fn unsupported_algorithm_is_rejected_after_deserialization() {
    let mut value = serialized_envelope();
    value["algorithm"] = json!("AES-256-GCM");

    let envelope: EncryptedEnvelope =
        serde_json::from_value(value).expect("JSON structure should deserialize");

    assert!(validate_envelope(&envelope).is_err());
}

#[test]
fn invalid_nonce_is_rejected_after_deserialization() {
    let mut value = serialized_envelope();
    value["nonce"] = json!("invalid");

    let envelope: EncryptedEnvelope =
        serde_json::from_value(value).expect("JSON structure should deserialize");

    assert!(validate_envelope(&envelope).is_err());
}

#[test]
fn invalid_timestamp_is_rejected_after_deserialization() {
    let mut value = serialized_envelope();
    value["created_at"] = json!("not-a-date");

    let envelope: EncryptedEnvelope =
        serde_json::from_value(value).expect("JSON structure should deserialize");

    assert!(validate_envelope(&envelope).is_err());
}
