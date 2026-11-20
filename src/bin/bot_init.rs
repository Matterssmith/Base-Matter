/*
 * Base-Matter — public, offline bot setup descriptor.
 * Generates no execution permission, credentials or private source code.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use matter_base::bot_setup::BotSetupRequest;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut request = BotSetupRequest::default();
    let mut arguments = env::args().skip(1);
    while let Some(flag) = arguments.next() {
        if flag == "--help" {
            println!(
                "matter-bot-init [--provider ID] [--language rust|python|cpp|c|go|typescript|javascript] [--system host|linux|windows|macos|android]"
            );
            println!("Generates a selector only. No private provider or Vault grant is installed.");
            return Ok(());
        }
        let value = arguments
            .next()
            .ok_or("Missing argument for bot setup option")?;
        let target = match flag.as_str() {
            "--provider" => &mut request.provider,
            "--language" => &mut request.language,
            "--system" => &mut request.system,
            _ => return Err("Unknown bot setup option".into()),
        };
        if target.is_some() {
            return Err("Duplicate bot setup option".into());
        }
        *target = Some(value);
    }
    let selection = request.select()?;
    println!("{}", serde_json::to_string_pretty(&selection)?);
    Ok(())
}

// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
