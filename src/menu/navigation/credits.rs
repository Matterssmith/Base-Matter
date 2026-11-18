// Independent reusable command description: credits.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "credits",
    es: "Créditos del proyecto a solicitud, no en el menú",
    en: "Project credits on request, not in the menu",
    usages: &["credits"],
    implemented: true,
};
