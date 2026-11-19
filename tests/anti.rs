/*
 * Base-Matter — durable anti policies and grouped command grammar.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use matter_base::anti::{canonical_rule, AntiAction, AntiStore, RULES};
use std::sync::atomic::{AtomicUsize, Ordering};

const SOCKET: &str = "socket-alpha";
const GROUP: &str = "120363111@g.us";
const SECOND_GROUP: &str = "120363222@g.us";

fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

#[test]
fn all_anti_rules_are_disabled_until_explicitly_configured() {
    let store = AntiStore::open(":memory:").unwrap();
    for rule in RULES {
        assert!(!store.get_global(SOCKET, rule).unwrap().enabled);
        assert!(!store.get(SOCKET, GROUP, rule).unwrap().enabled);
        assert_eq!(
            store.get(SOCKET, GROUP, rule).unwrap().action,
            AntiAction::Warn
        );
    }
    let inventory = store.command(SOCKET, Some(GROUP), &[]).unwrap();
    assert!(inventory.contains("Antis del grupo"));
    assert!(inventory.contains("status: off [warn]"));
    assert!(inventory.contains("oneview: off [warn]"));
    assert!(inventory.contains("sticker: off [warn]"));
    assert!(store
        .command(SOCKET, None, &[])
        .unwrap()
        .contains("socket global"));
}

#[test]
fn anti_status_means_whatsapp_group_status_not_list_all_antis() {
    let store = AntiStore::open(":memory:").unwrap();
    assert_eq!(canonical_rule("status").unwrap(), "antistatus");
    assert_eq!(canonical_rule("groupstatus").unwrap(), "antistatus");
    let response = store
        .command(SOCKET, Some(GROUP), &args(&["status"]))
        .unwrap();
    assert_eq!(response, "status: off [warn]");
    assert!(!response.contains("link:"));
    assert!(store
        .command(SOCKET, Some(GROUP), &args(&["status", "on"]))
        .unwrap()
        .contains("status: on"));
    assert_eq!(
        store
            .command(SOCKET, Some(GROUP), &args(&["status"]))
            .unwrap(),
        "status: on [warn]"
    );
    assert!(!store.get(SOCKET, GROUP, "oneview").unwrap().enabled);
    let global = store.command(SOCKET, None, &args(&["status"])).unwrap();
    assert_eq!(global, "status: off [warn]");
}

#[test]
fn global_defaults_are_independent_of_exact_group_overrides() {
    let store = AntiStore::open(":memory:").unwrap();
    store.command(SOCKET, None, &args(&["link", "on"])).unwrap();
    assert!(store.get(SOCKET, GROUP, "link").unwrap().enabled);
    assert!(store.get(SOCKET, SECOND_GROUP, "link").unwrap().enabled);
    store
        .command(SOCKET, Some(GROUP), &args(&["link", "off"]))
        .unwrap();
    assert!(!store.get(SOCKET, GROUP, "link").unwrap().enabled);
    assert!(store.get(SOCKET, SECOND_GROUP, "link").unwrap().enabled);
    store
        .command(SOCKET, Some(GROUP), &args(&["link", "inherit"]))
        .unwrap();
    assert!(store.get(SOCKET, GROUP, "link").unwrap().enabled);
    assert!(store
        .command(SOCKET, None, &args(&["link", "inherit"]))
        .is_err());
}

#[test]
fn actions_are_persisted_independently_from_activation_and_never_moderate() {
    static SEQUENCE: AtomicUsize = AtomicUsize::new(0);
    let path = std::env::temp_dir().join(format!(
        "base-matter-anti-test-{}-{}.sqlite",
        std::process::id(),
        SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    {
        let store = AntiStore::open(&path).unwrap();
        assert!(
            store
                .set_action(SOCKET, Some(GROUP), "movie", "delete")
                .unwrap()
                .contains("sin moderación")
                || store
                    .command(SOCKET, Some(GROUP), &args(&["movie"]))
                    .unwrap()
                    .contains("[delete]")
        );
        assert!(!store.get(SOCKET, GROUP, "movie").unwrap().enabled);
        store
            .command(SOCKET, Some(GROUP), &args(&["movie", "on"]))
            .unwrap();
        assert!(store
            .inspect_links(SOCKET, GROUP, "https://youtube.com")
            .unwrap()
            .is_empty());
    }
    {
        let store = AntiStore::open(&path).unwrap();
        assert!(store.get(SOCKET, GROUP, "movie").unwrap().enabled);
        assert_eq!(
            store.get(SOCKET, GROUP, "movie").unwrap().action,
            AntiAction::Delete
        );
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn link_detection_is_passive_and_uses_exact_domain_boundaries() {
    let store = AntiStore::open(":memory:").unwrap();
    store
        .edit_link_domains(
            SOCKET,
            Some(GROUP),
            "add",
            &args(&["chat.whatsapp.com", "t.me"]),
        )
        .unwrap();
    store
        .command(SOCKET, Some(GROUP), &args(&["link", "on"]))
        .unwrap();
    let accepted = store
        .inspect_links(SOCKET, GROUP, "https://chat.whatsapp.com/invite")
        .unwrap();
    assert_eq!(accepted.len(), 1);
    assert_eq!(accepted[0].rule, "antilink");
    assert!(store
        .inspect_links(SOCKET, GROUP, "https://notwhatsapp.com/invite")
        .unwrap()
        .is_empty());
    assert!(store
        .inspect_links(SOCKET, SECOND_GROUP, "https://chat.whatsapp.com/invite")
        .unwrap()
        .is_empty());
}

#[test]
fn untrusted_groups_and_unknown_filters_are_rejected() {
    let store = AntiStore::open(":memory:").unwrap();
    assert!(store.command(SOCKET, Some("../fake"), &[]).is_err());
    assert!(store
        .command(SOCKET, Some(GROUP), &args(&["unknown"]))
        .is_err());
    assert!(store
        .set_action(SOCKET, Some(GROUP), "link", "execute-shell")
        .is_err());
    assert!(store
        .command(SOCKET, Some(GROUP), &args(&["link", "nonsense"]))
        .is_err());
}

#[test]
fn social_network_filters_are_retired_and_domains_are_scoped() {
    let store = AntiStore::open(":memory:").unwrap();
    for retired in [
        "antilinkwa",
        "antilinktelegram",
        "antilinkinstagram",
        "antilinkyoutube",
        "antilinkdiscord",
        "antilinkphishing",
        "instagram",
        "telegram",
        "facebook",
        "twitter",
    ] {
        assert!(canonical_rule(retired).is_err(), "{retired}");
    }
    assert!(store.link_domains(SOCKET, Some(GROUP)).unwrap().is_empty());
    store
        .edit_link_domains(SOCKET, None, "add", &args(&["https://t.me/example"]))
        .unwrap();
    store
        .edit_link_domains(
            SOCKET,
            Some(GROUP),
            "add",
            &args(&["Instagram.com", "youtube.com"]),
        )
        .unwrap();
    assert_eq!(
        store.link_domains(SOCKET, Some(GROUP)).unwrap(),
        vec!["instagram.com", "t.me", "youtube.com"]
    );
    assert_eq!(
        store.link_domains(SOCKET, Some(SECOND_GROUP)).unwrap(),
        vec!["t.me"]
    );
    assert!(store
        .inspect_links(SOCKET, GROUP, "https://instagram.com/p")
        .unwrap()
        .is_empty()); // Link filter still OFF.
    store
        .command(SOCKET, Some(GROUP), &args(&["link", "on"]))
        .unwrap();
    assert_eq!(
        store
            .inspect_links(SOCKET, GROUP, "https://www.instagram.com/p")
            .unwrap()
            .len(),
        1
    );
    assert!(store
        .inspect_links(SOCKET, GROUP, "https://notinstagram.com/p")
        .unwrap()
        .is_empty());
    assert!(store
        .inspect_links(SOCKET, GROUP, "https://facebook.com/p")
        .unwrap()
        .is_empty());
    assert!(store
        .inspect_links(SOCKET, SECOND_GROUP, "https://t.me/foo")
        .unwrap()
        .is_empty()); // This chat remains OFF.
    store
        .edit_link_domains(SOCKET, Some(GROUP), "del", &args(&["Instagram.com"]))
        .unwrap();
    assert!(!store
        .link_domains(SOCKET, Some(GROUP))
        .unwrap()
        .contains(&"instagram.com".to_owned()));
    assert!(store
        .edit_link_domains(SOCKET, Some(GROUP), "add", &args(&["bad@domain.com"]))
        .is_err());
    assert!(store
        .edit_link_domains(SOCKET, Some(GROUP), "add", &args(&["http://localhost"]))
        .is_err());
    assert!(store
        .link_domains("other-socket", Some(GROUP))
        .unwrap()
        .is_empty());
}
#[test]
fn custom_link_domains_survive_restarts_and_do_not_enable_anti() {
    static SEQUENCE: AtomicUsize = AtomicUsize::new(0);
    let path = std::env::temp_dir().join(format!(
        "base-matter-link-domains-{}-{}.sqlite",
        std::process::id(),
        SEQUENCE.fetch_add(1, Ordering::Relaxed),
    ));
    {
        let store = AntiStore::open(&path).unwrap();
        store
            .edit_link_domains(SOCKET, None, "add", &args(&["youtube.com"]))
            .unwrap();
        store
            .edit_link_domains(SOCKET, Some(GROUP), "add", &args(&["t.me"]))
            .unwrap();
        assert!(!store.get(SOCKET, GROUP, "link").unwrap().enabled);
    }
    {
        let store = AntiStore::open(&path).unwrap();
        assert_eq!(
            store.link_domains(SOCKET, Some(GROUP)).unwrap(),
            vec!["t.me", "youtube.com"]
        );
        assert!(!store.get(SOCKET, GROUP, "link").unwrap().enabled);
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn all_anti_rules_have_shared_edit_metadata_without_enabling_moderation() {
    use matter_base::anti_guard::EditState;
    let store = AntiStore::open(":memory:").unwrap();
    for rule in RULES {
        assert!(!store.get(SOCKET, GROUP, rule).unwrap().enabled);
    }
    let (review, findings) = store
        .inspect_message(
            SOCKET,
            GROUP,
            "＝／＞anti link on https://t․me/abc",
            EditState::EditedReadable,
        )
        .unwrap();
    assert!(review.edited);
    assert!(review.command_camouflage);
    assert!(review.link_camouflage);
    assert!(findings.is_empty()); // OFF by default.
    let (opaque, findings) = store
        .inspect_message(SOCKET, GROUP, "", EditState::EditedEncryptedUnavailable)
        .unwrap();
    assert!(opaque.unreadable);
    assert!(findings.is_empty());
}

#[test]
fn edited_obfuscated_links_are_detected_only_with_active_generic_rule() {
    use matter_base::anti_guard::EditState;
    let store = AntiStore::open(":memory:").unwrap();
    store
        .edit_link_domains(SOCKET, Some(GROUP), "add", &args(&["t.me"]))
        .unwrap();
    store
        .command(SOCKET, Some(GROUP), &args(&["link", "on"]))
        .unwrap();
    let (review, found) = store
        .inspect_message(SOCKET, GROUP, "https://t․me/abc", EditState::EditedReadable)
        .unwrap();
    assert!(review.link_camouflage);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].rule, "antilink");
    assert!(store
        .inspect_message(
            SOCKET,
            GROUP,
            "https://not-t․me/abc",
            EditState::EditedReadable
        )
        .unwrap()
        .1
        .is_empty());
}
