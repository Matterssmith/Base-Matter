/*
 * Matter-Base — Key Provider
 *
 * Define los contratos para recuperar claves criptográficas
 * y demostrar la posesión del secreto de un proveedor.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::{anyhow, Result};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use zeroize::Zeroizing;

type HmacSha256 = Hmac<Sha256>;

/// Proveedor externo de claves criptográficas.
pub trait KeyProvider: Send + Sync {
    /// Devuelve el identificador del Vault consultado.
    fn vault_id(&self) -> Result<String>;

    /// Devuelve los identificadores de las claves disponibles.
    fn key_ids(&self) -> Result<Vec<String>>;

    /// Devuelve el identificador de la clave activa.
    fn active_key_id(&self) -> Result<Option<String>>;

    /// Recupera una clave mediante su identificador.
    fn load_key(&self, key_id: &str) -> Result<Zeroizing<[u8; 32]>>;
}

/// Proveedor que demuestra posesión de un secreto exclusivo.
///
/// El secreto debe ser independiente de la clave usada para
/// cifrar las afirmaciones de autenticación.
pub trait BoundKeyProvider: KeyProvider {
    /// Calcula una prueba HMAC sobre el desafío recibido.
    fn binding_proof(&self, message: &[u8]) -> Result<[u8; 32]>;
}

/// Calcula una prueba HMAC-SHA-256 de vinculación.
///
/// El secreto debe proceder de una configuración de confianza.
pub fn create_provider_binding_proof(
    binding_key: &[u8; 32],
    message: &[u8],
) -> Result<[u8; 32]> {
    let mut mac = HmacSha256::new_from_slice(binding_key)
        .map_err(|_| anyhow!("Invalid provider binding key"))?;

    mac.update(message);

    let output = mac.finalize().into_bytes();
    let mut proof = [0u8; 32];

    proof.copy_from_slice(&output);

    Ok(proof)
}