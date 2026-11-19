// Independent reusable command description: anticall.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti call",
    es: "Protección call; desactivada inicialmente",
    en: "Protection call; initially disabled",
    usages: &["anti call", "set anti call on|off|inherit"],
    implemented: true,
};
