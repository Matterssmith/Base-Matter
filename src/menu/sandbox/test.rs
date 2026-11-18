// Independent reusable command description: test.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "test",
    es: "Prueba directa de código o archivo; pendiente",
    en: "Direct source/file tests; pending",
    usages: &["test <archivo|codigo>"],
    implemented: false,
};
