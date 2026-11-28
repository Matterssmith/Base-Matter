// Independent reusable command description: set.
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "set",
    es: "Mostrar el estado actual del menú y consultar opciones de configuración",
    en: "Show current menu settings and configuration options",
    usages: &[
        "set",
        "set --mod",
        "set --global",
        "set style classic|none",
        "set lang es|en",
        "set menu web|classic|buttons",
        "set update manual|restart|reload",
        "set alerts reaction|text|off",
        "set alerts processing|success|admission|denied|error|inactive reaction|text|off",
    ],
    implemented: true,
};
