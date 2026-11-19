// Independent reusable command description: antistatus.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti status",
    es: "Protección status; desactivada inicialmente",
    en: "Protection status; initially disabled",
    usages: &["anti status", "set anti status on|off|inherit"],
    implemented: true,
};
