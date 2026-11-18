// Independent reusable command description: antilinkphishing.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antilinkphishing",
    es: "Protección linkphishing; desactivada inicialmente",
    en: "Protection linkphishing; initially disabled",
    usages: &["antilinkphishing on|off|inherit"],
    implemented: true,
};
