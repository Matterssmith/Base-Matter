/*
 * WhatsApp-Matter — encrypted community recovery never leaks member JIDs.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use std::fs;
use whatsapp_matter_rust_provider::recovery::{
    read_encrypted, save_encrypted, RecoveryMember, RecoverySnapshot,
};

fn fixture() -> (std::path::PathBuf, RecoverySnapshot) {
    let mut nonce = [0_u8; 12];
    getrandom::fill(&mut nonce).unwrap();
    let path = std::env::temp_dir().join(format!(
        "base-matter-recovery-test-{}-{}",
        std::process::id(),
        hex::encode(nonce)
    ));
    fs::create_dir(&path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o700)).unwrap();
    }
    let snapshot = RecoverySnapshot {
        version: 1,
        socket_id: "wa-test".into(),
        owner_jid: "5215512345678@s.whatsapp.net".into(),
        community_jid: "120363000123456@g.us".into(),
        community_name: "Mi comunidad".into(),
        captured_at: 1_793_000_000,
        members: vec![RecoveryMember {
            jid: "5215598765432@s.whatsapp.net".into(),
            phone_jid: Some("5215598765432@s.whatsapp.net".into()),
            lid_jid: Some("99887766@lid".into()),
        }],
    };
    (path, snapshot)
}

#[test]
fn roundtrip_is_ciphertext_only_and_bound_to_owner() {
    let (dir, snapshot) = fixture();
    let key = [0x47_u8; 32];
    let db = dir.join("session.db");
    let path = save_encrypted(&db, &key, &snapshot).unwrap();
    let saved = fs::read(&path).unwrap();
    assert!(!saved.windows(b"5215598765432".len()).any(|w| w == b"5215598765432"));
    assert!(!saved.windows(b"Mi comunidad".len()).any(|w| w == b"Mi comunidad"));
    let recovered = read_encrypted(
        &path,
        &key,
        &snapshot.socket_id,
        &snapshot.owner_jid,
    )
    .unwrap();
    assert_eq!(recovered, snapshot);
    assert!(read_encrypted(&path, &[0x13; 32], &snapshot.socket_id, &snapshot.owner_jid).is_err());
    assert!(read_encrypted(&path, &key, "wrong-socket", &snapshot.owner_jid).is_err());
    assert!(read_encrypted(&path, &key, &snapshot.socket_id, "wrong-owner").is_err());
    assert!(save_encrypted(&db, &key, &snapshot).is_ok());
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = fs::metadata(&path).unwrap().permissions().mode();
        assert_eq!(mode & 0o077, 0);
    }
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn rejects_untrusted_member_identity_and_file_edits() {
    let (dir, mut snapshot) = fixture();
    let key = [0x22_u8; 32];
    let db = dir.join("session.db");
    snapshot.members[0].jid = "../other".to_owned();
    assert!(save_encrypted(&db, &key, &snapshot).is_err());
    snapshot.members[0].jid = "5215598765432@s.whatsapp.net".to_owned();
    let path = save_encrypted(&db, &key, &snapshot).unwrap();
    let mut data = fs::read(&path).unwrap();
    let n = data.len() - 5;
    data[n] ^= 1;
    fs::write(&path, data).unwrap();
    assert!(read_encrypted(&path, &key, &snapshot.socket_id, &snapshot.owner_jid).is_err());
    fs::remove_dir_all(dir).unwrap();
}
