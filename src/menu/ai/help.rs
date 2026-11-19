// Independent reusable command description: help.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "help",
    es: "Ayuda contextual de comandos por categoría",
    en: "Contextual command help by category",
    usages: &["help", "help all", "help <categoria>", "help <comando>"],
    implemented: true,
};
