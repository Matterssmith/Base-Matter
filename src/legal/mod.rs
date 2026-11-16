/*!
 * Módulo: Legal & Consent Manager
 *
 * Función:
 * Administrar documentos legales, usuarios y decisiones de consentimiento.
 *
 * Responsabilidades:
 * - Registrar usuarios.
 * - Registrar documentos y versiones inmutables.
 * - Calcular huellas SHA-256.
 * - Validar decisiones según el tipo de documento.
 * - Conservar el historial de decisiones.
 * - Administrar consentimientos opcionales.
 * - Verificar requisitos legales de acceso.
 *
 * Seguridad:
 * - No almacena credenciales de plataformas.
 * - Delega acciones sensibles a LegalAuthorizer por principal y recurso.
 * - No autentica por sí mismo identidades de plataformas.
 * - Impide UPDATE y DELETE ordinarios del historial.
 * - Protege el contenido y los metadatos de documentos.
 * - Permite cambiar únicamente el estado activo de un documento.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::{anyhow, bail, Result};
use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

const LEGAL_STORE_SCHEMA_VERSION: i64 = 1;

/// Sensitive legal-store operation delegated to the consuming application's
/// authenticated identity and authorization layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegalAction {
    RegisterUser,
    ReadUser,
    RegisterDocument,
    SetDocumentActive,
    RecordDecision,
    ReadDecision,
    CheckAccess,
    ReadRequiredDocuments,
}

/// Resource details presented to an application authorization policy.
#[derive(Debug, Clone, Copy)]
pub struct LegalAuthorizationRequest<'a> {
    pub principal_id: &'a str,
    pub action: LegalAction,
    pub user_id: Option<&'a str>,
    pub document_id: Option<&'a str>,
    pub scope: Option<&'a str>,
}

/// Application policy that authenticates a principal and authorizes legal
/// operations. Matter-Base does not authenticate external platform identities.
pub trait LegalAuthorizer: Send + Sync {
    fn authorize(&self, request: &LegalAuthorizationRequest<'_>) -> Result<()>;
}

/// Authenticated principal supplied by the consuming application for a legal
/// operation. The authorizer must verify this ID against its trusted identity
/// context and enforce the requested action and resource.
pub struct LegalPrincipal<'a> {
    id: &'a str,
    authorizer: &'a dyn LegalAuthorizer,
}

impl<'a> LegalPrincipal<'a> {
    pub fn new(id: &'a str, authorizer: &'a dyn LegalAuthorizer) -> Result<Self> {
        validate_identifier(id, "principal_id")?;
        Ok(Self { id, authorizer })
    }

    fn authorize(
        &self,
        action: LegalAction,
        user_id: Option<&str>,
        document_id: Option<&str>,
        scope: Option<&str>,
    ) -> Result<()> {
        self.authorizer
            .authorize(&LegalAuthorizationRequest {
                principal_id: self.id,
                action,
                user_id,
                document_id,
                scope,
            })
            .map_err(|_| anyhow!("Legal authorization denied"))
    }
}

fn validate_identifier(value: &str, name: &str) -> Result<()> {
    if value.trim().is_empty() || value.len() > 128 || value.chars().any(char::is_control) {
        bail!("Invalid {name}");
    }
    Ok(())
}

/// Tipo de documento administrado por el sistema legal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LegalDocumentType {
    Terms,
    PrivacyNotice,
    Consent,
    Other,
}

impl LegalDocumentType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Terms => "terms",
            Self::PrivacyNotice => "privacy_notice",
            Self::Consent => "consent",
            Self::Other => "other",
        }
    }

    fn from_str(value: &str) -> Result<Self> {
        match value {
            "terms" => Ok(Self::Terms),
            "privacy_notice" => Ok(Self::PrivacyNotice),
            "consent" => Ok(Self::Consent),
            "other" => Ok(Self::Other),
            _ => bail!("Unknown legal document type"),
        }
    }
}

/// Decisión registrada por un usuario.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsentDecision {
    Accepted,
    Declined,
    Acknowledged,
    Granted,
    Withdrawn,
}

impl ConsentDecision {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Declined => "declined",
            Self::Acknowledged => "acknowledged",
            Self::Granted => "granted",
            Self::Withdrawn => "withdrawn",
        }
    }
}

/// Documento legal registrado.
#[derive(Debug, Clone)]
pub struct LegalDocument {
    pub document_id: String,
    pub scope: String,
    pub document_type: LegalDocumentType,
    pub version: String,
    pub locale: String,
    pub content: String,
    pub mandatory: bool,
    pub required_decision: ConsentDecision,
    pub active: bool,
}

/// Registro histórico de una decisión.
#[derive(Debug, Clone)]
pub struct ConsentRecord {
    pub record_id: i64,
    pub user_id: String,
    pub document_id: String,
    pub document_hash: String,
    pub decision: String,
    pub platform: Option<String>,
    pub created_at: i64,
}

/// Almacén persistente del registro legal.
#[derive(Clone)]
pub struct LegalStore {
    connection: Arc<Mutex<Connection>>,
}

impl LegalStore {
    /// Abre o crea una base de datos legal.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }

        let connection = Connection::open(path)?;

        let store = Self {
            connection: Arc::new(Mutex::new(connection)),
        };

        store.initialize()?;
        Ok(store)
    }

    /// Crea un almacén temporal para pruebas.
    pub fn open_in_memory() -> Result<Self> {
        let connection = Connection::open_in_memory()?;

        let store = Self {
            connection: Arc::new(Mutex::new(connection)),
        };

        store.initialize()?;
        Ok(store)
    }

    /// Inicializa tablas, índices y protecciones.
    ///
    /// Las tablas existentes se conservan. Los triggers se agregan
    /// mediante IF NOT EXISTS para permitir inicializaciones repetidas.
    /// La creación y el versionado del esquema son una sola transacción.
    fn initialize(&self) -> Result<()> {
        let mut connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("LegalStore lock poisoned"))?;

        // SQLite only changes foreign-key enforcement outside a transaction.
        connection.execute_batch("PRAGMA foreign_keys = ON;")?;
        let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        transaction.execute_batch(
            "CREATE TABLE IF NOT EXISTS matter_schema_versions (
                 component TEXT PRIMARY KEY NOT NULL,
                 version INTEGER NOT NULL
             );",
        )?;
        let current_version: Option<i64> = transaction
            .query_row(
                "SELECT version FROM matter_schema_versions WHERE component = ?1",
                params!["legal_store"],
                |row| row.get(0),
            )
            .optional()?;
        if current_version
            .is_some_and(|version| !(0..=LEGAL_STORE_SCHEMA_VERSION).contains(&version))
        {
            bail!("Legal store schema version is invalid or newer than this runtime supports");
        }

        transaction.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS legal_users (
                user_id TEXT PRIMARY KEY,
                created_at INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS legal_documents (
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
                UNIQUE (
                    scope,
                    document_type,
                    version,
                    locale
                )
            );

            CREATE TABLE IF NOT EXISTS legal_decisions (
                record_id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id TEXT NOT NULL,
                document_id TEXT NOT NULL,
                document_hash TEXT NOT NULL,
                decision TEXT NOT NULL,
                platform TEXT,
                created_at INTEGER NOT NULL,

                FOREIGN KEY (user_id)
                    REFERENCES legal_users(user_id),

                FOREIGN KEY (document_id)
                    REFERENCES legal_documents(document_id)
            );

            CREATE INDEX IF NOT EXISTS idx_legal_decisions_user
                ON legal_decisions(user_id);

            CREATE INDEX IF NOT EXISTS idx_legal_decisions_document
                ON legal_decisions(document_id);

            CREATE INDEX IF NOT EXISTS idx_legal_documents_scope
                ON legal_documents(scope, locale, active);

            CREATE TRIGGER IF NOT EXISTS legal_decisions_no_update
            BEFORE UPDATE ON legal_decisions
            BEGIN
                SELECT RAISE(
                    ABORT,
                    'Legal decision history is immutable'
                );
            END;

            CREATE TRIGGER IF NOT EXISTS legal_decisions_no_delete
            BEFORE DELETE ON legal_decisions
            BEGIN
                SELECT RAISE(
                    ABORT,
                    'Legal decision history is immutable'
                );
            END;

            CREATE TRIGGER IF NOT EXISTS legal_documents_protect_metadata
            BEFORE UPDATE ON legal_documents
            WHEN
                NEW.document_id IS NOT OLD.document_id
                OR NEW.scope IS NOT OLD.scope
                OR NEW.document_type IS NOT OLD.document_type
                OR NEW.version IS NOT OLD.version
                OR NEW.locale IS NOT OLD.locale
                OR NEW.content IS NOT OLD.content
                OR NEW.content_hash IS NOT OLD.content_hash
                OR NEW.mandatory IS NOT OLD.mandatory
                OR NEW.required_decision IS NOT OLD.required_decision
                OR NEW.created_at IS NOT OLD.created_at
            BEGIN
                SELECT RAISE(
                    ABORT,
                    'Registered legal document metadata is immutable'
                );
            END;

            CREATE TRIGGER IF NOT EXISTS legal_documents_no_delete
            BEFORE DELETE ON legal_documents
            BEGIN
                SELECT RAISE(
                    ABORT,
                    'Registered legal documents cannot be deleted'
                );
            END;
            ",
        )?;

        transaction.execute(
            "INSERT INTO matter_schema_versions (component, version)
             VALUES (?1, ?2)
             ON CONFLICT(component) DO UPDATE SET version = excluded.version",
            params!["legal_store", LEGAL_STORE_SCHEMA_VERSION],
        )?;
        transaction.commit()?;

        Ok(())
    }

    /// Valida la decisión requerida por el tipo de documento.
    fn validate_required_decision(
        document_type: &LegalDocumentType,
        decision: &ConsentDecision,
    ) -> Result<()> {
        let valid = matches!(
            (document_type, decision),
            (LegalDocumentType::Terms, ConsentDecision::Accepted)
                | (
                    LegalDocumentType::PrivacyNotice,
                    ConsentDecision::Acknowledged
                )
                | (LegalDocumentType::Consent, ConsentDecision::Granted)
                | (LegalDocumentType::Other, ConsentDecision::Accepted)
                | (LegalDocumentType::Other, ConsentDecision::Acknowledged)
        );

        if !valid {
            bail!("Invalid required decision for document type");
        }

        Ok(())
    }

    /// Valida una decisión enviada por el usuario.
    fn validate_decision(
        document_type: &LegalDocumentType,
        decision: &ConsentDecision,
    ) -> Result<()> {
        let valid = match document_type {
            LegalDocumentType::Terms => matches!(
                decision,
                ConsentDecision::Accepted | ConsentDecision::Declined
            ),
            LegalDocumentType::PrivacyNotice => matches!(
                decision,
                ConsentDecision::Acknowledged | ConsentDecision::Declined
            ),
            LegalDocumentType::Consent => matches!(
                decision,
                ConsentDecision::Granted | ConsentDecision::Declined | ConsentDecision::Withdrawn
            ),
            LegalDocumentType::Other => matches!(
                decision,
                ConsentDecision::Accepted
                    | ConsentDecision::Declined
                    | ConsentDecision::Acknowledged
            ),
        };

        if !valid {
            bail!("Decision is not valid for this document type");
        }

        Ok(())
    }

    /// Registra un usuario sin duplicar su identificador.
    pub fn register_user(&self, user_id: &str, principal: &LegalPrincipal<'_>) -> Result<()> {
        validate_identifier(user_id, "user_id")?;
        principal.authorize(LegalAction::RegisterUser, Some(user_id), None, None)?;

        let connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("LegalStore lock poisoned"))?;

        connection.execute(
            "
            INSERT OR IGNORE INTO legal_users (
                user_id,
                created_at
            ) VALUES (?1, ?2)
            ",
            params![user_id, Self::timestamp()],
        )?;

        Ok(())
    }

    /// Comprueba si un usuario está registrado.
    pub fn user_exists(&self, user_id: &str, principal: &LegalPrincipal<'_>) -> Result<bool> {
        validate_identifier(user_id, "user_id")?;
        principal.authorize(LegalAction::ReadUser, Some(user_id), None, None)?;
        let connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("LegalStore lock poisoned"))?;

        let exists = connection.query_row(
            "
            SELECT EXISTS(
                SELECT 1
                FROM legal_users
                WHERE user_id = ?1
            )
            ",
            params![user_id],
            |row| row.get(0),
        )?;

        Ok(exists)
    }

    /// Registra un documento y calcula su huella SHA-256.
    ///
    /// Un identificador existente no puede reutilizarse con
    /// contenido o metadatos diferentes.
    pub fn register_document(
        &self,
        document: &LegalDocument,
        principal: &LegalPrincipal<'_>,
    ) -> Result<String> {
        if document.document_id.trim().is_empty()
            || document.scope.trim().is_empty()
            || document.version.trim().is_empty()
            || document.locale.trim().is_empty()
            || document.content.trim().is_empty()
        {
            bail!("Document fields cannot be empty");
        }

        Self::validate_required_decision(&document.document_type, &document.required_decision)?;
        principal.authorize(
            LegalAction::RegisterDocument,
            None,
            Some(&document.document_id),
            Some(&document.scope),
        )?;

        let hash = Self::hash_content(&document.content);

        let mut connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("LegalStore lock poisoned"))?;
        let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;

        let existing: Option<(
            String,
            String,
            String,
            String,
            String,
            String,
            i64,
            String,
            i64,
        )> = transaction
            .query_row(
                "
            SELECT
                scope,
                document_type,
                version,
                locale,
                content,
                content_hash,
                mandatory,
                required_decision,
                active
            FROM legal_documents
            WHERE document_id = ?1
            ",
                params![document.document_id],
                |row| {
                    Ok((
                        row.get(0)?,
                        row.get(1)?,
                        row.get(2)?,
                        row.get(3)?,
                        row.get(4)?,
                        row.get(5)?,
                        row.get(6)?,
                        row.get(7)?,
                        row.get(8)?,
                    ))
                },
            )
            .optional()?;

        if let Some(existing) = existing {
            let expected_mandatory = document.mandatory as i64;
            let expected_active = document.active as i64;

            let matches = existing.0 == document.scope
                && existing.1 == document.document_type.as_str()
                && existing.2 == document.version
                && existing.3 == document.locale
                && existing.4 == document.content
                && existing.5 == hash
                && existing.6 == expected_mandatory
                && existing.7 == document.required_decision.as_str()
                && existing.8 == expected_active;

            if !matches {
                bail!("Document ID already exists with different metadata or content");
            }

            transaction.commit()?;
            return Ok(hash);
        }

        transaction.execute(
            "
            INSERT INTO legal_documents (
                document_id,
                scope,
                document_type,
                version,
                locale,
                content,
                content_hash,
                mandatory,
                required_decision,
                active,
                created_at
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5,
                ?6, ?7, ?8, ?9, ?10, ?11
            )
            ",
            params![
                document.document_id,
                document.scope,
                document.document_type.as_str(),
                document.version,
                document.locale,
                document.content,
                hash,
                document.mandatory,
                document.required_decision.as_str(),
                document.active,
                Self::timestamp(),
            ],
        )?;

        transaction.commit()?;

        Ok(hash)
    }

    /// Cambia únicamente el estado activo de un documento.
    ///
    /// El contenido, la versión y los metadatos permanecen inmutables.
    pub fn set_document_active(
        &self,
        document_id: &str,
        active: bool,
        principal: &LegalPrincipal<'_>,
    ) -> Result<()> {
        validate_identifier(document_id, "document_id")?;
        principal.authorize(
            LegalAction::SetDocumentActive,
            None,
            Some(document_id),
            None,
        )?;

        let connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("LegalStore lock poisoned"))?;

        let changed = connection.execute(
            "
            UPDATE legal_documents
            SET active = ?1
            WHERE document_id = ?2
            ",
            params![active, document_id],
        )?;

        if changed == 0 {
            bail!("Document not found");
        }

        Ok(())
    }

    /// Registra una decisión nueva sin modificar el historial.
    pub fn record_decision(
        &self,
        user_id: &str,
        document_id: &str,
        decision: ConsentDecision,
        platform: Option<&str>,
        principal: &LegalPrincipal<'_>,
    ) -> Result<()> {
        validate_identifier(user_id, "user_id")?;
        validate_identifier(document_id, "document_id")?;

        if platform.is_some_and(|value| value.trim().is_empty()) {
            bail!("Platform cannot be empty");
        }
        principal.authorize(
            LegalAction::RecordDecision,
            Some(user_id),
            Some(document_id),
            None,
        )?;

        let mut connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("LegalStore lock poisoned"))?;
        let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;

        let user_exists: bool = transaction.query_row(
            "
            SELECT EXISTS(
                SELECT 1
                FROM legal_users
                WHERE user_id = ?1
            )
            ",
            params![user_id],
            |row| row.get(0),
        )?;

        if !user_exists {
            bail!("User is not registered");
        }

        let document: Option<(String, String)> = transaction
            .query_row(
                "
            SELECT content_hash, document_type
            FROM legal_documents
            WHERE document_id = ?1
              AND active = 1
            ",
                params![document_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;

        let (document_hash, document_type) =
            document.ok_or_else(|| anyhow!("Document not found or inactive"))?;

        let document_type = LegalDocumentType::from_str(&document_type)?;

        Self::validate_decision(&document_type, &decision)?;

        if decision == ConsentDecision::Withdrawn {
            let previous: Option<String> = transaction
                .query_row(
                    "
                SELECT decision
                FROM legal_decisions
                WHERE user_id = ?1
                  AND document_id = ?2
                ORDER BY record_id DESC
                LIMIT 1
                ",
                    params![user_id, document_id],
                    |row| row.get(0),
                )
                .optional()?;

            if previous.as_deref() != Some("granted") {
                bail!("Consent cannot be withdrawn without a previous grant");
            }
        }

        transaction.execute(
            "
            INSERT INTO legal_decisions (
                user_id,
                document_id,
                document_hash,
                decision,
                platform,
                created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            ",
            params![
                user_id,
                document_id,
                document_hash,
                decision.as_str(),
                platform,
                Self::timestamp(),
            ],
        )?;

        transaction.commit()?;

        Ok(())
    }

    /// Comprueba si existen documentos obligatorios activos.
    pub fn has_required_documents(
        &self,
        scope: &str,
        locale: &str,
        principal: &LegalPrincipal<'_>,
    ) -> Result<bool> {
        principal.authorize(LegalAction::ReadRequiredDocuments, None, None, Some(scope))?;
        let connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("LegalStore lock poisoned"))?;

        let count: i64 = connection.query_row(
            "
            SELECT COUNT(*)
            FROM legal_documents
            WHERE scope = ?1
              AND locale = ?2
              AND mandatory = 1
              AND active = 1
            ",
            params![scope, locale],
            |row| row.get(0),
        )?;

        Ok(count > 0)
    }

    /// Comprueba si se cumplen todos los documentos obligatorios.
    ///
    /// El usuario y todos los requisitos se evalúan dentro de una única
    /// sentencia SQLite para observar una instantánea coherente.
    /// Si no hay documentos obligatorios activos, devuelve false.
    pub fn is_access_allowed(
        &self,
        user_id: &str,
        scope: &str,
        locale: &str,
        principal: &LegalPrincipal<'_>,
    ) -> Result<bool> {
        validate_identifier(user_id, "user_id")?;
        principal.authorize(LegalAction::CheckAccess, Some(user_id), None, Some(scope))?;
        let connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("LegalStore lock poisoned"))?;

        let (user_exists, required_count, accepted_count): (bool, i64, i64) = connection
            .query_row(
                "
            SELECT
                EXISTS(
                    SELECT 1
                    FROM legal_users
                    WHERE user_id = ?3
                ),
                COUNT(*),
                COALESCE(SUM(CASE WHEN
                    EXISTS (
                        SELECT 1
                        FROM legal_decisions AS decisions
                        WHERE decisions.user_id = ?3
                          AND decisions.document_id = documents.document_id
                          AND decisions.document_hash = documents.content_hash
                          AND decisions.decision = documents.required_decision
                          AND decisions.record_id = (
                              SELECT MAX(latest.record_id)
                              FROM legal_decisions AS latest
                              WHERE latest.user_id = ?3
                                AND latest.document_id = documents.document_id
                          )
                    )
                THEN 1 ELSE 0 END), 0)
            FROM legal_documents AS documents
            WHERE documents.scope = ?1
              AND documents.locale = ?2
              AND documents.mandatory = 1
              AND documents.active = 1
            ",
                params![scope, locale, user_id],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )?;

        Ok(user_exists && required_count > 0 && accepted_count == required_count)
    }

    /// Recupera la última decisión de un usuario sobre un documento.
    pub fn latest_decision(
        &self,
        user_id: &str,
        document_id: &str,
        principal: &LegalPrincipal<'_>,
    ) -> Result<Option<ConsentRecord>> {
        validate_identifier(user_id, "user_id")?;
        validate_identifier(document_id, "document_id")?;
        principal.authorize(
            LegalAction::ReadDecision,
            Some(user_id),
            Some(document_id),
            None,
        )?;
        let connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("LegalStore lock poisoned"))?;

        let record = connection
            .query_row(
                "
            SELECT
                record_id,
                user_id,
                document_id,
                document_hash,
                decision,
                platform,
                created_at
            FROM legal_decisions
            WHERE user_id = ?1
              AND document_id = ?2
            ORDER BY record_id DESC
            LIMIT 1
            ",
                params![user_id, document_id],
                |row| {
                    Ok(ConsentRecord {
                        record_id: row.get(0)?,
                        user_id: row.get(1)?,
                        document_id: row.get(2)?,
                        document_hash: row.get(3)?,
                        decision: row.get(4)?,
                        platform: row.get(5)?,
                        created_at: row.get(6)?,
                    })
                },
            )
            .optional()?;

        Ok(record)
    }

    /// Recupera el historial completo de decisiones de un usuario.
    pub fn decision_history(
        &self,
        user_id: &str,
        principal: &LegalPrincipal<'_>,
    ) -> Result<Vec<ConsentRecord>> {
        validate_identifier(user_id, "user_id")?;
        principal.authorize(LegalAction::ReadDecision, Some(user_id), None, None)?;
        let connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("LegalStore lock poisoned"))?;

        let mut statement = connection.prepare(
            "
            SELECT
                record_id,
                user_id,
                document_id,
                document_hash,
                decision,
                platform,
                created_at
            FROM legal_decisions
            WHERE user_id = ?1
            ORDER BY record_id ASC
            ",
        )?;

        let rows = statement.query_map(params![user_id], |row| {
            Ok(ConsentRecord {
                record_id: row.get(0)?,
                user_id: row.get(1)?,
                document_id: row.get(2)?,
                document_hash: row.get(3)?,
                decision: row.get(4)?,
                platform: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;

        let mut records = Vec::new();

        for record in rows {
            records.push(record?);
        }

        Ok(records)
    }

    /// Calcula la huella SHA-256 del contenido.
    fn hash_content(content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());

        format!("sha256:{}", hex::encode(hasher.finalize()))
    }

    /// Obtiene la fecha actual como Unix timestamp UTC.
    fn timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64
    }
}
