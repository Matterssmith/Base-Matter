// Independent reusable command description: update.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "update",
    es: "Actualización supervisada; no implementada",
    en: "Supervised update; not implemented",
    usages: &["update"],
    implemented: false,
};
