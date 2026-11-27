use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "restart",
    es: "Reiniciar el bot sin desvincular WhatsApp (owner global)",
    en: "Restart the bot without unlinking WhatsApp (global owner)",
    usages: &["restart"],
    implemented: true,
};
