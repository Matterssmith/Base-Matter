/*
 * Base-Matter — native whatsapp-rust owner control regression tests.
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::Result;
use matter_base::{
    activation::{ActivationSource, SocketActivationStore},
    control::{ControlDispatcher, DispatchResult},
    BotRegistry, CommandAuthorizer, CommandInvocation, CommandRouter, InboundReplayStore,
    WhatsAppNativeConfig, WhatsAppNativeIngress, WhatsAppNativeMessage, WhatsAppNativeOutcome,
};
use std::sync::Arc;
use time::OffsetDateTime;
const JID: &str = "5215555550000@s.whatsapp.net";
struct DenyBot;
impl CommandAuthorizer for DenyBot {
    fn authorize(&self, _: &CommandInvocation) -> Result<()> {
        anyhow::bail!("Bot-Matter is not running")
    }
}
fn fixture() -> (WhatsAppNativeIngress, Arc<SocketActivationStore>) {
    let activation = Arc::new(SocketActivationStore::open(":memory:").unwrap());
    activation.provision("socket-a", "owner").unwrap();
    activation.provision("socket-b", "other").unwrap();
    let router = Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(DenyBot)).unwrap());
    let dispatcher = Arc::new(ControlDispatcher::new(
        activation.clone(),
        Arc::new(BotRegistry::in_memory()),
        router,
        None,
    ).with_unified_live_controls());
    let ingress = WhatsAppNativeIngress::new(
        WhatsAppNativeConfig {
            socket_id: "socket-a".into(),
            actor_id: "owner".into(),
            owner_jid: JID.into(),
        },
        Arc::new(InboundReplayStore::in_memory([13; 32]).unwrap()),
        dispatcher,
    )
    .unwrap();
    (ingress, activation)
}
fn message<'a>(id: &'a str, text: &'a str) -> WhatsAppNativeMessage<'a> {
    WhatsAppNativeMessage {
        chat_jid: JID,
        sender_jid: JID,
        message_id: id,
        text,
        from_me: true,
        is_group: false,
        timestamp: OffsetDateTime::now_utc().unix_timestamp(),
    }
}
#[test]
fn owner_enables_chat_and_socket_with_one_bot_command() {
    let (ingress, activation) = fixture();
    assert_eq!(
        ingress.ingest(&message("a", "=/>bot on")).unwrap(),
        WhatsAppNativeOutcome::Dispatched(DispatchResult::BotPolicyChanged {
            scope: "current".into(),
            policy: Some(true),
        })
    );
    assert_eq!(
        ingress.ingest(&message("b", "=/>status")).unwrap(),
        WhatsAppNativeOutcome::Dispatched(DispatchResult::Status {
            socket_id: "socket-a".into(),
            enabled: true,
            source: ActivationSource::Group,
            bot_instance_id: None,
        })
    );
    assert!(
        activation
            .inspect_bot_defaults("socket-a", "owner")
            .unwrap()
            .socket_powered
    );
    assert_eq!(
        ingress.ingest(&message("c", "=/>bot off")).unwrap(),
        WhatsAppNativeOutcome::Dispatched(DispatchResult::BotPolicyChanged {
            scope: "current".into(),
            policy: Some(false),
        })
    );
}

#[test]
fn rejects_outsider_group_different_chat_and_self_fanout() {
    let (ingress, activation) = fixture();
    let mut value = message("a", "=/>bot on");
    value.from_me = false;
    assert_eq!(
        ingress.ingest(&value).unwrap(),
        WhatsAppNativeOutcome::Ignored
    );
    value.from_me = true;
    value.is_group = true;
    assert_eq!(
        ingress.ingest(&value).unwrap(),
        WhatsAppNativeOutcome::Ignored
    );
    value.is_group = false;
    value.chat_jid = "else@s.whatsapp.net";
    assert_eq!(
        ingress.ingest(&value).unwrap(),
        WhatsAppNativeOutcome::Ignored
    );
    value.chat_jid = JID;
    value.sender_jid = "else@s.whatsapp.net";
    assert_eq!(
        ingress.ingest(&value).unwrap(),
        WhatsAppNativeOutcome::Ignored
    );
    assert!(
        !activation
            .resolve_persisted("socket-a", None)
            .unwrap()
            .enabled
    );
}
#[test]
fn ping_and_legacy_aliases_never_reach_control() {
    let (ingress, _) = fixture();
    for text in ["#ping", "=on", "/>on", "=/>on", "=/>off", "#menu", "hello"] {
        assert_eq!(
            ingress.ingest(&message("a", text)).unwrap(),
            WhatsAppNativeOutcome::Ignored
        );
    }
    assert!(ingress.ingest(&message("b", "=/>ping")).is_err());
}
#[test]
fn duplicated_expired_and_malformed_events_fail_closed() {
    let (ingress, _) = fixture();
    let valid = message("one", "=/>bot on");
    assert!(matches!(
        ingress.ingest(&valid).unwrap(),
        WhatsAppNativeOutcome::Dispatched(_)
    ));
    assert_eq!(
        ingress.ingest(&valid).unwrap(),
        WhatsAppNativeOutcome::Duplicate
    );
    let mut expired = message("two", "=/>bot off");
    expired.timestamp -= 600;
    assert!(ingress.ingest(&expired).is_err());
    assert!(ingress.ingest(&message("", "=/>bot on")).is_err());
    assert!(ingress.ingest(&message("bad\nid", "=/>bot on")).is_err());
    // Retired aliases are ignored only when well formed; malformed
    // oversized frames must fail closed before command classification.
    assert!(ingress
        .ingest(&message("legacy-large", &format!("=/>on{}", " ".repeat(4096))))
        .is_err());
    // The active command must be subject to the same hard input limit.
    assert!(ingress
        .ingest(&message(
            "active-large",
            &format!("=/>bot on{}", " ".repeat(4096)),
        ))
        .is_err());
    // A rejected oversized frame cannot poison the replay store.
    assert!(matches!(
        ingress.ingest(&message("active-large", "=/>status")).unwrap(),
        WhatsAppNativeOutcome::Dispatched(_),
    ));
}
#[test]
fn invalid_owner_identity_rejected_before_events() {
    let (_ingress, store) = fixture();
    let router = Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(DenyBot)).unwrap());
    let dispatcher = Arc::new(ControlDispatcher::new(
        store,
        Arc::new(BotRegistry::in_memory()),
        router,
        None,
    ));
    for jid in ["", "123@g.us", "user@s.whatsapp.net\n"] {
        assert!(WhatsAppNativeIngress::new(
            WhatsAppNativeConfig {
                socket_id: "socket-a".into(),
                actor_id: "owner".into(),
                owner_jid: jid.into()
            },
            Arc::new(InboundReplayStore::in_memory([14; 32]).unwrap()),
            dispatcher.clone(),
        )
        .is_err());
    }
}
