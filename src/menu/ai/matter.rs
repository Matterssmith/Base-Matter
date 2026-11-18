// Independent reusable command description: matter.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "matter",
    es: "Asistente de código, requiere broker y autorización Vault",
    en: "Coding assistant; requires Vault authorization",
    usages: &["matter status", "matter ask <pregunta>", "matter plan <tarea>"],
    implemented: true,
};
