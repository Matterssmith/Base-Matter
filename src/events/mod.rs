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
