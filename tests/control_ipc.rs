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

#[test]
fn local_tcp_loopback_roundtrip_rejects_replay_before_second_effect() {
    use std::io::{Read, Write};
    use std::net::{Shutdown, TcpListener, TcpStream};
    let (service, activation) = make();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let server = std::thread::spawn(move || {
        for _ in 0..3 {
            let (stream, _) = listener.accept().unwrap();
            service.accept(stream).unwrap();
        }
    });
    let frames = [
        (signed("network-a", "=/>on"), "applied"),
        (signed("network-a", "=/>on"), "duplicate"),
        (signed("network-b", "=/>off"), "applied"),
    ];
    for (frame, outcome) in frames {
        let mut client = TcpStream::connect(addr).unwrap();
        client.write_all(&frame).unwrap();
        client.shutdown(Shutdown::Write).unwrap();
        let mut bytes = Vec::new();
        client.read_to_end(&mut bytes).unwrap();
        let response: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(response["ok"], true);
        assert_eq!(response["outcome"], outcome);
        if outcome == "applied" {
            assert!(response["message"]
                .as_str()
                .is_some_and(|text| text.starts_with("Base-Matter: socket")));
        } else {
            assert!(response.get("message").is_none());
        }
        assert!(!bytes.windows(7).any(|window| window == b"socket-"));
    }
    server.join().unwrap();
    assert!(
        !activation
            .resolve_persisted("socket-a", None)
            .unwrap()
            .enabled
    );
}

#[test]
fn incomplete_or_oversized_tcp_frames_fail_closed() {
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::time::{Duration, Instant};

    let (service, activation) = make();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let worker = std::thread::spawn(move || {
        for _ in 0..2 {
            let (stream, _) = listener.accept().unwrap();
            service.accept(stream).unwrap();
        }
    });

    let mut partial = TcpStream::connect(address).unwrap();
    partial.write_all(b"{").unwrap();
    partial
        .set_read_timeout(Some(Duration::from_secs(6)))
        .unwrap();
    let since = Instant::now();
    let mut response = Vec::new();
    partial.read_to_end(&mut response).unwrap();
    assert!(since.elapsed() < Duration::from_secs(6));
    let response: serde_json::Value = serde_json::from_slice(&response).unwrap();
    assert_eq!(response["ok"], false);
    assert_eq!(response["outcome"], "denied");
    drop(partial);

    let mut oversized = TcpStream::connect(address).unwrap();
    oversized.write_all(&vec![b'x'; 16 * 1024 + 1]).unwrap();
    oversized.shutdown(std::net::Shutdown::Write).unwrap();
    let mut response = Vec::new();
    oversized.read_to_end(&mut response).unwrap();
    let response: serde_json::Value = serde_json::from_slice(&response).unwrap();
    assert_eq!(response["ok"], false);
    assert_eq!(response["outcome"], "denied");
    worker.join().unwrap();
    assert!(
        !activation
            .resolve_persisted("socket-a", None)
            .unwrap()
            .enabled
    );
}
