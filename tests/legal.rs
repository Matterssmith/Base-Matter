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
mod common;
use common::{legal_admin, legal_user};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn new(label: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "matter-base-legal-{label}-{}-{nonce}",
            std::process::id()
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

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
fn legal_store_migration_versions_existing_schema_and_preserves_data() {
    let directory = TemporaryDirectory::new("legacy-schema");
    let database = directory.0.join("legal.db");
    let connection = rusqlite::Connection::open(&database).unwrap();
    connection
        .execute_batch(
            "CREATE TABLE legal_users (
                 user_id TEXT PRIMARY KEY,
                 created_at INTEGER NOT NULL
             );
             CREATE TABLE legal_documents (
                 document_id TEXT PRIMARY KEY,
                 scope TEXT NOT NULL,
                 document_type TEXT NOT NULL,
                 version TEXT NOT NULL,
                 locale TEXT NOT NULL,
                 content TEXT NOT NULL,
                 content_hash TEXT NOT NULL,
                 mandatory INTEGER NOT NULL,
                 required_decision TEXT NOT NULL,
                 active INTEGER NOT NULL,
                 created_at INTEGER NOT NULL,
                 UNIQUE(scope, document_type, version, locale)
             );
             CREATE TABLE legal_decisions (
                 record_id INTEGER PRIMARY KEY AUTOINCREMENT,
                 user_id TEXT NOT NULL,
                 document_id TEXT NOT NULL,
                 document_hash TEXT NOT NULL,
                 decision TEXT NOT NULL,
                 platform TEXT,
                 created_at INTEGER NOT NULL,
                 FOREIGN KEY(user_id) REFERENCES legal_users(user_id),
                 FOREIGN KEY(document_id) REFERENCES legal_documents(document_id)
             );
             INSERT INTO legal_users VALUES ('legacy-user', 1);
             INSERT INTO legal_documents VALUES (
                 'legacy-terms', 'scope', 'terms', '1.0.0', 'es',
                 'legacy terms', 'sha256:legacy', 1, 'accepted', 1, 1
             );
             INSERT INTO legal_decisions VALUES (
                 7, 'legacy-user', 'legacy-terms', 'sha256:legacy',
                 'accepted', 'legacy-platform', 2
             );",
        )
        .unwrap();
    drop(connection);

    let store = LegalStore::open(&database).unwrap();
    assert!(store
        .user_exists("legacy-user", &legal_user("legacy-user"))
        .unwrap());
    let history = store
        .decision_history("legacy-user", &legal_user("legacy-user"))
        .unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].record_id, 7);
    assert_eq!(history[0].document_hash, "sha256:legacy");
    assert_eq!(history[0].decision, "accepted");
    assert_eq!(history[0].platform.as_deref(), Some("legacy-platform"));
    drop(store);

    let connection = rusqlite::Connection::open(database).unwrap();
    let version: i64 = connection
        .query_row(
            "SELECT version FROM matter_schema_versions WHERE component = 'legal_store'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(version, 1);
}

#[test]
fn legal_store_schema_initialization_rolls_back_after_ddl_failure() {
    let directory = TemporaryDirectory::new("schema-rollback");
    let database = directory.0.join("legal.db");
    let connection = rusqlite::Connection::open(&database).unwrap();
    connection
        .execute_batch(
            "CREATE TABLE matter_schema_versions (
                 component TEXT PRIMARY KEY NOT NULL,
                 version INTEGER NOT NULL
             );
             INSERT INTO matter_schema_versions VALUES ('legal_store', 0);
             CREATE TABLE idx_legal_decisions_user (unrelated INTEGER);",
        )
        .unwrap();
    drop(connection);

    assert!(LegalStore::open(&database).is_err());
    let connection = rusqlite::Connection::open(&database).unwrap();
    let version: i64 = connection
        .query_row(
            "SELECT version FROM matter_schema_versions WHERE component = 'legal_store'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(version, 0);
    let legal_table_count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master
             WHERE type = 'table' AND name IN (
                 'legal_users', 'legal_documents', 'legal_decisions'
             )",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(legal_table_count, 0);
    drop(connection);

    let connection = rusqlite::Connection::open(&database).unwrap();
    connection
        .execute_batch("DROP TABLE idx_legal_decisions_user;")
        .unwrap();
    drop(connection);

    let store = LegalStore::open(&database).unwrap();
    store
        .register_user("recovered-user", &legal_user("recovered-user"))
        .unwrap();
    assert!(store
        .user_exists("recovered-user", &legal_user("recovered-user"))
        .unwrap());
}

#[test]
fn legal_store_rejects_future_schema_without_modifying_it() {
    let directory = TemporaryDirectory::new("future-schema");
    let database = directory.0.join("legal.db");
    let connection = rusqlite::Connection::open(&database).unwrap();
    connection
        .execute_batch(
            "CREATE TABLE matter_schema_versions (
                 component TEXT PRIMARY KEY NOT NULL,
                 version INTEGER NOT NULL
             );
             INSERT INTO matter_schema_versions VALUES ('legal_store', 99);
             CREATE TABLE caller_sentinel (value TEXT NOT NULL);
             INSERT INTO caller_sentinel VALUES ('preserve');",
        )
        .unwrap();
    drop(connection);

    assert!(LegalStore::open(&database).is_err());
    let connection = rusqlite::Connection::open(database).unwrap();
    let (version, sentinel): (i64, String) = connection
        .query_row(
            "SELECT
                (SELECT version FROM matter_schema_versions WHERE component = 'legal_store'),
                (SELECT value FROM caller_sentinel)",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .unwrap();
    assert_eq!(version, 99);
    assert_eq!(sentinel, "preserve");
}

#[test]
fn registers_user_and_document() {
    let store = LegalStore::open_in_memory().unwrap();

    store
        .register_user("user-001", &legal_user("user-001"))
        .unwrap();

    let hash = store
        .register_document(&create_terms(), &legal_admin())
        .unwrap();

    assert!(hash.starts_with("sha256:"));
    assert_eq!(hash.len(), 71);
    assert!(store
        .user_exists("user-001", &legal_user("user-001"))
        .unwrap());
}

#[test]
fn legal_authorization_rejects_cross_user_consent_and_admin_operations() {
    let store = LegalStore::open_in_memory().unwrap();
    store
        .register_user("victim-user", &legal_user("victim-user"))
        .unwrap();
    store
        .register_document(&create_terms(), &legal_admin())
        .unwrap();

    let decision_error = store
        .record_decision(
            "victim-user",
            "base-terms-1.0.0-es",
            ConsentDecision::Accepted,
            None,
            &legal_user("attacker-user"),
        )
        .unwrap_err();
    assert_eq!(decision_error.to_string(), "Legal authorization denied");
    assert!(store
        .decision_history("victim-user", &legal_user("victim-user"))
        .unwrap()
        .is_empty());
    assert!(store
        .user_exists("victim-user", &legal_user("attacker-user"))
        .is_err());
    assert!(store
        .is_access_allowed(
            "victim-user",
            "matter-base",
            "es",
            &legal_user("attacker-user"),
        )
        .is_err());
    let mut attacker_document = create_terms();
    attacker_document.document_id = "attacker-terms".to_owned();
    assert!(store
        .register_document(&attacker_document, &legal_user("attacker-user"))
        .is_err());
    assert!(store
        .set_document_active("base-terms-1.0.0-es", false, &legal_user("attacker-user"),)
        .is_err());
    assert!(store
        .has_required_documents("matter-base", "es", &legal_admin())
        .unwrap());
}

#[test]
fn accepted_terms_allow_access() {
    let store = LegalStore::open_in_memory().unwrap();

    store
        .register_user("user-002", &legal_user("user-002"))
        .unwrap();
    store
        .register_document(&create_terms(), &legal_admin())
        .unwrap();

    assert!(!store
        .is_access_allowed("user-002", "matter-base", "es", &legal_user("user-002"))
        .unwrap());

    store
        .record_decision(
            "user-002",
            "base-terms-1.0.0-es",
            ConsentDecision::Accepted,
            Some("test"),
            &legal_user("user-002"),
        )
        .unwrap();

    assert!(store
        .is_access_allowed("user-002", "matter-base", "es", &legal_user("user-002"))
        .unwrap());
}

#[test]
fn declined_terms_block_access() {
    let store = LegalStore::open_in_memory().unwrap();

    store
        .register_user("user-003", &legal_user("user-003"))
        .unwrap();
    store
        .register_document(&create_terms(), &legal_admin())
        .unwrap();

    store
        .record_decision(
            "user-003",
            "base-terms-1.0.0-es",
            ConsentDecision::Declined,
            None,
            &legal_user("user-003"),
        )
        .unwrap();

    assert!(!store
        .is_access_allowed("user-003", "matter-base", "es", &legal_user("user-003"))
        .unwrap());
}

#[test]
fn latest_decline_replaces_previous_acceptance() {
    let store = LegalStore::open_in_memory().unwrap();

    store
        .register_user("user-004", &legal_user("user-004"))
        .unwrap();
    store
        .register_document(&create_terms(), &legal_admin())
        .unwrap();

    store
        .record_decision(
            "user-004",
            "base-terms-1.0.0-es",
            ConsentDecision::Accepted,
            None,
            &legal_user("user-004"),
        )
        .unwrap();

    assert!(store
        .is_access_allowed("user-004", "matter-base", "es", &legal_user("user-004"))
        .unwrap());

    store
        .record_decision(
            "user-004",
            "base-terms-1.0.0-es",
            ConsentDecision::Declined,
            None,
            &legal_user("user-004"),
        )
        .unwrap();

    assert!(!store
        .is_access_allowed("user-004", "matter-base", "es", &legal_user("user-004"))
        .unwrap());

    assert_eq!(
        store
            .decision_history("user-004", &legal_user("user-004"))
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn missing_required_documents_block_access() {
    let store = LegalStore::open_in_memory().unwrap();

    store
        .register_user("user-005", &legal_user("user-005"))
        .unwrap();

    assert!(!store
        .is_access_allowed("user-005", "matter-base", "es", &legal_user("user-005"))
        .unwrap());

    assert!(!store
        .has_required_documents("matter-base", "es", &legal_admin())
        .unwrap());
}

#[test]
fn duplicate_document_with_same_metadata_is_idempotent() {
    let store = LegalStore::open_in_memory().unwrap();
    let document = create_terms();

    let first = store.register_document(&document, &legal_admin()).unwrap();
    let second = store.register_document(&document, &legal_admin()).unwrap();

    assert_eq!(first, second);
}

#[test]
fn duplicate_document_id_with_different_version_is_rejected() {
    let store = LegalStore::open_in_memory().unwrap();

    store
        .register_document(&create_terms(), &legal_admin())
        .unwrap();

    let mut changed = create_terms();
    changed.version = "2.0.0".to_string();

    assert!(store.register_document(&changed, &legal_admin()).is_err());
}

#[test]
fn duplicate_document_id_with_different_content_is_rejected() {
    let store = LegalStore::open_in_memory().unwrap();

    store
        .register_document(&create_terms(), &legal_admin())
        .unwrap();

    let mut changed = create_terms();
    changed.content = "Contenido modificado".to_string();

    assert!(store.register_document(&changed, &legal_admin()).is_err());
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

    assert!(store.register_document(&document, &legal_admin()).is_err());
}

#[test]
fn invalid_decision_for_document_type_is_rejected() {
    let store = LegalStore::open_in_memory().unwrap();

    store
        .register_user("user-006", &legal_user("user-006"))
        .unwrap();
    store
        .register_document(&create_terms(), &legal_admin())
        .unwrap();

    assert!(store
        .record_decision(
            "user-006",
            "base-terms-1.0.0-es",
            ConsentDecision::Granted,
            None,
            &legal_user("user-006")
        )
        .is_err());
}

#[test]
fn privacy_notice_requires_acknowledgement() {
    let store = LegalStore::open_in_memory().unwrap();

    store
        .register_user("user-007", &legal_user("user-007"))
        .unwrap();

    let privacy = create_document(
        "privacy-1.0.0-es",
        LegalDocumentType::PrivacyNotice,
        true,
        ConsentDecision::Acknowledged,
    );

    store.register_document(&privacy, &legal_admin()).unwrap();

    assert!(store
        .record_decision(
            "user-007",
            "privacy-1.0.0-es",
            ConsentDecision::Accepted,
            None,
            &legal_user("user-007")
        )
        .is_err());

    store
        .record_decision(
            "user-007",
            "privacy-1.0.0-es",
            ConsentDecision::Acknowledged,
            None,
            &legal_user("user-007"),
        )
        .unwrap();
}

#[test]
fn optional_consent_does_not_block_general_access() {
    let store = LegalStore::open_in_memory().unwrap();

    store
        .register_user("user-008", &legal_user("user-008"))
        .unwrap();
    store
        .register_document(&create_terms(), &legal_admin())
        .unwrap();

    let ai_consent = create_document(
        "ai-training-1.0.0-es",
        LegalDocumentType::Consent,
        false,
        ConsentDecision::Granted,
    );

    store
        .register_document(&ai_consent, &legal_admin())
        .unwrap();

    store
        .record_decision(
            "user-008",
            "base-terms-1.0.0-es",
            ConsentDecision::Accepted,
            None,
            &legal_user("user-008"),
        )
        .unwrap();

    store
        .record_decision(
            "user-008",
            "ai-training-1.0.0-es",
            ConsentDecision::Declined,
            None,
            &legal_user("user-008"),
        )
        .unwrap();

    assert!(store
        .is_access_allowed("user-008", "matter-base", "es", &legal_user("user-008"))
        .unwrap());
}

#[test]
fn consent_can_be_withdrawn_after_grant() {
    let store = LegalStore::open_in_memory().unwrap();

    store
        .register_user("user-009", &legal_user("user-009"))
        .unwrap();

    let consent = create_document(
        "ai-training-1.0.0-es",
        LegalDocumentType::Consent,
        false,
        ConsentDecision::Granted,
    );

    store.register_document(&consent, &legal_admin()).unwrap();

    assert!(store
        .record_decision(
            "user-009",
            "ai-training-1.0.0-es",
            ConsentDecision::Withdrawn,
            None,
            &legal_user("user-009")
        )
        .is_err());

    store
        .record_decision(
            "user-009",
            "ai-training-1.0.0-es",
            ConsentDecision::Granted,
            None,
            &legal_user("user-009"),
        )
        .unwrap();

    store
        .record_decision(
            "user-009",
            "ai-training-1.0.0-es",
            ConsentDecision::Withdrawn,
            None,
            &legal_user("user-009"),
        )
        .unwrap();

    let latest = store
        .latest_decision("user-009", "ai-training-1.0.0-es", &legal_user("user-009"))
        .unwrap()
        .unwrap();

    assert_eq!(latest.decision, "withdrawn");
    assert_eq!(
        store
            .decision_history("user-009", &legal_user("user-009"))
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn withdrawn_consent_cannot_be_withdrawn_twice() {
    let store = LegalStore::open_in_memory().unwrap();

    store
        .register_user("user-010", &legal_user("user-010"))
        .unwrap();

    let consent = create_document(
        "ai-training-2.0.0-es",
        LegalDocumentType::Consent,
        false,
        ConsentDecision::Granted,
    );

    store.register_document(&consent, &legal_admin()).unwrap();

    store
        .record_decision(
            "user-010",
            "ai-training-2.0.0-es",
            ConsentDecision::Granted,
            None,
            &legal_user("user-010"),
        )
        .unwrap();

    store
        .record_decision(
            "user-010",
            "ai-training-2.0.0-es",
            ConsentDecision::Withdrawn,
            None,
            &legal_user("user-010"),
        )
        .unwrap();

    assert!(store
        .record_decision(
            "user-010",
            "ai-training-2.0.0-es",
            ConsentDecision::Withdrawn,
            None,
            &legal_user("user-010")
        )
        .is_err());
}

#[test]
fn latest_decision_returns_last_record() {
    let store = LegalStore::open_in_memory().unwrap();

    store
        .register_user("user-011", &legal_user("user-011"))
        .unwrap();
    store
        .register_document(&create_terms(), &legal_admin())
        .unwrap();

    store
        .record_decision(
            "user-011",
            "base-terms-1.0.0-es",
            ConsentDecision::Accepted,
            Some("web"),
            &legal_user("user-011"),
        )
        .unwrap();

    store
        .record_decision(
            "user-011",
            "base-terms-1.0.0-es",
            ConsentDecision::Declined,
            Some("mobile"),
            &legal_user("user-011"),
        )
        .unwrap();

    let latest = store
        .latest_decision("user-011", "base-terms-1.0.0-es", &legal_user("user-011"))
        .unwrap()
        .unwrap();

    assert_eq!(latest.decision, "declined");
    assert_eq!(latest.platform.as_deref(), Some("mobile"));
}

#[test]
fn unregistered_user_cannot_record_decisions() {
    let store = LegalStore::open_in_memory().unwrap();

    store
        .register_document(&create_terms(), &legal_admin())
        .unwrap();

    assert!(store
        .record_decision(
            "unknown-user",
            "base-terms-1.0.0-es",
            ConsentDecision::Accepted,
            None,
            &legal_user("unknown-user")
        )
        .is_err());
}

#[test]
fn document_can_be_deactivated() {
    let store = LegalStore::open_in_memory().unwrap();

    store
        .register_user("user-012", &legal_user("user-012"))
        .unwrap();
    store
        .register_document(&create_terms(), &legal_admin())
        .unwrap();

    store
        .set_document_active("base-terms-1.0.0-es", false, &legal_admin())
        .unwrap();

    assert!(!store
        .has_required_documents("matter-base", "es", &legal_admin())
        .unwrap());

    assert!(store
        .record_decision(
            "user-012",
            "base-terms-1.0.0-es",
            ConsentDecision::Accepted,
            None,
            &legal_user("user-012")
        )
        .is_err());
}

#[test]
fn deactivated_document_can_be_reactivated() {
    let store = LegalStore::open_in_memory().unwrap();

    store
        .register_document(&create_terms(), &legal_admin())
        .unwrap();

    store
        .set_document_active("base-terms-1.0.0-es", false, &legal_admin())
        .unwrap();

    store
        .set_document_active("base-terms-1.0.0-es", true, &legal_admin())
        .unwrap();

    assert!(store
        .has_required_documents("matter-base", "es", &legal_admin())
        .unwrap());
}

#[test]
fn latest_decision_is_none_when_no_decision_exists() {
    let store = LegalStore::open_in_memory().unwrap();

    store
        .register_user("user-014", &legal_user("user-014"))
        .unwrap();
    store
        .register_document(&create_terms(), &legal_admin())
        .unwrap();

    assert!(store
        .latest_decision("user-014", "base-terms-1.0.0-es", &legal_user("user-014"))
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
        first.register_document(&document_a, &legal_admin())
    });
    let second_barrier = Arc::clone(&barrier);
    let second_thread = thread::spawn(move || {
        second_barrier.wait();
        second.register_document(&document_b, &legal_admin())
    });

    barrier.wait();
    assert!(first_thread.join().unwrap().is_ok());
    assert!(second_thread.join().unwrap().is_ok());
    let verification = LegalStore::open(&database).unwrap();
    assert!(verification
        .register_document(&create_terms(), &legal_admin())
        .is_ok());
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
    store
        .register_user("interrupted-user", &legal_user("interrupted-user"))
        .unwrap();
    store
        .register_document(&create_terms(), &legal_admin())
        .unwrap();

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
            &legal_user("interrupted-user")
        )
        .is_err());
    assert!(store
        .decision_history("interrupted-user", &legal_user("interrupted-user"))
        .unwrap()
        .is_empty());
    drop(external_connection);
    drop(store);
    std::fs::remove_dir_all(directory).unwrap();
}
