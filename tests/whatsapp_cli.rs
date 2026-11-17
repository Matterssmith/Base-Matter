/*
 * Base-Matter — offline controller preflight tests.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use std::{fs, process::Command};

#[test]
fn preflight_verifies_config_without_creating_databases_or_locks() {
    let mut nonce = [0_u8; 16];
    getrandom::fill(&mut nonce).unwrap();
    let dir = std::env::temp_dir().join(format!(
        "base-matter-wa-{}-{}", std::process::id(), hex::encode(nonce)
    ));
    fs::create_dir(&dir).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o700)).unwrap();
    }
    let activation = dir.join("activation.db");
    let replay = dir.join("replay.db");
    let vars = [
        ("MATTER_WA_SOCKET_ID", "socket-test".to_owned()),
        ("MATTER_WA_OWNER_ID", "owner-test".to_owned()),
        ("MATTER_WA_OWNER_JID", "5215512345678@s.whatsapp.net".to_owned()),
        ("MATTER_WA_BRIDGE_ADDR", "127.0.0.1:39819".to_owned()),
        ("MATTER_WA_BRIDGE_KEY_HEX", "07".repeat(32)),
        ("MATTER_WA_REPLAY_KEY_HEX", "09".repeat(32)),
        ("MATTER_WA_ACTIVATION_DB", activation.to_string_lossy().into_owned()),
        ("MATTER_WA_REPLAY_DB", replay.to_string_lossy().into_owned()),
    ];
    let success = Command::new(env!("CARGO_BIN_EXE_whatsapp-matter-control"))
        .arg("--check")
        .envs(vars.iter().map(|(k, v)| (k, v)))
        .output()
        .unwrap();
    assert!(
        success.status.success(),
        "Control preflight failed: {}",
        String::from_utf8_lossy(&success.stderr)
    );
    assert!(!activation.exists());
    assert!(!replay.exists());
    let denied = Command::new(env!("CARGO_BIN_EXE_whatsapp-matter-control"))
        .arg("--check")
        .envs(vars.iter().map(|(k, v)| (k, v)))
        .env("MATTER_WA_BRIDGE_ADDR", "0.0.0.0:39819")
        .output()
        .unwrap();
    assert!(!denied.status.success());
    fs::remove_dir_all(dir).unwrap();
}
