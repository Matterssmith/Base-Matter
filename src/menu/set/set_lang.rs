// Independent reusable command description: set_lang.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "set lang",
    es: "Cambiar idioma: español o inglés",
    en: "Change menu language: Spanish or English",
    usages: &["set lang es|en"],
    implemented: true,
};
