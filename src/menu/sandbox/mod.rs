// Category sandbox: a Rust module per command.
use crate::menu::Category;
pub mod console;
pub mod eval;
pub mod sandbox;
pub mod test;

pub const CATEGORY: Category = Category {
    id: "sandbox",
    es: "𝗣𝗥𝗨𝗘𝗕𝗔𝗦 𝗬 𝗖𝗢𝗡𝗦𝗢𝗟𝗔",
    en: "TESTS AND CONSOLE",
    icon: "◈",
    commands: &[
        sandbox::COMMAND,
        test::COMMAND,
        console::COMMAND,
        eval::COMMAND,
    ],
};
