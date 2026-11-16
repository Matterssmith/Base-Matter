/*
 * Matter-Base — Provider Replay Store Tests
 *
 * Verifica unicidad, límites y recuperación persistente de IDs de afirmación.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use matter_base::security::{
    InMemoryProviderAssertionReplayStore, ProviderAssertion, ProviderAssertionReplayStore,
    ProviderAuthenticator, SqliteProviderAssertionReplayStore,
};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock must be after Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "matter-base-replay-test-{}-{nonce}",
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

#[test]
fn sqlite_store_rejects_replays_and_expires_old_ids() {
    let directory = TemporaryDirectory::new();
    let database = directory.0.join("replay.db");
    let store = SqliteProviderAssertionReplayStore::open(&database).unwrap();

    store
        .record_once("vault", "audience", "assertion-1", 200, 100, 1)
        .unwrap();
    assert!(store
        .record_once("vault", "audience", "assertion-1", 200, 100, 1)
        .is_err());
    assert!(store
        .record_once("vault", "audience", "assertion-2", 200, 100, 1)
        .is_err());

    store
        .record_once("vault", "audience", "assertion-2", 300, 201, 1)
        .unwrap();
}

#[test]
fn sqlite_store_rejects_duplicate_assertions_across_connections() {
    let directory = TemporaryDirectory::new();
    let database = directory.0.join("concurrent-replay.db");
    let first = SqliteProviderAssertionReplayStore::open(&database).unwrap();
    let second = SqliteProviderAssertionReplayStore::open(&database).unwrap();
    let barrier = Arc::new(std::sync::Barrier::new(3));

    let first_barrier = Arc::clone(&barrier);
    let first_thread = thread::spawn(move || {
        first_barrier.wait();
        first.record_once("vault", "audience", "same-id", 200, 100, 10)
    });
    let second_barrier = Arc::clone(&barrier);
    let second_thread = thread::spawn(move || {
        second_barrier.wait();
        second.record_once("vault", "audience", "same-id", 200, 100, 10)
    });

    barrier.wait();
    let results = [first_thread.join().unwrap(), second_thread.join().unwrap()];
    let successes = results.iter().filter(|result| result.is_ok()).count();
    assert_eq!(successes, 1);
    let rejection = results.into_iter().find_map(Result::err).unwrap();
    assert!(rejection.to_string().contains("replay detected"));
}

#[test]
fn shared_memory_store_rejects_replays_across_verifiers() {
    let key = [13u8; 32];
    let nonce = [18u8; 24];
    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let assertion = ProviderAssertion {
        provider_id: "test-provider".to_owned(),
        vault_id: "test-vault".to_owned(),
        audience: "matter-base-test".to_owned(),
        assertion_id: "shared-memory-assertion".to_owned(),
        issued_at: now,
        expires_at: now + 120,
        permissions: vec!["read_key_ids".to_owned()],
    };
    let ciphertext = ProviderAuthenticator::seal_assertion(&assertion, &nonce, &key).unwrap();
    let replay_store: Arc<dyn ProviderAssertionReplayStore> =
        Arc::new(InMemoryProviderAssertionReplayStore::default());
    let first = ProviderAuthenticator::new_with_replay_store(
        "test-vault".to_owned(),
        "matter-base-test".to_owned(),
        Arc::clone(&replay_store),
    )
    .unwrap();
    let second = ProviderAuthenticator::new_with_replay_store(
        "test-vault".to_owned(),
        "matter-base-test".to_owned(),
        replay_store,
    )
    .unwrap();

    assert!(first.authenticate(&nonce, &ciphertext, &key).is_ok());
    assert!(second.authenticate(&nonce, &ciphertext, &key).is_err());
}

#[test]
fn authenticator_rejects_replay_after_restart_with_sqlite_store() {
    let directory = TemporaryDirectory::new();
    let database = directory.0.join("restart-replay.db");
    let key = [13u8; 32];
    let nonce = [17u8; 24];
    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let assertion = ProviderAssertion {
        provider_id: "test-provider".to_owned(),
        vault_id: "test-vault".to_owned(),
        audience: "matter-base-test".to_owned(),
        assertion_id: "persistent-assertion".to_owned(),
        issued_at: now,
        expires_at: now + 120,
        permissions: vec!["read_key_ids".to_owned()],
    };
    let ciphertext = ProviderAuthenticator::seal_assertion(&assertion, &nonce, &key).unwrap();

    {
        let replay_store = Arc::new(SqliteProviderAssertionReplayStore::open(&database).unwrap());
        let verifier = ProviderAuthenticator::new_with_replay_store(
            "test-vault".to_owned(),
            "matter-base-test".to_owned(),
            replay_store,
        )
        .unwrap();
        assert!(verifier.authenticate(&nonce, &ciphertext, &key).is_ok());
    }

    let replay_store = Arc::new(SqliteProviderAssertionReplayStore::open(&database).unwrap());
    let recovered_verifier = ProviderAuthenticator::new_with_replay_store(
        "test-vault".to_owned(),
        "matter-base-test".to_owned(),
        replay_store,
    )
    .unwrap();
    assert!(recovered_verifier
        .authenticate(&nonce, &ciphertext, &key)
        .is_err());
}

#[test]
fn sqlite_store_removes_expired_entries_from_other_scopes() {
    let directory = TemporaryDirectory::new();
    let database = directory.0.join("global-expiry.db");
    let store = SqliteProviderAssertionReplayStore::open(&database).unwrap();

    store
        .record_once("vault-a", "audience-a", "old-id", 150, 100, 10)
        .unwrap();
    store
        .record_once("vault-b", "audience-b", "new-id", 300, 200, 10)
        .unwrap();

    let connection = rusqlite::Connection::open(database).unwrap();
    let remaining: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM provider_assertion_replays WHERE expires_at <= 200",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(remaining, 0);
}
