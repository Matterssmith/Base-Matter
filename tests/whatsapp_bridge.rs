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
    let control = Arc::new(
        ControlDispatcher::new(
            Arc::clone(&activation),
            Arc::new(BotRegistry::in_memory()),
            router,
            None,
        )
        .with_unified_live_controls(),
    );
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
fn owner_enables_current_chat_with_one_bot_command_without_binding() {
    let (bridge, activation) = fixture();
    assert_eq!(
        bridge.ingest(&frame("id1", "=/>bot on")).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::BotPolicyChanged {
            scope: "current".into(),
            policy: Some(true),
        })
    );
    assert!(
        activation
            .inspect_bot_defaults("socket-1", "owner")
            .unwrap()
            .socket_powered
    );
    assert_eq!(
        bridge.ingest(&frame("id2", "=/>status")).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Status {
            socket_id: "socket-1".into(),
            enabled: true,
            source: matter_base::activation::ActivationSource::Group,
            bot_instance_id: None,
        })
    );
    assert_eq!(
        bridge.ingest(&frame("id3", "=/>bot off")).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::BotPolicyChanged {
            scope: "current".into(),
            policy: Some(false),
        })
    );
    assert_eq!(
        bridge.ingest(&frame("id4", "=/>status")).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Status {
            socket_id: "socket-1".into(),
            enabled: false,
            source: matter_base::activation::ActivationSource::Group,
            bot_instance_id: None,
        })
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
    assert!(bridge.ingest(&frame("normal5", "=/>on")).is_err());
    assert!(bridge.ingest(&frame("normal6", "=/>off")).is_err());
    // Vault must never become a group-scoped control.
    let mut group = frame("vault-group", "=/>vault");
    group.chat_jid = "12036312345@g.us".to_owned();
    sign(&mut group, &SECRET);
    assert!(bridge.ingest(&group).is_err());
    // Even the socket owner is not automatically an ecosystem owner.
    assert!(bridge.ingest(&frame("vault-self", "=/>vault")).is_err());
    assert!(bridge
        .ingest(&frame("vault-self-test", "=/>vault test"))
        .is_err());
    assert!(bridge
        .ingest(&frame(
            "global-role",
            "=/>set role owner --global 5551234567"
        ))
        .is_err());
    let mut outsider = frame("vault-outsider", "=/>vault");
    outsider.from_me = false;
    outsider.actor_id = "987654321012346@lid".to_owned();
    outsider.chat_jid = outsider.actor_id.clone();
    sign(&mut outsider, &SECRET);
    assert!(bridge.ingest(&outsider).is_err());
}

#[test]
fn signed_owner_group_bot_controls_are_scoped_and_replayed_once() {
    let (bridge, activation) = fixture();
    let mut group = frame("group-1", "=/>bot on");
    group.chat_jid = "12036312345@g.us".into();
    sign(&mut group, &SECRET);
    assert_eq!(
        bridge.ingest(&group).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::BotPolicyChanged {
            scope: "current".into(),
            policy: Some(true),
        })
    );
    assert_eq!(
        bridge.ingest(&group).unwrap(),
        WhatsAppBridgeOutcome::Duplicate
    );
    let mut status = frame("group-2", "=/>status");
    status.chat_jid = group.chat_jid.clone();
    sign(&mut status, &SECRET);
    assert_eq!(
        bridge.ingest(&status).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::GroupStatus {
            group_id: status.chat_jid.clone(),
            enabled: true,
            source: matter_base::activation::ActivationSource::Group,
        })
    );
    assert!(
        activation
            .resolve("socket-1", Some(&group.chat_jid), None)
            .unwrap()
            .enabled
    );
    let mut socket_status = frame("group-global-status", "=/>status --mod");
    socket_status.chat_jid = group.chat_jid.clone();
    sign(&mut socket_status, &SECRET);
    assert!(matches!(
        bridge.ingest(&socket_status).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::BotGlobalStatus {
            socket_powered: true,
            ..
        })
    ));
    let mut global_on = frame("group-3", "=/>bot --mod on");
    global_on.chat_jid = group.chat_jid.clone();
    sign(&mut global_on, &SECRET);
    assert!(bridge.ingest(&global_on).is_err());
    assert!(bridge.ingest(&frame("private-menu", "=/>menu")).is_err());
    let mut unknown = frame("group-4", "=/>bot on");
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
            .ingest(&frame("chat-global-1", "=/>bot --mod on"))
            .unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::BotPolicyChanged {
            scope: "socket-global".into(),
            policy: Some(true),
        })
    );
    assert_eq!(
        bridge
            .ingest(&frame("chat-global-2", "=/>status --mod"))
            .unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::BotGlobalStatus {
            socket_powered: true,
            default_policy: Some(true),
            group_policy: None,
            private_policy: None,
            channel_policy: None,
            community_policy: None,
        })
    );

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
    let mut value = frame("id1", "=/>bot on");
    let forged_prefix = if value.signature.starts_with("ab") {
        "ac"
    } else {
        "ab"
    };
    value.signature.replace_range(0..2, forged_prefix);
    assert!(bridge.ingest(&value).is_err());
    let mut value = frame("id2", "=/>bot on");
    value.socket_id = "other-socket".into();
    sign(&mut value, &SECRET);
    assert!(bridge.ingest(&value).is_err());
    let mut value = frame("id3", "=/>bot on");
    value.chat_jid = "invalid/group@g.us".into();
    sign(&mut value, &SECRET);
    assert!(bridge.ingest(&value).is_err());
    let mut value = frame("id4", "=/>bot on");
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
    let value = frame("id1", "=/>bot on");
    assert!(matches!(
        bridge.ingest(&value).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(_)
    ));
    assert_eq!(
        bridge.ingest(&value).unwrap(),
        WhatsAppBridgeOutcome::Duplicate
    );
    let mut old = frame("id2", "=/>bot off");
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

#[test]
fn test_command_responds_in_owner_private_and_verified_group() {
    let (bridge, _) = fixture();
    for (id, cmd) in [
        ("visual-private-help", "=/>test"),
        ("visual-private-ui", "=/>test ui"),
        ("visual-private-row", "=/>test row"),
    ] {
        assert!(matches!(
            bridge.ingest(&frame(id, cmd)).unwrap(),
            WhatsAppBridgeOutcome::Dispatched(DispatchResult::Extension {
                command,
                ..
            }) if command == "test"
        ));
    }
    let mut group = frame("visual-owner-group", "=/>test");
    group.chat_jid = "12036312345@g.us".into();
    sign(&mut group, &SECRET);
    assert!(matches!(
        bridge.ingest(&group).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Extension {
            command,
            ..
        }) if command == "test"
    ));
    assert_eq!(
        bridge.ingest(&group).unwrap(),
        WhatsAppBridgeOutcome::Duplicate
    );
    group.message_id = "visual-owner-ui".into();
    group.text = "=/>test ui".into();
    sign(&mut group, &SECRET);
    assert!(matches!(
        bridge.ingest(&group).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Extension {
            command,
            ..
        }) if command == "test"
    ));
    group.message_id = "visual-owner-html".into();
    group.text = "=/>test html".into();
    sign(&mut group, &SECRET);
    assert!(bridge.ingest(&group).is_err());
}

#[test]
fn public_test_is_readonly_and_obeys_chat_activation() {
    let (bridge, _) = fixture();
    let mut public = frame("visual-public-off", "=/>test");
    public.actor_id = "987654321012346@lid".into();
    public.chat_jid = public.actor_id.clone();
    public.from_me = false;
    sign(&mut public, &SECRET);
    assert!(matches!(
        bridge.ingest(&public).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Extension {
            command,
            output
        }) if command == "test" && output.contains("BOT APAGADO")
    ));
    public.message_id = "visual-public-ui-off".into();
    public.text = "=/>test ui".into();
    sign(&mut public, &SECRET);
    assert_eq!(
        bridge.ingest(&public).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Inactive)
    );
    // Only a verified socket owner can turn on the chat. Test probes never do.
    let mut owner_group = frame("visual-enable", "=/>bot on");
    owner_group.chat_jid = "12036312345@g.us".into();
    sign(&mut owner_group, &SECRET);
    assert!(matches!(
        bridge.ingest(&owner_group).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::BotPolicyChanged { .. })
    ));
    public.message_id = "visual-public-group".into();
    public.chat_jid = owner_group.chat_jid.clone();
    public.text = "=/>test row".into();
    sign(&mut public, &SECRET);
    assert!(matches!(
        bridge.ingest(&public).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Extension {
            command,
            ..
        }) if command == "test"
    ));
    public.message_id = "visual-public-html".into();
    public.text = "=/>test html".into();
    sign(&mut public, &SECRET);
    assert!(bridge.ingest(&public).is_err());
}

// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
