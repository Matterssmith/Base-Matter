// Independent reusable command description: set_style.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "set style",
    es: "Cambiar estilo visual: classic o none",
    en: "Change menu style: classic or none",
    usages: &["set style classic|none"],
    implemented: true,
};
