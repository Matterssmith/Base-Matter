/*
 * WhatsApp-Matter — offline provider signing, configuration and lease tests.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use std::{collections::HashMap, fs};
use whatsapp_matter_rust_provider::{sign_frame, Config, Event, SessionLease, SignedFrame};

const OWNER: &str = "5215512345678@s.whatsapp.net";

fn fixture() -> (HashMap<String, String>, std::path::PathBuf) {
    let root = std::env::temp_dir().join(format!(
        "whatsapp-matter-provider-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir(&root).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).unwrap();
    }
    let mut values = HashMap::new();
    for (name, value) in [
        ("MATTER_WA_PHONE", "5215512345678"),
        ("MATTER_WA_OWNER_JID", OWNER),
        ("MATTER_WA_SOCKET_ID", "socket-1"),
        ("MATTER_WA_OWNER_ID", "owner"),
        ("MATTER_WA_BRIDGE_ADDR", "127.0.0.1:39819"),
    ] {
        values.insert(name.to_owned(), value.to_owned());
    }
    values.insert(
        "MATTER_WA_SESSION_DB".into(),
        root.join("session.db").to_string_lossy().into_owned(),
    );
    values.insert("MATTER_WA_BRIDGE_KEY_HEX".into(), "07".repeat(32));
    (values, root)
}

fn event<'a>(text: &'a str) -> Event<'a> {
    Event {
        chat_jid: OWNER,
        sender_jid: OWNER,
        message_id: "msg-1",
        text,
        from_me: true,
        is_group: false,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
    }
}

#[test]
fn creates_valid_signed_owner_control_only_for_private_self_chat() {
    let (vars, root) = fixture();
    let c = Config::from_reader(|name| vars.get(name).cloned()).unwrap();
    let frame = c.signed_control(&event("=/>status")).unwrap().unwrap();
    assert_eq!(frame.owner_jid, OWNER);
    assert_eq!(frame.text, "=/>status");
    assert_eq!(frame.signature.len(), 64);
    let mut bad = event("=/>on");
    bad.from_me = false;
    assert!(c.signed_control(&bad).unwrap().is_none());
    bad.from_me = true;
    bad.is_group = true;
    assert!(c.signed_control(&bad).unwrap().is_none());
    bad.is_group = false;
    bad.chat_jid = "other@s.whatsapp.net";
    assert!(c.signed_control(&bad).unwrap().is_none());
    assert!(c.signed_control(&event("#ping")).unwrap().is_none());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn signature_matches_rust_base_control_protocol_fixture() {
    let value = SignedFrame {
        version: 1,
        socket_id: "socket-1".into(),
        actor_id: "owner".into(),
        owner_jid: OWNER.into(),
        chat_jid: OWNER.into(),
        message_id: "msg-1".into(),
        text: "=/>status".into(),
        from_me: true,
        timestamp: 1_794_873_600,
        signature: String::new(),
    };
    let mut key = [0; 32];
    for (index, byte) in key.iter_mut().enumerate() {
        *byte = (index + 1) as u8;
    }
    assert_eq!(
        sign_frame(&value, &key).unwrap(),
        "747dc915a2734e90d1f2042cc42df7e360f848efcacda76baa8f27925b3124c7"
    );
}

#[test]
fn rejects_missing_reused_or_public_bridge_credentials() {
    let (mut vars, root) = fixture();
    for (name, bad) in [
        ("MATTER_WA_PHONE", "+5215512345678"),
        ("MATTER_WA_OWNER_JID", "123@g.us"),
        ("MATTER_WA_BRIDGE_ADDR", "0.0.0.0:39819"),
        ("MATTER_WA_SOCKET_ID", "socket/1"),
    ] {
        let old = vars.insert(name.to_owned(), bad.to_owned()).unwrap();
        assert!(Config::from_reader(|name| vars.get(name).cloned()).is_err());
        vars.insert(name.to_owned(), old);
    }
    let old = vars.insert("MATTER_WA_BRIDGE_KEY_HEX".into(), "00".repeat(32));
    assert!(Config::from_reader(|name| vars.get(name).cloned()).is_err());
    vars.insert("MATTER_WA_BRIDGE_KEY_HEX".into(), old.unwrap());
    vars.remove("MATTER_WA_SESSION_DB");
    assert!(Config::from_reader(|name| vars.get(name).cloned()).is_err());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn exclusive_session_lock_never_removes_sqlite_file() {
    let (vars, root) = fixture();
    let c = Config::from_reader(|name| vars.get(name).cloned()).unwrap();
    fs::write(&c.session_db, b"existing session").unwrap();
    let lease = SessionLease::acquire(&c.session_db).unwrap();
    assert!(SessionLease::acquire(&c.session_db).is_err());
    drop(lease);
    assert_eq!(fs::read(&c.session_db).unwrap(), b"existing session");
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn old_or_malformed_control_events_are_rejected() {
    let (vars, root) = fixture();
    let c = Config::from_reader(|name| vars.get(name).cloned()).unwrap();
    let mut old = event("=/>off");
    old.timestamp -= 500;
    assert!(c.signed_control(&old).is_err());
    let mut invalid = event("=/>on");
    invalid.message_id = "";
    assert!(c.signed_control(&invalid).is_err());
    fs::remove_dir_all(root).unwrap();
}


#[test]
fn preflight_validates_configuration_without_writing_a_session() {
    use std::process::Command;
    let (config, dir) = fixture();
    let result = Command::new(env!("CARGO_BIN_EXE_whatsapp-matter-rust-provider"))
        .arg("--check")
        .envs(&config)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "Rust provider preflight failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(!dir.join("session.db").exists());
    assert!(!dir.join("session.owner.lock").exists());
    let mut invalid = config;
    invalid.insert("MATTER_WA_PHONE".into(), "+5215512345678".into());
    let denied = Command::new(env!("CARGO_BIN_EXE_whatsapp-matter-rust-provider"))
        .arg("--check")
        .envs(&invalid)
        .output()
        .unwrap();
    assert!(!denied.status.success());
    fs::remove_dir_all(dir).unwrap();
}
