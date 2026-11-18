/*
 * Base-Matter — authenticated public chat reads, no role or power escalation.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Result};
use hmac::{Hmac, Mac};
use matter_base::{
    activation::{ActivationSource, SocketActivationStore},
    control::{ControlDispatcher, DispatchResult},
    control_style::{ControlTheme, ControlThemes},
    BotRegistry, CommandAuthorizer, CommandInvocation, CommandRouter,
    InboundReplayStore, WhatsAppBridgeConfig, WhatsAppBridgeOutcome,
    WhatsAppControlBridge, WhatsAppControlFrame,
};
use sha2::Sha256;
use std::sync::Arc;
use time::OffsetDateTime;

const KEY: [u8; 32] = [7u8; 32];
const OWNER_JID: &str = "5215512345678@s.whatsapp.net";
const READER: &str = "987654321012345@lid";
const GROUP: &str = "120363555666777@g.us";

struct DenyBot;
impl CommandAuthorizer for DenyBot {
    fn authorize(&self, _: &CommandInvocation) -> Result<()> {
        bail!("Normal bot commands unavailable")
    }
}

fn fixture() -> WhatsAppControlBridge {
    let activation = Arc::new(SocketActivationStore::open(":memory:").unwrap());
    activation.provision("socket-1", "owner").unwrap();
    let router = Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(DenyBot)).unwrap());
    let themes = Arc::new(ControlThemes::new(
        ControlTheme::new("Matterssmith", "base-matter", "✦").unwrap(),
    ));
    let controller = Arc::new(
        ControlDispatcher::new(
            activation, Arc::new(BotRegistry::in_memory()), router, None,
        )
        .with_public_menu(themes)
        .with_silent_bot_commands(),
    );
    WhatsAppControlBridge::new(
        WhatsAppBridgeConfig {
            socket_id: "socket-1".into(),
            owner_actor_id: "owner".into(),
            owner_jid: OWNER_JID.into(),
        },
        KEY,
        Arc::new(InboundReplayStore::in_memory([9u8; 32]).unwrap()),
        controller,
    ).unwrap()
}

fn sign(frame: &mut WhatsAppControlFrame) {
    let mut mac = Hmac::<Sha256>::new_from_slice(&KEY).unwrap();
    mac.update(b"Base-Matter:WhatsApp-Control:v1\0");
    for field in [
        frame.version.to_string(),
        frame.socket_id.clone(),
        frame.actor_id.clone(),
        frame.owner_jid.clone(),
        frame.chat_jid.clone(),
        frame.message_id.clone(),
        frame.text.clone(),
    ] {
        mac.update(&(field.len() as u32).to_be_bytes());
        mac.update(field.as_bytes());
    }
    mac.update(&[u8::from(frame.from_me)]);
    mac.update(&frame.timestamp.to_be_bytes());
    frame.signature = hex::encode(mac.finalize().into_bytes());
}

fn frame(id: &str, text: &str, actor: &str, chat: &str, from_me: bool)
    -> WhatsAppControlFrame
{
    let mut value = WhatsAppControlFrame {
        version: 1,
        socket_id: "socket-1".into(),
        actor_id: actor.into(),
        owner_jid: OWNER_JID.into(),
        chat_jid: chat.into(),
        message_id: id.into(),
        text: text.into(),
        from_me,
        timestamp: OffsetDateTime::now_utc().unix_timestamp(),
        signature: String::new(),
    };
    sign(&mut value);
    value
}

#[test]
fn other_group_members_read_status_and_menu_only_when_enabled() {
    let bridge = fixture();
    let pending = frame("pub-1", "=/>status", READER, GROUP, false);
    assert_eq!(
        bridge.ingest(&pending).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::GroupStatus {
            group_id: GROUP.into(),
            enabled: false,
            source: ActivationSource::PoweredOff,
        }),
    );
    assert_eq!(bridge.ingest(&pending).unwrap(), WhatsAppBridgeOutcome::Duplicate);
    assert_eq!(
        bridge.ingest(&frame("pub-2", "=/>menu", READER, GROUP, false)).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Inactive),
    );
    bridge.ingest(&frame("own-1", "=/>on", "owner", OWNER_JID, true)).unwrap();
    bridge.ingest(&frame("own-2", "=/>bot --global on", "owner", OWNER_JID, true)).unwrap();
    assert!(matches!(
        bridge.ingest(&frame("pub-3", "=/>menu", READER, GROUP, false)).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Extension { command, output })
            if command == "menu" && output.contains("Matterssmith")
    ));
    assert!(matches!(
        bridge.ingest(&frame("pub-4", "=/>menu group", READER, GROUP, false)).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Extension { command, output })
            if command == "menu" && output.contains("=/>group")
    ));
    bridge.ingest(&frame("own-3", "=/>bot group off", "owner", OWNER_JID, true)).unwrap();
    assert_eq!(
        bridge.ingest(&frame("pub-5", "=/>menu", READER, GROUP, false)).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Inactive),
    );
    assert_eq!(
        bridge.ingest(&frame("pub-6", "=/>estado", READER, GROUP, false)).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::GroupStatus {
            group_id: GROUP.into(),
            enabled: false,
            source: ActivationSource::Group,
        }),
    );
}

#[test]
fn incoming_private_chat_can_read_but_cannot_change_any_policy() {
    let bridge = fixture();
    bridge.ingest(&frame("own-1", "=/>on", "owner", OWNER_JID, true)).unwrap();
    bridge.ingest(&frame("own-2", "=/>bot --global on", "owner", OWNER_JID, true)).unwrap();
    let incoming = frame("pub-private", "=/>status", READER, READER, false);
    assert_eq!(
        bridge.ingest(&incoming).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Status {
            socket_id: "socket-1".into(),
            enabled: true,
            source: ActivationSource::Socket,
            bot_instance_id: None,
        }),
    );
    for (index, text) in [
        "=/>on",
        "=/>off",
        "=/>group on",
        "=/>status --global",
        "=/>bot --global on",
        "=/>bot private off",
        "=/>set role 5215512345678 admin",
        "=/>roles",
        "=/>menu other extra",
    ].iter().enumerate() {
        let test = frame(&format!("denied-{index}"), text, READER, READER, false);
        assert!(bridge.ingest(&test).is_err(), "must reject {text}");
    }
    let wrong_chat = frame("wrong-1", "=/>menu", READER, OWNER_JID, false);
    assert!(bridge.ingest(&wrong_chat).is_err());
    let mut forged = frame("wrong-2", "=/>menu", READER, GROUP, false);
    forged.actor_id = "5215519999999@lid".into();
    assert!(bridge.ingest(&forged).is_err());
    let impersonation = frame("wrong-3", "=/>on", "owner", GROUP, false);
    assert!(bridge.ingest(&impersonation).is_err());
}
