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
        // This check must precede the idempotent fast path as well:
        // an already-themed payload is not exempt from transport bounds.
        if body.is_empty() || body.len() > 3800 || body.chars().any(|c| c.is_control() && c != '\n')
        {
            bail!("Invalid control response");
        }
        // Repeated rendering of a preformatted menu/preview must be
        // idempotent even for an authorized, non-default theme.
        let (prefix, suffix) = self
            .template
            .split_once("{{BODY}}")
            .ok_or_else(|| anyhow!("Invalid control template"))?;
        let replace_fields = |part: &str| {
            part.replace("{{TITLE}}", &self.title)
                .replace("{{ACCENT}}", &self.accent)
        };
        let rendered_prefix = replace_fields(prefix);
        let rendered_suffix = replace_fields(suffix);
        if body.starts_with(&rendered_prefix)
            && body.trim_end().ends_with(rendered_suffix.trim_end())
        {
            return Ok(body.to_owned());
        }
        if body.len() > 3200 {
            bail!("Control body exceeds template allowance");
        }
        let result = self
            .template
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

/// Validate the user-visible frame. Credits live only in source files
/// and the explicit =/>credits command, never in automatic footers.
/// TITLE and ACCENT are optional customization fields, not mandatory brands.
fn validate_template(source: &str) -> Result<()> {
    if source.is_empty()
        || source.len() > 1500
        || source.chars().any(|c| c.is_control() && c != '\n')
        || source.matches("{{BODY}}").count() != 1
        || source.contains("Créditos:")
        || source.contains("Lournyx Matterssmith")
    {
        bail!("Invalid control template");
    }
    let (prefix, suffix) = source
        .split_once("{{BODY}}")
        .ok_or_else(|| anyhow!("Invalid control template"))?;
    if prefix.trim().is_empty() || suffix.trim().is_empty() {
        bail!("Control frame must enclose the body");
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

    fn selected(&self, community_id: Option<&str>) -> Result<ControlTheme> {
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
        Ok(selected.unwrap_or_else(|| self.fallback.clone()))
    }

    /// One theme selection for every successful command response,
    /// not only the menu. The caller must supply a VERIFIED chat scope.
    pub fn decorate(&self, community_id: Option<&str>, body: &str) -> Result<String> {
        self.selected(community_id)?.decorate(body)
    }

    pub fn render(&self, community_id: Option<&str>, section: &str) -> Result<String> {
        self.selected(community_id)?.render(section)
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
