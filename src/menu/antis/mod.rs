// Category antis: a Rust module per command.
use crate::menu::Category;
pub mod anti;
pub mod antiaudio;
pub mod antibot;
pub mod anticall;
pub mod antichannel;
pub mod anticontact;
pub mod antidelete;
pub mod antidocument;
pub mod antiflood;
pub mod antiforward;
pub mod antigif;
pub mod antigroup;
pub mod antiimage;
pub mod antilink;
pub mod antilocation;
pub mod antimedia;
pub mod antimention;
pub mod antipoll;
pub mod antispam;
pub mod antistatus;
pub mod antisticker;
pub mod antivideo;
pub mod antiviewonce;
#[path = "antis.rs"]
pub mod listing_command;

pub const CATEGORY: Category = Category {
    id: "antis",
    es: "𝗣𝗥𝗢𝗧𝗘𝗖𝗖𝗜𝗢𝗡𝗘𝗦",
    en: "PROTECTIONS",
    icon: "♧",
    commands: &[
        listing_command::COMMAND,
        anti::COMMAND,
        antilink::COMMAND,
        antigroup::COMMAND,
        antichannel::COMMAND,
        antistatus::COMMAND,
        antiviewonce::COMMAND,
        anticall::COMMAND,
        antisticker::COMMAND,
        antiimage::COMMAND,
        antivideo::COMMAND,
        antiaudio::COMMAND,
        antidocument::COMMAND,
        antigif::COMMAND,
        antilocation::COMMAND,
        anticontact::COMMAND,
        antipoll::COMMAND,
        antiforward::COMMAND,
        antispam::COMMAND,
        antiflood::COMMAND,
        antimention::COMMAND,
        antibot::COMMAND,
        antimedia::COMMAND,
        antidelete::COMMAND,
    ],
};
