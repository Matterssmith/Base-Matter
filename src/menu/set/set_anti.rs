// Independent reusable command description: set_anti.
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "set anti",
    es: "Configurar una protección para el grupo o para este socket",
    en: "Configure a protection for the group or this socket",
    usages: &[
        "set anti <tipo> on|off|inherit",
        "set anti --mod <tipo> on|off",
        "set anti <tipo> action warn|delete|kick",
        "set anti --mod <tipo> action warn|delete|kick",
    ],
    implemented: true,
};
