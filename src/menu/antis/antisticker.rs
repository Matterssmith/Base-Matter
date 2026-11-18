// Independent reusable command description: antisticker.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antisticker",
    es: "Protección sticker; desactivada inicialmente",
    en: "Protection sticker; initially disabled",
    usages: &["antisticker on|off|inherit"],
    implemented: true,
};
