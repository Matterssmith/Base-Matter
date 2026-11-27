// Independent reusable command description: set_style.
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "set style",
    es: "Consultar o cambiar el estilo visual del menú",
    en: "Inspect or change menu visual style",
    usages: &[
        "set style",
        "set style --mod",
        "set style matter|none",
        "set --mod style matter|none",
    ],
    implemented: true,
};
