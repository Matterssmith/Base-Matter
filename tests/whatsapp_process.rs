/*
 * Base-Matter — real Rust control process, crash, replay and restart regression.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};
use time::OffsetDateTime;

fn controller_binary() -> std::path::PathBuf {
    option_env!("CARGO_BIN_EXE_whatsapp-matter-control")
        .map(Into::into)
        .or_else(|| std::env::var_os("CARGO_BIN_EXE_whatsapp-matter-control").map(Into::into))
        .expect("Cargo did not supply the WhatsApp control binary for integration tests")
}

const OWNER_JID: &str = "5215512345678@s.whatsapp.net";
const BRIDGE_KEY: [u8; 32] = [7; 32];

struct Running(Child);
impl Drop for Running {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn command(vars: &HashMap<String, String>, arg: &str) -> Command {
    let mut command = Command::new(controller_binary());
    command
        .arg(arg)
        .envs(vars)
        .env_remove("MATTER_VAULT_IPC_SOCKET")
        .env_remove("MATTER_VAULT_CLIENT_ID")
        .env_remove("MATTER_VAULT_CLIENT_KEY_FILE")
        .env_remove("MATTER_VAULT_TENANT")
        .env_remove("MATTER_VAULT_BOT")
        .env_remove("MATTER_VAULT_SOCKET_ID")
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    command
}

fn start(vars: &HashMap<String, String>, addr: &str) -> Running {
    let mut child = Running(command(vars, "--serve").spawn().unwrap());
    let now = Instant::now();
    while now.elapsed() < Duration::from_secs(7) {
        if let Some(status) = child.0.try_wait().unwrap() {
            panic!("Control process failed before listening: {status}");
        }
        if TcpStream::connect(addr).is_ok() {
            return child;
        }
        thread::sleep(Duration::from_millis(50));
    }
    panic!("Control process did not bind to loopback");
}

fn signed(id: &str, text: &str) -> Vec<u8> {
    let stamp = OffsetDateTime::now_utc().unix_timestamp();
    let mut mac = Hmac::<Sha256>::new_from_slice(&BRIDGE_KEY).unwrap();
    mac.update(b"Base-Matter:WhatsApp-Control:v1\0");
    for part in [
        "1",
        "socket-test",
        "owner-test",
        OWNER_JID,
        OWNER_JID,
        id,
        text,
    ] {
        let data = part.as_bytes();
        mac.update(&(data.len() as u32).to_be_bytes());
        mac.update(data);
    }
    mac.update(&[1]);
    mac.update(&stamp.to_be_bytes());
    serde_json::to_vec(&serde_json::json!({
        "version":1,
        "socket_id":"socket-test",
        "actor_id":"owner-test",
        "owner_jid":OWNER_JID,
        "chat_jid":OWNER_JID,
        "message_id":id,
        "text":text,
        "from_me":true,
        "timestamp":stamp,
        "signature":hex::encode(mac.finalize().into_bytes()),
    }))
    .unwrap()
}

fn deliver(addr: &str, frame: &[u8]) -> serde_json::Value {
    let mut stream = TcpStream::connect(addr).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    stream.write_all(frame).unwrap();
    stream.shutdown(Shutdown::Write).unwrap();
    let mut response = Vec::new();
    stream.read_to_end(&mut response).unwrap();
    serde_json::from_slice(&response).unwrap()
}

#[test]
fn real_controller_process_preserves_replay_and_recovers_after_abrupt_stop() {
    let mut salt = [0_u8; 16];
    getrandom::fill(&mut salt).unwrap();
    let base = std::env::temp_dir().join(format!(
        "base-matter-wa-process-{}-{}",
        std::process::id(),
        hex::encode(salt)
    ));
    fs::create_dir(&base).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&base, fs::Permissions::from_mode(0o700)).unwrap();
    }
    let port = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = port.local_addr().unwrap().to_string();
    drop(port);
    let activation = base.join("activation.db");
    let replay = base.join("replay.db");
    let mut vars = HashMap::new();
    for (k, v) in [
        ("MATTER_WA_SOCKET_ID", "socket-test".to_owned()),
        ("MATTER_WA_OWNER_ID", "owner-test".to_owned()),
        ("MATTER_WA_OWNER_JID", OWNER_JID.to_owned()),
        ("MATTER_WA_BRIDGE_ADDR", addr.clone()),
        ("MATTER_WA_BRIDGE_KEY_HEX", hex::encode(BRIDGE_KEY)),
        ("MATTER_WA_REPLAY_KEY_HEX", "09".repeat(32)),
        (
            "MATTER_WA_ACTIVATION_DB",
            activation.to_string_lossy().into_owned(),
        ),
        ("MATTER_WA_REPLAY_DB", replay.to_string_lossy().into_owned()),
    ] {
        vars.insert(k.to_owned(), v);
    }

    assert!(command(&vars, "--check").status().unwrap().success());
    assert!(command(&vars, "--init").status().unwrap().success());
    let server = start(&vars, &addr);
    assert!(!command(&vars, "--check").status().unwrap().success());
    // While OFF, even valid owner requests to Vault are silently ignored.
    // No Vault credentials are passed to this synthetic test.
    let vault = deliver(&addr, &signed("vault-status", "=/>vault"));
    assert_eq!(vault["outcome"], "ignored");
    assert!(vault.get("message").is_none());
    let role_grant = deliver(
        &addr,
        &signed("global-role-owner", "=/>set role owner --global 5551234567"),
    );
    assert_eq!(role_grant["outcome"], "ignored");
    assert!(role_grant.get("message").is_none());
    let role_grant_help = deliver(
        &addr,
        &signed("global-role-help", "=/>set role owner --global"),
    );
    assert_eq!(role_grant_help["outcome"], "ignored");
    assert!(role_grant_help.get("message").is_none());
    let vault_test = deliver(&addr, &signed("vault-test", "=/>vault test"));
    assert_eq!(vault_test["outcome"], "ignored");
    assert!(vault_test.get("message").is_none());
    assert_eq!(
        deliver(&addr, &signed("off-on", "=/>bot on"))["outcome"],
        "ignored"
    );
    assert_eq!(
        deliver(&addr, &signed("off-status", "=/>status"))["outcome"],
        "applied"
    );
    assert!(!command(&vars, "--set-chat-policy")
        .args(["self", "on"])
        .status()
        .unwrap()
        .success());
    drop(server);
    assert!(command(&vars, "--set-chat-policy")
        .args(["self", "on"])
        .status()
        .unwrap()
        .success());
    let server = start(&vars, &addr);
    assert_eq!(
        deliver(&addr, &signed("off-on", "=/>bot on"))["outcome"],
        "duplicate"
    );
    // When ON, the same Vault operations still require independent credentials.
    for (id, text) in [
        ("active-vault", "=/>vault"),
        ("active-vault-test", "=/>vault test"),
        ("active-role", "=/>set role owner --global 5551234567"),
        ("active-role-help", "=/>set role owner --global"),
    ] {
        let denied = deliver(&addr, &signed(id, text));
        assert_eq!(denied["outcome"], "denied");
        assert!(denied.get("message").is_none());
    }
    assert_eq!(
        deliver(&addr, &signed("first", "=/>bot on"))["outcome"],
        "applied"
    );
    assert_eq!(
        deliver(&addr, &signed("first", "=/>bot on"))["outcome"],
        "duplicate"
    );
    assert_eq!(
        deliver(&addr, &signed("second", "=/>bot off"))["outcome"],
        "applied"
    );
    drop(server);

    // The persistent v2 lock is available after a killed process.
    let lock = activation.with_extension("controller.lock");
    assert!(lock.exists());
    assert!(command(&vars, "--check").status().unwrap().success());
    let server = start(&vars, &addr);
    // Replays from before the restart remain rejected by persisted SQLite.
    assert_eq!(
        deliver(&addr, &signed("first", "=/>bot on"))["outcome"],
        "duplicate"
    );
    assert_eq!(
        deliver(&addr, &signed("third", "=/>bot on"))["outcome"],
        "ignored"
    );
    assert_eq!(
        deliver(&addr, &signed("fourth", "=/>bot off"))["outcome"],
        "ignored"
    );
    assert_eq!(
        deliver(&addr, &signed("restart-status", "=/>status"))["outcome"],
        "applied"
    );
    assert!(!command(&vars, "--check").status().unwrap().success());
    drop(server);
    assert!(activation.exists());
    assert!(replay.exists());
    assert!(command(&vars, "--check").status().unwrap().success());
    fs::remove_dir_all(&base).unwrap();
}
