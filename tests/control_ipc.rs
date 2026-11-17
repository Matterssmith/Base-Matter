/*
 * Base-Matter — authenticated in-process loopback server regressions.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::Result;
use hmac::{Hmac, Mac};
use matter_base::{
    activation::SocketActivationStore, control::ControlDispatcher, BotRegistry, CommandAuthorizer,
    CommandInvocation, CommandRouter, ControlIpcOutcome, InboundReplayStore, WhatsAppBridgeConfig,
    WhatsAppControlBridge, WhatsAppControlIpc,
};
use sha2::Sha256;
use std::sync::Arc;
use time::OffsetDateTime;
const JID: &str = "5215555550000@s.whatsapp.net";
const KEY: [u8; 32] = [7; 32];
struct DenyBot;
impl CommandAuthorizer for DenyBot {
    fn authorize(&self, _: &CommandInvocation) -> Result<()> {
        anyhow::bail!("Bot not available")
    }
}
fn make() -> (WhatsAppControlIpc, Arc<SocketActivationStore>) {
    let activation = Arc::new(SocketActivationStore::open(":memory:").unwrap());
    activation.provision("socket-a", "alice").unwrap();
    let router = Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(DenyBot)).unwrap());
    let dispatch = Arc::new(ControlDispatcher::new(
        activation.clone(),
        Arc::new(BotRegistry::in_memory()),
        router,
        None,
    ));
    let bridge = Arc::new(
        WhatsAppControlBridge::new(
            WhatsAppBridgeConfig {
                socket_id: "socket-a".into(),
                owner_actor_id: "alice".into(),
                owner_jid: JID.into(),
            },
            KEY,
            Arc::new(InboundReplayStore::in_memory([8; 32]).unwrap()),
            dispatch,
        )
        .unwrap(),
    );
    (WhatsAppControlIpc::new(bridge), activation)
}
fn signed(id: &str, text: &str) -> Vec<u8> {
    let timestamp = OffsetDateTime::now_utc().unix_timestamp();
    let values = ["1", "socket-a", "alice", JID, JID, id, text];
    let mut mac = Hmac::<Sha256>::new_from_slice(&KEY).unwrap();
    mac.update(b"Base-Matter:WhatsApp-Control:v1\0");
    for value in values {
        let bytes = value.as_bytes();
        mac.update(&(bytes.len() as u32).to_be_bytes());
        mac.update(bytes);
    }
    mac.update(&[1]);
    mac.update(&timestamp.to_be_bytes());
    let signature = hex::encode(mac.finalize().into_bytes());
    serde_json::to_vec(&serde_json::json!({
        "version":1,"socket_id":"socket-a","actor_id":"alice",
        "owner_jid":JID,"chat_jid":JID,"message_id":id,
        "text":text,"from_me":true,"timestamp":timestamp,"signature":signature,
    }))
    .unwrap()
}
#[test]
fn authenticated_wire_changes_socket_only_once() {
    let (service, activation) = make();
    let message = signed("one", "=/>on");
    assert_eq!(service.dispatch(&message), ControlIpcOutcome::Applied);
    assert_eq!(service.dispatch(&message), ControlIpcOutcome::Duplicate);
    assert_eq!(
        service.dispatch(&signed("two", "=/>status")),
        ControlIpcOutcome::Applied
    );
    assert_eq!(
        service.dispatch(&signed("three", "=/>off")),
        ControlIpcOutcome::Applied
    );
    assert!(
        !activation
            .resolve_persisted("socket-a", None)
            .unwrap()
            .enabled
    );
}
#[test]
fn invalid_frames_and_ping_are_denied_without_leaks() {
    let (service, _) = make();
    assert_eq!(service.dispatch(b"{"), ControlIpcOutcome::Denied);
    assert_eq!(
        service.dispatch(&vec![b'a'; 16385]),
        ControlIpcOutcome::Denied
    );
    let mut forged = signed("one", "=/>on");
    let i = forged.iter().position(|byte| *byte == b'a').unwrap_or(0);
    forged[i] = b'z';
    assert_eq!(service.dispatch(&forged), ControlIpcOutcome::Denied);
    assert_eq!(
        service.dispatch(&signed("two", "=/>ping")),
        ControlIpcOutcome::Denied
    );
    assert_eq!(
        service.dispatch(&signed("three", "#ping")),
        ControlIpcOutcome::Ignored
    );
}
#[test]
fn cannot_bind_public_network_address() {
    let (service, _) = make();
    assert!(service.serve("0.0.0.0:12345".parse().unwrap()).is_err());
    assert!(service.serve("127.0.0.1:0".parse().unwrap()).is_err());
}
