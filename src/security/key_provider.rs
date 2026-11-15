
/*
 * Matter-Base — Key Provider
 *
 * Define el contrato para recuperar material criptográfico
 * desde un proveedor seguro y autenticado.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::Result;
use zeroize::Zeroizing;

/// Proveedor externo de claves criptográficas.
///
/// Las implementaciones deben:
/// - Autenticar la identidad solicitante.
/// - Autorizar el acceso a cada clave.
/// - Verificar la integridad y autenticidad del sobre.
/// - Comprobar el vault y el formato de los registros.
/// - Entregar las claves en memoria protegida.
/// - No registrar ni persistir claves en texto plano.
pub trait KeyProvider: Send + Sync {
    /// Devuelve el identificador del Vault consultado.
    fn vault_id(&self) -> Result<String>;

    /// Devuelve los identificadores de las claves disponibles.
    fn key_ids(&self) -> Result<Vec<String>>;

    /// Devuelve el identificador de la clave activa.
    ///
    /// `None` indica que el proveedor no tiene una clave activa.
    fn active_key_id(&self) -> Result<Option<String>>;

    /// Recupera una clave mediante su identificador.
    ///
    /// La implementación debe devolver únicamente claves
    /// autenticadas y autorizadas para el solicitante.
    fn load_key(&self, key_id: &str) -> Result<Zeroizing<[u8; 32]>>;
}
