/*
 * Matter-Base — Authenticated platform ingress and replay protection.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use crate::control::{ControlDispatcher, DispatchResult};
use anyhow::{anyhow, bail, Result};
use hmac::{Hmac, Mac};
use rusqlite::{params, Connection, TransactionBehavior};
use sha2::Sha256;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use zeroize::Zeroizing;

const MAX_EVENT_BYTES: usize = 16 * 1024;
const MAX_EVENT_ID_BYTES: usize = 256;
const MAX_CONTEXT_BYTES: usize = 128;
const MAX_REPLAY_RECORDS: i64 = 100_000;

/// Untrusted incoming provider payload. It contains NO authenticated actor
/// or socket identity; the application verifier must independently derive
/// those from an authenticated provider connection.
#[derive(Debug, Clone)]
pub struct InboundEvent {
    pub platform: String,
    pub event_id: String,
    pub text: String,
}

impl InboundEvent {
    fn validate(&self) -> Result<()> {
        validate_platform(&self.platform)?;
        if self.event_id.is_empty()
            || self.event_id.len() > MAX_EVENT_ID_BYTES
            || self.event_id.chars().any(char::is_control)
            || self.text.is_empty()
            || self.text.len() > MAX_EVENT_BYTES
        {
            bail!("Invalid inbound event");
        }
        Ok(())
    }
}

/// Trusted data derived from the provider's verified connection and subject.
#[derive(Debug, Clone)]
pub struct VerifiedIngress {
    pub platform: String,
    pub socket_id: String,
    pub actor_id: String,
    pub source_id: String,
    pub group_id: Option<String>,
}

impl VerifiedIngress {
    fn validate(&self) -> Result<()> {
        validate_platform(&self.platform)?;
        validate_context(&self.socket_id)?;
        validate_context(&self.actor_id)?;
        validate_context(&self.source_id)?;
        if let Some(group) = &self.group_id {
            validate_context(group)?;
        }
        Ok(())
    }
}

/// The provider adapter MUST verify webhook signatures or the live connection
/// identity. Claimed user IDs in event text are never trusted.
pub trait InboundAuthenticator: Send + Sync {
    fn authenticate(&self, event: &InboundEvent) -> Result<VerifiedIngress>;
}

/// Required consent, ownership, connection generation and platform policy.
/// Runs BEFORE replay claim or bot handler, including on control commands.
pub trait InboundAdmission: Send + Sync {
    fn authorize(&self, event: &InboundEvent, trusted: &VerifiedIngress) -> Result<()>;
}

/// Durable HMAC-keyed replay guard. The secret is injected by the application
/// and never persisted. All processes must share the SAME active key while
/// enforcing a retention window. Rotations and pruning require coordination.
pub struct InboundReplayStore {
    connection: Mutex<Connection>,
    key: Zeroizing<[u8; 32]>,
}

impl InboundReplayStore {
    pub fn open(path: impl AsRef<Path>, key: [u8; 32]) -> Result<Self> {
        Self::from_connection(Connection::open(path)?, key)
    }

    pub fn in_memory(key: [u8; 32]) -> Result<Self> {
        Self::from_connection(Connection::open_in_memory()?, key)
    }

    fn from_connection(connection: Connection, key: [u8; 32]) -> Result<Self> {
        if key.iter().all(|byte| *byte == 0) {
            bail!("Invalid replay binding key");
        }
        connection.busy_timeout(Duration::from_secs(5))?;
        connection.execute_batch(
            "CREATE TABLE IF NOT EXISTS inbound_replay(
                 fingerprint TEXT PRIMARY KEY NOT NULL,
                 recorded_at INTEGER NOT NULL
             );
             CREATE INDEX IF NOT EXISTS idx_inbound_replay_time
                 ON inbound_replay(recorded_at);",
        )?;
        Ok(Self {
            connection: Mutex::new(connection),
            key: Zeroizing::new(key),
        })
    }

    /// Returns false for a repeated event. The claim occurs before handler
    /// side effects; failures leave uncertain effects and must not be retried
    /// blindly. Exhausted storage fails closed until trusted pruning occurs.
    pub fn claim(&self, platform: &str, socket: &str, event_id: &str) -> Result<bool> {
        validate_platform(platform)?;
        validate_context(socket)?;
        if event_id.is_empty()
            || event_id.len() > MAX_EVENT_ID_BYTES
            || event_id.chars().any(char::is_control)
        {
            bail!("Invalid replay identifier");
        }
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.key[..])
            .map_err(|_| anyhow!("Replay fingerprint unavailable"))?;
        mac.update(b"matter-inbound-replay-v1\0");
        for part in [platform.as_bytes(), socket.as_bytes(), event_id.as_bytes()] {
            mac.update(&(part.len() as u32).to_be_bytes());
            mac.update(part);
        }
        let fingerprint = hex::encode(mac.finalize().into_bytes());
        let mut connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Inbound replay store unavailable"))?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let existing: i64 = tx.query_row(
            "SELECT COUNT(*) FROM inbound_replay WHERE fingerprint=?1",
            params![fingerprint],
            |row| row.get(0),
        )?;
        if existing != 0 {
            return Ok(false);
        }
        let count: i64 = tx.query_row(
            "SELECT COUNT(*) FROM inbound_replay",
            [],
            |row| row.get(0),
        )?;
        if count >= MAX_REPLAY_RECORDS {
            bail!("Inbound replay capacity exhausted");
        }
        tx.execute(
            "INSERT INTO inbound_replay(fingerprint,recorded_at)
             VALUES(?1,?2)",
            params![
                fingerprint,
                time::OffsetDateTime::now_utc().unix_timestamp()
            ],
        )?;
        tx.commit()?;
        Ok(true)
    }

    /// Retention must be configured by trusted host policy; pruning reopens
    /// the replay window and requires provider delivery idempotency controls.
    pub fn prune_before(&self, timestamp: i64) -> Result<usize> {
        let mut connection = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Inbound replay store unavailable"))?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let removed = tx.execute(
            "DELETE FROM inbound_replay WHERE recorded_at < ?1",
            params![timestamp],
        )?;
        tx.commit()?;
        Ok(removed)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum InboundOutcome {
    Duplicate,
    Dispatched(DispatchResult),
}

/// Trusted callback gateway. No provider authenticator or admission policy
/// is bundled. The replay claim precedes command effects; a crash after
/// claiming may need manual reconciliation, not automatic re-execution.
pub struct InboundGateway {
    authenticate: Arc<dyn InboundAuthenticator>,
    admission: Arc<dyn InboundAdmission>,
    replay: Arc<InboundReplayStore>,
    control: Arc<ControlDispatcher>,
}

impl InboundGateway {
    pub fn new(
        authenticate: Arc<dyn InboundAuthenticator>,
        admission: Arc<dyn InboundAdmission>,
        replay: Arc<InboundReplayStore>,
        control: Arc<ControlDispatcher>,
    ) -> Self {
        Self {
            authenticate,
            admission,
            replay,
            control,
        }
    }

    pub fn ingest(&self, event: &InboundEvent) -> Result<InboundOutcome> {
        event.validate()?;
        let trusted = catch_unwind(AssertUnwindSafe(|| self.authenticate.authenticate(event)))
            .map_err(|_| anyhow!("Inbound authentication denied"))?
            .map_err(|_| anyhow!("Inbound authentication denied"))?;
        trusted.validate()?;
        if trusted.platform != event.platform {
            bail!("Inbound provider identity mismatch");
        }
        catch_unwind(AssertUnwindSafe(|| self.admission.authorize(event, &trusted)))
            .map_err(|_| anyhow!("Inbound admission denied"))?
            .map_err(|_| anyhow!("Inbound admission denied"))?;
        if !self
            .replay
            .claim(&trusted.platform, &trusted.socket_id, &event.event_id)?
        {
            return Ok(InboundOutcome::Duplicate);
        }
        self.control
            .dispatch(
                &trusted.socket_id,
                &trusted.actor_id,
                &trusted.source_id,
                trusted.group_id.as_deref(),
                &event.text,
            )
            .map(InboundOutcome::Dispatched)
    }
}

fn validate_platform(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    {
        bail!("Invalid inbound platform");
    }
    Ok(())
}

fn validate_context(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > MAX_CONTEXT_BYTES
        || !value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || b"._:@/-".contains(&byte)
        })
    {
        bail!("Invalid authenticated ingress context");
    }
    Ok(())
}
