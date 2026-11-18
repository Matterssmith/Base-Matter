// Independent reusable command description: antilinkx.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antilinkx",
    es: "Protección linkx; desactivada inicialmente",
    en: "Protection linkx; initially disabled",
    usages: &["antilinkx on|off|inherit"],
    implemented: true,
};
