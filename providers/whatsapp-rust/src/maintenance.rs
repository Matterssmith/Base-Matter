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

fn styled_sections(sections: &[(&str, &str)], plain: bool) -> Result<String> {
    if sections.iter().any(|(_, body)| {
        body.is_empty() || body.chars().any(|c| c.is_control() && c != '\n')
    }) {
        bail!("Invalid maintenance report");
    }
    let text = if plain {
        sections
            .iter()
            .map(|(title, body)| format!("{title}\n{body}"))
            .collect::<Vec<_>>()
            .join("\n\n")
    } else {
        whatsapp_matter_rust_provider::classic_template::maintenance_reports(sections)
            .ok_or_else(|| anyhow::anyhow!("Invalid maintenance layout"))?
    };
    if text.len() > 3800 {
        bail!("Maintenance report exceeds transport limit");
    }
    Ok(text)
}

pub fn styled(title: &str, body: &str, plain: bool) -> Result<String> {
    styled_sections(&[(title, body)], plain)
}

pub fn update_message(body: &str, verification: &str, action: &str, plain: bool) -> Result<String> {
    let mut summary = Vec::new();
    let mut changes = Vec::new();
    let mut reading_changes = false;
    for line in body.lines() {
        if line == "Cambios (máx. 5):" {
            reading_changes = true;
        } else if line.starts_with("• ") && reading_changes {
            changes.push(line);
        } else {
            reading_changes = false;
            summary.push(line);
        }
    }
    let summary = summary.join("\n");
    let changes = if changes.is_empty() {
        "Sin cambios aplicados en esta operación.".to_owned()
    } else {
        changes.join("\n")
    };
    styled_sections(
        &[
            ("Update", &summary),
            ("Cambios", &changes),
            ("Verificación", verification),
            ("Restart", action),
        ],
        plain,
    )
}

fn policy_path(session: &Path) -> Result<PathBuf> {
    Ok(session
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Missing private state"))?
        .join("maintenance-update-policy"))
}

pub fn update_policy(session: &Path) -> Result<String> {
    let path = policy_path(session)?;
    match fs::symlink_metadata(&path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok("manual".into()),
        Err(error) => Err(error.into()),
        Ok(meta) => {
            if !meta.is_file() || meta.len() > 16 {
                bail!("Invalid policy file");
            }
            let value = fs::read_to_string(path)?;
            match value.trim() {
                "manual" | "restart" | "reload" => Ok(value.trim().to_owned()),
                _ => bail!("Invalid update policy"),
            }
        }
    }
}

pub fn set_update(session: &Path, command: &str) -> Result<String> {
    let words: Vec<_> = command.split_whitespace().collect();
    match words.as_slice() {
        ["=/>set", "update"] => {}
        ["=/>set", "update", mode @ ("manual" | "none" | "restart" | "reload")] => {
            let mode = if *mode == "none" { "manual" } else { *mode };
            let path = policy_path(session)?;
            let _ = update_policy(session)?;
            let temporary = path.with_extension("tmp");
            let mut options = OpenOptions::new();
            options.write(true).create_new(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            let mut file = options.open(&temporary)?;
            let written = file
                .write_all(mode.as_bytes())
                .and_then(|()| file.sync_all())
                .and_then(|()| fs::rename(&temporary, &path));
            if let Err(error) = written {
                let _ = fs::remove_file(temporary);
                return Err(error.into());
            }
        }
        _ => bail!("Invalid update policy command"),
    }
    let policy = update_policy(session)?;
    Ok(format!(
        "Modo: {policy}\nmanual: actualizar sin reiniciar.\nrestart: verificar y reiniciar automáticamente.\nreload: recarga de vistas del socket; código Rust nuevo requiere restart."
    ))
}

pub fn activation_pending() -> bool {
    let Ok(running) = std::env::var("MATTER_WA_RUNNING_REVISION") else {
        return false;
    };
    Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(root())
        .output()
        .is_ok_and(|result| {
            result.status.success() && String::from_utf8_lossy(&result.stdout).trim() != running
        })
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
        .take(1100)
        .collect();
    let body = if body.trim().is_empty() {
        "Estado: Fallido\nMotivo: actualización sin resultado verificable.".into()
    } else {
        body
    };
    (
        format!("{}\nDescarga: sin detener el bot.", body.trim()),
        state,
    )
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Pending {
    chat: String,
    plain: bool,
    #[serde(default)]
    update: Option<String>,
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

pub fn request_restart(
    session: &Path,
    chat: &str,
    plain: bool,
    update: Option<&str>,
) -> Result<()> {
    let (request, result) = paths(session)?;
    if !valid_chat(chat) || result.exists() {
        bail!("Invalid restart request");
    }
    let data = serde_json::to_vec(&Pending {
        chat: chat.to_owned(),
        plain,
        update: update.map(str::to_owned),
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
        if !meta.is_file() || meta.len() > 8192 {
            bail!("Invalid private maintenance report");
        }
    }
    let pending: Pending = serde_json::from_slice(&fs::read(request)?)?;
    if !valid_chat(&pending.chat) {
        bail!("Invalid maintenance destination");
    }
    let status = fs::read_to_string(result)?;
    let (verification, body) = match status.trim() {
        "verified" | "verified-built" | "verified-reused" => (
            "Estado: OK\nFormato: OK\nPruebas: OK\nClippy: OK\nCompilación: OK (verificada).",
            "Estado: Completado\nControlador: activo\nWhatsApp: conectado\nSesión: conservada.",
        ),
        "build-failed" => (
            "Estado: Fallido\nAlguna verificación o compilación fue rechazada.",
            "Estado: Fallido\nWhatsApp: conectado posteriormente.",
        ),
        "start-failed" => (
            "Estado: Arranque rechazado.",
            "Estado: Fallido\nWhatsApp: conectado posteriormente.",
        ),
        _ => bail!("Unknown maintenance result"),
    };
    let message = match pending.update.as_deref() {
        Some(update) => {
            let update = update
                .lines()
                .filter(|line| !line.starts_with("Validación y activación pendientes"))
                .collect::<Vec<_>>()
                .join("\n");
            update_message(&update, verification, body, pending.plain)?
        }
        None => styled_sections(&[("Verificación", verification), ("Restart", body)], pending.plain)?,
    };
    Ok(Some((pending.chat, message)))
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
    fn update_policy_persists_without_implicit_restart() {
        let directory = std::env::temp_dir().join(format!(
            "matter-update-policy-test-{}",
            std::process::id()
        ));
        fs::create_dir(&directory).unwrap();
        let session = directory.join("session.db");
        assert_eq!(update_policy(&session).unwrap(), "manual");
        set_update(&session, "=/>set update restart").unwrap();
        assert_eq!(update_policy(&session).unwrap(), "restart");
        set_update(&session, "=/>set update reload").unwrap();
        assert_eq!(update_policy(&session).unwrap(), "reload");
        assert!(set_update(&session, "=/>set update arbitrary").is_err());
        set_update(&session, "=/>set update none").unwrap();
        assert_eq!(update_policy(&session).unwrap(), "manual");
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn maintenance_style_and_destination() {
        assert_eq!(
            styled("Update", "Estado: OK", true).unwrap(),
            "Update\nEstado: OK"
        );
        let matter = styled("Update", "Estado: OK", false).unwrap();
        assert_eq!(
            matter,
            "╭─ 💠 𝗨𝗽𝗱𝗮𝘁𝗲\n│·♪·╚[ Estado: OK\n╰───────────⟨ 🌀"
        );
        assert!(styled("Restart", "Estado: OK", false)
            .unwrap()
            .starts_with("╭─ 💠 𝗥𝗲𝘀𝘁𝗮𝗿𝘁"));
        assert!(styled("Update", "Estado:\u{0000}", false).is_err());
        let report = update_message(
            "Estado: OK\nCambios (máx. 5):\n• abc cambio",
            "Estado: Pendiente",
            "Estado: Manual",
            false,
        )
        .unwrap();
        assert!(report.contains("𝗖𝗮𝗺𝗯𝗶𝗼𝘀"));
        assert!(report.contains("𝗩𝗲𝗿𝗶𝗳𝗶𝗰𝗮𝗰𝗶ó𝗻"));
        assert!(report.ends_with("╰───────────⟨ 🌀"));
        assert!(valid_chat("12345-67890@g.us"));
        assert!(!valid_chat("123@newsletter"));
        assert!(!valid_chat("123@lid\n"));
    }
}
