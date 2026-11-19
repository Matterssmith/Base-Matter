// Independent reusable command description: antigif.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti gif",
    es: "Protección gif; desactivada inicialmente",
    en: "Protection gif; initially disabled",
    usages: &["anti gif", "set anti gif on|off|inherit"],
    implemented: true,
};
