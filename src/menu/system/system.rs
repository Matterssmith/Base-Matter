// Independent reusable command description: system.
use crate::menu::CommandSpec;

pub const COMMAND: CommandSpec = CommandSpec {
    name: "system",
    es: "Estado de servicios y sockets",
    en: "Service and socket status",
    usages: &["system"],
    implemented: false,
};
