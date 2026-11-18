// Independent reusable command description: antiviewonce.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antiviewonce",
    es: "Protección viewonce; desactivada inicialmente",
    en: "Protection viewonce; initially disabled",
    usages: &["antiviewonce on|off|inherit"],
    implemented: true,
};
