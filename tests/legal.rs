/*!
 * Pruebas: Legal & Consent Manager
 *
 * Función:
 * Verificar documentos, decisiones, consentimientos y acceso legal.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use matter_base::{ConsentDecision, LegalDocument, LegalDocumentType, LegalStore};

fn create_document(
    document_id: &str,
    document_type: LegalDocumentType,
    mandatory: bool,
    required_decision: ConsentDecision,
) -> LegalDocument {
    LegalDocument {
        document_id: document_id.to_string(),
        scope: "matter-base".to_string(),
        document_type,
        version: "1.0.0".to_string(),
        locale: "es".to_string(),
        content: format!("Contenido de prueba: {document_id}"),
        mandatory,
        required_decision,
        active: true,
    }
}

fn create_terms() -> LegalDocument {
    create_document(
        "base-terms-1.0.0-es",
        LegalDocumentType::Terms,
        true,
        ConsentDecision::Accepted,
    )
}

#[test]
fn registers_user_and_document() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-001").unwrap();

    let hash = store.register_document(&create_terms()).unwrap();

    assert!(hash.starts_with("sha256:"));
    assert_eq!(hash.len(), 71);
    assert!(store.user_exists("user-001").unwrap());
}

#[test]
fn accepted_terms_allow_access() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-002").unwrap();
    store.register_document(&create_terms()).unwrap();

    assert!(!store
        .is_access_allowed("user-002", "matter-base", "es")
        .unwrap());

    store
        .record_decision(
            "user-002",
            "base-terms-1.0.0-es",
            ConsentDecision::Accepted,
            Some("test"),
        )
        .unwrap();

    assert!(store
        .is_access_allowed("user-002", "matter-base", "es")
        .unwrap());
}

#[test]
fn declined_terms_block_access() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-003").unwrap();
    store.register_document(&create_terms()).unwrap();

    store
        .record_decision(
            "user-003",
            "base-terms-1.0.0-es",
            ConsentDecision::Declined,
            None,
        )
        .unwrap();

    assert!(!store
        .is_access_allowed("user-003", "matter-base", "es")
        .unwrap());
}

#[test]
fn latest_decline_replaces_previous_acceptance() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-004").unwrap();
    store.register_document(&create_terms()).unwrap();

    store
        .record_decision(
            "user-004",
            "base-terms-1.0.0-es",
            ConsentDecision::Accepted,
            None,
        )
        .unwrap();

    assert!(store
        .is_access_allowed("user-004", "matter-base", "es")
        .unwrap());

    store
        .record_decision(
            "user-004",
            "base-terms-1.0.0-es",
            ConsentDecision::Declined,
            None,
        )
        .unwrap();

    assert!(!store
        .is_access_allowed("user-004", "matter-base", "es")
        .unwrap());

    assert_eq!(store.decision_history("user-004").unwrap().len(), 2);
}

#[test]
fn missing_required_documents_block_access() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-005").unwrap();

    assert!(!store
        .is_access_allowed("user-005", "matter-base", "es")
        .unwrap());

    assert!(!store.has_required_documents("matter-base", "es").unwrap());
}

#[test]
fn duplicate_document_with_same_metadata_is_idempotent() {
    let store = LegalStore::open_in_memory().unwrap();
    let document = create_terms();

    let first = store.register_document(&document).unwrap();
    let second = store.register_document(&document).unwrap();

    assert_eq!(first, second);
}

#[test]
fn duplicate_document_id_with_different_version_is_rejected() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_document(&create_terms()).unwrap();

    let mut changed = create_terms();
    changed.version = "2.0.0".to_string();

    assert!(store.register_document(&changed).is_err());
}

#[test]
fn duplicate_document_id_with_different_content_is_rejected() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_document(&create_terms()).unwrap();

    let mut changed = create_terms();
    changed.content = "Contenido modificado".to_string();

    assert!(store.register_document(&changed).is_err());
}

#[test]
fn invalid_required_decision_is_rejected() {
    let store = LegalStore::open_in_memory().unwrap();

    let document = create_document(
        "invalid-terms",
        LegalDocumentType::Terms,
        true,
        ConsentDecision::Granted,
    );

    assert!(store.register_document(&document).is_err());
}

#[test]
fn invalid_decision_for_document_type_is_rejected() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-006").unwrap();
    store.register_document(&create_terms()).unwrap();

    assert!(store
        .record_decision(
            "user-006",
            "base-terms-1.0.0-es",
            ConsentDecision::Granted,
            None,
        )
        .is_err());
}

#[test]
fn privacy_notice_requires_acknowledgement() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-007").unwrap();

    let privacy = create_document(
        "privacy-1.0.0-es",
        LegalDocumentType::PrivacyNotice,
        true,
        ConsentDecision::Acknowledged,
    );

    store.register_document(&privacy).unwrap();

    assert!(store
        .record_decision(
            "user-007",
            "privacy-1.0.0-es",
            ConsentDecision::Accepted,
            None,
        )
        .is_err());

    store
        .record_decision(
            "user-007",
            "privacy-1.0.0-es",
            ConsentDecision::Acknowledged,
            None,
        )
        .unwrap();
}

#[test]
fn optional_consent_does_not_block_general_access() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-008").unwrap();
    store.register_document(&create_terms()).unwrap();

    let ai_consent = create_document(
        "ai-training-1.0.0-es",
        LegalDocumentType::Consent,
        false,
        ConsentDecision::Granted,
    );

    store.register_document(&ai_consent).unwrap();

    store
        .record_decision(
            "user-008",
            "base-terms-1.0.0-es",
            ConsentDecision::Accepted,
            None,
        )
        .unwrap();

    store
        .record_decision(
            "user-008",
            "ai-training-1.0.0-es",
            ConsentDecision::Declined,
            None,
        )
        .unwrap();

    assert!(store
        .is_access_allowed("user-008", "matter-base", "es")
        .unwrap());
}

#[test]
fn consent_can_be_withdrawn_after_grant() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-009").unwrap();

    let consent = create_document(
        "ai-training-1.0.0-es",
        LegalDocumentType::Consent,
        false,
        ConsentDecision::Granted,
    );

    store.register_document(&consent).unwrap();

    assert!(store
        .record_decision(
            "user-009",
            "ai-training-1.0.0-es",
            ConsentDecision::Withdrawn,
            None,
        )
        .is_err());

    store
        .record_decision(
            "user-009",
            "ai-training-1.0.0-es",
            ConsentDecision::Granted,
            None,
        )
        .unwrap();

    store
        .record_decision(
            "user-009",
            "ai-training-1.0.0-es",
            ConsentDecision::Withdrawn,
            None,
        )
        .unwrap();

    let latest = store
        .latest_decision("user-009", "ai-training-1.0.0-es")
        .unwrap()
        .unwrap();

    assert_eq!(latest.decision, "withdrawn");
    assert_eq!(store.decision_history("user-009").unwrap().len(), 2);
}

#[test]
fn withdrawn_consent_cannot_be_withdrawn_twice() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-010").unwrap();

    let consent = create_document(
        "ai-training-2.0.0-es",
        LegalDocumentType::Consent,
        false,
        ConsentDecision::Granted,
    );

    store.register_document(&consent).unwrap();

    store
        .record_decision(
            "user-010",
            "ai-training-2.0.0-es",
            ConsentDecision::Granted,
            None,
        )
        .unwrap();

    store
        .record_decision(
            "user-010",
            "ai-training-2.0.0-es",
            ConsentDecision::Withdrawn,
            None,
        )
        .unwrap();

    assert!(store
        .record_decision(
            "user-010",
            "ai-training-2.0.0-es",
            ConsentDecision::Withdrawn,
            None,
        )
        .is_err());
}

#[test]
fn latest_decision_returns_last_record() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-011").unwrap();
    store.register_document(&create_terms()).unwrap();

    store
        .record_decision(
            "user-011",
            "base-terms-1.0.0-es",
            ConsentDecision::Accepted,
            Some("web"),
        )
        .unwrap();

    store
        .record_decision(
            "user-011",
            "base-terms-1.0.0-es",
            ConsentDecision::Declined,
            Some("mobile"),
        )
        .unwrap();

    let latest = store
        .latest_decision("user-011", "base-terms-1.0.0-es")
        .unwrap()
        .unwrap();

    assert_eq!(latest.decision, "declined");
    assert_eq!(latest.platform.as_deref(), Some("mobile"));
}

#[test]
fn unregistered_user_cannot_record_decisions() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_document(&create_terms()).unwrap();

    assert!(store
        .record_decision(
            "unknown-user",
            "base-terms-1.0.0-es",
            ConsentDecision::Accepted,
            None,
        )
        .is_err());
}

#[test]
fn document_can_be_deactivated() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-012").unwrap();
    store.register_document(&create_terms()).unwrap();

    store
        .set_document_active("base-terms-1.0.0-es", false)
        .unwrap();

    assert!(!store.has_required_documents("matter-base", "es").unwrap());

    assert!(store
        .record_decision(
            "user-012",
            "base-terms-1.0.0-es",
            ConsentDecision::Accepted,
            None,
        )
        .is_err());
}

#[test]
fn deactivated_document_can_be_reactivated() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_document(&create_terms()).unwrap();

    store
        .set_document_active("base-terms-1.0.0-es", false)
        .unwrap();

    store
        .set_document_active("base-terms-1.0.0-es", true)
        .unwrap();

    assert!(store.has_required_documents("matter-base", "es").unwrap());
}

#[test]
fn latest_decision_is_none_when_no_decision_exists() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-014").unwrap();
    store.register_document(&create_terms()).unwrap();

    assert!(store
        .latest_decision("user-014", "base-terms-1.0.0-es",)
        .unwrap()
        .is_none());
}

#[test]
fn concurrent_registration_of_same_document_is_idempotent_across_connections() {
    use std::sync::{Arc, Barrier};
    use std::thread;

    let directory = std::env::temp_dir().join(format!(
        "matter-base-legal-race-{}-{}",
        std::process::id(),
        time::OffsetDateTime::now_utc().unix_timestamp_nanos()
    ));
    std::fs::create_dir_all(&directory).unwrap();
    let database = directory.join("legal.db");
    let first = LegalStore::open(&database).unwrap();
    let second = LegalStore::open(&database).unwrap();
    let barrier = Arc::new(Barrier::new(3));
    let document_a = create_terms();
    let document_b = create_terms();

    let first_barrier = Arc::clone(&barrier);
    let first_thread = thread::spawn(move || {
        first_barrier.wait();
        first.register_document(&document_a)
    });
    let second_barrier = Arc::clone(&barrier);
    let second_thread = thread::spawn(move || {
        second_barrier.wait();
        second.register_document(&document_b)
    });

    barrier.wait();
    assert!(first_thread.join().unwrap().is_ok());
    assert!(second_thread.join().unwrap().is_ok());
    let verification = LegalStore::open(&database).unwrap();
    assert!(verification.register_document(&create_terms()).is_ok());
    drop(verification);
    std::fs::remove_dir_all(directory).unwrap();
}

#[test]
fn interrupted_decision_write_rolls_back_without_partial_history() {
    let directory = std::env::temp_dir().join(format!(
        "matter-base-legal-interruption-{}-{}",
        std::process::id(),
        time::OffsetDateTime::now_utc().unix_timestamp_nanos()
    ));
    std::fs::create_dir_all(&directory).unwrap();
    let database = directory.join("legal.db");
    let store = LegalStore::open(&database).unwrap();
    store.register_user("interrupted-user").unwrap();
    store.register_document(&create_terms()).unwrap();

    let external_connection = rusqlite::Connection::open(&database).unwrap();
    external_connection
        .execute_batch(
            "CREATE TRIGGER fail_legal_decision_insert
             BEFORE INSERT ON legal_decisions
             BEGIN
                 SELECT RAISE(ABORT, 'simulated storage interruption');
             END;",
        )
        .unwrap();

    assert!(store
        .record_decision(
            "interrupted-user",
            "base-terms-1.0.0-es",
            ConsentDecision::Accepted,
            None,
        )
        .is_err());
    assert!(store
        .decision_history("interrupted-user")
        .unwrap()
        .is_empty());
    drop(external_connection);
    drop(store);
    std::fs::remove_dir_all(directory).unwrap();
}
