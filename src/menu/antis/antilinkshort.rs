// Independent reusable command description: antilinkshort.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antilinkshort",
    es: "Protección linkshort; desactivada inicialmente",
    en: "Protection linkshort; initially disabled",
    usages: &["antilinkshort on|off|inherit"],
    implemented: true,
};
