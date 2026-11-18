// Independent reusable command description: antimention.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antimention",
    es: "Protección mention; desactivada inicialmente",
    en: "Protection mention; initially disabled",
    usages: &["antimention on|off|inherit"],
    implemented: true,
};
