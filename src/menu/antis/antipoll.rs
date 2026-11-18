// Independent reusable command description: antipoll.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antipoll",
    es: "Protección poll; desactivada inicialmente",
    en: "Protection poll; initially disabled",
    usages: &["antipoll on|off|inherit"],
    implemented: true,
};
