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
        "set style --chat [matter|none|inherit]",
        "set style --user [matter|none|inherit] [@usuario]",
        "set priority --chat [user,chat,socket,global]",
        "set style --mod",
        "set style --global",
        "set style matter|none",
        "set --mod style matter|none",
        "set --global style matter|none",
        "set style inherit",
    ],
    implemented: true,
};
