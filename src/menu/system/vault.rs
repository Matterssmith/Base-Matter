// Base-Matter — Command descriptor for protected Vault connection status.
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "vault",
    es: "Estado del enlace cifrado al servicio Vault (sólo chat propio)",
    en: "Encrypted Vault service link status (owner self-chat only)",
    usages: &["vault", "vault status", "vault test"],
    implemented: true,
};
