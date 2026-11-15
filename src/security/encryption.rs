
/*
 * Matter-Base — Authenticated Encryption
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::{bail, Context, Result};
use base64::{
    engine::general_purpose::URL_SAFE_NO_PAD,
    Engine,
};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng, Payload},
    XChaCha20Poly1305, XNonce,
};
use serde::{Deserialize, Serialize};

const FORMAT_VERSION: u32 = 1;
const ALGORITHM: &str = "XChaCha20-Poly1305";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EncryptedEnvelope {
    pub format_version: u32,
    pub envelope_id: String,
    pub scope: String,
    pub record_type: String,
    pub key_id: String,
    pub algorithm: String,
    pub nonce: String,
    pub ciphertext: String,
    pub created_at: String,
}

fn associated_data(
    envelope_id: &str,
    scope: &str,
    record_type: &str,
    key_id: &str,
) -> Vec<u8> {
    format!(
        "matter-vault:v1\0{}\0{}\0{}\0{}",
        envelope_id, scope, record_type, key_id
    )
    .into_bytes()
}

fn validate_text(value: &str, name: &str) -> Result<()> {
    if value.is_empty() || value.len() > 128 {
        bail!("Invalid {name}");
    }

    if value.contains('\0') {
        bail!("Invalid {name}");
    }

    Ok(())
}

pub fn encrypt(
    plaintext: &[u8],
    key: &[u8; 32],
    envelope_id: &str,
    scope: &str,
    record_type: &str,
    key_id: &str,
    created_at: &str,
) -> Result<EncryptedEnvelope> {
    validate_text(envelope_id, "envelope_id")?;
    validate_text(scope, "scope")?;
    validate_text(record_type, "record_type")?;
    validate_text(key_id, "key_id")?;

    if created_at.is_empty() || created_at.len() > 64 {
        bail!("Invalid created_at");
    }

    let cipher = XChaCha20Poly1305::new(key.into());

    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);

    let aad = associated_data(
        envelope_id,
        scope,
        record_type,
        key_id,
    );

    let ciphertext = cipher
        .encrypt(
            &nonce,
            Payload {
                msg: plaintext,
                aad: &aad,
            },
        )
        .context("Encryption failed")?;

    Ok(EncryptedEnvelope {
        format_version: FORMAT_VERSION,
        envelope_id: envelope_id.to_owned(),
        scope: scope.to_owned(),
        record_type: record_type.to_owned(),
        key_id: key_id.to_owned(),
        algorithm: ALGORITHM.to_owned(),
        nonce: URL_SAFE_NO_PAD.encode(nonce),
        ciphertext: URL_SAFE_NO_PAD.encode(ciphertext),
        created_at: created_at.to_owned(),
    })
}

pub fn decrypt(
    envelope: &EncryptedEnvelope,
    key: &[u8; 32],
) -> Result<Vec<u8>> {
    if envelope.format_version != FORMAT_VERSION {
        bail!("Unsupported envelope version");
    }

    if envelope.algorithm != ALGORITHM {
        bail!("Unsupported encryption algorithm");
    }

    validate_text(&envelope.envelope_id, "envelope_id")?;
    validate_text(&envelope.scope, "scope")?;
    validate_text(&envelope.record_type, "record_type")?;
    validate_text(&envelope.key_id, "key_id")?;

    let nonce_bytes = URL_SAFE_NO_PAD
        .decode(&envelope.nonce)
        .context("Invalid nonce encoding")?;

    let ciphertext = URL_SAFE_NO_PAD
        .decode(&envelope.ciphertext)
        .context("Invalid ciphertext encoding")?;

    let nonce: [u8; 24] = nonce_bytes
        .try_into()
        .map_err(|_| anyhow::anyhow!("Invalid nonce length"))?;

    if ciphertext.len() < 16 {
        bail!("Ciphertext is too short");
    }

    let cipher = XChaCha20Poly1305::new(key.into());

    let aad = associated_data(
        &envelope.envelope_id,
        &envelope.scope,
        &envelope.record_type,
        &envelope.key_id,
    );

    cipher
        .decrypt(
            XNonce::from_slice(&nonce),
            Payload {
                msg: &ciphertext,
                aad: &aad,
            },
        )
        .context("Authentication or decryption failed")
}
