// Independent reusable command description: antimedia.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti media",
    es: "Protección media; desactivada inicialmente",
    en: "Protection media; initially disabled",
    usages: &["anti media", "set anti media on|off|inherit"],
    implemented: true,
};
