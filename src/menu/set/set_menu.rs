// Independent reusable command description: set_menu.
// Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "set menu",
    es: "Consultar o elegir el menú predeterminado para el socket propietario",
    en: "Inspect or choose the owner's default menu transport",
    usages: &[
        "set menu",
        "set menu web",
        "set menu matter",
        "set menu buttons",
    ],
    implemented: true,
};
