// Independent reusable command description: antiviewonce.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti oneview",
    es: "Protección viewonce; desactivada inicialmente",
    en: "Protection viewonce; initially disabled",
    usages: &["anti oneview", "set anti oneview on|off|inherit"],
    implemented: true,
};
