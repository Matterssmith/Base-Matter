// Independent reusable command description: antigroup.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antigroup",
    es: "Protección group; desactivada inicialmente",
    en: "Protection group; initially disabled",
    usages: &["antigroup on|off|inherit"],
    implemented: true,
};
