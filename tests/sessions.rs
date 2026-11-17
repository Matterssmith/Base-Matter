/*
 * Matter-Base — Session and message transport tests.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::{bail, Result};
use matter_base::{
    ConnectionState, MessageAdmission, MessageService, OutboundMessage, SessionRecord,
    SessionRegistry, TransportDriver,
};
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier};

struct Database(PathBuf);

impl Database {
    fn new() -> Self {
        let mut id = [0_u8; 16];
        getrandom::fill(&mut id).unwrap();
        Self(std::env::temp_dir().join(format!("matter-sessions-{}.db", hex::encode(id))))
    }

    fn store(&self) -> SessionRegistry {
        SessionRegistry::open(&self.0).unwrap()
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

struct Policy(bool);

impl MessageAdmission for Policy {
    fn authorize(&self, _message: &OutboundMessage) -> Result<()> {
        if self.0 {
            Ok(())
        } else {
            bail!("private policy details")
        }
    }
}

struct Driver {
    called: AtomicUsize,
    fail: bool,
}

impl TransportDriver for Driver {
    fn transmit(&self, _session: &SessionRecord, _message: &OutboundMessage) -> Result<String> {
        self.called.fetch_add(1, Ordering::SeqCst);
        if self.fail {
            bail!("private transport diagnostics")
        }
        Ok("driver-ack".to_owned())
    }
}

fn message() -> OutboundMessage {
    OutboundMessage {
        socket_id: "socket-1".to_owned(),
        actor_id: "owner-1".to_owned(),
        destination_id: "group-1".to_owned(),
        text: "hello".to_owned(),
    }
}

#[test]
fn state_transitions_are_owner_scoped_versioned_and_durable() {
    let db = Database::new();
    let registry = db.store();
    let original = registry
        .register("socket-1", "owner-1", "whatsapp")
        .unwrap();
    assert_eq!(original.revision, 0);
    assert_eq!(original.state, ConnectionState::Disconnected);
    assert!(registry
        .register("socket-1", "owner-2", "whatsapp")
        .is_err());
    assert!(registry.inspect("socket-1", "owner-2").is_err());
    assert!(registry
        .transition("socket-1", "owner-1", 0, ConnectionState::Connected)
        .is_err());
    let connecting = registry
        .transition("socket-1", "owner-1", 0, ConnectionState::Connecting)
        .unwrap();
    assert_eq!(connecting.revision, 1);
    assert!(registry
        .transition("socket-1", "owner-1", 0, ConnectionState::Faulted)
        .is_err());
    registry
        .transition("socket-1", "owner-1", 1, ConnectionState::Connected)
        .unwrap();
    drop(registry);
    let restored = db.store();
    assert_eq!(restored.inspect("socket-1", "owner-1").unwrap().revision, 2);
    assert_eq!(
        restored
            .audit_history("socket-1", "owner-1", 10, 0)
            .unwrap()
            .len(),
        3
    );
    assert!(restored
        .audit_history("socket-1", "owner-2", 10, 0)
        .is_err());
}

#[test]
fn independent_connections_cannot_override_a_newer_revision() {
    let db = Database::new();
    db.store()
        .register("socket-1", "owner-1", "telegram")
        .unwrap();
    let start = Arc::new(Barrier::new(3));
    let mut threads = Vec::new();
    for _ in 0..2 {
        let path = db.0.clone();
        let gate = Arc::clone(&start);
        threads.push(std::thread::spawn(move || {
            let registry = SessionRegistry::open(path).unwrap();
            gate.wait();
            registry.transition("socket-1", "owner-1", 0, ConnectionState::Connecting)
        }));
    }
    start.wait();
    let successes = threads
        .into_iter()
        .filter(|thread| thread.join().unwrap().is_ok())
        .count();
    assert_eq!(successes, 1);
    assert_eq!(
        db.store().inspect("socket-1", "owner-1").unwrap().revision,
        1
    );
}

#[test]
fn audit_failure_prevents_state_mutation() {
    let db = Database::new();
    let registry = db.store();
    registry.register("socket-1", "owner-1", "discord").unwrap();
    let conn = Connection::open(&db.0).unwrap();
    conn.execute_batch(
        "CREATE TRIGGER reject_session_audit BEFORE INSERT ON session_audit
         BEGIN SELECT RAISE(ABORT,'injected'); END;",
    )
    .unwrap();
    assert!(registry
        .transition("socket-1", "owner-1", 0, ConnectionState::Connecting)
        .is_err());
    assert_eq!(registry.inspect("socket-1", "owner-1").unwrap().revision, 0);
    assert!(conn.execute("DELETE FROM session_audit", []).is_err());
}

#[test]
fn delivery_requires_admission_and_connected_state_before_driver() {
    let sessions = Arc::new(SessionRegistry::in_memory().unwrap());
    sessions
        .register("socket-1", "owner-1", "whatsapp")
        .unwrap();
    let driver = Arc::new(Driver {
        called: AtomicUsize::new(0),
        fail: false,
    });
    let denied = MessageService::new(
        Arc::clone(&sessions),
        Arc::new(Policy(false)),
        driver.clone(),
    );
    assert_eq!(
        denied.send(&message()).unwrap_err().to_string(),
        "Message admission denied"
    );
    let allowed = MessageService::new(
        Arc::clone(&sessions),
        Arc::new(Policy(true)),
        driver.clone(),
    );
    assert!(allowed.send(&message()).is_err());
    assert_eq!(driver.called.load(Ordering::SeqCst), 0);
    sessions
        .transition("socket-1", "owner-1", 0, ConnectionState::Connecting)
        .unwrap();
    sessions
        .transition("socket-1", "owner-1", 1, ConnectionState::Connected)
        .unwrap();
    let sent = allowed.send(&message()).unwrap();
    assert_eq!(sent.driver_receipt, "driver-ack");
    assert_eq!(sent.session_revision, 2);
    assert_eq!(driver.called.load(Ordering::SeqCst), 1);
}

#[test]
fn transport_errors_are_sanitized_and_payloads_not_persisted() {
    let sessions = Arc::new(SessionRegistry::in_memory().unwrap());
    sessions
        .register("socket-1", "owner-1", "whatsapp")
        .unwrap();
    sessions
        .transition("socket-1", "owner-1", 0, ConnectionState::Connecting)
        .unwrap();
    sessions
        .transition("socket-1", "owner-1", 1, ConnectionState::Connected)
        .unwrap();
    let failed = MessageService::new(
        sessions.clone(),
        Arc::new(Policy(true)),
        Arc::new(Driver {
            called: AtomicUsize::new(0),
            fail: true,
        }),
    );
    let mut secret = message();
    secret.text = "super-private-content".to_owned();
    let error = failed.send(&secret).unwrap_err().to_string();
    assert_eq!(error, "Transport operation failed");
    assert!(!error.contains("private"));
    let history = sessions
        .audit_history("socket-1", "owner-1", 10, 0)
        .unwrap();
    assert!(!format!("{history:?}").contains("super-private-content"));
}

#[test]
fn oversized_messages_fail_before_transport() {
    let sessions = Arc::new(SessionRegistry::in_memory().unwrap());
    let driver = Arc::new(Driver {
        called: AtomicUsize::new(0),
        fail: false,
    });
    let service = MessageService::new(sessions, Arc::new(Policy(true)), driver.clone());
    let mut outbound = message();
    outbound.text = "X".repeat(16 * 1024 + 1);
    assert!(service.send(&outbound).is_err());
    assert_eq!(driver.called.load(Ordering::SeqCst), 0);
}
