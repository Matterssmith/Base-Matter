/*
 * Base-Matter — single-terminal development launcher.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use std::{env, error::Error, path::PathBuf, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let script = root.join("scripts/whatsapp-one-terminal.sh");
    if !script.is_file() {
        return Err("Single-terminal launcher script is missing".into());
    }
    let status = Command::new("bash")
        .arg(script)
        .args(env::args_os().skip(1))
        .current_dir(root)
        .status()?;
    if !status.success() {
        return Err(format!("WhatsApp launcher failed with status: {status}").into());
    }
    Ok(())
}
