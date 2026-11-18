/*
 * Matter-Base — Scoped Resource Authorization
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use crate::{BotRegistry, CommandAuthorizer, CommandInvocation};
use anyhow::{anyhow, bail, Result};
use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::{Arc, Mutex},
};

const MAX_POLICY_BYTES: usize = 1024 * 1024;
const MAX_ROLES: usize = 64;
const MAX_SUBJECTS: usize = 1024;
const MAX_PERMISSIONS: usize = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionEffect {
    Allow,
    Deny,
}

/// Each selector is an exact identifier or the whole-string wildcard `*`.
/// Wildcards never extend beyond the enclosing bot instance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcePermission {
    pub resource_type: String,
    pub resource_id: String,
    pub action: String,
    pub effect: PermissionEffect,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoleDefinition {
    pub inherits: Vec<String>,
    pub permissions: Vec<ResourcePermission>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RolePolicy {
    pub roles: BTreeMap<String, RoleDefinition>,
    pub assignments: BTreeMap<String, BTreeSet<String>>,
}

pub struct ResourceRequest<'a> {
    pub bot_instance_id: &'a str,
    pub subject_id: &'a str,
    pub resource_type: &'a str,
    pub resource_id: &'a str,
    pub action: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleAuditRecord {
    pub id: i64,
    pub actor_id: String,
    pub action: String,
    pub target_id: String,
    pub detail: String,
    pub created_at: i64,
}

/// Persistent bot-scoped roles, deny-first resource policy and atomic audit.
/// The application authenticates actor/subject IDs. The injected bot registry
/// is a trusted, fixed dependency; only its current owners administer policy.
/// Policy is pinned to an owner ID: requests fail closed while that ID no longer
/// owns the bot. A different owner must reset the scope. No global-owner bypass.
pub struct RoleStore {
    connection: Mutex<Connection>,
    bots: Arc<BotRegistry>,
}

impl RoleStore {
    pub fn open(path: impl AsRef<Path>, bots: Arc<BotRegistry>) -> Result<Self> {
        let mut c = Connection::open(path)?;
        let tx = c.transaction_with_behavior(TransactionBehavior::Immediate)?;
        tx.execute_batch(
            "CREATE TABLE IF NOT EXISTS matter_schema_versions (
            component TEXT PRIMARY KEY NOT NULL, version INTEGER NOT NULL);",
        )?;
        let version: Option<i64> = tx
            .query_row(
                "SELECT version FROM matter_schema_versions WHERE component='resource_roles'",
                [],
                |r| r.get(0),
            )
            .optional()?;
        if version.is_some_and(|v| v != 1) {
            bail!("Unsupported role schema");
        }
        tx.execute_batch("CREATE TABLE IF NOT EXISTS resource_role_scopes (
            bot_instance_id TEXT PRIMARY KEY NOT NULL, owner_id TEXT NOT NULL, policy TEXT NOT NULL);
            CREATE TABLE IF NOT EXISTS resource_role_audit (
            id INTEGER PRIMARY KEY AUTOINCREMENT, bot_instance_id TEXT NOT NULL,
            actor_id TEXT NOT NULL, action TEXT NOT NULL, target_id TEXT NOT NULL,
            detail TEXT NOT NULL, created_at INTEGER NOT NULL);
            CREATE INDEX IF NOT EXISTS resource_role_audit_bot ON resource_role_audit(bot_instance_id,id);
            CREATE TRIGGER IF NOT EXISTS role_audit_no_update BEFORE UPDATE ON resource_role_audit
                BEGIN SELECT RAISE(ABORT,'Immutable role audit'); END;
            CREATE TRIGGER IF NOT EXISTS role_audit_no_delete BEFORE DELETE ON resource_role_audit
                BEGIN SELECT RAISE(ABORT,'Immutable role audit'); END;
            CREATE TRIGGER IF NOT EXISTS role_audit_no_replace BEFORE INSERT ON resource_role_audit
                WHEN EXISTS(SELECT 1 FROM resource_role_audit WHERE id=NEW.id)
                BEGIN SELECT RAISE(ABORT,'Immutable role audit'); END;
            INSERT INTO matter_schema_versions VALUES('resource_roles',1) ON CONFLICT(component) DO NOTHING;")?;
        tx.commit()?;
        Ok(Self {
            connection: Mutex::new(c),
            bots,
        })
    }

    /// Creates an empty, deny-by-default scope. Existing scopes are not replaced.
    pub fn provision_scope(&self, bot: &str, actor: &str) -> Result<()> {
        self.bots.authorize_owner(bot, actor)?;
        let mut c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Role store lock poisoned"))?;
        let tx = c.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let policy = encode(&RolePolicy::default())?;
        tx.execute(
            "INSERT INTO resource_role_scopes VALUES(?1,?2,?3)",
            params![bot, actor, policy],
        )?;
        audit(&tx, bot, actor, "provision", bot, "{}")?;
        tx.commit()?;
        Ok(())
    }

    /// Explicitly discards roles and assignments, pins policy to the current
    /// bot owner, and retains prior audit. Also recovers a corrupt policy body.
    pub fn reset_scope(&self, bot: &str, actor: &str) -> Result<()> {
        self.bots.authorize_owner(bot, actor)?;
        let mut c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Role store lock poisoned"))?;
        let tx = c.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let old_owner: String = tx.query_row(
            "SELECT owner_id FROM resource_role_scopes WHERE bot_instance_id=?1",
            [bot],
            |r| r.get(0),
        )?;
        tx.execute(
            "UPDATE resource_role_scopes SET owner_id=?2,policy=?3 WHERE bot_instance_id=?1",
            params![bot, actor, encode(&RolePolicy::default())?],
        )?;
        audit(
            &tx,
            bot,
            actor,
            "reset",
            bot,
            &serde_json::to_string(&old_owner)?,
        )?;
        tx.commit()?;
        Ok(())
    }

    pub fn set_role(&self, bot: &str, actor: &str, name: &str, role: RoleDefinition) -> Result<()> {
        validate_name(name)?;
        validate_role(&role)?;
        let detail = serde_json::to_string(&role)?;
        self.mutate(bot, actor, "set_role", name, &detail, |policy| {
            policy.roles.insert(name.to_owned(), role);
            Ok(())
        })
    }

    /// Removes direct assignments too; inherited references must be removed
    /// explicitly first or validation rejects the entire transaction.
    pub fn remove_role(&self, bot: &str, actor: &str, name: &str) -> Result<()> {
        validate_name(name)?;
        self.mutate(bot, actor, "remove_role", name, "{}", |policy| {
            if policy.roles.remove(name).is_none() {
                bail!("Role unavailable");
            }
            for roles in policy.assignments.values_mut() {
                roles.remove(name);
            }
            policy.assignments.retain(|_, roles| !roles.is_empty());
            Ok(())
        })
    }

    pub fn assign_role(&self, bot: &str, actor: &str, subject: &str, role: &str) -> Result<()> {
        validate_id(subject)?;
        validate_name(role)?;
        self.mutate(
            bot,
            actor,
            "assign_role",
            subject,
            &serde_json::to_string(role)?,
            |policy| {
                if !policy.roles.contains_key(role) {
                    bail!("Role unavailable");
                }
                policy
                    .assignments
                    .entry(subject.to_owned())
                    .or_default()
                    .insert(role.to_owned());
                Ok(())
            },
        )
    }

    pub fn revoke_role(&self, bot: &str, actor: &str, subject: &str, role: &str) -> Result<()> {
        validate_id(subject)?;
        validate_name(role)?;
        self.mutate(
            bot,
            actor,
            "revoke_role",
            subject,
            &serde_json::to_string(role)?,
            |policy| {
                if let Some(roles) = policy.assignments.get_mut(subject) {
                    roles.remove(role);
                }
                policy.assignments.retain(|_, roles| !roles.is_empty());
                Ok(())
            },
        )
    }

    fn mutate(
        &self,
        bot: &str,
        actor: &str,
        action: &str,
        target: &str,
        detail: &str,
        mutation: impl FnOnce(&mut RolePolicy) -> Result<()>,
    ) -> Result<()> {
        self.bots.authorize_owner(bot, actor)?;
        let mut c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Role store lock poisoned"))?;
        let tx = c.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let (owner, mut policy) = load(&tx, bot)?;
        if owner != actor {
            bail!("Role scope requires owner reset");
        }
        mutation(&mut policy)?;
        let json = encode(&policy)?;
        tx.execute(
            "UPDATE resource_role_scopes SET policy=?2 WHERE bot_instance_id=?1",
            params![bot, json],
        )?;
        audit(&tx, bot, actor, action, target, detail)?;
        tx.commit()?;
        Ok(())
    }

    pub fn policy(&self, bot: &str, actor: &str) -> Result<RolePolicy> {
        self.bots.authorize_owner(bot, actor)?;
        let c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Role store lock poisoned"))?;
        let (owner, policy) = load(&c, bot)?;
        if owner != actor {
            bail!("Role scope requires owner reset");
        }
        Ok(policy)
    }

    /// A matching deny beats every matching allow, including inherited grants.
    /// No matching allow means denial. Ownership/role names imply no privilege.
    /// This is an admission snapshot; later revocation cannot undo effects.
    pub fn is_allowed(&self, request: &ResourceRequest<'_>) -> Result<bool> {
        for value in [
            request.bot_instance_id,
            request.subject_id,
            request.resource_type,
            request.resource_id,
            request.action,
        ] {
            validate_id(value)?;
            if value == "*" {
                bail!("A resource request must be concrete");
            }
        }
        let (owner, policy) = {
            let c = self
                .connection
                .lock()
                .map_err(|_| anyhow!("Role store lock poisoned"))?;
            load(&c, request.bot_instance_id)?
        };
        self.bots.authorize_owner(request.bot_instance_id, &owner)?;
        let mut pending: Vec<_> = policy
            .assignments
            .get(request.subject_id)
            .into_iter()
            .flatten()
            .map(String::as_str)
            .collect();
        let mut visited = BTreeSet::new();
        let mut allowed = false;
        while let Some(name) = pending.pop() {
            if !visited.insert(name) {
                continue;
            }
            let role = policy
                .roles
                .get(name)
                .ok_or_else(|| anyhow!("Role unavailable"))?;
            pending.extend(role.inherits.iter().map(String::as_str));
            for permission in &role.permissions {
                if matches(&permission.resource_type, request.resource_type)
                    && matches(&permission.resource_id, request.resource_id)
                    && matches(&permission.action, request.action)
                {
                    if permission.effect == PermissionEffect::Deny {
                        return Ok(false);
                    }
                    allowed = true;
                }
            }
        }
        Ok(allowed)
    }

    /// Current bot owners can inspect retained policy audit even before reset.
    /// Pages contain 1..=100 entries; details never contain message payloads.
    pub fn audit_history(
        &self,
        bot: &str,
        actor: &str,
        after_id: i64,
        limit: usize,
    ) -> Result<Vec<RoleAuditRecord>> {
        if after_id < 0 || !(1..=100).contains(&limit) {
            bail!("Invalid role audit page");
        }
        self.bots.authorize_owner(bot, actor)?;
        let c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Role store lock poisoned"))?;
        let mut stmt = c.prepare(
            "SELECT id,actor_id,action,target_id,detail,created_at
            FROM resource_role_audit WHERE bot_instance_id=?1 AND id>?2 ORDER BY id LIMIT ?3",
        )?;
        let rows = stmt.query_map(params![bot, after_id, limit as i64], |r| {
            Ok(RoleAuditRecord {
                id: r.get(0)?,
                actor_id: r.get(1)?,
                action: r.get(2)?,
                target_id: r.get(3)?,
                detail: r.get(4)?,
                created_at: r.get(5)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
    }
}

pub struct RoleCommandAuthorizer {
    roles: Arc<RoleStore>,
}
impl RoleCommandAuthorizer {
    pub fn new(roles: Arc<RoleStore>) -> Self {
        Self { roles }
    }
}
impl CommandAuthorizer for RoleCommandAuthorizer {
    fn authorize(&self, invocation: &CommandInvocation) -> Result<()> {
        if !self.roles.is_allowed(&ResourceRequest {
            bot_instance_id: &invocation.bot_instance_id,
            subject_id: &invocation.actor_id,
            resource_type: "command",
            resource_id: &invocation.command,
            action: "execute",
        })? {
            bail!("Resource authorization denied");
        }
        Ok(())
    }
}

fn matches(selector: &str, value: &str) -> bool {
    selector == "*" || selector == value
}
fn validate_id(value: &str) -> Result<()> {
    if value.trim().is_empty() || value.len() > 128 || value.chars().any(char::is_control) {
        bail!("Invalid policy identifier");
    }
    Ok(())
}
fn validate_name(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > 64
        || !value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, b'_' | b'-' | b'.'))
    {
        bail!("Invalid role name");
    }
    Ok(())
}
fn validate_role(role: &RoleDefinition) -> Result<()> {
    if role.inherits.len() > 8 || role.permissions.len() > MAX_PERMISSIONS {
        bail!("Role capacity exceeded");
    }
    let mut parents = BTreeSet::new();
    for parent in &role.inherits {
        validate_name(parent)?;
        if !parents.insert(parent) {
            bail!("Duplicate inherited role");
        }
    }
    for permission in &role.permissions {
        for value in [
            &permission.resource_type,
            &permission.resource_id,
            &permission.action,
        ] {
            validate_id(value)?;
        }
    }
    Ok(())
}
fn validate_policy(policy: &RolePolicy) -> Result<()> {
    if policy.roles.len() > MAX_ROLES || policy.assignments.len() > MAX_SUBJECTS {
        bail!("Policy capacity exceeded");
    }
    for (name, role) in &policy.roles {
        validate_name(name)?;
        validate_role(role)?;
    }
    let mut depths = BTreeMap::new();
    for name in policy.roles.keys() {
        depth(name, &policy.roles, &mut BTreeSet::new(), &mut depths)?;
    }
    for (subject, roles) in &policy.assignments {
        validate_id(subject)?;
        if roles.len() > 16
            || roles.is_empty()
            || roles.iter().any(|r| !policy.roles.contains_key(r))
        {
            bail!("Invalid role assignments");
        }
    }
    Ok(())
}
fn depth(
    name: &str,
    roles: &BTreeMap<String, RoleDefinition>,
    visiting: &mut BTreeSet<String>,
    depths: &mut BTreeMap<String, usize>,
) -> Result<usize> {
    if let Some(depth) = depths.get(name) {
        return Ok(*depth);
    }
    if !visiting.insert(name.to_owned()) {
        bail!("Role inheritance cycle");
    }
    let role = roles
        .get(name)
        .ok_or_else(|| anyhow!("Inherited role unavailable"))?;
    let mut max_depth = 1;
    for parent in &role.inherits {
        max_depth = max_depth.max(1 + depth(parent, roles, visiting, depths)?);
    }
    if max_depth > 16 {
        bail!("Role inheritance depth exceeded");
    }
    visiting.remove(name);
    depths.insert(name.to_owned(), max_depth);
    Ok(max_depth)
}
fn encode(policy: &RolePolicy) -> Result<String> {
    validate_policy(policy)?;
    let json = serde_json::to_string(policy)?;
    if json.len() > MAX_POLICY_BYTES {
        bail!("Policy size exceeded");
    }
    Ok(json)
}
fn load(c: &Connection, bot: &str) -> Result<(String, RolePolicy)> {
    let (owner, json): (String,Option<String>) = c.query_row("SELECT owner_id,
        CASE WHEN length(CAST(policy AS BLOB))<=?2 THEN policy END FROM resource_role_scopes WHERE bot_instance_id=?1",
        params![bot,MAX_POLICY_BYTES as i64], |r| Ok((r.get(0)?,r.get(1)?)))?;
    let json = json.ok_or_else(|| anyhow!("Policy size exceeded"))?;
    let policy: RolePolicy =
        serde_json::from_str(&json).map_err(|_| anyhow!("Invalid stored role policy"))?;
    validate_policy(&policy)?;
    Ok((owner, policy))
}
fn audit(
    c: &Connection,
    bot: &str,
    actor: &str,
    action: &str,
    target: &str,
    detail: &str,
) -> Result<()> {
    if detail.len() > 64 * 1024 {
        bail!("Role audit detail size exceeded");
    }
    c.execute("INSERT INTO resource_role_audit(bot_instance_id,actor_id,action,target_id,detail,created_at)
        VALUES(?1,?2,?3,?4,?5,?6)", params![bot,actor,action,target,detail,time::OffsetDateTime::now_utc().unix_timestamp()])?;
    Ok(())
}
