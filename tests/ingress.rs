/*
 * Matter-Base — Authenticated inbound event and replay regression tests.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Result};
use matter_base::{
    activation::SocketActivationStore,
    control::{ControlDispatcher, DispatchResult},
    BotRegistry, CommandAuthorizer, CommandInvocation, CommandRouter,
    InboundAdmission, InboundAuthenticator, InboundEvent, InboundGateway,
    InboundOutcome, InboundReplayStore, VerifiedIngress,
};
use rusqlite::Connection;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

const KEY: [u8; 32] = [0x72; 32];

struct PermitCommands;
impl CommandAuthorizer for PermitCommands {
    fn authorize(&self, _inv: &CommandInvocation) -> Result<()> {
        Ok(())
    }
}

struct Auth {
    platform: &'static str,
    actor: &'static str,
}
impl InboundAuthenticator for Auth {
    fn authenticate(&self, _event: &InboundEvent) -> Result<VerifiedIngress> {
        Ok(VerifiedIngress {
            platform: self.platform.into(),
            socket_id: "socket".into(),
            actor_id: self.actor.into(),
            source_id: "room".into(),
            group_id: Some("room".into()),
        })
    }
}

struct Gate(AtomicBool);
impl InboundAdmission for Gate {
    fn authorize(&self, _event: &InboundEvent, _trusted: &VerifiedIngress) -> Result<()> {
        if self.0.load(Ordering::SeqCst) {
            Ok(())
        } else {
            bail!("private consent or provider details")
        }
    }
}

fn gateway(
    replay: Arc<InboundReplayStore>,
    gate: Arc<Gate>,
    auth: Arc<Auth>,
    calls: Arc<AtomicUsize>,
) -> (InboundGateway, Arc<SocketActivationStore>) {
    let activation = Arc::new(SocketActivationStore::open_in_memory().unwrap());
    activation.provision("socket", "owner").unwrap();
    let bots = Arc::new(BotRegistry::in_memory());
    let bot = bots.register("family", "v1", "bot", "owner").unwrap();
    activation
        .bind_bot("socket", "owner", bot.instance_id(), &bots)
        .unwrap();
    activation.set_socket_policy("socket", "owner", Some(true)).unwrap();
    let router = Arc::new(CommandRouter::new(
        vec!["#".into()],
        Arc::new(PermitCommands),
    ).unwrap());
    router.register("ping", &[], move |_| {
        calls.fetch_add(1, Ordering::SeqCst);
        Ok("pong".into())
    }).unwrap();
    let control = Arc::new(ControlDispatcher::new(
        activation.clone(),
        bots,
        router,
        None,
    ));
    (InboundGateway::new(auth, gate, replay, control), activation)
}

fn event(id: &str) -> InboundEvent {
    InboundEvent {
        platform: "whatsapp".into(),
        event_id: id.into(),
        text: "#ping".into(),
    }
}

#[test]
fn authenticated_gateway_gates_power_and_replays_before_handler_effects() {
    let replay = Arc::new(InboundReplayStore::in_memory(KEY).unwrap());
    let gate = Arc::new(Gate(AtomicBool::new(true)));
    let calls = Arc::new(AtomicUsize::new(0));
    let (gateway, activation) = gateway(
        replay,
        gate,
        Arc::new(Auth {
            platform: "whatsapp",
            actor: "member",
        }),
        calls.clone(),
    );
    assert_eq!(
        gateway.ingest(&event("off-event")).unwrap(),
        InboundOutcome::Dispatched(DispatchResult::Inactive)
    );
    activation.set_power("socket", "owner", true).unwrap();
    assert_eq!(
        gateway.ingest(&event("event-1")).unwrap(),
        InboundOutcome::Dispatched(DispatchResult::Message(Some("pong".into())))
    );
    assert_eq!(gateway.ingest(&event("event-1")).unwrap(), InboundOutcome::Duplicate);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(
        gateway.ingest(&event("off-event")).unwrap(),
        InboundOutcome::Duplicate
    );
}

#[test]
fn denial_and_platform_mismatch_do_not_claim_events() {
    let replay = Arc::new(InboundReplayStore::in_memory(KEY).unwrap());
    let gate = Arc::new(Gate(AtomicBool::new(false)));
    let calls = Arc::new(AtomicUsize::new(0));
    let (gateway, activation) = gateway(
        replay,
        gate.clone(),
        Arc::new(Auth {
            platform: "whatsapp",
            actor: "member",
        }),
        calls,
    );
    activation.set_power("socket", "owner", true).unwrap();
    assert_eq!(
        gateway.ingest(&event("event-x")).unwrap_err().to_string(),
        "Inbound admission denied"
    );
    gate.0.store(true, Ordering::SeqCst);
    assert!(gateway.ingest(&event("event-x")).is_ok());
    let replay = Arc::new(InboundReplayStore::in_memory(KEY).unwrap());
    let (wrong, _) = gateway(
        replay,
        gate,
        Arc::new(Auth {
            platform: "telegram",
            actor: "member",
        }),
        Arc::new(AtomicUsize::new(0)),
    );
    assert!(wrong.ingest(&event("event-x")).is_err());
}

#[test]
fn replay_ids_are_keyed_by_verified_platform_and_socket_and_not_stored_raw() {
    let mut nonce = [0_u8; 16];
    getrandom::fill(&mut nonce).unwrap();
    let path = std::env::temp_dir().join(format!("matter-ingress-{}.db", hex::encode(nonce)));
    let store = InboundReplayStore::open(&path, KEY).unwrap();
    assert!(store.claim("whatsapp", "socket-1", "secret-event-123").unwrap());
    assert!(!store.claim("whatsapp", "socket-1", "secret-event-123").unwrap());
    assert!(store.claim("telegram", "socket-1", "secret-event-123").unwrap());
    assert!(store.claim("whatsapp", "socket-2", "secret-event-123").unwrap());
    let conn = Connection::open(&path).unwrap();
    let fingerprint: String = conn
        .query_row("SELECT fingerprint FROM inbound_replay LIMIT 1", [], |r| r.get(0))
        .unwrap();
    assert!(!fingerprint.contains("secret-event-123"));
    drop(conn);
    drop(store);
    let reopened = InboundReplayStore::open(&path, KEY).unwrap();
    assert!(!reopened.claim("whatsapp", "socket-1", "secret-event-123").unwrap());
    drop(reopened);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn concurrent_claims_across_sqlite_connections_have_one_winner() {
    let mut nonce = [0_u8; 16];
    getrandom::fill(&mut nonce).unwrap();
    let path = std::env::temp_dir().join(format!("matter-replay-{}.db", hex::encode(nonce)));
    {
        let first = InboundReplayStore::open(&path, KEY).unwrap();
        let second = InboundReplayStore::open(&path, KEY).unwrap();
        let first_result = first.claim("discord", "socket", "event").unwrap();
        let second_result = second.claim("discord", "socket", "event").unwrap();
        assert!(first_result);
        assert!(!second_result);
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn empty_replay_key_and_oversized_events_are_rejected() {
    assert!(InboundReplayStore::in_memory([0_u8; 32]).is_err());
    let store = InboundReplayStore::in_memory(KEY).unwrap();
    assert!(store.claim("whatsapp", "socket", "").is_err());
    let too_long = "x".repeat(16 * 1024 + 1);
    let gateway = {
        let replay = Arc::new(store);
        gateway(
            replay,
            Arc::new(Gate(AtomicBool::new(true))),
            Arc::new(Auth {
                platform: "whatsapp",
                actor: "member",
            }),
            Arc::new(AtomicUsize::new(0)),
        ).0
    };
    let mut raw = event("too-long");
    raw.text = too_long;
    assert!(gateway.ingest(&raw).is_err());
}
