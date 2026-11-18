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
        .command(SOCKET, Some(GROUP), &args(&["linkwa", "on"]))
        .unwrap();
    let accepted = store
        .inspect_links(SOCKET, GROUP, "https://chat.whatsapp.com/invite")
        .unwrap();
    assert_eq!(accepted.len(), 1);
    assert_eq!(accepted[0].rule, "antilinkwa");
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
