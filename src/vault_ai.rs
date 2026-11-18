// Base-Matter — Vault-owned AI execution and credential boundary.
// The client sends an authorized request to a local Unix socket. It never
// obtains provider API keys, endpoints, or decrypted Vault records.
use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};
use std::env;

const MAX_PROMPT_BYTES: usize = 2400;
const MAX_REPLY_BYTES: usize = 8192;
const MAX_TEXT_BYTES: usize = 2200;

#[derive(Debug, Serialize)]
struct Request<'a> {
    protocol: u32,
    nonce: &'a str,
    capability: &'a str,
    policy_required: bool,
    socket_id: &'a str,
    actor_id: &'a str,
    prompt: &'a str,
}

#[derive(Debug, Deserialize)]
struct Response {
    protocol: u32,
    nonce: String,
    approved: bool,
    #[serde(default)]
    policy_applied: bool,
    #[serde(default)]
    grant_id: Option<String>,
    #[serde(default)]
    grant_revision: Option<u64>,
    #[serde(default)]
    capability: Option<String>,
    #[serde(default)]
    provider: Option<String>,
    #[serde(default)]
    text: Option<String>,
}

pub fn provider_name(name: &str) -> bool {
    let len = name.len();
    (2..=64).contains(&len)
        && name.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

fn validate_request(capability: &str, socket: &str, actor: &str, prompt: &str) -> Result<()> {
    if !matches!(capability, "coding-assistant" | "menu-guidance") {
        bail!("Unsupported Vault AI capability");
    }
    for identifier in [socket, actor] {
        if identifier.is_empty()
            || identifier.len() > 128
            || !identifier.bytes().all(|b| b.is_ascii_alphanumeric() || b"._:-@".contains(&b))
        {
            bail!("Invalid Vault AI request identity");
        }
    }
    if prompt.trim().is_empty()
        || prompt.len() > MAX_PROMPT_BYTES
        || prompt.chars().any(|c| c.is_control() && c != '\n' && c != '\t')
    {
        bail!("Invalid Vault AI prompt");
    }
    Ok(())
}

// The Vault worker must verify the owner-granted usage license, its
// revocation and expiry, apply the policy text to the model before each call,
// and enforce permissions in code. The AI response alone is not a grant.
// Old/legacy workers without a policy receipt are rejected.
fn validate_receipt(receipt: &Response, expected_nonce: &str, capability: &str) -> Result<()> {
    let valid_grant = receipt.grant_id.as_deref().is_some_and(|grant| {
        (4..=96).contains(&grant.len())
            && grant.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    });
    if receipt.protocol != 1
        || receipt.nonce != expected_nonce
        || !receipt.approved
        || !receipt.policy_applied
        || !valid_grant
        || receipt.grant_revision.unwrap_or_default() == 0
        || receipt.capability.as_deref() != Some(capability)
    {
        bail!("Vault AI policy or permission receipt invalid; no answer accepted");
    }
    Ok(())
}

pub fn configured() -> bool {
    env::var_os("MATTER_VAULT_AI_SOCKET").is_some()
        && env::var("MATTER_VAULT_AI_UID")
            .ok()
            .and_then(|value| value.parse::<u32>().ok())
            .is_some()
}

pub fn ask(capability: &str, socket: &str, actor: &str, prompt: &str) -> Result<(String, String)> {
    validate_request(capability, socket, actor, prompt)?;
    #[cfg(not(unix))]
    {
        bail!("Vault AI requires a protected local Unix-socket gateway")
    }
    #[cfg(unix)]
    {
        call_unix(capability, socket, actor, prompt)
    }
}

#[cfg(unix)]
fn call_unix(capability: &str, socket: &str, actor: &str, prompt: &str) -> Result<(String, String)> {
    use std::fs;
    use std::io::{Read, Write};
    use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
    use std::os::unix::net::UnixStream;
    use std::path::{Component, Path};
    use std::time::Duration;

    let location = env::var("MATTER_VAULT_AI_SOCKET")
        .map_err(|_| anyhow!("Vault AI is not configured"))?;
    let path = Path::new(&location);
    if !path.is_absolute() || location.len() > 240 {
        bail!("Vault AI socket must be an absolute local path");
    }
    if path.components().any(|part| matches!(part, Component::ParentDir)) {
        bail!("Vault AI socket path traversal denied");
    }
    let expected_uid = env::var("MATTER_VAULT_AI_UID")
        .map_err(|_| anyhow!("Vault AI expected Unix UID not configured"))?
        .parse::<u32>()
        .map_err(|_| anyhow!("Vault AI expected UID invalid"))?;
    let metadata = fs::symlink_metadata(path)
        .map_err(|_| anyhow!("Vault AI gateway unavailable"))?;
    if !metadata.file_type().is_socket()
        || metadata.file_type().is_symlink()
        || metadata.permissions().mode() & 0o077 != 0
        || metadata.uid() != expected_uid
    {
        bail!("Vault AI gateway socket is not private");
    }
    let parent = path.parent().ok_or_else(|| anyhow!("Vault AI socket parent missing"))?;
    let parent_metadata = fs::symlink_metadata(parent)?;
    if !parent_metadata.file_type().is_dir()
        || parent_metadata.file_type().is_symlink()
        || parent_metadata.permissions().mode() & 0o077 != 0
        || parent_metadata.uid() != expected_uid
    {
        bail!("Vault AI gateway directory is not private");
    }
    let mut random = [0_u8; 16];
    getrandom::fill(&mut random).map_err(|_| anyhow!("Vault AI nonce unavailable"))?;
    let nonce = hex::encode(random);
    let serialized = serde_json::to_vec(&Request {
        protocol: 1,
        nonce: &nonce,
        capability,
        policy_required: true,
        socket_id: socket,
        actor_id: actor,
        prompt,
    })?;
    if serialized.len() > 4096 {
        bail!("Vault AI request too large");
    }
    let mut connection =
        UnixStream::connect(path).map_err(|_| anyhow!("Vault AI gateway unreachable"))?;
    connection.set_read_timeout(Some(Duration::from_secs(28)))?;
    connection.set_write_timeout(Some(Duration::from_secs(3)))?;
    connection.write_all(&serialized)?;
    connection.shutdown(std::net::Shutdown::Write)?;
    let mut bytes = Vec::new();
    connection
        .take((MAX_REPLY_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| anyhow!("Vault AI response timeout"))?;
    if bytes.is_empty() || bytes.len() > MAX_REPLY_BYTES {
        bail!("Invalid Vault AI response size");
    }
    let response: Response = serde_json::from_slice(&bytes)
        .map_err(|_| anyhow!("Invalid Vault AI response format"))?;
    validate_receipt(&response, &nonce, capability)?;
    let provider = response.provider.ok_or_else(|| anyhow!("Vault AI provider missing"))?;
    if !provider_name(&provider) {
        bail!("Invalid Vault AI provider identity");
    }
    let text = response.text.ok_or_else(|| anyhow!("Vault AI answer missing"))?;
    if text.trim().is_empty()
        || text.len() > MAX_TEXT_BYTES
        || text.chars().any(|c| c.is_control() && c != '\n' && c != '\t')
    {
        bail!("Invalid Vault AI answer");
    }
    Ok((provider, text))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_ids_are_stable_and_not_endpoints() {
        assert!(provider_name("claude"));
        assert!(provider_name("venice"));
        assert!(provider_name("openai"));
        assert!(provider_name("alya-core"));
        assert!(provider_name("future-provider-32"));
        for invalid in ["", "x", "OPENAI", "https://api.example.com", "../keys", "test env"] {
            assert!(!provider_name(invalid));
        }
    }

    #[test]
    fn no_ai_answer_without_owner_grant_policy_receipt() {
        let mut receipt = Response {
            protocol: 1,
            nonce: "ab12".to_owned(),
            approved: true,
            policy_applied: false,
            grant_id: Some("owner-grant-001".to_owned()),
            grant_revision: Some(3),
            capability: Some("coding-assistant".to_owned()),
            provider: Some("claude".to_owned()),
            text: Some("Response".to_owned()),
        };
        assert!(validate_receipt(&receipt, "ab12", "coding-assistant").is_err());
        receipt.policy_applied = true;
        assert!(validate_receipt(&receipt, "ab12", "coding-assistant").is_ok());
        assert!(validate_receipt(&receipt, "wrong", "coding-assistant").is_err());
        assert!(validate_receipt(&receipt, "ab12", "host-execution").is_err());
        receipt.grant_revision = Some(0);
        assert!(validate_receipt(&receipt, "ab12", "coding-assistant").is_err());
        receipt.grant_revision = Some(3);
        receipt.approved = false;
        assert!(validate_receipt(&receipt, "ab12", "coding-assistant").is_err());
    }

    #[test]
    fn request_rejects_privilege_and_invalid_identities() {
        assert!(validate_request("coding-assistant", "socket-1", "actor-1", "Review Rust").is_ok());
        assert!(validate_request("menu-guidance", "socket-1", "actor-1", "help").is_ok());
        assert!(validate_request("host-exec", "socket-1", "actor-1", "ls").is_err());
        assert!(validate_request("coding-assistant", "socket/1", "actor-1", "help").is_err());
        assert!(validate_request("coding-assistant", "socket-1", "actor-1", "").is_err());
        assert!(validate_request("coding-assistant", "socket-1", "actor-1", &"x".repeat(2500)).is_err());
    }
}
