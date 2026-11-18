// Independent reusable command description: antiflood.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antiflood",
    es: "Protección flood; desactivada inicialmente",
    en: "Protection flood; initially disabled",
    usages: &["antiflood on|off|inherit"],
    implemented: true,
};
