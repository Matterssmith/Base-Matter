/*
 * Base-Matter — versionable Matterssmith control templates independent of command logic.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use crate::menu::{Language, MenuPreferences, Style};
use anyhow::{anyhow, bail, Result};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

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
            template: include_str!("../templates/matterssmith-control.txt").replace("\r\n", "\n"),
        })
    }

    /// The layout lives in a separate UTF-8 file, not in command handlers.
    /// An authorized host may replace it per scope after validation.
    pub fn with_template(mut self, source: &str) -> Result<Self> {
        let normalized = source.replace("\r\n", "\n");
        validate_template(&normalized)?;
        self.template = normalized;
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
        // The original Matterssmith menu is a complete owner-approved
        // WhatsApp message, including its banner and one credit footer.
        // Keep it idempotent; other replies still use their scoped frame.
        let default_frame =
            include_str!("../templates/matterssmith-control.txt").replace("\r\n", "\n");
        let original_menu =
            include_str!("../templates/matterssmith-menu.txt").replace("\r\n", "\n");
        if self.template == default_frame
            && body.trim_end_matches('\n') == original_menu.trim_end_matches('\n')
        {
            return Ok(original_menu.trim_end_matches('\n').to_owned());
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

    /// The original full menu has no categories and is stored in a separate file.
    pub fn render(&self, section: &str) -> Result<String> {
        if section != "menu" {
            bail!("Global control has no menu categories");
        }
        // Git working trees on Windows may use CRLF. Normalize trusted
        // static templates before enforcing WhatsApp's control-char bounds.
        self.decorate(&include_str!("../templates/matterssmith-menu.txt").replace("\r\n", "\n"))
    }
}

/// Validate the common response frame. The standalone owner-approved menu
/// can carry a credit footer; generic status/control frames cannot.
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
    dynamic_menu: Option<Arc<MenuPreferences>>,
}

impl ControlThemes {
    pub fn new(fallback: ControlTheme) -> Self {
        Self {
            fallback,
            communities: RwLock::new(HashMap::new()),
            dynamic_menu: None,
        }
    }

    pub fn with_dynamic_menu(mut self, preferences: Arc<MenuPreferences>) -> Self {
        self.dynamic_menu = Some(preferences);
        self
    }

    pub fn set_menu_style(&self, value: &str) -> Result<()> {
        let preferences = self
            .dynamic_menu
            .as_ref()
            .ok_or_else(|| anyhow!("Dynamic menu not configured"))?;
        preferences.set_style(Style::parse(value)?)
    }

    pub fn set_menu_language(&self, value: &str) -> Result<()> {
        let preferences = self
            .dynamic_menu
            .as_ref()
            .ok_or_else(|| anyhow!("Dynamic menu not configured"))?;
        preferences.set_language(Language::parse(value)?)
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
        // Dynamic menus are self-contained. The same verified IPC response
        // passes through this renderer twice; never add another frame.
        if self.dynamic_menu.is_some()
            && (body.starts_with("💠 Menú de Base-Matter")
                || body.starts_with("💠 Base-Matter Menu")
                || body.starts_with("💠 Menú completo")
                || body.starts_with("💠 Complete Menu")
                || body.starts_with("💠 Ayuda de Base-Matter")
                || body.starts_with("💠 Base-Matter Help")
                || body.starts_with("🛡️ Protecciones")
                || body.starts_with("꒦꒷✰꒷꒦︶꒦꒷✧")
                || body.starts_with("Menú de control global")
                || body.starts_with("Global control menu"))
        {
            if body.is_empty()
                || body.len() > 3800
                || body
                    .chars()
                    .any(|character| character.is_control() && character != '\n')
            {
                bail!("Invalid dynamic menu body");
            }
            return Ok(body.to_owned());
        }
        self.selected(community_id)?.decorate(body)
    }

    pub fn render(&self, community_id: Option<&str>, section: &str) -> Result<String> {
        let theme = self.selected(community_id)?;
        let Some(preferences) = &self.dynamic_menu else {
            return theme.render(section);
        };
        let body = if let Some(query) = section.strip_prefix("help") {
            crate::menu::render_help(query, preferences.get()?)?
        } else {
            crate::menu::render(section, preferences.get()?)?
        };
        let default_template =
            include_str!("../templates/matterssmith-control.txt").replace("\r\n", "\n");
        if theme.template == default_template {
            Ok(body)
        } else {
            theme.decorate(&body)
        }
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
