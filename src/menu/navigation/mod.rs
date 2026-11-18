// Category navigation: a Rust module per command.
use crate::menu::Category;
pub mod menu;
pub mod credits;

pub const CATEGORY: Category = Category {
    id: "navigation",
    es: "𝗡𝗔𝗩𝗘𝗚𝗔𝗖𝗜𝗢́𝗡",
    en: "NAVIGATION",
    icon: "☷",
    commands: &[
        menu::COMMAND,
        credits::COMMAND,
    ],
};
