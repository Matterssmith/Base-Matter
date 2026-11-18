// Independent reusable command description: antivideo.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antivideo",
    es: "Protección video; desactivada inicialmente",
    en: "Protection video; initially disabled",
    usages: &["antivideo on|off|inherit"],
    implemented: true,
};
