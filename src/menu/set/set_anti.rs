// Independent reusable command description: set_anti.
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "set anti",
    es: "Consultar o configurar protecciones del chat y predeterminados del socket",
    en: "Inspect or configure chat protection and socket defaults",
    usages: &[
        "set anti",
        "set anti --mod",
        "set anti --global",
        "set anti <tipo> on|off|inherit",
        "set anti --mod <tipo> on|off",
        "set anti <tipo> action warn|delete|kick",
        "set anti --mod <tipo> action warn|delete|kick",
    ],
    implemented: true,
};
