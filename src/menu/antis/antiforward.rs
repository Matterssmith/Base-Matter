// Independent reusable command description: antiforward.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti forward",
    es: "Protección forward; desactivada inicialmente",
    en: "Protection forward; initially disabled",
    usages: &["anti forward", "set anti forward on|off|inherit"],
    implemented: true,
};
