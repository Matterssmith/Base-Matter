// Independent reusable command description: antigif.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antigif",
    es: "Protección gif; desactivada inicialmente",
    en: "Protection gif; initially disabled",
    usages: &["antigif on|off|inherit"],
    implemented: true,
};
