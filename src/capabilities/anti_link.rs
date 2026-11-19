/*
 * Base-Matter — Anti-link capability bridge, with NO private rules or secrets.
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use super::{
    bounded_response, CapabilityCall, CapabilityError, CapabilityScope, CommandContext,
    ProtectedCapabilityGateway,
};

pub const CAPABILITY_ID: &str = "anti.link";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LinkAction {
    Warn,
    Delete,
    Kick,
}
impl LinkAction {
    fn as_str(self) -> &'static str {
        match self {
            Self::Warn => "warn",
            Self::Delete => "delete",
            Self::Kick => "kick",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AntiLinkCommand {
    Status,
    SetEnabled(bool),
    SetAction(LinkAction),
    AddDomain(String),
    RemoveDomain(String),
    ListDomains,
}

/// Public invocation bridge; this function NEVER moderates a WhatsApp group.
/// Both native controls and external bot commands must be independently
/// authenticated by the protected gateway, including mutation permission.
pub fn invoke<G: ProtectedCapabilityGateway + ?Sized>(
    gateway: &G,
    context: &CommandContext,
    scope: CapabilityScope,
    command: AntiLinkCommand,
) -> Result<String, CapabilityError> {
    context.validate()?;
    if scope == CapabilityScope::Group
        && !context
            .group_id
            .as_deref()
            .is_some_and(|id| id.ends_with("@g.us"))
    {
        return Err(CapabilityError::InvalidContext);
    }

    let (action_id, arguments, writes) = match command {
        AntiLinkCommand::Status => ("status", vec![], false),
        AntiLinkCommand::SetEnabled(value) => (
            "configure.enabled",
            vec![if value { "on" } else { "off" }.to_owned()],
            true,
        ),
        AntiLinkCommand::SetAction(action) => {
            ("configure.action", vec![action.as_str().to_owned()], true)
        }
        AntiLinkCommand::AddDomain(domain) => {
            if !valid_domain(&domain) {
                return Err(CapabilityError::InvalidArgument);
            }
            ("domain.add", vec![domain], true)
        }
        AntiLinkCommand::RemoveDomain(domain) => {
            if !valid_domain(&domain) {
                return Err(CapabilityError::InvalidArgument);
            }
            ("domain.remove", vec![domain], true)
        }
        AntiLinkCommand::ListDomains => ("domain.list", vec![], false),
    };
    if writes && scope == CapabilityScope::GeneralDefaults {
        return Err(CapabilityError::Denied);
    }
    let call = CapabilityCall {
        capability_id: CAPABILITY_ID,
        action_id,
        scope,
        context: context.clone(),
        arguments,
    };
    bounded_response(gateway.invoke(&call)?)
}
fn valid_domain(domain: &str) -> bool {
    if domain.is_empty() || domain.len() > 253 || !domain.is_ascii() {
        return false;
    }
    domain.split('.').all(|label| {
        !label.is_empty()
            && label.len() <= 63
            && label.as_bytes()[0].is_ascii_alphanumeric()
            && label.as_bytes()[label.len() - 1].is_ascii_alphanumeric()
            && label
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
    })
}

// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
