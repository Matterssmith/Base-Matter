/*
 * Matter-Base — Bot Registry Tests
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use matter_base::{BotRegistry, SqliteBotIdentityStore};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "matter-base-bot-registry-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn in_memory_registry_keeps_identity_independent_and_names_unique() {
    let registry = BotRegistry::in_memory();
    let first = registry
        .register("family-a", "release-1", "matter")
        .unwrap();
    let second = registry
        .register("family-a", "release-2", "matter")
        .unwrap();

    assert_eq!(first.family_id(), second.family_id());
    assert_ne!(first.release_key(), second.release_key());
    assert_ne!(first.instance_id(), second.instance_id());
    assert_eq!(first.display_name(), "matter");
    assert_eq!(second.display_name(), "matter (2)");
    assert_eq!(registry.get(first.instance_id()).unwrap(), Some(first));
    assert_eq!(registry.list().unwrap().len(), 2);
}

#[test]
fn sqlite_registry_recovers_identity_after_reopening() {
    let directory = TemporaryDirectory::new();
    let database = directory.0.join("bots.db");
    let identity = {
        let registry = BotRegistry::open_sqlite(&database).unwrap();
        registry
            .register("family-a", "release-1", "matter")
            .unwrap()
    };

    let recovered = BotRegistry::open_sqlite(&database).unwrap();
    assert_eq!(
        recovered.get(identity.instance_id()).unwrap(),
        Some(identity.clone())
    );
    assert_eq!(recovered.list().unwrap(), vec![identity]);
}

#[test]
fn sqlite_name_allocation_is_atomic_across_connections() {
    let directory = TemporaryDirectory::new();
    let database = directory.0.join("concurrent-bots.db");
    let first = BotRegistry::new(Arc::new(SqliteBotIdentityStore::open(&database).unwrap()));
    let second = BotRegistry::new(Arc::new(SqliteBotIdentityStore::open(&database).unwrap()));
    let barrier = Arc::new(Barrier::new(3));

    let first_barrier = Arc::clone(&barrier);
    let first_thread = thread::spawn(move || {
        first_barrier.wait();
        first.register("family-a", "release-1", "matter")
    });
    let second_barrier = Arc::clone(&barrier);
    let second_thread = thread::spawn(move || {
        second_barrier.wait();
        second.register("family-a", "release-1", "matter")
    });

    barrier.wait();
    let first = first_thread.join().unwrap().unwrap();
    let second = second_thread.join().unwrap().unwrap();
    let names = [first.display_name(), second.display_name()];
    assert_ne!(names[0], names[1]);
    assert_eq!(names.iter().filter(|name| **name == "matter").count(), 1);
}

#[test]
fn registry_rejects_invalid_identity_fields() {
    let registry = BotRegistry::in_memory();
    assert!(registry.register("\n", "release-1", "matter").is_err());
    assert!(registry.register("family-a", "", "matter").is_err());
    assert!(registry
        .register("family-a", "release-1", "\u{7f}")
        .is_err());
    assert!(registry.get(" ").is_err());
}
