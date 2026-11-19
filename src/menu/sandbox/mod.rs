// Category sandbox: a Rust module per command.
use crate::menu::Category;
pub mod console;
pub mod eval;
#[path = "sandbox.rs"]
pub mod main_command;
pub mod test;

pub const CATEGORY: Category = Category {
    id: "sandbox",
    es: "𝗣𝗥𝗨𝗘𝗕𝗔𝗦 𝗬 𝗖𝗢𝗡𝗦𝗢𝗟𝗔",
    en: "TESTS AND CONSOLE",
    icon: "◈",
    commands: &[
        main_command::COMMAND,
        test::COMMAND,
        console::COMMAND,
        eval::COMMAND,
    ],
};
