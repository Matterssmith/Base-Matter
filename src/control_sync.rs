/*
 * Matter-Base — Encrypted control-record outbox.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use crate::authorization::RoleStore;
use crate::security::encryption::{self, EncryptedEnvelope};
use crate::security::key_manager::KeyManager;
use anyhow::{anyhow, bail, Result};
use hmac::{Hmac, Mac};
use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use sha2::Sha256;
use std::path::Path;
use std::sync::Mutex;
use std::time::Duration;
use zeroize::Zeroizing;

const OUTBOX_SCHEMA: i64 = 1;
const MAX_FIELD_BYTES: usize = 128;

/// Remote writer accepts only already-authenticated ciphertext envelopes.
/// Implementations must use compare-and-swap revisions and deduplicate
/// retries of the same envelope_id; secrets/tokens belong outside this trait.
pub trait EncryptedEnvelopeSink: Send + Sync {
    fn put(&self, envelope: &EncryptedEnvelope, expected_revision: Option<&str>) -> Result<String>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutboxStatus {
    pub sequence: i64,
    pub synchronized: bool,
    pub remote_revision: Option<String>,
}

/// Durable local outbox for changed encrypted control-plane records.
/// Never logs/stores plaintext or key material, and never talks to GitHub
/// without a caller-installed authenticated sink.
pub struct EncryptedControlOutbox {
    connection: Mutex<Connection>,
}

impl EncryptedControlOutbox {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Self::from_connection(Connection::open(path)?)
    }

    pub fn in_memory() -> Result<Self> {
        Self::from_connection(Connection::open_in_memory()?)
    }

    fn from_connection(mut connection: Connection) -> Result<Self> {
        connection.busy_timeout(Duration::from_secs(5))?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        tx.execute_batch(
            "CREATE TABLE IF NOT EXISTS matter_schema_versions(
                 component TEXT PRIMARY KEY NOT NULL, version INTEGER NOT NULL
             );",
        )?;
        let version: Option<i64> = tx
            .query_row(
                "SELECT version FROM matter_schema_versions WHERE component='encrypted_outbox'",
                [],
                |row| row.get(0),
            )
            .optional()?;
        if version.is_some_and(|value| value != OUTBOX_SCHEMA) {
            bail!("Unsupported encrypted outbox schema");
        }
        tx.execute_batch(
            "CREATE TABLE IF NOT EXISTS encrypted_control_outbox(
                 scope TEXT NOT NULL,
                 record_type TEXT NOT NULL,
                 sequence INTEGER NOT NULL CHECK(sequence > 0),
                 fingerprint TEXT NOT NULL,
                 envelope_json TEXT NOT NULL,
                 remote_revision TEXT,
                 synchronized INTEGER NOT NULL CHECK(synchronized IN (0,1)),
                 PRIMARY KEY(scope,record_type)
             );
             CREATE TABLE IF NOT EXISTS encrypted_outbox_audit(
                 id INTEGER PRIMARY KEY AUTOINCREMENT,
                 scope TEXT NOT NULL,
                 record_type TEXT NOT NULL,
                 sequence INTEGER NOT NULL,
                 action TEXT NOT NULL,
                 created_at INTEGER NOT NULL
             );
             CREATE TRIGGER IF NOT EXISTS encrypted_outbox_audit_no_update
                 BEFORE UPDATE ON encrypted_outbox_audit
                 BEGIN SELECT RAISE(ABORT,'Immutable outbox audit'); END;
             CREATE TRIGGER IF NOT EXISTS encrypted_outbox_audit_no_delete
                 BEFORE DELETE ON encrypted_outbox_audit
                 BEGIN SELECT RAISE(ABORT,'Immutable outbox audit'); END;
             INSERT OR IGNORE INTO matter_schema_versions(component,version)
                 VALUES('encrypted_outbox',1);",
        )?;
        tx.commit()?;
        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    /// Queue plaintext only after encrypting with the caller's authenticated
    /// active key. Repeated identical content under that key is a no-op.
    /// An encrypted pending record survives sink failures and process restart.
    pub fn enqueue(
        &self,
        manager: &KeyManager,
        scope: &str,
        record_type: &str,
        plaintext: &[u8],
    ) -> Result<bool> {
        validate_scope(scope)?;
        validate_type(record_type)?;
        let (key_id, key) = manager.get_active()?;
        let mut hmac = Hmac::<Sha256>::new_from_slice(key)
            .map_err(|_| anyhow!("Cannot initialize control fingerprint"))?;
        hmac.update(b"matter-control-outbox-fingerprint-v1\0");
        hmac.update(scope.as_bytes());
        hmac.update(b"\0");
        hmac.update(record_type.as_bytes());
        hmac.update(b"\0");
        hmac.update(plaintext);
        let fingerprint = format!("{}:{}", key_id, hex::encode(hmac.finalize().into_bytes()));

        let mut random = [0_u8; 16];
        getrandom::fill(&mut random).map_err(|_| anyhow!("Cannot allocate envelope ID"))?;
        let envelope_id = format!("control-{}", hex::encode(random));
        let now = time::OffsetDateTime::now_utc();
        let created_at = format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
            now.year(),
            now.month() as u8,
            now.day(),
            now.hour(),
            now.minute(),
            now.second()
        );
        let envelope = encryption::encrypt(
            plaintext,
            key,
            &envelope_id,
            scope,
            record_type,
            key_id,
            &created_at,
        )?;
        let envelope_json = serde_json::to_string(&envelope)?;
        let mut connection = self.lock()?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let previous: Option<(String, i64)> = tx
            .query_row(
                "SELECT fingerprint,sequence FROM encrypted_control_outbox
                 WHERE scope=?1 AND record_type=?2",
                params![scope, record_type],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;
        if previous.as_ref().is_some_and(|(old, _)| old == &fingerprint) {
            return Ok(false);
        }
        let sequence = match previous {
            Some((_, prior)) => prior
                .checked_add(1)
                .ok_or_else(|| anyhow!("Outbox sequence overflow"))?,
            None => 1,
        };
        tx.execute(
            "INSERT INTO encrypted_control_outbox(
                 scope,record_type,sequence,fingerprint,envelope_json,
                 remote_revision,synchronized
             ) VALUES(?1,?2,?3,?4,?5,NULL,0)
             ON CONFLICT(scope,record_type)
             DO UPDATE SET sequence=excluded.sequence,
               fingerprint=excluded.fingerprint,
               envelope_json=excluded.envelope_json,synchronized=0",
            params![scope, record_type, sequence, fingerprint, envelope_json],
        )?;
        audit(&tx, scope, record_type, sequence, "queued")?;
        tx.commit()?;
        Ok(true)
    }

    /// Exports a consistent, owner-authorized role snapshot and encrypts it.
    /// Plaintext is zeroized after envelope creation.
    pub fn enqueue_role_snapshot(
        &self,
        roles: &RoleStore,
        actor: &str,
        manager: &KeyManager,
        scope: &str,
    ) -> Result<bool> {
        let plaintext = Zeroizing::new(roles.export_snapshot(scope, actor)?);
        self.enqueue(manager, scope, "roles", plaintext.as_slice())
    }

    /// Publish one pending record using the sink's compare-and-swap contract.
    /// Sink failure leaves the same ciphertext pending for safe retry by an
    /// idempotent implementation. Never retry a transport effect blindly.
    pub fn flush(
        &self,
        sink: &dyn EncryptedEnvelopeSink,
        scope: &str,
        record_type: &str,
    ) -> Result<bool> {
        validate_scope(scope)?;
        validate_type(record_type)?;
        let pending: Option<(i64, String, Option<String>, i64)> = {
            let connection = self.lock()?;
            connection
                .query_row(
                    "SELECT sequence,envelope_json,remote_revision,synchronized
                     FROM encrypted_control_outbox
                     WHERE scope=?1 AND record_type=?2",
                    params![scope, record_type],
                    |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
                )
                .optional()?
        };
        let (sequence, encoded, expected_revision, synchronized) =
            pending.ok_or_else(|| anyhow!("Unknown outbox record"))?;
        if synchronized == 1 {
            return Ok(false);
        }
        if synchronized != 0 {
            bail!("Invalid outbox synchronization status");
        }
        let envelope: EncryptedEnvelope = serde_json::from_str(&encoded)?;
        encryption::validate_envelope(&envelope)?;
        if envelope.scope != scope || envelope.record_type != record_type {
            bail!("Outbox identity mismatch");
        }
        let remote_revision = sink
            .put(&envelope, expected_revision.as_deref())
            .map_err(|_| anyhow!("Encrypted envelope publication failed"))?;
        validate_revision(&remote_revision)?;
        let mut connection = self.lock()?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let updated = tx.execute(
            "UPDATE encrypted_control_outbox SET remote_revision=?1,synchronized=1
             WHERE scope=?2 AND record_type=?3 AND sequence=?4 AND synchronized=0",
            params![remote_revision, scope, record_type, sequence],
        )?;
        if updated != 1 {
            bail!("Outbox changed during external publication; reconcile remote state");
        }
        audit(&tx, scope, record_type, sequence, "published")?;
        tx.commit()?;
        Ok(true)
    }

    pub fn status(&self, scope: &str, record_type: &str) -> Result<Option<OutboxStatus>> {
        validate_scope(scope)?;
        validate_type(record_type)?;
        let connection = self.lock()?;
        connection
            .query_row(
                "SELECT sequence,synchronized,remote_revision
                 FROM encrypted_control_outbox WHERE scope=?1 AND record_type=?2",
                params![scope, record_type],
                |row| {
                    Ok(OutboxStatus {
                        sequence: row.get(0)?,
                        synchronized: row.get::<_, i64>(1)? == 1,
                        remote_revision: row.get(2)?,
                    })
                },
            )
            .optional()
            .map_err(Into::into)
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, Connection>> {
        self.connection
            .lock()
            .map_err(|_| anyhow!("Encrypted outbox lock poisoned"))
    }
}

fn audit(
    tx: &rusqlite::Transaction<'_>,
    scope: &str,
    record_type: &str,
    sequence: i64,
    action: &str,
) -> Result<()> {
    tx.execute(
        "INSERT INTO encrypted_outbox_audit(
             scope,record_type,sequence,action,created_at
         ) VALUES(?1,?2,?3,?4,?5)",
        params![
            scope,
            record_type,
            sequence,
            action,
            time::OffsetDateTime::now_utc().unix_timestamp()
        ],
    )?;
    Ok(())
}

fn validate_scope(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > MAX_FIELD_BYTES
        || !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._:-".contains(&b))
    {
        bail!("Invalid outbox scope");
    }
    Ok(())
}

fn validate_type(record_type: &str) -> Result<()> {
    if !matches!(record_type, "roles" | "permissions" | "configuration" | "audit") {
        bail!("Unsupported control record type");
    }
    Ok(())
}

fn validate_revision(value: &str) -> Result<()> {
    validate_scope(value)
}
