// Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "set update",
    es: "Política de activación tras actualizar: manual, restart o reload (owner global; reload no disponible todavía)",
    en: "Activation policy: manual, restart or reload (global owner; reload not yet available)",
    usages: &["set update", "set update manual|restart|reload"],
    implemented: true,
};
