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
///
/// Las implementaciones deben:
/// - Autenticar la identidad solicitante.
/// - Autorizar el acceso a cada clave.
/// - Verificar la integridad y autenticidad de los registros.
/// - Comprobar el Vault y el formato de los registros.
/// - Entregar las claves en memoria protegida.
/// - No registrar ni persistir claves en texto plano.
pub trait KeyProvider: Send + Sync {
    /// Devuelve el identificador del Vault consultado.
    fn vault_id(&self) -> Result<String>;

    /// Devuelve los identificadores de las claves disponibles.
    fn key_ids(&self) -> Result<Vec<String>>;

    /// Devuelve el identificador de la clave activa.
    ///
    /// `None` indica que no existe una clave activa.
    fn active_key_id(&self) -> Result<Option<String>>;

    /// Recupera una clave mediante su identificador.
    fn load_key(&self, key_id: &str) -> Result<Zeroizing<[u8; 32]>>;
}

/// Proveedor que demuestra posesión del secreto autenticado.
///
/// La prueba debe calcularse con el secreto privado del proveedor.
/// No debe aceptar como válida una identidad o prueba suministrada
/// libremente por el solicitante.
pub trait BoundKeyProvider: KeyProvider {
    /// Calcula una prueba HMAC-SHA-256 sobre el mensaje recibido.
    fn binding_proof(&self, message: &[u8]) -> Result<[u8; 32]>;
}

/// Deriva una clave específica para vinculación de proveedores.
///
/// La separación de dominio evita reutilizar directamente
/// la clave de cifrado de afirmaciones para otros protocolos.
pub fn derive_provider_binding_key(
    authentication_key: &[u8; 32],
) -> Result<Zeroizing<[u8; 32]>> {
    let mut mac = HmacSha256::new_from_slice(authentication_key)
        .map_err(|_| anyhow!("Invalid provider authentication key"))?;

    mac.update(b"matter-base:provider-binding-key:v1");

    let output = mac.finalize().into_bytes();
    let mut derived = Zeroizing::new([0u8; 32]);

    derived.copy_from_slice(&output);

    Ok(derived)
}

/// Genera la prueba que debe devolver el proveedor.
///
/// El adaptador del proveedor debe conservar su secreto
/// fuera del repositorio y de los registros de ejecución.
pub fn create_provider_binding_proof(
    authentication_key: &[u8; 32],
    message: &[u8],
) -> Result<[u8; 32]> {
    let binding_key =
        derive_provider_binding_key(authentication_key)?;

    let mut mac = HmacSha256::new_from_slice(&binding_key)
        .map_err(|_| anyhow!("Invalid provider binding key"))?;

    mac.update(message);

    let output = mac.finalize().into_bytes();
    let mut proof = [0u8; 32];

    proof.copy_from_slice(&output);

    Ok(proof)
}