/*
 * Base-Matter — Vault IPC client (no Vault master key).
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Result};
#[cfg(unix)]
use anyhow::anyhow;
#[cfg(unix)]
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
#[cfg(unix)]
use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    XChaCha20Poly1305, XNonce,
};
use serde::Serialize;
#[cfg(unix)]
use serde::Deserialize;
use std::path::PathBuf;
#[cfg(unix)]
use std::{
    fs,
    io::{Read, Write},
    net::Shutdown,
    os::unix::{fs::PermissionsExt, net::UnixStream},
    time::Duration,
};
use zeroize::Zeroizing;
#[cfg(unix)]
const VERSION: u16 = 1;
#[cfg(unix)]
const MAX_WIRE: u64 = 2 * 1024 * 1024 + 8192;
const MAX_VALUE: usize = 1024 * 1024;
#[cfg(unix)]
const DOMAIN: &[u8] = b"matter-vault-ipc-aead-v1";

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
            || !matches!(operation, "ping" | "get" | "put")
            || value.is_some_and(|bytes| bytes.len() > MAX_VALUE)
        {
            bail!("invalid_vault_client_request");
        }
        Ok(())
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
    #[cfg(unix)]
    pub fn invoke(
        &self,
        operation: &str,
        record: &str,
        expected_revision: Option<u64>,
        value: Option<&[u8]>,
    ) -> Result<VaultResponse> {
        self.validate(operation, record, value)?;
        if (operation != "put" && (expected_revision.is_some() || value.is_some()))
            || (operation == "put" && value.is_none())
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
            UnixStream::connect(&self.socket_path).map_err(|_| anyhow!("vault_unavailable"))?;
        stream.set_read_timeout(Some(Duration::from_secs(5)))?;
        stream.set_write_timeout(Some(Duration::from_secs(5)))?;
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
    pub fn invoke(
        &self,
        operation: &str,
        record: &str,
        expected_revision: Option<u64>,
        value: Option<&[u8]>,
    ) -> Result<VaultResponse> {
        self.validate(operation, record, value)?;
        let _ = expected_revision;
        bail!("encrypted_unix_ipc_unavailable_on_this_platform")
    }
}
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
