/*
 * Base-Matter — operator Vault IPC client CLI.
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{anyhow, bail, Result};
use matter_base::vault_ipc::VaultClient;
use std::{env, fs, io::Write, path::Path};
use zeroize::Zeroizing;
fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 7 {
        bail!("Usage: matter-vault-client ping|read|write SOCKET_PATH CLIENT_ID KEY_FILE TENANT BOT SOCKET [RECORD] [FILE] [EXPECTED_REVISION|new]");
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
fn secure_write(path: &Path, bytes: &[u8]) -> Result<()> {
    if !path.is_absolute() {
        bail!("output_must_be_absolute");
    }
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    #[cfg(not(unix))]
    bail!("secure_output_unavailable");
    let mut file = options.open(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    Ok(())
}
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
