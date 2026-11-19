// Independent reusable command description: antichannel.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti channel",
    es: "Protección channel; desactivada inicialmente",
    en: "Protection channel; initially disabled",
    usages: &["anti channel", "set anti channel on|off|inherit"],
    implemented: true,
};
