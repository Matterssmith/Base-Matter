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
        let mut handlers = self.handlers.write().expect("EventBus lock poisoned");

        handlers.entry(event).or_default().push(Arc::new(handler));
    }

    pub fn publish(&self, event: &str, payload: &str) {
        let handlers = {
            let handlers = self.handlers.read().expect("EventBus lock poisoned");

            handlers.get(event).cloned().unwrap_or_default()
        };

        for handler in handlers {
            handler(payload);
        }
    }

    pub fn listener_count(&self, event: &str) -> usize {
        let handlers = self.handlers.read().expect("EventBus lock poisoned");

        handlers.get(event).map_or(0, Vec::len)
    }

    pub fn clear(&self, event: &str) {
        let mut handlers = self.handlers.write().expect("EventBus lock poisoned");

        handlers.remove(event);
    }

    pub fn clear_all(&self) {
        let mut handlers = self.handlers.write().expect("EventBus lock poisoned");

        handlers.clear();
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
