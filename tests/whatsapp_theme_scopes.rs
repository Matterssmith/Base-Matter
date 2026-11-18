/*
 * Base-Matter — every authenticated control response uses the same
 * externally defined, verified-chat style template.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Result};
use hmac::{Hmac, Mac};
use matter_base::{
    activation::SocketActivationStore,
    control::ControlDispatcher,
    control_style::{ControlTheme, ControlThemes},
    BotRegistry, CommandAuthorizer, CommandInvocation, CommandRouter, InboundReplayStore,
    WhatsAppBridgeConfig, WhatsAppControlBridge, WhatsAppControlIpc,
};
use sha2::Sha256;
use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    sync::Arc,
};
use time::OffsetDateTime;

const JID: &str = "5215512345678@s.whatsapp.net";
const GROUP: &str = "120363555777999@g.us";
const KEY: [u8; 32] = [7; 32];

struct DenyBot;
impl CommandAuthorizer for DenyBot {
    fn authorize(&self, _: &CommandInvocation) -> Result<()> {
        bail!("Bot-Matter remains blocked")
    }
}

fn signed(id: &str, body: &str, chat: &str) -> Vec<u8> {
    let timestamp = OffsetDateTime::now_utc().unix_timestamp();
    let mut mac = Hmac::<Sha256>::new_from_slice(&KEY).unwrap();
    mac.update(b"Base-Matter:WhatsApp-Control:v1\0");
    for part in ["1", "socket-test", "owner", JID, chat, id, body] {
        mac.update(&(part.len() as u32).to_be_bytes());
        mac.update(part.as_bytes());
    }
    mac.update(&[1]);
    mac.update(&timestamp.to_be_bytes());
    serde_json::to_vec(&serde_json::json!({
        "version": 1,
        "socket_id": "socket-test",
        "actor_id": "owner",
        "owner_jid": JID,
        "chat_jid": chat,
        "message_id": id,
        "text": body,
        "from_me": true,
        "timestamp": timestamp,
        "signature": hex::encode(mac.finalize().into_bytes()),
    }))
    .unwrap()
}

fn send(addr: std::net::SocketAddr, bytes: &[u8]) -> serde_json::Value {
    let mut client = TcpStream::connect(addr).unwrap();
    client.write_all(bytes).unwrap();
    client.shutdown(Shutdown::Write).unwrap();
    let mut result = Vec::new();
    client.read_to_end(&mut result).unwrap();
    serde_json::from_slice(&result).unwrap()
}

#[test]
fn customized_group_template_formats_status_bot_on_and_off_without_copying_roles() {
    let activation = Arc::new(SocketActivationStore::open(":memory:").unwrap());
    activation.provision("socket-test", "owner").unwrap();
    let router = Arc::new(CommandRouter::new(vec!["#".to_owned()], Arc::new(DenyBot)).unwrap());
    let themes = Arc::new(ControlThemes::new(
        ControlTheme::new("Base-Matter", "base-matter", "✦").unwrap(),
    ));
    let group_theme = ControlTheme::new("Grupo Pruebas", "base-matter", "◇")
        .unwrap()
        .with_template(concat!(
            "╭─☆ {{TITLE}} {{ACCENT}}\n",
            "│ {{BODY}}\n",
            "╰─☆\n",
        ))
        .unwrap();
    themes.set_community(GROUP, group_theme).unwrap();
    let control = Arc::new(
        ControlDispatcher::new(activation, Arc::new(BotRegistry::in_memory()), router, None)
            .with_public_menu(Arc::clone(&themes))
            .with_unified_live_controls()
            .with_silent_bot_commands(),
    );
    let bridge = Arc::new(
        WhatsAppControlBridge::new(
            WhatsAppBridgeConfig {
                socket_id: "socket-test".into(),
                owner_actor_id: "owner".into(),
                owner_jid: JID.into(),
            },
            KEY,
            Arc::new(InboundReplayStore::in_memory([8; 32]).unwrap()),
            control,
        )
        .unwrap(),
    );
    let ipc = WhatsAppControlIpc::new(bridge).with_themes(themes);
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let worker = std::thread::spawn(move || {
        for _ in 0..4 {
            let (stream, _) = listener.accept().unwrap();
            ipc.accept(stream).unwrap();
        }
    });

    let replies = [
        send(addr, &signed("s1", "=/>status", GROUP)),
        send(addr, &signed("s2", "=/>bot on", GROUP)),
        send(addr, &signed("s3", "=/>status", GROUP)),
        send(addr, &signed("s4", "=/>bot off", GROUP)),
    ];
    worker.join().unwrap();

    for reply in replies {
        assert_eq!(reply["outcome"], "applied");
        let text = reply["message"].as_str().unwrap();
        assert!(text.starts_with("╭─☆ Grupo Pruebas ◇"));
        assert!(!text.contains("Créditos:"));
        assert!(!text.contains("Lournyx Matterssmith"));
        assert_eq!(text.matches("╭─☆ Grupo Pruebas ◇").count(), 1);
        assert!(!text.contains("admin"));
        assert!(!text.contains("\\n"));
    }
}
