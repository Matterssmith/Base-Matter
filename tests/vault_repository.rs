/* Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use matter_base::security::encryption::{decrypt, encrypt};
use serde_json::Value;

/// Reads the actual separately checked-out Vault metadata. Does not modify it,
/// access production keys or claim an authenticated remote key-service test.
#[test]
#[ignore = "requires MATTER_VAULT_CHECKOUT pointing to the inspected repository"]
fn real_vault_repository_metadata_matches_base_envelope_contract() {
    let root = std::path::PathBuf::from(
        std::env::var("MATTER_VAULT_CHECKOUT").expect("explicit Vault checkout required"),
    );
    let read = |path: &str| -> Value {
        serde_json::from_slice(&std::fs::read(root.join(path)).unwrap()).unwrap()
    };
    let manifest = read("vault/manifest.json");
    let versions = read("versions/schema.json");
    let schema = read("schemas/encrypted-envelope.schema.json");
    assert_eq!(manifest["vault_id"], "matterssmith-primary-vault");
    assert_eq!(manifest["format_version"], 1);
    assert_eq!(manifest["schema_version"], 1);
    assert_eq!(versions["envelope_format"], 1);
    assert_eq!(versions["minimum_reader_version"], 1);
    assert_eq!(schema["additionalProperties"], false);
    assert_eq!(schema["properties"]["format_version"]["const"], 1);
    assert_eq!(
        schema["properties"]["algorithm"]["const"],
        "XChaCha20-Poly1305"
    );
    let mut key = zeroize::Zeroizing::new([0u8; 32]);
    getrandom::fill(key.as_mut()).unwrap();
    for kind in [
        "roles",
        "permissions",
        "configuration",
        "audit",
        "encrypted_key",
    ] {
        assert!(schema["properties"]["record_type"]["enum"]
            .as_array()
            .unwrap()
            .contains(&Value::String(kind.into())));
        let envelope = encrypt(
            b"ephemeral contract probe",
            &key,
            "probe",
            "test",
            kind,
            "test-key",
            "2026-11-16T00:00:00Z",
        )
        .unwrap();
        let value = serde_json::to_value(&envelope).unwrap();
        let fields = value.as_object().unwrap();
        let required = schema["required"].as_array().unwrap();
        assert_eq!(fields.len(), required.len());
        for field in required {
            assert!(fields.contains_key(field.as_str().unwrap()));
        }
        assert_eq!(
            decrypt(&envelope, &key).unwrap(),
            b"ephemeral contract probe"
        );
    }
}
