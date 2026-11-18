// Independent reusable command description: antilinktelegram.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antilinktelegram",
    es: "Protección linktelegram; desactivada inicialmente",
    en: "Protection linktelegram; initially disabled",
    usages: &["antilinktelegram on|off|inherit"],
    implemented: true,
};
