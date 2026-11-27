use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "update",
    es: "Actualizar alpha y reiniciar (owner global; supervisor local)",
    en: "Update alpha and restart (global owner; local supervisor)",
    usages: &["update"],
    implemented: true,
};
