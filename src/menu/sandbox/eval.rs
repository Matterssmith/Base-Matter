// Independent reusable command description: eval.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "eval",
    es: "Evaluación directa sin aislamiento; pendiente",
    en: "Direct, unisolated evaluation; pending",
    usages: &["eval <codigo>"],
    implemented: false,
};
