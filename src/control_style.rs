/*
 * Base-Matter — reusable Matterssmith category menu and scoped visual styling.
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
            if value.is_empty() || value.len() > 80 || value.chars().any(char::is_control) {
                bail!("Invalid control theme");
            }
        }
        Ok(Self {
            title: title.to_owned(),
            bot_id: bot_id.to_owned(),
            accent: accent.to_owned(),
        })
    }

    /// Shared visual shell for every control response.
    /// Uses actual line breaks, never the literal characters backslash+n.
    /// Idempotent so the menu can pass through the IPC formatter unchanged.
    pub fn decorate(&self, body: &str) -> Result<String> {
        const HEADER: &str = "*꒦꒷✰꒷꒦︶꒦꒷✧꒷꒦︶꒦꒷✰꒷꒦*";
        if body.starts_with(HEADER) {
            return Ok(body.to_owned());
        }
        if body.is_empty()
            || body.len() > 3500
            || body.chars().any(|c| c.is_control() && c != '\n')
        {
            bail!("Invalid control response");
        }
        let result = format!(
            "{HEADER}\n꧁✪⃢.✪ 𝐌𝐚𝐭𝐭𝐞𝐫𝐬𝐬𝐦𝐢𝐭𝐡 ۞₪»̶⟩\n*{}* · {}\n\n╭─💠 𝗖𝗼𝗻𝘁𝗿𝗼𝗹\n{}\n╰───────────⟨ 🌀\n\nCréditos: Hecho por Lournyx Matterssmith [Lou-Nyx].",
            self.title, self.accent, body
        );
        if result.len() > 3800 {
            bail!("Themed control reply exceeds transport limit");
        }
        Ok(result)
    }

    /// One compact global control menu. The example menu's decoration is
    /// reused, not its category taxonomy, external links or bot commands.
    pub fn render(&self, section: &str) -> Result<String> {
        if section != "menu" {
            bail!("Global control has no menu categories");
        }
        let body = concat!(
            "💬 𝗖𝗼𝗺𝗮𝗻𝗱𝗼𝘀 𝗱𝗲 𝗰𝗼𝗻𝘁𝗿𝗼𝗹\n",
            "╭─💠 𝗕𝗮𝘀𝗲-𝗠𝗮𝘁𝘁𝗲𝗿\n",
            "│·♪·╠[ ◉ =/>status — Estado de este chat\n",
            "│·♪·╠[ ⓘ =/>status --global — Configuración general [propietario]\n",
            "│·♪·╠[ ✦ =/>on / =/>off — Bot en este chat [propietario]\n",
            "│·♪·╠[ ◉ =/>bot — Información de activación\n",
            "│·♪·╠[ ♧ =/>bot inherit — Heredar política [propietario]\n",
            "│·♪·╠[ ⚙︎ =/>bot --global on/off — Base del socket [propietario]\n",
            "│·♪·╠[ ✧ =/>bot group/private/channel on/off — Tipos [propietario]\n",
            "│·♪·╠[ ♛ =/>set — Configuraciones [propietario]\n",
            "│·♪·╠[ ◈ =/>roles — Vault-Matter [solo consulta]\n",
            "│·♪·╚[ ⴵ =/>audit — Auditoría [propietario]\n",
            "╰───────────⟨ 🌀\n",
            "📌 El menú se muestra si este chat tiene el bot activado.\n",
            "📌 =/>status responde incluso si está apagado."
        );
        self.decorate(body)
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
