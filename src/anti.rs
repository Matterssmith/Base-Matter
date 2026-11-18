/*
 * Base-Matter — isolated, durable group anti-rule configuration.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{anyhow, bail, Result};
use rusqlite::{params, Connection, OptionalExtension};
use std::{fs, path::Path, sync::Mutex, time::Duration};

/// Rules are independent and all deny-by-default (disabled) until enabled
/// by the authenticated socket owner for the exact signed group.
pub const RULES: &[&str] = &[
    "antilink",
    "antilinkwa",
    "antilinktelegram",
    "antilinkdiscord",
    "antilinkinstagram",
    "antilinkfacebook",
    "antilinktiktok",
    "antilinkyoutube",
    "antilinkx",
    "antilinkshort",
    "antilinkphishing",
    "antigroup",
    "antichannel",
    "antiviewonce",
    "anticall",
    "antisticker",
    "antiimage",
    "antivideo",
    "antiaudio",
    "antidocument",
    "antigif",
    "antilocation",
    "anticontact",
    "antipoll",
    "antiforward",
    "antispam",
    "antiflood",
    "antimention",
    "antibot",
    "antimedia",
    "antidelete",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AntiAction {
    Warn,
    Delete,
    Kick,
}
impl AntiAction {
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "warn" | "avisar" | "warning" => Ok(Self::Warn),
            "delete" | "eliminar" | "borrar" => Ok(Self::Delete),
            "kick" | "expulsar" => Ok(Self::Kick),
            _ => bail!("Acción inválida. Usa warn, delete o kick"),
        }
    }
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Warn => "warn",
            Self::Delete => "delete",
            Self::Kick => "kick",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AntiRule {
    pub enabled: bool,
    pub action: AntiAction,
}

/// A matched rule is a *detection candidate*, not permission to moderate.
/// The provider must authenticate sender, group type and admin authority
/// before any future action adapter may execute it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AntiFinding {
    pub rule: &'static str,
    pub requested_action: AntiAction,
}

pub struct AntiStore {
    db: Mutex<Connection>,
}

fn valid_socket(socket: &str) -> Result<()> {
    if socket.is_empty()
        || socket.len() > 128
        || !socket.bytes().all(|c| c.is_ascii_alphanumeric() || b"._:-".contains(&c))
    {
        bail!("Socket inválido");
    }
    Ok(())
}

fn valid_scope(socket: &str, group: &str) -> Result<()> {
    if socket.is_empty()
        || socket.len() > 128
        || !socket.bytes().all(|c| c.is_ascii_alphanumeric() || b"._:-".contains(&c))
        || group.len() > 128
        || !group.ends_with("@g.us")
        || !group.bytes().all(|c| c.is_ascii_alphanumeric() || b"@._:-".contains(&c))
    {
        bail!("Ámbito de grupo no verificado");
    }
    Ok(())
}
/// Accept the grouped `=/>anti <rule>` syntax while keeping the earlier
/// canonical database keys stable across upgrades.
pub fn canonical_rule(rule: &str) -> Result<&'static str> {
    let key = match rule {
        "link" => "antilink",
        "linkwa" | "whatsapp" => "antilinkwa",
        "telegram" => "antilinktelegram",
        "discord" => "antilinkdiscord",
        "instagram" => "antilinkinstagram",
        "facebook" => "antilinkfacebook",
        "tiktok" => "antilinktiktok",
        "youtube" => "antilinkyoutube",
        "x" | "twitter" => "antilinkx",
        "short" | "shortlink" => "antilinkshort",
        "phishing" => "antilinkphishing",
        "group" | "groups" | "invite" => "antigroup",
        "channel" | "channels" => "antichannel",
        "oneview" | "viewonce" | "once" => "antiviewonce",
        "call" | "calls" => "anticall",
        "sticker" | "stiker" | "stickers" => "antisticker",
        "image" | "photo" | "picture" => "antiimage",
        "movie" | "video" => "antivideo",
        "audio" | "voice" => "antiaudio",
        "document" | "file" => "antidocument",
        "gif" => "antigif",
        "location" => "antilocation",
        "contact" => "anticontact",
        "poll" => "antipoll",
        "forward" => "antiforward",
        "spam" => "antispam",
        "flood" => "antiflood",
        "mention" | "mentions" => "antimention",
        "bot" => "antibot",
        "media" => "antimedia",
        "delete" => "antidelete",
        other => other,
    };
    RULES.iter().copied().find(|value| *value == key)
        .ok_or_else(|| anyhow!("Protección desconocida"))
}

impl AntiStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if path != Path::new(":memory:") {
            match fs::symlink_metadata(path) {
                Ok(meta) => {
                    if !meta.file_type().is_file() || meta.file_type().is_symlink() {
                        bail!("Base de protecciones insegura");
                    }
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        if meta.permissions().mode() & 0o077 != 0 {
                            bail!("Base de protecciones debe ser privada");
                        }
                    }
                }
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    let mut opts = fs::OpenOptions::new();
                    opts.write(true).create_new(true);
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::OpenOptionsExt;
                        opts.mode(0o600);
                    }
                    opts.open(path)?;
                }
                Err(error) => return Err(error.into()),
            }
        }
        let db = Connection::open(path)?;
        db.busy_timeout(Duration::from_secs(3))?;
        db.execute_batch(
            "CREATE TABLE IF NOT EXISTS group_anti_rules (
               socket_id TEXT NOT NULL,
               group_jid TEXT NOT NULL,
               rule TEXT NOT NULL,
               enabled INTEGER NOT NULL CHECK(enabled IN (0,1)),
               action TEXT NOT NULL CHECK(action IN ('warn','delete','kick')),
               PRIMARY KEY(socket_id, group_jid, rule)
             );
             CREATE TABLE IF NOT EXISTS socket_anti_defaults (
               socket_id TEXT NOT NULL,
               rule TEXT NOT NULL,
               enabled INTEGER NOT NULL CHECK(enabled IN (0,1)),
               action TEXT NOT NULL CHECK(action IN ('warn','delete','kick')),
               PRIMARY KEY(socket_id, rule)
             );
             CREATE TABLE IF NOT EXISTS group_anti_audit (
               id INTEGER PRIMARY KEY,
               socket_id TEXT NOT NULL,
               group_jid TEXT NOT NULL,
               rule TEXT NOT NULL,
               operation TEXT NOT NULL,
               created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
             );",
        )?;
        Ok(Self { db: Mutex::new(db) })
    }

    /// Effective setting: explicit group rule > socket default > disabled/warn.
    pub fn get(&self, socket: &str, group: &str, name: &str) -> Result<AntiRule> {
        valid_scope(socket, group)?;
        let rule = canonical_rule(name)?;
        let group_value = {
            let db = self.db.lock().map_err(|_| anyhow!("Protecciones no disponibles"))?;
            db.query_row(
                "SELECT enabled, action FROM group_anti_rules
                 WHERE socket_id=?1 AND group_jid=?2 AND rule=?3",
                params![socket, group, rule],
                |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?)),
            )
            .optional()?
        };
        if let Some(value) = group_value {
            return Self::decode(value);
        }
        self.get_global(socket, rule)
    }

    fn decode(row: (i64, String)) -> Result<AntiRule> {
        if row.0 != 0 && row.0 != 1 {
            bail!("Estado persistido inválido");
        }
        Ok(AntiRule {
            enabled: row.0 == 1,
            action: AntiAction::parse(&row.1)?,
        })
    }

    pub fn get_global(&self, socket: &str, name: &str) -> Result<AntiRule> {
        valid_socket(socket)?;
        let rule = canonical_rule(name)?;
        let db = self.db.lock().map_err(|_| anyhow!("Protecciones no disponibles"))?;
        let row = db.query_row(
            "SELECT enabled, action FROM socket_anti_defaults
             WHERE socket_id=?1 AND rule=?2",
            params![socket, rule],
            |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?)),
        ).optional()?;
        match row {
            Some(value) => Self::decode(value),
            None => Ok(AntiRule { enabled: false, action: AntiAction::Warn }),
        }
    }

    /// Writes are atomic with an immutable, bounded audit entry.
    fn set(&self, socket: &str, group: Option<&str>, name: &str, enabled: Option<bool>, action: Option<AntiAction>) -> Result<AntiRule> {
        valid_socket(socket)?;
        if let Some(group) = group {
            valid_scope(socket, group)?;
        }
        let rule = canonical_rule(name)?;
        let previous = if let Some(group) = group {
            self.get(socket, group, name)?
        } else {
            self.get_global(socket, name)?
        };
        let selected = AntiRule {
            enabled: enabled.unwrap_or(previous.enabled),
            action: action.unwrap_or(previous.action),
        };
        let mut db = self.db.lock().map_err(|_| anyhow!("Protecciones no disponibles"))?;
        let tx = db.transaction()?;
        if let Some(group) = group {
            tx.execute(
                "INSERT INTO group_anti_rules(socket_id,group_jid,rule,enabled,action)
                 VALUES(?1,?2,?3,?4,?5)
                 ON CONFLICT(socket_id,group_jid,rule)
                 DO UPDATE SET enabled=excluded.enabled, action=excluded.action",
                params![socket, group, rule, i64::from(selected.enabled), selected.action.as_str()],
            )?;
        } else {
            tx.execute(
                "INSERT INTO socket_anti_defaults(socket_id,rule,enabled,action)
                 VALUES(?1,?2,?3,?4)
                 ON CONFLICT(socket_id,rule)
                 DO UPDATE SET enabled=excluded.enabled, action=excluded.action",
                params![socket, rule, i64::from(selected.enabled), selected.action.as_str()],
            )?;
        }
        tx.execute(
            "INSERT INTO group_anti_audit(socket_id,group_jid,rule,operation)
             VALUES(?1,?2,?3,?4)",
            params![
                socket,
                group.unwrap_or("@global"),
                rule,
                format!("enabled={};action={}", selected.enabled, selected.action.as_str())
            ],
        )?;
        tx.commit()?;
        Ok(selected)
    }

    /// Revert just one group rule to its socket default.
    pub fn inherit(&self, socket: &str, group: &str, name: &str) -> Result<AntiRule> {
        valid_scope(socket, group)?;
        let rule = canonical_rule(name)?;
        let mut db = self.db.lock().map_err(|_| anyhow!("Protecciones no disponibles"))?;
        let tx = db.transaction()?;
        tx.execute(
            "DELETE FROM group_anti_rules WHERE socket_id=?1 AND group_jid=?2 AND rule=?3",
            params![socket, group, rule],
        )?;
        tx.execute(
            "INSERT INTO group_anti_audit(socket_id,group_jid,rule,operation)
             VALUES(?1,?2,?3,'inherit')",
            params![socket, group, rule],
        )?;
        tx.commit()?;
        drop(db);
        self.get_global(socket, rule)
    }

    fn display(&self, socket: &str, group: Option<&str>, name: &str) -> Result<String> {
        let rule = canonical_rule(name)?;
        let setting = match group {
            Some(group) => self.get(socket, group, rule)?,
            None => self.get_global(socket, rule)?,
        };
        Ok(format!(
            "{}: {} [{}]",
            rule.strip_prefix("anti").unwrap_or(rule),
            if setting.enabled { "on" } else { "off" },
            setting.action.as_str(),
        ))
    }

    fn all_status(&self, socket: &str, group: Option<&str>) -> Result<String> {
        let mut lines = Vec::with_capacity(RULES.len());
        for rule in RULES {
            lines.push(self.display(socket, group, rule)?);
        }
        let scope = if group.is_some() { "grupo" } else { "socket global" };
        Ok(format!(
            "Antis del {scope} (solo configuración; moderación automática no conectada):\n{}",
            lines.join("\n")
        ))
    }

    /// Group context uses Some(group), socket-global configuration uses None.
    /// No command in this parser may authorize its own caller.
    pub fn command(&self, socket: &str, group: Option<&str>, args: &[String]) -> Result<String> {
        valid_socket(socket)?;
        if let Some(group) = group {
            valid_scope(socket, group)?;
        }
        match args {
            [] => self.all_status(socket, group),
            [word] if word == "status" => self.all_status(socket, group),
            [name] => self.display(socket, group, name),
            [name, word] if word == "status" => self.display(socket, group, name),
            [name, word] if matches!(word.as_str(), "on" | "off") => {
                let result = self.set(socket, group, name, Some(word == "on"), None)?;
                Ok(format!(
                    "{}: {} [{}]; configuración guardada, sin moderación automática.",
                    canonical_rule(name)?.strip_prefix("anti").unwrap_or(name),
                    if result.enabled { "on" } else { "off" },
                    result.action.as_str(),
                ))
            }
            [name, word] if word == "inherit" && group.is_some() => {
                let result = self.inherit(socket, group.expect("checked scope"), name)?;
                Ok(format!(
                    "{}: heredado [{}]; configuración guardada.",
                    canonical_rule(name)?.strip_prefix("anti").unwrap_or(name),
                    result.action.as_str(),
                ))
            }
            _ => bail!("Uso: anti [tipo] [status|on|off|inherit]"),
        }
    }

    pub fn set_action(&self, socket: &str, group: Option<&str>, name: &str, action: &str) -> Result<String> {
        let selected = self.set(socket, group, name, None, Some(AntiAction::parse(action)?))?;
        Ok(format!(
            "{}: acción {} guardada; moderación automática pendiente.",
            canonical_rule(name)?.strip_prefix("anti").unwrap_or(name),
            selected.action.as_str(),
        ))
    }

    /// Text-only link classification. No chat mutations occur here.
    /// Broader message/media protections require verified metadata.
    pub fn inspect_links(&self, socket: &str, group: &str, text: &str) -> Result<Vec<AntiFinding>> {
        valid_scope(socket, group)?;
        if text.len() > 4096 { bail!("Mensaje excede límite"); }
        let lowered = text.to_ascii_lowercase();
        let mut domains = Vec::new();
        for word in lowered.split_whitespace() {
            let w = word.trim_matches(|c: char| "<>()[]{}'\",;!".contains(c));
            let after = w.strip_prefix("https://").or_else(|| w.strip_prefix("http://"));
            if let Some(host) = after.and_then(|s| s.split('/').next()) {
                let host = host.split('?').next().unwrap_or("");
                let host = host.split(':').next().unwrap_or("");
                if host.is_empty() || host.len() > 253 || !host.contains('.') || !host.bytes().all(|b| b.is_ascii_alphanumeric() || b".-".contains(&b)) { continue; }
                domains.push(host.to_owned());
            }
        }
        let mut found = Vec::new();
        for name in RULES.iter().copied().filter(|s| s.starts_with("antilink")) {
            let rule = self.get(socket, group, name)?;
            if !rule.enabled { continue; }
            let relevant = domains.iter().any(|host| {
                let suffix = |name: &str| host == name || host.ends_with(&format!(".{name}"));
                match name {
                    "antilink" => true,
                    "antilinkwa" => suffix("chat.whatsapp.com") || suffix("whatsapp.com"),
                    "antilinktelegram" => suffix("t.me") || suffix("telegram.me"),
                    "antilinkdiscord" => suffix("discord.gg") || suffix("discord.com"),
                    "antilinkinstagram" => suffix("instagram.com"),
                    "antilinkfacebook" => suffix("facebook.com") || suffix("fb.com"),
                    "antilinktiktok" => suffix("tiktok.com"),
                    "antilinkyoutube" => suffix("youtube.com") || suffix("youtu.be"),
                    "antilinkx" => suffix("x.com") || suffix("twitter.com"),
                    "antilinkshort" => suffix("bit.ly") || suffix("tinyurl.com") || suffix("t.co"),
                    "antilinkphishing" => false, // No claim without verified threat intelligence.
                    _ => false,
                }
            });
            if relevant {
                found.push(AntiFinding { rule: name, requested_action: rule.action });
            }
        }
        Ok(found)
    }
}

// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
