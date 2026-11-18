// Independent reusable command description: antimedia.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antimedia",
    es: "Protección media; desactivada inicialmente",
    en: "Protection media; initially disabled",
    usages: &["antimedia on|off|inherit"],
    implemented: true,
};
