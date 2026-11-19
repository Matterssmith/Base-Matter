// Independent reusable command description: set_lang.
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "set lang",
    es: "Consultar o cambiar el idioma del menú",
    en: "Inspect or change menu language",
    usages: &[
        "set lang",
        "set lang --mod",
        "set lang es|en",
        "set --mod lang es|en",
    ],
    implemented: true,
};
