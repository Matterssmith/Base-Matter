// Independent reusable command description: antilinktiktok.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antilinktiktok",
    es: "Protección linktiktok; desactivada inicialmente",
    en: "Protection linktiktok; initially disabled",
    usages: &["antilinktiktok on|off|inherit"],
    implemented: true,
};
