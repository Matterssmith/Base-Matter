// Independent reusable command description: antibot.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antibot",
    es: "Protección bot; desactivada inicialmente",
    en: "Protection bot; initially disabled",
    usages: &["antibot on|off|inherit"],
    implemented: true,
};
