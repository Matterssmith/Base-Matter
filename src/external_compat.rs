/*
 * Base-Matter — third-party #matter syntax conversion only.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Result};

/// Parses an external bot's voluntary prefix without authenticating it.
/// The caller MUST verify provider, actor, socket and consent, then route the
/// resulting canonical command through Base-Matter's authenticated ingress.
/// This parser NEVER dispatches a command or accesses a session.
pub fn canonical_external_matter(message: &str) -> Result<Option<String>> {
    let Some(suffix) = message.strip_prefix("#matter") else {
        return Ok(None);
    };
    if message.len() > 4096 || message.chars().any(char::is_control) {
        bail!("Invalid external command");
    }
    if !suffix.is_empty() && !suffix.starts_with(' ') && !suffix.starts_with('\t') {
        return Ok(None);
    }
    let body = suffix.trim();
    if body.split_whitespace().count() > 16 {
        bail!("External command arguments exceed limit");
    }
    Ok(Some(format!(
        "=/>{}",
        if body.is_empty() { "status" } else { body }
    )))
}
