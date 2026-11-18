// Independent reusable command description: menu.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "menu",
    es: "Menú automático por categoría y página",
    en: "Automatic menu by category and page",
    usages: &["menu", "menu <categoria>", "menu <categoria> <pagina>"],
    implemented: true,
};
