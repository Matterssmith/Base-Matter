// Independent reusable command description: antiaudio.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antiaudio",
    es: "Protección audio; desactivada inicialmente",
    en: "Protection audio; initially disabled",
    usages: &["antiaudio on|off|inherit"],
    implemented: true,
};
