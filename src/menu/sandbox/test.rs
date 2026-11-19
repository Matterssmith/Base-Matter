// Command description: safe WhatsApp UI tests; code execution is NOT available.
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "test",
    es: "Pruebas visuales NativeFlow, ButtonsRow y HTML (sin ejecutar código)",
    en: "NativeFlow, ButtonsRow and HTML tests (no code execution)",
    usages: &["test", "test ui", "test row", "test html"],
    implemented: true,
};
