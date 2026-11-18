// Independent reusable command description: antispam.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antispam",
    es: "Protección spam; desactivada inicialmente",
    en: "Protection spam; initially disabled",
    usages: &["antispam on|off|inherit"],
    implemented: true,
};
