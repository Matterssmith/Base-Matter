use matter_base::{
    ConsentDecision, CoreConfig, EventBus, LegalDocument, LegalDocumentType, LegalStore,
    MatterContext, MatterRuntime, ServiceRegistry, Translator,
};
mod common;
use common::{legal_admin, legal_user};
use std::fs;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;

struct TemporaryDirectory(PathBuf);

struct PanickingStringConversion;

impl From<PanickingStringConversion> for String {
    fn from(_: PanickingStringConversion) -> Self {
        panic!("test conversion panic");
    }
}

impl TemporaryDirectory {
    fn new() -> Self {
        let mut random = [0_u8; 16];
        getrandom::fill(&mut random).unwrap();
        let nonce = hex::encode(random);
        let path = std::env::temp_dir().join(format!(
            "matter-base-core-test-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir(&path).expect("test directory must be unique");
        Self(path)
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

use std::collections::HashMap;
use std::sync::Barrier;
use std::sync::{Arc, Mutex};
use std::thread;

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
    let runtime = MatterRuntime::with_legal_store(CoreConfig::default(), create_legal_store());

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

    context
        .legal
        .register_user("user-1", &legal_user("user-1"))
        .unwrap();

    assert!(context
        .legal
        .decision_history("user-1", &legal_user("user-1"))
        .unwrap()
        .is_empty());
}

#[test]
fn runtime_blocks_access_without_legal_acceptance() {
    let legal = create_legal_store();

    legal
        .register_user("user-1", &legal_user("user-1"))
        .unwrap();
    legal
        .register_document(&create_terms(), &legal_admin())
        .unwrap();

    let runtime = MatterRuntime::with_legal_store(CoreConfig::default(), legal);

    assert!(!runtime
        .is_access_allowed("user-1", "test-app", "es", &legal_user("user-1"))
        .unwrap());

    assert!(runtime
        .ensure_access("user-1", "test-app", "es", &legal_user("user-1"))
        .is_err());
}

#[test]
fn runtime_allows_access_after_acceptance() {
    let legal = create_legal_store();

    legal
        .register_user("user-1", &legal_user("user-1"))
        .unwrap();
    legal
        .register_document(&create_terms(), &legal_admin())
        .unwrap();

    legal
        .record_decision(
            "user-1",
            "app-terms-v1",
            ConsentDecision::Accepted,
            Some("test"),
            &legal_user("user-1"),
        )
        .unwrap();

    let runtime = MatterRuntime::with_legal_store(CoreConfig::default(), legal);

    assert!(runtime
        .is_access_allowed("user-1", "test-app", "es", &legal_user("user-1"))
        .unwrap());

    assert!(runtime
        .ensure_access("user-1", "test-app", "es", &legal_user("user-1"))
        .is_ok());
}

#[test]
fn runtime_recovers_legal_access_after_reopening_database() {
    let directory = TemporaryDirectory::new();
    let database = directory.0.join("legal.db");
    let config = CoreConfig::default();

    {
        let runtime = MatterRuntime::open(config.clone(), &database).unwrap();
        runtime
            .context
            .legal
            .register_user("persistent-user", &legal_user("persistent-user"))
            .unwrap();
        runtime
            .context
            .legal
            .register_document(&create_terms(), &legal_admin())
            .unwrap();
        runtime
            .context
            .legal
            .record_decision(
                "persistent-user",
                "app-terms-v1",
                ConsentDecision::Accepted,
                Some("integration-test"),
                &legal_user("persistent-user"),
            )
            .unwrap();
        assert!(runtime
            .is_access_allowed(
                "persistent-user",
                "test-app",
                "es",
                &legal_user("persistent-user")
            )
            .unwrap());
    }

    let recovered = MatterRuntime::open(config, &database).unwrap();
    assert!(recovered
        .context
        .legal
        .user_exists("persistent-user", &legal_user("persistent-user"))
        .unwrap());
    assert_eq!(
        recovered
            .context
            .legal
            .decision_history("persistent-user", &legal_user("persistent-user"))
            .unwrap()
            .len(),
        1
    );
    assert!(recovered
        .is_access_allowed(
            "persistent-user",
            "test-app",
            "es",
            &legal_user("persistent-user")
        )
        .unwrap());
}

#[test]
fn concurrent_consent_withdrawals_are_serialized_across_connections() {
    let directory = TemporaryDirectory::new();
    let database = directory.0.join("concurrent-legal.db");
    let mut consent_document = create_terms();
    consent_document.document_id = "app-consent-v1".to_owned();
    consent_document.document_type = LegalDocumentType::Consent;
    consent_document.mandatory = false;
    consent_document.required_decision = ConsentDecision::Granted;
    let initial = LegalStore::open(&database).unwrap();
    initial
        .register_user("concurrent-user", &legal_user("concurrent-user"))
        .unwrap();
    initial
        .register_document(&consent_document, &legal_admin())
        .unwrap();
    initial
        .record_decision(
            "concurrent-user",
            "app-consent-v1",
            ConsentDecision::Granted,
            None,
            &legal_user("concurrent-user"),
        )
        .unwrap();

    // Independent SQLite connections exercise database transactions instead
    // of relying on the in-process mutex shared by cloned stores.
    let first = LegalStore::open(&database).unwrap();
    let second = LegalStore::open(&database).unwrap();
    let barrier = Arc::new(Barrier::new(3));
    let first_barrier = Arc::clone(&barrier);
    let first_thread = thread::spawn(move || {
        first_barrier.wait();
        first.record_decision(
            "concurrent-user",
            "app-consent-v1",
            ConsentDecision::Withdrawn,
            None,
            &legal_user("concurrent-user"),
        )
    });
    let second_barrier = Arc::clone(&barrier);
    let second_thread = thread::spawn(move || {
        second_barrier.wait();
        second.record_decision(
            "concurrent-user",
            "app-consent-v1",
            ConsentDecision::Withdrawn,
            None,
            &legal_user("concurrent-user"),
        )
    });

    barrier.wait();
    let successes = [first_thread.join().unwrap(), second_thread.join().unwrap()]
        .into_iter()
        .filter(Result::is_ok)
        .count();

    assert_eq!(successes, 1);
    let recovered = LegalStore::open(&database).unwrap();
    let history = recovered
        .decision_history("concurrent-user", &legal_user("concurrent-user"))
        .unwrap();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0].decision, "granted");
    assert_eq!(history[1].decision, "withdrawn");
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
fn event_bus_invokes_callbacks_without_holding_registry_lock() {
    let bus = EventBus::new();
    let callback_bus = bus.clone();
    bus.subscribe("reentrant", move |_| {
        callback_bus.subscribe("reentrant", |_| {});
    });

    bus.publish("reentrant", "payload");

    assert_eq!(bus.listener_count("reentrant"), 2);
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
fn event_bus_and_translator_convert_inputs_before_locking() {
    let bus = EventBus::new();
    let bus_result = catch_unwind(AssertUnwindSafe(|| {
        bus.subscribe(PanickingStringConversion, |_| {});
    }));
    assert!(bus_result.is_err());
    bus.subscribe("healthy-event", |_| {});
    assert_eq!(bus.listener_count("healthy-event"), 1);

    let translator = Translator::default();
    let locale_result = catch_unwind(AssertUnwindSafe(|| {
        translator.set_locale(PanickingStringConversion);
    }));
    assert!(locale_result.is_err());
    translator.set_locale("en");

    let register_result = catch_unwind(AssertUnwindSafe(|| {
        translator.register(PanickingStringConversion, HashMap::new());
    }));
    assert!(register_result.is_err());
    translator.register(
        "en",
        HashMap::from([("hello".to_owned(), "world".to_owned())]),
    );
    assert_eq!(translator.translate("hello"), "world");
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
