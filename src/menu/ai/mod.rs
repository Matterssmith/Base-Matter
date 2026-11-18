// Category ai: a Rust module per command.
use crate::menu::Category;
pub mod matter;
pub mod help;

pub const CATEGORY: Category = Category {
    id: "ai",
    es: "𝗔𝗦𝗜𝗦𝗧𝗘𝗡𝗧𝗘 𝗜𝗔",
    en: "AI ASSISTANT",
    icon: "✧",
    commands: &[
        matter::COMMAND,
        help::COMMAND,
    ],
};
