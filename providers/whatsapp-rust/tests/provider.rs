/*
 * WhatsApp-Matter — offline configuration and ownership tests.
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use std::{collections::HashMap, fs};
use whatsapp_matter_rust_provider::{Config, SessionLease};
use zeroize::Zeroizing;
fn fixture() -> (HashMap<String, String>, std::path::PathBuf) {
    let mut n = [0u8; 16];
    // Tests do not need provider network or an actual phone.
    n.copy_from_slice(&std::process::id().to_be_bytes().repeat(4)[..16]);
    let root = std::env::temp_dir().join(format!(
        "whatsapp-matter-config-{}-{}",
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
    let mut vars = HashMap::new();
    vars.insert("MATTER_WA_PHONE".into(), "5215512345678".into());
    vars.insert(
        "MATTER_WA_OWNER_JID".into(),
        "5215512345678@s.whatsapp.net".into(),
    );
    vars.insert("MATTER_WA_SOCKET_ID".into(), "socket-a".into());
    vars.insert("MATTER_WA_OWNER_ID".into(), "alice".into());
    for (env, name) in [
        ("MATTER_WA_SESSION_DB", "session.db"),
        ("MATTER_WA_ACTIVATION_DB", "activation.db"),
        ("MATTER_WA_REPLAY_DB", "replay.db"),
    ] {
        vars.insert(env.into(), root.join(name).to_string_lossy().into_owned());
    }
    vars.insert("MATTER_WA_REPLAY_KEY_HEX".into(), "11".repeat(32));
    (vars, root)
}
#[test]
fn validates_code_only_config_and_private_store_paths() {
    let (vars, root) = fixture();
    let config = Config::from_reader(|key| vars.get(key).cloned()).unwrap();
    assert_eq!(config.socket_id, "socket-a");
    assert_eq!(config.owner_jid, "5215512345678@s.whatsapp.net");
    assert_eq!(config.replay_key[..], [17u8; 32]);
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn fails_closed_for_incomplete_or_untrusted_configuration() {
    let (mut vars, root) = fixture();
    for (key, invalid) in [
        ("MATTER_WA_PHONE", "+5215512345678"),
        ("MATTER_WA_OWNER_JID", "123@g.us"),
        ("MATTER_WA_SOCKET_ID", "other/socket"),
        ("MATTER_WA_REPLAY_KEY_HEX", &"00".repeat(32)),
    ] {
        let old = vars.insert(key.into(), invalid.into()).unwrap();
        assert!(Config::from_reader(|name| vars.get(name).cloned()).is_err());
        vars.insert(key.into(), old);
    }
    vars.remove("MATTER_WA_SESSION_DB");
    assert!(Config::from_reader(|key| vars.get(key).cloned()).is_err());
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn session_lease_does_not_allow_competing_socket_or_delete_database() {
    let (vars, root) = fixture();
    let config = Config::from_reader(|key| vars.get(key).cloned()).unwrap();
    fs::write(&config.session_db, b"original session").unwrap();
    let lease = SessionLease::acquire(&config.session_db).unwrap();
    assert!(SessionLease::acquire(&config.session_db).is_err());
    drop(lease);
    assert_eq!(fs::read(&config.session_db).unwrap(), b"original session");
    assert!(SessionLease::acquire(&config.session_db).is_ok());
    fs::remove_dir_all(root).unwrap();
}
#[test]
fn all_key_material_is_zeroized_on_drop() {
    let (vars, root) = fixture();
    let config = Config::from_reader(|key| vars.get(key).cloned()).unwrap();
    let _: Zeroizing<[u8; 32]> = config.replay_key;
    fs::remove_dir_all(root).unwrap();
}
