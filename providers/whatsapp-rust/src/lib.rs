/*
 * WhatsApp-Matter — provider configuration and exclusive SQLite session lease.
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{anyhow, bail, Result};
use std::{
    env,
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Component, Path, PathBuf},
};
use zeroize::Zeroizing;

pub struct Config {
    pub phone: String,
    pub owner_jid: String,
    pub socket_id: String,
    pub actor_id: String,
    pub session_db: PathBuf,
    pub activation_db: PathBuf,
    pub replay_db: PathBuf,
    pub replay_key: Zeroizing<[u8; 32]>,
}
impl Config {
    pub fn from_env() -> Result<Self> {
        Self::from_reader(|key| env::var(key).ok())
    }

    /// The operator-owned configuration is trusted, never messages from chat.
    pub fn from_reader(mut get: impl FnMut(&str) -> Option<String>) -> Result<Self> {
        let mut required = |key| {
            let value = get(key).ok_or_else(|| anyhow!("Missing provider configuration"))?;
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
            bail!("Phone must contain 8-15 international digits");
        }
        let owner_jid = required("MATTER_WA_OWNER_JID")?;
        let user = owner_jid.split('@').next().unwrap_or("");
        if owner_jid.len() > 128
            || user.is_empty()
            || !(owner_jid.ends_with("@s.whatsapp.net") || owner_jid.ends_with("@lid"))
            || !user
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"._:-".contains(&b))
        {
            bail!("Invalid WhatsApp owner JID");
        }
        let socket_id = required("MATTER_WA_SOCKET_ID")?;
        let actor_id = required("MATTER_WA_OWNER_ID")?;
        for id in [&socket_id, &actor_id] {
            if id.len() > 128
                || !id
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"._:-".contains(&b))
            {
                bail!("Invalid socket or owner ID");
            }
        }
        let session_db = private_path(&required("MATTER_WA_SESSION_DB")?)?;
        let activation_db = private_path(&required("MATTER_WA_ACTIVATION_DB")?)?;
        let replay_db = private_path(&required("MATTER_WA_REPLAY_DB")?)?;
        if session_db == activation_db || session_db == replay_db || activation_db == replay_db {
            bail!("Each SQLite database must have its own absolute path");
        }
        let key = Zeroizing::new(required("MATTER_WA_REPLAY_KEY_HEX")?);
        if key.len() != 64 || !key.bytes().all(|b| b.is_ascii_hexdigit()) {
            bail!("Replay secret must be a 256-bit hex string");
        }
        let mut replay_key = Zeroizing::new([0_u8; 32]);
        hex::decode_to_slice(key.as_bytes(), &mut replay_key[..])
            .map_err(|_| anyhow!("Invalid replay secret"))?;
        if replay_key.iter().all(|b| *b == 0) {
            bail!("Replay secret must not be zero");
        }
        Ok(Self {
            phone,
            owner_jid,
            socket_id,
            actor_id,
            session_db,
            activation_db,
            replay_db,
            replay_key,
        })
    }
}

/// Never operate on an implicit working-directory DB or symlinked path.
/// The directory must already exist and be private to its operator.
pub fn private_path(raw: &str) -> Result<PathBuf> {
    let path = PathBuf::from(raw);
    if !path.is_absolute()
        || path.components().any(|c| matches!(c, Component::ParentDir))
        || path.file_name().is_none()
    {
        bail!("SQLite paths must be absolute without parent traversal");
    }
    let parent = path
        .parent()
        .ok_or_else(|| anyhow!("Invalid SQLite directory"))?;
    let metadata = fs::symlink_metadata(parent)?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        bail!("SQLite directory must be an ordinary directory");
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if metadata.permissions().mode() & 0o077 != 0 {
            bail!("SQLite directory must not permit group or other access");
        }
    }
    if let Ok(meta) = fs::symlink_metadata(&path) {
        if !meta.is_file() || meta.file_type().is_symlink() {
            bail!("SQLite store path is not a regular file");
        }
    }
    Ok(path)
}

/// Refuse simultaneous ownership. A crash leaves the lock, requiring an
/// operator to check the old process before a manual recovery.
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
            .map_err(|_| anyhow!("Session locked or stale; verify previous process"))?;
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
