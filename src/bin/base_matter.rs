/*
 * Base-Matter — single-terminal development launcher.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use std::{env, error::Error, path::PathBuf, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // The default Cargo run must prove core/provider correctness before
    // touching the existing WhatsApp alpha session. The legacy one-terminal
    // pairing launcher remains explicit and is never called implicitly.
    let script = root.join("scripts/matter-verified-run.sh");
    if !script.is_file() {
        return Err("Verified alpha launcher script is missing".into());
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
