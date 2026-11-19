// Dedicated AntiLink domain-list configuration: no platform-specific filters.
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "set anti link",
    es: "Agregar, eliminar o consultar dominios del chat o socket",
    en: "Add, remove or list blocked domains per chat or socket",
    usages: &[
        "set anti link list",
        "set anti link add <dominio> [dominio...]",
        "set anti link del <dominio> [dominio...]",
        "set anti link --global add <dominio> [dominio...]",
        "set anti link --global list",
    ],
    implemented: true,
};
