// Category status: a Rust module per command.
use crate::menu::Category;
pub mod status;

pub const CATEGORY: Category = Category {
    id: "status",
    es: "𝗘𝗦𝗧𝗔𝗗𝗢 𝗬 𝗖𝗢𝗡𝗦𝗨𝗟𝗧𝗔𝗦",
    en: "STATUS AND CONSULTATION",
    icon: "◈",
    commands: &[status::COMMAND],
};
