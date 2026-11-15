
/*!
 * Pruebas: Legal & Consent Manager
 *
 * Función:
 * Verificar el registro de documentos y las decisiones legales.
 *
 * Casos:
 * - Registro de usuarios.
 * - Registro de documentos.
 * - Aceptación de términos.
 * - Rechazo de términos.
 * - Bloqueo cuando falta aceptación.
 * - Conservación del historial.
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

fn create_terms() -> LegalDocument {
    LegalDocument {
        document_id: "base-terms-1.0.0-es".to_string(),
        scope: "matter-base".to_string(),
        document_type: LegalDocumentType::Terms,
        version: "1.0.0".to_string(),
        locale: "es".to_string(),
        content: "Términos de prueba de Matter-Base".to_string(),
        mandatory: true,
        required_decision: ConsentDecision::Accepted,
        active: true,
    }
}

#[test]
fn registers_user_and_document() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-001").unwrap();

    let hash = store.register_document(&create_terms()).unwrap();

    assert!(hash.starts_with("sha256:"));
    assert_eq!(hash.len(), 71);
}

#[test]
fn accepted_terms_allow_access() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-002").unwrap();
    store.register_document(&create_terms()).unwrap();

    assert!(!store.is_access_allowed(
        "user-002",
        "matter-base",
        "es"
    ).unwrap());

    store.record_decision(
        "user-002",
        "base-terms-1.0.0-es",
        ConsentDecision::Accepted,
        Some("whatsapp"),
    ).unwrap();

    assert!(store.is_access_allowed(
        "user-002",
        "matter-base",
        "es"
    ).unwrap());
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
        Some("telegram"),
    ).unwrap();

    assert!(!store.is_access_allowed(
        "user-003",
        "matter-base",
        "es"
    ).unwrap());
}

#[test]
fn new_decline_replaces_previous_acceptance_for_access() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-004").unwrap();
    store.register_document(&create_terms()).unwrap();

    store.record_decision(
        "user-004",
        "base-terms-1.0.0-es",
        ConsentDecision::Accepted,
        None,
    ).unwrap();

    assert!(store.is_access_allowed(
        "user-004",
        "matter-base",
        "es"
    ).unwrap());

    store.record_decision(
        "user-004",
        "base-terms-1.0.0-es",
        ConsentDecision::Declined,
        None,
    ).unwrap();

    assert!(!store.is_access_allowed(
        "user-004",
        "matter-base",
        "es"
    ).unwrap());

    let history = store.decision_history("user-004").unwrap();

    assert_eq!(history.len(), 2);
}

#[test]
fn missing_required_documents_block_access() {
    let store = LegalStore::open_in_memory().unwrap();

    store.register_user("user-005").unwrap();

    assert!(!store.is_access_allowed(
        "user-005",
        "matter-base",
        "es"
    ).unwrap());
}
