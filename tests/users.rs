use anyhow::{bail, Result};
use matter_base::{
    ExternalIdentity, UserAction, UserAuthorizationRequest, UserAuthorizer, UserPrincipal,
    UserRegistry,
};
use std::sync::atomic::{AtomicBool, Ordering};

struct Policy {
    deny: AtomicBool,
}
impl UserAuthorizer for Policy {
    fn authorize(&self, request: &UserAuthorizationRequest<'_>) -> Result<()> {
        if self.deny.load(Ordering::SeqCst) {
            bail!("secret upstream policy details");
        }
        if matches!(
            request.action,
            UserAction::ReadData | UserAction::WriteData | UserAction::DeleteData
        ) && request.bot_instance_id.is_none()
        {
            bail!("bot scope required");
        }
        Ok(())
    }
}
fn principal<'a>(id: &'a str, policy: &'a Policy) -> UserPrincipal<'a> {
    UserPrincipal::new(id, policy).unwrap()
}

#[test]
fn explicit_identity_links_are_unique_scoped_and_persistent() {
    let path = std::env::temp_dir().join(format!(
        "matter-users-{}-{}.db",
        std::process::id(),
        time::OffsetDateTime::now_utc().unix_timestamp_nanos()
    ));
    let policy = Policy {
        deny: AtomicBool::new(false),
    };
    let p = principal("app-user", &policy);
    let identity = ExternalIdentity::new("whatsapp", "platform-42").unwrap();
    let user_id;
    {
        let users = UserRegistry::open_sqlite(&path).unwrap();
        user_id = users.create_user(&p).unwrap();
        assert_eq!(users.resolve_identity(&identity, &p).unwrap(), None);
        users.link_identity(&user_id, &identity, &p).unwrap();
        users.link_identity(&user_id, &identity, &p).unwrap();
        assert_eq!(
            users.resolve_identity(&identity, &p).unwrap().as_deref(),
            Some(user_id.as_str())
        );
        assert!(users.link_identity("other-user", &identity, &p).is_err());
        users
            .set_bot_data("bot-a", &user_id, "theme", "dark", &p)
            .unwrap();
        users
            .set_bot_data("bot-b", &user_id, "theme", "light", &p)
            .unwrap();
        assert_eq!(
            users
                .bot_data("bot-a", &user_id, "theme", &p)
                .unwrap()
                .as_deref(),
            Some("dark")
        );
        assert_eq!(
            users
                .bot_data("bot-b", &user_id, "theme", &p)
                .unwrap()
                .as_deref(),
            Some("light")
        );
    }
    {
        let users = UserRegistry::open_sqlite(&path).unwrap();
        assert_eq!(
            users.resolve_identity(&identity, &p).unwrap().as_deref(),
            Some(user_id.as_str())
        );
        assert_eq!(
            users
                .bot_data("bot-a", &user_id, "theme", &p)
                .unwrap()
                .as_deref(),
            Some("dark")
        );
        assert_eq!(
            users
                .bot_data("bot-b", &user_id, "theme", &p)
                .unwrap()
                .as_deref(),
            Some("light")
        );
        assert!(users
            .delete_bot_data("bot-a", &user_id, "theme", &p)
            .unwrap());
        assert_eq!(
            users.bot_data("bot-a", &user_id, "theme", &p).unwrap(),
            None
        );
        assert_eq!(
            users
                .bot_data("bot-b", &user_id, "theme", &p)
                .unwrap()
                .as_deref(),
            Some("light")
        );
    }
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn denied_operations_fail_closed_and_do_not_expose_policy_diagnostics() {
    let users = UserRegistry::in_memory();
    let policy = Policy {
        deny: AtomicBool::new(true),
    };
    let p = principal("untrusted", &policy);
    let error = users.create_user(&p).unwrap_err().to_string();
    assert_eq!(error, "User authorization denied");
    assert!(!error.contains("secret"));
}

#[test]
fn provider_namespaces_are_independent_and_unlink_is_scoped() {
    let users = UserRegistry::in_memory();
    let policy = Policy {
        deny: AtomicBool::new(false),
    };
    let p = principal("app", &policy);
    let user = users.create_user(&p).unwrap();
    let first = ExternalIdentity::new("telegram", "same-subject").unwrap();
    let second = ExternalIdentity::new("discord", "same-subject").unwrap();
    users.link_identity(&user, &first, &p).unwrap();
    users.link_identity(&user, &second, &p).unwrap();
    assert_eq!(
        users.resolve_identity(&first, &p).unwrap(),
        Some(user.clone())
    );
    assert_eq!(
        users.resolve_identity(&second, &p).unwrap(),
        Some(user.clone())
    );
    assert!(users.unlink_identity("another-user", &first, &p).is_err());
    users.unlink_identity(&user, &first, &p).unwrap();
    assert_eq!(users.resolve_identity(&first, &p).unwrap(), None);
    assert_eq!(users.resolve_identity(&second, &p).unwrap(), Some(user));
}

#[test]
fn concurrent_identity_claims_across_connections_have_one_owner() {
    use std::sync::{Arc, Barrier};
    let path = std::env::temp_dir().join(format!(
        "matter-user-race-{}-{}.db",
        std::process::id(),
        time::OffsetDateTime::now_utc().unix_timestamp_nanos()
    ));
    let bootstrap = UserRegistry::open_sqlite(&path).unwrap();
    let policy = Policy {
        deny: AtomicBool::new(false),
    };
    let p = principal("app", &policy);
    let first_user = bootstrap.create_user(&p).unwrap();
    let second_user = bootstrap.create_user(&p).unwrap();
    drop(bootstrap);
    let identity = ExternalIdentity::new("discord", "same-user").unwrap();
    let barrier = Arc::new(Barrier::new(3));
    let mut threads = Vec::new();
    for user in [first_user, second_user] {
        let path = path.clone();
        let barrier = barrier.clone();
        threads.push(std::thread::spawn(move || {
            let registry = UserRegistry::open_sqlite(path).unwrap();
            let policy = Policy {
                deny: AtomicBool::new(false),
            };
            let principal = UserPrincipal::new("app", &policy).unwrap();
            let identity = ExternalIdentity::new("discord", "same-user").unwrap();
            barrier.wait();
            registry.link_identity(&user, &identity, &principal).is_ok()
        }));
    }
    barrier.wait();
    let successes = threads
        .into_iter()
        .map(|thread| thread.join().unwrap())
        .filter(|succeeded| *succeeded)
        .count();
    assert_eq!(successes, 1);
    let registry = UserRegistry::open_sqlite(&path).unwrap();
    assert!(registry.resolve_identity(&identity, &p).unwrap().is_some());
    drop(registry);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn runtime_uses_injected_persistent_shared_user_registry() {
    use matter_base::{BotRegistry, CoreConfig, LegalStore, MatterRuntime};
    let path = std::env::temp_dir().join(format!(
        "matter-user-runtime-{}-{}.db",
        std::process::id(),
        time::OffsetDateTime::now_utc().unix_timestamp_nanos()
    ));
    let policy = Policy {
        deny: AtomicBool::new(false),
    };
    let principal = principal("app", &policy);
    let user_id;
    {
        let runtime = MatterRuntime::with_stores_and_users(
            CoreConfig::default(),
            LegalStore::open_in_memory().unwrap(),
            BotRegistry::in_memory(),
            UserRegistry::open_sqlite(&path).unwrap(),
        );
        user_id = runtime.context().users.create_user(&principal).unwrap();
        runtime
            .context()
            .users
            .set_bot_data("bot-x", &user_id, "locale", "es", &principal)
            .unwrap();
    }
    {
        let runtime = MatterRuntime::with_stores_and_users(
            CoreConfig::default(),
            LegalStore::open_in_memory().unwrap(),
            BotRegistry::in_memory(),
            UserRegistry::open_sqlite(&path).unwrap(),
        );
        assert_eq!(
            runtime
                .context()
                .users
                .bot_data("bot-x", &user_id, "locale", &principal)
                .unwrap()
                .as_deref(),
            Some("es")
        );
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn sqlite_user_schema_migration_rolls_back_on_conflict_and_retries() {
    use rusqlite::{params, Connection};
    let path = std::env::temp_dir().join(format!(
        "matter-user-migration-{}-{}.db",
        std::process::id(),
        time::OffsetDateTime::now_utc().unix_timestamp_nanos()
    ));
    {
        let connection = Connection::open(&path).unwrap();
        connection.execute_batch(
            "CREATE TABLE matter_schema_versions (component TEXT PRIMARY KEY NOT NULL, version INTEGER NOT NULL);
             INSERT INTO matter_schema_versions(component,version) VALUES('users',0);
             CREATE TABLE matter_users (legacy TEXT);"
        ).unwrap();
    }
    assert!(UserRegistry::open_sqlite(&path).is_err());
    {
        let connection = Connection::open(&path).unwrap();
        let version: i64 = connection
            .query_row(
                "SELECT version FROM matter_schema_versions WHERE component=?1",
                params!["users"],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(version, 0);
        let related_tables: i64 = connection.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('user_external_identities','bot_user_data')",
            [], |row| row.get(0)
        ).unwrap();
        assert_eq!(related_tables, 0);
        connection.execute("DROP TABLE matter_users", []).unwrap();
    }
    let users = UserRegistry::open_sqlite(&path).unwrap();
    let policy = Policy {
        deny: AtomicBool::new(false),
    };
    assert!(users.create_user(&principal("app", &policy)).is_ok());
    drop(users);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn sqlite_user_store_rejects_future_schema_without_rewriting_it() {
    use rusqlite::{params, Connection};
    let path = std::env::temp_dir().join(format!(
        "matter-user-future-{}-{}.db",
        std::process::id(),
        time::OffsetDateTime::now_utc().unix_timestamp_nanos()
    ));
    {
        let connection = Connection::open(&path).unwrap();
        connection.execute_batch(
            "CREATE TABLE matter_schema_versions (component TEXT PRIMARY KEY NOT NULL, version INTEGER NOT NULL);
             CREATE TABLE protected_sentinel (value TEXT NOT NULL);"
        ).unwrap();
        connection
            .execute(
                "INSERT INTO matter_schema_versions(component,version) VALUES(?1,?2)",
                params!["users", 2],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO protected_sentinel(value) VALUES('preserved')",
                [],
            )
            .unwrap();
    }
    assert!(UserRegistry::open_sqlite(&path).is_err());
    {
        let connection = Connection::open(&path).unwrap();
        let version: i64 = connection
            .query_row(
                "SELECT version FROM matter_schema_versions WHERE component=?1",
                params!["users"],
                |row| row.get(0),
            )
            .unwrap();
        let sentinel: String = connection
            .query_row("SELECT value FROM protected_sentinel", [], |row| row.get(0))
            .unwrap();
        let user_tables: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='matter_users'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(version, 2);
        assert_eq!(sentinel, "preserved");
        assert_eq!(user_tables, 0);
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn missing_user_data_write_is_rejected_without_a_partial_value() {
    let users = UserRegistry::in_memory();
    let policy = Policy {
        deny: AtomicBool::new(false),
    };
    let principal = principal("app", &policy);
    assert!(users
        .set_bot_data("bot-a", "missing-user", "preference", "x", &principal)
        .is_err());
    assert_eq!(
        users
            .bot_data("bot-a", "missing-user", "preference", &principal)
            .unwrap(),
        None
    );
    assert!(users
        .set_bot_data(
            "bot-a",
            "missing-user",
            "large",
            &"x".repeat(65_537),
            &principal
        )
        .is_err());
}
