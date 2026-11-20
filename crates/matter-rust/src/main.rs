/*
 * Matter-Rust — offline discovery and HTTP capability explorer.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use matter_rust_core::{MatterRust, Platform, ENGINE_NAME};
use std::process::ExitCode;

fn print_status() {
    let engine = MatterRust::new();
    println!(
        "{ENGINE_NAME} | registered transport sessions: {}",
        engine.registered_sessions()
    );
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

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [] => {
            print_status();
            ExitCode::SUCCESS
        }
        [command] if command == "--status" => {
            print_status();
            ExitCode::SUCCESS
        }
        [command] if command == "--serve-preview" => {
            match matter_rust_core::web::serve_preview(
                matter_rust_core::web::DEFAULT_PREVIEW_PORT,
            ) {
                Ok(()) => ExitCode::SUCCESS,
                Err(error) => {
                    eprintln!("Matter-Rust: cannot serve local preview: {error}");
                    ExitCode::FAILURE
                }
            }
        }
        [command, port_flag, port] if command == "--serve-preview" && port_flag == "--port" => {
            let Ok(port) = port.parse::<u16>() else {
                eprintln!("Matter-Rust: invalid numeric port");
                return ExitCode::FAILURE;
            };
            match matter_rust_core::web::serve_preview(port) {
                Ok(()) => ExitCode::SUCCESS,
                Err(error) => {
                    eprintln!("Matter-Rust: cannot serve local preview: {error}");
                    ExitCode::FAILURE
                }
            }
        }
        [command] if command == "--help" => {
            println!("Matter-Rust: --status | --serve-preview [--port 39841] | --help");
            print_status();
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("Matter-Rust: unsupported CLI arguments; use --help");
            ExitCode::FAILURE
        }
    }
}
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
