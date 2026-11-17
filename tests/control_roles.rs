/*
 * Matter-Base — Reserved role control regression.
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use anyhow::Result;
use matter_base::{
    activation::SocketActivationStore,
    control::{ControlDispatcher, DispatchResult},
    BotRegistry, CommandAuthorizer, CommandInvocation, CommandRouter, RoleStore,
};
use std::sync::Arc;

struct Allow;
impl CommandAuthorizer for Allow {
    fn authorize(&self, _: &CommandInvocation) -> Result<()> {
        Ok(())
    }
}

#[test]
fn role_control_works_while_off_and_observes_current_owners() {
    let mut bytes = [0_u8; 16];
    getrandom::fill(&mut bytes).unwrap();
    let db = std::env::temp_dir().join(format!("matter-role-cmd-{}.db", hex::encode(bytes)));
    let activation = Arc::new(SocketActivationStore::open(&db).unwrap());
    activation.provision("socket", "alice").unwrap();
    let bots = Arc::new(BotRegistry::in_memory());
    let bot = bots.register("family", "v1", "name", "alice").unwrap();
    let bot_id = bot.instance_id().to_owned();
    activation
        .bind_bot("socket", "alice", &bot_id, &bots)
        .unwrap();
    let roles = Arc::new(RoleStore::open(&db).unwrap());
    roles.provision_scope(&bot_id, "alice").unwrap();
    roles.create_role(&bot_id, "alice", "staff", None).unwrap();
    let router = Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(Allow)).unwrap());
    let dispatcher = ControlDispatcher::new_with_persistent_policy(
        activation.clone(),
        bots.clone(),
        router.clone(),
    )
    .with_roles(roles.clone());
    assert!(
        !activation
            .resolve_persisted("socket", None)
            .unwrap()
            .enabled
    );
    assert!(dispatcher
        .dispatch("socket", "bob", "room", None, "=/>set role bob staff")
        .is_err());
    assert!(dispatcher
        .dispatch("socket", "alice", "room", None, "=/>set role bob unknown")
        .is_err());
    assert_eq!(
        dispatcher
            .dispatch("socket", "alice", "room", None, "=/>set role bob staff")
            .unwrap(),
        DispatchResult::RoleChanged {
            bot_instance_id: bot_id.clone(),
            user_id: "bob".into(),
            role_id: "staff".into(),
            granted: true,
            changed: true,
        }
    );
    assert_eq!(
        dispatcher
            .dispatch("socket", "alice", "room", None, "/>set role bob staff")
            .unwrap(),
        DispatchResult::RoleChanged {
            bot_instance_id: bot_id.clone(),
            user_id: "bob".into(),
            role_id: "staff".into(),
            granted: true,
            changed: false,
        }
    );
    assert_eq!(
        dispatcher
            .dispatch("socket", "alice", "room", None, "=unset role bob staff")
            .unwrap(),
        DispatchResult::RoleChanged {
            bot_instance_id: bot_id.clone(),
            user_id: "bob".into(),
            role_id: "staff".into(),
            granted: false,
            changed: true,
        }
    );
    let plain =
        ControlDispatcher::new_with_persistent_policy(activation.clone(), bots.clone(), router);
    assert!(plain
        .dispatch("socket", "alice", "room", None, "=set role bob staff")
        .is_err());
    bots.transfer_owner(&bot_id, "alice", "charlie").unwrap();
    assert!(dispatcher
        .dispatch("socket", "alice", "room", None, "=set role bob staff")
        .is_err());
    assert!(dispatcher
        .dispatch("socket", "charlie", "room", None, "=set role bob staff")
        .is_err());
    assert!(
        !activation
            .resolve_persisted("socket", None)
            .unwrap()
            .enabled
    );
    assert_eq!(
        roles.audit_history(&bot_id, "alice", 100, 0).unwrap().len(),
        4
    );
    drop(dispatcher);
    drop(plain);
    drop(roles);
    drop(activation);
    let _ = std::fs::remove_file(db);
}
