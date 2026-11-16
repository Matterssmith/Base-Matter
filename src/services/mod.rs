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
            .expect("ServiceRegistry lock poisoned");

        services.insert(TypeId::of::<T>(), Arc::new(service));
    }

    pub fn get<T>(&self) -> Option<Arc<T>>
    where
        T: Any + Send + Sync,
    {
        let services = self.services.read().expect("ServiceRegistry lock poisoned");

        services
            .get(&TypeId::of::<T>())
            .cloned()
            .and_then(|service| service.downcast::<T>().ok())
    }

    pub fn contains<T>(&self) -> bool
    where
        T: Any + Send + Sync,
    {
        let services = self.services.read().expect("ServiceRegistry lock poisoned");

        services.contains_key(&TypeId::of::<T>())
    }

    pub fn remove<T>(&self) -> bool
    where
        T: Any + Send + Sync,
    {
        let mut services = self
            .services
            .write()
            .expect("ServiceRegistry lock poisoned");

        services.remove(&TypeId::of::<T>()).is_some()
    }

    pub fn clear(&self) {
        let mut services = self
            .services
            .write()
            .expect("ServiceRegistry lock poisoned");

        services.clear();
    }

    pub fn len(&self) -> usize {
        let services = self.services.read().expect("ServiceRegistry lock poisoned");

        services.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
