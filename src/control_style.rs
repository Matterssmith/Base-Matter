/*
 * Base-Matter — versionable Matterssmith control templates independent of command logic.
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
    template: String,
}

impl ControlTheme {
    pub fn new(title: &str, bot_id: &str, accent: &str) -> Result<Self> {
        for value in [title, bot_id, accent] {
            if value.is_empty() || value.len() > 80 || value.chars().any(char::is_control) {
                bail!("Invalid control theme");
            }
        }
        Ok(Self {
            title: title.to_owned(),
            bot_id: bot_id.to_owned(),
            accent: accent.to_owned(),
            template: include_str!("../templates/matterssmith-control.txt").to_owned(),
        })
    }

    /// The layout lives in a separate UTF-8 file, not in command handlers.
    /// An authorized host may replace it per scope after validation.
    pub fn with_template(mut self, source: &str) -> Result<Self> {
        validate_template(source)?;
        self.template = source.to_owned();
        Ok(self)
    }

    /// Render a verified, bounded reply. Refuse suspicious control chars;
    /// never reinterpret literal command text or authorize an operation.
    pub fn decorate(&self, body: &str) -> Result<String> {
        const DEFAULT_HEADER: &str = "*꒦꒷✰꒷꒦︶꒦꒷✧꒷꒦︶꒦꒷✰꒷꒦*";
        const CREDIT: &str = "Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].";
        if body.starts_with(DEFAULT_HEADER) && body.ends_with(CREDIT) {
            return Ok(body.to_owned());
        }
        if body.is_empty()
            || body.len() > 3200
            || body.chars().any(|c| c.is_control() && c != '\n')
        {
            bail!("Invalid control response");
        }
        let result = self.template
            .replace("{{TITLE}}", &self.title)
            .replace("{{ACCENT}}", &self.accent)
            .replace("{{BODY}}", body.trim_end_matches('\n'));
        if result.len() > 3800 {
            bail!("Themed control reply exceeds transport limit");
        }
        Ok(result)
    }

    /// Global menu has no categories. Its content is also a separate file.
    pub fn render(&self, section: &str) -> Result<String> {
        if section != "menu" {
            bail!("Global control has no menu categories");
        }
        self.decorate(include_str!("../templates/matterssmith-menu.txt"))
    }
}

/// Validate the format itself before storing or rendering it. The footer
/// attribution is mandatory and a user template cannot inject controls.
fn validate_template(source: &str) -> Result<()> {
    const CREDIT: &str = "Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].";
    if source.is_empty()
        || source.len() > 1500
        || source.chars().any(|c| c.is_control() && c != '\n')
        || !source.contains("{{BODY}}")
        || !source.contains("{{TITLE}}")
        || !source.contains("{{ACCENT}}")
        || source.matches("{{BODY}}").count() != 1
        || !source.contains(CREDIT)
    {
        bail!("Invalid control template");
    }
    Ok(())
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
        selected
            .unwrap_or_else(|| self.fallback.clone())
            .render(section)
    }
}

fn validate_community_id(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > 128
        || !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"@._:-".contains(&b))
    {
        bail!("Invalid community identifier");
    }
    Ok(())
}
