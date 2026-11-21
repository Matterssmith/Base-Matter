/*
 * Base-Matter — offline bot factory descriptor CLI.
 * Never installs providers, starts sessions or grants runtime permissions.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use matter_base::bot_factory::BotProjectRequest;
use matter_base::bot_setup::BotSetupRequest;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut request = BotProjectRequest::default();
    let mut legacy = false;
    let mut args = env::args().skip(1);
    while let Some(flag) = args.next() {
        if flag == "--help" {
            println!("matter-bot-init [--language msel|rust|python|cpp|c|go|typescript|javascript|lua|kotlin|swift]");
            println!("  [--platform social-matter|whatsapp|telegram|discord]... [--provider ID]");
            println!("  [--provider-for PLATFORM=LIBRARY_ID]... [--system host|linux|windows|macos|android|ios|web|embedded]");
            println!("  [--legacy-selector]");
            println!("Defaults: language=msel (Matter-MSEL), platform=social-matter, provider=default, system=host.");
            println!(
                "Provider aliases and custom libraries require approval by an authenticated host."
            );
            println!("Output is a descriptor only. No session, Vault access, provider installation or privilege is granted.");
            return Ok(());
        }
        if flag == "--legacy-selector" {
            if legacy {
                return Err("Duplicate --legacy-selector option".into());
            }
            legacy = true;
            continue;
        }
        let value = args.next().ok_or("Missing argument for bot setup option")?;
        match flag.as_str() {
            "--language" => {
                if request.language.replace(value).is_some() {
                    return Err("Duplicate --language option".into());
                }
            }
            "--system" => {
                if request.system.replace(value).is_some() {
                    return Err("Duplicate --system option".into());
                }
            }
            "--provider" => {
                if request.provider.replace(value).is_some() {
                    return Err("Duplicate --provider option".into());
                }
            }
            "--platform" => {
                request.platforms.get_or_insert_with(Vec::new).push(value);
            }
            "--provider-for" => {
                let (platform, adapter) = value
                    .split_once('=')
                    .ok_or("Expected --provider-for PLATFORM=LIBRARY_ID")?;
                if platform.is_empty() || adapter.is_empty() {
                    return Err("Expected non-empty platform and library ID".into());
                }
                if request
                    .adapters
                    .insert(platform.to_owned(), adapter.to_owned())
                    .is_some()
                {
                    return Err("Duplicate --provider-for option".into());
                }
            }
            _ => return Err("Unknown bot setup option".into()),
        }
    }

    if legacy {
        if request.platforms.is_some() || !request.adapters.is_empty() {
            return Err("Legacy selector does not support --platform or --provider-for".into());
        }
        let legacy_request = BotSetupRequest {
            language: request.language,
            provider: request.provider,
            system: request.system,
        };
        println!(
            "{}",
            serde_json::to_string_pretty(&legacy_request.select()?)?
        );
        return Ok(());
    }
    println!("{}", serde_json::to_string_pretty(&request.select()?)?);
    Ok(())
}

// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
