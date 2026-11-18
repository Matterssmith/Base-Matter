// Independent reusable command description: antichannel.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antichannel",
    es: "Protección channel; desactivada inicialmente",
    en: "Protection channel; initially disabled",
    usages: &["antichannel on|off|inherit"],
    implemented: true,
};
