
/*!
 * Pruebas: Legal & Consent Manager
 *
 * Función:
 * Verificar documentos, decisiones, consentimientos e integridad histórica.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use matter_base::{
    ConsentDecision,
    LegalDocument,
    LegalDocumentType,
    LegalStore,
};

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

    store.record_decision(
        "user-002",
        "base-terms-1.0.0-es",
        ConsentDecision::Accepted,
        Some("test"),
    ).unwrap();

    assert!(store
        .is_access_allowed("user-002", "matter-base", "es")
        .unwrap());
}

#[test]
fn declined_terms_block_access() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-003").unwrap();
    store.register_document(&create_terms()).unwrap();

    store.record_decision(
        "user-003",
        "base-terms-1.0.0-es",
        ConsentDecision::Declined,
        None,
    ).unwrap();

    assert!(!store
        .is_access_allowed("user-003", "matter-base", "es")
        .unwrap());
}

#[test]
fn latest_decline_replaces_previous_acceptance() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-004").unwrap();
    store.register_document(&create_terms()).unwrap();

    store.record_decision(
        "user-004",
        "base-terms-1.0.0-es",
        ConsentDecision::Accepted,
        None,
    ).unwrap();

    assert!(store
        .is_access_allowed("user-004", "matter-base", "es")
        .unwrap());

    store.record_decision(
        "user-004",
        "base-terms-1.0.0-es",
        ConsentDecision::Declined,
        None,
    ).unwrap();

    assert!(!store
        .is_access_allowed("user-004", "matter-base", "es")
        .unwrap());

    assert_eq!(
        store.decision_history("user-004").unwrap().len(),
        2
    );
}

#[test]
fn missing_required_documents_block_access() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-005").unwrap();

    assert!(!store
        .is_access_allowed("user-005", "matter-base", "es")
        .unwrap());

    assert!(!store
        .has_required_documents("matter-base", "es")
        .unwrap());
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

    let document = create_terms();
    store.register_document(&document).unwrap();

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

    assert!(store.record_decision(
        "user-006",
        "base-terms-1.0.0-es",
        ConsentDecision::Granted,
        None,
    ).is_err());
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

    assert!(store.record_decision(
        "user-007",
        "privacy-1.0.0-es",
        ConsentDecision::Accepted,
        None,
    ).is_err());

    store.record_decision(
        "user-007",
        "privacy-1.0.0-es",
        ConsentDecision::Acknowledged,
        None,
    ).unwrap();
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

    store.record_decision(
        "user-008",
        "base-terms-1.0.0-es",
        ConsentDecision::Accepted,
        None,
    ).unwrap();

    store.record_decision(
        "user-008",
        "ai-training-1.0.0-es",
        ConsentDecision::Declined,
        None,
    ).unwrap();

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

    assert!(store.record_decision(
        "user-009",
        "ai-training-1.0.0-es",
        ConsentDecision::Withdrawn,
        None,
    ).is_err());

    store.record_decision(
        "user-009",
        "ai-training-1.0.0-es",
        ConsentDecision::Granted,
        None,
    ).unwrap();

    store.record_decision(
        "user-009",
        "ai-training-1.0.0-es",
        ConsentDecision::Withdrawn,
        None,
    ).unwrap();

    let latest = store.latest_decision(
        "user-009",
        "ai-training-1.0.0-es",
    ).unwrap().unwrap();

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

    store.record_decision(
        "user-010",
        "ai-training-2.0.0-es",
        ConsentDecision::Granted,
        None,
    ).unwrap();

    store.record_decision(
        "user-010",
        "ai-training-2.0.0-es",
        ConsentDecision::Withdrawn,
        None,
    ).unwrap();

    assert!(store.record_decision(
        "user-010",
        "ai-training-2.0.0-es",
        ConsentDecision::Withdrawn,
        None,
    ).is_err());
}

#[test]
fn latest_decision_returns_last_record() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-011").unwrap();
    store.register_document(&create_terms()).unwrap();

    store.record_decision(
        "user-011",
        "base-terms-1.0.0-es",
        ConsentDecision::Accepted,
        Some("web"),
    ).unwrap();

    store.record_decision(
        "user-011",
        "base-terms-1.0.0-es",
        ConsentDecision::Declined,
        Some("mobile"),
    ).unwrap();

    let latest = store.latest_decision(
        "user-011",
        "base-terms-1.0.0-es",
    ).unwrap().unwrap();

    assert_eq!(latest.decision, "declined");
    assert_eq!(latest.platform.as_deref(), Some("mobile"));
}

#[test]
fn unregistered_user_cannot_record_decisions() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_document(&create_terms()).unwrap();

    assert!(store.record_decision(
        "unknown-user",
        "base-terms-1.0.0-es",
        ConsentDecision::Accepted,
        None,
    ).is_err());
}

#[test]
fn document_can_be_deactivated_without_changing_metadata() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-012").unwrap();
    store.register_document(&create_terms()).unwrap();

    store.set_document_active(
        "base-terms-1.0.0-es",
        false,
    ).unwrap();

    assert!(!store
        .has_required_documents("matter-base", "es")
        .unwrap());

    assert!(store.record_decision(
        "user-012",
        "base-terms-1.0.0-es",
        ConsentDecision::Accepted,
        None,
    ).is_err());
}

#[test]
fn document_metadata_cannot_be_updated_directly() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_document(&create_terms()).unwrap();

    let connection = store.connection.lock().unwrap();

    let result = connection.execute(
        "
        UPDATE legal_documents
        SET content = 'Contenido alterado'
        WHERE document_id = 'base-terms-1.0.0-es'
        ",
        [],
    );

    assert!(result.is_err());
}

#[test]
fn decision_history_cannot_be_updated_or_deleted() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-013").unwrap();
    store.register_document(&create_terms()).unwrap();

    store.record_decision(
        "user-013",
        "base-terms-1.0.0-es",
        ConsentDecision::Accepted,
        None,
    ).unwrap();

    let connection = store.connection.lock().unwrap();

    let update = connection.execute(
        "
        UPDATE legal_decisions
        SET decision = 'declined'
        WHERE user_id = 'user-013'
        ",
        [],
    );

    let delete = connection.execute(
        "
        DELETE FROM legal_decisions
        WHERE user_id = 'user-013'
        ",
        [],
    );

    assert!(update.is_err());
    assert!(delete.is_err());
}
