// Independent reusable command description: antispam.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti spam",
    es: "Protección spam; desactivada inicialmente",
    en: "Protection spam; initially disabled",
    usages: &["anti spam", "set anti spam on|off|inherit"],
    implemented: true,
};
