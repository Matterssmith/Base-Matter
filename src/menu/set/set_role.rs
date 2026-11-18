// Independent reusable command description: set_role.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "set role",
    es: "Gestionar rangos mediante Vault-Matter",
    en: "Manage ranks through Vault-Matter",
    usages: &["set role <identidad> <rango>"],
    implemented: false,
};
