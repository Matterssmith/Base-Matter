/*
 * Base-Matter — owner-only WhatsApp control plane, separate Rust process.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{anyhow, bail, Result};
use matter_base::{
    activation::SocketActivationStore, control::ControlDispatcher, BotRegistry, CommandAuthorizer,
    CommandInvocation, CommandRouter, ControlExtensionRegistry, InboundReplayStore,
    WhatsAppBridgeConfig, WhatsAppControlBridge, WhatsAppControlIpc,
};
use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
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
const CONTROL_LEASE_MARKER: &[u8] = b"Base-Matter:WhatsApp-Control-Lease:v2\n";

struct ExclusiveLock {
    _file: File,
}
impl ExclusiveLock {
    fn existing(path: &PathBuf) -> Result<File> {
        let metadata = fs::symlink_metadata(path)?;
        if !metadata.file_type().is_file() || metadata.len() != CONTROL_LEASE_MARKER.len() as u64 {
            bail!("Legacy or unsafe control lock requires verified recovery");
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if metadata.permissions().mode() & 0o077 != 0 {
                bail!("Control lock must be private");
            }
        }
        let mut file = OpenOptions::new().read(true).write(true).open(path)?;
        let mut marker = Vec::new();
        file.read_to_end(&mut marker)?;
        if marker != CONTROL_LEASE_MARKER {
            bail!("Legacy or unsafe control lock requires verified recovery");
        }
        Ok(file)
    }

    fn check_available(path: &PathBuf) -> Result<()> {
        match fs::symlink_metadata(path) {
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(_) => bail!("Control lock cannot be inspected"),
            Ok(_) => {}
        };
        let file = Self::existing(path)?;
        file.try_lock()
            .map_err(|_| anyhow!("Control plane already owned by another process"))?;
        Ok(())
    }

    fn acquire(path: PathBuf) -> Result<Self> {
        let mut options = OpenOptions::new();
        options.read(true).write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let (mut file, fresh) = match options.open(&path) {
            Ok(file) => (file, true),
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
                (Self::existing(&path)?, false)
            }
            Err(_) => bail!("Control lock could not be opened"),
        };
        file.try_lock()
            .map_err(|_| anyhow!("Control plane already owned by another process"))?;
        if fresh {
            file.write_all(CONTROL_LEASE_MARKER)?;
            file.sync_all()?;
        }
        Ok(Self { _file: file })
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
        ExclusiveLock::check_available(&activation_path.with_extension("controller.lock"))?;
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
    let extensions = Arc::new(ControlExtensionRegistry::new());
    // A signed, socket-owner-authorized request for live provider discovery.
    // Mutations are intentionally unsupported in this alpha checkpoint.
    extensions.register("start", &[], |_, arguments| {
        if !arguments.is_empty() {
            bail!("Community changes require an approved plan");
        }
        Ok("Community REGISTER read-only inventory authorized".to_owned())
    })?;
    let dispatcher = Arc::new(
        ControlDispatcher::new(activation, Arc::new(BotRegistry::in_memory()), router, None)
            .with_control_extensions(extensions),
    );
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

#[cfg(test)]
mod lock_tests {
    use super::*;

    #[test]
    fn controller_lock_is_exclusive_and_created_private() {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = env::temp_dir().join(format!(
            "base-matter-control-lock-{}-{nonce}",
            std::process::id()
        ));
        let lease = ExclusiveLock::acquire(path.clone()).unwrap();
        assert!(ExclusiveLock::acquire(path.clone()).is_err());
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = fs::metadata(&path).unwrap().permissions().mode();
            assert_eq!(mode & 0o077, 0, "Control lock must be private at creation");
        }
        assert!(ExclusiveLock::check_available(&path).is_err());
        drop(lease);
        assert!(path.exists());
        ExclusiveLock::check_available(&path).unwrap();
        let resumed = ExclusiveLock::acquire(path.clone()).unwrap();
        assert!(ExclusiveLock::acquire(path.clone()).is_err());
        drop(resumed);
        fs::remove_file(path).unwrap();
    }
}
