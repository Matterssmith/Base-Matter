/*
 * Matter-Base — Cross-module Flow Tests
 *
 * Verifica el uso conjunto de identidad, runtime, persistencia, eventos,
 * servicios y ciclo de vida de módulos.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::Result;
use matter_base::{
    BotRegistry, ConsentDecision, CoreConfig, CoreModule, LegalDocument, LegalDocumentType,
    MatterContext, MatterRuntime, UserAction, UserAuthorizationRequest, UserAuthorizer,
    UserPrincipal, UserRegistry,
};
mod common;
use common::{legal_admin, legal_user};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Barrier, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("matter-base-flow-{}-{nonce}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

struct RuntimeService(&'static str);

struct RuntimeModule {
    callbacks: Arc<Mutex<Vec<&'static str>>>,
    fail_start: bool,
}

impl CoreModule for RuntimeModule {
    fn name(&self) -> &str {
        "flow-module"
    }

    fn initialize(&self, context: &MatterContext) -> Result<()> {
        context.services.register(RuntimeService("initialized"));
        self.callbacks.lock().unwrap().push("initialized");
        Ok(())
    }

    fn start(&self, context: &MatterContext) -> Result<()> {
        self.callbacks.lock().unwrap().push("started");
        if self.fail_start {
            anyhow::bail!("simulated module start failure");
        }
        context.events.publish_checked("runtime.started", "ok")?;
        Ok(())
    }

    fn stop(&self, _context: &MatterContext) -> Result<()> {
        self.callbacks.lock().unwrap().push("stopped");
        Ok(())
    }
}

struct AllowUserCreation;

impl UserAuthorizer for AllowUserCreation {
    fn authorize(&self, request: &UserAuthorizationRequest<'_>) -> Result<()> {
        if request.action == UserAction::Create {
            Ok(())
        } else {
            anyhow::bail!("unexpected action")
        }
    }
}

struct BotOwnerDataPolicy {
    bots: BotRegistry,
    user_id: String,
}

impl UserAuthorizer for BotOwnerDataPolicy {
    fn authorize(&self, request: &UserAuthorizationRequest<'_>) -> Result<()> {
        if !matches!(
            request.action,
            UserAction::ReadData | UserAction::WriteData | UserAction::DeleteData
        ) || request.user_id != Some(self.user_id.as_str())
        {
            anyhow::bail!("user data scope denied");
        }

        let bot_id = request
            .bot_instance_id
            .ok_or_else(|| anyhow::anyhow!("bot scope required"))?;
        self.bots
            .authorize(bot_id, request.principal_id, matter_base::BotAction::Manage)
    }
}

#[test]
fn bot_owner_authorization_and_shared_user_data_are_isolated_together() {
    let directory = TemporaryDirectory::new();
    let bots_path = directory.0.join("bots.db");
    let users_path = directory.0.join("users.db");
    let (first_bot, second_bot, user_id) = {
        let bots = BotRegistry::open_sqlite(&bots_path).unwrap();
        let first_bot = bots.register("family", "v1", "first", "owner-a").unwrap();
        let second_bot = bots.register("family", "v1", "second", "owner-b").unwrap();
        let users = UserRegistry::open_sqlite(&users_path).unwrap();
        let create_principal = UserPrincipal::new("owner-a", &AllowUserCreation).unwrap();
        let user_id = users.create_user(&create_principal).unwrap();
        let owner_policy = BotOwnerDataPolicy {
            bots,
            user_id: user_id.clone(),
        };
        let owner_principal = UserPrincipal::new("owner-a", &owner_policy).unwrap();

        users
            .set_bot_data(
                first_bot.instance_id(),
                &user_id,
                "preference",
                "dark",
                &owner_principal,
            )
            .unwrap();
        assert!(users
            .set_bot_data(
                second_bot.instance_id(),
                &user_id,
                "preference",
                "leak",
                &owner_principal
            )
            .is_err());
        (first_bot, second_bot, user_id)
    };

    // Reopen both independent stores to verify the cross-module policy is
    // still based on recovered bot ownership and user-scoped data.
    let bots = BotRegistry::open_sqlite(&bots_path).unwrap();
    let users = UserRegistry::open_sqlite(&users_path).unwrap();
    assert_eq!(
        bots.get(first_bot.instance_id()).unwrap(),
        Some(first_bot.clone())
    );
    let owner_policy = BotOwnerDataPolicy {
        bots: bots.clone(),
        user_id: user_id.clone(),
    };
    let owner_principal = UserPrincipal::new("owner-a", &owner_policy).unwrap();
    assert_eq!(
        users
            .bot_data(
                first_bot.instance_id(),
                &user_id,
                "preference",
                &owner_principal
            )
            .unwrap()
            .as_deref(),
        Some("dark")
    );

    let other_owner_policy = BotOwnerDataPolicy {
        bots,
        user_id: user_id.clone(),
    };
    let other_owner = UserPrincipal::new("owner-b", &other_owner_policy).unwrap();
    assert_eq!(
        users
            .bot_data(
                second_bot.instance_id(),
                &user_id,
                "preference",
                &other_owner
            )
            .unwrap(),
        None
    );
    assert!(users
        .bot_data(
            first_bot.instance_id(),
            &user_id,
            "preference",
            &other_owner
        )
        .is_err());
}

#[test]
fn persistent_identity_runtime_service_events_and_module_lifecycle_work_together() {
    let directory = TemporaryDirectory::new();
    let registry_path = directory.0.join("bots.db");
    let registry = BotRegistry::open_sqlite(&registry_path).unwrap();
    let identity = registry
        .register("shared-family", "release-1", "matter", "owner-1")
        .unwrap();
    let runtime = MatterRuntime::open_with_bot_registry(
        CoreConfig::default(),
        directory.0.join("legal.db"),
        registry,
    )
    .unwrap();
    assert_eq!(
        runtime.context.bots.get(identity.instance_id()).unwrap(),
        Some(identity.clone())
    );

    runtime
        .context
        .legal
        .register_user("flow-user", &legal_user("flow-user"))
        .unwrap();
    runtime
        .context
        .legal
        .register_document(
            &LegalDocument {
                document_id: "flow-terms-v1".to_owned(),
                scope: "matter-base-flow".to_owned(),
                document_type: LegalDocumentType::Terms,
                version: "1.0.0".to_owned(),
                locale: "es".to_owned(),
                content: "Integration flow terms".to_owned(),
                mandatory: true,
                required_decision: ConsentDecision::Accepted,
                active: true,
            },
            &legal_admin(),
        )
        .unwrap();
    assert!(runtime
        .ensure_access(
            "flow-user",
            "matter-base-flow",
            "es",
            &legal_user("flow-user")
        )
        .is_err());
    runtime
        .context
        .legal
        .record_decision(
            "flow-user",
            "flow-terms-v1",
            ConsentDecision::Accepted,
            Some("integration-test"),
            &legal_user("flow-user"),
        )
        .unwrap();
    assert!(runtime
        .ensure_access(
            "flow-user",
            "matter-base-flow",
            "es",
            &legal_user("flow-user")
        )
        .is_ok());
    runtime
        .context
        .legal
        .record_decision(
            "flow-user",
            "flow-terms-v1",
            ConsentDecision::Declined,
            Some("integration-test"),
            &legal_user("flow-user"),
        )
        .unwrap();
    assert!(runtime
        .ensure_access(
            "flow-user",
            "matter-base-flow",
            "es",
            &legal_user("flow-user")
        )
        .is_err());
    assert_eq!(
        runtime
            .context
            .legal
            .decision_history("flow-user", &legal_user("flow-user"))
            .unwrap()
            .len(),
        2
    );

    let callbacks = Arc::new(Mutex::new(Vec::new()));
    let observed = Arc::new(Mutex::new(Vec::new()));
    let observed_event = Arc::clone(&observed);
    runtime
        .context
        .events
        .subscribe("runtime.started", move |payload| {
            observed_event.lock().unwrap().push(payload.to_owned());
        });

    runtime
        .register_module(Box::new(RuntimeModule {
            callbacks: Arc::clone(&callbacks),
            fail_start: false,
        }))
        .unwrap();
    runtime.initialize().unwrap();
    runtime.start().unwrap();

    let service = runtime.context.services.get::<RuntimeService>().unwrap();
    assert_eq!(service.0, "initialized");
    assert_eq!(*observed.lock().unwrap(), ["ok"]);
    assert!(runtime
        .context
        .legal
        .user_exists("flow-user", &legal_user("flow-user"))
        .unwrap());

    runtime.shutdown().unwrap();
    assert_eq!(
        *callbacks.lock().unwrap(),
        ["initialized", "started", "stopped"]
    );

    drop(runtime);
    let recovered_registry = BotRegistry::open_sqlite(&registry_path).unwrap();
    let recovered_runtime = MatterRuntime::open_with_bot_registry(
        CoreConfig::default(),
        directory.0.join("legal.db"),
        recovered_registry,
    )
    .unwrap();
    assert_eq!(
        recovered_runtime
            .context
            .bots
            .get(identity.instance_id())
            .unwrap(),
        Some(identity.clone())
    );
    assert!(recovered_runtime
        .context
        .legal
        .user_exists("flow-user", &legal_user("flow-user"))
        .unwrap());
    assert_eq!(
        recovered_runtime
            .context
            .legal
            .decision_history("flow-user", &legal_user("flow-user"))
            .unwrap()
            .len(),
        2
    );
    assert!(recovered_runtime
        .ensure_access(
            "flow-user",
            "matter-base-flow",
            "es",
            &legal_user("flow-user")
        )
        .is_err());
}

#[test]
fn runtime_propagates_start_failure_and_cleans_module_state() {
    let runtime = MatterRuntime::with_legal_store(
        CoreConfig::default(),
        matter_base::LegalStore::open_in_memory().unwrap(),
    );
    let callbacks = Arc::new(Mutex::new(Vec::new()));
    runtime
        .register_module(Box::new(RuntimeModule {
            callbacks: Arc::clone(&callbacks),
            fail_start: true,
        }))
        .unwrap();

    assert!(runtime.start().is_err());
    assert_eq!(
        *callbacks.lock().unwrap(),
        ["initialized", "started", "stopped"]
    );
    runtime.shutdown().unwrap();
}

#[test]
fn cloned_runtime_serializes_concurrent_start_calls() {
    let runtime = MatterRuntime::with_legal_store(
        CoreConfig::default(),
        matter_base::LegalStore::open_in_memory().unwrap(),
    );
    let callbacks = Arc::new(Mutex::new(Vec::new()));
    runtime
        .register_module(Box::new(RuntimeModule {
            callbacks: Arc::clone(&callbacks),
            fail_start: false,
        }))
        .unwrap();

    let barrier = Arc::new(Barrier::new(3));
    let first_runtime = runtime.clone();
    let first_barrier = Arc::clone(&barrier);
    let first = std::thread::spawn(move || {
        first_barrier.wait();
        first_runtime.start()
    });
    let second_runtime = runtime.clone();
    let second_barrier = Arc::clone(&barrier);
    let second = std::thread::spawn(move || {
        second_barrier.wait();
        second_runtime.start()
    });

    barrier.wait();
    first.join().unwrap().unwrap();
    second.join().unwrap().unwrap();
    assert_eq!(*callbacks.lock().unwrap(), ["initialized", "started"]);
    runtime.shutdown().unwrap();
    assert_eq!(
        *callbacks.lock().unwrap(),
        ["initialized", "started", "stopped"]
    );
}
