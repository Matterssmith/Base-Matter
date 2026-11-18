// Independent reusable command description: antis.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antis",
    es: "Listar filtros, apagados por defecto",
    en: "List filters, off by default",
    usages: &["antis", "antis --global"],
    implemented: true,
};
