/*
 * Matter-Base — Provider Assertion Replay Store
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::{anyhow, bail, Result};
use rusqlite::{params, Connection, TransactionBehavior};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

const MAX_TOTAL_REPLAY_ENTRIES: usize = 65_536;

fn validate_identifier(value: &str) -> Result<()> {
    if value.trim().is_empty() || value.len() > 128 || value.chars().any(char::is_control) {
        bail!("Invalid provider replay identifier");
    }

    Ok(())
}

/// Atomically records assertion IDs and rejects replays.
///
/// Implementations must expire old entries, enforce `capacity` within the
/// `(vault_id, audience)` scope, and atomically record the new ID before
/// returning.
pub trait ProviderAssertionReplayStore: Send + Sync {
    fn record_once(
        &self,
        vault_id: &str,
        audience: &str,
        assertion_id: &str,
        expires_at: i64,
        now: i64,
        capacity: usize,
    ) -> Result<()>;
}

/// Bounded replay store scoped to the lifetime of its shared instance.
#[derive(Default)]
pub struct InMemoryProviderAssertionReplayStore {
    entries: Mutex<HashMap<(String, String, String), i64>>,
}

impl ProviderAssertionReplayStore for InMemoryProviderAssertionReplayStore {
    fn record_once(
        &self,
        vault_id: &str,
        audience: &str,
        assertion_id: &str,
        expires_at: i64,
        now: i64,
        capacity: usize,
    ) -> Result<()> {
        validate_identifier(vault_id)?;
        validate_identifier(audience)?;
        validate_identifier(assertion_id)?;

        if expires_at <= now || capacity == 0 {
            bail!("Provider replay record is invalid");
        }

        let mut entries = self
            .entries
            .lock()
            .map_err(|_| anyhow!("Provider replay store lock poisoned"))?;
        entries.retain(|_, stored_expiry| *stored_expiry > now);

        let key = (
            vault_id.to_owned(),
            audience.to_owned(),
            assertion_id.to_owned(),
        );

        if entries.contains_key(&key) {
            bail!("Provider assertion replay detected");
        }

        let scope_count = entries
            .keys()
            .filter(|(stored_vault, stored_audience, _)| {
                stored_vault == vault_id && stored_audience == audience
            })
            .count();

        if scope_count >= capacity {
            bail!("Provider replay cache is full for this scope");
        }
        if entries.len() >= MAX_TOTAL_REPLAY_ENTRIES {
            bail!("Provider replay store has reached its global capacity");
        }

        entries.insert(key, expires_at);
        Ok(())
    }
}

/// SQLite-backed replay store for restart-safe, cross-connection protection.
#[derive(Clone)]
pub struct SqliteProviderAssertionReplayStore {
    connection: Arc<Mutex<Connection>>,
}

impl SqliteProviderAssertionReplayStore {
    /// Opens or creates the replay database and its schema.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }

        let connection = Connection::open(path)?;
        connection.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS provider_assertion_replays (
                vault_id TEXT NOT NULL,
                audience TEXT NOT NULL,
                assertion_id TEXT NOT NULL,
                expires_at INTEGER NOT NULL,
                PRIMARY KEY (vault_id, audience, assertion_id)
            );

            CREATE INDEX IF NOT EXISTS idx_provider_assertion_replays_global_expiry
                ON provider_assertion_replays (expires_at);
            ",
        )?;

        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }
}

impl ProviderAssertionReplayStore for SqliteProviderAssertionReplayStore {
    fn record_once(
        &self,
        vault_id: &str,
        audience: &str,
        assertion_id: &str,
        expires_at: i64,
        now: i64,
        capacity: usize,
    ) -> Result<()> {
        validate_identifier(vault_id)?;
        validate_identifier(audience)?;
        validate_identifier(assertion_id)?;

        if expires_at <= now || capacity == 0 {
            bail!("Provider replay record is invalid");
        }

        let mut connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Provider replay store lock poisoned"))?;
        let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;

        transaction.execute(
            "DELETE FROM provider_assertion_replays WHERE expires_at <= ?1",
            params![now],
        )?;

        let existing: bool = transaction.query_row(
            "SELECT EXISTS(
                SELECT 1 FROM provider_assertion_replays
                WHERE vault_id = ?1 AND audience = ?2 AND assertion_id = ?3
            )",
            params![vault_id, audience, assertion_id],
            |row| row.get(0),
        )?;
        if existing {
            bail!("Provider assertion replay detected");
        }

        let current_count: i64 = transaction.query_row(
            "SELECT COUNT(*) FROM provider_assertion_replays
             WHERE vault_id = ?1 AND audience = ?2",
            params![vault_id, audience],
            |row| row.get(0),
        )?;
        let capacity = i64::try_from(capacity).unwrap_or(i64::MAX);
        if current_count >= capacity {
            bail!("Provider replay cache is full for this scope");
        }
        let total_count: i64 = transaction.query_row(
            "SELECT COUNT(*) FROM provider_assertion_replays",
            [],
            |row| row.get(0),
        )?;
        if total_count >= MAX_TOTAL_REPLAY_ENTRIES as i64 {
            bail!("Provider replay store has reached its global capacity");
        }

        transaction.execute(
            "INSERT INTO provider_assertion_replays (
                vault_id, audience, assertion_id, expires_at
            ) VALUES (?1, ?2, ?3, ?4)",
            params![vault_id, audience, assertion_id, expires_at],
        )?;
        transaction.commit()?;
        Ok(())
    }
}
