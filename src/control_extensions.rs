/*
 * Base-Matter — authorized global control command extensions.
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{anyhow, bail, Result};
use std::collections::{HashMap, HashSet};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, RwLock};

const RESERVED: &[&str] = &["on", "off", "status", "set", "unset", "ping"];
type Handler = dyn Fn(&ControlExtensionContext<'_>, &[String]) -> Result<String> + Send + Sync;

#[derive(Debug)]
pub struct ControlExtensionContext<'a> {
    pub socket_id: &'a str,
    pub actor_id: &'a str,
    pub source_id: &'a str,
    pub group_id: Option<&'a str>,
}
struct Entry {
    canonical: String,
    handler: Arc<Handler>,
}
#[derive(Default)]
pub struct ControlExtensionRegistry {
    entries: RwLock<HashMap<String, Arc<Entry>>>,
}
impl ControlExtensionRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Only trusted Rust host code may register handlers. Built-in
    /// control verbs cannot be overridden, even through aliases.
    pub fn register<F>(&self, name: &str, aliases: &[&str], handler: F) -> Result<()>
    where
        F: Fn(&ControlExtensionContext<'_>, &[String]) -> Result<String> + Send + Sync + 'static,
    {
        validate_name(name)?;
        if aliases.len() > 16 {
            bail!("Too many control aliases");
        }
        let mut all = HashSet::new();
        all.insert(name.to_owned());
        for alias in aliases {
            validate_name(alias)?;
            if !all.insert((*alias).to_owned()) {
                bail!("Duplicate control alias");
            }
        }
        let entry = Arc::new(Entry {
            canonical: name.to_owned(),
            handler: Arc::new(handler),
        });
        let mut map = self
            .entries
            .write()
            .map_err(|_| anyhow!("Control extension registry unavailable"))?;
        if all.iter().any(|alias| map.contains_key(alias)) {
            bail!("Control extension already registered");
        }
        for alias in all {
            map.insert(alias, Arc::clone(&entry));
        }
        Ok(())
    }

    /// Runs after ControlDispatcher authorizes the current socket owner.
    /// No registry lock is held while calling application code.
    pub fn execute(
        &self,
        name: &str,
        context: &ControlExtensionContext<'_>,
        args: &[String],
    ) -> Result<Option<(String, String)>> {
        if args.len() > 16 || args.iter().any(|arg| arg.len() > 256) {
            bail!("Control extension arguments exceed limit");
        }
        let entry = {
            let map = self
                .entries
                .read()
                .map_err(|_| anyhow!("Control extension registry unavailable"))?;
            map.get(name).cloned()
        };
        let Some(entry) = entry else {
            return Ok(None);
        };
        let value = catch_unwind(AssertUnwindSafe(|| (entry.handler)(context, args)))
            .map_err(|_| anyhow!("Control extension execution failed"))?
            .map_err(|_| anyhow!("Control extension execution failed"))?;
        if value.len() > 4096 {
            bail!("Control extension response too large");
        }
        Ok(Some((entry.canonical.clone(), value)))
    }
}
fn validate_name(name: &str) -> Result<()> {
    // The owner explicitly reserved this exact recovery command spelling.
    // Other '+'-containing names remain invalid.
    if matches!(name, "add++" | "menú") {
        return Ok(());
    }
    if name.is_empty()
        || name.len() > 64
        || RESERVED.contains(&name)
        || !name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-' || b == b'_')
    {
        bail!("Invalid or reserved control command");
    }
    Ok(())
}
// Créditos:
// Hecho por Lournyx Matterssmith [Lou-Nyx].
