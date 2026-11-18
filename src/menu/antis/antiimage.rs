// Independent reusable command description: antiimage.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antiimage",
    es: "Protección image; desactivada inicialmente",
    en: "Protection image; initially disabled",
    usages: &["antiimage on|off|inherit"],
    implemented: true,
};
