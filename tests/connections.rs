/*
 * Matter-Base — Connection lifecycle integration tests.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::{bail, Result};
use matter_base::{
    ConnectionAdmission, ConnectionDriver, ConnectionService, ConnectionState,
    SessionCredentialSource, SessionRecord, SessionRegistry,
};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use zeroize::Zeroizing;

struct Policy(bool);
impl ConnectionAdmission for Policy {
    fn authorize(&self, _session: &SessionRecord, _actor: &str) -> Result<()> {
        if self.0 {
            Ok(())
        } else {
            bail!("private policy diagnostics")
        }
    }
}

struct Credentials {
    fail: bool,
    loads: AtomicUsize,
}
impl SessionCredentialSource for Credentials {
    fn load(&self, _session: &SessionRecord) -> Result<Zeroizing<Vec<u8>>> {
        self.loads.fetch_add(1, Ordering::SeqCst);
        if self.fail {
            bail!("private secret lookup details")
        }
        Ok(Zeroizing::new(vec![0x34, 0x59, 0x12]))
    }
}

struct Driver {
    connects: AtomicUsize,
    disconnects: AtomicUsize,
    fail_connect: bool,
    live: AtomicBool,
}
impl Driver {
    fn new(fail_connect: bool) -> Self {
        Self {
            connects: AtomicUsize::new(0),
            disconnects: AtomicUsize::new(0),
            fail_connect,
            live: AtomicBool::new(true),
        }
    }
}
impl ConnectionDriver for Driver {
    fn connect(&self, _session: &SessionRecord, credential: &[u8]) -> Result<()> {
        assert_eq!(credential, [0x34, 0x59, 0x12]);
        self.connects.fetch_add(1, Ordering::SeqCst);
        if self.fail_connect {
            bail!("private connection failure")
        }
        self.live.store(true, Ordering::SeqCst);
        Ok(())
    }

    fn disconnect(&self, _session: &SessionRecord) -> Result<()> {
        self.disconnects.fetch_add(1, Ordering::SeqCst);
        self.live.store(false, Ordering::SeqCst);
        Ok(())
    }

    fn is_connected(&self, _session: &SessionRecord) -> Result<bool> {
        Ok(self.live.load(Ordering::SeqCst))
    }
}

fn service(
    sessions: Arc<SessionRegistry>,
    allowed: bool,
    credentials: Arc<Credentials>,
    driver: Arc<Driver>,
) -> ConnectionService {
    ConnectionService::new(sessions, Arc::new(Policy(allowed)), credentials, driver)
}

#[test]
fn connects_disconnects_with_authenticated_owner_and_durable_transitions() {
    let sessions = Arc::new(SessionRegistry::in_memory().unwrap());
    sessions.register("socket", "owner", "whatsapp").unwrap();
    let driver = Arc::new(Driver::new(false));
    let credentials = Arc::new(Credentials {
        fail: false,
        loads: AtomicUsize::new(0),
    });
    let svc = service(sessions.clone(), true, credentials.clone(), driver.clone());
    assert!(svc.connect("socket", "other").is_err());
    assert_eq!(credentials.loads.load(Ordering::SeqCst), 0);
    let record = svc.connect("socket", "owner").unwrap();
    assert_eq!(record.state, ConnectionState::Connected);
    assert_eq!(record.revision, 2);
    assert!(svc.check_health("socket", "owner").unwrap());
    assert!(svc.connect("socket", "owner").is_err());
    let stopped = svc.disconnect("socket", "owner").unwrap();
    assert_eq!(stopped.state, ConnectionState::Disconnected);
    assert_eq!(stopped.revision, 3);
    assert_eq!(driver.connects.load(Ordering::SeqCst), 1);
    assert_eq!(driver.disconnects.load(Ordering::SeqCst), 1);
    assert_eq!(svc.disconnect("socket", "owner").unwrap().revision, 3);
}

#[test]
fn denied_and_failed_credential_load_does_not_start_driver() {
    let sessions = Arc::new(SessionRegistry::in_memory().unwrap());
    sessions.register("socket", "owner", "telegram").unwrap();
    let driver = Arc::new(Driver::new(false));
    let creds = Arc::new(Credentials {
        fail: true,
        loads: AtomicUsize::new(0),
    });
    let denied = service(sessions.clone(), false, creds.clone(), driver.clone());
    assert_eq!(
        denied.connect("socket", "owner").unwrap_err().to_string(),
        "Connection access denied"
    );
    assert_eq!(creds.loads.load(Ordering::SeqCst), 0);
    let svc = service(sessions.clone(), true, creds.clone(), driver.clone());
    assert_eq!(
        svc.connect("socket", "owner").unwrap_err().to_string(),
        "Connection operation failed"
    );
    assert_eq!(
        sessions.inspect("socket", "owner").unwrap().state,
        ConnectionState::Faulted
    );
    assert_eq!(driver.connects.load(Ordering::SeqCst), 0);
}

#[test]
fn driver_failure_faults_state_without_leaking_details_and_allows_recovery() {
    let sessions = Arc::new(SessionRegistry::in_memory().unwrap());
    sessions.register("socket", "owner", "discord").unwrap();
    let creds = Arc::new(Credentials {
        fail: false,
        loads: AtomicUsize::new(0),
    });
    let failing = Arc::new(Driver::new(true));
    let svc = service(sessions.clone(), true, creds.clone(), failing.clone());
    let error = svc.connect("socket", "owner").unwrap_err().to_string();
    assert_eq!(error, "Connection operation failed");
    assert!(!error.contains("private"));
    assert_eq!(
        sessions.inspect("socket", "owner").unwrap().state,
        ConnectionState::Faulted
    );
    assert_eq!(failing.disconnects.load(Ordering::SeqCst), 1);
    let healthy = Arc::new(Driver::new(false));
    let recovered = service(sessions.clone(), true, creds, healthy);
    assert_eq!(
        recovered.connect("socket", "owner").unwrap().state,
        ConnectionState::Connected
    );
}

#[test]
fn health_failure_faults_and_requires_explicit_reconnection() {
    let sessions = Arc::new(SessionRegistry::in_memory().unwrap());
    sessions.register("socket", "owner", "whatsapp").unwrap();
    let driver = Arc::new(Driver::new(false));
    let creds = Arc::new(Credentials {
        fail: false,
        loads: AtomicUsize::new(0),
    });
    let svc = service(sessions.clone(), true, creds, driver.clone());
    svc.connect("socket", "owner").unwrap();
    driver.live.store(false, Ordering::SeqCst);
    assert!(!svc.check_health("socket", "owner").unwrap());
    assert_eq!(
        sessions.inspect("socket", "owner").unwrap().state,
        ConnectionState::Faulted
    );
    assert!(!svc.check_health("socket", "owner").unwrap());
    assert_eq!(
        svc.connect("socket", "owner").unwrap().state,
        ConnectionState::Connected
    );
}

#[test]
fn uncertain_state_can_be_marked_faulted_after_restart() {
    let sessions = Arc::new(SessionRegistry::in_memory().unwrap());
    sessions.register("socket", "owner", "whatsapp").unwrap();
    sessions
        .transition("socket", "owner", 0, ConnectionState::Connecting)
        .unwrap();
    let creds = Arc::new(Credentials {
        fail: false,
        loads: AtomicUsize::new(0),
    });
    let svc = service(sessions.clone(), true, creds, Arc::new(Driver::new(false)));
    assert_eq!(
        svc.mark_uncertain("socket", "owner").unwrap().state,
        ConnectionState::Faulted
    );
    assert!(svc.connect("socket", "owner").is_ok());
}
