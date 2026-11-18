// Independent reusable command description: antilinkdiscord.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antilinkdiscord",
    es: "Protección linkdiscord; desactivada inicialmente",
    en: "Protection linkdiscord; initially disabled",
    usages: &["antilinkdiscord on|off|inherit"],
    implemented: true,
};
