/*
 * Base-Matter — operator Vault IPC client CLI.
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{anyhow, bail, Result};
use matter_base::vault_ipc::VaultClient;
#[cfg(unix)]
use std::io::{Read, Write};
use std::{env, fs, path::Path};
use zeroize::Zeroizing;
fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 7 {
        bail!("Usage: matter-vault-client ping|self-test|global-style|owner-check|owner-check-platform|read|write SOCKET_PATH CLIENT_ID KEY_FILE TENANT BOT SOCKET [RECORD] [FILE] [EXPECTED_REVISION|new]");
    }
    let client = VaultClient {
        socket_path: args[1].clone().into(),
        client_id: args[2].clone(),
        client_key_file: args[3].clone().into(),
        tenant: args[4].clone(),
        bot: args[5].clone(),
        socket: args[6].clone(),
    };
    match args[0].as_str() {
        "ping" if args.len() == 7 => {
            client.invoke("ping", "health", None, None)?;
            println!("vault_authenticated");
        }
        "global-style" if args.len() == 7 => {
            // Operator diagnostics must contact the running service, not the cache.
            client.invoke("global_style_get", "global-menu-style-v1", None, None)?;
            println!("vault_global_style_authenticated");
        }
        "self-test" if args.len() == 7 => {
            let revision = client.verify_roundtrip()?;
            println!("vault_roundtrip_verified revision={revision}");
        }
        #[cfg(unix)]
        "owner-check" if args.len() == 7 => {
            // Operator diagnostics and CI: never provide a phone or LID
            // as an argv or environment value. The result is boolean only.
            let mut input = Zeroizing::new(String::new());
            std::io::stdin().take(128).read_to_string(&mut input)?;
            if input.len() >= 128 || input.lines().count() != 1 {
                bail!("global_owner_identity_invalid");
            }
            client.require_global_owner(input.trim())?;
            println!("global_owner_authorized");
        }
        #[cfg(unix)]
        "owner-check-platform" if args.len() == 8 => {
            // The platform is a non-secret selection, while the canonical
            // subject arrives only through stdin and is never echoed.
            let mut input = Zeroizing::new(String::new());
            std::io::stdin().take(128).read_to_string(&mut input)?;
            if input.len() >= 128 || input.lines().count() != 1 {
                bail!("global_owner_identity_invalid");
            }
            client.require_global_owner_for_platform(&args[7], input.trim())?;
            println!("global_owner_authorized");
        }
        "read" if args.len() == 9 => {
            let result = client.invoke("get", &args[7], None, None)?;
            if let Some(value) = result.value {
                secure_write(Path::new(&args[8]), &value)?;
                println!("record_read revision={}", result.revision.unwrap_or(0));
            } else {
                println!("record_missing");
            }
        }
        "write" if args.len() == 10 => {
            let value = Zeroizing::new(fs::read(&args[8])?);
            if value.len() > 1024 * 1024 {
                bail!("vault_input_too_large");
            }
            let revision = if args[9] == "new" {
                None
            } else {
                Some(
                    args[9]
                        .parse::<u64>()
                        .map_err(|_| anyhow!("invalid_revision"))?,
                )
            };
            let result = client.invoke("put", &args[7], revision, Some(&value))?;
            println!("record_written revision={}", result.revision.unwrap_or(0));
        }
        _ => bail!("invalid_vault_client_command"),
    }
    Ok(())
}
#[cfg(unix)]
fn secure_write(path: &Path, bytes: &[u8]) -> Result<()> {
    if !path.is_absolute() {
        bail!("output_must_be_absolute");
    }
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    use std::os::unix::fs::OpenOptionsExt;
    options.mode(0o600);
    let mut file = options.open(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    Ok(())
}
#[cfg(not(unix))]
fn secure_write(_path: &Path, _bytes: &[u8]) -> Result<()> {
    bail!("secure_output_unavailable")
}
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
