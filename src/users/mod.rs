/*!
 * Matter-Base — Shared Users
 *
 * Provides a global application user ID, explicit external identity links,
 * and private data scoped by bot instance and user. Authentication and policy
 * decisions are supplied by the consuming application.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::{anyhow, bail, Result};
use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

const USER_SCHEMA_VERSION: i64 = 1;
const MAX_ID_BYTES: usize = 128;
const MAX_DATA_BYTES: usize = 65_536;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternalIdentity {
    provider: String,
    subject: String,
}

impl ExternalIdentity {
    pub fn new(provider: impl Into<String>, subject: impl Into<String>) -> Result<Self> {
        let provider = provider.into();
        let subject = subject.into();
        validate(&provider, "identity provider")?;
        validate(&subject, "external identity")?;
        Ok(Self { provider, subject })
    }

    pub fn provider(&self) -> &str {
        &self.provider
    }
    pub fn subject(&self) -> &str {
        &self.subject
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserAction {
    Create,
    LinkIdentity,
    UnlinkIdentity,
    ResolveIdentity,
    ReadData,
    WriteData,
    DeleteData,
}

#[derive(Debug, Clone, Copy)]
pub struct UserAuthorizationRequest<'a> {
    pub principal_id: &'a str,
    pub action: UserAction,
    pub user_id: Option<&'a str>,
    pub bot_instance_id: Option<&'a str>,
    pub external_identity: Option<&'a ExternalIdentity>,
    pub data_key: Option<&'a str>,
}

/// The application must authenticate the principal and enforce ownership,
/// bot scope, and any required consent before approving an operation.
pub trait UserAuthorizer: Send + Sync {
    fn authorize(&self, request: &UserAuthorizationRequest<'_>) -> Result<()>;
}

pub struct UserPrincipal<'a> {
    id: &'a str,
    authorizer: &'a dyn UserAuthorizer,
}

impl<'a> UserPrincipal<'a> {
    pub fn new(id: &'a str, authorizer: &'a dyn UserAuthorizer) -> Result<Self> {
        validate(id, "principal_id")?;
        Ok(Self { id, authorizer })
    }

    fn authorize(
        &self,
        action: UserAction,
        user_id: Option<&str>,
        bot: Option<&str>,
        identity: Option<&ExternalIdentity>,
        key: Option<&str>,
    ) -> Result<()> {
        self.authorizer
            .authorize(&UserAuthorizationRequest {
                principal_id: self.id,
                action,
                user_id,
                bot_instance_id: bot,
                external_identity: identity,
                data_key: key,
            })
            .map_err(|_| anyhow!("User authorization denied"))
    }
}

/// Replaceable persistence contract. Implementations must make identity links
/// unique by provider/subject and persist each scoped-data write atomically.
pub trait UserStore: Send + Sync {
    fn create_user(&self, user_id: &str) -> Result<()>;
    fn link_identity(&self, user_id: &str, identity: &ExternalIdentity) -> Result<()>;
    fn unlink_identity(&self, user_id: &str, identity: &ExternalIdentity) -> Result<()>;
    fn resolve_identity(&self, identity: &ExternalIdentity) -> Result<Option<String>>;
    fn set_data(&self, bot: &str, user: &str, key: &str, value: &str) -> Result<()>;
    fn get_data(&self, bot: &str, user: &str, key: &str) -> Result<Option<String>>;
    fn delete_data(&self, bot: &str, user: &str, key: &str) -> Result<bool>;
}

#[derive(Clone)]
pub struct UserRegistry {
    store: Arc<dyn UserStore>,
}

impl UserRegistry {
    pub fn new(store: Arc<dyn UserStore>) -> Self {
        Self { store }
    }
    pub fn in_memory() -> Self {
        Self::new(Arc::new(InMemoryUserStore::default()))
    }
    pub fn open_sqlite(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self::new(Arc::new(SqliteUserStore::open(path)?)))
    }

    pub fn create_user(&self, principal: &UserPrincipal<'_>) -> Result<String> {
        principal.authorize(UserAction::Create, None, None, None, None)?;
        for _ in 0..3 {
            let mut bytes = [0u8; 16];
            getrandom::fill(&mut bytes).map_err(|_| anyhow!("Could not generate user ID"))?;
            let user_id = format!("usr_{}", hex::encode(bytes));
            match self.store.create_user(&user_id) {
                Ok(()) => return Ok(user_id),
                Err(e) if e.downcast_ref::<UserIdCollision>().is_some() => continue,
                Err(e) => return Err(e),
            }
        }
        bail!("Could not allocate a unique user ID")
    }

    pub fn link_identity(
        &self,
        user_id: &str,
        identity: &ExternalIdentity,
        principal: &UserPrincipal<'_>,
    ) -> Result<()> {
        validate(user_id, "user_id")?;
        principal.authorize(
            UserAction::LinkIdentity,
            Some(user_id),
            None,
            Some(identity),
            None,
        )?;
        self.store.link_identity(user_id, identity)
    }

    pub fn unlink_identity(
        &self,
        user_id: &str,
        identity: &ExternalIdentity,
        principal: &UserPrincipal<'_>,
    ) -> Result<()> {
        validate(user_id, "user_id")?;
        principal.authorize(
            UserAction::UnlinkIdentity,
            Some(user_id),
            None,
            Some(identity),
            None,
        )?;
        self.store.unlink_identity(user_id, identity)
    }

    pub fn resolve_identity(
        &self,
        identity: &ExternalIdentity,
        principal: &UserPrincipal<'_>,
    ) -> Result<Option<String>> {
        principal.authorize(
            UserAction::ResolveIdentity,
            None,
            None,
            Some(identity),
            None,
        )?;
        self.store.resolve_identity(identity)
    }

    pub fn set_bot_data(
        &self,
        bot: &str,
        user: &str,
        key: &str,
        value: &str,
        principal: &UserPrincipal<'_>,
    ) -> Result<()> {
        validate(bot, "bot_instance_id")?;
        validate(user, "user_id")?;
        validate(key, "data key")?;
        if value.len() > MAX_DATA_BYTES || value.chars().any(|c| c == '\0') {
            bail!("Invalid user data value");
        }
        principal.authorize(
            UserAction::WriteData,
            Some(user),
            Some(bot),
            None,
            Some(key),
        )?;
        self.store.set_data(bot, user, key, value)
    }

    pub fn bot_data(
        &self,
        bot: &str,
        user: &str,
        key: &str,
        principal: &UserPrincipal<'_>,
    ) -> Result<Option<String>> {
        validate(bot, "bot_instance_id")?;
        validate(user, "user_id")?;
        validate(key, "data key")?;
        principal.authorize(UserAction::ReadData, Some(user), Some(bot), None, Some(key))?;
        self.store.get_data(bot, user, key)
    }

    pub fn delete_bot_data(
        &self,
        bot: &str,
        user: &str,
        key: &str,
        principal: &UserPrincipal<'_>,
    ) -> Result<bool> {
        validate(bot, "bot_instance_id")?;
        validate(user, "user_id")?;
        validate(key, "data key")?;
        principal.authorize(
            UserAction::DeleteData,
            Some(user),
            Some(bot),
            None,
            Some(key),
        )?;
        self.store.delete_data(bot, user, key)
    }
}

#[derive(Default)]
struct MemoryUsers {
    users: HashSet<String>,
    identities: HashMap<(String, String), String>,
    data: HashMap<(String, String, String), String>,
}

#[derive(Default)]
struct InMemoryUserStore {
    state: Mutex<MemoryUsers>,
}

impl UserStore for InMemoryUserStore {
    fn create_user(&self, user_id: &str) -> Result<()> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| anyhow!("User store lock poisoned"))?;
        if !state.users.insert(user_id.to_owned()) {
            return Err(UserIdCollision.into());
        }
        Ok(())
    }
    fn link_identity(&self, user_id: &str, identity: &ExternalIdentity) -> Result<()> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| anyhow!("User store lock poisoned"))?;
        if !state.users.contains(user_id) {
            bail!("User not found");
        }
        let key = (identity.provider.clone(), identity.subject.clone());
        if let Some(owner) = state.identities.get(&key) {
            if owner == user_id {
                return Ok(());
            }
            bail!("External identity is already linked");
        }
        state.identities.insert(key, user_id.to_owned());
        Ok(())
    }
    fn unlink_identity(&self, user_id: &str, identity: &ExternalIdentity) -> Result<()> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| anyhow!("User store lock poisoned"))?;
        let key = (identity.provider.clone(), identity.subject.clone());
        if state.identities.get(&key).map(String::as_str) != Some(user_id) {
            bail!("External identity link not found");
        }
        state.identities.remove(&key);
        Ok(())
    }
    fn resolve_identity(&self, identity: &ExternalIdentity) -> Result<Option<String>> {
        let state = self
            .state
            .lock()
            .map_err(|_| anyhow!("User store lock poisoned"))?;
        Ok(state
            .identities
            .get(&(identity.provider.clone(), identity.subject.clone()))
            .cloned())
    }
    fn set_data(&self, bot: &str, user: &str, key: &str, value: &str) -> Result<()> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| anyhow!("User store lock poisoned"))?;
        if !state.users.contains(user) {
            bail!("User not found");
        }
        state.data.insert(
            (bot.to_owned(), user.to_owned(), key.to_owned()),
            value.to_owned(),
        );
        Ok(())
    }
    fn get_data(&self, bot: &str, user: &str, key: &str) -> Result<Option<String>> {
        let state = self
            .state
            .lock()
            .map_err(|_| anyhow!("User store lock poisoned"))?;
        Ok(state
            .data
            .get(&(bot.to_owned(), user.to_owned(), key.to_owned()))
            .cloned())
    }
    fn delete_data(&self, bot: &str, user: &str, key: &str) -> Result<bool> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| anyhow!("User store lock poisoned"))?;
        Ok(state
            .data
            .remove(&(bot.to_owned(), user.to_owned(), key.to_owned()))
            .is_some())
    }
}

#[derive(Debug)]
struct UserIdCollision;
impl std::fmt::Display for UserIdCollision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("user ID collision")
    }
}
impl std::error::Error for UserIdCollision {}

fn validate(value: &str, name: &str) -> Result<()> {
    if value.trim().is_empty() || value.len() > MAX_ID_BYTES || value.chars().any(char::is_control)
    {
        bail!("Invalid {name}");
    }
    Ok(())
}

/// SQLite implementation. Foreign keys bind private data to a registered user;
/// bot IDs are authorization checked by the supplied policy at every access.
#[derive(Clone)]
struct SqliteUserStore {
    connection: Arc<Mutex<Connection>>,
}

impl SqliteUserStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        Self::from_connection(Connection::open(path)?)
    }
    fn from_connection(mut connection: Connection) -> Result<Self> {
        connection.busy_timeout(std::time::Duration::from_secs(5))?;
        connection.execute_batch("PRAGMA foreign_keys = ON;")?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        tx.execute_batch("CREATE TABLE IF NOT EXISTS matter_schema_versions (component TEXT PRIMARY KEY NOT NULL, version INTEGER NOT NULL);")?;
        let version: Option<i64> = tx
            .query_row(
                "SELECT version FROM matter_schema_versions WHERE component='users'",
                [],
                |r| r.get(0),
            )
            .optional()?;
        if version.is_some_and(|v| v > USER_SCHEMA_VERSION || v < 0) {
            bail!("Unsupported user store schema version");
        }
        if version.unwrap_or(0) < 1 {
            tx.execute_batch("CREATE TABLE matter_users (user_id TEXT PRIMARY KEY NOT NULL, created_at INTEGER NOT NULL);
                CREATE TABLE user_external_identities (provider TEXT NOT NULL, subject TEXT NOT NULL, user_id TEXT NOT NULL REFERENCES matter_users(user_id) ON DELETE CASCADE, PRIMARY KEY(provider, subject));
                CREATE INDEX idx_user_external_identity_user ON user_external_identities(user_id);
                CREATE TABLE bot_user_data (bot_instance_id TEXT NOT NULL, user_id TEXT NOT NULL REFERENCES matter_users(user_id) ON DELETE CASCADE, data_key TEXT NOT NULL, value TEXT NOT NULL, updated_at INTEGER NOT NULL, PRIMARY KEY(bot_instance_id,user_id,data_key));")?;
            tx.execute("INSERT INTO matter_schema_versions(component,version) VALUES('users',1) ON CONFLICT(component) DO UPDATE SET version=excluded.version", [])?;
        }
        tx.commit()?;
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }
}

impl UserStore for SqliteUserStore {
    fn create_user(&self, user_id: &str) -> Result<()> {
        validate(user_id, "user_id")?;
        let c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("User store lock poisoned"))?;
        match c.execute(
            "INSERT INTO matter_users(user_id,created_at) VALUES(?1,?2)",
            params![user_id, time::OffsetDateTime::now_utc().unix_timestamp()],
        ) {
            Ok(1) => Ok(()),
            Ok(_) => bail!("User creation did not persist"),
            Err(e) if e.sqlite_error_code() == Some(rusqlite::ErrorCode::ConstraintViolation) => {
                Err(UserIdCollision.into())
            }
            Err(e) => Err(e.into()),
        }
    }
    fn link_identity(&self, user_id: &str, identity: &ExternalIdentity) -> Result<()> {
        let mut c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("User store lock poisoned"))?;
        let tx = c.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let exists: bool = tx.query_row(
            "SELECT EXISTS(SELECT 1 FROM matter_users WHERE user_id=?1)",
            [user_id],
            |r| r.get(0),
        )?;
        if !exists {
            bail!("User not found");
        }
        let current: Option<String> = tx
            .query_row(
                "SELECT user_id FROM user_external_identities WHERE provider=?1 AND subject=?2",
                params![identity.provider, identity.subject],
                |r| r.get(0),
            )
            .optional()?;
        if let Some(current) = current {
            if current == user_id {
                return tx.commit().map_err(Into::into);
            }
            bail!("External identity is already linked");
        }
        tx.execute(
            "INSERT INTO user_external_identities(provider,subject,user_id) VALUES(?1,?2,?3)",
            params![identity.provider, identity.subject, user_id],
        )?;
        tx.commit()?;
        Ok(())
    }
    fn unlink_identity(&self, user_id: &str, identity: &ExternalIdentity) -> Result<()> {
        let c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("User store lock poisoned"))?;
        let changed = c.execute(
            "DELETE FROM user_external_identities WHERE provider=?1 AND subject=?2 AND user_id=?3",
            params![identity.provider, identity.subject, user_id],
        )?;
        if changed != 1 {
            bail!("External identity link not found");
        }
        Ok(())
    }
    fn resolve_identity(&self, identity: &ExternalIdentity) -> Result<Option<String>> {
        let c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("User store lock poisoned"))?;
        c.query_row(
            "SELECT user_id FROM user_external_identities WHERE provider=?1 AND subject=?2",
            params![identity.provider, identity.subject],
            |r| r.get(0),
        )
        .optional()
        .map_err(Into::into)
    }
    fn set_data(&self, bot: &str, user: &str, key: &str, value: &str) -> Result<()> {
        let c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("User store lock poisoned"))?;
        let changed = c.execute("INSERT INTO bot_user_data(bot_instance_id,user_id,data_key,value,updated_at) SELECT ?1,user_id,?3,?4,?5 FROM matter_users WHERE user_id=?2 ON CONFLICT(bot_instance_id,user_id,data_key) DO UPDATE SET value=excluded.value,updated_at=excluded.updated_at", params![bot,user,key,value,time::OffsetDateTime::now_utc().unix_timestamp()])?;
        if changed != 1 {
            bail!("User not found or data was not stored");
        }
        Ok(())
    }
    fn get_data(&self, bot: &str, user: &str, key: &str) -> Result<Option<String>> {
        let c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("User store lock poisoned"))?;
        c.query_row("SELECT value FROM bot_user_data WHERE bot_instance_id=?1 AND user_id=?2 AND data_key=?3", params![bot,user,key], |r| r.get(0)).optional().map_err(Into::into)
    }
    fn delete_data(&self, bot: &str, user: &str, key: &str) -> Result<bool> {
        let c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("User store lock poisoned"))?;
        Ok(c.execute(
            "DELETE FROM bot_user_data WHERE bot_instance_id=?1 AND user_id=?2 AND data_key=?3",
            params![bot, user, key],
        )? == 1)
    }
}
