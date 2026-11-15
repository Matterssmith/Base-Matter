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
use time::{
    format_description::well_known::Rfc3339,
    OffsetDateTime,
};

const FORMAT_VERSION: u32 = 1;
const ALGORITHM: &str = "XChaCha20-Poly1305";
const NONCE_LENGTH: usize = 24;
const TAG_LENGTH: usize = 16;
const MAX_TEXT_LENGTH: usize = 128;

const ALLOWED_RECORD_TYPES: &[&str] = &[
    "roles",
    "permissions",
    "configuration",
    "audit",
    "encrypted_key",
];

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
        envelope_id,
        scope,
        record_type,
        key_id
    )
    .into_bytes()
}

fn validate_text(value: &str, name: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.len() > MAX_TEXT_LENGTH
        || value.chars().any(char::is_control)
    {
        bail!("Invalid {name}");
    }

    Ok(())
}

fn validate_record_type(record_type: &str) -> Result<()> {
    if !ALLOWED_RECORD_TYPES.contains(&record_type) {
        bail!("Unsupported record type");
    }

    Ok(())
}

fn validate_timestamp(value: &str) -> Result<()> {
    if value.is_empty() || value.len() > 64 {
        bail!("Invalid created_at");
    }

    OffsetDateTime::parse(value, &Rfc3339)
        .context("created_at must be a valid RFC3339 timestamp")?;

    Ok(())
}

fn decode_canonical_base64(
    value: &str,
    name: &str,
) -> Result<Vec<u8>> {
    let decoded = URL_SAFE_NO_PAD
        .decode(value)
        .with_context(|| format!("Invalid {name} encoding"))?;

    if URL_SAFE_NO_PAD.encode(&decoded) != value {
        bail!("Non-canonical {name} encoding");
    }

    Ok(decoded)
}

fn validate_metadata(envelope: &EncryptedEnvelope) -> Result<()> {
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

    validate_record_type(&envelope.record_type)?;
    validate_timestamp(&envelope.created_at)?;

    Ok(())
}

/// Validates the envelope structure and encoded fields.
pub fn validate_envelope(envelope: &EncryptedEnvelope) -> Result<()> {
    validate_metadata(envelope)?;

    let nonce = decode_canonical_base64(
        &envelope.nonce,
        "nonce",
    )?;

    if nonce.len() != NONCE_LENGTH {
        bail!("Invalid nonce length");
    }

    let ciphertext = decode_canonical_base64(
        &envelope.ciphertext,
        "ciphertext",
    )?;

    if ciphertext.len() < TAG_LENGTH {
        bail!("Ciphertext is too short");
    }

    Ok(())
}

/// Encrypts plaintext using XChaCha20-Poly1305.
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

    validate_record_type(record_type)?;
    validate_timestamp(created_at)?;

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

    let envelope = EncryptedEnvelope {
        format_version: FORMAT_VERSION,
        envelope_id: envelope_id.to_owned(),
        scope: scope.to_owned(),
        record_type: record_type.to_owned(),
        key_id: key_id.to_owned(),
        algorithm: ALGORITHM.to_owned(),
        nonce: URL_SAFE_NO_PAD.encode(nonce),
        ciphertext: URL_SAFE_NO_PAD.encode(ciphertext),
        created_at: created_at.to_owned(),
    };

    validate_envelope(&envelope)?;

    Ok(envelope)
}

/// Validates and decrypts an authenticated envelope.
pub fn decrypt(
    envelope: &EncryptedEnvelope,
    key: &[u8; 32],
) -> Result<Vec<u8>> {
    validate_envelope(envelope)?;

    let nonce_bytes = decode_canonical_base64(
        &envelope.nonce,
        "nonce",
    )?;

    let ciphertext = decode_canonical_base64(
        &envelope.ciphertext,
        "ciphertext",
    )?;

    let nonce: [u8; NONCE_LENGTH] = nonce_bytes
        .try_into()
        .map_err(|_| anyhow::anyhow!("Invalid nonce length"))?;

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