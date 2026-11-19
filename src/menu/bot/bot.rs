// Independent reusable command description: bot.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "bot",
    es: "Activar o heredar políticas del chat o socket",
    en: "Activate or inherit chat and socket policies",
    usages: &[
        "bot on|off|inherit",
        "bot --global on|off|inherit",
        "bot group|private|channel|community on|off|inherit",
    ],
    implemented: true,
};
