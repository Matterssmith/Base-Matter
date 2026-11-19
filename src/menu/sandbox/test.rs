// Command description: safe WhatsApp UI tests; code execution is NOT available.
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "test",
    es: "Pruebas visuales en WhatsApp (sin ejecutar código)",
    en: "Visual WhatsApp tests (no code execution)",
    usages: &["test", "test ui", "test html"],
    implemented: true,
};
