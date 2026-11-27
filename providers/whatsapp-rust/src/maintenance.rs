// Fixed maintenance operations; no shell text or Git arguments come from WhatsApp.
// Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    time::{Instant, SystemTime, UNIX_EPOCH},
};
use whatsapp_matter_rust_provider::classic_template::{self, ReportBlock, ReportRow};

const REQUEST: &str = "maintenance-restart.pending.json";
const RESULT: &str = "maintenance-restart.result";

fn styled_sections(sections: &[(&str, &str)], plain: bool) -> Result<String> {
    if sections
        .iter()
        .any(|(_, body)| body.is_empty() || body.chars().any(|c| c.is_control() && c != '\n'))
    {
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

/// The execution path, not the wording of a notice, decides which blocks exist.
pub enum UpdateAction {
    None,
    Note(String),
    Reload(String),
    Restart(String),
}

fn render_update_blocks(blocks: &[ReportBlock<'_>], plain: bool) -> Result<String> {
    for block in blocks {
        for row in &block.rows {
            if std::iter::once(row.text)
                .chain(row.children.iter().copied())
                .any(|line| line.chars().any(char::is_control))
            {
                bail!("Invalid maintenance row");
            }
        }
    }
    let text = if plain {
        let mut sections = Vec::new();
        for block in blocks {
            let mut lines = vec![block.title.to_owned()];
            for row in &block.rows {
                lines.push(row.text.to_owned());
                lines.extend(row.children.iter().map(|line| format!("  {line}")));
            }
            sections.push(lines.join("\n"));
        }
        sections.join("\n\n")
    } else {
        classic_template::render_report_blocks(blocks)
            .ok_or_else(|| anyhow::anyhow!("Invalid maintenance layout"))?
    };
    if text.len() > 3800 {
        bail!("Maintenance report exceeds transport limit");
    }
    Ok(text)
}

fn update_blocks<'a>(
    body: &'a str,
    verification: Option<&'a str>,
    action: &'a UpdateAction,
    plain: bool,
) -> Vec<ReportBlock<'a>> {
    let mut summary = Vec::new();
    let mut changes = Vec::new();
    let mut tail = Vec::new();
    let mut reading_changes = false;
    let mut applied = false;
    for line in body.lines() {
        if line == "Cambios (máx. 5):" {
            reading_changes = true;
        } else if let Some(change) = line.strip_prefix("• ").filter(|_| reading_changes) {
            changes.push(change);
        } else {
            reading_changes = false;
            if let Some(count) = line.strip_prefix("Commits aplicados: ") {
                applied = count.trim().parse::<u64>().is_ok_and(|count| count > 0);
            }
            if line.starts_with("Bot:") || line.starts_with("Duración:") {
                tail.push(line);
            } else {
                summary.push(line);
            }
        }
    }
    let mut rows = summary.into_iter().map(ReportRow::line).collect::<Vec<_>>();
    if applied {
        if !changes.is_empty() {
            rows.push(ReportRow {
                text: if plain {
                    "Cambios"
                } else {
                    classic_template::report_section_label("changes").unwrap_or("Cambios")
                },
                children: changes,
            });
        }
        if let Some(verification) = verification.filter(|value| !value.trim().is_empty()) {
            rows.push(ReportRow {
                text: if plain {
                    "Verificación"
                } else {
                    classic_template::report_section_label("verification").unwrap_or("Verificación")
                },
                children: verification.lines().collect(),
            });
        }
    }
    if let UpdateAction::Note(note) = action {
        rows.extend(note.lines().map(ReportRow::line));
    }
    rows.extend(tail.into_iter().map(ReportRow::line));
    let mut blocks = vec![ReportBlock {
        title: "Update",
        rows,
    }];
    match action {
        UpdateAction::Reload(body) => blocks.push(ReportBlock {
            title: "Reload",
            rows: body.lines().map(ReportRow::line).collect(),
        }),
        UpdateAction::Restart(body) => blocks.push(ReportBlock {
            title: "Restart",
            rows: body.lines().map(ReportRow::line).collect(),
        }),
        UpdateAction::None | UpdateAction::Note(_) => {}
    }
    blocks
}

pub fn update_message(
    body: &str,
    verification: Option<&str>,
    action: &UpdateAction,
    plain: bool,
) -> Result<String> {
    render_update_blocks(&update_blocks(body, verification, action, plain), plain)
}

pub fn update_console_rows(
    body: &str,
    verification: Option<&str>,
    action: &UpdateAction,
) -> String {
    update_blocks(body, verification, action, false)
        .iter()
        .map(|block| classic_template::report_rows_with_close(&block.rows, false))
        .collect::<Vec<_>>()
        .join("")
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
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok("reload".into()),
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
        "Modo: {policy}\nmanual: actualizar sin reiniciar.\nrestart: verificar y reiniciar automáticamente.\nreload (predeterminado): recarga de vistas del socket; código Rust nuevo requiere restart."
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
    let started = Instant::now();
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
    let body = if output.status.code() == Some(0) {
        format!(
            "{}\nDuración: {:.2}s\nBot: Activo",
            body.trim(),
            started.elapsed().as_secs_f64()
        )
    } else {
        format!("{}\nBot: Activo", body.trim())
    };
    (body, state)
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Pending {
    chat: String,
    plain: bool,
    #[serde(default)]
    update: Option<String>,
    #[serde(default)]
    started_ms: Option<u64>,
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
        started_ms: unix_ms(),
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

fn unix_ms() -> Option<u64> {
    u64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()?
            .as_millis(),
    )
    .ok()
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
        "verified-built" => (
            Some("Integridad: OK\nFormato: OK\nPruebas: OK\nClippy: OK\nCompilación: OK"),
            "Estado: Completado\nControlador: activo\nWhatsApp: conectado\nSesión: conservada.",
        ),
        "verified" => (
            Some("Compilación verificada: OK"),
            "Estado: Completado\nControlador: activo\nWhatsApp: conectado\nSesión: conservada.",
        ),
        "verified-reused" => (
            Some("Compilación verificada: OK (reutilizada)"),
            "Estado: Completado\nControlador: activo\nWhatsApp: conectado\nSesión: conservada.",
        ),
        "build-failed" => (
            Some("Verificación o compilación: Fallida"),
            "Estado: Fallido\nWhatsApp: conectado posteriormente.",
        ),
        "start-failed" => (
            Some("Arranque: rechazado"),
            "Estado: Fallido\nWhatsApp: conectado posteriormente.",
        ),
        _ => bail!("Unknown maintenance result"),
    };
    let mut body = body.to_owned();
    if let Some(elapsed) = pending
        .started_ms
        .and_then(|started| unix_ms()?.checked_sub(started))
    {
        body.push_str(&format!("\nDuración: {:.2}s", elapsed as f64 / 1000.0));
    }
    let message = match pending.update.as_deref() {
        Some(update) => {
            let update = update
                .lines()
                .filter(|line| {
                    !line.starts_with("Validación y activación pendientes")
                        && !line.starts_with("Versión en ejecución:")
                        && !line.starts_with("Bot:")
                })
                .map(|line| {
                    if line == "Estado: Código actualizado"
                        && matches!(
                            status.trim(),
                            "verified" | "verified-built" | "verified-reused"
                        )
                    {
                        "Estado: Completado"
                    } else {
                        line
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            update_message(
                &update,
                verification,
                &UpdateAction::Restart(body),
                pending.plain,
            )?
        }
        None => {
            if let Some(verification) = verification {
                body.push('\n');
                body.push_str(verification);
            }
            styled("Restart", &body, pending.plain)?
        }
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
        let directory =
            std::env::temp_dir().join(format!("matter-update-policy-test-{}", std::process::id()));
        fs::create_dir(&directory).unwrap();
        let session = directory.join("session.db");
        assert_eq!(update_policy(&session).unwrap(), "reload");
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
        assert_eq!(matter, "╭─ 💠 𝗨𝗽𝗱𝗮𝘁𝗲\n│·♪·╚[ Estado: OK\n╰───────────⟨ 🌀");
        assert!(styled("Restart", "Estado: OK", false)
            .unwrap()
            .starts_with("╭─ 💠 𝗥𝗲𝘀𝘁𝗮𝗿𝘁"));
        assert!(styled("Update", "Estado:\u{0000}", false).is_err());
        let report = update_message(
            "Estado: Código actualizado\nCommits aplicados: 1\nCambios (máx. 5):\n• abc cambio",
            Some("Integridad: OK"),
            &UpdateAction::Note("Activar: =/>restart".into()),
            false,
        )
        .unwrap();
        assert!(report.contains("│·♪·╠[ 𝗖𝗮𝗺𝗯𝗶𝗼𝘀\n│·♪·　 └ abc cambio"));
        assert!(report.contains("𝗩𝗲𝗿𝗶𝗳𝗶𝗰𝗮𝗰𝗶ó𝗻"));
        assert!(!report.contains("╭─ 💠 𝗥𝗲𝘀𝘁𝗮𝗿𝘁"));
        assert_eq!(report.matches("╭─ 💠").count(), 1);
        assert!(report.ends_with("╰───────────⟨ 🌀"));
        let plain = update_message(
            "Estado: Código actualizado\nCommits aplicados: 1\nCambios (máx. 5):\n• abc cambio",
            Some("Integridad: OK"),
            &UpdateAction::None,
            true,
        )
        .unwrap();
        assert!(plain.contains("Cambios\n  abc cambio"));
        assert!(!plain.contains("💠"));
        assert!(!plain.contains("✅"));
        let current = update_message(
            "Estado: Actualizado\nVersión: abc\nCambios pendientes: 0\nBot: Activo",
            None,
            &UpdateAction::None,
            false,
        )
        .unwrap();
        assert_eq!(
            current,
            "╭─ 💠 𝗨𝗽𝗱𝗮𝘁𝗲\n│·♪·╠[ Estado: ✅ Actualizado\n│·♪·╠[ Versión: abc\n│·♪·╠[ Cambios pendientes: 0\n│·♪·╚[ Bot: Activo\n╰───────────⟨ 🌀"
        );
        let restarted = update_message(
            "Estado: Código actualizado\nCommits aplicados: 1",
            Some("Pruebas: OK"),
            &UpdateAction::Restart("Estado: Completado".into()),
            false,
        )
        .unwrap();
        assert_eq!(restarted.matches("╭─ 💠").count(), 2);
        assert!(restarted.contains("   ││\n╭─ 💠 𝗥𝗲𝘀𝘁𝗮𝗿𝘁"));
        assert!(restarted.ends_with("╰───────────⟨ 🌀"));
        let reloaded = update_message(
            "Estado: Código actualizado\nCommits aplicados: 1",
            Some("Integridad: OK"),
            &UpdateAction::Reload("Estado: Recarga parcial".into()),
            false,
        )
        .unwrap();
        assert!(reloaded.contains("╭─ 💠 𝗥𝗲𝗹𝗼𝗮𝗱"));
        assert!(!reloaded.contains("╭─ 💠 𝗥𝗲𝘀𝘁𝗮𝗿𝘁"));
        let failed = update_message(
            "Estado: Bloqueado\nCommits locales pendientes: 2",
            None,
            &UpdateAction::None,
            false,
        )
        .unwrap();
        assert_eq!(failed.matches("╭─ 💠").count(), 1);
        assert!(!failed.contains("𝗩𝗲𝗿𝗶𝗳𝗶𝗰𝗮𝗰𝗶ó𝗻"));
        assert!(valid_chat("12345-67890@g.us"));
        assert!(!valid_chat("123@newsletter"));
        assert!(!valid_chat("123@lid\n"));
    }
}
