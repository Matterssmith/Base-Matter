// Fixed maintenance operations; no shell text or Git arguments come from WhatsApp.
// Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

const REQUEST: &str = "maintenance-restart.pending.json";
const RESULT: &str = "maintenance-restart.result";

pub fn styled(title: &str, body: &str, plain: bool) -> Result<String> {
    let text = format!("{title}\n{body}");
    if plain {
        Ok(text)
    } else {
        whatsapp_matter_rust_provider::render_provider_control(&text)
    }
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

pub fn update_source() -> (String, &'static str) {
    let output = Command::new("bash")
        .arg(root().join("scripts/matter-update-source.sh"))
        .current_dir(root())
        .env("GIT_TERMINAL_PROMPT", "0")
        .output();
    let Ok(output) = output else {
        return (
            "Estado: Fallido\nMotivo: no se pudo iniciar Git.".into(),
            "MANTENIMIENTO FALLIDO",
        );
    };
    let state = match output.status.code() {
        Some(0) => "MANTENIMIENTO COMPLETADO",
        Some(10) => "MANTENIMIENTO SIN CAMBIOS",
        Some(20) => "MANTENIMIENTO BLOQUEADO",
        _ => "MANTENIMIENTO FALLIDO",
    };
    let report = String::from_utf8_lossy(&output.stdout);
    let body: String = report
        .chars()
        .filter(|c| {
            *c == '\n'
                || (!c.is_control()
                    && !matches!(*c, '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}'))
        })
        .take(700)
        .collect();
    let body = if body.trim().is_empty() {
        "Estado: Fallido\nMotivo: actualización sin resultado verificable.".into()
    } else {
        body
    };
    (format!("{}\nBot: sigue activo; sin reinicio.", body.trim()), state)
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Pending {
    chat: String,
    plain: bool,
}

fn paths(session: &Path) -> Result<(PathBuf, PathBuf)> {
    let parent = session
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Missing state"))?;
    Ok((parent.join(REQUEST), parent.join(RESULT)))
}

fn valid_chat(chat: &str) -> bool {
    let Some((id, domain)) = chat.split_once('@') else {
        return false;
    };
    !id.is_empty()
        && id.len() <= 80
        && id.bytes().all(|b| b.is_ascii_digit() || b == b'-')
        && matches!(domain, "g.us" | "lid" | "s.whatsapp.net")
}

pub fn request_restart(session: &Path, chat: &str, plain: bool) -> Result<()> {
    let (request, result) = paths(session)?;
    if !valid_chat(chat) || result.exists() {
        bail!("Invalid restart request");
    }
    let data = serde_json::to_vec(&Pending {
        chat: chat.to_owned(),
        plain,
    })?;
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(&request)?;
    if let Err(error) = file.write_all(&data).and_then(|()| file.sync_all()) {
        let _ = fs::remove_file(request);
        return Err(error.into());
    }
    Ok(())
}

pub fn restart_report(session: &Path) -> Result<Option<(String, String)>> {
    let (request, result) = paths(session)?;
    if !request.is_file() || !result.is_file() {
        return Ok(None);
    }
    for file in [&request, &result] {
        let meta = fs::symlink_metadata(file)?;
        if !meta.is_file() || meta.len() > 1024 {
            bail!("Invalid private maintenance report");
        }
    }
    let pending: Pending = serde_json::from_slice(&fs::read(request)?)?;
    if !valid_chat(&pending.chat) {
        bail!("Invalid maintenance destination");
    }
    let status = fs::read_to_string(result)?;
    let body = match status.trim() {
        "verified" => "Estado: Completado\nControlador: activo\nWhatsApp: conectado\nSesión: conservada.",
        "build-failed" => "Estado: Fallido\nMotivo: verificación o compilación rechazada durante el reinicio anterior.\nWhatsApp: conectado posteriormente.",
        "start-failed" => "Estado: Fallido\nMotivo: arranque anterior rechazado.\nWhatsApp: conectado posteriormente.",
        _ => bail!("Unknown maintenance result"),
    };
    Ok(Some((pending.chat, styled("Restart", body, pending.plain)?)))
}

pub fn clear_restart(session: &Path) {
    if let Ok((request, result)) = paths(session) {
        let _ = fs::remove_file(result);
        let _ = fs::remove_file(request);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maintenance_style_and_destination() {
        assert_eq!(
            styled("Update", "Estado: OK", true).unwrap(),
            "Update\nEstado: OK"
        );
        let matter = styled("Update", "Estado: OK", false).unwrap();
        assert!(matter.contains("Estado: OK"));
        assert!(matter.contains("╭"));
        assert!(valid_chat("12345-67890@g.us"));
        assert!(!valid_chat("123@newsletter"));
        assert!(!valid_chat("123@lid\n"));
    }
}
