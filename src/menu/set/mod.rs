// Category set: a Rust module per command.
use crate::menu::Category;
pub mod set;
pub mod set_anti;
pub mod set_lang;
pub mod set_role;
pub mod set_style;

pub const CATEGORY: Category = Category {
    id: "set",
    es: "𝗔𝗗𝗠𝗜𝗡𝗜𝗦𝗧𝗥𝗔𝗖𝗜𝗢́𝗡",
    en: "CONFIGURATION",
    icon: "♛",
    commands: &[
        set::COMMAND,
        set_style::COMMAND,
        set_lang::COMMAND,
        set_role::COMMAND,
        set_anti::COMMAND,
    ],
};
