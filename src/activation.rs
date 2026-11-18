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

#[derive(Clone, Copy)]
pub(crate) enum GlobalPolicy {
    Configured(Option<bool>),
    Persisted,
}

pub(crate) struct SocketAdmission {
    pub decision: ActivationDecision,
    pub owner_id: String,
    pub bot_instance_id: Option<String>,
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
        if version.is_some_and(|v| !(1..=4).contains(&v)) {
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
            value INTEGER, created_at INTEGER NOT NULL, target_id TEXT);
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
            INSERT INTO matter_schema_versions VALUES('socket_activation',4)
            ON CONFLICT(component) DO UPDATE SET version=excluded.version;",
        )?;
        let has_target: bool = tx.query_row(
            "SELECT EXISTS(SELECT 1 FROM pragma_table_info('activation_audit') WHERE name='target_id')",
            [], |r| r.get(0),
        )?;
        if !has_target {
            tx.execute_batch("ALTER TABLE activation_audit ADD COLUMN target_id TEXT;")?;
        }
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
            None,
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
        audit(&tx, socket, actor, "bind_bot", None, None, Some(bot))?;
        tx.commit()?;
        Ok(())
    }

    /// The current socket owner may release a stale bot binding without bot
    /// authority. Power and local overrides are reset atomically with audit.
    /// No binding is an idempotent no-op; global policy is never changed.
    pub fn release_binding(&self, socket: &str, actor: &str) -> Result<()> {
        validate(socket)?;
        validate(actor)?;
        let mut c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Activation lock poisoned"))?;
        let tx = c.transaction_with_behavior(TransactionBehavior::Immediate)?;
        require_owner(&tx, socket, actor)?;
        let bot: Option<String> = tx
            .query_row(
                "SELECT bot_instance_id FROM activation_bindings WHERE socket_id=?1",
                [socket],
                |r| r.get(0),
            )
            .optional()?;
        let Some(bot) = bot else {
            return Ok(());
        };
        reset_binding(&tx, socket)?;
        audit(
            &tx,
            socket,
            actor,
            "release_binding",
            None,
            Some(false),
            Some(&bot),
        )?;
        tx.commit()?;
        Ok(())
    }

    /// Owner-authorized handover of a socket, independent of bot ownership.
    /// The new owner must explicitly bind a bot and power on. A global policy
    /// administrator has no special transfer authority. The caller validates
    /// the intended recipient's authenticated application identity.
    pub fn transfer_owner(&self, socket: &str, actor: &str, new_owner: &str) -> Result<()> {
        validate(socket)?;
        validate(actor)?;
        validate(new_owner)?;
        let mut c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Activation lock poisoned"))?;
        let tx = c.transaction_with_behavior(TransactionBehavior::Immediate)?;
        require_owner(&tx, socket, actor)?;
        if actor == new_owner {
            return Ok(());
        }
        reset_binding(&tx, socket)?;
        tx.execute(
            "UPDATE activation_sockets SET owner_id=?2 WHERE socket_id=?1",
            params![socket, new_owner],
        )?;
        audit(
            &tx,
            socket,
            actor,
            "transfer_owner",
            None,
            Some(false),
            Some(new_owner),
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
        audit(&tx, socket, actor, action, group, value, None)?;
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
        Ok(self
            .admission(socket, group, GlobalPolicy::Configured(global_default))?
            .decision)
    }

    /// Reads power, group, socket and persisted global policy in one snapshot.
    /// No global provisioning/policy means no fallback (deny). Invalid stored
    /// policy values fail closed even if another scope would override them.
    pub fn resolve_persisted(
        &self,
        socket: &str,
        group: Option<&str>,
    ) -> Result<ActivationDecision> {
        Ok(self
            .admission(socket, group, GlobalPolicy::Persisted)?
            .decision)
    }

    pub(crate) fn admission(
        &self,
        socket: &str,
        group: Option<&str>,
        global: GlobalPolicy,
    ) -> Result<SocketAdmission> {
        let (persisted, configured) = match global {
            GlobalPolicy::Configured(value) => (false, value),
            GlobalPolicy::Persisted => (true, None),
        };
        validate(socket)?;
        if let Some(group) = group {
            validate(group)?;
        }
        let c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Activation lock poisoned"))?;
        type PolicyRow = (
            i64,
            Option<i64>,
            Option<i64>,
            Option<i64>,
            String,
            Option<String>,
        );
        let row: Option<PolicyRow> = c
            .query_row(
                "SELECT s.powered,s.policy,g.policy,
             CASE WHEN ?3 THEN (SELECT policy FROM activation_global WHERE singleton=1) ELSE ?4 END,
             s.owner_id,b.bot_instance_id
             FROM activation_sockets s
             LEFT JOIN activation_groups g ON g.socket_id=s.socket_id AND g.group_id=?2
             LEFT JOIN activation_bindings b ON b.socket_id=s.socket_id
             WHERE s.socket_id=?1",
                params![socket, group, persisted, configured],
                |r| {
                    Ok((
                        r.get(0)?,
                        r.get(1)?,
                        r.get(2)?,
                        r.get(3)?,
                        r.get(4)?,
                        r.get(5)?,
                    ))
                },
            )
            .optional()?;
        let (powered, socket_policy, group_policy, global_policy, owner_id, bot_instance_id) =
            row.ok_or_else(|| anyhow!("Socket unavailable"))?;
        let powered = decode_policy(powered)?;
        let socket_policy = socket_policy.map(decode_policy).transpose()?;
        let group_policy = group_policy.map(decode_policy).transpose()?;
        let global_policy = global_policy.map(decode_policy).transpose()?;
        let decision = if !powered {
            ActivationDecision {
                enabled: false,
                source: ActivationSource::PoweredOff,
            }
        } else {
            [
                (group_policy, ActivationSource::Group),
                (socket_policy, ActivationSource::Socket),
                (global_policy, ActivationSource::Global),
            ]
            .into_iter()
            .find_map(|(value, source)| value.map(|enabled| ActivationDecision { enabled, source }))
            .unwrap_or(ActivationDecision {
                enabled: false,
                source: ActivationSource::None,
            })
        };
        Ok(SocketAdmission {
            decision,
            owner_id,
            bot_instance_id,
        })
    }

    /// Read raw per-socket bot defaults regardless of operational power.
    /// Only the verified socket owner may inspect these settings. The
    /// synthetic kind keys are internal configuration scopes, not group
    /// JIDs and never confer role permissions.
    pub fn inspect_bot_defaults(
        &self,
        socket: &str,
        actor: &str,
    ) -> Result<(bool, Option<bool>, Option<bool>, Option<bool>, Option<bool>)> {
        validate(socket)?;
        validate(actor)?;
        let c = self
            .connection
            .lock()
            .map_err(|_| anyhow!("Activation lock poisoned"))?;
        require_owner(&c, socket, actor)?;
        let (power, socket_policy): (i64, Option<i64>) = c.query_row(
            "SELECT powered, policy FROM activation_sockets WHERE socket_id=?1",
            [socket],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;
        let mut kinds = [None; 3];
        for (index, mode) in ["group", "private", "channel"].iter().enumerate() {
            let key = format!("kind:{mode}");
            let stored: Option<i64> = c
                .query_row(
                    "SELECT policy FROM activation_groups WHERE socket_id=?1 AND group_id=?2",
                    params![socket, key],
                    |row| row.get(0),
                )
                .optional()?;
            kinds[index] = stored.map(decode_policy).transpose()?;
        }
        Ok((
            decode_policy(power)?,
            socket_policy.map(decode_policy).transpose()?,
            kinds[0],
            kinds[1],
            kinds[2],
        ))
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
            let mut statement = tx.prepare("SELECT id,action,group_id,value,created_at,actor_id,target_id FROM activation_audit WHERE socket_id=?1 ORDER BY id")?;
            let mut rows = statement.query([socket])?;
            let mut records = Vec::new();
            while let Some(r) = rows.next()? {
                let value: Option<i64> = r.get(3)?;
                records.push(ActivationAuditRecord {
                    id: r.get(0)?,
                    action: r.get(1)?,
                    group_id: r.get(2)?,
                    value: value.map(decode_policy).transpose()?,
                    created_at: r.get(4)?,
                    actor_id: r.get(5)?,
                    target_id: r.get(6)?,
                });
            }
            records
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
    pub actor_id: String,
    /// Bot ID for binding/release, new owner for transfer; absent for legacy
    /// rows and operations without a target. Interpret together with action.
    pub target_id: Option<String>,
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
    target: Option<&str>,
) -> Result<()> {
    c.execute("INSERT INTO activation_audit(socket_id,actor_id,action,group_id,value,created_at,target_id) VALUES(?1,?2,?3,?4,?5,?6,?7)", params![socket,actor,action,group,value,time::OffsetDateTime::now_utc().unix_timestamp(),target])?;
    Ok(())
}

fn reset_binding(c: &Connection, socket: &str) -> Result<()> {
    c.execute(
        "UPDATE activation_sockets SET powered=0,policy=NULL WHERE socket_id=?1",
        [socket],
    )?;
    c.execute("DELETE FROM activation_groups WHERE socket_id=?1", [socket])?;
    c.execute(
        "DELETE FROM activation_bindings WHERE socket_id=?1",
        [socket],
    )?;
    Ok(())
}

fn decode_policy(value: i64) -> Result<bool> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => bail!("Invalid persisted activation policy"),
    }
}
