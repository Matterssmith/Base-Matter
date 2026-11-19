// Independent reusable command description: antivideo.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti movie",
    es: "Protección video; desactivada inicialmente",
    en: "Protection video; initially disabled",
    usages: &["anti movie", "set anti movie on|off|inherit"],
    implemented: true,
};
