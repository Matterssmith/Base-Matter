/*
 * Matter-Base — Cross-component outbound admission regressions.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::Result;
use matter_base::{
    activation::SocketActivationStore, BotRegistry, ConnectionState, ControlPlaneMessageAdmission,
    MessageAdmission, MessageService, OutboundMessage, RoleStore, RuleEffect, SessionRecord,
    SessionRegistry, TransportDriver,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

struct Policy(bool);

impl MessageAdmission for Policy {
    fn authorize(&self, _message: &OutboundMessage) -> Result<()> {
        if self.0 {
            Ok(())
        } else {
            anyhow::bail!("external legal or provider policy denied")
        }
    }
}

struct Driver(AtomicUsize);

impl TransportDriver for Driver {
    fn transmit(&self, _session: &SessionRecord, _message: &OutboundMessage) -> Result<String> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok("received".to_owned())
    }
}

struct Fixture {
    activation: Arc<SocketActivationStore>,
    sessions: Arc<SessionRegistry>,
    roles: Arc<RoleStore>,
    bots: Arc<BotRegistry>,
    bot_id: String,
    db_path: std::path::PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let mut random = [0_u8; 16];
        getrandom::fill(&mut random).unwrap();
        let db_path =
            std::env::temp_dir().join(format!("matter-outbound-{}.db", hex::encode(random)));
        let activation = Arc::new(SocketActivationStore::open(&db_path).unwrap());
        let sessions = Arc::new(SessionRegistry::open(&db_path).unwrap());
        let roles = Arc::new(RoleStore::open(&db_path).unwrap());
        let bots = Arc::new(BotRegistry::in_memory());
        let bot = bots.register("family", "v1", "bot", "owner").unwrap();
        let bot_id = bot.instance_id().to_owned();
        activation.provision("socket", "owner").unwrap();
        activation
            .bind_bot("socket", "owner", &bot_id, &bots)
            .unwrap();
        activation
            .set_socket_policy("socket", "owner", Some(true))
            .unwrap();
        sessions.register("socket", "owner", "whatsapp").unwrap();
        sessions
            .transition("socket", "owner", 0, ConnectionState::Connecting)
            .unwrap();
        sessions
            .transition("socket", "owner", 1, ConnectionState::Connected)
            .unwrap();
        roles.provision_scope(&bot_id, "owner").unwrap();
        roles.create_role(&bot_id, "owner", "sender", None).unwrap();
        roles
            .set_rule(
                &bot_id,
                "owner",
                "sender",
                "message:send",
                "execute",
                RuleEffect::Allow,
            )
            .unwrap();
        roles.grant(&bot_id, "owner", "member", "sender").unwrap();
        Self {
            activation,
            sessions,
            roles,
            bots,
            bot_id,
            db_path,
        }
    }

    fn service(&self, additional: bool, driver: Arc<Driver>) -> MessageService {
        let policy = Arc::new(ControlPlaneMessageAdmission::new_with_persistent_policy(
            self.activation.clone(),
            self.sessions.clone(),
            self.bots.clone(),
            self.roles.clone(),
            Arc::new(Policy(additional)),
        ));
        MessageService::new(self.sessions.clone(), policy, driver)
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.db_path);
    }
}

fn outbound() -> OutboundMessage {
    OutboundMessage {
        socket_id: "socket".to_owned(),
        actor_id: "member".to_owned(),
        destination_id: "group-a".to_owned(),
        group_id: Some("group-a".to_owned()),
        text: "hello".to_owned(),
    }
}

#[test]
fn outbound_respects_activation_group_policy_and_role_revocation() {
    let f = Fixture::new();
    let driver = Arc::new(Driver(AtomicUsize::new(0)));
    let service = f.service(true, driver.clone());
    assert!(service.send(&outbound()).is_err());
    assert_eq!(driver.0.load(Ordering::SeqCst), 0);
    f.activation.set_power("socket", "owner", true).unwrap();
    assert_eq!(
        service.send(&outbound()).unwrap().driver_receipt,
        "received"
    );
    f.activation
        .set_group_policy("socket", "owner", "group-a", Some(false))
        .unwrap();
    assert!(service.send(&outbound()).is_err());
    f.activation
        .set_group_policy("socket", "owner", "group-a", None)
        .unwrap();
    f.roles
        .revoke(&f.bot_id, "owner", "member", "sender")
        .unwrap();
    assert!(service.send(&outbound()).is_err());
    assert_eq!(driver.0.load(Ordering::SeqCst), 1);
}

#[test]
fn external_policy_and_session_owner_disagreement_fail_closed() {
    let f = Fixture::new();
    f.activation.set_power("socket", "owner", true).unwrap();
    let driver = Arc::new(Driver(AtomicUsize::new(0)));
    let denied = f.service(false, driver.clone());
    assert_eq!(
        denied.send(&outbound()).unwrap_err().to_string(),
        "Message admission denied"
    );
    let allowed = f.service(true, driver.clone());
    assert!(allowed.send(&outbound()).is_ok());
    let untrusted = Arc::new(SessionRegistry::in_memory().unwrap());
    untrusted
        .register("socket", "other-owner", "whatsapp")
        .unwrap();
    untrusted
        .transition("socket", "other-owner", 0, ConnectionState::Connecting)
        .unwrap();
    untrusted
        .transition("socket", "other-owner", 1, ConnectionState::Connected)
        .unwrap();
    let mismatched = MessageService::new(
        untrusted.clone(),
        Arc::new(ControlPlaneMessageAdmission::new_with_persistent_policy(
            f.activation.clone(),
            untrusted,
            f.bots.clone(),
            f.roles.clone(),
            Arc::new(Policy(true)),
        )),
        driver.clone(),
    );
    assert!(mismatched.send(&outbound()).is_err());
    assert_eq!(driver.0.load(Ordering::SeqCst), 1);
}

#[test]
fn power_off_and_released_binding_prevent_new_sends() {
    let f = Fixture::new();
    let driver = Arc::new(Driver(AtomicUsize::new(0)));
    let service = f.service(true, driver.clone());
    f.activation.set_power("socket", "owner", true).unwrap();
    service.send(&outbound()).unwrap();
    f.activation.set_power("socket", "owner", false).unwrap();
    assert!(service.send(&outbound()).is_err());
    f.activation.release_binding("socket", "owner").unwrap();
    f.activation.set_power("socket", "owner", true).unwrap();
    assert!(service.send(&outbound()).is_err());
    assert_eq!(driver.0.load(Ordering::SeqCst), 1);
}
