// Independent reusable command description: antigroup.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti group",
    es: "Protección group; desactivada inicialmente",
    en: "Protection group; initially disabled",
    usages: &["anti group", "set anti group on|off|inherit"],
    implemented: true,
};
