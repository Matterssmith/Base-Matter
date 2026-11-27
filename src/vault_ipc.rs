/*
 * Base-Matter — Vault IPC client (no Vault master key).
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
#[cfg(unix)]
use anyhow::anyhow;
use anyhow::{bail, Result};
#[cfg(unix)]
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
#[cfg(unix)]
use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    XChaCha20Poly1305, XNonce,
};
#[cfg(unix)]
use serde::Deserialize;
use serde::Serialize;
#[cfg(unix)]
use std::{
    fs,
    io::{Read, Write},
    net::Shutdown,
    os::unix::{
        fs::{MetadataExt, OpenOptionsExt, PermissionsExt},
        net::UnixStream,
    },
};
use std::{path::PathBuf, time::Duration};
use zeroize::Zeroizing;
#[cfg(unix)]
const VERSION: u16 = 1;
#[cfg(unix)]
const MAX_WIRE: u64 = 2 * 1024 * 1024 + 8192;
const MAX_VALUE: usize = 1024 * 1024;
#[cfg(unix)]
const DOMAIN: &[u8] = b"matter-vault-ipc-aead-v1";

#[derive(Serialize)]
pub(crate) struct PresentationRequest<'a> {
    pub chat: &'a str,
    pub actor: &'a str,
    pub target: Option<&'a str>,
    pub kind: &'a str,
    pub admin: bool,
    pub value: Option<&'a str>,
}

#[derive(Clone, Debug, Serialize)]
pub struct RecordScope {
    pub vault: String,
    pub tenant: String,
    pub bot: String,
    pub socket: String,
    pub kind: String,
    pub record: String,
}
#[cfg(unix)]
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Packet {
    version: u16,
    client_id: String,
    nonce: String,
    ciphertext: String,
}
#[cfg(unix)]
#[derive(Serialize)]
struct Request {
    version: u16,
    operation: String,
    request_id: String,
    scope: RecordScope,
    expected_revision: Option<u64>,
    value: Option<String>,
}
#[cfg(unix)]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Response {
    version: u16,
    ok: bool,
    revision: Option<u64>,
    value: Option<String>,
    error: Option<String>,
}
pub struct VaultResponse {
    pub revision: Option<u64>,
    pub value: Option<Zeroizing<Vec<u8>>>,
}
#[cfg(unix)]
struct CacheMaterial {
    path: PathBuf,
    context: Vec<u8>,
    cipher: XChaCha20Poly1305,
}
#[derive(Clone, Debug)]
pub struct VaultClient {
    pub socket_path: PathBuf,
    pub client_id: String,
    pub client_key_file: PathBuf,
    pub tenant: String,
    pub bot: String,
    pub socket: String,
}
fn safe_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 96
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._:-".contains(&b))
}
#[cfg(unix)]
fn aad(id: &str, direction: &str) -> Vec<u8> {
    let mut out = DOMAIN.to_vec();
    out.extend_from_slice(b":");
    out.extend_from_slice(id.as_bytes());
    out.extend_from_slice(b":");
    out.extend_from_slice(direction.as_bytes());
    out
}
impl VaultClient {
    /// Optional host-provisioned service identity; no secrets in env values.
    pub fn from_environment() -> Result<Option<Self>> {
        let names = [
            "MATTER_VAULT_IPC_SOCKET",
            "MATTER_VAULT_CLIENT_ID",
            "MATTER_VAULT_CLIENT_KEY_FILE",
            "MATTER_VAULT_TENANT",
            "MATTER_VAULT_BOT",
            "MATTER_VAULT_SOCKET_ID",
        ];
        let values = names
            .iter()
            .map(|name| std::env::var(name).ok())
            .collect::<Vec<_>>();
        if values.iter().all(Option::is_none) {
            return Ok(None);
        }
        if values.iter().any(Option::is_none) {
            bail!("incomplete_vault_client_configuration");
        }
        let values = values
            .into_iter()
            .map(|value| value.expect("checked before"))
            .collect::<Vec<_>>();
        let client = Self {
            socket_path: PathBuf::from(&values[0]),
            client_id: values[1].clone(),
            client_key_file: PathBuf::from(&values[2]),
            tenant: values[3].clone(),
            bot: values[4].clone(),
            socket: values[5].clone(),
        };
        client.validate("ping", "health", None)?;
        Ok(Some(client))
    }

    /// WhatsApp adapter passes the sender JID proven by its HMAC callback.
    /// A display name, typed username or mention never proves this identity.
    pub fn require_global_owner(&self, identity: &str) -> Result<()> {
        self.require_global_owner_for_platform("whatsapp", identity)
    }

    /// Common capability for future verified Telegram and Discord adapters.
    /// Platform and immutable subject ID must come from authenticated provider
    /// metadata. Never pass a username string as a verified subject.
    pub fn require_global_owner_for_platform(
        &self,
        platform: &str,
        subject_id: &str,
    ) -> Result<()> {
        self.require_global_owner_with_timeout(platform, subject_id, Duration::from_secs(5))
    }

    /// Best-effort console metadata, never an authorization cache.
    pub(crate) fn console_global_owner(&self, identity: &str) -> bool {
        self.require_global_owner_with_timeout("whatsapp", identity, Duration::from_millis(250))
            .is_ok()
    }

    fn require_global_owner_with_timeout(
        &self,
        platform: &str,
        subject_id: &str,
        timeout: Duration,
    ) -> Result<()> {
        let numeric = |value: &str, min: usize, max: usize| {
            (min..=max).contains(&value.len())
                && value.bytes().all(|byte| byte.is_ascii_digit())
                && value.bytes().any(|byte| byte != b'0')
        };
        let valid = match platform {
            "whatsapp" => subject_id
                .strip_suffix("@s.whatsapp.net")
                .or_else(|| subject_id.strip_suffix("@lid"))
                .is_some_and(|value| numeric(value, 8, 20)),
            "telegram" => numeric(subject_id, 1, 20),
            "discord" => numeric(subject_id, 15, 20),
            _ => false,
        };
        if !valid {
            bail!("invalid_global_owner_principal");
        }
        let principal = serde_json::json!({
            "platform": platform,
            "subject_id": subject_id,
        });
        let encoded = serde_json::to_vec(&principal)?;
        let result = self.invoke_with_timeout(
            "owner_check",
            "global-owner-check-v2",
            None,
            Some(&encoded),
            timeout,
        )?;
        if !result
            .value
            .as_ref()
            .is_some_and(|bytes| bytes.as_slice() == b"global-owner:verified")
        {
            bail!("global_owner_denied");
        }
        Ok(())
    }

    /// Both independently enrolled global owners have identical authority.
    /// The bridge signs the platform principal; Vault independently enforces
    /// membership, regardless of which Matter bot/socket received it.
    pub fn grant_secondary_global_owner(&self, requester: &str, target_phone: &str) -> Result<()> {
        let valid_pn = |id: &str| {
            id.strip_suffix("@s.whatsapp.net").is_some_and(|digits| {
                (8..=20).contains(&digits.len()) && digits.bytes().all(|byte| byte.is_ascii_digit())
            })
        };
        if !valid_pn(requester) {
            bail!("Se requiere un WhatsApp PN verificado de un owner global");
        }
        let digits = target_phone.strip_prefix('+').unwrap_or(target_phone);
        if !(8..=20).contains(&digits.len()) || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
            bail!("Uso: =/>set role owner --global NUMERO");
        }
        let new_subject = format!("{digits}@s.whatsapp.net");
        if requester == new_subject {
            bail!("El destino ya es la identidad que solicita el cambio");
        }
        let payload = serde_json::to_vec(&serde_json::json!({
            "requester": { "platform": "whatsapp", "subject_id": requester },
            "new_subject_id": new_subject
        }))?;
        let response = self.invoke(
            "owner_grant_secondary",
            "global-owner-secondary-v1",
            None,
            Some(&payload),
        )?;
        if !response
            .value
            .as_ref()
            .is_some_and(|bytes| bytes.as_slice() == b"owner-secondary:updated")
        {
            bail!("Concesión global denegada por Vault");
        }
        Ok(())
    }

    /// Status/testing require an authorized global owner. Both enrolled
    /// owners receive the same capabilities from the Vault authority.
    pub fn global_owner_vault_command(
        &self,
        verified_identity: &str,
        args: &[&str],
    ) -> Result<String> {
        self.require_global_owner(verified_identity)?;
        let test = matches!(args, ["test"]);
        if !(args.is_empty() || matches!(args, ["status"]) || test) {
            bail!("Uso: =/>vault [status|test]");
        }
        let answer = if test {
            if self.verify_roundtrip().is_ok() {
                "Vault-Matter: lectura y escritura cifradas verificadas. Owners globales con igual autoridad; las operaciones disponibles siguen las políticas del ecosistema."
            } else {
                "Vault-Matter: prueba de lectura/escritura denegada o fallida."
            }
        } else if self.invoke("ping", "health", None, None).is_ok() {
            "Vault-Matter: servicio cifrado autenticado. Los owners globales registrados tienen autoridad equivalente en Matter."
        } else {
            "Vault-Matter: servicio no disponible o permiso denegado."
        };
        Ok(answer.to_owned())
    }

    fn validate(&self, operation: &str, record: &str, value: Option<&[u8]>) -> Result<()> {
        if !self.socket_path.is_absolute()
            || !self.client_key_file.is_absolute()
            || ![
                self.client_id.as_str(),
                self.tenant.as_str(),
                self.bot.as_str(),
                self.socket.as_str(),
                record,
            ]
            .iter()
            .all(|id| safe_id(id))
            || !matches!(
                operation,
                "ping"
                    | "get"
                    | "put"
                    | "owner_check"
                    | "owner_grant_secondary"
                    | "global_style_get"
                    | "global_style_set"
                    | "presentation_config"
            )
            || value.is_some_and(|bytes| bytes.len() > MAX_VALUE)
        {
            bail!("invalid_vault_client_request");
        }
        Ok(())
    }
    /// Only presentation and Anti configuration may be read offline.
    /// Identity, authority, enrollment, health and secrets never use this cache.
    #[cfg(unix)]
    fn cacheable_config(operation: &str, record: &str) -> bool {
        let hex_suffix = |value: &str, length: usize| {
            value.len() == length && value.bytes().all(|byte| byte.is_ascii_hexdigit())
        };
        match operation {
            "global_style_get" => record == "global-menu-style-v1",
            "get" => {
                matches!(
                    record,
                    "menu-style-v1" | "menu-language-v1" | "menu-mode-v1"
                ) || record
                    .strip_prefix("menu-style-chat-")
                    .is_some_and(|value| hex_suffix(value, 64))
                    || record.strip_prefix("anti-").is_some_and(|value| {
                        value.rsplit_once('-').is_some_and(|(name, hash)| {
                            (name == "domains" || crate::anti::RULES.contains(&name))
                                && hex_suffix(hash, 32)
                        })
                    })
            }
            _ => false,
        }
    }

    #[cfg(unix)]
    fn cache_material(&self, operation: &str, record: &str) -> Result<Option<CacheMaterial>> {
        let Some(directory) = std::env::var_os("MATTER_VAULT_READ_CACHE_DIR") else {
            return Ok(None);
        };
        self.cache_material_in(PathBuf::from(directory), operation, record)
            .map(Some)
    }

    #[cfg(unix)]
    fn cache_material_in(
        &self,
        directory: PathBuf,
        operation: &str,
        record: &str,
    ) -> Result<CacheMaterial> {
        use hmac::{Hmac, Mac};
        use sha2::{Digest, Sha256};
        let metadata = fs::symlink_metadata(&directory)?;
        let key_metadata = fs::symlink_metadata(&self.client_key_file)?;
        if !directory.is_absolute()
            || !metadata.is_dir()
            || metadata.file_type().is_symlink()
            || metadata.permissions().mode() & 0o077 != 0
            || metadata.uid() != key_metadata.uid()
        {
            bail!("unsafe_vault_read_cache");
        }
        let context = serde_json::to_vec(&[
            "matter-vault-offline-read-v1",
            self.client_id.as_str(),
            self.tenant.as_str(),
            self.bot.as_str(),
            self.socket.as_str(),
            operation,
            record,
        ])?;
        let key = self.key()?;
        let mut derivation = <Hmac<Sha256> as Mac>::new_from_slice(&key[..])
            .map_err(|_| anyhow!("vault_cache_key_unavailable"))?;
        derivation.update(b"matter-vault-offline-cache-key-v1");
        let derived = Zeroizing::new(<[u8; 32]>::from(derivation.finalize().into_bytes()));
        let cipher = XChaCha20Poly1305::new_from_slice(&derived[..])
            .map_err(|_| anyhow!("vault_cache_cipher_unavailable"))?;
        let path = directory.join(format!("{}.bin", hex::encode(Sha256::digest(&context))));
        Ok(CacheMaterial {
            path,
            context,
            cipher,
        })
    }

    #[cfg(unix)]
    fn save_config_snapshot(
        &self,
        operation: &str,
        record: &str,
        value: Option<&str>,
    ) -> Result<()> {
        if !Self::cacheable_config(operation, record) {
            return Ok(());
        }
        let Some(material) = self.cache_material(operation, record)? else {
            return Ok(());
        };
        Self::write_snapshot(material, value)
    }

    #[cfg(unix)]
    fn write_snapshot(material: CacheMaterial, value: Option<&str>) -> Result<()> {
        let CacheMaterial {
            path,
            context,
            cipher,
        } = material;
        let clear = Zeroizing::new(serde_json::to_vec(&value)?);
        let mut nonce = [0u8; 24];
        getrandom::fill(&mut nonce).map_err(|_| anyhow!("random_unavailable"))?;
        let ciphertext = cipher
            .encrypt(
                XNonce::from_slice(&nonce),
                Payload {
                    msg: &clear,
                    aad: &context,
                },
            )
            .map_err(|_| anyhow!("vault_cache_encryption_failed"))?;
        let temporary = path.with_extension(format!("{}.tmp", hex::encode(nonce)));
        let result = (|| -> Result<()> {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(&temporary)?;
            file.write_all(&nonce)?;
            file.write_all(&ciphertext)?;
            file.sync_all()?;
            fs::rename(&temporary, &path)?;
            Ok(())
        })();
        if result.is_err() {
            let _ = fs::remove_file(&temporary);
        }
        result
    }

    #[cfg(unix)]
    fn load_config_snapshot(&self, operation: &str, record: &str) -> Result<Option<String>> {
        let Some(material) = self.cache_material(operation, record)? else {
            bail!("vault_offline_no_snapshot");
        };
        self.read_snapshot(material)
    }

    #[cfg(unix)]
    fn read_snapshot(&self, material: CacheMaterial) -> Result<Option<String>> {
        let CacheMaterial {
            path,
            context,
            cipher,
        } = material;
        let metadata = match fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                bail!("vault_offline_no_snapshot");
            }
            Err(error) => return Err(error.into()),
        };
        if !metadata.is_file()
            || metadata.file_type().is_symlink()
            || metadata.permissions().mode() & 0o077 != 0
            || metadata.uid() != fs::symlink_metadata(&self.client_key_file)?.uid()
            || metadata.len() > MAX_WIRE
        {
            bail!("unsafe_vault_read_cache");
        }
        let bytes = fs::read(path)?;
        if bytes.len() < 40 {
            bail!("invalid_vault_read_cache");
        }
        let clear = Zeroizing::new(
            cipher
                .decrypt(
                    XNonce::from_slice(&bytes[..24]),
                    Payload {
                        msg: &bytes[24..],
                        aad: &context,
                    },
                )
                .map_err(|_| anyhow!("vault_read_cache_unauthenticated"))?,
        );
        Ok(serde_json::from_slice(&clear)?)
    }

    fn refresh_snapshot(&self, operation: &str, record: &str, value: Option<&str>) {
        #[cfg(unix)]
        if self.save_config_snapshot(operation, record, value).is_err() {
            eprintln!("Vault-Matter: encrypted read cache refresh failed");
        }
        #[cfg(not(unix))]
        let _ = (operation, record, value);
    }

    fn read_config(&self, operation: &str, record: &str) -> Result<(Option<String>, bool)> {
        match self.invoke(operation, record, None, None) {
            Ok(response) => {
                let value = response
                    .value
                    .map(|value| Ok::<_, anyhow::Error>(std::str::from_utf8(&value)?.to_owned()))
                    .transpose()?;
                self.refresh_snapshot(operation, record, value.as_deref());
                Ok((value, false))
            }
            Err(error) => {
                #[cfg(unix)]
                if error.to_string() == "vault_unavailable"
                    && Self::cacheable_config(operation, record)
                {
                    return Ok((self.load_config_snapshot(operation, record)?, true));
                }
                Err(error)
            }
        }
    }

    pub fn global_style_read(&self) -> Result<(crate::menu::Style, bool)> {
        let (value, offline) = self
            .read_config("global_style_get", "global-menu-style-v1")
            .map_err(|error| {
                if error.to_string() == "vault_operation_denied" {
                    anyhow::anyhow!("vault_global_style_denied")
                } else {
                    error
                }
            })?;
        let value = value.ok_or_else(|| anyhow::anyhow!("Missing global style"))?;
        Ok((crate::menu::Style::parse(&value)?, offline))
    }

    pub fn global_style(&self) -> Result<crate::menu::Style> {
        Ok(self.global_style_read()?.0)
    }

    pub fn set_global_style(&self, principal: &str, style: crate::menu::Style) -> Result<()> {
        let payload = serde_json::to_vec(&serde_json::json!({
            "requester": {"platform": "whatsapp", "subject_id": principal},
            "style": style.as_str(),
        }))?;
        let response = self.invoke(
            "global_style_set",
            "global-menu-style-v1",
            None,
            Some(&payload),
        )?;
        if let Some(value) = response.value.as_ref() {
            let value = std::str::from_utf8(value)?;
            crate::menu::Style::parse(value)?;
            self.refresh_snapshot("global_style_get", "global-menu-style-v1", Some(value));
        }
        Ok(())
    }

    pub(crate) fn presentation_config(
        &self,
        request: &PresentationRequest<'_>,
    ) -> Result<Option<String>> {
        let payload = serde_json::to_vec(request)?;
        self.invoke(
            "presentation_config",
            "presentation-v1",
            None,
            Some(&payload),
        )?
        .value
        .map(|value| Ok(std::str::from_utf8(&value)?.to_owned()))
        .transpose()
    }

    pub fn config_get_with_source(&self, record: &str) -> Result<(Option<String>, bool)> {
        self.read_config("get", record)
    }

    pub fn config_get(&self, record: &str) -> Result<Option<String>> {
        Ok(self.config_get_with_source(record)?.0)
    }

    pub fn config_set(&self, record: &str, value: &str) -> Result<()> {
        let previous = self.invoke("get", record, None, None)?;
        self.invoke("put", record, previous.revision, Some(value.as_bytes()))?;
        self.refresh_snapshot("get", record, Some(value));
        Ok(())
    }

    /// Verify a real encrypted read/write roundtrip on a reserved synthetic
    /// record. Does not grant roles or access to any other bot or socket.
    pub fn verify_roundtrip(&self) -> Result<u64> {
        const RECORD: &str = "connection-health-v1";
        const MARKER: &[u8] = b"matter-vault-health-check-v1";
        let previous = self.invoke("get", RECORD, None, None)?;
        let written = self.invoke("put", RECORD, previous.revision, Some(MARKER))?;
        let Some(revision) = written.revision else {
            bail!("vault_self_test_no_revision");
        };
        let checked = self.invoke("get", RECORD, None, None)?;
        if checked.revision != Some(revision)
            || !matches!(checked.value.as_ref(), Some(bytes) if bytes.as_slice() == MARKER)
        {
            bail!("vault_self_test_mismatch");
        }
        Ok(revision)
    }

    #[cfg(unix)]
    fn key(&self) -> Result<Zeroizing<Vec<u8>>> {
        let meta = fs::symlink_metadata(&self.client_key_file)?;
        if !meta.file_type().is_file()
            || meta.file_type().is_symlink()
            || meta.permissions().mode() & 0o077 != 0
        {
            bail!("unsafe_vault_client_key");
        }
        let key = Zeroizing::new(fs::read(&self.client_key_file)?);
        if key.len() != 32 {
            bail!("bad_vault_client_key");
        }
        Ok(key)
    }
    pub fn invoke(
        &self,
        operation: &str,
        record: &str,
        expected_revision: Option<u64>,
        value: Option<&[u8]>,
    ) -> Result<VaultResponse> {
        self.invoke_with_timeout(
            operation,
            record,
            expected_revision,
            value,
            Duration::from_secs(5),
        )
    }

    #[cfg(unix)]
    fn invoke_with_timeout(
        &self,
        operation: &str,
        record: &str,
        expected_revision: Option<u64>,
        value: Option<&[u8]>,
        timeout: Duration,
    ) -> Result<VaultResponse> {
        self.validate(operation, record, value)?;
        if (operation != "put"
            && operation != "owner_check"
            && operation != "owner_grant_secondary"
            && operation != "global_style_set"
            && operation != "presentation_config"
            && (expected_revision.is_some() || value.is_some()))
            || (operation == "put" && value.is_none())
            || (operation == "owner_check"
                && (!matches!(record, "global-owner-check-v1" | "global-owner-check-v2")
                    || expected_revision.is_some()
                    || value.is_none_or(|identity| identity.len() > 256)))
            || (operation == "presentation_config"
                && (record != "presentation-v1"
                    || expected_revision.is_some()
                    || value.is_none_or(|payload| payload.len() > 1024)))
            || (operation == "global_style_set"
                && (record != "global-menu-style-v1"
                    || expected_revision.is_some()
                    || value.is_none_or(|payload| payload.len() > 384)))
            || (operation == "owner_grant_secondary"
                && (record != "global-owner-secondary-v1"
                    || expected_revision.is_some()
                    || value.is_none_or(|payload| payload.len() > 512)))
        {
            bail!("invalid_vault_operation_arguments");
        }
        let key = self.key()?;
        let cipher = XChaCha20Poly1305::new_from_slice(&key[..])
            .map_err(|_| anyhow!("vault_cipher_unavailable"))?;
        let mut nonce = [0u8; 24];
        getrandom::fill(&mut nonce).map_err(|_| anyhow!("random_unavailable"))?;
        let mut request_id = [0u8; 16];
        getrandom::fill(&mut request_id).map_err(|_| anyhow!("random_unavailable"))?;
        let request = Request {
            version: VERSION,
            operation: operation.into(),
            request_id: hex::encode(request_id),
            scope: RecordScope {
                vault: "matter-vault".into(),
                tenant: self.tenant.clone(),
                bot: self.bot.clone(),
                socket: self.socket.clone(),
                kind: "configuration".into(),
                record: record.into(),
            },
            expected_revision,
            value: value.map(|v| URL_SAFE_NO_PAD.encode(v)),
        };
        let clear = Zeroizing::new(serde_json::to_vec(&request)?);
        let ciphertext = cipher
            .encrypt(
                XNonce::from_slice(&nonce),
                Payload {
                    msg: &clear,
                    aad: &aad(&self.client_id, "request"),
                },
            )
            .map_err(|_| anyhow!("vault_encryption_failed"))?;
        let packet = Packet {
            version: VERSION,
            client_id: self.client_id.clone(),
            nonce: URL_SAFE_NO_PAD.encode(nonce),
            ciphertext: URL_SAFE_NO_PAD.encode(ciphertext),
        };
        let payload = serde_json::to_vec(&packet)?;
        if payload.len() as u64 > MAX_WIRE {
            bail!("vault_request_too_large");
        }
        let mut stream =
            UnixStream::connect(&self.socket_path).map_err(|error| match error.kind() {
                std::io::ErrorKind::NotFound | std::io::ErrorKind::ConnectionRefused => {
                    anyhow!("vault_unavailable")
                }
                _ => anyhow!("vault_connection_failed"),
            })?;
        stream.set_read_timeout(Some(timeout))?;
        stream.set_write_timeout(Some(timeout))?;
        stream.write_all(&payload)?;
        stream.shutdown(Shutdown::Write)?;
        let mut encoded = Vec::new();
        (&mut stream).take(MAX_WIRE + 1).read_to_end(&mut encoded)?;
        if encoded.len() as u64 > MAX_WIRE {
            bail!("vault_response_too_large");
        }
        let packet: Packet =
            serde_json::from_slice(&encoded).map_err(|_| anyhow!("invalid_vault_response"))?;
        if packet.version != VERSION || packet.client_id != self.client_id {
            bail!("vault_response_identity_mismatch");
        }
        let nonce = URL_SAFE_NO_PAD.decode(&packet.nonce)?;
        if nonce.len() != 24 {
            bail!("vault_response_nonce_invalid");
        }
        let ciphertext = URL_SAFE_NO_PAD.decode(&packet.ciphertext)?;
        let decrypted = cipher
            .decrypt(
                XNonce::from_slice(&nonce),
                Payload {
                    msg: &ciphertext,
                    aad: &aad(&self.client_id, "response"),
                },
            )
            .map_err(|_| anyhow!("vault_response_unauthenticated"))?;
        let clear = Zeroizing::new(decrypted);
        let response: Response =
            serde_json::from_slice(&clear).map_err(|_| anyhow!("vault_response_invalid"))?;
        if response.version != VERSION || !response.ok || response.error.is_some() {
            bail!("vault_operation_denied");
        }
        let value = match response.value {
            None => None,
            Some(encoded) => {
                if encoded.len() > MAX_WIRE as usize {
                    bail!("vault_value_too_large");
                }
                let bytes = Zeroizing::new(URL_SAFE_NO_PAD.decode(encoded)?);
                if bytes.len() > MAX_VALUE {
                    bail!("vault_value_too_large");
                }
                Some(bytes)
            }
        };
        Ok(VaultResponse {
            revision: response.revision,
            value,
        })
    }
    #[cfg(not(unix))]
    fn invoke_with_timeout(
        &self,
        operation: &str,
        record: &str,
        expected_revision: Option<u64>,
        value: Option<&[u8]>,
        _timeout: Duration,
    ) -> Result<VaultResponse> {
        self.validate(operation, record, value)?;
        let _ = expected_revision;
        bail!("encrypted_unix_ipc_unavailable_on_this_platform")
    }
}
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].

#[cfg(all(test, unix))]
mod cache_tests {
    use super::*;

    #[test]
    fn cache_is_configuration_only() {
        assert!(VaultClient::cacheable_config("get", "menu-style-v1"));
        assert!(VaultClient::cacheable_config(
            "global_style_get",
            "global-menu-style-v1"
        ));
        assert!(!VaultClient::cacheable_config(
            "get",
            "global-owner-check-v2"
        ));
        assert!(!VaultClient::cacheable_config(
            "owner_check",
            "global-owner-check-v2"
        ));
        assert!(!VaultClient::cacheable_config("put", "menu-style-v1"));
        assert!(!VaultClient::cacheable_config(
            "get",
            "connection-health-v1"
        ));
    }

    #[test]
    fn encrypted_snapshot_is_scope_bound_and_offline_writes_fail() -> Result<()> {
        let mut random = [0u8; 16];
        getrandom::fill(&mut random).map_err(|_| anyhow!("random_unavailable"))?;
        let directory = std::env::temp_dir().join(format!("matter-cache-{}", hex::encode(random)));
        use std::os::unix::fs::DirBuilderExt;
        fs::DirBuilder::new().mode(0o700).create(&directory)?;
        let result = (|| -> Result<()> {
            let key = directory.join("client.key");
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(&key)?;
            file.write_all(&[9u8; 32])?;
            let client = VaultClient {
                socket_path: directory.join("absent.sock"),
                client_id: "cache-test".into(),
                client_key_file: key,
                tenant: "test".into(),
                bot: "test".into(),
                socket: "test".into(),
            };
            let material = || client.cache_material_in(directory.clone(), "get", "menu-style-v1");
            let path = material()?.path;
            VaultClient::write_snapshot(material()?, Some("none"))?;
            assert_eq!(client.read_snapshot(material()?)?, Some("none".into()));
            let mut foreign =
                client.cache_material_in(directory.clone(), "get", "menu-language-v1")?;
            foreign.path = path.clone();
            assert!(client.read_snapshot(foreign).is_err());
            let mut bytes = fs::read(&path)?;
            let last = bytes.len() - 1;
            bytes[last] ^= 1;
            fs::write(&path, bytes)?;
            assert!(client.read_snapshot(material()?).is_err());
            VaultClient::write_snapshot(material()?, None)?;
            assert_eq!(client.read_snapshot(material()?)?, None);
            assert!(client.config_set("menu-style-v1", "matter").is_err());
            assert!(client
                .set_global_style("12345678@lid", crate::menu::Style::None)
                .is_err());
            Ok(())
        })();
        fs::remove_dir_all(directory)?;
        result
    }
}
