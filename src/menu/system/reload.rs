// Independent reusable command description: reload.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "reload",
    es: "Recarga supervisada; no implementada",
    en: "Supervised reload; not implemented",
    usages: &["reload"],
    implemented: false,
};
