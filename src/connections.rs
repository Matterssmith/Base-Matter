/*
 * Matter-Base — External connection lifecycle contracts.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use crate::sessions::{ConnectionState, SessionRecord, SessionRegistry};
use anyhow::{anyhow, bail, Result};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Mutex};
use zeroize::Zeroizing;

const MAX_CREDENTIAL_BYTES: usize = 2 * 1024 * 1024;

/// Authenticate the actor at the host ingress before granting a lifecycle
/// operation. The source ID and session ID are not proof of identity.
pub trait ConnectionAdmission: Send + Sync {
    fn authorize(&self, session: &SessionRecord, actor: &str) -> Result<()>;
}

/// Source for already-authorized credentials. Returned bytes are zeroized.
/// Implementors must keep sessions/keys outside Matter-Base storage.
pub trait SessionCredentialSource: Send + Sync {
    fn load(&self, session: &SessionRecord) -> Result<Zeroizing<Vec<u8>>>;
}

/// Concrete provider connections are implemented in platform repositories.
/// Adapters must fence network operations by session revision/generation and
/// reject writes once the corresponding connection has been revoked.
pub trait ConnectionDriver: Send + Sync {
    fn connect(&self, session: &SessionRecord, credential: &[u8]) -> Result<()>;
    fn disconnect(&self, session: &SessionRecord) -> Result<()>;
    fn is_connected(&self, session: &SessionRecord) -> Result<bool>;
}

/// Never returns credentials or inserts them into persisted metadata.
/// The local lock serializes operations through one instance; independent
/// instances rely on SessionRegistry's revision compare-and-swap and the
/// external driver's generation fencing.
pub struct ConnectionService {
    sessions: Arc<SessionRegistry>,
    admission: Arc<dyn ConnectionAdmission>,
    credential_source: Arc<dyn SessionCredentialSource>,
    driver: Arc<dyn ConnectionDriver>,
    local_lock: Mutex<()>,
}

impl ConnectionService {
    pub fn new(
        sessions: Arc<SessionRegistry>,
        admission: Arc<dyn ConnectionAdmission>,
        credential_source: Arc<dyn SessionCredentialSource>,
        driver: Arc<dyn ConnectionDriver>,
    ) -> Self {
        Self {
            sessions,
            admission,
            credential_source,
            driver,
            local_lock: Mutex::new(()),
        }
    }

    fn authorized(&self, socket: &str, actor: &str) -> Result<SessionRecord> {
        let session = self
            .sessions
            .inspect(socket, actor)
            .map_err(|_| anyhow!("Connection access denied"))?;
        self.admission
            .authorize(&session, actor)
            .map_err(|_| anyhow!("Connection access denied"))?;
        Ok(session)
    }

    /// Transitions Disconnected/Faulted -> Connecting -> Connected on success.
    /// Credential and driver errors leave a Faulted state when state ownership
    /// remains valid. A partially successful driver is asked to disconnect;
    /// a driver that cannot revoke its generation must not be used in prod.
    pub fn connect(&self, socket: &str, actor: &str) -> Result<SessionRecord> {
        let _lock = self
            .local_lock
            .lock()
            .map_err(|_| anyhow!("Connection operation unavailable"))?;
        let record = self.authorized(socket, actor)?;
        if !matches!(
            record.state,
            ConnectionState::Disconnected | ConnectionState::Faulted
        ) {
            bail!("Connection is already active or connecting");
        }
        let connecting = self.sessions.transition(
            socket,
            actor,
            record.revision,
            ConnectionState::Connecting,
        )?;
        let attempt = catch_unwind(AssertUnwindSafe(|| -> Result<()> {
            let credentials = self.credential_source.load(&connecting)?;
            if credentials.is_empty() || credentials.len() > MAX_CREDENTIAL_BYTES {
                bail!("Invalid credential size");
            }
            self.driver.connect(&connecting, &credentials)?;
            Ok(())
        }));
        match attempt {
            Ok(Ok(())) => match self.sessions.transition(
                socket,
                actor,
                connecting.revision,
                ConnectionState::Connected,
            ) {
                Ok(connected) => Ok(connected),
                Err(_) => {
                    let _ = catch_unwind(AssertUnwindSafe(|| self.driver.disconnect(&connecting)));
                    bail!("Connection state changed before activation")
                }
            },
            _ => {
                let _ = catch_unwind(AssertUnwindSafe(|| self.driver.disconnect(&connecting)));
                let _ = self.sessions.transition(
                    socket,
                    actor,
                    connecting.revision,
                    ConnectionState::Faulted,
                );
                bail!("Connection operation failed")
            }
        }
    }

    /// Explicitly disconnects a verified active connection. The external
    /// driver must revoke the session before the metadata becomes disconnected.
    pub fn disconnect(&self, socket: &str, actor: &str) -> Result<SessionRecord> {
        let _lock = self
            .local_lock
            .lock()
            .map_err(|_| anyhow!("Connection operation unavailable"))?;
        let record = self.authorized(socket, actor)?;
        if record.state == ConnectionState::Disconnected {
            return Ok(record);
        }
        if record.state == ConnectionState::Connecting {
            bail!("Cannot disconnect while connecting; reconcile the state first");
        }
        if record.state == ConnectionState::Connected {
            let stopped = catch_unwind(AssertUnwindSafe(|| self.driver.disconnect(&record)));
            if !matches!(stopped, Ok(Ok(()))) {
                let _ = self.sessions.transition(
                    socket,
                    actor,
                    record.revision,
                    ConnectionState::Faulted,
                );
                bail!("Connection shutdown failed");
            }
        }
        self.sessions
            .transition(socket, actor, record.revision, ConnectionState::Disconnected)
    }

    /// A live probe is required before assuming stored Connected means online.
    /// Failed probes leave the session faulted; they do not trigger retries.
    pub fn check_health(&self, socket: &str, actor: &str) -> Result<bool> {
        let _lock = self
            .local_lock
            .lock()
            .map_err(|_| anyhow!("Connection operation unavailable"))?;
        let record = self.authorized(socket, actor)?;
        if record.state != ConnectionState::Connected {
            return Ok(false);
        }
        let health = catch_unwind(AssertUnwindSafe(|| self.driver.is_connected(&record)));
        if matches!(health, Ok(Ok(true))) {
            return Ok(true);
        }
        let _ = self.sessions.transition(
            socket,
            actor,
            record.revision,
            ConnectionState::Faulted,
        );
        Ok(false)
    }

    /// Recovery after an interrupted process or uncertain network outcome.
    /// Requires application authorization; does not prove that the provider
    /// disconnected, so adapters must revoke stale generations separately.
    pub fn mark_uncertain(&self, socket: &str, actor: &str) -> Result<SessionRecord> {
        let _lock = self
            .local_lock
            .lock()
            .map_err(|_| anyhow!("Connection operation unavailable"))?;
        let record = self.authorized(socket, actor)?;
        if record.state == ConnectionState::Disconnected || record.state == ConnectionState::Faulted {
            return Ok(record);
        }
        self.sessions
            .transition(socket, actor, record.revision, ConnectionState::Faulted)
    }
}
