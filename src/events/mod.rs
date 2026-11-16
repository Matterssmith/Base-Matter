use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

type EventHandler = Arc<dyn Fn(&str) + Send + Sync>;

#[derive(Clone)]
pub struct EventBus {
    handlers: Arc<RwLock<HashMap<String, Vec<EventHandler>>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn subscribe<F>(&self, event: impl Into<String>, handler: F)
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        let event = event.into();
        let mut handlers = self
            .handlers
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        handlers.entry(event).or_default().push(Arc::new(handler));
    }

    pub fn publish(&self, event: &str, payload: &str) {
        let handlers = {
            let handlers = self
                .handlers
                .read()
                .unwrap_or_else(std::sync::PoisonError::into_inner);

            handlers.get(event).cloned().unwrap_or_default()
        };

        for handler in handlers {
            handler(payload);
        }
    }

    /// Dispatches to every listener and reports whether any handler panicked.
    ///
    /// Unlike `publish`, this method isolates unwinding panics per handler so
    /// later listeners still run. Panic-abort builds cannot recover panics.
    pub fn publish_checked(&self, event: &str, payload: &str) -> Result<()> {
        let handlers = {
            let handlers = self
                .handlers
                .read()
                .unwrap_or_else(std::sync::PoisonError::into_inner);

            handlers.get(event).cloned().unwrap_or_default()
        };

        let mut panicked_handlers = 0;
        for handler in handlers {
            if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| handler(payload))).is_err()
            {
                panicked_handlers += 1;
            }
        }

        if panicked_handlers > 0 {
            return Err(anyhow!("{panicked_handlers} event handler(s) panicked"));
        }

        Ok(())
    }

    pub fn listener_count(&self, event: &str) -> usize {
        let handlers = self
            .handlers
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        handlers.get(event).map_or(0, Vec::len)
    }

    pub fn clear(&self, event: &str) {
        let mut handlers = self
            .handlers
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        handlers.remove(event);
    }

    pub fn clear_all(&self) {
        let mut handlers = self
            .handlers
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        handlers.clear();
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::EventBus;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[test]
    fn checked_publish_runs_remaining_handlers_and_reports_panics() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let bus = EventBus::new();
        let calls = Arc::new(AtomicUsize::new(0));
        bus.subscribe("event", |_| panic!("handler failure"));
        let called = Arc::clone(&calls);
        bus.subscribe("event", move |_| {
            called.fetch_add(1, Ordering::SeqCst);
        });

        let result = bus.publish_checked("event", "payload");
        assert!(result.unwrap_err().to_string().contains("1 event handler"));
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn recovers_after_registry_lock_poisoning() {
        let bus = EventBus::new();
        let poisoned_bus = bus.clone();
        let _ = catch_unwind(AssertUnwindSafe(|| {
            let mut handlers = poisoned_bus.handlers.write().unwrap();
            handlers.insert("partial".to_owned(), Vec::new());
            panic!("simulate a panic while mutating the registry");
        }));

        bus.subscribe("ready", |_| {});
        assert_eq!(bus.listener_count("ready"), 1);
        bus.publish("ready", "payload");
    }
}
