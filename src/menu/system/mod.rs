// Category system: a Rust module per command.
use crate::menu::Category;
pub mod system;
pub mod update;
pub mod restart;
pub mod reload;

pub const CATEGORY: Category = Category {
    id: "system",
    es: "𝗦𝗜𝗦𝗧𝗘𝗠𝗔",
    en: "SYSTEM",
    icon: "⚙︎",
    commands: &[
        system::COMMAND,
        update::COMMAND,
        restart::COMMAND,
        reload::COMMAND,
    ],
};
