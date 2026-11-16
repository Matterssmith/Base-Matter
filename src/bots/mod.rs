/*
 * Matter-Base — Bot Identity Registry
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::{anyhow, bail, Result};
use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

const MAX_IDENTIFIER_BYTES: usize = 128;
const INSTANCE_ID_RANDOM_BYTES: usize = 16;
const BOT_REGISTRY_SCHEMA_VERSION: i64 = 1;

#[derive(Debug)]
struct InstanceIdentifierCollision;

impl fmt::Display for InstanceIdentifierCollision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("instance identifier collision")
    }
}

impl std::error::Error for InstanceIdentifierCollision {}

/// Stable identity of one bot instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BotIdentity {
    family_id: String,
    release_key: String,
    instance_id: String,
    display_name: String,
}

impl BotIdentity {
    pub fn family_id(&self) -> &str {
        &self.family_id
    }

    pub fn release_key(&self) -> &str {
        &self.release_key
    }

    pub fn instance_id(&self) -> &str {
        &self.instance_id
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }
}

/// Storage contract for atomic identity registration and recovery.
pub trait BotIdentityStore: Send + Sync {
    /// Registers one instance, resolving a duplicate visible name atomically.
    fn register(
        &self,
        family_id: &str,
        release_key: &str,
        instance_id: &str,
        requested_name: &str,
    ) -> Result<BotIdentity>;

    fn get(&self, instance_id: &str) -> Result<Option<BotIdentity>>;

    fn list(&self) -> Result<Vec<BotIdentity>>;
}

/// Thread-safe bot registry backed by an interchangeable identity store.
#[derive(Clone)]
pub struct BotRegistry {
    store: Arc<dyn BotIdentityStore>,
}

impl BotRegistry {
    pub fn new(store: Arc<dyn BotIdentityStore>) -> Self {
        Self { store }
    }

    pub fn in_memory() -> Self {
        Self::new(Arc::new(InMemoryBotIdentityStore::default()))
    }

    pub fn open_sqlite(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self::new(Arc::new(SqliteBotIdentityStore::open(path)?)))
    }

    /// Registers a new instance with a generated, independent instance ID.
    pub fn register(
        &self,
        family_id: &str,
        release_key: &str,
        requested_name: &str,
    ) -> Result<BotIdentity> {
        validate_identifier(family_id, "family_id")?;
        validate_identifier(release_key, "release_key")?;
        validate_identifier(requested_name, "display name")?;

        for _ in 0..3 {
            let instance_id = generate_instance_id()?;
            match self
                .store
                .register(family_id, release_key, &instance_id, requested_name)
            {
                Ok(identity) => return Ok(identity),
                Err(error)
                    if error
                        .downcast_ref::<InstanceIdentifierCollision>()
                        .is_some() =>
                {
                    continue;
                }
                Err(error) => return Err(error),
            }
        }

        bail!("Could not allocate a unique bot instance identifier")
    }

    pub fn get(&self, instance_id: &str) -> Result<Option<BotIdentity>> {
        validate_identifier(instance_id, "instance_id")?;
        self.store.get(instance_id)
    }

    pub fn list(&self) -> Result<Vec<BotIdentity>> {
        self.store.list()
    }
}

fn validate_identifier(value: &str, name: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.len() > MAX_IDENTIFIER_BYTES
        || value.chars().any(char::is_control)
    {
        bail!("Invalid {name}");
    }
    Ok(())
}

fn validate_identity(identity: &BotIdentity) -> Result<()> {
    validate_identifier(&identity.family_id, "family_id")?;
    validate_identifier(&identity.release_key, "release_key")?;
    validate_identifier(&identity.instance_id, "instance_id")?;
    validate_identifier(&identity.display_name, "display name")?;
    Ok(())
}

fn generate_instance_id() -> Result<String> {
    let mut random = [0u8; INSTANCE_ID_RANDOM_BYTES];
    getrandom::fill(&mut random).map_err(|_| anyhow!("Could not generate bot instance ID"))?;
    Ok(format!("inst_{}", hex::encode(random)))
}

fn resolve_name<F>(requested_name: &str, mut exists: F) -> Result<String>
where
    F: FnMut(&str) -> Result<bool>,
{
    if !exists(requested_name)? {
        return Ok(requested_name.to_owned());
    }

    for suffix in 2u64..=1_000_000 {
        let candidate = format!("{requested_name} ({suffix})");
        validate_identifier(&candidate, "display name")?;
        if !exists(&candidate)? {
            return Ok(candidate);
        }
    }

    bail!("Could not resolve a unique bot display name")
}

#[derive(Default)]
pub struct InMemoryBotIdentityStore {
    identities: Mutex<BTreeMap<String, BotIdentity>>,
}

impl BotIdentityStore for InMemoryBotIdentityStore {
    fn register(
        &self,
        family_id: &str,
        release_key: &str,
        instance_id: &str,
        requested_name: &str,
    ) -> Result<BotIdentity> {
        validate_identifier(family_id, "family_id")?;
        validate_identifier(release_key, "release_key")?;
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(requested_name, "display name")?;
        let mut identities = self
            .identities
            .lock()
            .map_err(|_| anyhow!("Bot identity store lock poisoned"))?;
        if identities.contains_key(instance_id) {
            return Err(InstanceIdentifierCollision.into());
        }
        let display_name = resolve_name(requested_name, |candidate| {
            Ok(identities
                .values()
                .any(|identity| identity.display_name == candidate))
        })?;
        let identity = BotIdentity {
            family_id: family_id.to_owned(),
            release_key: release_key.to_owned(),
            instance_id: instance_id.to_owned(),
            display_name,
        };
        identities.insert(instance_id.to_owned(), identity.clone());
        Ok(identity)
    }

    fn get(&self, instance_id: &str) -> Result<Option<BotIdentity>> {
        let identities = self
            .identities
            .lock()
            .map_err(|_| anyhow!("Bot identity store lock poisoned"))?;
        let identity = identities.get(instance_id).cloned();
        if let Some(identity) = &identity {
            validate_identity(identity)?;
        }
        Ok(identity)
    }

    fn list(&self) -> Result<Vec<BotIdentity>> {
        let identities = self
            .identities
            .lock()
            .map_err(|_| anyhow!("Bot identity store lock poisoned"))?;
        let identities = identities.values().cloned().collect::<Vec<_>>();
        for identity in &identities {
            validate_identity(identity)?;
        }
        Ok(identities)
    }
}

/// SQLite-backed registry store with atomic name allocation and persistence.
#[derive(Clone)]
pub struct SqliteBotIdentityStore {
    connection: Arc<Mutex<Connection>>,
}

impl SqliteBotIdentityStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        let mut connection = Connection::open(path)?;
        connection.busy_timeout(std::time::Duration::from_secs(5))?;
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
                params!["bot_registry"],
                |row| row.get(0),
            )
            .optional()?;
        if current_version.is_some_and(|version| version > BOT_REGISTRY_SCHEMA_VERSION) {
            bail!("Bot registry schema version is newer than this runtime supports");
        }
        if current_version.unwrap_or(0) < 1 {
            transaction.execute_batch(
                "CREATE TABLE IF NOT EXISTS bot_identities (
                     instance_id TEXT PRIMARY KEY NOT NULL,
                     family_id TEXT NOT NULL,
                     release_key TEXT NOT NULL,
                     display_name TEXT NOT NULL UNIQUE,
                     created_at INTEGER NOT NULL
                 );
                 CREATE INDEX IF NOT EXISTS idx_bot_identities_family_release
                     ON bot_identities (family_id, release_key);",
            )?;
            transaction.execute(
                "INSERT INTO matter_schema_versions (component, version)
                 VALUES (?1, ?2)
                 ON CONFLICT(component) DO UPDATE SET version = excluded.version",
                params!["bot_registry", BOT_REGISTRY_SCHEMA_VERSION],
            )?;
        }
        transaction.commit()?;
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }
}

impl BotIdentityStore for SqliteBotIdentityStore {
    fn register(
        &self,
        family_id: &str,
        release_key: &str,
        instance_id: &str,
        requested_name: &str,
    ) -> Result<BotIdentity> {
        validate_identifier(family_id, "family_id")?;
        validate_identifier(release_key, "release_key")?;
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(requested_name, "display name")?;
        let mut connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Bot identity store lock poisoned"))?;
        let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let existing: bool = transaction.query_row(
            "SELECT EXISTS(SELECT 1 FROM bot_identities WHERE instance_id = ?1)",
            params![instance_id],
            |row| row.get(0),
        )?;
        if existing {
            return Err(InstanceIdentifierCollision.into());
        }

        let display_name = resolve_name(requested_name, |candidate| {
            transaction
                .query_row(
                    "SELECT EXISTS(SELECT 1 FROM bot_identities WHERE display_name = ?1)",
                    params![candidate],
                    |row| row.get(0),
                )
                .map_err(Into::into)
        })?;
        transaction.execute(
            "INSERT INTO bot_identities (
                instance_id, family_id, release_key, display_name, created_at
             ) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                instance_id,
                family_id,
                release_key,
                display_name,
                time::OffsetDateTime::now_utc().unix_timestamp()
            ],
        )?;
        transaction.commit()?;

        Ok(BotIdentity {
            family_id: family_id.to_owned(),
            release_key: release_key.to_owned(),
            instance_id: instance_id.to_owned(),
            display_name,
        })
    }

    fn get(&self, instance_id: &str) -> Result<Option<BotIdentity>> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Bot identity store lock poisoned"))?;
        let identity = connection
            .query_row(
                "SELECT family_id, release_key, instance_id, display_name
             FROM bot_identities WHERE instance_id = ?1",
                params![instance_id],
                |row| {
                    Ok(BotIdentity {
                        family_id: row.get(0)?,
                        release_key: row.get(1)?,
                        instance_id: row.get(2)?,
                        display_name: row.get(3)?,
                    })
                },
            )
            .optional()?;
        if let Some(identity) = &identity {
            validate_identity(identity)?;
        }
        Ok(identity)
    }

    fn list(&self) -> Result<Vec<BotIdentity>> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Bot identity store lock poisoned"))?;
        let mut statement = connection.prepare(
            "SELECT family_id, release_key, instance_id, display_name
             FROM bot_identities ORDER BY instance_id",
        )?;
        let rows = statement.query_map([], |row| {
            Ok(BotIdentity {
                family_id: row.get(0)?,
                release_key: row.get(1)?,
                instance_id: row.get(2)?,
                display_name: row.get(3)?,
            })
        })?;
        let identities = rows.collect::<rusqlite::Result<Vec<_>>>()?;
        for identity in &identities {
            validate_identity(identity)?;
        }
        Ok(identities)
    }
}
