// Independent reusable command description: antisticker.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti sticker",
    es: "Protección sticker; desactivada inicialmente",
    en: "Protection sticker; initially disabled",
    usages: &["anti sticker", "set anti sticker on|off|inherit"],
    implemented: true,
};
