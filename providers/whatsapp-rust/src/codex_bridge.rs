// Private, fixed-protocol broker; no model, shell or Git credentials in WhatsApp.
// Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Pending {
    id: String,
    chat: String,
    plain: bool,
    seen: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Reply {
    state: String,
    pr: Option<u64>,
    #[serde(default)]
    files: Vec<String>,
    #[serde(default)]
    commit: Option<String>,
}

fn path(session: &Path) -> Result<PathBuf> {
    Ok(session
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Missing private state"))?
        .join("codex-pending.json"))
}

fn valid_private(chat: &str) -> bool {
    chat.split_once('@').is_some_and(|(id, domain)| {
        (8..=20).contains(&id.len())
            && id.bytes().all(|b| b.is_ascii_digit())
            && matches!(domain, "lid" | "s.whatsapp.net")
    })
}

fn read(session: &Path) -> Result<Option<Pending>> {
    let path = path(session)?;
    let meta = match fs::symlink_metadata(&path) {
        Ok(meta) => meta,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e.into()),
    };
    if !meta.is_file() || meta.len() > 4096 {
        bail!("Invalid Codex pending state");
    }
    let pending: Pending = serde_json::from_slice(&fs::read(path)?)?;
    if !valid_private(&pending.chat)
        || pending.id.len() != 32
        || !pending.id.bytes().all(|b| b.is_ascii_hexdigit())
    {
        bail!("Invalid Codex pending identity");
    }
    Ok(Some(pending))
}

fn save(session: &Path, pending: &Pending, create: bool) -> Result<()> {
    let path = path(session)?;
    let target = if create {
        path.clone()
    } else {
        path.with_extension("tmp")
    };
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(&target)?;
    let data = serde_json::to_vec(pending)?;
    let result = file.write_all(&data).and_then(|()| file.sync_all());
    if let Err(e) = result {
        let _ = fs::remove_file(target);
        return Err(e.into());
    }
    if !create {
        fs::rename(target, path)?;
    }
    Ok(())
}

#[cfg(unix)]
fn request(value: serde_json::Value) -> Result<Reply> {
    use std::{io::Read, net::Shutdown, os::unix::net::UnixStream, time::Duration};
    let mut stream = UnixStream::connect("/run/matter-codex/worker.sock")?;
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    stream.set_write_timeout(Some(Duration::from_secs(3)))?;
    stream.write_all(&serde_json::to_vec(&value)?)?;
    stream.shutdown(Shutdown::Write)?;
    let mut bytes = Vec::new();
    stream.take(4097).read_to_end(&mut bytes)?;
    if bytes.len() > 4096 {
        bail!("Codex reply too large");
    }
    Ok(serde_json::from_slice(&bytes)?)
}

#[cfg(not(unix))]
fn request(_value: serde_json::Value) -> Result<Reply> {
    bail!("Codex worker requires Unix IPC")
}

fn report(reply: &Reply) -> Result<String> {
    let description = match reply.state.as_str() {
        "queued" => "En cola",
        "running" => "Codex trabajando",
        "review" => "Pull request preparado; pendiente de revisión",
        "ready" => "Integrado en alpha; ya puedes ejecutar =/>update",
        "failed" => "Fallido; no se confirma actualización",
        "no_changes" => "Sin cambios",
        "closed" => "Pull request cerrado sin integrar",
        "missing" => "Tarea no encontrada; no se confirma ejecución",
        _ => bail!("Unknown Codex status"),
    };
    let mut body = format!("Estado: {description}\nRepositorio: Base-Matter\nRama objetivo: alpha");
    if let Some(commit) = reply
        .commit
        .as_deref()
        .filter(|value| value.len() == 40 && value.bytes().all(|b| b.is_ascii_hexdigit()))
    {
        body.push_str(&format!("\nCommit: {}", &commit[..12]));
    }
    for file in reply.files.iter().take(5) {
        let safe: String = file.chars().filter(|c| !c.is_control()).take(100).collect();
        if !safe.is_empty() {
            body.push_str(&format!("\nCambio: {safe}"));
        }
    }
    if let Some(pr) = reply.pr.filter(|pr| *pr > 0) {
        body.push_str(&format!(
            "\nPR: https://github.com/Matterssmith/Base-Matter/pull/{pr}"
        ));
    }
    Ok(body)
}

pub fn command(session: &Path, chat: &str, instruction: &str, plain: bool) -> Result<String> {
    if !valid_private(chat) {
        bail!("Codex requires private chat");
    }
    if let Some(pending) = read(session)? {
        if pending.chat != chat {
            bail!("A previous private task is pending");
        }
        if instruction != "status" {
            return Ok("Estado: Tarea pendiente\nConsulta: =/>codex status".into());
        }
        return report(&request(
            serde_json::json!({"op":"status","id":pending.id}),
        )?);
    }
    if instruction == "status" {
        return Ok("Estado: Sin tareas pendientes".into());
    }
    if instruction.len() > 1200
        || instruction.is_empty()
        || instruction.chars().any(|c| c.is_control() && c != '\n')
    {
        bail!("Invalid Codex instruction");
    }
    // Do not strand a private job when the worker has not been provisioned.
    request(serde_json::json!({"op":"ping"}))?;
    let mut random = [0u8; 16];
    getrandom::fill(&mut random).map_err(|_| anyhow::anyhow!("Random ID unavailable"))?;
    let pending = Pending {
        id: hex::encode(random),
        chat: chat.into(),
        plain,
        seen: String::new(),
    };
    save(session, &pending, true)?;
    let reply = request(serde_json::json!({
        "op":"submit","id":pending.id,"instruction":instruction,
    }));
    match reply {
        Ok(reply) => report(&reply),
        Err(_) => Ok(
            "Estado: Envío no confirmado\nConsulta: =/>codex status\nNo se confirma ningún cambio."
                .into(),
        ),
    }
}

pub fn notice(session: &Path) -> Result<Option<(String, String, String)>> {
    let Some(pending) = read(session)? else {
        return Ok(None);
    };
    let reply = request(serde_json::json!({"op":"status","id":pending.id}))?;
    if pending.seen == reply.state {
        return Ok(None);
    }
    let body = super::maintenance::styled("Codex", &report(&reply)?, pending.plain)?;
    Ok(Some((
        pending.chat,
        body,
        format!("{}:{}", pending.id, reply.state),
    )))
}

pub fn ack_notice(session: &Path, marker: &str) -> Result<()> {
    let Some(mut pending) = read(session)? else {
        return Ok(());
    };
    let Some((id, state)) = marker.split_once(':') else {
        bail!("Invalid acknowledgement");
    };
    if pending.id != id {
        bail!("Codex task changed");
    }
    if matches!(
        state,
        "ready" | "failed" | "no_changes" | "closed" | "missing"
    ) {
        fs::remove_file(path(session)?)?;
    } else {
        pending.seen = state.into();
        save(session, &pending, false)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_only_and_ready_requires_worker_state() {
        assert!(valid_private("11111111111@lid"));
        assert!(!valid_private("11111111111@g.us"));
        let reply = Reply {
            state: "review".into(),
            pr: Some(42),
            files: vec![],
            commit: None,
        };
        assert!(!report(&reply).unwrap().contains("=/>update"));
        let reply = Reply {
            state: "ready".into(),
            pr: Some(42),
            files: vec![],
            commit: None,
        };
        assert!(report(&reply).unwrap().contains("=/>update"));
    }
}
