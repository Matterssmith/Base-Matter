// Independent reusable command description: reload.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "reload",
    es: "Recargar vistas del socket; código Rust nuevo requiere reinicio",
    en: "Refresh socket views; new compiled Rust code requires restart",
    usages: &["reload"],
    implemented: true,
};
