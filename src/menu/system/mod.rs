// Category system: a Rust module per command.
use crate::menu::Category;
pub mod codex;
#[path = "system.rs"]
pub mod main_command;
pub mod reload;
pub mod restart;
pub mod update;
pub mod vault;

pub const CATEGORY: Category = Category {
    id: "system",
    es: "𝗦𝗜𝗦𝗧𝗘𝗠𝗔",
    en: "SYSTEM",
    icon: "⚙︎",
    commands: &[
        main_command::COMMAND,
        update::COMMAND,
        restart::COMMAND,
        reload::COMMAND,
        vault::COMMAND,
        codex::COMMAND,
    ],
};
