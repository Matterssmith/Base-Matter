/*
 * Base-Matter — reusable community-scoped menu, help and h styling.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{anyhow, bail, Result};
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControlTheme {
    pub title: String,
    pub bot_id: String,
    pub accent: String,
}

impl ControlTheme {
    pub fn new(title: &str, bot_id: &str, accent: &str) -> Result<Self> {
        for value in [title, bot_id, accent] {
            if value.is_empty()
                || value.len() > 80
                || value.chars().any(char::is_control)
            {
                bail!("Invalid control theme");
            }
        }
        Ok(Self {
            title: title.to_owned(),
            bot_id: bot_id.to_owned(),
            accent: accent.to_owned(),
        })
    }

    pub fn render(&self, section: &str) -> Result<String> {
        if !matches!(section, "menu" | "help" | "h") {
            bail!("Unsupported control help section");
        }
        let heading = if section == "menu" { "MENÚ" } else { "AYUDA" };
        Ok(format!(
            "╭〔 {} · {} 〕\n┃ BOT ID: {}\n┃ =/>start  Comunidad REGISTER (inspección)\n┃ =/>status Estado del socket\n┃ =/>on / =/>off  Control del dueño\n┃ =/>menu / =/>help / =/>h\n╰━━━━━━━━━━\nCréditos: Hecho por Lournyx Matterssmith [Lou-Nyx].",
            self.accent, heading, self.bot_id
        ))
    }
}

/// Presentation only. The trusted host must authorize writes to this registry;
/// a theme never grants roles, session access or group administration.
pub struct ControlThemes {
    fallback: ControlTheme,
    communities: RwLock<HashMap<String, ControlTheme>>,
}

impl ControlThemes {
    pub fn new(fallback: ControlTheme) -> Self {
        Self {
            fallback,
            communities: RwLock::new(HashMap::new()),
        }
    }

    pub fn set_community(&self, community_id: &str, theme: ControlTheme) -> Result<()> {
        validate_community_id(community_id)?;
        self.communities
            .write()
            .map_err(|_| anyhow!("Community theme registry unavailable"))?
            .insert(community_id.to_owned(), theme);
        Ok(())
    }

    pub fn render(&self, community_id: Option<&str>, section: &str) -> Result<String> {
        let selected = if let Some(community_id) = community_id {
            validate_community_id(community_id)?;
            self.communities
                .read()
                .map_err(|_| anyhow!("Community theme registry unavailable"))?
                .get(community_id)
                .cloned()
        } else {
            None
        };
        selected.unwrap_or_else(|| self.fallback.clone()).render(section)
    }
}

fn validate_community_id(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > 128
        || !value.bytes().all(|b| b.is_ascii_alphanumeric() || b"@._:-".contains(&b))
    {
        bail!("Invalid community identifier");
    }
    Ok(())
}
