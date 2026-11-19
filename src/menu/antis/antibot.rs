// Independent reusable command description: antibot.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti bot",
    es: "Protección bot; desactivada inicialmente",
    en: "Protection bot; initially disabled",
    usages: &["anti bot", "set anti bot on|off|inherit"],
    implemented: true,
};
