// Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use whatsapp_rust::prelude::*;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::Mutex,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Reaction,
    Text,
    Off,
}

impl Mode {
    fn parse(value: &str) -> Result<Self> {
        match value {
            "reaction" => Ok(Self::Reaction),
            "text" => Ok(Self::Text),
            "off" => Ok(Self::Off),
            _ => bail!("Invalid alert mode"),
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Reaction => "reaction",
            Self::Text => "text",
            Self::Off => "off",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Processing,
    Success,
    Admission,
    Denied,
    Error,
    Inactive,
}

impl Kind {
    pub fn emoji(self) -> &'static str {
        let name = match self {
            Self::Processing => "processing",
            Self::Success => "success",
            Self::Admission | Self::Denied => "warning",
            Self::Error => "error",
            Self::Inactive => "inactive",
        };
        whatsapp_matter_rust_provider::classic_template::alert_emoji(name)
    }

    pub fn message(self) -> &'static str {
        match self {
            Self::Processing => "Procesando comando.",
            Self::Success => "Comando completado.",
            Self::Admission => "Comando reconocido, no disponible en este contexto.",
            Self::Denied => "Comando denegado o servicio no disponible.",
            Self::Error => "No se pudo completar el comando.",
            Self::Inactive => "Bot inactivo en este chat.",
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Policy {
    #[serde(default)]
    processing: Option<Mode>,
    #[serde(default)]
    success: Option<Mode>,
    admission: Mode,
    denied: Mode,
    error: Mode,
    inactive: Mode,
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            processing: None,
            success: None,
            admission: Mode::Reaction,
            denied: Mode::Reaction,
            error: Mode::Reaction,
            inactive: Mode::Off,
        }
    }
}

static LOCK: Mutex<()> = Mutex::new(());

fn path(session: &Path) -> Result<PathBuf> {
    Ok(session
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Missing private state"))?
        .join("command-alerts.json"))
}

fn read(session: &Path) -> Result<Policy> {
    let path = path(session)?;
    let meta = match fs::symlink_metadata(&path) {
        Ok(meta) => meta,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(Policy::default());
        }
        Err(error) => return Err(error.into()),
    };
    if !meta.is_file() || meta.len() > 1024 {
        bail!("Invalid alert state");
    }
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

pub fn mode(session: &Path, kind: Kind) -> Mode {
    let Ok(policy) = read(session) else {
        return Mode::Off;
    };
    match kind {
        Kind::Processing => policy.processing.unwrap_or(policy.error),
        Kind::Success => policy.success.unwrap_or(policy.error),
        Kind::Admission => policy.admission,
        Kind::Denied => policy.denied,
        Kind::Error => policy.error,
        Kind::Inactive => policy.inactive,
    }
}

pub fn configure(session: &Path, command: &str) -> Result<String> {
    let _guard = LOCK
        .lock()
        .map_err(|_| anyhow::anyhow!("Alert configuration unavailable"))?;
    let mut policy = read(session)?;
    let parts: Vec<_> = command.split_whitespace().collect();
    match parts.as_slice() {
        ["=/>set", "alerts"] => {}
        ["=/>set", "alerts", value] => {
            let mode = Mode::parse(value)?;
            policy.processing = Some(mode);
            policy.success = Some(mode);
            policy.admission = mode;
            policy.denied = mode;
            policy.error = mode;
            // Never enable notifications in disabled chats implicitly.
            if mode == Mode::Off {
                policy.inactive = Mode::Off;
            }
        }
        ["=/>set", "alerts", kind, value] => {
            let mode = Mode::parse(value)?;
            match *kind {
                "processing" => policy.processing = Some(mode),
                "success" => policy.success = Some(mode),
                "admission" => policy.admission = mode,
                "denied" => policy.denied = mode,
                "error" => policy.error = mode,
                "inactive" => policy.inactive = mode,
                _ => bail!("Invalid alert type"),
            }
        }
        _ => bail!("Invalid alert command"),
    }
    if parts.len() > 2 {
        let path = path(session)?;
        let temp = path.with_extension("tmp");
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options.open(&temp)?;
        let data = serde_json::to_vec(&policy)?;
        let result = file
            .write_all(&data)
            .and_then(|()| file.sync_all())
            .and_then(|()| fs::rename(&temp, &path));
        if let Err(error) = result {
            let _ = fs::remove_file(temp);
            return Err(error.into());
        }
    }
    Ok(format!(
        "Ámbito: Socket\nProcesando: {}\nÉxito: {}\nAdmisión: {}\nPermisos: {}\nErrores: {}\nInactivo: {}\nModos: reaction | text | off",
        policy.processing.unwrap_or(policy.error).label(),
        policy.success.unwrap_or(policy.error).label(),
        policy.admission.label(),
        policy.denied.label(),
        policy.error.label(),
        policy.inactive.label()
    ))
}

/// Only metadata obtained from the authenticated WhatsApp callback is stored.
/// Old pending records without this field continue delivering their reports.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeferredTarget {
    chat: String,
    id: String,
    participant: Option<String>,
    from_me: bool,
    pub processing: bool,
}

impl DeferredTarget {
    pub fn capture(ctx: &whatsapp_rust::bot::MessageContext, processing: bool) -> Option<Self> {
        let key = ctx.message_key();
        let target = Self {
            chat: key.remote_jid?,
            id: key.id?,
            participant: key.participant,
            from_me: key.from_me?,
            processing,
        };
        target.key(&ctx.info.source.chat.to_non_ad_string())?;
        Some(target)
    }

    pub fn key(&self, chat: &str) -> Option<whatsapp_rust::prelude::wa::MessageKey> {
        let jid = self.chat.parse::<whatsapp_rust::Jid>().ok()?;
        if jid.to_non_ad_string() != chat
            || self.id.is_empty()
            || self.id.len() > 256
            || self.id.chars().any(char::is_control)
        {
            return None;
        }
        if chat.ends_with("@g.us") {
            let participant = self.participant.as_ref()?;
            participant.parse::<whatsapp_rust::Jid>().ok()?;
        } else if self.participant.is_some() {
            return None;
        }
        Some(whatsapp_rust::prelude::wa::MessageKey {
            remote_jid: Some(self.chat.clone()),
            from_me: Some(self.from_me),
            id: Some(self.id.clone()),
            participant: self.participant.clone(),
        })
    }
}

pub async fn deliver_deferred(
    client: &whatsapp_rust::client::Client,
    session: &Path,
    chat: &str,
    target: Option<&DeferredTarget>,
    kind: Kind,
) {
    let Some(target) = target else {
        return;
    };
    let Some(key) = target.key(chat) else {
        return;
    };
    let Ok(jid) = chat.parse::<whatsapp_rust::Jid>() else {
        return;
    };
    let emoji = match mode(session, kind) {
        Mode::Reaction => kind.emoji(),
        // The existing result report is the text notification, not a duplicate.
        Mode::Text | Mode::Off if target.processing => "",
        Mode::Text | Mode::Off => return,
    };
    if !matches!(
        tokio::time::timeout(
            std::time::Duration::from_secs(8),
            client.send_reaction(jid, key, emoji),
        )
        .await,
        Ok(Ok(_))
    ) {
        eprintln!("Matter: reacción diferida no confirmada");
    }
}

/// Classify the actual terminal state, not just whether IPC was accepted.
/// Deferred jobs keep their processing reaction until an explicit result report.
pub fn completion(state: &str) -> Option<Kind> {
    match state {
        "DUPLICADO" | "MANTENIMIENTO AUTORIZADO" | "CODEX EN CURSO" => None,
        "IGNORADO" => Some(Kind::Inactive),
        "IPC FALLÓ" => Some(Kind::Denied),
        "MANTENIMIENTO BLOQUEADO" | "RECARGA PARCIAL" | "CODEX EN REVISIÓN" => Some(Kind::Admission),
        "MANTENIMIENTO COMPLETADO" | "MANTENIMIENTO SIN CAMBIOS" => Some(Kind::Success),
        _ if state.starts_with("NO ADMITIDO")
            || state.starts_with("TIMEOUT")
            || state.starts_with("SIN ENVÍO") =>
        {
            Some(Kind::Admission)
        }
        _ if state.starts_with("ENVÍO ACEPTADO")
            || state.starts_with("ENVIADO")
            || state.starts_with("HTML ENVIADO") =>
        {
            Some(Kind::Success)
        }
        _ => Some(Kind::Error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alert_preferences_persist_and_default_to_reaction() {
        let dir = std::env::temp_dir().join(format!("matter-alerts-{}", std::process::id()));
        fs::create_dir(&dir).unwrap();
        let session = dir.join("session.db");
        assert_eq!(mode(&session, Kind::Processing), Mode::Reaction);
        assert_eq!(mode(&session, Kind::Success), Mode::Reaction);
        assert_eq!(mode(&session, Kind::Denied), Mode::Reaction);
        assert_eq!(mode(&session, Kind::Inactive), Mode::Off);
        configure(&session, "=/>set alerts text").unwrap();
        assert_eq!(mode(&session, Kind::Admission), Mode::Text);
        assert_eq!(mode(&session, Kind::Inactive), Mode::Off);
        configure(&session, "=/>set alerts error off").unwrap();
        assert_eq!(mode(&session, Kind::Error), Mode::Off);
        assert_eq!(mode(&session, Kind::Denied), Mode::Text);
        assert!(configure(&session, "=/>set alerts denied invalid").is_err());
        configure(&session, "=/>set alerts off").unwrap();
        assert_eq!(mode(&session, Kind::Processing), Mode::Off);
        assert_eq!(mode(&session, Kind::Success), Mode::Off);
        assert_eq!(mode(&session, Kind::Denied), Mode::Off);
        configure(&session, "=/>set alerts success reaction").unwrap();
        assert_eq!(mode(&session, Kind::Success), Mode::Reaction);
        assert_eq!(mode(&session, Kind::Error), Mode::Off);
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn deferred_reaction_cannot_change_destination() {
        let target = DeferredTarget {
            chat: "120363123456789@g.us".into(),
            id: "message-1".into(),
            participant: Some("12345678901@lid".into()),
            from_me: false,
            processing: true,
        };
        assert!(target.key("120363123456789@g.us").is_some());
        assert!(target.key("120363123456780@g.us").is_none());
        let encoded = serde_json::to_vec(&target).unwrap();
        let restored: DeferredTarget = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(
            restored.key("120363123456789@g.us").unwrap().id.as_deref(),
            Some("message-1")
        );
    }

    #[test]
    fn terminal_states_distinguish_real_outcomes() {
        assert_eq!(Kind::Processing.emoji(), "💭");
        assert_eq!(Kind::Success.emoji(), "✅");
        assert_eq!(Kind::Denied.emoji(), "⚠️");
        assert_eq!(Kind::Error.emoji(), "❌");
        assert_eq!(completion("MANTENIMIENTO AUTORIZADO"), None);
        assert_eq!(completion("CODEX EN CURSO"), None);
        assert_eq!(completion("DUPLICADO"), None);
        assert_eq!(completion("RECARGA PARCIAL"), Some(Kind::Admission));
        assert_eq!(completion("MANTENIMIENTO COMPLETADO"), Some(Kind::Success));
        assert_eq!(completion("IPC ERROR TRANSPORTE"), Some(Kind::Error));
    }
}
