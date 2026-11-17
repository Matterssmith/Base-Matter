/*
 * Base-Matter — owner-only WhatsApp control plane, separate Rust process.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{anyhow, bail, Result};
use matter_base::{
    activation::SocketActivationStore, control::ControlDispatcher, BotRegistry, CommandAuthorizer,
    CommandInvocation, CommandRouter, InboundReplayStore, WhatsAppBridgeConfig,
    WhatsAppControlBridge, WhatsAppControlIpc,
};
use std::{
    env,
    fs::{self, File, OpenOptions},
    io::Write,
    net::{Ipv4Addr, SocketAddrV4},
    path::{Component, PathBuf},
    sync::Arc,
};
use zeroize::Zeroizing;

struct DenyBot;
impl CommandAuthorizer for DenyBot {
    fn authorize(&self, _: &CommandInvocation) -> Result<()> {
        bail!("Bot-Matter cannot run from the global controller")
    }
}
struct ExclusiveLock {
    path: PathBuf,
    _file: File,
}
impl ExclusiveLock {
    fn acquire(path: PathBuf) -> Result<Self> {
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&path)
            .map_err(|_| anyhow!("Control plane already owned or recovery required"))?;
        writeln!(file, "{}", std::process::id())?;
        file.sync_all()?;
        Ok(Self { path, _file: file })
    }
}
impl Drop for ExclusiveLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}
fn required(name: &str) -> Result<Zeroizing<String>> {
    let value = Zeroizing::new(env::var(name).map_err(|_| anyhow!("Missing controller setting"))?);
    if value.is_empty() {
        bail!("Empty controller setting");
    }
    Ok(value)
}
fn key(name: &str) -> Result<[u8; 32]> {
    let text = required(name)?;
    if text.len() != 64 || !text.bytes().all(|b| b.is_ascii_hexdigit()) {
        bail!("Controller key must contain 32 hex bytes");
    }
    let mut result = [0_u8; 32];
    hex::decode_to_slice(text.as_bytes(), &mut result)
        .map_err(|_| anyhow!("Controller key invalid"))?;
    if result.iter().all(|b| *b == 0) {
        bail!("Controller key is zero");
    }
    Ok(result)
}
fn private_database(name: &str) -> Result<PathBuf> {
    let path = PathBuf::from(required(name)?.as_str());
    if !path.is_absolute()
        || path.components().any(|c| matches!(c, Component::ParentDir))
        || path.file_name().is_none()
    {
        bail!("Controller database path must be absolute");
    }
    let parent = path
        .parent()
        .ok_or_else(|| anyhow!("Missing controller directory"))?;
    let metadata = fs::symlink_metadata(parent)?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        bail!("Controller directory must be ordinary");
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if metadata.permissions().mode() & 0o077 != 0 {
            bail!("Controller directory must be owner-only");
        }
    }
    if let Ok(info) = fs::symlink_metadata(&path) {
        if !info.is_file() || info.file_type().is_symlink() {
            bail!("Controller database must be a regular file");
        }
    }
    Ok(path)
}
fn main() -> Result<()> {
    let action = env::args().nth(1).unwrap_or_else(|| "--help".into());
    if action == "--help" {
        println!("Base-Matter WhatsApp control: --check, --init once, then --serve on loopback. No QR, ping, or Bot-Matter runtime.");
        return Ok(());
    }
    if action != "--init" && action != "--serve" && action != "--check" {
        bail!("Unknown controller operation");
    }
    let socket = required("MATTER_WA_SOCKET_ID")?.to_string();
    let owner = required("MATTER_WA_OWNER_ID")?.to_string();
    let jid = required("MATTER_WA_OWNER_JID")?.to_string();
    let activation_path = private_database("MATTER_WA_ACTIVATION_DB")?;
    let replay_path = private_database("MATTER_WA_REPLAY_DB")?;
    if activation_path == replay_path {
        bail!("Control and replay DB must differ");
    }
    let bind: SocketAddrV4 = required("MATTER_WA_BRIDGE_ADDR")?
        .parse()
        .map_err(|_| anyhow!("Invalid control listener address"))?;
    if *bind.ip() != Ipv4Addr::LOCALHOST || bind.port() == 0 {
        bail!("Only 127.0.0.1 is permitted for control listener");
    }
    let bridge_key = key("MATTER_WA_BRIDGE_KEY_HEX")?;
    let replay_key = key("MATTER_WA_REPLAY_KEY_HEX")?;
    if bridge_key == replay_key {
        bail!("Transport authentication and replay keys must differ");
    }
    if action == "--check" {
        // Verify configuration without creating SQLite, locks or a listener.
        let jid_user = jid.split('@').next().unwrap_or("");
        if socket.is_empty()
            || owner.is_empty()
            || socket.len() > 128
            || owner.len() > 128
            || [socket.as_str(), owner.as_str()].iter().any(|value| {
                !value
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"._:-".contains(&b))
            })
            || jid_user.is_empty()
            || jid.len() > 128
            || !(jid.ends_with("@s.whatsapp.net") || jid.ends_with("@lid"))
            || !jid_user
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"._:-".contains(&b))
        {
            bail!("Invalid owner or socket configuration");
        }
        println!("Base-Matter control settings checked; no session was opened.");
        return Ok(());
    }
    let activation = Arc::new(SocketActivationStore::open(&activation_path)?);
    if action == "--init" {
        activation.provision(&socket, &owner)?;
        println!("Control socket provisioned; no live WhatsApp connection opened.");
        return Ok(());
    }
    // Socket must already exist and belong to this authenticated host owner.
    activation.audit_history(&socket, &owner)?;
    let lock = activation_path.with_extension("controller.lock");
    let _lease = ExclusiveLock::acquire(lock)?;
    let replay = Arc::new(InboundReplayStore::open(&replay_path, replay_key)?);
    let router = Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(DenyBot))?);
    let dispatcher = Arc::new(ControlDispatcher::new(
        activation,
        Arc::new(BotRegistry::in_memory()),
        router,
        None,
    ));
    let bridge = Arc::new(WhatsAppControlBridge::new(
        WhatsAppBridgeConfig {
            socket_id: socket,
            owner_actor_id: owner,
            owner_jid: jid,
        },
        bridge_key,
        replay,
        dispatcher,
    )?);
    let service = WhatsAppControlIpc::new(bridge);
    println!("Base-Matter owner control listening on local loopback.");
    service.serve(bind)
}
