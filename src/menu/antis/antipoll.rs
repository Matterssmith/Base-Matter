// Independent reusable command description: antipoll.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti poll",
    es: "Protección poll; desactivada inicialmente",
    en: "Protection poll; initially disabled",
    usages: &["anti poll", "set anti poll on|off|inherit"],
    implemented: true,
};
