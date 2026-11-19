// Category roles: a Rust module per command.
use crate::menu::Category;
pub mod audit;
pub mod roles;

pub const CATEGORY: Category = Category {
    id: "roles",
    es: "𝗥𝗢𝗟𝗘𝗦 𝗬 𝗔𝗨𝗗𝗜𝗧𝗢𝗥𝗜́𝗔",
    en: "ROLES AND AUDIT",
    icon: "♛",
    commands: &[roles::COMMAND, audit::COMMAND],
};
