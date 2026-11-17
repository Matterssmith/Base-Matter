/*
 * Matter-Base — Persistent Command Audit
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use crate::{BotRegistry, CommandInvocation};
use anyhow::{anyhow, bail, Result};
use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use std::{path::Path, sync::Mutex};

/// Pending means no durable outcome is known. It never implies safe retry:
/// a handler may have executed before a process or audit-storage failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandAuditOutcome {
    Pending,
    Denied,
    MiddlewareRejected,
    Failed,
    Succeeded,
}

impl CommandAuditOutcome {
    fn decode(value: Option<&str>) -> Result<Self> {
        Ok(match value {
            None => Self::Pending,
            Some("denied") => Self::Denied,
            Some("middleware_rejected") => Self::MiddlewareRejected,
            Some("failed") => Self::Failed,
            Some("succeeded") => Self::Succeeded,
            _ => bail!("Invalid command audit outcome"),
        })
    }

    fn as_str(self) -> Result<&'static str> {
        Ok(match self {
            Self::Pending => bail!("Pending is not a terminal command outcome"),
            Self::Denied => "denied",
            Self::MiddlewareRejected => "middleware_rejected",
            Self::Failed => "failed",
            Self::Succeeded => "succeeded",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandAuditRecord {
    pub id: i64,
    pub bot_instance_id: String,
    pub actor_id: String,
    pub source_id: String,
    pub command: String,
    pub started_at: i64,
    pub finished_at: Option<i64>,
    pub outcome: CommandAuditOutcome,
}

/// Durable attempts and outcomes without arguments, message bodies, responses,
/// or callback diagnostics. Use a database controlled by the application.
/// SQL triggers prohibit modifying/deleting records through ordinary writes;
/// they do not defend against a database administrator rewriting the file.
pub struct CommandAuditStore {
    connection: Mutex<Connection>,
}

impl CommandAuditStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let mut connection = Connection::open(path)?;
        connection.execute_batch("PRAGMA foreign_keys=ON;")?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        tx.execute_batch(
            "CREATE TABLE IF NOT EXISTS matter_schema_versions (
            component TEXT PRIMARY KEY NOT NULL, version INTEGER NOT NULL);",
        )?;
        let version: Option<i64> = tx
            .query_row(
                "SELECT version FROM matter_schema_versions WHERE component='command_audit'",
                [],
                |r| r.get(0),
            )
            .optional()?;
        if version.is_some_and(|v| v != 1) {
            bail!("Unsupported command audit schema");
        }
        tx.execute_batch("CREATE TABLE IF NOT EXISTS command_audit_attempts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            bot_instance_id TEXT NOT NULL, actor_id TEXT NOT NULL,
            source_id TEXT NOT NULL, command TEXT NOT NULL, started_at INTEGER NOT NULL);
            CREATE INDEX IF NOT EXISTS command_audit_bot_id
                ON command_audit_attempts(bot_instance_id,id);
            CREATE TABLE IF NOT EXISTS command_audit_outcomes (
            attempt_id INTEGER PRIMARY KEY REFERENCES command_audit_attempts(id),
            outcome TEXT NOT NULL CHECK(outcome IN ('denied','middleware_rejected','failed','succeeded')),
            finished_at INTEGER NOT NULL);
            CREATE TRIGGER IF NOT EXISTS command_audit_attempt_no_update
                BEFORE UPDATE ON command_audit_attempts BEGIN SELECT RAISE(ABORT,'Immutable command audit'); END;
            CREATE TRIGGER IF NOT EXISTS command_audit_attempt_no_delete
                BEFORE DELETE ON command_audit_attempts BEGIN SELECT RAISE(ABORT,'Immutable command audit'); END;
            CREATE TRIGGER IF NOT EXISTS command_audit_attempt_no_replace
                BEFORE INSERT ON command_audit_attempts
                WHEN EXISTS(SELECT 1 FROM command_audit_attempts WHERE id=NEW.id)
                BEGIN SELECT RAISE(ABORT,'Immutable command audit'); END;
            CREATE TRIGGER IF NOT EXISTS command_audit_outcome_no_update
                BEFORE UPDATE ON command_audit_outcomes BEGIN SELECT RAISE(ABORT,'Immutable command audit'); END;
            CREATE TRIGGER IF NOT EXISTS command_audit_outcome_no_delete
                BEFORE DELETE ON command_audit_outcomes BEGIN SELECT RAISE(ABORT,'Immutable command audit'); END;
            CREATE TRIGGER IF NOT EXISTS command_audit_outcome_no_replace
                BEFORE INSERT ON command_audit_outcomes
                WHEN EXISTS(SELECT 1 FROM command_audit_outcomes WHERE attempt_id=NEW.attempt_id)
                BEGIN SELECT RAISE(ABORT,'Immutable command audit'); END;
            INSERT INTO matter_schema_versions VALUES('command_audit',1)
                ON CONFLICT(component) DO NOTHING;")?;
        tx.commit()?;
        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    /// Only the current bot owner may read its audit history. Actor identity
    /// must come from authenticated ingress. Authorization and audit storage
    /// are separate stores; revocation does not cancel an admitted read.
    /// Pagination is by increasing attempt ID, with 1..=1000 rows per call.
    pub fn history(
        &self,
        bot: &str,
        actor: &str,
        bots: &BotRegistry,
        after_id: i64,
        limit: usize,
    ) -> Result<Vec<CommandAuditRecord>> {
        if after_id < 0 || !(1..=1000).contains(&limit) {
            bail!("Invalid command audit page");
        }
        bots.authorize_owner(bot, actor)?;
        let c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Command audit lock poisoned"))?;
        let mut statement = c.prepare(
            "SELECT a.id,a.bot_instance_id,a.actor_id,a.source_id,a.command,
            a.started_at,o.finished_at,o.outcome FROM command_audit_attempts a
            LEFT JOIN command_audit_outcomes o ON o.attempt_id=a.id
            WHERE a.bot_instance_id=?1 AND a.id>?2 ORDER BY a.id LIMIT ?3",
        )?;
        let mut rows = statement.query(params![bot, after_id, limit as i64])?;
        let mut result = Vec::new();
        while let Some(row) = rows.next()? {
            let outcome: Option<String> = row.get(7)?;
            result.push(CommandAuditRecord {
                id: row.get(0)?,
                bot_instance_id: row.get(1)?,
                actor_id: row.get(2)?,
                source_id: row.get(3)?,
                command: row.get(4)?,
                started_at: row.get(5)?,
                finished_at: row.get(6)?,
                outcome: CommandAuditOutcome::decode(outcome.as_deref())?,
            });
        }
        Ok(result)
    }

    pub(crate) fn begin(&self, invocation: &CommandInvocation) -> Result<i64> {
        let c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Command audit lock poisoned"))?;
        c.execute("INSERT INTO command_audit_attempts(bot_instance_id,actor_id,source_id,command,started_at)
            VALUES(?1,?2,?3,?4,?5)", params![invocation.bot_instance_id,invocation.actor_id,
            invocation.source_id,invocation.command,time::OffsetDateTime::now_utc().unix_timestamp()])?;
        Ok(c.last_insert_rowid())
    }

    pub(crate) fn finish(&self, id: i64, outcome: CommandAuditOutcome) -> Result<()> {
        let c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Command audit lock poisoned"))?;
        c.execute(
            "INSERT INTO command_audit_outcomes(attempt_id,outcome,finished_at) VALUES(?1,?2,?3)",
            params![
                id,
                outcome.as_str()?,
                time::OffsetDateTime::now_utc().unix_timestamp()
            ],
        )?;
        Ok(())
    }
}
