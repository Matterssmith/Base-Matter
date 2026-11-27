// Category set: a Rust module per command.
use crate::menu::Category;
#[path = "set.rs"]
pub mod main_command;
pub mod set_anti;
pub mod set_antilink;
pub mod set_lang;
pub mod set_menu;
pub mod set_role;
pub mod set_style;
pub mod set_update;

pub const CATEGORY: Category = Category {
    id: "set",
    es: "𝗔𝗗𝗠𝗜𝗡𝗜𝗦𝗧𝗥𝗔𝗖𝗜𝗢́𝗡",
    en: "CONFIGURATION",
    icon: "♛",
    commands: &[
        main_command::COMMAND,
        set_style::COMMAND,
        set_update::COMMAND,
        set_lang::COMMAND,
        set_menu::COMMAND,
        set_role::COMMAND,
        set_anti::COMMAND,
    ],
};
