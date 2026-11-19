// Category bot: a Rust module per command.
use crate::menu::Category;
#[path = "bot.rs"]
pub mod main_command;

pub const CATEGORY: Category = Category {
    id: "bot",
    es: "𝗔𝗖𝗧𝗜𝗩𝗔𝗖𝗜𝗢́𝗡 𝗗𝗘𝗟 𝗕𝗢𝗧",
    en: "BOT ACTIVATION",
    icon: "✦",
    commands: &[main_command::COMMAND],
};
