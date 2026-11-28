// Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
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

#[derive(Clone, Copy)]
pub enum Kind {
    Admission,
    Denied,
    Error,
    Inactive,
}

impl Kind {
    pub fn emoji(self) -> &'static str {
        match self {
            Self::Admission => "⚠️",
            Self::Denied => "🚫",
            Self::Error => "❌",
            Self::Inactive => "🔕",
        }
    }

    pub fn message(self) -> &'static str {
        match self {
            Self::Admission => "Comando no disponible o no admitido.",
            Self::Denied => "Comando denegado o servicio no disponible.",
            Self::Error => "No se pudo completar el comando.",
            Self::Inactive => "Bot inactivo en este chat.",
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Policy {
    admission: Mode,
    denied: Mode,
    error: Mode,
    inactive: Mode,
}

impl Default for Policy {
    fn default() -> Self {
        Self {
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
        "Ámbito: Socket\nAdmisión: {}\nPermisos: {}\nErrores: {}\nInactivo: {}\nModos: reaction | text | off",
        policy.admission.label(),
        policy.denied.label(),
        policy.error.label(),
        policy.inactive.label()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alert_preferences_persist_and_default_to_reaction() {
        let dir = std::env::temp_dir().join(format!("matter-alerts-{}", std::process::id()));
        fs::create_dir(&dir).unwrap();
        let session = dir.join("session.db");
        assert_eq!(mode(&session, Kind::Denied), Mode::Reaction);
        assert_eq!(mode(&session, Kind::Inactive), Mode::Off);
        configure(&session, "=/>set alerts text").unwrap();
        assert_eq!(mode(&session, Kind::Admission), Mode::Text);
        assert_eq!(mode(&session, Kind::Inactive), Mode::Off);
        configure(&session, "=/>set alerts error off").unwrap();
        assert_eq!(mode(&session, Kind::Error), Mode::Off);
        assert_eq!(mode(&session, Kind::Denied), Mode::Text);
        assert!(configure(&session, "=/>set alerts denied invalid").is_err());
        fs::remove_dir_all(dir).unwrap();
    }
}
