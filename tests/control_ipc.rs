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
    let dispatch = Arc::new(
        ControlDispatcher::new(
            activation.clone(),
            Arc::new(BotRegistry::in_memory()),
            router,
            None,
        )
        .with_unified_live_controls(),
    );
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
    assert_eq!(
        service.dispatch(&signed("disabled-on", "=/>bot on")),
        ControlIpcOutcome::Ignored
    );
    activation
        .set_live_bot_policy("socket-a", "alice", Some("kind:self"), Some(true))
        .unwrap();
    let message = signed("one", "=/>bot on");
    assert_eq!(service.dispatch(&message), ControlIpcOutcome::Applied);
    assert_eq!(service.dispatch(&message), ControlIpcOutcome::Duplicate);
    assert_eq!(
        service.dispatch(&signed("two", "=/>status")),
        ControlIpcOutcome::Applied
    );
    assert_eq!(
        service.dispatch(&signed("three", "=/>bot off")),
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
fn invalid_frames_are_denied_and_authenticated_ping_is_readonly() {
    let (service, activation) = make();
    assert_eq!(
        service.dispatch(&signed("disabled-ping", "=/>ping")),
        ControlIpcOutcome::Ignored
    );
    activation
        .set_live_bot_policy("socket-a", "alice", Some("kind:self"), Some(true))
        .unwrap();
    assert_eq!(service.dispatch(b"{"), ControlIpcOutcome::Denied);
    assert_eq!(
        service.dispatch(&vec![b'a'; 16385]),
        ControlIpcOutcome::Denied
    );
    let mut forged = signed("one", "=/>bot on");
    let i = forged.iter().position(|byte| *byte == b'a').unwrap_or(0);
    forged[i] = b'z';
    assert_eq!(service.dispatch(&forged), ControlIpcOutcome::Denied);
    assert_eq!(
        service.dispatch(&signed("two", "=/>ping")),
        ControlIpcOutcome::Applied
    );
    assert_eq!(
        service.dispatch(&signed("two", "=/>ping")),
        ControlIpcOutcome::Duplicate
    );
    assert_eq!(
        service.dispatch(&signed("ping-extra", "=/>ping extra")),
        ControlIpcOutcome::Denied
    );
    assert_eq!(
        service.dispatch(&signed("old-on", "=/>on")),
        ControlIpcOutcome::Denied
    );
    assert_eq!(
        service.dispatch(&signed("old-off", "=/>off")),
        ControlIpcOutcome::Denied
    );
    assert_eq!(
        service.dispatch(&signed("three", "#ping")),
        ControlIpcOutcome::Ignored
    );
}
#[test]
fn external_private_default_does_not_disable_owner_self_chat_and_self_off_is_silent() {
    use matter_base::ControlExtensionRegistry;
    let activation = Arc::new(SocketActivationStore::open(":memory:").unwrap());
    activation.provision("socket-a", "alice").unwrap();
    let router = Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(DenyBot)).unwrap());
    let extensions = Arc::new(ControlExtensionRegistry::new());
    extensions
        .register("menu", &[], |_, _| Ok("menu".to_owned()))
        .unwrap();
    let control = Arc::new(
        ControlDispatcher::new(
            Arc::clone(&activation),
            Arc::new(BotRegistry::in_memory()),
            router,
            None,
        )
        .with_control_extensions(extensions)
        .with_silent_bot_commands()
        .with_unified_live_controls(),
    );
    let bridge = Arc::new(
        WhatsAppControlBridge::new(
            WhatsAppBridgeConfig {
                socket_id: "socket-a".into(),
                owner_actor_id: "alice".into(),
                owner_jid: JID.into(),
            },
            KEY,
            Arc::new(InboundReplayStore::in_memory([8; 32]).unwrap()),
            control,
        )
        .unwrap(),
    );
    let service = WhatsAppControlIpc::new(bridge);
    assert_eq!(
        service.dispatch(&signed("m-1", "=/>menu")),
        ControlIpcOutcome::Ignored
    );
    assert_eq!(
        service.dispatch(&signed("s-1", "=/>status")),
        ControlIpcOutcome::Applied
    );
    activation
        .set_live_bot_policy("socket-a", "alice", None, Some(true))
        .unwrap();
    assert_eq!(
        service.dispatch(&signed("b-1", "=/>bot --mod on")),
        ControlIpcOutcome::Applied
    );

    assert_eq!(
        service.dispatch(&signed("m-2", "=/>menu")),
        ControlIpcOutcome::Applied
    );
    // "private" is the default for *external* private chats only.
    // It must not disable the paired owner's self-chat control surface.
    assert_eq!(
        service.dispatch(&signed("b-2", "=/>bot private off")),
        ControlIpcOutcome::Applied
    );
    assert_eq!(
        service.dispatch(&signed("m-3", "=/>menu")),
        ControlIpcOutcome::Applied
    );
    assert_eq!(
        service.dispatch(&signed("s-2", "=/>status")),
        ControlIpcOutcome::Applied
    );
    // An explicit self-chat override CAN disable its ordinary menu.
    assert_eq!(
        service.dispatch(&signed("b-3", "=/>bot off")),
        ControlIpcOutcome::Applied
    );
    assert_eq!(
        service.dispatch(&signed("m-4", "=/>menu")),
        ControlIpcOutcome::Ignored
    );
    // Status alone stays reachable while self-chat is off.
    assert_eq!(
        service.dispatch(&signed("s-3", "=/>status")),
        ControlIpcOutcome::Applied
    );
    assert_eq!(
        service.dispatch(&signed("b-4", "=/>bot inherit")),
        ControlIpcOutcome::Ignored
    );
    activation
        .set_live_bot_policy("socket-a", "alice", Some("kind:self"), None)
        .unwrap();
    assert_eq!(
        service.dispatch(&signed("m-5", "=/>menu")),
        ControlIpcOutcome::Applied
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
    activation
        .set_live_bot_policy("socket-a", "alice", Some("kind:self"), Some(true))
        .unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let server = std::thread::spawn(move || {
        for _ in 0..3 {
            let (stream, _) = listener.accept().unwrap();
            service.accept(stream).unwrap();
        }
    });
    let frames = [
        (signed("network-a", "=/>bot on"), "applied"),
        (signed("network-a", "=/>bot on"), "duplicate"),
        (signed("network-b", "=/>bot off"), "applied"),
    ];
    // Keep the integration assertion aligned with the actual control
    // template; the whitespace between frame glyphs is presentation, not
    // an IPC contract. Still validate frame, content, and actual newlines.
    // The runtime normalizes checked-out templates to LF on Windows.
    // Compare with the same canonical newlines in this integration test.
    let template = matter_base::classic_template::expand(include_str!("../templates/matterssmith-control.txt"));
    let (frame_prefix, frame_suffix) = template.split_once("{{BODY}}").unwrap();
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
            assert!(
                response["message"].as_str().is_some_and(|text| {
                    text.starts_with(frame_prefix)
                        && text.ends_with(frame_suffix)
                        && text.contains("Política del bot para")
                        && text.contains('\n')
                        && !text.contains("\\n")
                }),
                "Authenticated IPC response does not match the configured control frame"
            );
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
