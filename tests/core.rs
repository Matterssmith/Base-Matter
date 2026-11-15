use matter_base::{
    ConsentDecision,
    CoreConfig,
    EventBus,
    LegalDocument,
    LegalDocumentType,
    LegalStore,
    MatterContext,
    MatterRuntime,
    ServiceRegistry,
    Translator,
};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn create_legal_store() -> LegalStore {
    LegalStore::open_in_memory().unwrap()
}

fn create_terms() -> LegalDocument {
    LegalDocument {
        document_id: "app-terms-v1".to_string(),
        scope: "test-app".to_string(),
        document_type: LegalDocumentType::Terms,
        version: "1.0.0".to_string(),
        locale: "es".to_string(),
        content: "Terminos de prueba".to_string(),
        mandatory: true,
        required_decision: ConsentDecision::Accepted,
        active: true,
    }
}

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
    let runtime = MatterRuntime::with_legal_store(
        CoreConfig::default(),
        create_legal_store(),
    );

    assert_eq!(runtime.config().name, "Matter Application");
}

#[test]
fn context_creates_event_bus() {
    let context = MatterContext::new(create_legal_store());

    assert_eq!(context.events.listener_count("test"), 0);
}

#[test]
fn context_exposes_legal_store() {
    let context = MatterContext::new(create_legal_store());

    context.legal.register_user("user-1").unwrap();

    assert!(context.legal.decision_history("user-1").unwrap().is_empty());
}

#[test]
fn runtime_blocks_access_without_legal_acceptance() {
    let legal = create_legal_store();

    legal.register_user("user-1").unwrap();
    legal.register_document(&create_terms()).unwrap();

    let runtime =
        MatterRuntime::with_legal_store(CoreConfig::default(), legal);

    assert!(!runtime
        .is_access_allowed("user-1", "test-app", "es")
        .unwrap());

    assert!(runtime
        .ensure_access("user-1", "test-app", "es")
        .is_err());
}

#[test]
fn runtime_allows_access_after_acceptance() {
    let legal = create_legal_store();

    legal.register_user("user-1").unwrap();
    legal.register_document(&create_terms()).unwrap();

    legal
        .record_decision(
            "user-1",
            "app-terms-v1",
            ConsentDecision::Accepted,
            Some("test"),
        )
        .unwrap();

    let runtime =
        MatterRuntime::with_legal_store(CoreConfig::default(), legal);

    assert!(runtime
        .is_access_allowed("user-1", "test-app", "es")
        .unwrap());

    assert!(runtime
        .ensure_access("user-1", "test-app", "es")
        .is_ok());
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

    translations.insert("hello".to_string(), "Hola".to_string());

    translator.register("es", translations);

    assert_eq!(translator.translate("hello"), "Hola");
}

#[test]
fn translator_returns_key_when_missing() {
    let translator = Translator::new("es");

    assert_eq!(translator.translate("missing.key"), "missing.key");
}