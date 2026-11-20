/*
 * Matter-Rust — offline platform readiness command.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use matter_rust_core::{MatterRust, Platform, ENGINE_NAME};

fn main() {
    let engine = MatterRust::new();
    println!("{ENGINE_NAME} | registered transport sessions: {}", engine.registered_sessions());
    for platform in [
        Platform::WhatsApp,
        Platform::Telegram,
        Platform::Discord,
        Platform::SocialMatter,
    ] {
        let state = if platform.production_enabled() {
            "connector-required"
        } else {
            "reserved-disabled"
        };
        println!("{}: {state}", platform.name());
    }
    println!("No live accounts connected. No roles or messages are sent.");
}
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
