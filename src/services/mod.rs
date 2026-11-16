use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

type Service = Arc<dyn Any + Send + Sync>;

#[derive(Default)]
pub struct ServiceRegistry {
    services: RwLock<HashMap<TypeId, Service>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<T>(&self, service: T)
    where
        T: Any + Send + Sync,
    {
        let mut services = self
            .services
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        services.insert(TypeId::of::<T>(), Arc::new(service));
    }

    pub fn get<T>(&self) -> Option<Arc<T>>
    where
        T: Any + Send + Sync,
    {
        let services = self
            .services
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        services
            .get(&TypeId::of::<T>())
            .cloned()
            .and_then(|service| service.downcast::<T>().ok())
    }

    pub fn contains<T>(&self) -> bool
    where
        T: Any + Send + Sync,
    {
        let services = self
            .services
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        services.contains_key(&TypeId::of::<T>())
    }

    pub fn remove<T>(&self) -> bool
    where
        T: Any + Send + Sync,
    {
        let mut services = self
            .services
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        services.remove(&TypeId::of::<T>()).is_some()
    }

    pub fn clear(&self) {
        let mut services = self
            .services
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        services.clear();
    }

    pub fn len(&self) -> usize {
        let services = self
            .services
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        services.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::ServiceRegistry;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[test]
    fn recovers_after_registry_lock_poisoning() {
        let registry = ServiceRegistry::new();
        let poisoned_registry = &registry;
        let _ = catch_unwind(AssertUnwindSafe(|| {
            let mut services = poisoned_registry.services.write().unwrap();
            services.clear();
            panic!("simulate a panic while mutating the registry");
        }));

        registry.register(42_u32);
        assert_eq!(*registry.get::<u32>().unwrap(), 42);
        assert_eq!(registry.len(), 1);
    }
}
