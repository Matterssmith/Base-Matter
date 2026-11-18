// Independent reusable command description: antilink.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antilink",
    es: "Protección link; desactivada inicialmente",
    en: "Protection link; initially disabled",
    usages: &["antilink on|off|inherit"],
    implemented: true,
};
