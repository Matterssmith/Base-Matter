// Independent reusable command description: antiflood.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti flood",
    es: "Protección flood; desactivada inicialmente",
    en: "Protection flood; initially disabled",
    usages: &["anti flood", "set anti flood on|off|inherit"],
    implemented: true,
};
