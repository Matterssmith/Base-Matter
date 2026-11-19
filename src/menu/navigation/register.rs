// Public optional account enrollment information; does not create an account.
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "register",
    es: "Registro voluntario; el modo invitado permite funciones básicas",
    en: "Optional registration; guest access allows basic features",
    usages: &["register"],
    implemented: true,
};
