// Independent reusable command description: antilinkinstagram.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antilinkinstagram",
    es: "Protección linkinstagram; desactivada inicialmente",
    en: "Protection linkinstagram; initially disabled",
    usages: &["antilinkinstagram on|off|inherit"],
    implemented: true,
};
