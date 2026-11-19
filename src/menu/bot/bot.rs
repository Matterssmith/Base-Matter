// Independent reusable command description: bot.
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "bot",
    es: "Consultar o configurar el bot según su ámbito",
    en: "Inspect or configure bot state per scope",
    usages: &[
        "bot",
        "bot --mod",
        "bot --global",
        "bot on|off|inherit",
        "bot --mod on|off|inherit",
        "bot group|private|channel|community on|off|inherit",
    ],
    implemented: true,
};
