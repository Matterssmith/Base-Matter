// Independent reusable command description: antistatus.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antistatus",
    es: "Protección status; desactivada inicialmente",
    en: "Protection status; initially disabled",
    usages: &["antistatus on|off|inherit"],
    implemented: true,
};
