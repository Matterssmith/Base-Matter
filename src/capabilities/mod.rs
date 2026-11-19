/*
 * Base-Matter — Public protected-capability bridge contracts.
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
pub mod anti_link;

use std::{error::Error, fmt};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityScope {
    Group,
    Socket,
    GeneralDefaults,
}

/// These are routing claims, NOT an authenticated proof or permission.
/// A trusted gateway must independently verify actor, socket, bot and group.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandContext {
    pub bot_instance_id: String,
    pub actor_id: String,
    pub socket_id: String,
    pub group_id: Option<String>,
    pub message_id: String,
}

impl CommandContext {
    pub fn validate(&self) -> Result<(), CapabilityError> {
        if !valid_id(&self.bot_instance_id)
            || !valid_id(&self.actor_id)
            || !valid_id(&self.socket_id)
            || !valid_id(&self.message_id)
            || self.group_id.as_deref().is_some_and(|v| !valid_id(v))
        {
            return Err(CapabilityError::InvalidContext);
        }
        Ok(())
    }
}
fn valid_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.bytes().all(|b| b.is_ascii_alphanumeric() || b"@._:-".contains(&b))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityCall {
    pub capability_id: &'static str,
    pub action_id: &'static str,
    pub scope: CapabilityScope,
    pub context: CommandContext,
    pub arguments: Vec<String>,
}

/// Only a separately authenticated and authorized service may implement
/// production execution. No private implementation or key is shipped here.
pub trait ProtectedCapabilityGateway: Send + Sync {
    fn invoke(&self, call: &CapabilityCall) -> Result<String, CapabilityError>;
}

/// Safe fallback when a protected capability is not provisioned.
pub struct UnavailableGateway;
impl ProtectedCapabilityGateway for UnavailableGateway {
    fn invoke(&self, _: &CapabilityCall) -> Result<String, CapabilityError> {
        Err(CapabilityError::Unavailable)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityError {
    InvalidContext,
    InvalidArgument,
    Denied,
    Unavailable,
    Failed,
    ResponseTooLarge,
}
impl fmt::Display for CapabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidContext => "Invalid capability context",
            Self::InvalidArgument => "Invalid capability argument",
            Self::Denied => "Capability denied",
            Self::Unavailable => "Protected capability unavailable",
            Self::Failed => "Protected capability execution failed",
            Self::ResponseTooLarge => "Protected capability response too large",
        })
    }
}
impl Error for CapabilityError {}

pub(crate) fn bounded_response(value: String) -> Result<String, CapabilityError> {
    if value.len() > 4096 {
        return Err(CapabilityError::ResponseTooLarge);
    }
    Ok(value)
}

// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
