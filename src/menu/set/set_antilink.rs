// Dedicated AntiLink domain-list configuration: no platform-specific filters.
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "set anti link",
    es: "Consultar filtro y lista de dominios o cambiarla en el chat/socket",
    en: "Inspect link filter/domains or edit them for this chat/socket",
    usages: &[
        "set anti link",
        "set anti --mod link",
        "set anti link list",
        "set anti link add <dominio> [dominio...]",
        "set anti link del <dominio> [dominio...]",
        "set anti --mod link add <dominio> [dominio...]",
        "set anti --mod link list",
    ],
    implemented: true,
};
