/*
 * Matter-Base — Universal Command Router
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use crate::{CommandAuditOutcome, CommandAuditStore};
use anyhow::{anyhow, bail, Result};
use std::collections::{HashMap, HashSet};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, RwLock};

const MAX_MESSAGE_BYTES: usize = 16 * 1024;
const MAX_ARGUMENTS: usize = 256;
const MAX_NAME_BYTES: usize = 64;
const MAX_COMMANDS: usize = 4096;
const MAX_ALIASES: usize = 64;
const MAX_MIDDLEWARE: usize = 256;

/// Platform-neutral command invocation created from a prefixed message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandInvocation {
    pub bot_instance_id: String,
    pub actor_id: String,
    pub source_id: String,
    pub command: String,
    pub args: Vec<String>,
}

/// Authenticated request policy used before middleware and handler execution.
/// Implementations must derive actor identity from trusted request state and
/// enforce bot ownership, permissions, activation, and resource scope.
pub trait CommandAuthorizer: Send + Sync {
    fn authorize(&self, invocation: &CommandInvocation) -> Result<()>;
}

type Handler = dyn Fn(&CommandInvocation) -> Result<String> + Send + Sync;
type Middleware = dyn Fn(&CommandInvocation) -> Result<()> + Send + Sync;

struct CommandEntry {
    handler: Arc<Handler>,
}

#[derive(Default)]
struct RouterState {
    handlers: HashMap<String, CommandEntry>,
    names: HashMap<String, String>,
    middleware: Vec<Arc<Middleware>>,
}

/// Synchronous, platform-neutral command dispatch with mandatory authorization.
/// Message arguments are separated on ASCII/Unicode whitespace; quoting and
/// platform-specific parsing are the adapter's responsibility.
pub struct CommandRouter {
    prefixes: Vec<String>,
    authorizer: Arc<dyn CommandAuthorizer>,
    state: RwLock<RouterState>,
    audit: Option<Arc<CommandAuditStore>>,
}

impl CommandRouter {
    pub fn new(prefixes: Vec<String>, authorizer: Arc<dyn CommandAuthorizer>) -> Result<Self> {
        if prefixes.is_empty() || prefixes.len() > 16 {
            bail!("Command router requires between 1 and 16 prefixes");
        }
        let mut unique = HashSet::new();
        for prefix in &prefixes {
            if prefix.is_empty()
                || prefix.len() > 16
                || prefix.chars().any(char::is_whitespace)
                || !unique.insert(prefix.clone())
            {
                bail!("Invalid or duplicate command prefix");
            }
        }
        let mut prefixes = prefixes;
        prefixes.sort_by_key(|prefix| std::cmp::Reverse(prefix.len()));
        Ok(Self {
            prefixes,
            authorizer,
            state: RwLock::new(RouterState::default()),
            audit: None,
        })
    }

    /// Persist a recognized command attempt before invoking any callback, and
    /// its outcome before returning. Audit write failure stops dispatch; a
    /// failed outcome write cannot undo handler effects. Never retry blindly.
    /// Invalid/unregistered commands and non-command messages are not recorded.
    pub fn new_with_audit(
        prefixes: Vec<String>,
        authorizer: Arc<dyn CommandAuthorizer>,
        audit: Arc<CommandAuditStore>,
    ) -> Result<Self> {
        let mut router = Self::new(prefixes, authorizer)?;
        router.audit = Some(audit);
        Ok(router)
    }

    /// Adds a synchronous middleware callback. Callbacks execute in registration
    /// order after authorization and outside the router lock.
    pub fn add_middleware<F>(&self, middleware: F) -> Result<()>
    where
        F: Fn(&CommandInvocation) -> Result<()> + Send + Sync + 'static,
    {
        let mut state = self
            .state
            .write()
            .map_err(|_| anyhow!("Command router lock poisoned"))?;
        if state.middleware.len() >= MAX_MIDDLEWARE {
            bail!("Command router middleware capacity reached");
        }
        state.middleware.push(Arc::new(middleware));
        Ok(())
    }

    /// Registers a command name and aliases atomically. Names are matched
    /// case-insensitively and may contain ASCII letters, digits, `_`, and `-`.
    pub fn register<F>(&self, name: &str, aliases: &[String], handler: F) -> Result<()>
    where
        F: Fn(&CommandInvocation) -> Result<String> + Send + Sync + 'static,
    {
        let name = normalize_command_name(name)?;
        if aliases.len() > MAX_ALIASES {
            bail!("Command has too many aliases");
        }
        let mut normalized = Vec::with_capacity(aliases.len() + 1);
        normalized.push(name.clone());
        for alias in aliases {
            normalized.push(normalize_command_name(alias)?);
        }
        let mut unique = HashSet::new();
        if normalized.iter().any(|entry| !unique.insert(entry.clone())) {
            bail!("Duplicate command name or alias");
        }

        let mut state = self
            .state
            .write()
            .map_err(|_| anyhow!("Command router lock poisoned"))?;
        if normalized
            .iter()
            .any(|entry| state.names.contains_key(entry))
        {
            bail!("Command name or alias is already registered");
        }
        if state.handlers.len() >= MAX_COMMANDS {
            bail!("Command router capacity reached");
        }
        state.handlers.insert(
            name.clone(),
            CommandEntry {
                handler: Arc::new(handler),
            },
        );
        for entry in normalized {
            state.names.insert(entry, name.clone());
        }
        Ok(())
    }

    /// Routes a raw message. Non-command messages return `Ok(None)`. For a
    /// matching prefix, invalid input, denied access, and handler failures are
    /// returned as errors; successful dispatch returns the handler response.
    pub fn route(
        &self,
        bot_instance_id: &str,
        actor_id: &str,
        source_id: &str,
        message: &str,
    ) -> Result<Option<String>> {
        validate_context_id(bot_instance_id, "bot instance ID")?;
        validate_context_id(actor_id, "actor ID")?;
        validate_context_id(source_id, "source ID")?;
        if message.len() > MAX_MESSAGE_BYTES {
            bail!("Command message exceeds the size limit");
        }
        let Some(prefix) = self
            .prefixes
            .iter()
            .find(|prefix| message.starts_with(*prefix))
        else {
            return Ok(None);
        };
        let body = message[prefix.len()..].trim();
        let mut tokens = body.split_whitespace();
        let command = tokens
            .next()
            .ok_or_else(|| anyhow!("Command name is missing"))?;
        let command = normalize_command_name(command)?;
        let args = tokens.map(str::to_owned).collect::<Vec<_>>();
        if args.len() > MAX_ARGUMENTS {
            bail!("Command has too many arguments");
        }

        let (canonical, handler, middleware) = {
            let state = self
                .state
                .read()
                .map_err(|_| anyhow!("Command router lock poisoned"))?;
            let canonical = state
                .names
                .get(&command)
                .ok_or_else(|| anyhow!("Command is not registered"))?;
            let entry = state
                .handlers
                .get(canonical)
                .ok_or_else(|| anyhow!("Command registry is inconsistent"))?;
            (
                canonical.clone(),
                Arc::clone(&entry.handler),
                state.middleware.clone(),
            )
        };
        let invocation = CommandInvocation {
            bot_instance_id: bot_instance_id.to_owned(),
            actor_id: actor_id.to_owned(),
            source_id: source_id.to_owned(),
            command: canonical,
            args,
        };

        let attempt = self
            .audit
            .as_ref()
            .map(|audit| audit.begin(&invocation))
            .transpose()
            .map_err(|_| anyhow!("Command audit persistence failed"))?;
        let finish = |outcome| -> Result<()> {
            if let (Some(audit), Some(attempt)) = (&self.audit, attempt) {
                audit
                    .finish(attempt, outcome)
                    .map_err(|_| anyhow!("Command audit persistence failed"))?;
            }
            Ok(())
        };
        let authorization =
            catch_unwind(AssertUnwindSafe(|| self.authorizer.authorize(&invocation)))
                .map_err(|_| anyhow!("Command authorization denied"))
                .and_then(|result| result.map_err(|_| anyhow!("Command authorization denied")));
        if let Err(error) = authorization {
            finish(CommandAuditOutcome::Denied)?;
            return Err(error);
        }
        for middleware in middleware {
            let result = catch_unwind(AssertUnwindSafe(|| middleware(&invocation)))
                .map_err(|_| anyhow!("Command middleware rejected"))
                .and_then(|result| result.map_err(|_| anyhow!("Command middleware rejected")));
            if let Err(error) = result {
                finish(CommandAuditOutcome::MiddlewareRejected)?;
                return Err(error);
            }
        }
        let result = catch_unwind(AssertUnwindSafe(|| handler(&invocation)))
            .map_err(|_| anyhow!("Command execution failed"))
            .and_then(|result| result.map_err(|_| anyhow!("Command execution failed")));
        finish(if result.is_ok() {
            CommandAuditOutcome::Succeeded
        } else {
            CommandAuditOutcome::Failed
        })?;
        result.map(Some)
    }
}

fn normalize_command_name(value: &str) -> Result<String> {
    if value.is_empty()
        || value.len() > MAX_NAME_BYTES
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
    {
        bail!("Invalid command name");
    }
    Ok(value.to_ascii_lowercase())
}

fn validate_context_id(value: &str, label: &str) -> Result<()> {
    if value.trim().is_empty() || value.len() > 128 || value.chars().any(char::is_control) {
        bail!("Invalid {label}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{CommandAuthorizer, CommandInvocation, CommandRouter};
    use anyhow::{bail, Result};
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Arc;

    struct Policy {
        allowed: AtomicBool,
    }

    impl CommandAuthorizer for Policy {
        fn authorize(&self, _invocation: &CommandInvocation) -> Result<()> {
            if self.allowed.load(Ordering::SeqCst) {
                Ok(())
            } else {
                bail!("sensitive policy detail")
            }
        }
    }

    fn router(policy: Arc<Policy>) -> CommandRouter {
        CommandRouter::new(vec!["#".to_owned(), "!".to_owned()], policy).unwrap()
    }

    #[test]
    fn resolves_alias_and_dispatches_context_and_arguments() {
        let router = router(Arc::new(Policy {
            allowed: AtomicBool::new(true),
        }));
        router
            .register("menu", &["m".to_owned()], |invocation| {
                Ok(format!(
                    "{}:{}:{}:{}",
                    invocation.bot_instance_id,
                    invocation.actor_id,
                    invocation.command,
                    invocation.args.join(",")
                ))
            })
            .unwrap();

        assert_eq!(
            router
                .route("bot-1", "user-1", "room-1", "#M one two")
                .unwrap(),
            Some("bot-1:user-1:menu:one,two".to_owned())
        );
        assert_eq!(
            router
                .route("bot-1", "user-1", "room-1", " ordinary text")
                .unwrap(),
            None
        );
    }

    #[test]
    fn authorization_uses_canonical_name_for_aliases() {
        struct MenuOnly;
        impl CommandAuthorizer for MenuOnly {
            fn authorize(&self, invocation: &CommandInvocation) -> Result<()> {
                if invocation.command == "menu" {
                    Ok(())
                } else {
                    bail!("wrong command scope")
                }
            }
        }

        let router = CommandRouter::new(vec!["#".to_owned()], Arc::new(MenuOnly)).unwrap();
        router
            .register("menu", &["m".to_owned()], |_| Ok("menu".to_owned()))
            .unwrap();
        assert_eq!(
            router.route("bot-1", "user-1", "room-1", "#m").unwrap(),
            Some("menu".to_owned())
        );
    }

    #[test]
    fn authorization_denial_is_sanitized_and_prevents_middleware_and_handler() {
        let policy = Arc::new(Policy {
            allowed: AtomicBool::new(false),
        });
        let router = router(policy);
        let middleware_calls = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&middleware_calls);
        router
            .add_middleware(move |_| {
                observed.fetch_add(1, Ordering::SeqCst);
                Ok(())
            })
            .unwrap();
        let handler_calls = Arc::clone(&middleware_calls);
        router
            .register("help", &[], move |_| {
                handler_calls.fetch_add(1, Ordering::SeqCst);
                Ok("help".to_owned())
            })
            .unwrap();

        let error = router
            .route("bot-1", "intruder", "room-1", "#help")
            .unwrap_err()
            .to_string();
        assert_eq!(error, "Command authorization denied");
        assert_eq!(middleware_calls.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn duplicate_aliases_prefixes_and_unregistered_commands_are_rejected() {
        let policy = Arc::new(Policy {
            allowed: AtomicBool::new(true),
        });
        assert!(CommandRouter::new(vec!["#".to_owned(), "#".to_owned()], policy.clone()).is_err());
        let router = router(policy);
        router
            .register("help", &["h".to_owned()], |_| Ok(String::new()))
            .unwrap();
        assert!(router
            .register("menu", &["H".to_owned()], |_| Ok(String::new()))
            .is_err());
        assert!(router
            .route("bot-1", "user-1", "room-1", "#missing")
            .is_err());
    }

    #[test]
    fn middleware_runs_in_order_and_can_stop_dispatch() {
        let router = router(Arc::new(Policy {
            allowed: AtomicBool::new(true),
        }));
        let order = Arc::new(std::sync::Mutex::new(Vec::new()));
        for marker in [1, 2] {
            let order = Arc::clone(&order);
            router
                .add_middleware(move |_| {
                    order.lock().unwrap().push(marker);
                    Ok(())
                })
                .unwrap();
        }
        router
            .register("run", &[], |_| Ok("done".to_owned()))
            .unwrap();
        assert_eq!(
            router.route("bot-1", "user-1", "room-1", "!run").unwrap(),
            Some("done".to_owned())
        );
        assert_eq!(*order.lock().unwrap(), [1, 2]);

        let blocked = CommandRouter::new(
            vec!["#".to_owned()],
            Arc::new(Policy {
                allowed: AtomicBool::new(true),
            }),
        )
        .unwrap();
        blocked
            .add_middleware(|_| bail!("middleware denied"))
            .unwrap();
        blocked
            .register("run", &[], |_| Ok("must not execute".to_owned()))
            .unwrap();
        assert!(blocked.route("bot-1", "user-1", "room-1", "#run").is_err());
    }

    #[test]
    fn rejects_oversized_messages_and_invalid_context_before_dispatch() {
        let router = router(Arc::new(Policy {
            allowed: AtomicBool::new(true),
        }));
        router
            .register("run", &[], |_| Ok("done".to_owned()))
            .unwrap();
        assert!(router
            .route(
                "bot-1",
                "user-1",
                "room-1",
                &format!("#run {}", "x".repeat(16 * 1024))
            )
            .is_err());
        assert!(router.route("", "user-1", "room-1", "#run").is_err());
    }

    #[test]
    fn callback_panics_are_sanitized_and_do_not_poison_router_state() {
        let router = router(Arc::new(Policy {
            allowed: AtomicBool::new(true),
        }));
        router
            .register("panic", &[], |_| panic!("private callback diagnostic"))
            .unwrap();
        router
            .register("ok", &[], |_| Ok("done".to_owned()))
            .unwrap();

        let error = router
            .route("bot-1", "user-1", "room-1", "#panic")
            .unwrap_err()
            .to_string();
        assert_eq!(error, "Command execution failed");
        assert!(!error.contains("private"));
        assert_eq!(
            router.route("bot-1", "user-1", "room-1", "#ok").unwrap(),
            Some("done".to_owned())
        );
    }
}
