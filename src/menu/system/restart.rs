// Independent reusable command description: restart.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "restart",
    es: "Reinicio supervisado; no implementado",
    en: "Supervised restart; not implemented",
    usages: &["restart"],
    implemented: false,
};
