// Independent reusable command description: set.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "set",
    es: "Mostrar configuraciones disponibles",
    en: "Show available settings",
    usages: &["set"],
    implemented: true,
};
