// Independent reusable command description: roles.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "roles",
    es: "Consulta local; Vault aún sin sincronización de permisos",
    en: "Local query; Vault role synchronization unavailable",
    usages: &["roles"],
    implemented: true,
};
