/*
 * Base-Matter — pairing-code-only and global control regressions.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::{bail, Result};
use matter_base::{
    activation::SocketActivationStore,
    control::{ControlDispatcher, DispatchResult},
    BotRegistry, CommandAuthorizer, CommandInvocation, CommandRouter, ConnectionAdmission,
    ConnectionState, PairingCodeProvider, PairingCodeService, SessionRecord, SessionRegistry,
};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

struct Auth(bool);
impl ConnectionAdmission for Auth {
    fn authorize(&self, _session: &SessionRecord, _actor: &str) -> Result<()> {
        if self.0 {
            Ok(())
        } else {
            bail!("Private admission diagnostic");
        }
    }
}

struct Provider {
    attempts: AtomicUsize,
    code: &'static str,
}
impl PairingCodeProvider for Provider {
    fn request_code(&self, _session: &SessionRecord, phone: &str) -> Result<String> {
        assert_eq!(phone, "5215512345678");
        self.attempts.fetch_add(1, Ordering::SeqCst);
        Ok(self.code.to_owned())
    }
}

fn fixture(
    platform: &str,
    allow: bool,
    code: &'static str,
) -> (Arc<SessionRegistry>, PairingCodeService, Arc<Provider>) {
    let sessions = Arc::new(SessionRegistry::in_memory().unwrap());
    sessions.register("socket", "owner", platform).unwrap();
    let provider = Arc::new(Provider {
        attempts: AtomicUsize::new(0),
        code,
    });
    let service =
        PairingCodeService::new(sessions.clone(), Arc::new(Auth(allow)), provider.clone());
    (sessions, service, provider)
}

#[test]
fn owner_can_request_transient_whatsapp_pairing_code_without_logging() {
    let (_sessions, service, provider) = fixture("whatsapp", true, "AB12CD34");
    let code = service
        .request_code("socket", "owner", "5215512345678")
        .unwrap();
    assert_eq!(code.expose_to_authenticated_owner(), "AB12CD34");
    assert!(!format!("{code:?}").contains("AB12CD34"));
    assert_eq!(provider.attempts.load(Ordering::SeqCst), 1);
}

#[test]
fn nonowner_invalid_phone_and_policy_denial_never_call_provider() {
    let (_sessions, service, provider) = fixture("whatsapp", true, "AB12CD34");
    assert!(service
        .request_code("socket", "intruder", "5215512345678")
        .is_err());
    for phone in [
        "",
        "+5215512345678",
        "123",
        "5215512345.78",
        "0000000000000000",
    ] {
        assert!(service.request_code("socket", "owner", phone).is_err());
    }
    assert_eq!(provider.attempts.load(Ordering::SeqCst), 0);
    let (_, denied, other) = fixture("whatsapp", false, "AB12CD34");
    assert_eq!(
        denied
            .request_code("socket", "owner", "5215512345678")
            .unwrap_err()
            .to_string(),
        "Pairing request denied"
    );
    assert_eq!(other.attempts.load(Ordering::SeqCst), 0);
}

#[test]
fn refuses_other_platforms_and_already_connected_sessions() {
    let (_sessions, service, provider) = fixture("telegram", true, "AB12CD34");
    assert!(service
        .request_code("socket", "owner", "5215512345678")
        .is_err());
    assert_eq!(provider.attempts.load(Ordering::SeqCst), 0);
    let (sessions, service, provider) = fixture("whatsapp", true, "AB12CD34");
    sessions
        .transition("socket", "owner", 0, ConnectionState::Connecting)
        .unwrap();
    sessions
        .transition("socket", "owner", 1, ConnectionState::Connected)
        .unwrap();
    assert!(service
        .request_code("socket", "owner", "5215512345678")
        .is_err());
    assert_eq!(provider.attempts.load(Ordering::SeqCst), 0);
}

#[test]
fn invalid_provider_codes_fail_closed() {
    for bad in ["", "123", "abc-def", "12345678901234567"] {
        let (_sessions, service, _) = fixture("whatsapp", true, bad);
        assert_eq!(
            service
                .request_code("socket", "owner", "5215512345678")
                .unwrap_err()
                .to_string(),
            "Invalid pairing code from provider"
        );
    }
}

struct NoCommands;
impl CommandAuthorizer for NoCommands {
    fn authorize(&self, _inv: &CommandInvocation) -> Result<()> {
        Ok(())
    }
}

#[test]
fn global_prefix_controls_unbound_socket_without_bot_or_ping_handler() {
    let activation = Arc::new(SocketActivationStore::open(":memory:").unwrap());
    activation.provision("socket", "owner").unwrap();
    let router = Arc::new(CommandRouter::new(vec!["#".to_owned()], Arc::new(NoCommands)).unwrap());
    let dispatcher = ControlDispatcher::new(
        activation.clone(),
        Arc::new(BotRegistry::in_memory()),
        router,
        None,
    );
    assert_eq!(
        dispatcher
            .dispatch("socket", "owner", "room", None, "=/>on")
            .unwrap(),
        DispatchResult::Control {
            socket_id: "socket".to_owned(),
            powered: true,
        }
    );
    assert!(dispatcher
        .dispatch("socket", "owner", "room", None, "=/>ping")
        .is_err());
    assert!(dispatcher
        .dispatch("socket", "intruder", "room", None, "=/>off")
        .is_err());
    assert_eq!(
        dispatcher
            .dispatch("socket", "owner", "room", None, "=/>off")
            .unwrap(),
        DispatchResult::Control {
            socket_id: "socket".to_owned(),
            powered: false,
        }
    );
    assert_eq!(
        dispatcher
            .dispatch("socket", "owner", "room", None, "#ping")
            .unwrap(),
        DispatchResult::Inactive
    );
}
