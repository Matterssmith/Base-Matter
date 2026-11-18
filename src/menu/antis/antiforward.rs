// Independent reusable command description: antiforward.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antiforward",
    es: "Protección forward; desactivada inicialmente",
    en: "Protection forward; initially disabled",
    usages: &["antiforward on|off|inherit"],
    implemented: true,
};
