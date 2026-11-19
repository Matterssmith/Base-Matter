// Independent reusable command description: antidelete.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti delete",
    es: "Protección delete; desactivada inicialmente",
    en: "Protection delete; initially disabled",
    usages: &["anti delete", "set anti delete on|off|inherit"],
    implemented: true,
};
