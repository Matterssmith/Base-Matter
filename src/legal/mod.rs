
/*!
 * Módulo: Legal & Consent Manager
 *
 * Función:
 * Administrar documentos legales, usuarios y decisiones de consentimiento.
 *
 * Responsabilidades:
 * - Registrar usuarios mediante identificadores internos.
 * - Registrar documentos y sus versiones.
 * - Calcular y verificar huellas SHA-256.
 * - Registrar aceptaciones, rechazos y retiradas.
 * - Conservar el historial de decisiones.
 * - Verificar los requisitos legales de acceso.
 *
 * Seguridad:
 * - No almacena credenciales de plataformas.
 * - No elimina el historial de decisiones.
 * - Mantiene separados los consentimientos opcionales.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::{anyhow, bail, Result};
use rusqlite::{params, Connection, OptionalExtension};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

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
}

/// Decisión requerida para considerar cumplido un documento obligatorio.
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

/// Documento legal registrado en Matter-Base.
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

/// Registro histórico de una decisión del usuario.
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
            fs::create_dir_all(parent)?;
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

    /// Inicializa las tablas del registro legal.
    fn initialize(&self) -> Result<()> {
        let connection = self.connection.lock()
            .map_err(|_| anyhow!("LegalStore lock poisoned"))?;

        connection.execute_batch(
            "
            PRAGMA foreign_keys = ON;

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
            "
        )?;

        Ok(())
    }

    /// Registra un usuario sin duplicar su identificador.
    pub fn register_user(&self, user_id: &str) -> Result<()> {
        if user_id.trim().is_empty() {
            bail!("User ID cannot be empty");
        }

        let connection = self.connection.lock()
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

    /// Registra un documento y calcula su huella SHA-256.
    ///
    /// Una versión ya registrada no puede modificarse.
    pub fn register_document(
        &self,
        document: &LegalDocument,
    ) -> Result<String> {
        if document.document_id.trim().is_empty()
            || document.scope.trim().is_empty()
            || document.version.trim().is_empty()
            || document.locale.trim().is_empty()
            || document.content.trim().is_empty()
        {
            bail!("Document fields cannot be empty");
        }

        let hash = Self::hash_content(&document.content);

        let connection = self.connection.lock()
            .map_err(|_| anyhow!("LegalStore lock poisoned"))?;

        let existing: Option<String> = connection.query_row(
            "
            SELECT content_hash
            FROM legal_documents
            WHERE document_id = ?1
            ",
            params![document.document_id],
            |row| row.get(0),
        ).optional()?;

        if let Some(existing_hash) = existing {
            if existing_hash != hash {
                bail!(
                    "Document ID already exists with different content"
                );
            }

            return Ok(hash);
        }

        connection.execute(
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

        Ok(hash)
    }

    /// Registra una decisión del usuario sobre un documento.
    ///
    /// Cada decisión genera un nuevo registro histórico.
    pub fn record_decision(
        &self,
        user_id: &str,
        document_id: &str,
        decision: ConsentDecision,
        platform: Option<&str>,
    ) -> Result<()> {
        let connection = self.connection.lock()
            .map_err(|_| anyhow!("LegalStore lock poisoned"))?;

        let user_exists: bool = connection.query_row(
            "
            SELECT EXISTS(
                SELECT 1 FROM legal_users WHERE user_id = ?1
            )
            ",
            params![user_id],
            |row| row.get(0),
        )?;

        if !user_exists {
            bail!("User is not registered");
        }

        let document_hash: Option<String> = connection.query_row(
            "
            SELECT content_hash
            FROM legal_documents
            WHERE document_id = ?1 AND active = 1
            ",
            params![document_id],
            |row| row.get(0),
        ).optional()?;

        let document_hash = document_hash
            .ok_or_else(|| anyhow!("Document not found or inactive"))?;

        connection.execute(
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

        Ok(())
    }

    /// Comprueba si existen documentos obligatorios para un alcance.
    pub fn has_required_documents(
        &self,
        scope: &str,
        locale: &str,
    ) -> Result<bool> {
        let connection = self.connection.lock()
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

    /// Verifica que el usuario haya cumplido todos los documentos
    /// obligatorios activos de un alcance y un idioma.
    ///
    /// Si no existen documentos obligatorios, devuelve false.
    pub fn is_access_allowed(
        &self,
        user_id: &str,
        scope: &str,
        locale: &str,
    ) -> Result<bool> {
        let connection = self.connection.lock()
            .map_err(|_| anyhow!("LegalStore lock poisoned"))?;

        let user_exists: bool = connection.query_row(
            "
            SELECT EXISTS(
                SELECT 1 FROM legal_users WHERE user_id = ?1
            )
            ",
            params![user_id],
            |row| row.get(0),
        )?;

        if !user_exists {
            return Ok(false);
        }

        let required_count: i64 = connection.query_row(
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

        if required_count == 0 {
            return Ok(false);
        }

        let accepted_count: i64 = connection.query_row(
            "
            SELECT COUNT(*)
            FROM legal_documents AS documents
            WHERE documents.scope = ?1
              AND documents.locale = ?2
              AND documents.mandatory = 1
              AND documents.active = 1
              AND EXISTS (
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
            ",
            params![scope, locale, user_id],
            |row| row.get(0),
        )?;

        Ok(accepted_count == required_count)
    }

    /// Recupera el historial completo de decisiones de un usuario.
    pub fn decision_history(
        &self,
        user_id: &str,
    ) -> Result<Vec<ConsentRecord>> {
        let connection = self.connection.lock()
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
            "
        )?;

        let rows = statement.query_map(
            params![user_id],
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
        )?;

        let mut records = Vec::new();

        for record in rows {
            records.push(record?);
        }

        Ok(records)
    }

    /// Calcula la huella SHA-256 del contenido de un documento.
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
