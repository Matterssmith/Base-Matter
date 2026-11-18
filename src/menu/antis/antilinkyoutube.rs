// Independent reusable command description: antilinkyoutube.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antilinkyoutube",
    es: "Protección linkyoutube; desactivada inicialmente",
    en: "Protection linkyoutube; initially disabled",
    usages: &["antilinkyoutube on|off|inherit"],
    implemented: true,
};
