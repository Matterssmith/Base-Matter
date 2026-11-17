/*
 * Base-Matter — platform-independent transport isolation regressions.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::Result;
use matter_base::connections::ConnectionDriver;
use matter_base::platforms::{MatterPlatform, PlatformConnectionDrivers, PlatformTransports};
use matter_base::sessions::{
    ConnectionState, OutboundMessage, SessionRecord, TransportDriver,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

struct EchoTransport {
    name: &'static str,
    calls: Arc<AtomicUsize>,
}
impl TransportDriver for EchoTransport {
    fn transmit(&self, _session: &SessionRecord, _message: &OutboundMessage) -> Result<String> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Ok(self.name.to_owned())
    }
}

struct ProbeConnection(Arc<AtomicUsize>);
impl ConnectionDriver for ProbeConnection {
    fn connect(&self, _session: &SessionRecord, _credential: &[u8]) -> Result<()> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    fn disconnect(&self, _session: &SessionRecord) -> Result<()> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    fn is_connected(&self, _session: &SessionRecord) -> Result<bool> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(true)
    }
}

fn session(platform: &str, socket_id: &str) -> SessionRecord {
    SessionRecord {
        socket_id: socket_id.to_owned(),
        owner_id: "owner".to_owned(),
        platform: platform.to_owned(),
        state: ConnectionState::Connected,
        revision: 1,
    }
}

fn message(socket_id: &str) -> OutboundMessage {
    OutboundMessage {
        socket_id: socket_id.to_owned(),
        actor_id: "owner".into(),
        destination_id: "peer".into(),
        group_id: None,
        text: "hello".into(),
    }
}

#[test]
fn known_platform_ids_are_stable_and_unknown_values_fail_closed() {
    for (kind, id) in [
        (MatterPlatform::WhatsApp, "whatsapp"),
        (MatterPlatform::Telegram, "telegram"),
        (MatterPlatform::Discord, "discord"),
    ] {
        assert_eq!(kind.as_str(), id);
        assert_eq!(MatterPlatform::parse(id).unwrap(), kind);
    }
    for invalid in ["", "WhatsApp", "matrix", "whatsapp/other", " telegram"] {
        assert!(MatterPlatform::parse(invalid).is_err());
    }
}

#[test]
fn transports_route_to_exact_platform_without_cross_socket_fallback() {
    let registry = PlatformTransports::new();
    let wa = Arc::new(AtomicUsize::new(0));
    let tg = Arc::new(AtomicUsize::new(0));
    let dc = Arc::new(AtomicUsize::new(0));
    for (kind, name, calls) in [
        (MatterPlatform::WhatsApp, "wa", Arc::clone(&wa)),
        (MatterPlatform::Telegram, "tg", Arc::clone(&tg)),
        (MatterPlatform::Discord, "dc", Arc::clone(&dc)),
    ] {
        registry
            .register(kind, Arc::new(EchoTransport { name, calls }))
            .unwrap();
    }

    for (platform, expected) in [
        ("whatsapp", "wa"),
        ("telegram", "tg"),
        ("discord", "dc"),
    ] {
        assert_eq!(
            registry.transmit(&session(platform, "socket-1"), &message("socket-1")).unwrap(),
            expected
        );
    }
    assert_eq!(wa.load(Ordering::SeqCst), 1);
    assert_eq!(tg.load(Ordering::SeqCst), 1);
    assert_eq!(dc.load(Ordering::SeqCst), 1);
    assert!(registry
        .transmit(&session("whatsapp", "other-socket"), &message("socket-1"))
        .is_err());
    assert!(registry
        .transmit(&session("unknown", "socket-1"), &message("socket-1"))
        .is_err());
    assert_eq!(wa.load(Ordering::SeqCst), 1);
}

#[test]
fn absent_and_duplicate_registrations_are_denied_without_replacement() {
    let registry = PlatformTransports::new();
    let count = Arc::new(AtomicUsize::new(0));
    assert!(registry
        .transmit(&session("discord", "socket-1"), &message("socket-1"))
        .is_err());
    registry
        .register(
            MatterPlatform::Discord,
            Arc::new(EchoTransport {
                name: "original",
                calls: Arc::clone(&count),
            }),
        )
        .unwrap();
    assert!(registry
        .register(
            MatterPlatform::Discord,
            Arc::new(EchoTransport {
                name: "replacement",
                calls: Arc::new(AtomicUsize::new(0)),
            }),
        )
        .is_err());
    assert_eq!(
        registry
            .transmit(&session("discord", "socket-1"), &message("socket-1"))
            .unwrap(),
        "original"
    );
    assert_eq!(count.load(Ordering::SeqCst), 1);
}

#[test]
fn connection_lifecycle_dispatches_without_cross_platform_fallback() {
    let registry = PlatformConnectionDrivers::new();
    let wa = Arc::new(AtomicUsize::new(0));
    registry
        .register(
            MatterPlatform::WhatsApp,
            Arc::new(ProbeConnection(Arc::clone(&wa))),
        )
        .unwrap();
    let target = session("whatsapp", "socket-1");
    registry.connect(&target, b"credential").unwrap();
    assert!(registry.is_connected(&target).unwrap());
    registry.disconnect(&target).unwrap();
    assert_eq!(wa.load(Ordering::SeqCst), 3);

    for id in ["telegram", "discord", "unknown"] {
        let other = session(id, "socket-2");
        assert!(registry.connect(&other, b"secret").is_err());
        assert!(registry.disconnect(&other).is_err());
        assert!(registry.is_connected(&other).is_err());
    }
    assert_eq!(wa.load(Ordering::SeqCst), 3);
    assert!(registry
        .register(
            MatterPlatform::WhatsApp,
            Arc::new(ProbeConnection(Arc::new(AtomicUsize::new(0))))
        )
        .is_err());
}
