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
