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
const MAX_BOT_CO_OWNERS: usize = 256;
const INSTANCE_ID_RANDOM_BYTES: usize = 16;
const BOT_REGISTRY_SCHEMA_VERSION: i64 = 3;

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
    owner_id: Option<String>,
    co_owner_ids: Vec<String>,
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

    /// Returns the owning authenticated application user, if this is a legacy
    /// identity awaiting an externally authorized ownership claim.
    pub fn owner_id(&self) -> Option<&str> {
        self.owner_id.as_deref()
    }

    pub fn co_owner_ids(&self) -> &[String] {
        &self.co_owner_ids
    }
}

/// Audited ownership change for one bot instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BotAction {
    Manage,
    ManageOwners,
    Delete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BotAuditRecord {
    pub record_id: i64,
    pub instance_id: String,
    pub actor_id: String,
    pub action: String,
    pub target_owner_id: Option<String>,
    pub created_at: i64,
}

/// External policy required to claim identities created before ownership was
/// added. The implementation must authenticate the actor and verify authority.
pub trait OwnerRecoveryAuthorizer: Send + Sync {
    fn authorize_claim(&self, instance_id: &str, actor_id: &str) -> Result<()>;
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
        owner_id: &str,
    ) -> Result<BotIdentity>;

    fn transfer_owner(&self, instance_id: &str, actor_id: &str, new_owner_id: &str) -> Result<()>;

    fn add_co_owner(&self, instance_id: &str, actor_id: &str, co_owner_id: &str) -> Result<()>;

    fn remove_co_owner(&self, instance_id: &str, actor_id: &str, co_owner_id: &str) -> Result<()>;

    fn claim_unowned(&self, instance_id: &str, owner_id: &str) -> Result<()>;

    fn remove_owned(&self, instance_id: &str, actor_id: &str) -> Result<()>;

    fn audit_history(&self, instance_id: &str, actor_id: &str) -> Result<Vec<BotAuditRecord>>;

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
        owner_id: &str,
    ) -> Result<BotIdentity> {
        validate_identifier(family_id, "family_id")?;
        validate_identifier(release_key, "release_key")?;
        validate_identifier(requested_name, "display name")?;
        validate_identifier(owner_id, "owner_id")?;

        for _ in 0..3 {
            let instance_id = generate_instance_id()?;
            match self.store.register(
                family_id,
                release_key,
                &instance_id,
                requested_name,
                owner_id,
            ) {
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

    /// Authorizes a bot-owner operation using an actor ID established by the
    /// application's authenticated request boundary.
    pub fn authorize_owner(&self, instance_id: &str, actor_id: &str) -> Result<()> {
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(actor_id, "actor_id")?;
        let identity = self
            .store
            .get(instance_id)?
            .ok_or_else(|| anyhow!("Bot instance not found"))?;
        if identity.owner_id() != Some(actor_id) {
            bail!("Bot owner authorization denied");
        }
        Ok(())
    }

    /// Checks whether an authenticated actor has the requested bot scope.
    pub fn authorize(&self, instance_id: &str, actor_id: &str, action: BotAction) -> Result<()> {
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(actor_id, "actor_id")?;
        let identity = self
            .store
            .get(instance_id)?
            .ok_or_else(|| anyhow!("Bot instance not found"))?;
        if identity.owner_id() == Some(actor_id) {
            return Ok(());
        }
        if action == BotAction::Manage && identity.co_owner_ids().iter().any(|id| id == actor_id) {
            return Ok(());
        }
        bail!("Bot authorization denied")
    }

    pub fn add_co_owner(&self, instance_id: &str, actor_id: &str, co_owner_id: &str) -> Result<()> {
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(actor_id, "actor_id")?;
        validate_identifier(co_owner_id, "co_owner_id")?;
        self.store.add_co_owner(instance_id, actor_id, co_owner_id)
    }

    pub fn remove_co_owner(
        &self,
        instance_id: &str,
        actor_id: &str,
        co_owner_id: &str,
    ) -> Result<()> {
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(actor_id, "actor_id")?;
        validate_identifier(co_owner_id, "co_owner_id")?;
        self.store
            .remove_co_owner(instance_id, actor_id, co_owner_id)
    }

    /// Transfers ownership. Only the currently stored owner can transfer it.
    pub fn transfer_owner(
        &self,
        instance_id: &str,
        actor_id: &str,
        new_owner_id: &str,
    ) -> Result<()> {
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(actor_id, "actor_id")?;
        validate_identifier(new_owner_id, "owner_id")?;
        self.store
            .transfer_owner(instance_id, actor_id, new_owner_id)
    }

    /// Claims a legacy identity only after a caller-supplied trusted recovery
    /// policy authenticates and authorizes the actor.
    pub fn claim_unowned(
        &self,
        instance_id: &str,
        actor_id: &str,
        authorizer: &dyn OwnerRecoveryAuthorizer,
    ) -> Result<()> {
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(actor_id, "actor_id")?;
        authorizer.authorize_claim(instance_id, actor_id)?;
        self.store.claim_unowned(instance_id, actor_id)
    }

    /// Deletes an identity only when the actor is its current owner.
    pub fn remove_owned(&self, instance_id: &str, actor_id: &str) -> Result<()> {
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(actor_id, "actor_id")?;
        self.store.remove_owned(instance_id, actor_id)
    }

    /// Returns ownership audit records only to the bot's current owner.
    pub fn audit_history(&self, instance_id: &str, actor_id: &str) -> Result<Vec<BotAuditRecord>> {
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(actor_id, "actor_id")?;
        self.store.audit_history(instance_id, actor_id)
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
    if let Some(owner_id) = &identity.owner_id {
        validate_identifier(owner_id, "owner_id")?;
    }
    if identity.co_owner_ids.len() > MAX_BOT_CO_OWNERS {
        bail!("Invalid bot co-owner list");
    }
    let mut co_owners = std::collections::HashSet::new();
    for co_owner_id in &identity.co_owner_ids {
        validate_identifier(co_owner_id, "co_owner_id")?;
        if Some(co_owner_id) == identity.owner_id.as_ref() || !co_owners.insert(co_owner_id) {
            bail!("Invalid bot co-owner list");
        }
    }
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
struct InMemoryRegistryState {
    identities: BTreeMap<String, BotIdentity>,
    audit: Vec<BotAuditRecord>,
}

#[derive(Default)]
pub struct InMemoryBotIdentityStore {
    state: Mutex<InMemoryRegistryState>,
}

impl BotIdentityStore for InMemoryBotIdentityStore {
    fn register(
        &self,
        family_id: &str,
        release_key: &str,
        instance_id: &str,
        requested_name: &str,
        owner_id: &str,
    ) -> Result<BotIdentity> {
        validate_identifier(family_id, "family_id")?;
        validate_identifier(release_key, "release_key")?;
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(requested_name, "display name")?;
        validate_identifier(owner_id, "owner_id")?;
        let mut state = self
            .state
            .lock()
            .map_err(|_| anyhow!("Bot identity store lock poisoned"))?;
        if state.identities.contains_key(instance_id) {
            return Err(InstanceIdentifierCollision.into());
        }
        let display_name = resolve_name(requested_name, |candidate| {
            Ok(state
                .identities
                .values()
                .any(|identity| identity.display_name == candidate))
        })?;
        let identity = BotIdentity {
            family_id: family_id.to_owned(),
            release_key: release_key.to_owned(),
            instance_id: instance_id.to_owned(),
            display_name,
            owner_id: Some(owner_id.to_owned()),
            co_owner_ids: Vec::new(),
        };
        state
            .identities
            .insert(instance_id.to_owned(), identity.clone());
        append_memory_audit(
            &mut state,
            instance_id,
            owner_id,
            "register",
            Some(owner_id),
        );
        Ok(identity)
    }

    fn get(&self, instance_id: &str) -> Result<Option<BotIdentity>> {
        let state = self
            .state
            .lock()
            .map_err(|_| anyhow!("Bot identity store lock poisoned"))?;
        let identity = state.identities.get(instance_id).cloned();
        if let Some(identity) = &identity {
            validate_identity(identity)?;
        }
        Ok(identity)
    }

    fn list(&self) -> Result<Vec<BotIdentity>> {
        let state = self
            .state
            .lock()
            .map_err(|_| anyhow!("Bot identity store lock poisoned"))?;
        let identities = state.identities.values().cloned().collect::<Vec<_>>();
        for identity in &identities {
            validate_identity(identity)?;
        }
        Ok(identities)
    }

    fn transfer_owner(&self, instance_id: &str, actor_id: &str, new_owner_id: &str) -> Result<()> {
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(actor_id, "actor_id")?;
        validate_identifier(new_owner_id, "owner_id")?;
        let mut state = self
            .state
            .lock()
            .map_err(|_| anyhow!("Bot identity store lock poisoned"))?;
        let identity = state
            .identities
            .get_mut(instance_id)
            .ok_or_else(|| anyhow!("Bot instance not found"))?;
        if identity.owner_id.as_deref() != Some(actor_id) {
            bail!("Bot owner authorization denied");
        }
        if actor_id == new_owner_id {
            bail!("New owner must differ from the current owner");
        }
        identity.owner_id = Some(new_owner_id.to_owned());
        identity
            .co_owner_ids
            .retain(|id| id != actor_id && id != new_owner_id);
        append_memory_audit(
            &mut state,
            instance_id,
            actor_id,
            "transfer_owner",
            Some(new_owner_id),
        );
        Ok(())
    }

    fn add_co_owner(&self, instance_id: &str, actor_id: &str, co_owner_id: &str) -> Result<()> {
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(actor_id, "actor_id")?;
        validate_identifier(co_owner_id, "co_owner_id")?;
        let mut state = self
            .state
            .lock()
            .map_err(|_| anyhow!("Bot identity store lock poisoned"))?;
        let identity = state
            .identities
            .get_mut(instance_id)
            .ok_or_else(|| anyhow!("Bot instance not found"))?;
        if identity.owner_id.as_deref() != Some(actor_id) {
            bail!("Bot owner authorization denied");
        }
        if actor_id == co_owner_id {
            bail!("Owner cannot also be a co-owner");
        }
        if !identity.co_owner_ids.iter().any(|id| id == co_owner_id) {
            if identity.co_owner_ids.len() >= MAX_BOT_CO_OWNERS {
                bail!("Bot co-owner limit reached");
            }
            identity.co_owner_ids.push(co_owner_id.to_owned());
            append_memory_audit(
                &mut state,
                instance_id,
                actor_id,
                "add_co_owner",
                Some(co_owner_id),
            );
        }
        Ok(())
    }

    fn remove_co_owner(&self, instance_id: &str, actor_id: &str, co_owner_id: &str) -> Result<()> {
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(actor_id, "actor_id")?;
        validate_identifier(co_owner_id, "co_owner_id")?;
        let mut state = self
            .state
            .lock()
            .map_err(|_| anyhow!("Bot identity store lock poisoned"))?;
        let identity = state
            .identities
            .get_mut(instance_id)
            .ok_or_else(|| anyhow!("Bot instance not found"))?;
        if identity.owner_id.as_deref() != Some(actor_id) {
            bail!("Bot owner authorization denied");
        }
        let before = identity.co_owner_ids.len();
        identity.co_owner_ids.retain(|id| id != co_owner_id);
        if identity.co_owner_ids.len() == before {
            bail!("Co-owner not found");
        }
        append_memory_audit(
            &mut state,
            instance_id,
            actor_id,
            "remove_co_owner",
            Some(co_owner_id),
        );
        Ok(())
    }

    fn claim_unowned(&self, instance_id: &str, owner_id: &str) -> Result<()> {
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(owner_id, "owner_id")?;
        let mut state = self
            .state
            .lock()
            .map_err(|_| anyhow!("Bot identity store lock poisoned"))?;
        let identity = state
            .identities
            .get_mut(instance_id)
            .ok_or_else(|| anyhow!("Bot instance not found"))?;
        if identity.owner_id.is_some() {
            bail!("Bot instance already has an owner");
        }
        identity.owner_id = Some(owner_id.to_owned());
        append_memory_audit(
            &mut state,
            instance_id,
            owner_id,
            "claim_unowned",
            Some(owner_id),
        );
        Ok(())
    }

    fn remove_owned(&self, instance_id: &str, actor_id: &str) -> Result<()> {
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(actor_id, "actor_id")?;
        let mut state = self
            .state
            .lock()
            .map_err(|_| anyhow!("Bot identity store lock poisoned"))?;
        let identity = state
            .identities
            .get(instance_id)
            .ok_or_else(|| anyhow!("Bot instance not found"))?;
        if identity.owner_id.as_deref() != Some(actor_id) {
            bail!("Bot owner authorization denied");
        }
        state.identities.remove(instance_id);
        append_memory_audit(&mut state, instance_id, actor_id, "remove", None);
        Ok(())
    }

    fn audit_history(&self, instance_id: &str, actor_id: &str) -> Result<Vec<BotAuditRecord>> {
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(actor_id, "actor_id")?;
        let state = self
            .state
            .lock()
            .map_err(|_| anyhow!("Bot identity store lock poisoned"))?;
        let identity = state
            .identities
            .get(instance_id)
            .ok_or_else(|| anyhow!("Bot instance not found"))?;
        if identity.owner_id.as_deref() != Some(actor_id) {
            bail!("Bot owner authorization denied");
        }
        Ok(state
            .audit
            .iter()
            .filter(|record| record.instance_id == instance_id)
            .cloned()
            .collect())
    }
}

fn append_memory_audit(
    state: &mut InMemoryRegistryState,
    instance_id: &str,
    actor_id: &str,
    action: &str,
    target_owner_id: Option<&str>,
) {
    state.audit.push(BotAuditRecord {
        record_id: i64::try_from(state.audit.len())
            .unwrap_or(i64::MAX)
            .saturating_add(1),
        instance_id: instance_id.to_owned(),
        actor_id: actor_id.to_owned(),
        action: action.to_owned(),
        target_owner_id: target_owner_id.map(str::to_owned),
        created_at: time::OffsetDateTime::now_utc().unix_timestamp(),
    });
}

fn insert_sqlite_audit(
    transaction: &rusqlite::Transaction<'_>,
    instance_id: &str,
    actor_id: &str,
    action: &str,
    target_owner_id: Option<&str>,
) -> Result<()> {
    transaction.execute(
        "INSERT INTO bot_registry_audit (instance_id, actor_id, action, target_owner_id, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![instance_id, actor_id, action, target_owner_id, time::OffsetDateTime::now_utc().unix_timestamp()],
    )?;
    Ok(())
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
        let mut version = current_version.unwrap_or(0);
        if version < 1 {
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
                "INSERT INTO matter_schema_versions (component, version) VALUES (?1, 1)
                 ON CONFLICT(component) DO UPDATE SET version = excluded.version",
                params!["bot_registry"],
            )?;
            version = 1;
        }
        if version < 2 {
            transaction.execute_batch(
                "ALTER TABLE bot_identities ADD COLUMN owner_id TEXT;
                 CREATE TABLE bot_registry_audit (
                     record_id INTEGER PRIMARY KEY AUTOINCREMENT,
                     instance_id TEXT NOT NULL,
                     actor_id TEXT NOT NULL,
                     action TEXT NOT NULL,
                     target_owner_id TEXT,
                     created_at INTEGER NOT NULL
                 );
                 CREATE INDEX idx_bot_registry_audit_instance
                     ON bot_registry_audit (instance_id, record_id);",
            )?;
            transaction.execute(
                "UPDATE matter_schema_versions SET version = 2 WHERE component = ?1",
                params!["bot_registry"],
            )?;
        }
        if version < 3 {
            transaction.execute_batch(
                "ALTER TABLE bot_identities ADD COLUMN co_owner_ids TEXT NOT NULL DEFAULT '[]';",
            )?;
            transaction.execute(
                "UPDATE matter_schema_versions SET version = 3 WHERE component = ?1",
                params!["bot_registry"],
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
        owner_id: &str,
    ) -> Result<BotIdentity> {
        validate_identifier(family_id, "family_id")?;
        validate_identifier(release_key, "release_key")?;
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(requested_name, "display name")?;
        validate_identifier(owner_id, "owner_id")?;
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
                instance_id, family_id, release_key, display_name, created_at, owner_id, co_owner_ids
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                instance_id,
                family_id,
                release_key,
                display_name,
                time::OffsetDateTime::now_utc().unix_timestamp(),
                owner_id,
                "[]"
            ],
        )?;
        insert_sqlite_audit(
            &transaction,
            instance_id,
            owner_id,
            "register",
            Some(owner_id),
        )?;
        transaction.commit()?;

        Ok(BotIdentity {
            family_id: family_id.to_owned(),
            release_key: release_key.to_owned(),
            instance_id: instance_id.to_owned(),
            display_name,
            owner_id: Some(owner_id.to_owned()),
            co_owner_ids: Vec::new(),
        })
    }

    fn get(&self, instance_id: &str) -> Result<Option<BotIdentity>> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Bot identity store lock poisoned"))?;
        let identity = connection
            .query_row(
                "SELECT family_id, release_key, instance_id, display_name, owner_id, co_owner_ids
             FROM bot_identities WHERE instance_id = ?1",
                params![instance_id],
                |row| {
                    Ok(BotIdentity {
                        family_id: row.get(0)?,
                        release_key: row.get(1)?,
                        instance_id: row.get(2)?,
                        display_name: row.get(3)?,
                        owner_id: row.get(4)?,
                        co_owner_ids: serde_json::from_str(&row.get::<_, String>(5)?)
                            .map_err(|_| rusqlite::Error::InvalidQuery)?,
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
            "SELECT family_id, release_key, instance_id, display_name, owner_id, co_owner_ids
             FROM bot_identities ORDER BY instance_id",
        )?;
        let rows = statement.query_map([], |row| {
            Ok(BotIdentity {
                family_id: row.get(0)?,
                release_key: row.get(1)?,
                instance_id: row.get(2)?,
                display_name: row.get(3)?,
                owner_id: row.get(4)?,
                co_owner_ids: serde_json::from_str(&row.get::<_, String>(5)?)
                    .map_err(|_| rusqlite::Error::InvalidQuery)?,
            })
        })?;
        let identities = rows.collect::<rusqlite::Result<Vec<_>>>()?;
        for identity in &identities {
            validate_identity(identity)?;
        }
        Ok(identities)
    }

    fn transfer_owner(&self, instance_id: &str, actor_id: &str, new_owner_id: &str) -> Result<()> {
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(actor_id, "actor_id")?;
        validate_identifier(new_owner_id, "owner_id")?;
        if actor_id == new_owner_id {
            bail!("New owner must differ from the current owner");
        }
        let mut connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Bot identity store lock poisoned"))?;
        let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let owner: Option<String> = transaction
            .query_row(
                "SELECT owner_id FROM bot_identities WHERE instance_id = ?1",
                params![instance_id],
                |row| row.get(0),
            )
            .optional()?
            .ok_or_else(|| anyhow!("Bot instance not found"))?;
        if owner.as_deref() != Some(actor_id) {
            bail!("Bot owner authorization denied");
        }
        let co_owners_json: String = transaction.query_row(
            "SELECT co_owner_ids FROM bot_identities WHERE instance_id = ?1",
            params![instance_id],
            |row| row.get(0),
        )?;
        let mut co_owners: Vec<String> = serde_json::from_str(&co_owners_json)?;
        if co_owners.len() > MAX_BOT_CO_OWNERS {
            bail!("Invalid bot co-owner list");
        }
        co_owners.retain(|id| id != actor_id && id != new_owner_id);
        transaction.execute(
            "UPDATE bot_identities SET owner_id = ?1, co_owner_ids = ?2 WHERE instance_id = ?3",
            params![
                new_owner_id,
                serde_json::to_string(&co_owners)?,
                instance_id
            ],
        )?;
        insert_sqlite_audit(
            &transaction,
            instance_id,
            actor_id,
            "transfer_owner",
            Some(new_owner_id),
        )?;
        transaction.commit()?;
        Ok(())
    }

    fn add_co_owner(&self, instance_id: &str, actor_id: &str, co_owner_id: &str) -> Result<()> {
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(actor_id, "actor_id")?;
        validate_identifier(co_owner_id, "co_owner_id")?;
        if actor_id == co_owner_id {
            bail!("Owner cannot also be a co-owner");
        }
        let mut connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Bot identity store lock poisoned"))?;
        let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let (owner, encoded): (String, String) = transaction
            .query_row(
                "SELECT owner_id, co_owner_ids FROM bot_identities WHERE instance_id = ?1",
                params![instance_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?
            .ok_or_else(|| anyhow!("Bot instance not found"))?;
        if owner != actor_id {
            bail!("Bot owner authorization denied");
        }
        let mut co_owners: Vec<String> = serde_json::from_str(&encoded)?;
        if co_owners.len() > MAX_BOT_CO_OWNERS {
            bail!("Invalid bot co-owner list");
        }
        if !co_owners.iter().any(|id| id == co_owner_id) {
            if co_owners.len() >= MAX_BOT_CO_OWNERS {
                bail!("Bot co-owner limit reached");
            }
            co_owners.push(co_owner_id.to_owned());
            transaction.execute(
                "UPDATE bot_identities SET co_owner_ids = ?1 WHERE instance_id = ?2",
                params![serde_json::to_string(&co_owners)?, instance_id],
            )?;
            insert_sqlite_audit(
                &transaction,
                instance_id,
                actor_id,
                "add_co_owner",
                Some(co_owner_id),
            )?;
        }
        transaction.commit()?;
        Ok(())
    }

    fn remove_co_owner(&self, instance_id: &str, actor_id: &str, co_owner_id: &str) -> Result<()> {
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(actor_id, "actor_id")?;
        validate_identifier(co_owner_id, "co_owner_id")?;
        let mut connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Bot identity store lock poisoned"))?;
        let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let (owner, encoded): (String, String) = transaction
            .query_row(
                "SELECT owner_id, co_owner_ids FROM bot_identities WHERE instance_id = ?1",
                params![instance_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?
            .ok_or_else(|| anyhow!("Bot instance not found"))?;
        if owner != actor_id {
            bail!("Bot owner authorization denied");
        }
        let mut co_owners: Vec<String> = serde_json::from_str(&encoded)?;
        if co_owners.len() > MAX_BOT_CO_OWNERS {
            bail!("Invalid bot co-owner list");
        }
        let before = co_owners.len();
        co_owners.retain(|id| id != co_owner_id);
        if co_owners.len() == before {
            bail!("Co-owner not found");
        }
        transaction.execute(
            "UPDATE bot_identities SET co_owner_ids = ?1 WHERE instance_id = ?2",
            params![serde_json::to_string(&co_owners)?, instance_id],
        )?;
        insert_sqlite_audit(
            &transaction,
            instance_id,
            actor_id,
            "remove_co_owner",
            Some(co_owner_id),
        )?;
        transaction.commit()?;
        Ok(())
    }

    fn claim_unowned(&self, instance_id: &str, owner_id: &str) -> Result<()> {
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(owner_id, "owner_id")?;
        let mut connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Bot identity store lock poisoned"))?;
        let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let changed = transaction.execute(
            "UPDATE bot_identities SET owner_id = ?1 WHERE instance_id = ?2 AND owner_id IS NULL",
            params![owner_id, instance_id],
        )?;
        if changed == 0 {
            let exists: bool = transaction.query_row(
                "SELECT EXISTS(SELECT 1 FROM bot_identities WHERE instance_id = ?1)",
                params![instance_id],
                |row| row.get(0),
            )?;
            if !exists {
                bail!("Bot instance not found");
            }
            bail!("Bot instance already has an owner");
        }
        insert_sqlite_audit(
            &transaction,
            instance_id,
            owner_id,
            "claim_unowned",
            Some(owner_id),
        )?;
        transaction.commit()?;
        Ok(())
    }

    fn remove_owned(&self, instance_id: &str, actor_id: &str) -> Result<()> {
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(actor_id, "actor_id")?;
        let mut connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Bot identity store lock poisoned"))?;
        let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let owner: Option<String> = transaction
            .query_row(
                "SELECT owner_id FROM bot_identities WHERE instance_id = ?1",
                params![instance_id],
                |row| row.get(0),
            )
            .optional()?
            .ok_or_else(|| anyhow!("Bot instance not found"))?;
        if owner.as_deref() != Some(actor_id) {
            bail!("Bot owner authorization denied");
        }
        transaction.execute(
            "DELETE FROM bot_identities WHERE instance_id = ?1",
            params![instance_id],
        )?;
        insert_sqlite_audit(&transaction, instance_id, actor_id, "remove", None)?;
        transaction.commit()?;
        Ok(())
    }

    fn audit_history(&self, instance_id: &str, actor_id: &str) -> Result<Vec<BotAuditRecord>> {
        validate_identifier(instance_id, "instance_id")?;
        validate_identifier(actor_id, "actor_id")?;
        let mut connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Bot identity store lock poisoned"))?;
        let transaction = connection.transaction_with_behavior(TransactionBehavior::Deferred)?;
        let owner: Option<String> = transaction
            .query_row(
                "SELECT owner_id FROM bot_identities WHERE instance_id = ?1",
                params![instance_id],
                |row| row.get(0),
            )
            .optional()?
            .ok_or_else(|| anyhow!("Bot instance not found"))?;
        if owner.as_deref() != Some(actor_id) {
            bail!("Bot owner authorization denied");
        }
        let mut statement = transaction.prepare(
            "SELECT record_id, instance_id, actor_id, action, target_owner_id, created_at
             FROM bot_registry_audit WHERE instance_id = ?1 ORDER BY record_id",
        )?;
        let rows = statement.query_map(params![instance_id], |row| {
            Ok(BotAuditRecord {
                record_id: row.get(0)?,
                instance_id: row.get(1)?,
                actor_id: row.get(2)?,
                action: row.get(3)?,
                target_owner_id: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?;
        let records = rows.collect::<rusqlite::Result<Vec<_>>>()?;
        drop(statement);
        transaction.commit()?;
        Ok(records)
    }
}
