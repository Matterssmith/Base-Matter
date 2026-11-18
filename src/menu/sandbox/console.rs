// Independent reusable command description: console.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "console",
    es: "Consola directa sin aislamiento; pendiente",
    en: "Direct, unisolated console; pending",
    usages: &["console <orden>"],
    implemented: false,
};
