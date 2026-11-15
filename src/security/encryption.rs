/*!
 * Módulo: Encryption
 *
 * Función:
 * Proteger datos persistentes mediante cifrado autenticado.
 *
 * Responsabilidades:
 * - Cifrar documentos JSON.
 * - Descifrar documentos JSON.
 * - Validar la integridad del contenido.
 * - Obtener la clave desde el entorno.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::{anyhow, bail, Context, Result};

use chacha20poly1305::{
    aead::{
        Aead,
        AeadCore,
        KeyInit,
        OsRng,
        Payload,
    },
    XChaCha20Poly1305,
    XNonce,
};

use serde::{
    de::DeserializeOwned,
    Deserialize,
    Serialize,
};

use zeroize::Zeroize;

const KEY_ENV: &str = "MATTER_CONTROL_ENCRYPTION_KEY";

const FORMAT_VERSION: u8 = 1;

const ALGORITHM: &str = "XChaCha20-Poly1305";

const ASSOCIATED_DATA: &[u8] =
    b"Matter-Base/control/roles/v1";

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct EncryptedEnvelope {
    version: u8,
    algorithm: String,
    nonce: String,
    ciphertext: String,
}

/// Cifra un documento JSON y devuelve su envoltorio cifrado.
pub fn encrypt_json<T: Serialize>(value: &T) -> Result<String> {
    let mut plaintext = serde_json::to_vec(value)
        .context("No se pudo serializar el documento")?;

    let mut key = load_key()?;

    let encrypted = encrypt_with_key(&plaintext, &key);

    plaintext.zeroize();
    key.zeroize();

    let envelope = encrypted?;

    serde_json::to_string_pretty(&envelope)
        .context("No se pudo serializar el documento cifrado")
}

/// Descifra un documento JSON y recupera su estructura original.
pub fn decrypt_json<T: DeserializeOwned>(
    encrypted_document: &str,
) -> Result<T> {
    let envelope: EncryptedEnvelope =
        serde_json::from_str(encrypted_document)
            .context("Documento cifrado inválido")?;

    validate_envelope(&envelope)?;

    let mut key = load_key()?;

    let decrypted = decrypt_with_key(&envelope, &key);

    key.zeroize();

    let mut plaintext = decrypted?;

    let result = serde_json::from_slice(&plaintext)
        .context("El contenido descifrado no es JSON válido");

    plaintext.zeroize();

    result
}

/// Obtiene una clave de 256 bits desde una variable de entorno.
fn load_key() -> Result<[u8; 32]> {
    let mut encoded = std::env::var(KEY_ENV)
        .context("Falta la clave de cifrado")?;

    let decoded_result = hex::decode(encoded.trim());

    encoded.zeroize();

    let mut decoded = decoded_result
        .context("La clave debe estar codificada en hexadecimal")?;

    if decoded.len() != 32 {
        decoded.zeroize();

        bail!("La clave debe contener exactamente 32 bytes");
    }

    let mut key = [0u8; 32];

    key.copy_from_slice(&decoded);

    decoded.zeroize();

    Ok(key)
}

/// Cifra bytes utilizando una clave proporcionada.
fn encrypt_with_key(
    plaintext: &[u8],
    key: &[u8; 32],
) -> Result<EncryptedEnvelope> {
    let cipher = XChaCha20Poly1305::new_from_slice(key)
        .map_err(|_| anyhow!("Clave criptográfica inválida"))?;

    let nonce =
        XChaCha20Poly1305::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(
            &nonce,
            Payload {
                msg: plaintext,
                aad: ASSOCIATED_DATA,
            },
        )
        .map_err(|_| anyhow!("No se pudo cifrar el documento"))?;

    Ok(EncryptedEnvelope {
        version: FORMAT_VERSION,
        algorithm: ALGORITHM.to_string(),
        nonce: hex::encode(nonce),
        ciphertext: hex::encode(ciphertext),
    })
}

/// Descifra bytes y valida su autenticidad.
fn decrypt_with_key(
    envelope: &EncryptedEnvelope,
    key: &[u8; 32],
) -> Result<Vec<u8>> {
    let nonce_bytes = hex::decode(&envelope.nonce)
        .context("Nonce inválido")?;

    if nonce_bytes.len() != 24 {
        bail!("Longitud de nonce inválida");
    }

    let ciphertext = hex::decode(&envelope.ciphertext)
        .context("Contenido cifrado inválido")?;

    let cipher = XChaCha20Poly1305::new_from_slice(key)
        .map_err(|_| anyhow!("Clave criptográfica inválida"))?;

    cipher
        .decrypt(
            XNonce::from_slice(&nonce_bytes),
            Payload {
                msg: &ciphertext,
                aad: ASSOCIATED_DATA,
            },
        )
        .map_err(|_| anyhow!("El contenido no superó la validación criptográfica"))
}

/// Impide aceptar formatos o algoritmos desconocidos.
fn validate_envelope(envelope: &EncryptedEnvelope) -> Result<()> {
    if envelope.version != FORMAT_VERSION {
        bail!("Versión de cifrado no compatible");
    }

    if envelope.algorithm != ALGORITHM {
        bail!("Algoritmo de cifrado no permitido");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypted_document_hides_identifiers() {
        let key = [42u8; 32];

        let original = serde_json::json!({
            "sockets": {
                "SOCKET_PRIVATE": {
                    "roles": {
                        "USER_PRIVATE": "co-owner"
                    }
                }
            }
        });

        let plaintext =
            serde_json::to_vec(&original).unwrap();

        let envelope =
            encrypt_with_key(&plaintext, &key).unwrap();

        let encoded =
            serde_json::to_string(&envelope).unwrap();

        assert!(!encoded.contains("SOCKET_PRIVATE"));
        assert!(!encoded.contains("USER_PRIVATE"));
        assert!(!encoded.contains("co-owner"));

        let decoded_envelope: EncryptedEnvelope =
            serde_json::from_str(&encoded).unwrap();

        let decrypted =
            decrypt_with_key(&decoded_envelope, &key).unwrap();

        let recovered: serde_json::Value =
            serde_json::from_slice(&decrypted).unwrap();

        assert_eq!(original, recovered);
    }

    #[test]
    fn modified_ciphertext_is_rejected() {
        let key = [42u8; 32];

        let original = br#"{"role":"staff"}"#;

        let mut envelope =
            encrypt_with_key(original, &key).unwrap();

        let mut ciphertext =
            hex::decode(&envelope.ciphertext).unwrap();

        ciphertext[0] ^= 1;

        envelope.ciphertext = hex::encode(ciphertext);

        assert!(decrypt_with_key(&envelope, &key).is_err());
    }
}