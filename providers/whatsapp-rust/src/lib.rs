/*
 * WhatsApp-Matter — standalone Rust provider, bridge signing and session lease.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{anyhow, bail, Result};
use hmac::{Hmac, Mac};
use serde::Serialize;
use sha2::Sha256;
use std::{
    env,
    fs::{self, File, OpenOptions},
    io::Write,
    net::{Ipv4Addr, SocketAddrV4},
    path::{Component, Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use zeroize::Zeroizing;

const DOMAIN: &[u8] = b"Base-Matter:WhatsApp-Control:v1\0";

pub struct Config {
    pub phone: String,
    pub owner_jid: String,
    pub socket_id: String,
    pub actor_id: String,
    pub session_db: PathBuf,
    pub bridge_addr: SocketAddrV4,
    pub bridge_key: Zeroizing<[u8; 32]>,
}

#[derive(Debug)]
pub struct Event<'a> {
    pub chat_jid: &'a str,
    pub sender_jid: &'a str,
    pub message_id: &'a str,
    pub text: &'a str,
    pub from_me: bool,
    pub is_group: bool,
    pub timestamp: i64,
}

#[derive(Debug, Serialize)]
pub struct SignedFrame {
    pub version: u8,
    pub socket_id: String,
    pub actor_id: String,
    pub owner_jid: String,
    pub chat_jid: String,
    pub message_id: String,
    pub text: String,
    pub from_me: bool,
    pub timestamp: i64,
    pub signature: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Self::from_reader(|name| env::var(name).ok())
    }

    pub fn from_reader(mut get: impl FnMut(&str) -> Option<String>) -> Result<Self> {
        let mut required = |name: &str| {
            let value = get(name).ok_or_else(|| anyhow!("Missing provider configuration"))?;
            if value.is_empty() {
                bail!("Empty provider configuration");
            }
            Ok(value)
        };
        let phone = required("MATTER_WA_PHONE")?;
        if !(8..=15).contains(&phone.len())
            || phone.starts_with('0')
            || !phone.bytes().all(|b| b.is_ascii_digit())
        {
            bail!("Invalid international pairing phone");
        }
        let owner_jid = required("MATTER_WA_OWNER_JID")?;
        let user = owner_jid.split('@').next().unwrap_or("");
        if user.is_empty()
            || owner_jid.len() > 128
            || !(owner_jid.ends_with("@s.whatsapp.net") || owner_jid.ends_with("@lid"))
            || !user
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"._:-".contains(&b))
        {
            bail!("Invalid owner private JID");
        }
        let socket_id = required("MATTER_WA_SOCKET_ID")?;
        let actor_id = required("MATTER_WA_OWNER_ID")?;
        for value in [&socket_id, &actor_id] {
            if value.len() > 128
                || !value
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"._:-".contains(&b))
            {
                bail!("Invalid provider socket or actor ID");
            }
        }
        let session_db = private_path(&required("MATTER_WA_SESSION_DB")?)?;
        let addr: SocketAddrV4 = required("MATTER_WA_BRIDGE_ADDR")?
            .parse()
            .map_err(|_| anyhow!("Invalid loopback bridge address"))?;
        if *addr.ip() != Ipv4Addr::LOCALHOST || addr.port() == 0 {
            bail!("Bridge must be bound to 127.0.0.1 with a nonzero port");
        }
        let raw = Zeroizing::new(required("MATTER_WA_BRIDGE_KEY_HEX")?);
        if raw.len() != 64 || !raw.bytes().all(|b| b.is_ascii_hexdigit()) {
            bail!("Invalid 256-bit bridge key");
        }
        let mut bridge_key = Zeroizing::new([0_u8; 32]);
        hex::decode_to_slice(raw.as_bytes(), &mut bridge_key[..])
            .map_err(|_| anyhow!("Invalid bridge key encoding"))?;
        if bridge_key.iter().all(|b| *b == 0) {
            bail!("Bridge key must not be zero");
        }
        Ok(Self {
            phone,
            owner_jid,
            socket_id,
            actor_id,
            session_db,
            bridge_addr: addr,
            bridge_key,
        })
    }

    /// Signs only recent, self-authored messages in the owner's own private
    /// chat. All identifiers come from the trusted whatsapp-rust callback.
    pub fn signed_control(&self, event: &Event<'_>) -> Result<Option<SignedFrame>> {
        if !event.from_me
            || event.is_group
            || event.chat_jid != self.owner_jid
            || event.sender_jid != self.owner_jid
            || !event.text.starts_with("=/>")
        {
            return Ok(None);
        }
        if event.message_id.is_empty()
            || event.message_id.len() > 256
            || event.message_id.chars().any(char::is_control)
            || event.text.len() > 4096
        {
            bail!("Invalid WhatsApp control data");
        }
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
        if now.abs_diff(event.timestamp) > 120 {
            bail!("Expired WhatsApp control data");
        }
        let mut result = SignedFrame {
            version: 1,
            socket_id: self.socket_id.clone(),
            actor_id: self.actor_id.clone(),
            owner_jid: self.owner_jid.clone(),
            chat_jid: self.owner_jid.clone(),
            message_id: event.message_id.to_owned(),
            text: event.text.to_owned(),
            from_me: true,
            timestamp: event.timestamp,
            signature: String::new(),
        };
        result.signature = sign_frame(&result, &self.bridge_key)?;
        Ok(Some(result))
    }
}

pub fn sign_frame(frame: &SignedFrame, secret: &[u8; 32]) -> Result<String> {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret)
        .map_err(|_| anyhow!("Bridge signature unavailable"))?;
    mac.update(DOMAIN);
    let values = [
        frame.version.to_string(),
        frame.socket_id.clone(),
        frame.actor_id.clone(),
        frame.owner_jid.clone(),
        frame.chat_jid.clone(),
        frame.message_id.clone(),
        frame.text.clone(),
    ];
    for value in &values {
        let bytes = value.as_bytes();
        mac.update(&(u32::try_from(bytes.len())?).to_be_bytes());
        mac.update(bytes);
    }
    mac.update(&[u8::from(frame.from_me)]);
    mac.update(&frame.timestamp.to_be_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}

/// Require a pre-existing private directory; never use a relative database.
pub fn private_path(raw: &str) -> Result<PathBuf> {
    let path = PathBuf::from(raw);
    if !path.is_absolute()
        || path
            .components()
            .any(|part| matches!(part, Component::ParentDir))
        || path.file_name().is_none()
    {
        bail!("Session path must be absolute without parent traversal");
    }
    let parent = path
        .parent()
        .ok_or_else(|| anyhow!("Session directory missing"))?;
    let info = fs::symlink_metadata(parent)?;
    if !info.is_dir() || info.file_type().is_symlink() {
        bail!("Session directory must be an ordinary directory");
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if info.permissions().mode() & 0o077 != 0 {
            bail!("Session directory must be accessible to the owner only");
        }
    }
    if let Ok(info) = fs::symlink_metadata(&path) {
        if !info.is_file() || info.file_type().is_symlink() {
            bail!("Session database must be an ordinary file");
        }
    }
    Ok(path)
}

pub struct SessionLease {
    path: PathBuf,
    _file: File,
}
impl SessionLease {
    pub fn acquire(database: &Path) -> Result<Self> {
        let path = database.with_extension("owner.lock");
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|_| anyhow!("Session already owned or a lock requires recovery"))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            file.set_permissions(fs::Permissions::from_mode(0o600))?;
        }
        writeln!(file, "{}", std::process::id())?;
        file.sync_all()?;
        Ok(Self { path, _file: file })
    }
}
impl Drop for SessionLease {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}
