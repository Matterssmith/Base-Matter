// Independent reusable command description: status.
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "status",
    es: "Consultar estado del chat, socket o valor predeterminado general",
    en: "Inspect current chat, socket or system default status",
    usages: &["status", "status --mod", "status --global"],
    implemented: true,
};
