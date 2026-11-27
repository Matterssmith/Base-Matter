/*
 * Base-Matter — versionable Matterssmith control templates independent of command logic.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use crate::menu::{Language, MenuMode, MenuPreferences, MenuViewer, Style};
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
            template: crate::classic_template::expanded_layout("control"),
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
        let default_frame = crate::classic_template::expanded_layout("control");
        let original_menu = crate::classic_template::expanded_layout("legacy-menu");
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
        self.decorate(&crate::classic_template::expanded_layout("legacy-menu"))
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
    vault_config: Option<crate::vault_ipc::VaultClient>,
    require_vault_config: bool,
}

impl ControlThemes {
    pub fn new(fallback: ControlTheme) -> Self {
        Self {
            fallback,
            communities: RwLock::new(HashMap::new()),
            dynamic_menu: None,
            vault_config: None,
            require_vault_config: false,
        }
    }

    pub fn with_dynamic_menu(mut self, preferences: Arc<MenuPreferences>) -> Self {
        self.dynamic_menu = Some(preferences);
        self
    }

    /// Production uses Vault for configuration writes and reads. The local
    /// preferences backend remains available for isolated core tests.
    pub fn with_vault_config(mut self, client: Option<crate::vault_ipc::VaultClient>) -> Self {
        self.vault_config = client;
        self.require_vault_config = true;
        self
    }

    fn config_client(&self) -> Result<&crate::vault_ipc::VaultClient> {
        self.vault_config
            .as_ref()
            .ok_or_else(|| anyhow!("Vault-Matter no disponible"))
    }

    fn style_record(chat: Option<&str>) -> String {
        match chat {
            None => "menu-style-v1".to_owned(),
            Some(chat) => {
                use sha2::{Digest, Sha256};
                format!(
                    "menu-style-chat-{}",
                    hex::encode(Sha256::digest(chat.as_bytes()))
                )
            }
        }
    }

    pub fn scoped_style(
        &self,
        scope: crate::control_scope::CommandScope,
        chat: &str,
    ) -> Result<Style> {
        Ok(self.scoped_style_read(scope, chat)?.0)
    }

    pub fn scoped_style_read(
        &self,
        scope: crate::control_scope::CommandScope,
        chat: &str,
    ) -> Result<(Style, bool)> {
        use crate::control_scope::CommandScope;
        let vault = self.config_client()?;
        let mut offline = false;
        if scope == CommandScope::Chat {
            validate_community_id(chat)?;
            let (value, cached) = vault.config_get_with_source(&Self::style_record(Some(chat)))?;
            offline |= cached;
            if let Some(value) = value {
                if value != "inherit" {
                    return Ok((Style::parse(&value)?, offline));
                }
            }
        }
        if scope != CommandScope::Global {
            let (value, cached) = vault.config_get_with_source(&Self::style_record(None))?;
            offline |= cached;
            if let Some(value) = value {
                if value != "inherit" {
                    return Ok((Style::parse(&value)?, offline));
                }
            }
        }
        let (style, cached) = vault.global_style_read()?;
        Ok((style, offline || cached))
    }

    pub fn write_scoped_style(
        &self,
        scope: crate::control_scope::CommandScope,
        chat: &str,
        principal: &str,
        value: &str,
    ) -> Result<()> {
        use crate::control_scope::CommandScope;
        let vault = self.config_client()?;
        if scope == CommandScope::Global {
            return vault.set_global_style(principal, Style::parse(value)?);
        }
        if value != "inherit" {
            Style::parse(value)?;
        }
        let record = if scope == CommandScope::Chat {
            validate_community_id(chat)?;
            Self::style_record(Some(chat))
        } else {
            Self::style_record(None)
        };
        vault.config_set(&record, value)
    }

    pub fn effective_preferences(&self, chat: Option<&str>) -> Result<crate::menu::Preferences> {
        if !self.require_vault_config {
            return self.current_menu_preferences();
        }
        use crate::control_scope::CommandScope;
        let scope = if chat.is_some() {
            CommandScope::Chat
        } else {
            CommandScope::Socket
        };
        // Presentation falls back to factory defaults during Vault outages.
        // Configuration commands use strict reads and never silently write locally.
        let style = self
            .scoped_style(scope, chat.unwrap_or_default())
            .unwrap_or(Style::Classic);
        let language = self
            .config_client()
            .and_then(|vault| match vault.config_get("menu-language-v1")? {
                Some(value) => Language::parse(&value),
                None => Ok(Language::Es),
            })
            .unwrap_or(Language::Es);
        Ok(crate::menu::Preferences { style, language })
    }

    pub fn decorate_status(&self, chat: Option<&str>, body: &str) -> Result<String> {
        let mut theme = self.selected(chat)?;
        if self.dynamic_menu.is_some() && self.effective_preferences(chat)?.style == Style::None {
            return Self::plain_response(body);
        }
        if theme.template == crate::classic_template::expanded_layout("control") {
            theme.template = crate::classic_template::expanded_layout("status");
        }
        theme.decorate(body)
    }

    fn plain_response(body: &str) -> Result<String> {
        if body.is_empty() || body.len() > 3800 || body.chars().any(|c| c.is_control() && c != '\n')
        {
            bail!("Invalid control response");
        }
        Ok(body.to_owned())
    }

    /// Read the effective local menu settings; no mutation or permissions.
    /// Callers must still be authenticated by the control dispatcher.
    pub fn current_menu_preferences(&self) -> Result<crate::menu::Preferences> {
        if self.require_vault_config {
            let vault = self.config_client()?;
            let style = self.scoped_style(crate::control_scope::CommandScope::Socket, "")?;
            let language = match vault.config_get("menu-language-v1")? {
                Some(value) => Language::parse(&value)?,
                None => Language::Es,
            };
            return Ok(crate::menu::Preferences { style, language });
        }
        self.dynamic_menu
            .as_ref()
            .ok_or_else(|| anyhow!("Dynamic menu not configured"))?
            .get()
    }

    pub fn current_menu_mode(&self) -> Result<MenuMode> {
        if self.require_vault_config {
            return match self.config_client()?.config_get("menu-mode-v1")? {
                Some(value) => MenuMode::parse(&value),
                None => Ok(MenuMode::Web),
            };
        }
        self.dynamic_menu
            .as_ref()
            .ok_or_else(|| anyhow!("Dynamic menu not configured"))?
            .get_mode()
    }

    pub fn set_menu_mode(&self, value: &str) -> Result<()> {
        if self.require_vault_config {
            return self
                .config_client()?
                .config_set("menu-mode-v1", MenuMode::parse(value)?.as_str());
        }
        self.dynamic_menu
            .as_ref()
            .ok_or_else(|| anyhow!("Dynamic menu not configured"))?
            .set_mode(MenuMode::parse(value)?)
    }

    pub fn set_menu_style(&self, value: &str) -> Result<()> {
        if self.require_vault_config {
            return self
                .config_client()?
                .config_set("menu-style-v1", Style::parse(value)?.as_str());
        }
        let preferences = self
            .dynamic_menu
            .as_ref()
            .ok_or_else(|| anyhow!("Dynamic menu not configured"))?;
        preferences.set_style(Style::parse(value)?)
    }

    pub fn set_menu_language(&self, value: &str) -> Result<()> {
        if self.require_vault_config {
            return self
                .config_client()?
                .config_set("menu-language-v1", Language::parse(value)?.as_str());
        }
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
        if self.dynamic_menu.is_some()
            && self.effective_preferences(community_id)?.style == Style::None
        {
            return Self::plain_response(body);
        }
        // Dynamic menus are self-contained. The same verified IPC response
        // passes through this renderer twice; never add another frame.
        let classic_header = crate::classic_template::expand("{{CLASSIC_HEADER}}");
        let classic_emblem = crate::classic_template::expand("{{CLASSIC_EMBLEM}}");
        if self.dynamic_menu.is_some()
            && (body.starts_with(&format!("{classic_header} 𝗖𝗮𝘁𝗲𝗴𝗼𝗿í𝗮𝘀"))
                || body.starts_with(&format!("{classic_header} 𝗔𝘆𝘂𝗱𝗮"))
                || body.starts_with(&format!("{classic_header} Categories"))
                || body.starts_with(&format!("{classic_header} Help"))
                || body.starts_with(&format!("{classic_emblem} Todo"))
                || body.starts_with(&format!("{classic_emblem} All commands"))
                || body.starts_with("╭─💠 𝗖𝗮𝘁𝗲𝗴𝗼𝗿í𝗮𝘀")
                || body.starts_with("╭─💠 𝗔𝘆𝘂𝗱𝗮")
                || body.starts_with("╭─💠 Categories")
                || body.starts_with("╭─💠 Help")
                || body.starts_with("💠 Menú de Base-Matter")
                || body.starts_with("💠 Base-Matter Menu")
                || body.starts_with("📖 Ayuda")
                || body.starts_with("📖 Help")
                || body.starts_with("💠 Todo")
                || body.starts_with("💠 All commands")
                || body.starts_with("💠 Menú completo")
                || body.starts_with("💠 Complete Menu")
                || body.starts_with("💠 Ayuda de Base-Matter")
                || body.starts_with("💠 Base-Matter Help")
                || body.starts_with("🛡️ Protecciones")
                || body.starts_with("꒦꒷✰꒷꒦︶꒦꒷✧")
                || body.starts_with("*꒦꒷✰꒷꒦︶꒦꒷✧")
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

    /// Use only after the WhatsApp bridge authenticates the frame and
    /// admits the read-only menu. Never infer a Vault global rank.
    pub fn render_verified_root(&self, viewer: MenuViewer<'_>) -> Result<String> {
        self.render_verified_chat_root(None, viewer)
    }

    pub fn render_verified_chat_root(
        &self,
        chat: Option<&str>,
        viewer: MenuViewer<'_>,
    ) -> Result<String> {
        crate::menu::render_root_with_viewer(self.effective_preferences(chat)?, viewer)
    }

    pub fn render(&self, community_id: Option<&str>, section: &str) -> Result<String> {
        let theme = self.selected(community_id)?;
        if self.dynamic_menu.is_none() {
            return theme.render(section);
        }
        let preferences = self.effective_preferences(community_id)?;
        let body = if let Some(query) = section.strip_prefix("help") {
            crate::menu::render_help(query, preferences)?
        } else {
            crate::menu::render(section, preferences)?
        };
        if preferences.style == Style::None {
            return Ok(body);
        }
        let default_template = crate::classic_template::expanded_layout("control");
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
