// Independent reusable command description: sandbox.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "sandbox",
    es: "Consola y pruebas aisladas; pendiente",
    en: "Isolated console and tests; pending",
    usages: &[
        "sandbox",
        "sandbox console <orden>",
        "sandbox test <archivo|codigo>",
    ],
    implemented: false,
};
