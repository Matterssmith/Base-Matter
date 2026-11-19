// Category bot: a Rust module per command.
use crate::menu::Category;
pub mod bot;

pub const CATEGORY: Category = Category {
    id: "bot",
    es: "𝗔𝗖𝗧𝗜𝗩𝗔𝗖𝗜𝗢́𝗡 𝗗𝗘𝗟 𝗕𝗢𝗧",
    en: "BOT ACTIVATION",
    icon: "✦",
    commands: &[bot::COMMAND],
};
