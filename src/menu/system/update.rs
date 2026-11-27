use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "update",
    es: "Actualizar código alpha sin reiniciar (owner global)",
    en: "Update alpha source without restarting (global owner)",
    usages: &["update"],
    implemented: true,
};
