// Independent reusable command description: antidelete.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antidelete",
    es: "Protección delete; desactivada inicialmente",
    en: "Protection delete; initially disabled",
    usages: &["antidelete on|off|inherit"],
    implemented: true,
};
