// Independent reusable command description: help.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "help",
    es: "Asistente de ayuda contextual; pendiente",
    en: "Contextual command assistance; pending",
    usages: &["help <pregunta>"],
    implemented: false,
};
