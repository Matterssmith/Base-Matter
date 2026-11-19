// Independent reusable command description: antiimage.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti image",
    es: "Protección image; desactivada inicialmente",
    en: "Protection image; initially disabled",
    usages: &["anti image", "set anti image on|off|inherit"],
    implemented: true,
};
