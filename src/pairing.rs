/*
 * Base-Matter — Pairing-code-only authentication boundary.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use crate::connections::ConnectionAdmission;
use crate::sessions::{ConnectionState, SessionRecord, SessionRegistry};
use anyhow::{anyhow, bail, Result};
use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Mutex};
use zeroize::Zeroizing;

/// Pairing codes are transient credentials. Never persist, log, or include
/// them in error/Debug output. The authenticated host may reveal this value
/// once to its socket owner over an independently protected channel.
pub struct PairingCode {
    value: Zeroizing<String>,
}

impl PairingCode {
    pub fn expose_to_authenticated_owner(&self) -> &str {
        self.value.as_str()
    }
}

impl fmt::Debug for PairingCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PairingCode([REDACTED])")
    }
}

/// Implementation belongs to the provider connection already authenticated
/// by the host. There is no QR method or QR fallback in this contract.
/// A provider MUST ensure only the current socket generation can request it.
pub trait PairingCodeProvider: Send + Sync {
    fn request_code(&self, session: &SessionRecord, international_digits: &str) -> Result<String>;
}

/// No QR mechanism and no test ping bot. Access is authenticated as the
/// session owner, subject to mandatory application admission. External
/// providers are responsible for connection/reconnect and distributed
/// throttling; this local lock only prevents concurrent calls in one host.
pub struct PairingCodeService {
    sessions: Arc<SessionRegistry>,
    admission: Arc<dyn ConnectionAdmission>,
    provider: Arc<dyn PairingCodeProvider>,
    local_lock: Mutex<()>,
}

impl PairingCodeService {
    pub fn new(
        sessions: Arc<SessionRegistry>,
        admission: Arc<dyn ConnectionAdmission>,
        provider: Arc<dyn PairingCodeProvider>,
    ) -> Self {
        Self {
            sessions,
            admission,
            provider,
            local_lock: Mutex::new(()),
        }
    }

    /// Requests a transient code for an unlinked WhatsApp session. Neither
    /// the phone number nor the returned code is written into SessionRegistry.
    /// The owning client enters the pairing code in WhatsApp's linked devices.
    pub fn request_code(&self, socket: &str, actor: &str, phone: &str) -> Result<PairingCode> {
        if !(8..=15).contains(&phone.len()) || !phone.bytes().all(|b| b.is_ascii_digit()) {
            bail!("Invalid pairing phone number");
        }
        let _lock = self
            .local_lock
            .lock()
            .map_err(|_| anyhow!("Pairing request unavailable"))?;
        let session = self
            .sessions
            .inspect(socket, actor)
            .map_err(|_| anyhow!("Pairing request denied"))?;
        if session.platform != "whatsapp"
            || !matches!(
                session.state,
                ConnectionState::Disconnected | ConnectionState::Connecting
            )
        {
            bail!("Pairing session is not eligible");
        }
        catch_unwind(AssertUnwindSafe(|| self.admission.authorize(&session, actor)))
            .map_err(|_| anyhow!("Pairing request denied"))?
            .map_err(|_| anyhow!("Pairing request denied"))?;
        let result = catch_unwind(AssertUnwindSafe(|| {
            self.provider.request_code(&session, phone)
        }))
        .map_err(|_| anyhow!("Pairing provider failed"))?
        .map_err(|_| anyhow!("Pairing provider failed"))?;
        let value = Zeroizing::new(result);
        if !(6..=16).contains(&value.len())
            || !value.bytes().all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
        {
            bail!("Invalid pairing code from provider");
        }
        Ok(PairingCode { value })
    }
}
