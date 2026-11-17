/*
 * Base-Matter — Rust global extensions security tests.
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::Result;
use matter_base::{
    activation::SocketActivationStore,
    control::{ControlDispatcher,DispatchResult},
    BotRegistry,CommandAuthorizer,CommandInvocation,CommandRouter,
    ControlExtensionRegistry,
};
use std::sync::{Arc,atomic::{AtomicUsize,Ordering}};

struct DenyBot;
impl CommandAuthorizer for DenyBot {
    fn authorize(&self,_:&CommandInvocation)->Result<()> {
        anyhow::bail!("Bot-Matter remains blocked")
    }
}
fn setup()->(ControlDispatcher,Arc<ControlExtensionRegistry>,Arc<SocketActivationStore>) {
    let activation=Arc::new(SocketActivationStore::open(":memory:").unwrap());
    activation.provision("socket-a","alice").unwrap();
    activation.provision("socket-b","bob").unwrap();
    let router=Arc::new(CommandRouter::new(vec!["#".into()],Arc::new(DenyBot)).unwrap());
    let extensions=Arc::new(ControlExtensionRegistry::new());
    let dispatcher=ControlDispatcher::new(
        activation.clone(),Arc::new(BotRegistry::in_memory()),router,None
    ).with_control_extensions(extensions.clone());
    (dispatcher,extensions,activation)
}
#[test]
fn owner_can_call_extension_with_socket_off_but_others_cannot() {
    let (control,registry,_) = setup();
    let calls=Arc::new(AtomicUsize::new(0));
    let copy=calls.clone();
    registry.register("inspect",&["who"],move |ctx,args|{
        copy.fetch_add(1,Ordering::SeqCst);
        Ok(format!("{}:{}:{}",ctx.actor_id,ctx.socket_id,args.join(":")))
    }).unwrap();
    assert_eq!(
        control.dispatch("socket-a","alice","private",None,"=/>who info").unwrap(),
        DispatchResult::Extension{
            command:"inspect".into(),output:"alice:socket-a:info".into()
        }
    );
    assert!(control.dispatch("socket-a","mallory","private",None,"=/>who").is_err());
    assert!(control.dispatch("socket-b","alice","private",None,"=/>who").is_err());
    assert_eq!(calls.load(Ordering::SeqCst),1);
}
#[test]
fn aliases_cannot_run_using_legacy_control_prefixes() {
    let (control,registry,_) = setup();
    registry.register("inspect",&[],|_,_|Ok("ok".into())).unwrap();
    assert!(control.dispatch("socket-a","alice","private",None,"=inspect").is_err());
    assert!(control.dispatch("socket-a","alice","private",None,"/>inspect").is_err());
    assert!(control.dispatch("socket-a","alice","private",None,"=/>unknown").is_err());
    assert_eq!(
        control.dispatch("socket-a","alice","private",None,"#inspect").unwrap(),
        DispatchResult::Inactive
    );
}
#[test]
fn reserved_names_and_alias_collisions_fail_closed() {
    let (_,registry,_) = setup();
    for name in ["on","off","status","set","unset","ping","Abc","","bad/name"] {
        assert!(registry.register(name,&[],|_,_|Ok("x".into())).is_err());
    }
    registry.register("inspect",&["alias"],|_,_|Ok("yes".into())).unwrap();
    assert!(registry.register("other",&["alias"],|_,_|Ok("bad".into())).is_err());
    assert!(registry.register("other",&["other"],|_,_|Ok("bad".into())).is_err());
}
#[test]
fn callbacks_are_sanitized_and_cannot_expose_secrets() {
    let (control,registry,_) = setup();
    registry.register("bad",&[],|_,_|anyhow::bail!("sensitive token")).unwrap();
    registry.register("panic",&[],|_,_|->Result<String>{panic!("secret panic");}).unwrap();
    for command in ["bad","panic"] {
        assert_eq!(
            control.dispatch("socket-a","alice","private",None,&format!("=/>{}",command))
                .unwrap_err().to_string(),
            "Control extension execution failed"
        );
    }
}
#[test]
fn oversized_inputs_and_outputs_are_rejected() {
    let (control,registry,_) = setup();
    registry.register("long",&[],|_,_|Ok("a".repeat(4097))).unwrap();
    assert!(control.dispatch("socket-a","alice","private",None,"=/>long").is_err());
    registry.register("args",&[],|_,args|Ok(args.len().to_string())).unwrap();
    let many=format!("=/>args {}",vec!["one";17].join(" "));
    assert!(control.dispatch("socket-a","alice","private",None,&many).is_err());
    let large=format!("=/>args {}", "x".repeat(257));
    assert!(control.dispatch("socket-a","alice","private",None,&large).is_err());
}
#[test]
fn existing_power_behavior_cannot_be_overridden() {
    let (control,registry,activation) = setup();
    registry.register("inspect",&[],|_,_|Ok("ok".into())).unwrap();
    assert_eq!(
        control.dispatch("socket-a","alice","private",None,"=/>on").unwrap(),
        DispatchResult::Control{socket_id:"socket-a".into(),powered:true}
    );
    assert!(!activation.resolve_persisted("socket-a",None).unwrap().enabled);
    assert!(control.dispatch("socket-a","alice","private",None,"=/>ping").is_err());
}
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
