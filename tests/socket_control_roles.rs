/*
 * Base-Matter — independent socket and group role lifecycle.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Result};
use matter_base::{
    activation::SocketActivationStore,
    control::{ControlDispatcher, DispatchResult},
    BotRegistry, CommandAuthorizer, CommandInvocation, CommandRouter, RoleStore,
};
use std::{fs, path::PathBuf, sync::Arc};

struct DenyBot;
impl CommandAuthorizer for DenyBot {
    fn authorize(&self, _: &CommandInvocation) -> Result<()> {
        bail!("Bot operations must not run during socket role control tests")
    }
}

fn fixture(path: &std::path::Path) -> (ControlDispatcher, Arc<RoleStore>) {
    let activation = Arc::new(SocketActivationStore::open(":memory:").unwrap());
    activation.provision("wa-alpha", "socket-owner").unwrap();
    let roles = Arc::new(RoleStore::open(path).unwrap());
    let router = Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(DenyBot)).unwrap());
    let dispatcher =
        ControlDispatcher::new(activation, Arc::new(BotRegistry::in_memory()), router, None)
            .with_socket_roles(Arc::clone(&roles));
    (dispatcher, roles)
}

fn temp_db() -> PathBuf {
    let mut salt = [0_u8; 12];
    getrandom::fill(&mut salt).unwrap();
    std::env::temp_dir().join(format!(
        "matter-socket-roles-{}-{}.db",
        std::process::id(),
        hex::encode(salt)
    ))
}

fn grants(roles: &RoleStore, scope: &str) -> Vec<serde_json::Value> {
    let data = roles.export_snapshot(scope, "socket-owner").unwrap();
    let parsed: serde_json::Value = serde_json::from_slice(&data).unwrap();
    parsed["grants"].as_array().unwrap().clone()
}

#[test]
fn socket_role_is_independent_of_bot_and_implicit_user_is_not_stored() {
    let db = temp_db();
    let (control, roles) = fixture(&db);
    let first = control
        .dispatch(
            "wa-alpha",
            "socket-owner",
            "private",
            None,
            "=/>set role 5215512345678 admin",
        )
        .unwrap();
    assert_eq!(
        first,
        DispatchResult::SocketRoleChanged {
            scope_id: "socket:wa-alpha".into(),
            user_id: "pn:5215512345678".into(),
            role_id: "admin".into(),
            granted: true,
            changed: true,
        }
    );
    assert_eq!(grants(&roles, "socket:wa-alpha").len(), 1);
    let repeated = control
        .dispatch(
            "wa-alpha",
            "socket-owner",
            "private",
            None,
            "=/>role 5215512345678 admin",
        )
        .unwrap();
    assert!(matches!(
        repeated,
        DispatchResult::SocketRoleChanged { changed: false, .. }
    ));
    control
        .dispatch(
            "wa-alpha",
            "socket-owner",
            "private",
            None,
            "=/>set role 5215512345678 moderator",
        )
        .unwrap();
    let current = grants(&roles, "socket:wa-alpha");
    assert_eq!(current.len(), 1);
    assert_eq!(current[0][1], "moderator");

    control
        .dispatch(
            "wa-alpha",
            "socket-owner",
            "private",
            None,
            "=/>set role 5215512345678 user",
        )
        .unwrap();
    assert!(grants(&roles, "socket:wa-alpha").is_empty());
    drop(control);
    drop(roles);
    let reopened = RoleStore::open(&db).unwrap();
    assert!(grants(&reopened, "socket:wa-alpha").is_empty());
    drop(reopened);
    fs::remove_file(db).unwrap();
}

#[test]
fn group_roles_are_separate_from_socket_roles_and_other_groups() {
    let db = temp_db();
    let (control, roles) = fixture(&db);
    control
        .dispatch(
            "wa-alpha",
            "socket-owner",
            "group",
            Some("120363111@g.us"),
            "=/>group role pn:5215512345678 trusted",
        )
        .unwrap();
    assert!(roles
        .export_snapshot("socket:wa-alpha", "socket-owner")
        .is_err());
    assert!(control
        .dispatch(
            "wa-alpha",
            "another-user",
            "group",
            Some("120363111@g.us"),
            "=/>group role pn:5215512345678 admin",
        )
        .is_err());
    assert!(control
        .dispatch(
            "wa-alpha",
            "socket-owner",
            "private",
            None,
            "=/>group role pn:5215512345678 admin",
        )
        .is_err());
    assert!(control
        .dispatch(
            "wa-alpha",
            "socket-owner",
            "group",
            Some("120363111@g.us"),
            "=/>set role 5215512345678 admin",
        )
        .is_err());
    assert!(control
        .dispatch(
            "wa-alpha",
            "socket-owner",
            "private",
            None,
            "=/>set role arbitrary-display-name admin",
        )
        .is_err());
    assert!(control
        .dispatch(
            "wa-alpha",
            "socket-owner",
            "private",
            None,
            "=/>set role 5215512345678 owner",
        )
        .is_err());
    drop(control);
    drop(roles);
    fs::remove_file(db).unwrap();
}
