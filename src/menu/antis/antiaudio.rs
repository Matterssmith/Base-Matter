// Independent reusable command description: antiaudio.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti audio",
    es: "Protección audio; desactivada inicialmente",
    en: "Protection audio; initially disabled",
    usages: &["anti audio", "set anti audio on|off|inherit"],
    implemented: true,
};
