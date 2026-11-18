// Independent reusable command description: status.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "status",
    es: "Estado del chat; --global consulta las políticas del socket",
    en: "Current chat status; --global shows socket policies",
    usages: &["status", "status --global"],
    implemented: true,
};
