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
        if version.is_some_and(|v| !(1..=3).contains(&v)) {
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
            CREATE TABLE IF NOT EXISTS activation_bindings (
            socket_id TEXT PRIMARY KEY NOT NULL REFERENCES activation_sockets(socket_id),
            bot_instance_id TEXT NOT NULL);
            CREATE TABLE IF NOT EXISTS activation_global (
            singleton INTEGER PRIMARY KEY CHECK(singleton=1), owner_id TEXT NOT NULL,
            policy INTEGER CHECK(policy IN (0,1)));
            CREATE TABLE IF NOT EXISTS activation_global_audit (
            id INTEGER PRIMARY KEY AUTOINCREMENT, actor_id TEXT NOT NULL,
            action TEXT NOT NULL, value INTEGER CHECK(value IN (0,1)),
            created_at INTEGER NOT NULL);
            INSERT INTO matter_schema_versions VALUES('socket_activation',3)
            ON CONFLICT(component) DO UPDATE SET version=excluded.version;",
        )?;
        tx.commit()?;
        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    /// Trusted bootstrap of the global policy administrator. This confers no
    /// socket ownership or power authority. Duplicate provisioning is rejected.
    pub fn provision_global(&self, authenticated_owner: &str) -> Result<()> {
        validate(authenticated_owner)?;
        let mut c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Activation lock poisoned"))?;
        let tx = c.transaction_with_behavior(TransactionBehavior::Immediate)?;
        tx.execute(
            "INSERT INTO activation_global(singleton,owner_id) VALUES(1,?1)",
            [authenticated_owner],
        )?;
        global_audit(&tx, authenticated_owner, "provision", None)?;
        tx.commit()?;
        Ok(())
    }

    /// Updates only the global fallback. None restores deny-by-default where
    /// neither group nor socket overrides apply. It never changes socket power.
    pub fn set_global_policy(&self, actor: &str, value: Option<bool>) -> Result<()> {
        validate(actor)?;
        let mut c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Activation lock poisoned"))?;
        let tx = c.transaction_with_behavior(TransactionBehavior::Immediate)?;
        require_global_owner(&tx, actor)?;
        tx.execute(
            "UPDATE activation_global SET policy=?1 WHERE singleton=1",
            [value],
        )?;
        global_audit(&tx, actor, "policy", value)?;
        tx.commit()?;
        Ok(())
    }

    /// Reads a bounded page after an audit ID. Only the stored global policy
    /// administrator may read it; socket ownership alone grants no access.
    pub fn global_audit_history(
        &self,
        actor: &str,
        after_id: i64,
        limit: usize,
    ) -> Result<Vec<GlobalActivationAuditRecord>> {
        validate(actor)?;
        if after_id < 0 || !(1..=1000).contains(&limit) {
            bail!("Invalid global activation audit page");
        }
        let mut c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Activation lock poisoned"))?;
        let tx = c.transaction()?;
        require_global_owner(&tx, actor)?;
        let mut records = Vec::new();
        {
            let mut statement = tx.prepare(
                "SELECT id,actor_id,action,value,created_at
                FROM activation_global_audit WHERE id>?1 ORDER BY id LIMIT ?2",
            )?;
            let mut rows = statement.query(params![after_id, limit as i64])?;
            while let Some(row) = rows.next()? {
                let value: Option<i64> = row.get(3)?;
                records.push(GlobalActivationAuditRecord {
                    id: row.get(0)?,
                    actor_id: row.get(1)?,
                    action: row.get(2)?,
                    value: value.map(decode_policy).transpose()?,
                    created_at: row.get(4)?,
                });
            }
        }
        tx.commit()?;
        Ok(records)
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

    /// Establishes a durable socket/bot association. Caller identity must be
    /// authenticated. Both socket ownership and current bot ownership are needed.
    /// Socket ownership remains separate from bot ownership after provisioning.
    pub fn bind_bot(
        &self,
        socket: &str,
        actor: &str,
        bot: &str,
        bots: &crate::BotRegistry,
    ) -> Result<()> {
        validate(socket)?;
        validate(actor)?;
        validate(bot)?;
        bots.authorize_owner(bot, actor)?;
        let mut c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Activation lock poisoned"))?;
        let tx = c.transaction_with_behavior(TransactionBehavior::Immediate)?;
        require_owner(&tx, socket, actor)?;
        let existing: Option<String> = tx
            .query_row(
                "SELECT bot_instance_id FROM activation_bindings WHERE socket_id=?1",
                [socket],
                |r| r.get(0),
            )
            .optional()?;
        if let Some(existing) = existing {
            if existing != bot {
                bail!("Socket is already bound to another bot");
            }
            return Ok(());
        }
        tx.execute(
            "INSERT INTO activation_bindings VALUES(?1,?2)",
            params![socket, bot],
        )?;
        audit(&tx, socket, actor, "bind_bot", None, None)?;
        tx.commit()?;
        Ok(())
    }

    /// Trusted dispatcher lookup; adapters cannot choose a different bot ID.
    pub(crate) fn binding(&self, socket: &str) -> Result<(String, String)> {
        validate(socket)?;
        let c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Activation lock poisoned"))?;
        c.query_row(
            "SELECT b.bot_instance_id,s.owner_id FROM activation_bindings b
            JOIN activation_sockets s ON s.socket_id=b.socket_id WHERE b.socket_id=?1",
            [socket],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional()?
        .ok_or_else(|| anyhow!("Socket has no bot binding"))
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
        self.resolve_with_global(socket, group, false, global_default)
    }

    /// Reads power, group, socket and persisted global policy in one snapshot.
    /// No global provisioning/policy means no fallback (deny). Invalid stored
    /// policy values fail closed even if another scope would override them.
    pub fn resolve_persisted(
        &self,
        socket: &str,
        group: Option<&str>,
    ) -> Result<ActivationDecision> {
        self.resolve_with_global(socket, group, true, None)
    }

    fn resolve_with_global(
        &self,
        socket: &str,
        group: Option<&str>,
        persisted: bool,
        configured: Option<bool>,
    ) -> Result<ActivationDecision> {
        validate(socket)?;
        if let Some(group) = group {
            validate(group)?;
        }
        let c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Activation lock poisoned"))?;
        type PolicyRow = (i64, Option<i64>, Option<i64>, Option<i64>);
        let row: Option<PolicyRow> = c
            .query_row(
                "SELECT s.powered,s.policy,g.policy,
             CASE WHEN ?3 THEN (SELECT policy FROM activation_global WHERE singleton=1) ELSE ?4 END
             FROM activation_sockets s
             LEFT JOIN activation_groups g ON g.socket_id=s.socket_id AND g.group_id=?2
             WHERE s.socket_id=?1",
                params![socket, group, persisted, configured],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
            )
            .optional()?;
        let (powered, socket_policy, group_policy, global_policy) =
            row.ok_or_else(|| anyhow!("Socket unavailable"))?;
        let powered = decode_policy(powered)?;
        let socket_policy = socket_policy.map(decode_policy).transpose()?;
        let group_policy = group_policy.map(decode_policy).transpose()?;
        let global_policy = global_policy.map(decode_policy).transpose()?;
        if !powered {
            return Ok(ActivationDecision {
                enabled: false,
                source: ActivationSource::PoweredOff,
            });
        }
        for (value, source) in [
            (group_policy, ActivationSource::Group),
            (socket_policy, ActivationSource::Socket),
            (global_policy, ActivationSource::Global),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalActivationAuditRecord {
    pub id: i64,
    pub actor_id: String,
    pub action: String,
    pub value: Option<bool>,
    pub created_at: i64,
}

fn require_global_owner(c: &Connection, actor: &str) -> Result<()> {
    let owner: Option<String> = c
        .query_row(
            "SELECT owner_id FROM activation_global WHERE singleton=1",
            [],
            |r| r.get(0),
        )
        .optional()?;
    if owner.as_deref() != Some(actor) {
        bail!("Global policy owner authorization denied");
    }
    Ok(())
}

fn global_audit(c: &Connection, actor: &str, action: &str, value: Option<bool>) -> Result<()> {
    c.execute(
        "INSERT INTO activation_global_audit(actor_id,action,value,created_at) VALUES(?1,?2,?3,?4)",
        params![
            actor,
            action,
            value,
            time::OffsetDateTime::now_utc().unix_timestamp()
        ],
    )?;
    Ok(())
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
