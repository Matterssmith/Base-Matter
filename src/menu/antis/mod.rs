// Category antis: a Rust module per command.
use crate::menu::Category;
pub mod antis;
pub mod anti;
pub mod antilink;
pub mod antilinkwa;
pub mod antilinktelegram;
pub mod antilinkdiscord;
pub mod antilinkinstagram;
pub mod antilinkfacebook;
pub mod antilinktiktok;
pub mod antilinkyoutube;
pub mod antilinkx;
pub mod antilinkshort;
pub mod antilinkphishing;
pub mod antigroup;
pub mod antichannel;
pub mod antistatus;
pub mod antiviewonce;
pub mod anticall;
pub mod antisticker;
pub mod antiimage;
pub mod antivideo;
pub mod antiaudio;
pub mod antidocument;
pub mod antigif;
pub mod antilocation;
pub mod anticontact;
pub mod antipoll;
pub mod antiforward;
pub mod antispam;
pub mod antiflood;
pub mod antimention;
pub mod antibot;
pub mod antimedia;
pub mod antidelete;

pub const CATEGORY: Category = Category {
    id: "antis",
    es: "𝗣𝗥𝗢𝗧𝗘𝗖𝗖𝗜𝗢𝗡𝗘𝗦",
    en: "PROTECTIONS",
    icon: "♧",
    commands: &[
        antis::COMMAND,
        anti::COMMAND,
        antilink::COMMAND,
        antilinkwa::COMMAND,
        antilinktelegram::COMMAND,
        antilinkdiscord::COMMAND,
        antilinkinstagram::COMMAND,
        antilinkfacebook::COMMAND,
        antilinktiktok::COMMAND,
        antilinkyoutube::COMMAND,
        antilinkx::COMMAND,
        antilinkshort::COMMAND,
        antilinkphishing::COMMAND,
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
