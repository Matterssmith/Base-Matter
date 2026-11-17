/*
 * Matter-Base — Session state and transport contracts.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use crate::activation::{GlobalPolicy, SocketActivationStore};
use crate::{BotRegistry, RoleStore};
use anyhow::{anyhow, bail, Result};
use rusqlite::{params, Connection, OptionalExtension, Transaction, TransactionBehavior};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const SESSION_SCHEMA_VERSION: i64 = 1;
const MAX_ID_BYTES: usize = 128;
const MAX_PLATFORM_BYTES: usize = 64;
const MAX_TEXT_BYTES: usize = 16 * 1024;
const MAX_RECEIPT_BYTES: usize = 256;
const MAX_AUDIT_PAGE: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Faulted,
}

impl ConnectionState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Disconnected => "disconnected",
            Self::Connecting => "connecting",
            Self::Connected => "connected",
            Self::Faulted => "faulted",
        }
    }

    fn from_str(value: &str) -> Result<Self> {
        match value {
            "disconnected" => Ok(Self::Disconnected),
            "connecting" => Ok(Self::Connecting),
            "connected" => Ok(Self::Connected),
            "faulted" => Ok(Self::Faulted),
            _ => bail!("Invalid persisted connection state"),
        }
    }

    fn permits(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Disconnected, Self::Connecting)
                | (Self::Connecting, Self::Connected)
                | (Self::Connecting, Self::Disconnected)
                | (Self::Connecting, Self::Faulted)
                | (Self::Connected, Self::Disconnected)
                | (Self::Connected, Self::Faulted)
                | (Self::Faulted, Self::Disconnected)
                | (Self::Faulted, Self::Connecting)
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionRecord {
    pub socket_id: String,
    pub owner_id: String,
    pub platform: String,
    pub state: ConnectionState,
    pub revision: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionAuditRecord {
    pub id: i64,
    pub actor_id: String,
    pub action: String,
    pub from_state: Option<String>,
    pub to_state: String,
    pub revision: i64,
    pub created_at: i64,
}

/// Persistent session metadata, never a platform session or credential store.
///
/// The host authenticates the actor and controls provisioning. Revision-based
/// compare-and-swap rejects stale state transitions across independent
/// connections. Socket power is governed by SocketActivationStore separately.
pub struct SessionRegistry {
    connection: Mutex<Connection>,
}

impl SessionRegistry {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Self::from_connection(Connection::open(path)?)
    }

    pub fn in_memory() -> Result<Self> {
        Self::from_connection(Connection::open_in_memory()?)
    }

    fn from_connection(mut connection: Connection) -> Result<Self> {
        connection.busy_timeout(Duration::from_secs(5))?;
        connection.execute_batch("PRAGMA foreign_keys=ON;")?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        tx.execute_batch(
            "CREATE TABLE IF NOT EXISTS matter_schema_versions (
                 component TEXT PRIMARY KEY NOT NULL,
                 version INTEGER NOT NULL
             );",
        )?;
        let version: Option<i64> = tx
            .query_row(
                "SELECT version FROM matter_schema_versions WHERE component='session_metadata'",
                [],
                |row| row.get(0),
            )
            .optional()?;
        if version.is_some_and(|value| value != SESSION_SCHEMA_VERSION) {
            bail!("Unsupported session metadata schema");
        }
        tx.execute_batch(
            "CREATE TABLE IF NOT EXISTS session_metadata (
                 socket_id TEXT PRIMARY KEY NOT NULL,
                 owner_id TEXT NOT NULL,
                 platform TEXT NOT NULL,
                 connection_state TEXT NOT NULL CHECK(
                     connection_state IN ('disconnected','connecting','connected','faulted')
                 ),
                 revision INTEGER NOT NULL CHECK(revision >= 0)
             );
             CREATE TABLE IF NOT EXISTS session_audit (
                 id INTEGER PRIMARY KEY AUTOINCREMENT,
                 socket_id TEXT NOT NULL,
                 actor_id TEXT NOT NULL,
                 action TEXT NOT NULL,
                 from_state TEXT,
                 to_state TEXT NOT NULL,
                 revision INTEGER NOT NULL,
                 created_at INTEGER NOT NULL
             );
             CREATE INDEX IF NOT EXISTS idx_session_audit_socket
                 ON session_audit(socket_id,id);
             CREATE TRIGGER IF NOT EXISTS session_audit_no_update
                 BEFORE UPDATE ON session_audit
                 BEGIN SELECT RAISE(ABORT,'Immutable session audit'); END;
             CREATE TRIGGER IF NOT EXISTS session_audit_no_delete
                 BEFORE DELETE ON session_audit
                 BEGIN SELECT RAISE(ABORT,'Immutable session audit'); END;
             INSERT OR IGNORE INTO matter_schema_versions(component,version)
                 VALUES('session_metadata',1);",
        )?;
        tx.commit()?;
        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    /// Trusted provisioning does not create or import platform credentials.
    pub fn register(&self, socket: &str, actor: &str, platform: &str) -> Result<SessionRecord> {
        validate_id(socket)?;
        validate_id(actor)?;
        validate_platform(platform)?;
        let mut connection = self.lock()?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        if let Some(existing) = read_record(&tx, socket)? {
            if existing.owner_id != actor || existing.platform != platform {
                bail!("Session already belongs to another owner or platform");
            }
            return Ok(existing);
        }
        tx.execute(
            "INSERT INTO session_metadata(
                 socket_id,owner_id,platform,connection_state,revision
             ) VALUES(?1,?2,?3,'disconnected',0)",
            params![socket, actor, platform],
        )?;
        insert_audit(
            &tx,
            socket,
            actor,
            "register",
            None,
            ConnectionState::Disconnected,
            0,
        )?;
        tx.commit()?;
        Ok(SessionRecord {
            socket_id: socket.to_owned(),
            owner_id: actor.to_owned(),
            platform: platform.to_owned(),
            state: ConnectionState::Disconnected,
            revision: 0,
        })
    }

    /// Owner-authorized metadata read. No secrets are exposed.
    pub fn inspect(&self, socket: &str, actor: &str) -> Result<SessionRecord> {
        validate_id(actor)?;
        let record = self.inspect_trusted(socket)?;
        if record.owner_id != actor {
            bail!("Session access denied");
        }
        Ok(record)
    }

    /// Owner-scoped optimistic state transition, with atomic audit append.
    pub fn transition(
        &self,
        socket: &str,
        actor: &str,
        expected_revision: i64,
        next: ConnectionState,
    ) -> Result<SessionRecord> {
        validate_id(socket)?;
        validate_id(actor)?;
        let mut connection = self.lock()?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let mut record = read_record(&tx, socket)?.ok_or_else(|| anyhow!("Unknown session"))?;
        if record.owner_id != actor {
            bail!("Session mutation denied");
        }
        if record.revision != expected_revision || expected_revision < 0 {
            bail!("Stale session transition");
        }
        if record.state == next {
            return Ok(record);
        }
        if !record.state.permits(next) {
            bail!("Invalid session state transition");
        }
        let next_revision = record
            .revision
            .checked_add(1)
            .ok_or_else(|| anyhow!("Session revision overflow"))?;
        tx.execute(
            "UPDATE session_metadata SET connection_state=?1,revision=?2
             WHERE socket_id=?3 AND revision=?4",
            params![next.as_str(), next_revision, socket, expected_revision],
        )?;
        insert_audit(
            &tx,
            socket,
            actor,
            "transition",
            Some(record.state),
            next,
            next_revision,
        )?;
        tx.commit()?;
        record.state = next;
        record.revision = next_revision;
        Ok(record)
    }

    pub fn audit_history(
        &self,
        socket: &str,
        actor: &str,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<SessionAuditRecord>> {
        validate_id(actor)?;
        if limit == 0 || limit > MAX_AUDIT_PAGE {
            bail!("Invalid session audit page size");
        }
        let limit = i64::try_from(limit)?;
        let offset = i64::try_from(offset)?;
        let connection = self.lock()?;
        let record = read_record(&connection, socket)?.ok_or_else(|| anyhow!("Unknown session"))?;
        if record.owner_id != actor {
            bail!("Session audit access denied");
        }
        let mut stmt = connection.prepare(
            "SELECT id,actor_id,action,from_state,to_state,revision,created_at
             FROM session_audit WHERE socket_id=?1 ORDER BY id DESC LIMIT ?2 OFFSET ?3",
        )?;
        let rows = stmt.query_map(params![socket, limit, offset], |row| {
            Ok(SessionAuditRecord {
                id: row.get(0)?,
                actor_id: row.get(1)?,
                action: row.get(2)?,
                from_state: row.get(3)?,
                to_state: row.get(4)?,
                revision: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
    }

    fn inspect_trusted(&self, socket: &str) -> Result<SessionRecord> {
        validate_id(socket)?;
        let connection = self.lock()?;
        read_record(&connection, socket)?.ok_or_else(|| anyhow!("Unknown session"))
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, Connection>> {
        self.connection
            .lock()
            .map_err(|_| anyhow!("Session registry lock poisoned"))
    }
}

fn read_record(connection: &Connection, socket: &str) -> Result<Option<SessionRecord>> {
    let row: Option<(String, String, String, String, i64)> = connection
        .query_row(
            "SELECT socket_id,owner_id,platform,connection_state,revision
             FROM session_metadata WHERE socket_id=?1",
            params![socket],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                ))
            },
        )
        .optional()?;
    row.map(|(socket_id, owner_id, platform, state, revision)| {
        if revision < 0 {
            bail!("Invalid persisted session revision");
        }
        Ok(SessionRecord {
            socket_id,
            owner_id,
            platform,
            state: ConnectionState::from_str(&state)?,
            revision,
        })
    })
    .transpose()
}

fn insert_audit(
    tx: &Transaction<'_>,
    socket: &str,
    actor: &str,
    action: &str,
    previous: Option<ConnectionState>,
    next: ConnectionState,
    revision: i64,
) -> Result<()> {
    tx.execute(
        "INSERT INTO session_audit(
             socket_id,actor_id,action,from_state,to_state,revision,created_at
         ) VALUES(?1,?2,?3,?4,?5,?6,?7)",
        params![
            socket,
            actor,
            action,
            previous.map(ConnectionState::as_str),
            next.as_str(),
            revision,
            time::OffsetDateTime::now_utc().unix_timestamp()
        ],
    )?;
    Ok(())
}

fn validate_id(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > MAX_ID_BYTES
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._:-".contains(&byte))
    {
        bail!("Invalid session identifier");
    }
    Ok(())
}

fn validate_platform(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > MAX_PLATFORM_BYTES
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    {
        bail!("Invalid platform name");
    }
    Ok(())
}

/// A validated outbound text request. The host authenticates its actor ID.
#[derive(Debug, Clone)]
pub struct OutboundMessage {
    pub socket_id: String,
    pub actor_id: String,
    pub destination_id: String,
    pub group_id: Option<String>,
    pub text: String,
}

impl OutboundMessage {
    fn validate(&self) -> Result<()> {
        validate_id(&self.socket_id)?;
        validate_id(&self.actor_id)?;
        validate_id(&self.destination_id)?;
        if let Some(group) = &self.group_id {
            validate_id(group)?;
        }
        if self.text.is_empty() || self.text.len() > MAX_TEXT_BYTES {
            bail!("Invalid outbound message length");
        }
        Ok(())
    }
}

/// Mandatory application policy. Implementations must enforce authenticated
/// identity, destination scope, socket activation, legal consent and roles.
pub trait MessageAdmission: Send + Sync {
    fn authorize(&self, message: &OutboundMessage) -> Result<()>;
}

/// External platform adapter; no provider is included in Matter-Base.
/// A driver must validate its own session generation at the send boundary.
pub trait TransportDriver: Send + Sync {
    fn transmit(&self, session: &SessionRecord, message: &OutboundMessage) -> Result<String>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportAcceptance {
    pub driver_receipt: String,
    pub session_revision: i64,
}

/// Bridges outbound delivery admission with the existing activation, bot and
/// role snapshots. An additional mandatory policy must check legal consent,
/// authenticated identity and any provider-specific destination restrictions.
/// This type does not claim network or cross-database atomicity.
pub struct ControlPlaneMessageAdmission {
    activation: Arc<SocketActivationStore>,
    sessions: Arc<SessionRegistry>,
    bots: Arc<BotRegistry>,
    roles: Arc<RoleStore>,
    additional: Arc<dyn MessageAdmission>,
    global_policy: GlobalPolicy,
}

impl ControlPlaneMessageAdmission {
    pub fn new(
        activation: Arc<SocketActivationStore>,
        sessions: Arc<SessionRegistry>,
        bots: Arc<BotRegistry>,
        roles: Arc<RoleStore>,
        additional: Arc<dyn MessageAdmission>,
        global_default: Option<bool>,
    ) -> Self {
        Self {
            activation,
            sessions,
            bots,
            roles,
            additional,
            global_policy: GlobalPolicy::Configured(global_default),
        }
    }

    pub fn new_with_persistent_policy(
        activation: Arc<SocketActivationStore>,
        sessions: Arc<SessionRegistry>,
        bots: Arc<BotRegistry>,
        roles: Arc<RoleStore>,
        additional: Arc<dyn MessageAdmission>,
    ) -> Self {
        Self {
            activation,
            sessions,
            bots,
            roles,
            additional,
            global_policy: GlobalPolicy::Persisted,
        }
    }
}

impl MessageAdmission for ControlPlaneMessageAdmission {
    fn authorize(&self, message: &OutboundMessage) -> Result<()> {
        let admission = self
            .activation
            .admission(
                &message.socket_id,
                message.group_id.as_deref(),
                self.global_policy,
            )
            .map_err(|_| anyhow!("Message admission denied"))?;
        if !admission.decision.enabled {
            bail!("Message admission denied");
        }
        let bound_bot = admission
            .bot_instance_id
            .ok_or_else(|| anyhow!("Message admission denied"))?;
        self.bots
            .authorize_owner(&bound_bot, &admission.owner_id)
            .map_err(|_| anyhow!("Message admission denied"))?;
        let session = self
            .sessions
            .inspect_trusted(&message.socket_id)
            .map_err(|_| anyhow!("Message admission denied"))?;
        if session.owner_id != admission.owner_id {
            bail!("Message admission denied");
        }
        if !self
            .roles
            .allowed(&bound_bot, &message.actor_id, "message:send", "execute")
            .unwrap_or(false)
        {
            bail!("Message admission denied");
        }
        self.additional
            .authorize(message)
            .map_err(|_| anyhow!("Message admission denied"))
    }
}

/// Synchronous transport boundary with fail-closed policy and session checks.
/// A driver receipt is not proof of recipient delivery, and transport effects
/// cannot be rolled back. There are no automatic retries or secret storage.
pub struct MessageService {
    sessions: Arc<SessionRegistry>,
    policy: Arc<dyn MessageAdmission>,
    driver: Arc<dyn TransportDriver>,
}

impl MessageService {
    pub fn new(
        sessions: Arc<SessionRegistry>,
        policy: Arc<dyn MessageAdmission>,
        driver: Arc<dyn TransportDriver>,
    ) -> Self {
        Self {
            sessions,
            policy,
            driver,
        }
    }

    pub fn send(&self, message: &OutboundMessage) -> Result<TransportAcceptance> {
        message.validate()?;
        self.policy
            .authorize(message)
            .map_err(|_| anyhow!("Message admission denied"))?;
        let session = self.sessions.inspect_trusted(&message.socket_id)?;
        if session.state != ConnectionState::Connected {
            bail!("Session is not connected");
        }
        let receipt = catch_unwind(AssertUnwindSafe(|| self.driver.transmit(&session, message)))
            .map_err(|_| anyhow!("Transport operation failed"))?
            .map_err(|_| anyhow!("Transport operation failed"))?;
        if receipt.is_empty() || receipt.len() > MAX_RECEIPT_BYTES {
            bail!("Invalid transport receipt");
        }
        Ok(TransportAcceptance {
            driver_receipt: receipt,
            session_revision: session.revision,
        })
    }
}
