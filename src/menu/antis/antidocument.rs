// Independent reusable command description: antidocument.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti document",
    es: "Protección document; desactivada inicialmente",
    en: "Protection document; initially disabled",
    usages: &["anti document", "set anti document on|off|inherit"],
    implemented: true,
};
