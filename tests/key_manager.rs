
/*
 * Matter-Base — Key Manager Tests
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use matter_base::security::KeyManager;

fn test_key(value: u8) -> [u8; 32] {
    [value; 32]
}

#[test]
fn inserts_and_retrieves_key() {
    let mut manager = KeyManager::new();

    manager
        .insert("key-001".to_owned(), test_key(1))
        .unwrap();

    assert!(manager.contains("key-001"));
    assert_eq!(manager.get("key-001").unwrap(), &test_key(1));
}

#[test]
fn rejects_empty_key_id() {
    let mut manager = KeyManager::new();

    assert!(manager.insert(String::new(), test_key(1)).is_err());
}

#[test]
fn rejects_whitespace_key_id() {
    let mut manager = KeyManager::new();

    assert!(manager.insert("   ".to_owned(), test_key(1)).is_err());
}

#[test]
fn rejects_key_id_longer_than_128_bytes() {
    let mut manager = KeyManager::new();

    let key_id = "a".repeat(129);

    assert!(manager.insert(key_id, test_key(1)).is_err());
}

#[test]
fn rejects_control_characters_in_key_id() {
    let mut manager = KeyManager::new();

    assert!(
        manager
            .insert("key\n001".to_owned(), test_key(1))
            .is_err()
    );
}

#[test]
fn rejects_duplicate_key_id() {
    let mut manager = KeyManager::new();

    manager
        .insert("key-001".to_owned(), test_key(1))
        .unwrap();

    assert!(
        manager
            .insert("key-001".to_owned(), test_key(2))
            .is_err()
    );

    assert_eq!(manager.get("key-001").unwrap(), &test_key(1));
}

#[test]
fn rejects_unknown_key_lookup() {
    let manager = KeyManager::new();

    assert!(manager.get("unknown").is_err());
}

#[test]
fn first_insert_does_not_implicitly_activate_key() {
    let mut manager = KeyManager::new();

    manager
        .insert("key-001".to_owned(), test_key(1))
        .unwrap();

    assert_eq!(manager.active_key_id(), None);
    assert!(manager.get_active().is_err());
}

#[test]
fn selects_existing_active_key() {
    let mut manager = KeyManager::new();

    manager
        .insert("key-001".to_owned(), test_key(1))
        .unwrap();

    manager
        .insert("key-002".to_owned(), test_key(2))
        .unwrap();

    manager.set_active("key-002").unwrap();

    let (key_id, key) = manager.get_active().unwrap();

    assert_eq!(key_id, "key-002");
    assert_eq!(key, &test_key(2));
}

#[test]
fn rejects_unknown_active_key() {
    let mut manager = KeyManager::new();

    assert!(manager.set_active("unknown").is_err());
    assert_eq!(manager.active_key_id(), None);
}

#[test]
fn rotation_activates_new_key_and_retains_previous_key() {
    let mut manager = KeyManager::new();

    manager
        .rotate("key-001".to_owned(), test_key(1))
        .unwrap();

    manager
        .rotate("key-002".to_owned(), test_key(2))
        .unwrap();

    assert_eq!(manager.active_key_id(), Some("key-002"));
    assert_eq!(manager.get("key-001").unwrap(), &test_key(1));
    assert_eq!(manager.get("key-002").unwrap(), &test_key(2));
}

#[test]
fn rotation_rejects_duplicate_without_changing_active_key() {
    let mut manager = KeyManager::new();

    manager
        .rotate("key-001".to_owned(), test_key(1))
        .unwrap();

    assert!(
        manager
            .rotate("key-001".to_owned(), test_key(2))
            .is_err()
    );

    assert_eq!(manager.active_key_id(), Some("key-001"));
    assert_eq!(manager.get("key-001").unwrap(), &test_key(1));
}

#[test]
fn can_switch_back_to_previous_key() {
    let mut manager = KeyManager::new();

    manager
        .rotate("key-001".to_owned(), test_key(1))
        .unwrap();

    manager
        .rotate("key-002".to_owned(), test_key(2))
        .unwrap();

    manager.set_active("key-001").unwrap();

    assert_eq!(manager.active_key_id(), Some("key-001"));
}

#[test]
fn cannot_remove_active_key() {
    let mut manager = KeyManager::new();

    manager
        .rotate("key-001".to_owned(), test_key(1))
        .unwrap();

    assert!(!manager.remove("key-001"));
    assert!(manager.contains("key-001"));
}

#[test]
fn can_remove_inactive_key() {
    let mut manager = KeyManager::new();

    manager
        .rotate("key-001".to_owned(), test_key(1))
        .unwrap();

    manager
        .insert("key-002".to_owned(), test_key(2))
        .unwrap();

    assert!(manager.remove("key-002"));
    assert!(!manager.contains("key-002"));
    assert!(manager.contains("key-001"));
}

#[test]
fn clear_removes_keys_and_active_selection() {
    let mut manager = KeyManager::new();

    manager
        .rotate("key-001".to_owned(), test_key(1))
        .unwrap();

    manager.clear();

    assert!(manager.is_empty());
    assert_eq!(manager.len(), 0);
    assert_eq!(manager.active_key_id(), None);
    assert!(manager.get_active().is_err());
}

#[test]
fn reports_key_count() {
    let mut manager = KeyManager::new();

    assert_eq!(manager.len(), 0);

    manager
        .insert("key-001".to_owned(), test_key(1))
        .unwrap();

    manager
        .insert("key-002".to_owned(), test_key(2))
        .unwrap();

    assert_eq!(manager.len(), 2);
    assert!(!manager.is_empty());
}
