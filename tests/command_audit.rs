/* Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Result};
use matter_base::{
    activation::SocketActivationStore,
    control::{ControlDispatcher, DispatchResult},
    BotRegistry, CommandAuditOutcome as Outcome, CommandAuditStore, CommandAuthorizer,
    CommandInvocation, CommandRouter,
};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

struct Temp(std::path::PathBuf);
impl Temp {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "matter-command-audit-{}-{}",
            std::process::id(),
            time::OffsetDateTime::now_utc().unix_timestamp_nanos()
        ));
        std::fs::create_dir(&p).unwrap();
        Self(p)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

struct Policy(u8);
impl CommandAuthorizer for Policy {
    fn authorize(&self, _: &CommandInvocation) -> Result<()> {
        match self.0 {
            1 => bail!("private denial diagnostic"),
            2 => panic!("private panic diagnostic"),
            _ => Ok(()),
        }
    }
}
fn router(audit: Arc<CommandAuditStore>, policy: u8) -> CommandRouter {
    CommandRouter::new_with_audit(vec!["#".into()], Arc::new(Policy(policy)), audit).unwrap()
}

#[test]
fn audited_dispatch_recovers_canonical_outcomes_and_enforces_owner_scoped_pages() {
    let dir = Temp::new();
    let path = dir.0.join("audit.db");
    let bots = BotRegistry::in_memory();
    let a = bots.register("f", "r", "a", "alice").unwrap();
    let b = bots.register("f", "r", "b", "bob").unwrap();
    {
        let audit = Arc::new(CommandAuditStore::open(&path).unwrap());
        let allowed = router(audit.clone(), 0);
        allowed
            .register("help", &["h".into()], |_| Ok("private response".into()))
            .unwrap();
        allowed
            .route(a.instance_id(), "member", "room", "#h private-payload-123")
            .unwrap();
        allowed
            .route(b.instance_id(), "member", "room", "#help")
            .unwrap();
        let denied = router(audit, 1);
        denied
            .register("help", &[], |_| panic!("denied handler ran"))
            .unwrap();
        let error = denied
            .route(a.instance_id(), "outsider", "room", "#help")
            .unwrap_err();
        assert_eq!(error.to_string(), "Command authorization denied");
        assert!(allowed
            .route(a.instance_id(), "member", "room", "#unknown")
            .is_err());
        assert_eq!(
            allowed
                .route(a.instance_id(), "member", "room", "ordinary message")
                .unwrap(),
            None
        );
    }
    let audit = CommandAuditStore::open(&path).unwrap();
    assert!(audit.history(a.instance_id(), "bob", &bots, 0, 10).is_err());
    assert!(audit
        .history(a.instance_id(), "alice", &bots, 0, 1001)
        .is_err());
    assert!(audit
        .history(a.instance_id(), "alice", &bots, -1, 10)
        .is_err());
    assert!(audit
        .history(a.instance_id(), "alice", &bots, 0, 0)
        .is_err());
    let first = audit
        .history(a.instance_id(), "alice", &bots, 0, 1)
        .unwrap();
    assert_eq!(first.len(), 1);
    assert_eq!(first[0].command, "help");
    assert_eq!(first[0].actor_id, "member");
    assert_eq!(first[0].outcome, Outcome::Succeeded);
    assert!(first[0].finished_at.is_some());
    let second = audit
        .history(a.instance_id(), "alice", &bots, first[0].id, 10)
        .unwrap();
    assert_eq!(second.len(), 1);
    assert_eq!(second[0].outcome, Outcome::Denied);
    bots.transfer_owner(a.instance_id(), "alice", "carol")
        .unwrap();
    assert!(audit
        .history(a.instance_id(), "alice", &bots, 0, 10)
        .is_err());
    assert_eq!(
        audit
            .history(a.instance_id(), "carol", &bots, 0, 10)
            .unwrap()
            .len(),
        2
    );
    let database = std::fs::read(&path).unwrap();
    for secret in [
        "private-payload-123",
        "private response",
        "private denial diagnostic",
    ] {
        assert!(!database
            .windows(secret.len())
            .any(|bytes| bytes == secret.as_bytes()));
    }
}

#[test]
fn callback_errors_and_unwinding_panics_have_durable_sanitized_outcomes() {
    let dir = Temp::new();
    let audit = Arc::new(CommandAuditStore::open(dir.0.join("audit.db")).unwrap());
    let bots = BotRegistry::in_memory();
    let bot = bots.register("f", "r", "a", "alice").unwrap();
    for policy in [1, 2] {
        let router = router(audit.clone(), policy);
        router
            .register("run", &[], |_| panic!("unauthorized handler"))
            .unwrap();
        assert_eq!(
            router
                .route(bot.instance_id(), "member", "room", "#run")
                .unwrap_err()
                .to_string(),
            "Command authorization denied"
        );
    }
    for panic in [false, true] {
        let router = router(audit.clone(), 0);
        router
            .add_middleware(move |_| {
                if panic {
                    panic!("private middleware diagnostic");
                }
                bail!("private middleware diagnostic")
            })
            .unwrap();
        router
            .register("run", &[], |_| panic!("rejected handler"))
            .unwrap();
        assert_eq!(
            router
                .route(bot.instance_id(), "member", "room", "#run")
                .unwrap_err()
                .to_string(),
            "Command middleware rejected"
        );
        let router = self::router(audit.clone(), 0);
        router
            .register("run", &[], move |_| {
                if panic {
                    panic!("private handler diagnostic");
                }
                bail!("private handler diagnostic")
            })
            .unwrap();
        assert_eq!(
            router
                .route(bot.instance_id(), "member", "room", "#run")
                .unwrap_err()
                .to_string(),
            "Command execution failed"
        );
    }
    let outcomes: Vec<_> = audit
        .history(bot.instance_id(), "alice", &bots, 0, 10)
        .unwrap()
        .into_iter()
        .map(|record| record.outcome)
        .collect();
    assert_eq!(
        outcomes,
        [
            Outcome::Denied,
            Outcome::Denied,
            Outcome::MiddlewareRejected,
            Outcome::Failed,
            Outcome::MiddlewareRejected,
            Outcome::Failed
        ]
    );
}

#[test]
fn audit_start_failure_prevents_callbacks_and_finish_failure_preserves_pending_attempt() {
    let dir = Temp::new();
    let path = dir.0.join("audit.db");
    let audit = Arc::new(CommandAuditStore::open(&path).unwrap());
    let bots = BotRegistry::in_memory();
    let bot = bots.register("f", "r", "a", "alice").unwrap();
    let c = rusqlite::Connection::open(&path).unwrap();
    let calls = Arc::new(AtomicUsize::new(0));
    struct CountingPolicy(Arc<AtomicUsize>);
    impl CommandAuthorizer for CountingPolicy {
        fn authorize(&self, _: &CommandInvocation) -> Result<()> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }
    let router = CommandRouter::new_with_audit(
        vec!["#".into()],
        Arc::new(CountingPolicy(calls.clone())),
        audit.clone(),
    )
    .unwrap();
    let effects = Arc::new(AtomicUsize::new(0));
    let handler_effects = effects.clone();
    router
        .register("run", &[], move |_| {
            handler_effects.fetch_add(1, Ordering::SeqCst);
            Ok("done".into())
        })
        .unwrap();
    c.execute_batch(
        "CREATE TRIGGER reject_start BEFORE INSERT ON command_audit_attempts
        BEGIN SELECT RAISE(ABORT,'private database diagnostic'); END;",
    )
    .unwrap();
    assert_eq!(
        router
            .route(bot.instance_id(), "member", "room", "#run")
            .unwrap_err()
            .to_string(),
        "Command audit persistence failed"
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert_eq!(effects.load(Ordering::SeqCst), 0);
    assert!(audit
        .history(bot.instance_id(), "alice", &bots, 0, 10)
        .unwrap()
        .is_empty());
    c.execute_batch(
        "DROP TRIGGER reject_start;
        CREATE TRIGGER reject_finish BEFORE INSERT ON command_audit_outcomes
        BEGIN SELECT RAISE(ABORT,'private database diagnostic'); END;",
    )
    .unwrap();
    assert_eq!(
        router
            .route(bot.instance_id(), "member", "room", "#run")
            .unwrap_err()
            .to_string(),
        "Command audit persistence failed"
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(effects.load(Ordering::SeqCst), 1);
    drop(router);
    drop(audit);
    let recovered = CommandAuditStore::open(&path).unwrap();
    let history = recovered
        .history(bot.instance_id(), "alice", &bots, 0, 10)
        .unwrap();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].outcome, Outcome::Pending);
    assert_eq!(history[0].finished_at, None);
}

#[test]
fn independent_router_connections_persist_unique_immutable_attempts_and_outcomes() {
    let dir = Temp::new();
    let path = dir.0.join("audit.db");
    let bots = BotRegistry::in_memory();
    let bot = bots.register("f", "r", "a", "alice").unwrap();
    let barrier = Arc::new(std::sync::Barrier::new(2));
    let mut joins = Vec::new();
    for _ in 0..2 {
        let router = router(Arc::new(CommandAuditStore::open(&path).unwrap()), 0);
        let barrier = barrier.clone();
        router
            .register("run", &[], move |_| {
                barrier.wait();
                Ok("done".into())
            })
            .unwrap();
        let id = bot.instance_id().to_owned();
        joins.push(std::thread::spawn(move || {
            router.route(&id, "member", "room", "#run").unwrap()
        }));
    }
    for join in joins {
        assert_eq!(join.join().unwrap(), Some("done".into()));
    }
    let audit = CommandAuditStore::open(&path).unwrap();
    let rows = audit
        .history(bot.instance_id(), "alice", &bots, 0, 10)
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert_ne!(rows[0].id, rows[1].id);
    assert!(rows
        .iter()
        .all(|record| record.outcome == Outcome::Succeeded));
    let c = rusqlite::Connection::open(&path).unwrap();
    for sql in [
        "UPDATE command_audit_attempts SET actor_id='forged'",
        "DELETE FROM command_audit_attempts",
        "UPDATE command_audit_outcomes SET outcome='failed'",
        "DELETE FROM command_audit_outcomes",
        "INSERT INTO command_audit_outcomes SELECT * FROM command_audit_outcomes",
        "INSERT OR REPLACE INTO command_audit_attempts SELECT * FROM command_audit_attempts",
        "INSERT OR REPLACE INTO command_audit_outcomes SELECT * FROM command_audit_outcomes",
    ] {
        assert!(c.execute(sql, []).is_err());
    }
    assert_eq!(
        audit
            .history(bot.instance_id(), "alice", &bots, 0, 10)
            .unwrap(),
        rows
    );
}

#[test]
fn schema_failure_rolls_back_and_future_version_is_untouched() {
    let dir = Temp::new();
    let path = dir.0.join("audit.db");
    let c = rusqlite::Connection::open(&path).unwrap();
    c.execute_batch("CREATE VIEW command_audit_outcomes AS SELECT 1;")
        .unwrap();
    assert!(CommandAuditStore::open(&path).is_err());
    let tables: i64 = c
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(tables, 0);
    c.execute_batch("DROP VIEW command_audit_outcomes;")
        .unwrap();
    drop(CommandAuditStore::open(&path).unwrap());
    c.execute_batch(
        "UPDATE matter_schema_versions SET version=99 WHERE component='command_audit';",
    )
    .unwrap();
    assert!(CommandAuditStore::open(&path).is_err());
    let version: i64 = c
        .query_row(
            "SELECT version FROM matter_schema_versions WHERE component='command_audit'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(version, 99);
}

#[test]
fn control_dispatch_persists_normal_command_outcomes_only_after_socket_admission() {
    let dir = Temp::new();
    let bots = Arc::new(BotRegistry::open_sqlite(dir.0.join("bots.db")).unwrap());
    let bot = bots.register("f", "r", "a", "alice").unwrap();
    let audit = Arc::new(CommandAuditStore::open(dir.0.join("audit.db")).unwrap());
    let router = Arc::new(router(audit.clone(), 0));
    router.register("help", &[], |_| Ok("help".into())).unwrap();
    let activation = Arc::new(SocketActivationStore::open(dir.0.join("activation.db")).unwrap());
    activation.provision("socket", "alice").unwrap();
    activation
        .bind_bot("socket", "alice", bot.instance_id(), &bots)
        .unwrap();
    let dispatcher = ControlDispatcher::new(activation.clone(), bots.clone(), router, Some(true));
    assert_eq!(
        dispatcher
            .dispatch("socket", "member", "room", None, "#help")
            .unwrap(),
        DispatchResult::Inactive
    );
    dispatcher
        .dispatch("socket", "alice", "room", None, "=on")
        .unwrap();
    assert_eq!(
        dispatcher
            .dispatch("socket", "member", "room", None, "#help")
            .unwrap(),
        DispatchResult::Message(Some("help".into()))
    );
    dispatcher
        .dispatch("socket", "alice", "room", None, "=off")
        .unwrap();
    assert_eq!(
        dispatcher
            .dispatch("socket", "member", "room", None, "#help")
            .unwrap(),
        DispatchResult::Inactive
    );
    let rows = audit
        .history(bot.instance_id(), "alice", &bots, 0, 10)
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].outcome, Outcome::Succeeded);
    assert_eq!(
        activation.audit_history("socket", "alice").unwrap().len(),
        4
    );
}
