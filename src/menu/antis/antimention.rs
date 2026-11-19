// Independent reusable command description: antimention.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti mention",
    es: "Protección mention; desactivada inicialmente",
    en: "Protection mention; initially disabled",
    usages: &["anti mention", "set anti mention on|off|inherit"],
    implemented: true,
};
