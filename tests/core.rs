use matter_base::{
    CoreConfig,
    EventBus,
    MatterContext,
    MatterRuntime,
    ServiceRegistry,
    Translator,
};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[test]
fn default_config_is_valid() {
    let config = CoreConfig::default();

    assert_eq!(config.name, "Matter Application");
    assert_eq!(config.environment, "production");
    assert_eq!(config.locale, "es");
    assert!(!config.debug);
}

#[test]
fn runtime_creates_context() {
    let runtime = MatterRuntime::new(CoreConfig::default());

    assert_eq!(runtime.config().name, "Matter Application");
}

#[test]
fn context_creates_event_bus() {
    let context = MatterContext::new();

    assert_eq!(context.events.listener_count("test"), 0);
}

#[test]
fn event_bus_publishes_events() {
    let bus = EventBus::new();
    let received = Arc::new(Mutex::new(String::new()));

    let target = Arc::clone(&received);

    bus.subscribe("test", move |payload| {
        *target.lock().unwrap() = payload.to_string();
    });

    bus.publish("test", "hello");

    assert_eq!(*received.lock().unwrap(), "hello");
}

#[test]
fn event_bus_clears_events() {
    let bus = EventBus::new();

    bus.subscribe("test", |_| {});

    assert_eq!(bus.listener_count("test"), 1);

    bus.clear("test");

    assert_eq!(bus.listener_count("test"), 0);
}

#[test]
fn service_registry_stores_and_retrieves_services() {
    let registry = ServiceRegistry::new();

    registry.register::<String>("Matter".to_string());

    let service = registry.get::<String>().unwrap();

    assert_eq!(&*service, "Matter");
}

#[test]
fn service_registry_removes_services() {
    let registry = ServiceRegistry::new();

    registry.register::<String>("Matter".to_string());

    assert!(registry.remove::<String>());
    assert!(!registry.contains::<String>());
}

#[test]
fn translator_registers_and_translates() {
    let translator = Translator::new("es");

    let mut translations = HashMap::new();

    translations.insert(
        "hello".to_string(),
        "Hola".to_string(),
    );

    translator.register("es", translations);

    assert_eq!(translator.translate("hello"), "Hola");
}

#[test]
fn translator_returns_key_when_missing() {
    let translator = Translator::new("es");

    assert_eq!(translator.translate("missing.key"), "missing.key");
}