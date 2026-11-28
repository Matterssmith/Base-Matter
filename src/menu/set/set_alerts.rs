// Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "set alerts",
    es: "Configurar alertas de comandos por reacción, texto o silencio",
    en: "Configure command alerts: reaction, text or silence",
    usages: &[
        "set alerts",
        "set alerts reaction|text|off",
        "set alerts processing|success|admission|denied|error|inactive reaction|text|off",
    ],
    implemented: true,
};
