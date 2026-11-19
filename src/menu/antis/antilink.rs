// Independent reusable command description: antilink.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti link",
    es: "Un único filtro de enlaces configurables, apagado inicialmente",
    en: "One configurable link filter, initially disabled",
    usages: &["anti link", "set anti link on|off|inherit"],
    implemented: true,
};
