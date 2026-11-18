// Independent reusable command description: antidocument.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antidocument",
    es: "Protección document; desactivada inicialmente",
    en: "Protection document; initially disabled",
    usages: &["antidocument on|off|inherit"],
    implemented: true,
};
