// Independent reusable command description: audit.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "audit",
    es: "Consultar registros locales del socket",
    en: "Inspect local socket audit records",
    usages: &["audit"],
    implemented: true,
};
