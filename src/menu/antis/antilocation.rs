// Independent reusable command description: antilocation.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antilocation",
    es: "Protección location; desactivada inicialmente",
    en: "Protection location; initially disabled",
    usages: &["antilocation on|off|inherit"],
    implemented: true,
};
