// Independent reusable command description: anti.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "anti",
    es: "Consultar o configurar una protección",
    en: "Inspect or configure a protection",
    usages: &["anti <tipo>", "anti <tipo> on|off|inherit"],
    implemented: true,
};
