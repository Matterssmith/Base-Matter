/*
 * Matter-Base — Persistent scoped role authorization.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use crate::commands::{CommandAuthorizer, CommandInvocation};
use anyhow::{anyhow, bail, Result};
use rusqlite::{params, Connection, OptionalExtension, Transaction, TransactionBehavior};
use std::collections::HashSet;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const SCHEMA_VERSION: i64 = 1;
const MAX_IDENTIFIER_BYTES: usize = 128;
const MAX_ROLES_PER_SCOPE: i64 = 128;
const MAX_GRANTS_PER_USER: i64 = 32;
const MAX_RULES_PER_ROLE: i64 = 256;
const MAX_ROLE_DEPTH: usize = 16;
const MAX_AUDIT_PAGE: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleEffect {
    Allow,
    Deny,
}

impl RuleEffect {
    fn value(self) -> i64 {
        match self {
            Self::Allow => 1,
            Self::Deny => -1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleAuditRecord {
    pub id: i64,
    pub actor_id: String,
    pub action: String,
    pub target_id: Option<String>,
    pub role_id: Option<String>,
    pub created_at: i64,
}

/// SQLite-backed policy for an independently owned scope (usually a bot).
///
/// The application must authenticate all actor IDs and only call
/// provision_scope after a trusted ownership decision. No arbitrary network
/// request may provision a scope. A scope owner administers roles but does not
/// receive an implicit command-execution bypass. An explicit Deny always wins.
/// This store never manages socket power or platform credentials.
pub struct RoleStore {
    connection: Mutex<Connection>,
}

impl RoleStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Self::from_connection(Connection::open(path)?)
    }

    pub fn in_memory() -> Result<Self> {
        Self::from_connection(Connection::open_in_memory()?)
    }

    fn from_connection(mut connection: Connection) -> Result<Self> {
        connection.busy_timeout(Duration::from_secs(5))?;
        connection.execute_batch("PRAGMA foreign_keys=ON;")?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        tx.execute_batch(
            "CREATE TABLE IF NOT EXISTS matter_schema_versions (
                 component TEXT PRIMARY KEY NOT NULL,
                 version INTEGER NOT NULL
             );",
        )?;
        let version: Option<i64> = tx
            .query_row(
                "SELECT version FROM matter_schema_versions WHERE component='role_authorization'",
                [],
                |row| row.get(0),
            )
            .optional()?;
        if version.is_some_and(|value| value != SCHEMA_VERSION) {
            bail!("Unsupported role authorization schema");
        }
        tx.execute_batch(
            "CREATE TABLE IF NOT EXISTS role_scopes (
                 scope_id TEXT PRIMARY KEY NOT NULL,
                 owner_id TEXT NOT NULL
             );
             CREATE TABLE IF NOT EXISTS role_definitions (
                 scope_id TEXT NOT NULL REFERENCES role_scopes(scope_id),
                 role_id TEXT NOT NULL,
                 parent_role TEXT,
                 PRIMARY KEY(scope_id, role_id),
                 FOREIGN KEY(scope_id, parent_role)
                     REFERENCES role_definitions(scope_id, role_id)
             );
             CREATE TABLE IF NOT EXISTS role_grants (
                 scope_id TEXT NOT NULL,
                 user_id TEXT NOT NULL,
                 role_id TEXT NOT NULL,
                 PRIMARY KEY(scope_id, user_id, role_id),
                 FOREIGN KEY(scope_id, role_id)
                     REFERENCES role_definitions(scope_id, role_id)
             );
             CREATE TABLE IF NOT EXISTS role_rules (
                 scope_id TEXT NOT NULL,
                 role_id TEXT NOT NULL,
                 resource TEXT NOT NULL,
                 action TEXT NOT NULL,
                 effect INTEGER NOT NULL CHECK(effect IN (-1,1)),
                 PRIMARY KEY(scope_id, role_id, resource, action),
                 FOREIGN KEY(scope_id, role_id)
                     REFERENCES role_definitions(scope_id, role_id)
             );
             CREATE TABLE IF NOT EXISTS role_audit (
                 id INTEGER PRIMARY KEY AUTOINCREMENT,
                 scope_id TEXT NOT NULL,
                 actor_id TEXT NOT NULL,
                 action TEXT NOT NULL,
                 target_id TEXT,
                 role_id TEXT,
                 created_at INTEGER NOT NULL
             );
             CREATE TRIGGER IF NOT EXISTS role_audit_immutable_update
                 BEFORE UPDATE ON role_audit
                 BEGIN SELECT RAISE(ABORT,'Role audit is immutable'); END;
             CREATE TRIGGER IF NOT EXISTS role_audit_immutable_delete
                 BEFORE DELETE ON role_audit
                 BEGIN SELECT RAISE(ABORT,'Role audit is immutable'); END;
             CREATE INDEX IF NOT EXISTS idx_role_grants_user
                 ON role_grants(scope_id, user_id);
             CREATE INDEX IF NOT EXISTS idx_role_audit_scope
                 ON role_audit(scope_id, id);
             INSERT OR IGNORE INTO matter_schema_versions(component,version)
                 VALUES('role_authorization',1);",
        )?;
        tx.commit()?;
        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    /// Trusted provisioning only. Existing ownership cannot be replaced.
    pub fn provision_scope(&self, scope: &str, owner: &str) -> Result<()> {
        validate_id(scope)?;
        validate_id(owner)?;
        let mut connection = self.lock()?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let existing: Option<String> = tx
            .query_row(
                "SELECT owner_id FROM role_scopes WHERE scope_id=?1",
                params![scope],
                |row| row.get(0),
            )
            .optional()?;
        match existing {
            Some(current) if current != owner => bail!("Scope owner mismatch"),
            Some(_) => return Ok(()),
            None => {}
        }
        tx.execute(
            "INSERT INTO role_scopes(scope_id,owner_id) VALUES(?1,?2)",
            params![scope, owner],
        )?;
        audit(&tx, scope, owner, "provision", None, None)?;
        tx.commit()?;
        Ok(())
    }

    /// Defines an immutable parent relationship; roles cannot form cycles.
    pub fn create_role(
        &self,
        scope: &str,
        actor: &str,
        role: &str,
        parent: Option<&str>,
    ) -> Result<()> {
        validate_id(role)?;
        if let Some(parent) = parent {
            validate_id(parent)?;
            if parent == role {
                bail!("Role cannot inherit itself");
            }
        }
        let mut connection = self.lock()?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        require_owner(&tx, scope, actor)?;
        let count: i64 = tx.query_row(
            "SELECT COUNT(*) FROM role_definitions WHERE scope_id=?1",
            params![scope],
            |row| row.get(0),
        )?;
        if count >= MAX_ROLES_PER_SCOPE {
            bail!("Role limit reached");
        }
        if let Some(parent) = parent {
            let chain = role_chain(&tx, scope, parent)?;
            if chain.len() >= MAX_ROLE_DEPTH {
                bail!("Role inheritance depth exceeded");
            }
        }
        tx.execute(
            "INSERT INTO role_definitions(scope_id,role_id,parent_role)
             VALUES(?1,?2,?3)",
            params![scope, role, parent],
        )?;
        audit(&tx, scope, actor, "create_role", None, Some(role))?;
        tx.commit()?;
        Ok(())
    }

    pub fn grant(&self, scope: &str, actor: &str, subject: &str, role: &str) -> Result<bool> {
        validate_id(subject)?;
        validate_id(role)?;
        let mut connection = self.lock()?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        require_owner(&tx, scope, actor)?;
        let current: i64 = tx.query_row(
            "SELECT COUNT(*) FROM role_grants WHERE scope_id=?1 AND user_id=?2",
            params![scope, subject],
            |row| row.get(0),
        )?;
        let exists: bool = tx.query_row(
            "SELECT EXISTS(SELECT 1 FROM role_grants
               WHERE scope_id=?1 AND user_id=?2 AND role_id=?3)",
            params![scope, subject, role],
            |row| row.get(0),
        )?;
        if exists {
            return Ok(false);
        }
        if current >= MAX_GRANTS_PER_USER {
            bail!("Membership limit reached");
        }
        tx.execute(
            "INSERT INTO role_grants(scope_id,user_id,role_id) VALUES(?1,?2,?3)",
            params![scope, subject, role],
        )?;
        audit(&tx, scope, actor, "grant", Some(subject), Some(role))?;
        tx.commit()?;
        Ok(true)
    }

    pub fn revoke(&self, scope: &str, actor: &str, subject: &str, role: &str) -> Result<bool> {
        validate_id(subject)?;
        validate_id(role)?;
        let mut connection = self.lock()?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        require_owner(&tx, scope, actor)?;
        let removed = tx.execute(
            "DELETE FROM role_grants WHERE scope_id=?1 AND user_id=?2 AND role_id=?3",
            params![scope, subject, role],
        )?;
        if removed != 0 {
            audit(&tx, scope, actor, "revoke", Some(subject), Some(role))?;
        }
        tx.commit()?;
        Ok(removed != 0)
    }

    pub fn set_rule(
        &self,
        scope: &str,
        actor: &str,
        role: &str,
        resource: &str,
        action: &str,
        effect: RuleEffect,
    ) -> Result<()> {
        validate_id(role)?;
        validate_rule_key(resource)?;
        validate_rule_key(action)?;
        let mut connection = self.lock()?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        require_owner(&tx, scope, actor)?;
        let existing: Option<i64> = tx
            .query_row(
                "SELECT effect FROM role_rules WHERE scope_id=?1 AND role_id=?2
                 AND resource=?3 AND action=?4",
                params![scope, role, resource, action],
                |row| row.get(0),
            )
            .optional()?;
        if existing == Some(effect.value()) {
            return Ok(());
        }
        if existing.is_none() {
            let count: i64 = tx.query_row(
                "SELECT COUNT(*) FROM role_rules WHERE scope_id=?1 AND role_id=?2",
                params![scope, role],
                |row| row.get(0),
            )?;
            if count >= MAX_RULES_PER_ROLE {
                bail!("Role rule limit reached");
            }
        }
        tx.execute(
            "INSERT INTO role_rules(scope_id,role_id,resource,action,effect)
             VALUES(?1,?2,?3,?4,?5)
             ON CONFLICT(scope_id,role_id,resource,action)
             DO UPDATE SET effect=excluded.effect",
            params![scope, role, resource, action, effect.value()],
        )?;
        audit(&tx, scope, actor, "set_rule", Some(resource), Some(role))?;
        tx.commit()?;
        Ok(())
    }

    /// Default deny, including unknown scopes and subjects.
    /// All grants, inheritance and rules are read from one SQL snapshot.
    pub fn allowed(
        &self,
        scope: &str,
        subject: &str,
        resource: &str,
        action: &str,
    ) -> Result<bool> {
        validate_id(scope)?;
        validate_id(subject)?;
        validate_rule_key(resource)?;
        validate_rule_key(action)?;
        let mut connection = self.lock()?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Deferred)?;
        let mut statement =
            tx.prepare("SELECT role_id FROM role_grants WHERE scope_id=?1 AND user_id=?2")?;
        let roles = statement.query_map(params![scope, subject], |row| row.get::<_, String>(0))?;
        let assigned = roles.collect::<rusqlite::Result<Vec<_>>>()?;
        drop(statement);
        if assigned.len() > MAX_GRANTS_PER_USER as usize {
            bail!("Corrupt role assignment count");
        }
        let mut visited = HashSet::new();
        for role in assigned {
            for ancestor in role_chain(&tx, scope, &role)? {
                visited.insert(ancestor);
            }
        }
        let mut allow = false;
        for role in visited {
            let mut stmt = tx.prepare(
                "SELECT effect FROM role_rules
                 WHERE scope_id=?1 AND role_id=?2
                   AND resource IN (?3,'*') AND action IN (?4,'*')",
            )?;
            let effects = stmt.query_map(params![scope, role, resource, action], |row| {
                row.get::<_, i64>(0)
            })?;
            for effect in effects {
                match effect? {
                    -1 => return Ok(false),
                    1 => allow = true,
                    _ => bail!("Corrupt role rule"),
                }
            }
        }
        Ok(allow)
    }

    pub fn audit_history(
        &self,
        scope: &str,
        actor: &str,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<RoleAuditRecord>> {
        if limit == 0 || limit > MAX_AUDIT_PAGE {
            bail!("Invalid audit page size");
        }
        let limit = i64::try_from(limit)?;
        let offset = i64::try_from(offset)?;
        let connection = self.lock()?;
        require_owner(&connection, scope, actor)?;
        let mut statement = connection.prepare(
            "SELECT id,actor_id,action,target_id,role_id,created_at
             FROM role_audit WHERE scope_id=?1 ORDER BY id DESC LIMIT ?2 OFFSET ?3",
        )?;
        let rows = statement.query_map(params![scope, limit, offset], |row| {
            Ok(RoleAuditRecord {
                id: row.get(0)?,
                actor_id: row.get(1)?,
                action: row.get(2)?,
                target_id: row.get(3)?,
                role_id: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, Connection>> {
        self.connection
            .lock()
            .map_err(|_| anyhow!("Role store lock poisoned"))
    }
}

fn validate_id(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > MAX_IDENTIFIER_BYTES
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._:-".contains(&byte))
    {
        bail!("Invalid authorization identifier");
    }
    Ok(())
}

fn validate_rule_key(value: &str) -> Result<()> {
    if value == "*" {
        return Ok(());
    }
    validate_id(value)
}

fn require_owner(connection: &Connection, scope: &str, actor: &str) -> Result<()> {
    validate_id(scope)?;
    validate_id(actor)?;
    let owner: Option<String> = connection
        .query_row(
            "SELECT owner_id FROM role_scopes WHERE scope_id=?1",
            params![scope],
            |row| row.get(0),
        )
        .optional()?;
    if owner.as_deref() != Some(actor) {
        bail!("Role administration denied");
    }
    Ok(())
}

fn role_chain(tx: &Transaction<'_>, scope: &str, role: &str) -> Result<Vec<String>> {
    let mut result = Vec::new();
    let mut current = Some(role.to_owned());
    let mut visited = HashSet::new();
    while let Some(name) = current {
        if result.len() == MAX_ROLE_DEPTH || !visited.insert(name.clone()) {
            bail!("Invalid role inheritance");
        }
        let parent: Option<Option<String>> = tx
            .query_row(
                "SELECT parent_role FROM role_definitions WHERE scope_id=?1 AND role_id=?2",
                params![scope, name],
                |row| row.get(0),
            )
            .optional()?;
        current = parent.ok_or_else(|| anyhow!("Missing inherited role"))?;
        result.push(name);
    }
    Ok(result)
}

fn audit(
    tx: &Transaction<'_>,
    scope: &str,
    actor: &str,
    action: &str,
    target: Option<&str>,
    role: Option<&str>,
) -> Result<()> {
    tx.execute(
        "INSERT INTO role_audit(
             scope_id,actor_id,action,target_id,role_id,created_at
         ) VALUES(?1,?2,?3,?4,?5,?6)",
        params![
            scope,
            actor,
            action,
            target,
            role,
            time::OffsetDateTime::now_utc().unix_timestamp()
        ],
    )?;
    Ok(())
}

/// Policy adapter for CommandRouter. The application must authenticate
/// invocation.actor_id before dispatch and bind it to the current socket.
/// Canonical commands are evaluated as command:<name> / execute.
#[derive(Clone)]
pub struct RoleCommandAuthorizer {
    store: Arc<RoleStore>,
}

impl RoleCommandAuthorizer {
    pub fn new(store: Arc<RoleStore>) -> Self {
        Self { store }
    }
}

impl CommandAuthorizer for RoleCommandAuthorizer {
    fn authorize(&self, invocation: &CommandInvocation) -> Result<()> {
        let resource = format!("command:{}", invocation.command);
        if self
            .store
            .allowed(
                &invocation.bot_instance_id,
                &invocation.actor_id,
                &resource,
                "execute",
            )
            .unwrap_or(false)
        {
            Ok(())
        } else {
            bail!("Command authorization denied")
        }
    }
}
