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

    /// A distinct Matterssmith default, adapted to an ornamental category
    /// menu; neither copied bot commands nor third-party links are imported.
    /// Only implemented commands are shown as active controls.
    pub fn render(&self, section: &str) -> Result<String> {
        let category = if section == "menu" {
            None
        } else {
            Some(
                section
                    .strip_prefix("menu ")
                    .ok_or_else(|| anyhow!("Unsupported control menu section"))?,
            )
        };
        let mut output = format!(
            "*꒦꒷✰꒷꒦︶꒦꒷✧꒷꒦︶꒦꒷✰꒷꒦*\\n꧁✪⃢.✪ 𝐌𝐚𝐭𝐭𝐞𝐫𝐬𝐬𝐦𝐢𝐭𝐡 ۞₪»̶⟩\\n*{}* · {}\\nBot ID: {}\\n",
            self.title, self.accent, self.bot_id
        );
        if category.is_none() {
            output.push_str(
                "💬 Selecciona una categoría de Base-Matter.\\n\\n╭─💠 𝗖𝗮𝘁𝗲𝗴𝗼𝗿í𝗮𝘀\\n"
            );
            for (symbol, label, slug) in [
                ("✪", "Principal", "main"),
                ("⚙︎", "Configuración", "config"),
                ("♧", "Grupo", "group"),
                ("◉", "Modo", "mode"),
                ("ⴵ", "Conexiones", "sockets"),
                ("♛", "Propietario", "owner"),
                ("✦", "Estilo", "style"),
                ("◈", "Vault", "vault"),
                ("ⓘ", "Información", "info"),
                ("⚙︎", "Sistema", "system"),
                ("✧", "Todo lo disponible", "all"),
            ] {
                output.push_str(&format!(
                    "│·♪·╠[ {symbol} · {label} [=/>menu {slug}]\\n"
                ));
            }
            output.push_str(
                "╰───────────⟨ 🌀\\n\\n💠 Instrucciones: =/>menu <categoría>\\n"
            );
            output.push_str(
                "El menú se muestra en chats habilitados. =/>status siempre informa el estado.\\n"
            );
        } else {
            let entries: &[(&str, &str)] = match category.unwrap_or_default() {
                "main" | "principal" => &[
                    ("=/>status", "Estado efectivo del chat"),
                    ("=/>status --global", "Política base del socket"),
                    ("=/>bot", "Uso de la activación"),
                    ("=/>menu", "Categorías"),
                ],
                "config" | "configuracion" | "configuración" => &[
                    ("=/>set", "Opciones de configuración"),
                    ("=/>bot --global on/off", "Predeterminado del socket [privado]"),
                    ("=/>bot group/private/channel on/off", "Por tipo [privado]"),
                    ("=/>set role", "Vault-Matter [aún no conectado]"),
                ],
                "group" | "grupo" => &[
                    ("=/>status", "Estado de este grupo"),
                    ("=/>bot on/off/inherit", "Excepción de este grupo"),
                    ("=/>group status", "Consulta grupal compatible"),
                    ("=/>group on/off/inherit", "Política grupal compatible"),
                    ("=/>start", "REGISTER [lectura]"),
                    ("=/>add++", "Recuperación [vista previa]"),
                ],
                "mode" | "modo" => &[
                    ("=/>bot --global on/off/inherit", "Base del socket [privado]"),
                    ("=/>bot group/private/channel on/off", "Tipos [privado]"),
                    ("=/>bot on/off/inherit", "Chat actual"),
                    ("=/>status", "Consultar sin activar el bot"),
                ],
                "sockets" | "conexiones" => &[
                    ("=/>on / =/>off", "Energía del socket [privado]"),
                    ("=/>status --global", "Estado base del socket"),
                    ("=/>audit", "Auditoría [privado]"),
                ],
                "owner" | "propietario" => &[
                    ("=/>on / =/>off", "Energía [solo propietario]"),
                    ("=/>roles", "Catálogo de rangos [privado]"),
                    ("=/>set role", "Vault requerido"),
                ],
                "style" | "estilo" => &[
                    ("=/>menu", "Diseño Matterssmith predeterminado"),
                    ("=/>set style", "Editor persistente [pendiente]"),
                    ("=/>set style --auto", "IA supervisada [pendiente]"),
                ],
                "vault" => &[
                    ("=/>roles", "Estado de los rangos [privado]"),
                    ("=/>set role", "Mutaciones bloqueadas sin Vault"),
                ],
                "info" | "informacion" | "información" => &[
                    ("=/>status", "Estado del chat"),
                    ("=/>menu main", "Comandos implementados"),
                ],
                "system" | "sistema" => &[
                    ("=/>status --global", "Política base"),
                    ("=/>audit", "Auditoría [privado]"),
                    ("=/>set", "Configuración"),
                ],
                "all" | "todo" => &[
                    ("=/>status", "Cualquier contexto conectado"),
                    ("=/>bot", "Activación y herencia"),
                    ("=/>menu <categoría>", "Menú por sección"),
                    ("=/>set", "Configuración"),
                    ("=/>roles", "Sin escritura local"),
                    ("=/>group status/on/off/inherit", "Compatibilidad grupal"),
                    ("=/>start / =/>add / =/>add++", "Lecturas y vistas previas"),
                ],
                _ => bail!("Unknown menu category"),
            };
            output.push_str(&format!("\\n╭─💠 {}\\n", category.unwrap_or_default()));
            for (command, purpose) in entries {
                output.push_str(&format!("│·♪·╠ {command} — {purpose}\\n"));
            }
            output.push_str("╰───────────⟨ 🌀\\n=/>menu · regresar a categorías\\n");
        }
        output.push_str("Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].");
        if output.len() > 3500 {
            bail!("Rendered menu exceeds secure transport limit");
        }
        Ok(output)
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
