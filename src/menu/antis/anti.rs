// Independent reusable command description: anti (read-only).
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti",
    es: "Consultar el estado de las protecciones",
    en: "Inspect protection states without changing settings",
    usages: &[
        "anti",
        "anti --mod",
        "anti --global",
        "anti <tipo>",
        "anti --mod <tipo>",
        "anti --global <tipo>",
        "anti audit",
        "anti --mod audit",
    ],
    implemented: true,
};
