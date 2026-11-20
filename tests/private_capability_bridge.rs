/*
 * Base-Matter — Protected bridge contract tests.
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use matter_base::capabilities::{
    anti_link::{invoke, AntiLinkCommand, LinkAction, CAPABILITY_ID},
    CapabilityCall, CapabilityError, CapabilityScope, CommandContext, ProtectedCapabilityGateway,
    UnavailableGateway,
};
use std::sync::Mutex;

fn context() -> CommandContext {
    CommandContext {
        bot_instance_id: "my-bot".into(),
        actor_id: "actor@s.whatsapp.net".into(),
        socket_id: "socket-1".into(),
        group_id: Some("group@g.us".into()),
        message_id: "msg-1".into(),
    }
}
#[derive(Default)]
struct RecordingGateway {
    calls: Mutex<Vec<CapabilityCall>>,
}
impl ProtectedCapabilityGateway for RecordingGateway {
    fn invoke(&self, call: &CapabilityCall) -> Result<String, CapabilityError> {
        self.calls.lock().unwrap().push(call.clone());
        Ok("disabled".into())
    }
}
#[test]
fn forwards_only_named_capability() {
    let gateway = RecordingGateway::default();
    let result = invoke(
        &gateway,
        &context(),
        CapabilityScope::Group,
        AntiLinkCommand::Status,
    );
    assert_eq!(result.unwrap(), "disabled");
    let calls = gateway.calls.lock().unwrap();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].capability_id, CAPABILITY_ID);
    assert_eq!(calls[0].action_id, "status");
}
#[test]
fn no_protected_gateway_denies() {
    assert_eq!(
        invoke(
            &UnavailableGateway,
            &context(),
            CapabilityScope::Group,
            AntiLinkCommand::Status
        ),
        Err(CapabilityError::Unavailable)
    );
}
#[test]
fn global_mutations_are_denied_before_gateway() {
    let gateway = RecordingGateway::default();
    assert_eq!(
        invoke(
            &gateway,
            &context(),
            CapabilityScope::GeneralDefaults,
            AntiLinkCommand::SetEnabled(true)
        ),
        Err(CapabilityError::Denied)
    );
    assert!(gateway.calls.lock().unwrap().is_empty());
}
#[test]
fn group_scope_requires_group_jid() {
    let gateway = RecordingGateway::default();
    let mut c = context();
    c.group_id = None;
    assert_eq!(
        invoke(
            &gateway,
            &c,
            CapabilityScope::Group,
            AntiLinkCommand::Status
        ),
        Err(CapabilityError::InvalidContext)
    );
}
#[test]
fn unsafe_domain_is_not_forwarded() {
    let gateway = RecordingGateway::default();
    assert_eq!(
        invoke(
            &gateway,
            &context(),
            CapabilityScope::Group,
            AntiLinkCommand::AddDomain("https://example.com/".into())
        ),
        Err(CapabilityError::InvalidArgument)
    );
    assert!(gateway.calls.lock().unwrap().is_empty());
}
#[test]
fn set_action_is_only_forwarded_as_intent() {
    let gateway = RecordingGateway::default();
    invoke(
        &gateway,
        &context(),
        CapabilityScope::Socket,
        AntiLinkCommand::SetAction(LinkAction::Warn),
    )
    .unwrap();
    let calls = gateway.calls.lock().unwrap();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].action_id, "configure.action");
    assert_eq!(calls[0].arguments, vec!["warn"]);
}
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
