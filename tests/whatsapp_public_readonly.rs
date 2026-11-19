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
    BotRegistry, CommandAuthorizer, CommandInvocation, CommandRouter, InboundReplayStore,
    WhatsAppBridgeConfig, WhatsAppBridgeOutcome, WhatsAppControlBridge, WhatsAppControlFrame,
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
        ControlDispatcher::new(activation, Arc::new(BotRegistry::in_memory()), router, None)
            .with_public_menu(themes)
            .with_silent_bot_commands()
            .with_unified_live_controls(),
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
    )
    .unwrap()
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

fn frame(id: &str, text: &str, actor: &str, chat: &str, from_me: bool) -> WhatsAppControlFrame {
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
    assert_eq!(
        bridge.ingest(&pending).unwrap(),
        WhatsAppBridgeOutcome::Duplicate
    );
    assert!(matches!(
        bridge.ingest(&frame("pub-global", "=/>status --global", READER, GROUP, false)).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Extension { command, output })
            if command == "status" && output.contains("privada del propietario")
    ));
    assert_eq!(
        bridge
            .ingest(&frame("pub-2", "=/>menu", READER, GROUP, false))
            .unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Inactive),
    );
    bridge
        .ingest(&frame("own-1", "=/>bot on", "owner", OWNER_JID, true))
        .unwrap();
    bridge
        .ingest(&frame(
            "own-2",
            "=/>bot --global on",
            "owner",
            OWNER_JID,
            true,
        ))
        .unwrap();
    // External groups stay disabled until the owner explicitly enables
    // the group kind, regardless of the socket-global fallback.
    assert_eq!(
        bridge
            .ingest(&frame("pub-early", "=/>menu", READER, GROUP, false))
            .unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Inactive),
    );
    bridge
        .ingest(&frame(
            "own-kind",
            "=/>bot group on",
            "owner",
            OWNER_JID,
            true,
        ))
        .unwrap();
    assert!(matches!(
        bridge.ingest(&frame("pub-3", "=/>menu", READER, GROUP, false)).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Extension { command, output })
            if command == "menu" && output.contains("│·♪·╠[ ◉ =/>status")
    ));
    assert!(matches!(
        bridge.ingest(&frame("pub-4", "=/>menu", READER, GROUP, false)).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Extension { command, output })
            if command == "menu" && output.contains("=/>bot --global on/off") && output.contains("Créditos: Hecho por Lournyx Matterssmith")
    ));
    assert!(matches!(
        bridge
            .ingest(&frame("pub-credits", "=/>credits", READER, GROUP, false))
            .unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Extension { command, output })
            if command == "credits"
                && output.contains("Hecho por Lournyx Matterssmith [Lou-Nyx].")
    ));
    bridge
        .ingest(&frame(
            "own-3",
            "=/>bot group off",
            "owner",
            OWNER_JID,
            true,
        ))
        .unwrap();
    assert_eq!(
        bridge
            .ingest(&frame("pub-5", "=/>menu", READER, GROUP, false))
            .unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Inactive),
    );
    assert_eq!(
        bridge
            .ingest(&frame("pub-6", "=/>estado", READER, GROUP, false))
            .unwrap(),
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
    bridge
        .ingest(&frame("own-1", "=/>bot on", "owner", OWNER_JID, true))
        .unwrap();
    bridge
        .ingest(&frame(
            "own-2",
            "=/>bot --global on",
            "owner",
            OWNER_JID,
            true,
        ))
        .unwrap();
    let incoming = frame("pub-private", "=/>status", READER, READER, false);
    assert_eq!(
        bridge.ingest(&incoming).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Status {
            socket_id: "socket-1".into(),
            enabled: false,
            source: ActivationSource::None,
            bot_instance_id: None,
        }),
    );
    bridge
        .ingest(&frame(
            "own-priv",
            "=/>bot private on",
            "owner",
            OWNER_JID,
            true,
        ))
        .unwrap();
    assert!(matches!(
        bridge.ingest(&frame("pub-private-2", "=/>menu", READER, READER, false)).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Extension { command, .. })
            if command == "menu"
    ));
    for (index, text) in [
        "=/>on",
        "=/>off",
        "=/>group on",
        "=/>bot --global on",
        "=/>bot private off",
        "=/>set role 5215512345678 admin",
        "=/>roles",
        "=/>menu nonexistent",
        "=/>menu other extra",
    ]
    .iter()
    .enumerate()
    {
        let test = frame(&format!("denied-{index}"), text, READER, READER, false);
        assert!(bridge.ingest(&test).is_err(), "must reject {text}");
    }
    assert!(matches!(
        bridge.ingest(&frame("pub-private-global", "=/>status --global", READER, READER, false)).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Extension { command, output })
            if command == "status" && output.contains("privada del propietario")
    ));
    let wrong_chat = frame("wrong-1", "=/>menu", READER, OWNER_JID, false);
    assert!(bridge.ingest(&wrong_chat).is_err());
    let mut forged = frame("wrong-2", "=/>menu", READER, GROUP, false);
    forged.actor_id = "5215519999999@lid".into();
    assert!(bridge.ingest(&forged).is_err());
    let impersonation = frame("wrong-3", "=/>bot on", "owner", GROUP, false);
    assert!(bridge.ingest(&impersonation).is_err());
}

#[test]
fn all_21_canonical_categories_and_three_pages_have_real_dynamic_menu_routing() {
    let activation = Arc::new(SocketActivationStore::open(":memory:").unwrap());
    activation.provision("socket-1", "owner").unwrap();
    let router = Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(DenyBot)).unwrap());
    let themes = Arc::new(
        ControlThemes::new(ControlTheme::new("Matterssmith", "base-matter", "✦").unwrap())
            .with_dynamic_menu(Arc::new(
                matter_base::menu::MenuPreferences::open(":memory:").unwrap(),
            )),
    );
    let controller = Arc::new(
        ControlDispatcher::new(activation, Arc::new(BotRegistry::in_memory()), router, None)
            .with_public_menu(themes)
            .with_silent_bot_commands()
            .with_unified_live_controls(),
    );
    let bridge = WhatsAppControlBridge::new(
        WhatsAppBridgeConfig {
            socket_id: "socket-1".into(),
            owner_actor_id: "owner".into(),
            owner_jid: OWNER_JID.into(),
        },
        KEY,
        Arc::new(InboundReplayStore::in_memory([9u8; 32]).unwrap()),
        controller,
    )
    .unwrap();
    for (index, command) in ["=/>bot on", "=/>bot --global on", "=/>bot private on"]
        .iter()
        .enumerate()
    {
        bridge
            .ingest(&frame(
                &format!("owner-enable-{index}"),
                command,
                "owner",
                OWNER_JID,
                true,
            ))
            .unwrap();
    }
    for (index, command) in [
        "=/>menu ia",
        "=/>menu anti",
        "=/>menu all",
        "=/>menu config",
        "=/>menu downloads",
        "=/>menu economy",
        "=/>menu fun",
        "=/>menu gacha",
        "=/>menu games",
        "=/>menu group",
        "=/>menu info",
        "=/>menu links",
        "=/>menu main",
        "=/>menu mode",
        "=/>menu owner",
        "=/>menu profile",
        "=/>menu reactions",
        "=/>menu search",
        "=/>menu sockets",
        "=/>menu system",
        "=/>menu utils",
        "=/>menu page 1",
        "=/>menu page 2",
        "=/>menu page 3",
    ]
    .iter()
    .enumerate()
    {
        let outcome = bridge
            .ingest(&frame(
                &format!("read-menu-{index}"),
                command,
                READER,
                READER,
                false,
            ))
            .unwrap();
        assert!(
            matches!(
                outcome,
                WhatsAppBridgeOutcome::Dispatched(DispatchResult::Extension { command, .. })
                    if command == "menu"
            ),
            "menu selector rejected: {command}"
        );
    }

    let root = bridge
        .ingest(&frame("read-root-21", "=/>menu", READER, READER, false))
        .unwrap();
    assert!(matches!(
        root,
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Extension { command, output })
            if command == "menu"
                && output.contains("[menu ia]")
                && !output.contains("[menu sockets]")
                && !output.contains("[menu gacha]")
                && output.contains("[menu utils]")
    ));
}

#[test]
fn guest_can_read_registration_info_without_enrollment_or_active_bot() {
    let bridge = fixture();
    assert!(matches!(
        bridge.ingest(&frame("guest-register-1", "=/>register", READER, READER, false)).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Extension { command, output })
            if command == "register"
                && output.contains("Modo invitado")
                && output.contains("No necesitas registrarte")
    ));
    assert!(matches!(
        bridge.ingest(&frame("guest-register-2", "=/>register start", READER, READER, false)).unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Extension { command, output })
            if command == "register"
                && output.contains("No se guardó")
    ));
    assert_eq!(
        bridge
            .ingest(&frame("guest-menu-off", "=/>menu", READER, READER, false))
            .unwrap(),
        WhatsAppBridgeOutcome::Dispatched(DispatchResult::Inactive)
    );
    assert!(bridge
        .ingest(&frame(
            "guest-mutation-denied",
            "=/>set role 123 admin",
            READER,
            READER,
            false
        ))
        .is_err());
}

#[test]
fn guest_capability_contract_does_not_require_identity_enrollment() {
    use matter_base::guest::{can_use_without_registration, GuestCapability as G};
    for capability in [G::Status, G::RegistrationInfo] {
        assert!(can_use_without_registration(capability, false));
    }
    for capability in [G::Menu, G::Help, G::UiDiagnostic, G::Credits, G::BotInfo] {
        assert!(!can_use_without_registration(capability, false));
        assert!(can_use_without_registration(capability, true));
    }
    for capability in [
        G::AdministrativeWrite,
        G::VaultProfile,
        G::IdentityVerification,
        G::CodeExecution,
    ] {
        assert!(!can_use_without_registration(capability, false));
        assert!(!can_use_without_registration(capability, true));
    }
}

#[test]
fn incoming_group_anti_events_are_verified_audited_and_never_executed() {
    use matter_base::anti::AntiStore;

    let activation = Arc::new(SocketActivationStore::open(":memory:").unwrap());
    activation.provision("socket-1", "owner").unwrap();
    let store = Arc::new(AntiStore::open(":memory:").unwrap());
    let router = Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(DenyBot)).unwrap());
    let controller = Arc::new(
        ControlDispatcher::new(
            activation,
            Arc::new(BotRegistry::in_memory()),
            router,
            None,
        )
        .with_group_antis(Arc::clone(&store))
        .with_unified_live_controls(),
    );
    let bridge = WhatsAppControlBridge::new(
        WhatsAppBridgeConfig {
            socket_id: "socket-1".into(),
            owner_actor_id: "owner".into(),
            owner_jid: OWNER_JID.into(),
        },
        KEY,
        Arc::new(InboundReplayStore::in_memory([9u8; 32]).unwrap()),
        controller,
    )
    .unwrap();

    let first = frame(
        "anti-edit-1",
        "\u{001f}matter-anti:edited:＝／＞bot on https://t․me/test",
        READER,
        GROUP,
        false,
    );
    assert_eq!(bridge.ingest(&first).unwrap(), WhatsAppBridgeOutcome::AntiObserved);
    assert_eq!(bridge.ingest(&first).unwrap(), WhatsAppBridgeOutcome::Duplicate);
    let opaque = frame(
        "anti-edit-2",
        "\u{001f}matter-anti:opaque:",
        READER,
        GROUP,
        false,
    );
    assert_eq!(bridge.ingest(&opaque).unwrap(), WhatsAppBridgeOutcome::AntiObserved);
    let mut forged = frame(
        "anti-edit-3",
        "\u{001f}matter-anti:original:hello",
        READER,
        GROUP,
        false,
    );
    forged.text = "\u{001f}matter-anti:original:=/>bot on".to_owned();
    assert!(bridge.ingest(&forged).is_err());
    let private = frame(
        "anti-edit-4",
        "\u{001f}matter-anti:edited:=/>bot on",
        READER,
        READER,
        false,
    );
    assert!(bridge.ingest(&private).is_err());
    assert!(!store.get("socket-1", GROUP, "link").unwrap().enabled);
}
