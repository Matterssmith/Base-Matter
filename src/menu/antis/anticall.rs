// Independent reusable command description: anticall.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anticall",
    es: "Protección call; desactivada inicialmente",
    en: "Protection call; initially disabled",
    usages: &["anticall on|off|inherit"],
    implemented: true,
};
