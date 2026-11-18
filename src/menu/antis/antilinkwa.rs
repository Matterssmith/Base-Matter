// Independent reusable command description: antilinkwa.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antilinkwa",
    es: "Protección linkwa; desactivada inicialmente",
    en: "Protection linkwa; initially disabled",
    usages: &["antilinkwa on|off|inherit"],
    implemented: true,
};
