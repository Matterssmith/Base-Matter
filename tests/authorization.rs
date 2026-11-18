/* Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use matter_base::{
    activation::SocketActivationStore,
    control::{ControlDispatcher, DispatchResult},
    BotRegistry, CommandAuditOutcome, CommandAuditStore, CommandRouter, PermissionEffect,
    ResourcePermission, ResourceRequest, RoleCommandAuthorizer, RoleDefinition, RoleStore,
};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

struct Temp(std::path::PathBuf);
impl Temp {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "matter-roles-{}-{}",
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
fn permission(resource: &str, effect: PermissionEffect) -> ResourcePermission {
    ResourcePermission {
        resource_type: "command".into(),
        resource_id: resource.into(),
        action: "execute".into(),
        effect,
    }
}
fn allowed(resource: &str) -> RoleDefinition {
    RoleDefinition {
        inherits: vec![],
        permissions: vec![permission(resource, PermissionEffect::Allow)],
    }
}
fn request<'a>(bot: &'a str, subject: &'a str, resource: &'a str) -> ResourceRequest<'a> {
    ResourceRequest {
        bot_instance_id: bot,
        subject_id: subject,
        resource_type: "command",
        resource_id: resource,
        action: "execute",
    }
}

#[test]
fn persistent_roles_gate_canonical_commands_with_socket_power_and_command_audit() {
    let dir = Temp::new();
    let bots = Arc::new(BotRegistry::open_sqlite(dir.0.join("bots.db")).unwrap());
    let a = bots.register("f", "r", "a", "alice").unwrap();
    let b = bots.register("f", "r", "b", "bob").unwrap();
    let roles_path = dir.0.join("roles.db");
    {
        let roles = RoleStore::open(&roles_path, bots.clone()).unwrap();
        roles.provision_scope(a.instance_id(), "alice").unwrap();
        roles.provision_scope(b.instance_id(), "bob").unwrap();
        roles
            .set_role(a.instance_id(), "alice", "reader", allowed("help"))
            .unwrap();
        roles
            .assign_role(a.instance_id(), "alice", "member", "reader")
            .unwrap();
    }
    let roles = Arc::new(RoleStore::open(&roles_path, bots.clone()).unwrap());
    let audit = Arc::new(CommandAuditStore::open(dir.0.join("audit.db")).unwrap());
    let router = Arc::new(
        CommandRouter::new_with_audit(
            vec!["#".into()],
            Arc::new(RoleCommandAuthorizer::new(roles.clone())),
            audit.clone(),
        )
        .unwrap(),
    );
    let calls = Arc::new(AtomicUsize::new(0));
    let handler_calls = calls.clone();
    router
        .register("help", &["h".into()], move |_| {
            handler_calls.fetch_add(1, Ordering::SeqCst);
            Ok("help".into())
        })
        .unwrap();
    let activation = Arc::new(SocketActivationStore::open(dir.0.join("activation.db")).unwrap());
    for (socket, owner, bot) in [("a", "alice", &a), ("b", "bob", &b)] {
        activation.provision(socket, owner).unwrap();
        activation
            .bind_bot(socket, owner, bot.instance_id(), &bots)
            .unwrap();
    }
    let dispatcher = ControlDispatcher::new(activation.clone(), bots.clone(), router, Some(true));
    assert_eq!(
        dispatcher
            .dispatch("a", "member", "room", None, "#h")
            .unwrap(),
        DispatchResult::Inactive
    );
    dispatcher
        .dispatch("a", "alice", "room", None, "=on")
        .unwrap();
    dispatcher
        .dispatch("b", "bob", "room", None, "=on")
        .unwrap();
    assert_eq!(
        dispatcher
            .dispatch("a", "member", "room", None, "#h")
            .unwrap(),
        DispatchResult::Message(Some("help".into()))
    );
    assert!(dispatcher
        .dispatch("b", "member", "room", None, "#h")
        .is_err());
    // Owning the bot or naming oneself global-owner does not imply command grants.
    assert!(dispatcher
        .dispatch("a", "alice", "room", None, "#help")
        .is_err());
    assert!(dispatcher
        .dispatch("a", "global-owner", "room", None, "#help")
        .is_err());
    roles
        .revoke_role(a.instance_id(), "alice", "member", "reader")
        .unwrap();
    assert!(dispatcher
        .dispatch("a", "member", "room", None, "#h")
        .is_err());
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    let records = audit
        .history(a.instance_id(), "alice", &bots, 0, 10)
        .unwrap();
    assert_eq!(records.len(), 4);
    assert_eq!(records[0].command, "help");
    assert_eq!(records[0].outcome, CommandAuditOutcome::Succeeded);
    assert!(records[1..]
        .iter()
        .all(|record| record.outcome == CommandAuditOutcome::Denied));
    assert!(roles.audit_history(a.instance_id(), "bob", 0, 100).is_err());
    assert_eq!(
        roles
            .audit_history(a.instance_id(), "alice", 0, 100)
            .unwrap()
            .len(),
        4
    );
    roles
        .set_role(
            a.instance_id(),
            "alice",
            "all",
            RoleDefinition {
                inherits: vec![],
                permissions: vec![ResourcePermission {
                    resource_type: "*".into(),
                    resource_id: "*".into(),
                    action: "*".into(),
                    effect: PermissionEffect::Allow,
                }],
            },
        )
        .unwrap();
    roles
        .assign_role(a.instance_id(), "alice", "member", "all")
        .unwrap();
    assert!(roles
        .is_allowed(&ResourceRequest {
            bot_instance_id: a.instance_id(),
            subject_id: "member",
            resource_type: "socket",
            resource_id: "a",
            action: "power",
        })
        .unwrap());
    assert!(dispatcher
        .dispatch("a", "member", "room", None, "=off a")
        .is_err());
    assert!(activation.resolve("a", None, Some(true)).unwrap().enabled);
}

#[test]
fn inherited_denials_dominate_wildcards_and_invalid_graph_changes_are_atomic() {
    let dir = Temp::new();
    let bots = Arc::new(BotRegistry::in_memory());
    let bot = bots.register("f", "r", "a", "alice").unwrap();
    let roles = RoleStore::open(dir.0.join("roles.db"), bots.clone()).unwrap();
    roles.provision_scope(bot.instance_id(), "alice").unwrap();
    roles
        .set_role(bot.instance_id(), "alice", "base", allowed("*"))
        .unwrap();
    let restricted = RoleDefinition {
        inherits: vec!["base".into()],
        permissions: vec![permission("delete", PermissionEffect::Deny)],
    };
    roles
        .set_role(bot.instance_id(), "alice", "restricted", restricted.clone())
        .unwrap();
    roles
        .assign_role(bot.instance_id(), "alice", "member", "restricted")
        .unwrap();
    roles
        .assign_role(bot.instance_id(), "alice", "member", "base")
        .unwrap();
    assert!(roles
        .is_allowed(&request(bot.instance_id(), "member", "help"))
        .unwrap());
    assert!(!roles
        .is_allowed(&request(bot.instance_id(), "member", "delete"))
        .unwrap());
    assert!(!roles
        .is_allowed(&request(bot.instance_id(), "stranger", "help"))
        .unwrap());
    let before = roles.policy(bot.instance_id(), "alice").unwrap();
    let count = roles
        .audit_history(bot.instance_id(), "alice", 0, 100)
        .unwrap()
        .len();
    let cyclic = RoleDefinition {
        inherits: vec!["restricted".into()],
        permissions: vec![],
    };
    assert!(roles
        .set_role(bot.instance_id(), "alice", "base", cyclic)
        .is_err());
    assert!(roles
        .set_role(
            bot.instance_id(),
            "alice",
            "missing",
            RoleDefinition {
                inherits: vec!["absent".into()],
                permissions: vec![]
            }
        )
        .is_err());
    assert!(roles
        .remove_role(bot.instance_id(), "alice", "base")
        .is_err());
    assert_eq!(roles.policy(bot.instance_id(), "alice").unwrap(), before);
    assert_eq!(
        roles
            .audit_history(bot.instance_id(), "alice", 0, 100)
            .unwrap()
            .len(),
        count
    );
    roles
        .remove_role(bot.instance_id(), "alice", "restricted")
        .unwrap();
    assert!(roles
        .is_allowed(&request(bot.instance_id(), "member", "delete"))
        .unwrap());
    roles
        .remove_role(bot.instance_id(), "alice", "base")
        .unwrap();
    assert!(!roles
        .is_allowed(&request(bot.instance_id(), "member", "help"))
        .unwrap());
}

#[test]
fn ownership_change_denies_policy_until_new_owner_resets_and_does_not_inherit_grants() {
    let dir = Temp::new();
    let bots = Arc::new(BotRegistry::in_memory());
    let bot = bots.register("f", "r", "a", "alice").unwrap();
    bots.add_co_owner(bot.instance_id(), "alice", "helper")
        .unwrap();
    let roles = RoleStore::open(dir.0.join("roles.db"), bots.clone()).unwrap();
    assert!(roles.provision_scope(bot.instance_id(), "helper").is_err());
    roles.provision_scope(bot.instance_id(), "alice").unwrap();
    roles
        .set_role(bot.instance_id(), "alice", "reader", allowed("help"))
        .unwrap();
    roles
        .assign_role(bot.instance_id(), "alice", "member", "reader")
        .unwrap();
    bots.transfer_owner(bot.instance_id(), "alice", "bob")
        .unwrap();
    assert!(roles
        .is_allowed(&request(bot.instance_id(), "member", "help"))
        .is_err());
    assert!(roles
        .set_role(bot.instance_id(), "alice", "other", allowed("*"))
        .is_err());
    assert!(roles
        .set_role(bot.instance_id(), "bob", "other", allowed("*"))
        .is_err());
    assert!(roles.reset_scope(bot.instance_id(), "alice").is_err());
    assert!(roles
        .audit_history(bot.instance_id(), "alice", 0, 100)
        .is_err());
    assert_eq!(
        roles
            .audit_history(bot.instance_id(), "bob", 0, 100)
            .unwrap()
            .len(),
        3
    );
    roles.reset_scope(bot.instance_id(), "bob").unwrap();
    assert!(roles
        .policy(bot.instance_id(), "bob")
        .unwrap()
        .roles
        .is_empty());
    assert!(!roles
        .is_allowed(&request(bot.instance_id(), "member", "help"))
        .unwrap());
    roles
        .set_role(bot.instance_id(), "bob", "reader", allowed("help"))
        .unwrap();
    roles
        .assign_role(bot.instance_id(), "bob", "new-member", "reader")
        .unwrap();
    assert!(roles
        .is_allowed(&request(bot.instance_id(), "new-member", "help"))
        .unwrap());
    assert!(!roles
        .is_allowed(&request(bot.instance_id(), "member", "help"))
        .unwrap());
}

#[test]
fn policy_limits_reject_excessive_roles_depth_membership_and_permissions() {
    let dir = Temp::new();
    let bots = Arc::new(BotRegistry::in_memory());
    let bot = bots.register("f", "r", "a", "alice").unwrap();
    let roles = RoleStore::open(dir.0.join("roles.db"), bots).unwrap();
    roles.provision_scope(bot.instance_id(), "alice").unwrap();
    for i in 0..16 {
        roles
            .set_role(
                bot.instance_id(),
                "alice",
                &format!("r{i}"),
                RoleDefinition {
                    inherits: if i == 0 {
                        vec![]
                    } else {
                        vec![format!("r{}", i - 1)]
                    },
                    permissions: vec![],
                },
            )
            .unwrap();
    }
    assert!(roles
        .set_role(
            bot.instance_id(),
            "alice",
            "too-deep",
            RoleDefinition {
                inherits: vec!["r15".into()],
                permissions: vec![]
            }
        )
        .is_err());
    for i in 16..64 {
        roles
            .set_role(
                bot.instance_id(),
                "alice",
                &format!("r{i}"),
                RoleDefinition::default(),
            )
            .unwrap();
    }
    assert!(roles
        .set_role(
            bot.instance_id(),
            "alice",
            "overflow",
            RoleDefinition::default()
        )
        .is_err());
    assert!(roles
        .set_role(
            bot.instance_id(),
            "alice",
            "r0",
            RoleDefinition {
                inherits: vec![],
                permissions: vec![permission("help", PermissionEffect::Allow); 129]
            }
        )
        .is_err());
    for i in 0..16 {
        roles
            .assign_role(bot.instance_id(), "alice", "member", &format!("r{i}"))
            .unwrap();
    }
    assert!(roles
        .assign_role(bot.instance_id(), "alice", "member", "r16")
        .is_err());
    assert_eq!(
        roles
            .policy(bot.instance_id(), "alice")
            .unwrap()
            .assignments["member"]
            .len(),
        16
    );
    assert!(roles
        .is_allowed(&request(bot.instance_id(), "member", "*"))
        .is_err());
    assert!(roles
        .audit_history(bot.instance_id(), "alice", 0, 101)
        .is_err());
}

#[test]
fn audit_failure_rolls_back_policy_and_corrupt_state_fails_closed_until_owner_reset() {
    let dir = Temp::new();
    let path = dir.0.join("roles.db");
    let bots = Arc::new(BotRegistry::in_memory());
    let a = bots.register("f", "r", "a", "alice").unwrap();
    let b = bots.register("f", "r", "b", "bob").unwrap();
    let roles = RoleStore::open(&path, bots).unwrap();
    roles.provision_scope(a.instance_id(), "alice").unwrap();
    roles
        .set_role(a.instance_id(), "alice", "reader", allowed("help"))
        .unwrap();
    roles
        .assign_role(a.instance_id(), "alice", "member", "reader")
        .unwrap();
    let before = roles.policy(a.instance_id(), "alice").unwrap();
    let c = rusqlite::Connection::open(&path).unwrap();
    c.execute_batch("CREATE TRIGGER reject_role_audit BEFORE INSERT ON resource_role_audit BEGIN SELECT RAISE(ABORT,'injected'); END;").unwrap();
    assert!(roles.provision_scope(b.instance_id(), "bob").is_err());
    assert!(roles
        .set_role(a.instance_id(), "alice", "reader", allowed("*"))
        .is_err());
    assert!(roles
        .assign_role(a.instance_id(), "alice", "new", "reader")
        .is_err());
    assert!(roles
        .revoke_role(a.instance_id(), "alice", "member", "reader")
        .is_err());
    assert!(roles
        .remove_role(a.instance_id(), "alice", "reader")
        .is_err());
    assert!(roles.reset_scope(a.instance_id(), "alice").is_err());
    assert_eq!(roles.policy(a.instance_id(), "alice").unwrap(), before);
    assert_eq!(
        roles
            .audit_history(a.instance_id(), "alice", 0, 100)
            .unwrap()
            .len(),
        3
    );
    c.execute_batch("DROP TRIGGER reject_role_audit;").unwrap();
    roles.provision_scope(b.instance_id(), "bob").unwrap();
    for body in [
        "{broken".to_owned(),
        "x".repeat(1024 * 1024 + 1),
        r#"{"roles":{"invalid":{"inherits":["missing"],"permissions":[]}},"assignments":{}}"#
            .to_owned(),
    ] {
        c.execute(
            "UPDATE resource_role_scopes SET policy=?2 WHERE bot_instance_id=?1",
            rusqlite::params![a.instance_id(), body],
        )
        .unwrap();
        assert!(roles
            .is_allowed(&request(a.instance_id(), "member", "help"))
            .is_err());
    }
    roles.reset_scope(a.instance_id(), "alice").unwrap();
    assert!(!roles
        .is_allowed(&request(a.instance_id(), "member", "help"))
        .unwrap());
}

#[test]
fn independent_connections_preserve_assignments_and_reject_a_concurrent_cycle() {
    let dir = Temp::new();
    let path = dir.0.join("roles.db");
    let bots = Arc::new(BotRegistry::in_memory());
    let bot = bots.register("f", "r", "a", "alice").unwrap();
    let first = RoleStore::open(&path, bots.clone()).unwrap();
    first.provision_scope(bot.instance_id(), "alice").unwrap();
    for name in ["a", "b"] {
        first
            .set_role(bot.instance_id(), "alice", name, allowed("help"))
            .unwrap();
    }
    let second = RoleStore::open(&path, bots.clone()).unwrap();
    let barrier = Arc::new(std::sync::Barrier::new(2));
    let mut joins = Vec::new();
    for (store, name, parent) in [(first, "a", "b"), (second, "b", "a")] {
        let barrier = barrier.clone();
        let id = bot.instance_id().to_owned();
        joins.push(std::thread::spawn(move || {
            barrier.wait();
            store.assign_role(&id, "alice", name, name).unwrap();
            store
                .set_role(
                    &id,
                    "alice",
                    name,
                    RoleDefinition {
                        inherits: vec![parent.into()],
                        permissions: vec![],
                    },
                )
                .is_ok()
        }));
    }
    let results: Vec<_> = joins.into_iter().map(|join| join.join().unwrap()).collect();
    assert_eq!(results.iter().filter(|success| **success).count(), 1);
    let recovered = RoleStore::open(&path, bots).unwrap();
    assert_eq!(
        recovered
            .policy(bot.instance_id(), "alice")
            .unwrap()
            .assignments
            .len(),
        2
    );
    for subject in ["a", "b"] {
        assert!(recovered
            .is_allowed(&request(bot.instance_id(), subject, "help"))
            .unwrap());
    }
    let rows = recovered
        .audit_history(bot.instance_id(), "alice", 0, 100)
        .unwrap();
    assert_eq!(rows.len(), 6);
    let c = rusqlite::Connection::open(&path).unwrap();
    for sql in [
        "DELETE FROM resource_role_audit",
        "UPDATE resource_role_audit SET actor_id='forged'",
        "INSERT OR REPLACE INTO resource_role_audit SELECT * FROM resource_role_audit",
    ] {
        assert!(c.execute(sql, []).is_err());
    }
    assert_eq!(
        recovered
            .audit_history(bot.instance_id(), "alice", 0, 100)
            .unwrap(),
        rows
    );
}

#[test]
fn role_schema_rollback_and_future_version_rejection_preserve_database() {
    let dir = Temp::new();
    let path = dir.0.join("roles.db");
    let bots = Arc::new(BotRegistry::in_memory());
    let c = rusqlite::Connection::open(&path).unwrap();
    c.execute_batch("CREATE VIEW resource_role_audit AS SELECT 1;")
        .unwrap();
    assert!(RoleStore::open(&path, bots.clone()).is_err());
    let tables: i64 = c
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(tables, 0);
    c.execute_batch("DROP VIEW resource_role_audit;").unwrap();
    drop(RoleStore::open(&path, bots.clone()).unwrap());
    c.execute_batch(
        "UPDATE matter_schema_versions SET version=99 WHERE component='resource_roles';",
    )
    .unwrap();
    assert!(RoleStore::open(&path, bots).is_err());
    let version: i64 = c
        .query_row(
            "SELECT version FROM matter_schema_versions WHERE component='resource_roles'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(version, 99);
}
