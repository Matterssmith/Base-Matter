// Independent reusable command description: anticontact.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti contact",
    es: "Protección contact; desactivada inicialmente",
    en: "Protection contact; initially disabled",
    usages: &["anti contact", "set anti contact on|off|inherit"],
    implemented: true,
};
