// Independent reusable command description: anticontact.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anticontact",
    es: "Protección contact; desactivada inicialmente",
    en: "Protection contact; initially disabled",
    usages: &["anticontact on|off|inherit"],
    implemented: true,
};
