/*
 * Matter-Base — Persistent Socket Activation
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{anyhow, bail, Result};
use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use std::{path::Path, sync::Mutex};

/// Policy precedence after the operational power gate: group, socket, global,
/// then deny. Connection state is deliberately not an activation input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationSource {
    PoweredOff,
    Group,
    Socket,
    Global,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivationDecision {
    pub enabled: bool,
    pub source: ActivationSource,
}

/// Persistent policy storage, not a network socket manager. The trusted
/// application provisions socket IDs/owners after authenticating their creation.
/// Every mutation and audit read checks the stored owner in the same transaction.
/// No global administrator bypass is provided.
pub struct SocketActivationStore {
    connection: Mutex<Connection>,
}

impl SocketActivationStore {
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
                "SELECT version FROM matter_schema_versions WHERE component='socket_activation'",
                [],
                |row| row.get(0),
            )
            .optional()?;
        if version.is_some_and(|v| v != 1) {
            bail!("Unsupported socket activation schema");
        }
        tx.execute_batch(
            "CREATE TABLE IF NOT EXISTS activation_sockets (
            socket_id TEXT PRIMARY KEY NOT NULL, owner_id TEXT NOT NULL,
            powered INTEGER NOT NULL DEFAULT 0 CHECK(powered IN (0,1)),
            policy INTEGER CHECK(policy IN (0,1)));
            CREATE TABLE IF NOT EXISTS activation_groups (
            socket_id TEXT NOT NULL REFERENCES activation_sockets(socket_id),
            group_id TEXT NOT NULL, policy INTEGER NOT NULL CHECK(policy IN (0,1)),
            PRIMARY KEY(socket_id,group_id));
            CREATE TABLE IF NOT EXISTS activation_audit (
            id INTEGER PRIMARY KEY AUTOINCREMENT, socket_id TEXT NOT NULL,
            actor_id TEXT NOT NULL, action TEXT NOT NULL, group_id TEXT,
            value INTEGER, created_at INTEGER NOT NULL);
            INSERT OR IGNORE INTO matter_schema_versions VALUES('socket_activation',1);",
        )?;
        tx.commit()?;
        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    /// Trusted provisioning boundary. Does not authenticate arbitrary actor
    /// strings or establish ownership of a platform connection.
    pub fn provision(&self, socket: &str, authenticated_owner: &str) -> Result<()> {
        validate(socket)?;
        validate(authenticated_owner)?;
        let mut c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Activation lock poisoned"))?;
        let tx = c.transaction_with_behavior(TransactionBehavior::Immediate)?;
        tx.execute(
            "INSERT INTO activation_sockets(socket_id,owner_id) VALUES(?1,?2)",
            params![socket, authenticated_owner],
        )?;
        audit(
            &tx,
            socket,
            authenticated_owner,
            "provision",
            None,
            Some(false),
        )?;
        tx.commit()?;
        Ok(())
    }

    pub fn set_power(&self, socket: &str, actor: &str, powered: bool) -> Result<()> {
        self.mutate(socket, actor, "power", None, Some(powered))
    }

    /// None removes the socket override and restores global fallback.
    pub fn set_socket_policy(&self, socket: &str, actor: &str, value: Option<bool>) -> Result<()> {
        self.mutate(socket, actor, "socket_policy", None, value)
    }

    /// None removes the group override and restores socket/global fallback.
    pub fn set_group_policy(
        &self,
        socket: &str,
        actor: &str,
        group: &str,
        value: Option<bool>,
    ) -> Result<()> {
        validate(group)?;
        self.mutate(socket, actor, "group_policy", Some(group), value)
    }

    fn mutate(
        &self,
        socket: &str,
        actor: &str,
        action: &str,
        group: Option<&str>,
        value: Option<bool>,
    ) -> Result<()> {
        validate(socket)?;
        validate(actor)?;
        let mut c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Activation lock poisoned"))?;
        let tx = c.transaction_with_behavior(TransactionBehavior::Immediate)?;
        require_owner(&tx, socket, actor)?;
        match action {
            "power" => {
                tx.execute(
                    "UPDATE activation_sockets SET powered=?2 WHERE socket_id=?1",
                    params![socket, value],
                )?;
            }
            "socket_policy" => {
                tx.execute(
                    "UPDATE activation_sockets SET policy=?2 WHERE socket_id=?1",
                    params![socket, value],
                )?;
            }
            "group_policy" => {
                if let Some(value) = value {
                    tx.execute(
                        "INSERT INTO activation_groups VALUES(?1,?2,?3)
                        ON CONFLICT(socket_id,group_id) DO UPDATE SET policy=excluded.policy",
                        params![socket, group, value],
                    )?;
                } else {
                    tx.execute(
                        "DELETE FROM activation_groups WHERE socket_id=?1 AND group_id=?2",
                        params![socket, group],
                    )?;
                }
            }
            _ => bail!("Invalid activation operation"),
        }
        audit(&tx, socket, actor, action, group, value)?;
        tx.commit()?;
        Ok(())
    }

    /// Internal runtime read: global_default must come from trusted runtime
    /// configuration, not from an inbound message. A single SQL snapshot reads
    /// both policy scopes. Missing sockets fail closed.
    pub fn resolve(
        &self,
        socket: &str,
        group: Option<&str>,
        global_default: Option<bool>,
    ) -> Result<ActivationDecision> {
        validate(socket)?;
        if let Some(group) = group {
            validate(group)?;
        }
        let c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Activation lock poisoned"))?;
        let row: Option<(i64, Option<i64>, Option<i64>)> = c
            .query_row(
                "SELECT s.powered,s.policy,g.policy FROM activation_sockets s
             LEFT JOIN activation_groups g ON g.socket_id=s.socket_id AND g.group_id=?2
             WHERE s.socket_id=?1",
                params![socket, group],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .optional()?;
        let (powered, socket_policy, group_policy) =
            row.ok_or_else(|| anyhow!("Socket unavailable"))?;
        let powered = decode_policy(powered)?;
        let socket_policy = socket_policy.map(decode_policy).transpose()?;
        let group_policy = group_policy.map(decode_policy).transpose()?;
        if !powered {
            return Ok(ActivationDecision {
                enabled: false,
                source: ActivationSource::PoweredOff,
            });
        }
        for (value, source) in [
            (group_policy, ActivationSource::Group),
            (socket_policy, ActivationSource::Socket),
            (global_default, ActivationSource::Global),
        ] {
            if let Some(enabled) = value {
                return Ok(ActivationDecision { enabled, source });
            }
        }
        Ok(ActivationDecision {
            enabled: false,
            source: ActivationSource::None,
        })
    }

    pub fn audit_history(&self, socket: &str, actor: &str) -> Result<Vec<ActivationAuditRecord>> {
        validate(socket)?;
        validate(actor)?;
        let mut c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Activation lock poisoned"))?;
        let tx = c.transaction()?;
        require_owner(&tx, socket, actor)?;
        let records = {
            let mut statement = tx.prepare("SELECT id,action,group_id,value,created_at FROM activation_audit WHERE socket_id=?1 ORDER BY id")?;
            let rows = statement.query_map([socket], |r| {
                Ok(ActivationAuditRecord {
                    id: r.get(0)?,
                    action: r.get(1)?,
                    group_id: r.get(2)?,
                    value: r.get(3)?,
                    created_at: r.get(4)?,
                })
            })?;
            rows.collect::<rusqlite::Result<Vec<_>>>()?
        };
        tx.commit()?;
        Ok(records)
    }
}

#[derive(Debug, Clone)]
pub struct ActivationAuditRecord {
    pub id: i64,
    pub action: String,
    pub group_id: Option<String>,
    pub value: Option<bool>,
    pub created_at: i64,
}

fn validate(value: &str) -> Result<()> {
    if value.trim().is_empty() || value.len() > 128 || value.chars().any(char::is_control) {
        bail!("Invalid activation identifier");
    }
    Ok(())
}
fn require_owner(c: &Connection, socket: &str, actor: &str) -> Result<()> {
    let owner: Option<String> = c
        .query_row(
            "SELECT owner_id FROM activation_sockets WHERE socket_id=?1",
            [socket],
            |r| r.get(0),
        )
        .optional()?;
    if owner.as_deref() != Some(actor) {
        bail!("Socket owner authorization denied");
    }
    Ok(())
}
fn audit(
    c: &Connection,
    socket: &str,
    actor: &str,
    action: &str,
    group: Option<&str>,
    value: Option<bool>,
) -> Result<()> {
    c.execute("INSERT INTO activation_audit(socket_id,actor_id,action,group_id,value,created_at) VALUES(?1,?2,?3,?4,?5,?6)", params![socket,actor,action,group,value,time::OffsetDateTime::now_utc().unix_timestamp()])?;
    Ok(())
}

fn decode_policy(value: i64) -> Result<bool> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => bail!("Invalid persisted activation policy"),
    }
}
