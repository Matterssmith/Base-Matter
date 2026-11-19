// Independent reusable command description: antilocation.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti location",
    es: "Protección location; desactivada inicialmente",
    en: "Protection location; initially disabled",
    usages: &["anti location", "set anti location on|off|inherit"],
    implemented: true,
};
