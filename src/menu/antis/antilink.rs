// Independent reusable command description: antilink.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "antilink",
    es: "Un único filtro de enlaces configurables, apagado inicialmente",
    en: "One configurable link filter, initially disabled",
    usages: &["antilink on|off|inherit"],
    implemented: true,
};
