
/*
 * Matter-Base — Key Rotation Tests
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use matter_base::security::{
    decrypt_with_manager,
    encrypt_with_manager,
    KeyManager,
};

const TIMESTAMP: &str = "2026-11-15T12:00:00Z";

fn key(value: u8) -> [u8; 32] {
    [value; 32]
}

#[test]
fn encrypt_uses_active_key() {
    let mut manager = KeyManager::new();

    manager.rotate("key-001".to_owned(), key(1)).unwrap();

    let envelope = encrypt_with_manager(
        b"matter-base",
        &manager,
        "envelope-001",
        "global",
        "configuration",
        TIMESTAMP,
    )
    .unwrap();

    assert_eq!(envelope.key_id, "key-001");
}

#[test]
fn decrypt_uses_envelope_key_id() {
    let mut manager = KeyManager::new();

    manager.rotate("key-001".to_owned(), key(1)).unwrap();

    let envelope = encrypt_with_manager(
        b"matter-base",
        &manager,
        "envelope-001",
        "global",
        "configuration",
        TIMESTAMP,
    )
    .unwrap();

    manager.rotate("key-002".to_owned(), key(2)).unwrap();

    let plaintext = decrypt_with_manager(&envelope, &manager).unwrap();

    assert_eq!(plaintext, b"matter-base");
}

#[test]
fn new_envelopes_use_rotated_key() {
    let mut manager = KeyManager::new();

    manager.rotate("key-001".to_owned(), key(1)).unwrap();
    manager.rotate("key-002".to_owned(), key(2)).unwrap();

    let envelope = encrypt_with_manager(
        b"new-data",
        &manager,
        "envelope-002",
        "global",
        "configuration",
        TIMESTAMP,
    )
    .unwrap();

    assert_eq!(envelope.key_id, "key-002");
}

#[test]
fn old_and_new_envelopes_remain_decryptable() {
    let mut manager = KeyManager::new();

    manager.rotate("key-001".to_owned(), key(1)).unwrap();

    let old_envelope = encrypt_with_manager(
        b"old-data",
        &manager,
        "envelope-old",
        "global",
        "configuration",
        TIMESTAMP,
    )
    .unwrap();

    manager.rotate("key-002".to_owned(), key(2)).unwrap();

    let new_envelope = encrypt_with_manager(
        b"new-data",
        &manager,
        "envelope-new",
        "global",
        "configuration",
        TIMESTAMP,
    )
    .unwrap();

    assert_eq!(
        decrypt_with_manager(&old_envelope, &manager).unwrap(),
        b"old-data"
    );

    assert_eq!(
        decrypt_with_manager(&new_envelope, &manager).unwrap(),
        b"new-data"
    );
}

#[test]
fn unknown_envelope_key_is_rejected() {
    let mut encryption_manager = KeyManager::new();

    encryption_manager
        .rotate("key-001".to_owned(), key(1))
        .unwrap();

    let envelope = encrypt_with_manager(
        b"matter",
        &encryption_manager,
        "envelope-003",
        "global",
        "configuration",
        TIMESTAMP,
    )
    .unwrap();

    let mut decryption_manager = KeyManager::new();

    decryption_manager
        .rotate("key-002".to_owned(), key(2))
        .unwrap();

    assert!(decrypt_with_manager(&envelope, &decryption_manager).is_err());
}

#[test]
fn wrong_key_for_existing_id_is_rejected() {
    let mut encryption_manager = KeyManager::new();

    encryption_manager
        .rotate("key-001".to_owned(), key(1))
        .unwrap();

    let envelope = encrypt_with_manager(
        b"matter",
        &encryption_manager,
        "envelope-004",
        "global",
        "configuration",
        TIMESTAMP,
    )
    .unwrap();

    let mut decryption_manager = KeyManager::new();

    decryption_manager
        .rotate("key-001".to_owned(), key(2))
        .unwrap();

    assert!(decrypt_with_manager(&envelope, &decryption_manager).is_err());
}

#[test]
fn encryption_without_active_key_is_rejected() {
    let manager = KeyManager::new();

    assert!(
        encrypt_with_manager(
            b"matter",
            &manager,
            "envelope-005",
            "global",
            "configuration",
            TIMESTAMP,
        )
        .is_err()
    );
}
