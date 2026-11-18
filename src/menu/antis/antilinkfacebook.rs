// Independent reusable command description: antilinkfacebook.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antilinkfacebook",
    es: "Protección linkfacebook; desactivada inicialmente",
    en: "Protection linkfacebook; initially disabled",
    usages: &["antilinkfacebook on|off|inherit"],
    implemented: true,
};
