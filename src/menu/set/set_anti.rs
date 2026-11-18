// Independent reusable command description: set_anti.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "set anti",
    es: "Definir la acción warn, delete o kick de un filtro",
    en: "Configure warn, delete or kick action for a filter",
    usages: &["set anti <tipo> action warn|delete|kick"],
    implemented: true,
};
