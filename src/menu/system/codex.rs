// Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "codex",
    es: "Enviar tarea al trabajador de desarrollo aislado (Owner principal; privado)",
    en: "Submit a task to the isolated development worker (primary owner; private)",
    usages: &["codex", "codex <instrucción>", "codex status"],
    implemented: true,
};
