/*
 * Base-Matter — authenticated WhatsApp control bridge regressions.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::Result;
use hmac::{Hmac, Mac};
use matter_base::{
    activation::SocketActivationStore,
    control::{ControlDispatcher, DispatchResult},
    BotRegistry, CommandAuthorizer, CommandInvocation, CommandRouter, InboundReplayStore,
    WhatsAppBridgeConfig, WhatsAppBridgeOutcome, WhatsAppControlBridge, WhatsAppControlFrame,
};
use sha2::Sha256;
use std::sync::Arc;
use time::OffsetDateTime;

const OWNER_JID: &str = "5215512345678@s.whatsapp.net";
const SECRET: [u8; 32] = [7; 32];

struct DenyBot;
impl CommandAuthorizer for DenyBot {
    fn authorize(&self, _: &CommandInvocation) -> Result<()> {
        anyhow::bail!("Bot-Matter must not execute")
    }
}

fn fixture() -> (WhatsAppControlBridge, Arc<SocketActivationStore>) {
    let activation = Arc::new(SocketActivationStore::open(":memory:").unwrap());
    activation.provision("socket-1", "owner").unwrap();
    let router = Arc::new(CommandRouter::new(vec!["#".to_owned()], Arc::new(DenyBot)).unwrap());
    let control = Arc::new(ControlDispatcher::new(
        Arc::clone(&activation),
        Arc::new(BotRegistry::in_memory()),
        router,
        None,
    ));
    let replay = Arc::new(InboundReplayStore::in_memory([9; 32]).unwrap());
    let bridge = WhatsAppControlBridge::new(
        WhatsAppBridgeConfig {
            socket_id: "socket-1".into(),
            owner_actor_id: "owner".into(),
            owner_jid: OWNER_JID.into(),
        },
        SECRET,
        replay,
        control,
    )
    .unwrap();
    (bridge, activation)
}

fn sign(frame: &mut WhatsAppControlFrame, key: &[u8; 32]) {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).unwrap();
    mac.update(b"Base-Matter:WhatsApp-Control:v1\0");
    let parts = [
        frame.version.to_string(),
        frame.socket_id.clone(),
        frame.actor_id.clone(),
        frame.owner_jid.clone(),
        frame.chat_jid.clone(),
        frame.message_id.clone(),
        frame.text.clone(),
    ];
    for part in parts {
        let data = part.as_bytes();
        mac.update(&(data.len() as u32).to_be_bytes());
        mac.update(data);
    }
    mac.update(&[u8::from(frame.from_me)]);
    mac.update(&frame.timestamp.to_be_bytes());
    frame.signature = hex::encode(mac.finalize().into_bytes());
}

fn frame(message_id: &str, text: &str) -> WhatsAppControlFrame {
    let mut value = WhatsAppControlFrame {
        version: 1,
        socket_id: "socket-1".into(),
        actor_id: "owner".into(),
        owner_jid: OWNER_JID.into(),
        chat_jid: OWNER_JID.into(),
        message_id: message_id.into(),
        text: text.into(),
        from_me: true,
        timestamp: OffsetDateTime::now_utc().unix_timestamp(),
        signature: String::new(),
    };
    sign(&mut value, &SECRET);
    value
}

#[test]
fn owner_can_control_unbound_socket_without_starting_bot() {
    let (bridge, activation) = fixture();
    assert_eq!(
        bridge.ingest(&frame("id1", "=/>on")).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Control {
            socket_id: "socket-1".into(),
            powered: true,
        })
    );
    // Socket power and command admission policy are deliberately separate.
    // Turning on an unconfigured socket must NOT enable bot command execution.
    assert_eq!(
        bridge.ingest(&frame("id-policy", "=/>status")).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Status {
            socket_id: "socket-1".into(),
            enabled: false,
            source: matter_base::activation::ActivationSource::None,
            bot_instance_id: None,
        })
    );
    activation
        .set_socket_policy("socket-1", "owner", Some(true))
        .unwrap();
    assert_eq!(
        bridge.ingest(&frame("id2", "=/>status")).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Status {
            socket_id: "socket-1".into(),
            enabled: true,
            source: matter_base::activation::ActivationSource::Socket,
            bot_instance_id: None,
        })
    );
    assert_eq!(
        bridge.ingest(&frame("id3", "=/>off")).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Control {
            socket_id: "socket-1".into(),
            powered: false,
        })
    );
    assert!(
        !activation
            .resolve_persisted("socket-1", None)
            .unwrap()
            .enabled
    );
}

#[test]
fn no_normal_bot_dispatch_ping_qr_or_legacy_global_aliases() {
    let (bridge, _activation) = fixture();
    assert_eq!(
        bridge.ingest(&frame("normal1", "#ping")).unwrap(),
        WhatsAppBridgeOutcome::Ignored
    );
    assert_eq!(
        bridge.ingest(&frame("normal2", "=on")).unwrap(),
        WhatsAppBridgeOutcome::Ignored
    );
    assert_eq!(
        bridge.ingest(&frame("normal3", "/>on")).unwrap(),
        WhatsAppBridgeOutcome::Ignored
    );
    assert!(bridge.ingest(&frame("normal4", "=/>ping")).is_err());
}

#[test]
fn signed_owner_group_controls_are_scoped_and_replayed_once() {
    let (bridge, activation) = fixture();
    let mut group = frame("group-1", "=/>group on");
    group.chat_jid = "12036312345@g.us".into();
    sign(&mut group, &SECRET);
    assert_eq!(
        bridge.ingest(&group).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::GroupPolicyChanged {
            group_id: group.chat_jid.clone(),
            policy: Some(true),
        })
    );
    assert_eq!(
        bridge.ingest(&group).unwrap(),
        WhatsAppBridgeOutcome::Duplicate
    );
    let mut status = frame("group-2", "=/>group status");
    status.chat_jid = group.chat_jid.clone();
    sign(&mut status, &SECRET);
    assert_eq!(
        bridge.ingest(&status).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::GroupStatus {
            group_id: status.chat_jid.clone(),
            enabled: false,
            source: matter_base::activation::ActivationSource::PoweredOff,
        })
    );
    assert!(
        !activation
            .resolve("socket-1", Some(&group.chat_jid), None)
            .unwrap()
            .enabled
    );
    let mut global_on = frame("group-3", "=/>on");
    global_on.chat_jid = group.chat_jid.clone();
    sign(&mut global_on, &SECRET);
    assert!(bridge.ingest(&global_on).is_err());
    assert!(bridge.ingest(&frame("private-menu", "=/>menu")).is_err());
    let mut unknown = frame("group-4", "=/>group on");
    unknown.chat_jid = "foreign@s.whatsapp.net".into();
    sign(&mut unknown, &SECRET);
    assert!(bridge.ingest(&unknown).is_err());
    let mut forged = group.clone();
    forged.message_id = "group-5".into();
    forged.signature = "00".repeat(32);
    assert!(bridge.ingest(&forged).is_err());
}

#[test]
fn signed_group_status_tracks_global_defaults_and_exact_group_override() {
    let (bridge, _) = fixture();
    let group = "120363555@g.us";
    let mut status = frame("chat-status-1", "=/>status");
    status.chat_jid = group.into();
    sign(&mut status, &SECRET);
    assert_eq!(
        bridge.ingest(&status).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::GroupStatus {
            group_id: group.into(),
            enabled: false,
            source: matter_base::activation::ActivationSource::PoweredOff,
        })
    );

    assert_eq!(
        bridge
            .ingest(&frame("chat-global-1", "=/>bot --global on"))
            .unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::BotPolicyChanged {
            scope: "socket-global".into(),
            policy: Some(true),
        })
    );
    assert_eq!(
        bridge
            .ingest(&frame("chat-global-2", "=/>status --global"))
            .unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::BotGlobalStatus {
            socket_powered: false,
            default_policy: Some(true),
            group_policy: None,
            private_policy: None,
            channel_policy: None,
        })
    );
    bridge.ingest(&frame("chat-power-1", "=/>on")).unwrap();
    status.message_id = "chat-status-2".into();
    sign(&mut status, &SECRET);
    assert_eq!(
        bridge.ingest(&status).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::GroupStatus {
            group_id: group.into(),
            enabled: false,
            source: matter_base::activation::ActivationSource::None,
        })
    );
    bridge
        .ingest(&frame("chat-kind-on", "=/>bot group on"))
        .unwrap();
    status.message_id = "chat-status-enabled".into();
    sign(&mut status, &SECRET);
    assert_eq!(
        bridge.ingest(&status).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::GroupStatus {
            group_id: group.into(),
            enabled: true,
            source: matter_base::activation::ActivationSource::Group,
        })
    );

    let mut change = frame("chat-bot-1", "=/>bot off");
    change.chat_jid = group.into();
    sign(&mut change, &SECRET);
    assert_eq!(
        bridge.ingest(&change).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::BotPolicyChanged {
            scope: "current".into(),
            policy: Some(false),
        })
    );
    status.message_id = "chat-status-3".into();
    sign(&mut status, &SECRET);
    assert_eq!(
        bridge.ingest(&status).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::GroupStatus {
            group_id: group.into(),
            enabled: false,
            source: matter_base::activation::ActivationSource::Group,
        })
    );
}

#[test]
fn rejects_forged_signatures_cross_socket_and_sender() {
    let (bridge, activation) = fixture();
    let mut value = frame("id1", "=/>on");
    let forged_prefix = if value.signature.starts_with("ab") {
        "ac"
    } else {
        "ab"
    };
    value.signature.replace_range(0..2, forged_prefix);
    assert!(bridge.ingest(&value).is_err());
    let mut value = frame("id2", "=/>on");
    value.socket_id = "other-socket".into();
    sign(&mut value, &SECRET);
    assert!(bridge.ingest(&value).is_err());
    let mut value = frame("id3", "=/>on");
    value.chat_jid = "123@g.us".into();
    sign(&mut value, &SECRET);
    assert!(bridge.ingest(&value).is_err());
    let mut value = frame("id4", "=/>on");
    value.from_me = false;
    sign(&mut value, &SECRET);
    assert!(bridge.ingest(&value).is_err());
    assert!(
        !activation
            .resolve_persisted("socket-1", None)
            .unwrap()
            .enabled
    );
}

#[test]
fn replay_and_timeout_fail_before_mutating_socket() {
    let (bridge, activation) = fixture();
    activation
        .set_socket_policy("socket-1", "owner", Some(true))
        .unwrap();
    let value = frame("id1", "=/>on");
    assert!(matches!(
        bridge.ingest(&value).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(_)
    ));
    assert_eq!(
        bridge.ingest(&value).unwrap(),
        WhatsAppBridgeOutcome::Duplicate
    );
    let mut old = frame("id2", "=/>off");
    old.timestamp -= 600;
    sign(&mut old, &SECRET);
    assert!(bridge.ingest(&old).is_err());
    assert!(
        activation
            .resolve_persisted("socket-1", None)
            .unwrap()
            .enabled
    );
}

#[test]
fn rejects_truncated_or_oversized_wire_frames_and_unknown_fields() {
    let (bridge, _) = fixture();
    assert!(bridge.ingest_json("{").is_err());
    let mut value = serde_json::to_value(frame("id1", "=/>status")).unwrap();
    value["new_field"] = serde_json::json!("unauthorized");
    assert!(bridge.ingest_json(&value.to_string()).is_err());
    assert!(bridge.ingest_json(&"x".repeat(17 * 1024)).is_err());
}

#[test]
fn signature_encoding_matches_hmac_sha256_protocol_vector() {
    let mut f = WhatsAppControlFrame {
        version: 1,
        socket_id: "socket-1".into(),
        actor_id: "owner".into(),
        owner_jid: OWNER_JID.into(),
        chat_jid: OWNER_JID.into(),
        message_id: "msg-1".into(),
        text: "=/>status".into(),
        from_me: true,
        timestamp: 1_794_873_600,
        signature: String::new(),
    };
    let mut key = [0_u8; 32];
    for (index, byte) in key.iter_mut().enumerate() {
        *byte = (index + 1) as u8;
    }
    sign(&mut f, &key);
    assert_eq!(
        f.signature,
        "747dc915a2734e90d1f2042cc42df7e360f848efcacda76baa8f27925b3124c7"
    );
}

// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
