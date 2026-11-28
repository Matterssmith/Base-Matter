// Dynamic command menu: each category has its own directory and one Rust file per command.
// Descriptors are documentation; they never grant execution capabilities.
use anyhow::{anyhow, bail, Result};
use rusqlite::{params, Connection, OptionalExtension};
use std::{fs, path::Path, sync::Mutex, time::Duration};

pub mod ai;
pub mod antis;
pub mod bot;
pub mod navigation;
pub mod roles;
pub mod sandbox;
pub mod set;
pub mod status;
pub mod system;
pub mod telemetry;
pub mod theme;

#[derive(Clone, Copy, Debug)]
pub struct CommandSpec {
    pub name: &'static str,
    pub es: &'static str,
    pub en: &'static str,
    pub usages: &'static [&'static str],
    pub implemented: bool,
}
#[derive(Clone, Copy)]
pub struct Category {
    pub id: &'static str,
    pub es: &'static str,
    pub en: &'static str,
    pub icon: &'static str,
    pub commands: &'static [CommandSpec],
}
// All visible categories come exclusively from docs/CATEGORY_CONTRACT.md.
// "all" aggregates these 20 categories and is never a command bucket.
const ANTI_COMMANDS: &[CommandSpec] = &[
    antis::anti::COMMAND,
    antis::antilink::COMMAND,
    antis::antigroup::COMMAND,
    antis::antichannel::COMMAND,
    antis::antistatus::COMMAND,
    antis::antiviewonce::COMMAND,
    antis::anticall::COMMAND,
    antis::antisticker::COMMAND,
    antis::antiimage::COMMAND,
    antis::antivideo::COMMAND,
    antis::antiaudio::COMMAND,
    antis::antidocument::COMMAND,
    antis::antigif::COMMAND,
    antis::antilocation::COMMAND,
    antis::anticontact::COMMAND,
    antis::antipoll::COMMAND,
    antis::antiforward::COMMAND,
    antis::antispam::COMMAND,
    antis::antiflood::COMMAND,
    antis::antimention::COMMAND,
    antis::antibot::COMMAND,
    antis::antimedia::COMMAND,
    antis::antidelete::COMMAND,
    set::set_anti::COMMAND,
    set::set_antilink::COMMAND,
];

pub static CATEGORIES: &[Category] = &[
    Category {
        id: "ia",
        es: "IA",
        en: "AI",
        icon: "ⴵ",
        commands: &[ai::matter::COMMAND],
    },
    Category {
        id: "anti",
        es: "Anti",
        en: "Anti",
        icon: "☾︎",
        commands: ANTI_COMMANDS,
    },
    Category {
        id: "config",
        es: "Configuración",
        en: "Configuration",
        icon: "⚙︎",
        commands: &[
            set::main_command::COMMAND,
            set::set_style::COMMAND,
            set::set_lang::COMMAND,
            set::set_update::COMMAND,
            set::set_alerts::COMMAND,
        ],
    },
    Category {
        id: "downloads",
        es: "Descargas",
        en: "Downloads",
        icon: "⇩",
        commands: &[],
    },
    Category {
        id: "economy",
        es: "Economía",
        en: "Economy",
        icon: "◈",
        commands: &[],
    },
    Category {
        id: "fun",
        es: "Diversión",
        en: "Fun",
        icon: "♫",
        commands: &[],
    },
    Category {
        id: "gacha",
        es: "Gacha",
        en: "Gacha",
        icon: "✰",
        commands: &[],
    },
    Category {
        id: "games",
        es: "Juegos",
        en: "Games",
        icon: "🎮",
        commands: &[],
    },
    Category {
        id: "group",
        es: "Grupo",
        en: "Group",
        icon: "♧",
        commands: &[],
    },
    Category {
        id: "info",
        es: "Información",
        en: "Information",
        icon: "ⓘ",
        commands: &[
            status::main_command::COMMAND,
            CommandSpec {
                name: "ping",
                es: "Medir respuesta del controlador",
                en: "Measure controller response latency",
                usages: &["ping"],
                implemented: true,
            },
            navigation::credits::COMMAND,
        ],
    },
    Category {
        id: "links",
        es: "Enlaces",
        en: "Links",
        icon: "🔗",
        commands: &[],
    },
    Category {
        id: "main",
        es: "Principal",
        en: "Main",
        icon: "✪",
        commands: &[
            navigation::menu::COMMAND,
            ai::help::COMMAND,
            navigation::register::COMMAND,
        ],
    },
    Category {
        id: "mode",
        es: "Modo",
        en: "Mode",
        icon: "◉",
        commands: bot::CATEGORY.commands,
    },
    Category {
        id: "owner",
        es: "Propietario",
        en: "Owner",
        icon: "♛",
        commands: &[
            roles::main_command::COMMAND,
            roles::audit::COMMAND,
            set::set_role::COMMAND,
        ],
    },
    Category {
        id: "profile",
        es: "Perfil",
        en: "Profile",
        icon: "♙",
        commands: &[],
    },
    Category {
        id: "reactions",
        es: "Reacciones",
        en: "Reactions",
        icon: "♡",
        commands: &[],
    },
    Category {
        id: "search",
        es: "Búsqueda",
        en: "Search",
        icon: "⌕",
        commands: &[],
    },
    Category {
        id: "sockets",
        es: "Conexiones",
        en: "Connections",
        icon: "ⴵ",
        commands: &[],
    },
    Category {
        id: "system",
        es: "Sistema",
        en: "System",
        icon: "⚙︎",
        commands: system::CATEGORY.commands,
    },
    Category {
        id: "utils",
        es: "Utilidades",
        en: "Utilities",
        icon: "✧",
        commands: sandbox::CATEGORY.commands,
    },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Style {
    Classic,
    None,
}
impl Style {
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "matter" => Ok(Self::Classic),
            "none" => Ok(Self::None),
            _ => bail!("Available styles: matter, none"),
        }
    }
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Classic => "matter",
            Self::None => "none",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    Es,
    En,
}
impl Language {
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "es" => Ok(Self::Es),
            "en" => Ok(Self::En),
            _ => bail!("Available languages: es, en"),
        }
    }
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Es => "es",
            Self::En => "en",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Preferences {
    pub style: Style,
    pub language: Language,
}
impl Default for Preferences {
    fn default() -> Self {
        Self {
            style: Style::Classic,
            language: Language::Es,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuMode {
    Web,
    Classic,
    Buttons,
}
impl MenuMode {
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "web" | "modern" => Ok(Self::Web),
            "classic" | "matter" => Ok(Self::Classic),
            "buttons" => Ok(Self::Buttons),
            _ => bail!("Available menu modes: web, classic, buttons"),
        }
    }
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Web => "web",
            Self::Classic => "classic",
            Self::Buttons => "buttons",
        }
    }

    // Preserve persisted SQLite/Vault values for existing installations.
    pub(crate) fn storage_str(self) -> &'static str {
        match self {
            Self::Classic => "matter",
            other => other.as_str(),
        }
    }
}

pub struct MenuPreferences {
    db: Mutex<Connection>,
}
impl MenuPreferences {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if path != Path::new(":memory:") {
            match fs::symlink_metadata(path) {
                Ok(metadata) => {
                    if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
                        bail!("Menu database path must be an ordinary file");
                    }
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        if metadata.permissions().mode() & 0o077 != 0 {
                            bail!("Menu preferences must be owner-only");
                        }
                    }
                }
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                    let mut options = fs::OpenOptions::new();
                    options.write(true).create_new(true);
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::OpenOptionsExt;
                        options.mode(0o600);
                    }
                    options.open(path)?;
                }
                Err(error) => return Err(error.into()),
            }
        }
        let mut db = Connection::open(path)?;
        db.busy_timeout(Duration::from_secs(3))?;
        db.execute_batch(
            "CREATE TABLE IF NOT EXISTS menu_preferences(
                singleton INTEGER PRIMARY KEY CHECK(singleton = 1),
                style TEXT NOT NULL CHECK(style IN ('matter', 'none')),
                language TEXT NOT NULL CHECK(language IN ('es', 'en')),
                mode TEXT NOT NULL DEFAULT 'web' CHECK(mode IN ('web', 'matter', 'buttons'))
            )",
        )?;
        // SQLite CREATE TABLE IF NOT EXISTS does not migrate older stores.
        // Preserve all existing style/language preferences and add only
        // the requested mode with the new default when absent.
        let mut has_mode = false;
        {
            let mut stmt = db.prepare("PRAGMA table_info(menu_preferences)")?;
            let columns = stmt.query_map([], |row| row.get::<_, String>(1))?;
            for column in columns {
                if column? == "mode" {
                    has_mode = true;
                    break;
                }
            }
        }
        if !has_mode {
            db.execute_batch(
                "ALTER TABLE menu_preferences ADD COLUMN mode TEXT NOT NULL DEFAULT 'web' CHECK(mode IN ('web', 'classic'))",
            )?;
        }
        // A previous alpha shipped a two-mode CHECK constraint. SQLite
        // cannot widen a CHECK in place; rebuild just this preferences table
        // in a rollback-safe transaction while preserving the saved values.
        let definition: String = db.query_row(
            "SELECT sql FROM sqlite_master WHERE type='table' AND name='menu_preferences'",
            [],
            |row| row.get(0),
        )?;
        if !definition.contains("'buttons'") || definition.contains("'classic'") {
            let tx = db.transaction()?;
            tx.execute_batch(
                "CREATE TABLE menu_preferences_next(
                    singleton INTEGER PRIMARY KEY CHECK(singleton = 1),
                    style TEXT NOT NULL CHECK(style IN ('matter', 'none')),
                    language TEXT NOT NULL CHECK(language IN ('es', 'en')),
                    mode TEXT NOT NULL DEFAULT 'web' CHECK(mode IN ('web', 'matter', 'buttons'))
                );
                INSERT INTO menu_preferences_next(singleton,style,language,mode)
                    SELECT singleton,
                        CASE style WHEN 'classic' THEN 'matter' ELSE style END,
                        language, CASE mode WHEN 'classic' THEN 'matter' ELSE mode END
                    FROM menu_preferences;
                DROP TABLE menu_preferences;
                ALTER TABLE menu_preferences_next RENAME TO menu_preferences;",
            )?;
            tx.commit()?;
        }
        Ok(Self { db: Mutex::new(db) })
    }
    pub fn get(&self) -> Result<Preferences> {
        let db = self
            .db
            .lock()
            .map_err(|_| anyhow!("Menu database locked"))?;
        let item: Option<(String, String)> = db
            .query_row(
                "SELECT style, language FROM menu_preferences WHERE singleton = 1",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;
        match item {
            Some((style, language)) => Ok(Preferences {
                style: Style::parse(&style)?,
                language: Language::parse(&language)?,
            }),
            None => Ok(Preferences::default()),
        }
    }
    pub fn get_mode(&self) -> Result<MenuMode> {
        let db = self
            .db
            .lock()
            .map_err(|_| anyhow!("Menu database locked"))?;
        let stored: Option<String> = db
            .query_row(
                "SELECT mode FROM menu_preferences WHERE singleton = 1",
                [],
                |row| row.get(0),
            )
            .optional()?;
        match stored {
            Some(mode) => MenuMode::parse(&mode),
            None => Ok(MenuMode::Web),
        }
    }

    pub fn set_mode(&self, mode: MenuMode) -> Result<()> {
        let db = self
            .db
            .lock()
            .map_err(|_| anyhow!("Menu database locked"))?;
        db.execute(
            "INSERT INTO menu_preferences(singleton,style,language,mode)
             VALUES(1,'matter','es',?1)
             ON CONFLICT(singleton) DO UPDATE SET mode=excluded.mode",
            params![mode.storage_str()],
        )?;
        Ok(())
    }

    pub fn set_style(&self, style: Style) -> Result<()> {
        let db = self
            .db
            .lock()
            .map_err(|_| anyhow!("Menu database locked"))?;
        db.execute(
            "INSERT INTO menu_preferences(singleton,style,language)
             VALUES(1,?1,COALESCE((SELECT language FROM menu_preferences WHERE singleton=1),'es'))
             ON CONFLICT(singleton) DO UPDATE SET style=excluded.style",
            params![style.as_str()],
        )?;
        Ok(())
    }
    pub fn set_language(&self, language: Language) -> Result<()> {
        let db = self
            .db
            .lock()
            .map_err(|_| anyhow!("Menu database locked"))?;
        db.execute(
            "INSERT INTO menu_preferences(singleton,style,language)
             VALUES(1,COALESCE((SELECT style FROM menu_preferences WHERE singleton=1),'matter'),?1)
             ON CONFLICT(singleton) DO UPDATE SET language=excluded.language",
            params![language.as_str()],
        )?;
        Ok(())
    }
}
/// Visible menu entries are derived from commands that have an actual implementation.
/// "all" is an aggregate view, never a command category.
pub fn visible_category_ids() -> Vec<&'static str> {
    let mut ids = Vec::new();
    for category in CATEGORIES {
        if category.commands.iter().any(|command| command.implemented) {
            ids.push(category.id);
            if category.id == "anti" {
                ids.push("all");
            }
        }
    }
    ids
}

/// Platform-neutral button descriptors derived from the same command catalog.
/// IDs carry the category identity; the provider adds presentation context.
pub fn button_menu_items(section: &str, language: Language) -> Vec<serde_json::Value> {
    let parts: Vec<_> = section.split_whitespace().collect();
    let root = matches!(
        parts.as_slice(),
        ["=/>menu" | "=/>menú"]
            | [
                "=/>menu" | "=/>menú",
                "classic" | "matter" | "web" | "modern" | "buttons" | "all"
            ]
            | ["=/>menu" | "=/>menú", "page", _]
    );
    if root {
        return visible_category_ids()
            .into_iter()
            .filter(|id| *id != "all" || !section.ends_with(" all"))
            .filter_map(|id| {
                let title = if id == "all" {
                    match language {
                        Language::Es => "Todo",
                        Language::En => "All",
                    }
                } else {
                    label(CATEGORIES.iter().find(|cat| cat.id == id)?, language)
                };
                Some(serde_json::json!({
                    "id": format!("matter.read.buttons.menu.{id}"),
                    "title": title,
                }))
            })
            .collect();
    }
    let (id, page) = match parts.as_slice() {
        ["=/>menu" | "=/>menú", id] => (*id, 1),
        ["=/>menu" | "=/>menú", id, page] => (*id, page.parse::<usize>().unwrap_or(0)),
        _ => return Vec::new(),
    };
    let Some(category) = CATEGORIES
        .iter()
        .find(|cat| cat.id == canonical_category(id))
    else {
        return Vec::new();
    };
    let commands: Vec<_> = category
        .commands
        .iter()
        .filter(|cmd| cmd.implemented)
        .collect();
    const PAGE_SIZE: usize = 7;
    let pages = commands.len().div_ceil(PAGE_SIZE);
    if page == 0 || page > pages {
        return Vec::new();
    }
    let mut items: Vec<_> = commands
        .iter()
        .skip((page - 1) * PAGE_SIZE)
        .take(PAGE_SIZE)
        .map(|cmd| {
            let (id, title) = match cmd.name {
                "status" => ("matter.read.status".to_owned(), "status".to_owned()),
                "ping" => ("matter.read.ping".to_owned(), "ping".to_owned()),
                "menu" => ("matter.read.menu.buttons".to_owned(), "menu".to_owned()),
                "help" => ("matter.read.help".to_owned(), "help".to_owned()),
                _ => (
                    format!("matter.read.help.command.{}", cmd.name.replace(' ', ".")),
                    format!(
                        "{}: {}",
                        match language {
                            Language::Es => "Ayuda",
                            Language::En => "Help",
                        },
                        cmd.name
                    ),
                ),
            };
            serde_json::json!({"id": id, "title": title})
        })
        .collect();
    if page < pages {
        items.push(serde_json::json!({
            "id": format!("matter.read.buttons.menu.{}.{}", category.id, page + 1),
            "title": match language {
                Language::Es => "Siguiente",
                Language::En => "Next",
            },
        }));
    }
    if page > 1 {
        items.push(serde_json::json!({
            "id": format!("matter.read.buttons.menu.{}.{}", category.id, page - 1),
            "title": match language {
                Language::Es => "Anterior",
                Language::En => "Previous",
            },
        }));
    }
    items
}

pub fn command_count() -> usize {
    CATEGORIES
        .iter()
        .flat_map(|category| category.commands.iter())
        .filter(|command| command.implemented)
        .count()
}
pub fn pending_count() -> usize {
    CATEGORIES
        .iter()
        .flat_map(|category| category.commands.iter())
        .filter(|command| !command.implemented)
        .count()
}
fn label(category: &Category, language: Language) -> &'static str {
    match language {
        Language::Es => category.es,
        Language::En => category.en,
    }
}
fn description(command: &CommandSpec, language: Language) -> &'static str {
    match language {
        Language::Es => command.es,
        Language::En => command.en,
    }
}
fn canonical_category(id: &str) -> &str {
    match id {
        "ai" => "ia",
        "antis" => "anti",
        "set" => "config",
        "bot" => "mode",
        "roles" => "owner",
        "status" => "info",
        "navigation" => "main",
        "sandbox" => "utils",
        _ => id,
    }
}

/// Presentation-only fields. They never authorize or imply a Vault global rank.
/// The verified transport decides whether the caller owns this socket.
#[derive(Clone, Copy, Debug)]
pub struct MenuViewer<'a> {
    pub user: &'a str,
    pub role: &'a str,
    pub status: &'a str,
    pub socket: &'a str,
    pub socket_owner: bool,
}

impl MenuViewer<'_> {
    pub fn unspecified() -> Self {
        Self {
            user: "@usuario",
            role: "No verificado",
            status: "Consultar status",
            socket: "Reservado",
            socket_owner: false,
        }
    }
}

fn concise_menu(options: Preferences) -> Result<String> {
    render_root_with_viewer(options, MenuViewer::unspecified())
}

pub fn render_root_with_viewer(options: Preferences, viewer: MenuViewer<'_>) -> Result<String> {
    theme::render_root(options, viewer)
}

fn verbose_menu(options: Preferences) -> Result<String> {
    theme::render_all(options)
}

fn resolve_quoted_help(message: &str) -> Result<&'static str> {
    if message.len() > 1000 || message.chars().any(char::is_control) && message.contains('\r') {
        bail!("La cita no contiene un comando válido");
    }
    for line in message.lines().take(12) {
        // Read only the command's literal first-level syntax; no execution,
        // Unicode rewriting, nested quoting or inferred authority.
        let line = line.trim().trim_start_matches('>').trim();
        let line = line.trim_matches(&['`', '*'][..]);
        let Some(command) = line.strip_prefix("=/>") else {
            continue;
        };
        let command = command.trim();
        if command.is_empty() || command.len() > 144 {
            continue;
        }
        let normalized = if command.starts_with("set antilink")
            && (command == "set antilink" || command.starts_with("set antilink "))
        {
            "set anti link"
        } else {
            command
        };
        if let Some(found) = CATEGORIES
            .iter()
            .flat_map(|category| category.commands.iter())
            .filter(|entry| {
                normalized == entry.name || normalized.starts_with(&format!("{} ", entry.name))
            })
            .max_by_key(|entry| entry.name.len())
        {
            return Ok(found.name);
        }
    }
    bail!("El mensaje citado no contiene un comando Matter reconocido")
}

fn full_help_note(name: &str, language: Language) -> &'static str {
    let spanish = language == Language::Es;
    match (name, spanish) {
        ("set anti link", true) => "Sin argumentos consulta estado y dominios del chat. --mod consulta/configura el socket. --global solo consulta valores de fábrica, sin política Vault compartida. Solo propietario verificado escribe; añadir dominios no activa el filtro.",
        ("set anti link", false) => "No arguments show current chat domains and filter status. --mod selects the authenticated socket. --global reads unconfigured factory defaults only. Owner controls writes.",
        ("anti", true) => "Sin argumentos consulta el chat actual. --mod consulta el socket, --global consulta valores de fábrica (sin Vault compartido). Consulta pública en chats habilitados; solo «set anti» modifica. «anti audit» muestra contadores pasivos; sin moderación automática.",
        ("anti", false) => "«anti» inspects the group, «anti --mod» this socket, «anti --global» the unconfigured global factory baseline. Only «set anti» writes. No automatic moderation.",
        ("anti link", true) => "Protección única de dominios: configura la lista con «set anti link». Actívala aparte; detección de solo lectura.",
        ("anti link", false) => "Single domain filter: configure with «set anti link». Enable separately; detection remains read-only.",
        ("help", true) => "También puedes responder a un mensaje con un comando escribiendo «=/>help». Se interpreta como consulta de ayuda, nunca como ejecución.",
        ("help", false) => "You can also reply to a message containing a command with «=/>help». Quoted commands are never executed.",
        ("register", true) => "Registro voluntario, aún sin alta real en Vault. El acceso invitado a funciones básicas no exige identificación civil.",
        ("register", false) => "Optional enrollment; Vault account creation is not live. Basic guest features require no civil ID.",
        ("set role", true) => "El cambio de rangos requiere Vault-Matter autenticado; actualmente las escrituras de roles permanecen bloqueadas.",
        ("set role", false) => "Role writes require authenticated Vault-Matter and are currently blocked.",
        ("bot", true) => "Sin argumentos consulta el estado actual. --mod consulta los predeterminados del socket. --global muestra el valor general; no acepta escrituras globales sin autoridad. En el chat actual usa on|off|inherit.",
        ("bot", false) => "No arguments show current chat state; --mod inspects socket defaults; --global reads system-level defaults. Global writes require authenticated authority.",
        ("status", true) => "Sin argumentos consulta el chat. --mod muestra el socket al propietario; --global únicamente el valor predeterminado general del proceso, si se conoce.",
        ("status", false) => "No arguments query the chat; --mod shows socket settings to the owner; --global reports any known process-wide fallback.",
        ("set", true) => "Sin argumentos muestra estilo e idioma actuales. --mod se refiere al socket; --global solo informa los valores iniciales.",
        ("set", false) => "No arguments show current style and language. --mod addresses this socket; --global only reports factory values.",
        ("set anti", true) => "Sin argumentos consulta los filtros actuales; --mod consulta o cambia este socket. --global solo consulta los valores de fábrica; no permite escritura.",
        ("set anti", false) => "No arguments inspect anti rules; --mod addresses this socket; --global reads factory defaults only.",
        ("set style", true) => "Consulta el estilo de chat, --mod del socket o --global de Matter.",
        ("set style", false) => "Read chat style, --mod socket style or --global Matter style.",
        ("set lang", true) => "Sin argumentos consulta el idioma actual del socket; --global no permite cambios.",
        ("set lang", false) => "No arguments read the current socket language; no shared global writes.",
        (_, true) => "Esta ficha describe el catálogo; ejecutar acciones depende de activación, permisos y soporte real del proveedor.",
        (_, false) => "This is command metadata; execution depends on activation, authorization and provider support.",
    }
}

fn help_usage(language: Language) -> String {
    let title = if language == Language::Es {
        "📖 Ayuda · Uso"
    } else {
        "📖 Help · Usage"
    };
    [
        title,
        "=/>help <comando>",
        "Responde a un mensaje con un comando escribiendo =/>help",
        "=/>help <categoria>",
        "=/>help <categoria> <pagina>",
        "=/>help all",
    ]
    .join("\n")
}

fn category_help(query: &str, page: usize, options: Preferences) -> Result<String> {
    if page == 0 {
        bail!("Help page must be positive");
    }
    let category = CATEGORIES
        .iter()
        .find(|category| category.id == canonical_category(query))
        .ok_or_else(|| anyhow!("Unknown help category"))?;
    const PAGE_SIZE: usize = 5;
    let count = category.commands.len();
    let pages = count.div_ceil(PAGE_SIZE).max(1);
    if page > pages {
        bail!("Help page does not exist");
    }
    let mut lines = vec![format!("📖 Ayuda: {}", label(category, options.language))];
    let start = (page - 1) * PAGE_SIZE;
    for command in &category.commands[start..count.min(start + PAGE_SIZE)] {
        lines.push(format!("• {}", command.name));
        lines.push(description(command, options.language).to_owned());
        if !command.implemented {
            lines.push(
                if options.language == Language::Es {
                    "No disponible"
                } else {
                    "Not available"
                }
                .to_owned(),
            );
        }
        for usage in command.usages {
            lines.push(format!("=/>{}", usage));
        }
    }
    if pages > 1 {
        lines.push(format!(
            "=/>help {query} {} ({page}/{pages})",
            if page < pages { page + 1 } else { 1 }
        ));
    }
    let output = lines.join("\n");
    if output.len() > 3450 {
        bail!("Help page exceeds safe message size");
    }
    Ok(output)
}

pub fn render_help(query: &str, options: Preferences) -> Result<String> {
    let query = query.trim();
    if query.is_empty() || query == "all" {
        return Ok(help_usage(options.language));
    }
    if let Some(encoded) = query.strip_prefix("--quoted ") {
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
        if encoded.len() > 1500
            || encoded.is_empty()
            || !encoded
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"-_".contains(&byte))
        {
            bail!("Cita inválida para ayuda");
        }
        let data = URL_SAFE_NO_PAD.decode(encoded)?;
        let quoted = std::str::from_utf8(&data)?;
        let command = resolve_quoted_help(quoted)?;
        return render_help(command, options);
    }
    // Exact command metadata wins over a category alias of the same name.
    // For example, 'anti' is a control verb AND the Anti category.
    let is_command = CATEGORIES.iter().any(|category| {
        category
            .commands
            .iter()
            .any(|command| command.name == query)
    });
    if !is_command
        && CATEGORIES
            .iter()
            .any(|category| category.id == canonical_category(query))
    {
        return category_help(query, 1, options);
    }
    if let [category, page] = query.split_whitespace().collect::<Vec<_>>().as_slice() {
        // Only numeric suffixes are pagination; "set role" and
        // "anti audit" are command queries, never category pages.
        if let Ok(number) = page.parse::<usize>() {
            if CATEGORIES
                .iter()
                .any(|entry| entry.id == canonical_category(category))
            {
                return category_help(category, number, options);
            }
        }
    }
    let command = CATEGORIES
        .iter()
        .flat_map(|category| category.commands.iter())
        .filter(|command| command.name == query || query.starts_with(&format!("{} ", command.name)))
        .max_by_key(|command| command.name.len())
        .ok_or_else(|| anyhow!("Comando no encontrado. Usa =/>help"))?;
    let category = CATEGORIES
        .iter()
        .find(|category| {
            category
                .commands
                .iter()
                .any(|item| item.name == command.name)
        })
        .ok_or_else(|| anyhow!("Comando sin categoría"))?;
    let mut lines = vec![
        format!("📖 Ayuda completa: {}", command.name),
        format!("Categoría: {}", label(category, options.language)),
        format!("Descripción: {}", description(command, options.language)),
        format!(
            "Estado: {}",
            if command.implemented {
                "En catálogo"
            } else {
                "Pendiente"
            }
        ),
    ];
    if !command.implemented {
        lines.push(
            if options.language == Language::Es {
                "No disponible"
            } else {
                "Not available"
            }
            .to_owned(),
        );
    }
    lines.push("Uso:".to_owned());
    for usage in command.usages {
        lines.push(format!("`=/>{}`", usage));
    }
    lines.push(String::new());
    lines.push(full_help_note(command.name, options.language).to_owned());
    let output = lines.join("\n");
    if output.len() > 3450 {
        bail!("Ayuda del comando demasiado extensa");
    }
    Ok(output)
}

pub fn render(section: &str, options: Preferences) -> Result<String> {
    match section.trim() {
        "menu" | "menú" => return concise_menu(options),
        "menu all" | "menú all" => return verbose_menu(options),
        // The web mode is a transport choice. A native/HTML-capable provider
        // renders it; the controller supplies a safe textual fallback.
        "menu web" | "menú web" | "menu modern" | "menú modern" => return concise_menu(options),
        "menu buttons" | "menú buttons" => return concise_menu(options),
        // The original category overview, independent from the interactive
        // root and distinct from the verbose "menu all" command.
        "menu classic" | "menú classic" | "menu matter" | "menú matter" => {
            return concise_menu(options)
        }
        "menu page 1" | "menu page 2" | "menu page 3" => return concise_menu(options),
        _ => {}
    }
    let parts: Vec<_> = section.split_whitespace().collect();
    let (category, page) = match parts.as_slice() {
        ["menu" | "menú", name] | [name] => (*name, 1),
        ["menu" | "menú", name, page] => (*name, page.parse::<usize>()?),
        _ => bail!("Usage: menu [category] [page]"),
    };
    if page == 0 {
        bail!("Menu page must be positive");
    }
    let id = canonical_category(category);
    let category = CATEGORIES
        .iter()
        .find(|item| item.id == id)
        .ok_or_else(|| anyhow!("Unknown menu category"))?;
    theme::render_category(category, page, options)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owner_category_contract_is_closed_ordered_and_deduplicated() {
        let allowed = [
            "ia",
            "anti",
            "config",
            "downloads",
            "economy",
            "fun",
            "gacha",
            "games",
            "group",
            "info",
            "links",
            "main",
            "mode",
            "owner",
            "profile",
            "reactions",
            "search",
            "sockets",
            "system",
            "utils",
        ];
        let actual: Vec<&str> = CATEGORIES.iter().map(|category| category.id).collect();
        assert_eq!(actual.as_slice(), allowed.as_slice());
        let mut seen = std::collections::BTreeSet::new();
        for category in CATEGORIES {
            for command in category.commands {
                assert!(
                    seen.insert(command.name),
                    "Duplicate command in menu: {}",
                    command.name
                );
            }
        }
        assert!(seen.contains("help"));
        assert!(seen.contains("matter"));
        assert!(!CATEGORIES.iter().any(|category| category.id == "all"));
        let root = render("menu", Preferences::default()).unwrap();
        let rows: Vec<_> = root
            .lines()
            .filter(|line| line.starts_with("│·♪·"))
            .collect();
        let canonical = [
            "ia", "anti", "all", "config", "info", "main", "mode", "owner", "system", "utils",
        ];
        assert_eq!(rows.len(), canonical.len());
        let configured: Vec<String> =
            serde_json::from_str(include_str!("../templates/menu-visible-categories.json"))
                .unwrap();
        assert_eq!(configured, visible_category_ids());
        for (row, id) in rows.iter().zip(canonical) {
            assert!(
                row.contains(&format!("[menu {id}]")),
                "Wrong category: {id}"
            );
        }
    }

    #[test]
    fn anti_catalog_matches_all_registered_rules() {
        assert_eq!(CATEGORIES.len(), 20);
        assert!(command_count() >= 30);
        assert!(pending_count() > 0);
        assert_eq!(antis::CATEGORY.commands.len(), crate::anti::RULES.len() + 2);
        for rule in crate::anti::RULES {
            let name = match *rule {
                "antiviewonce" => "anti oneview".to_owned(),
                "antivideo" => "anti movie".to_owned(),
                other => format!("anti {}", &other[4..]),
            };
            assert!(antis::CATEGORY
                .commands
                .iter()
                .any(|entry| entry.name == name));
        }
    }

    #[test]
    fn classic_menu_has_single_prefix_project_info_without_credits() {
        let menu = render("menu", Preferences::default()).unwrap();
        assert!(menu.starts_with("*꒦꒷✰꒷꒦︶"));
        assert!(menu.contains("│·♪·╠[ *ⴵ ·· 𝙸𝙰* `[menu ia]`"));
        assert!(menu.contains("│·♪·╚[ *✧ ·· 𝚄𝚝𝚒𝚕𝚒𝚍𝚊𝚍𝚎𝚜* `[menu utils]`"));
        assert!(menu.contains(&crate::classic_template::expand("{{CLASSIC_JOINED_CLOSE}}")));
        assert!(menu.contains("menu all"));
        assert!(menu.contains("menu anti"));
        assert!(menu.contains("menu mode"));
        assert!(menu.matches("=/>").count() >= 1);
        assert!(menu.contains("Proyecto: Matter Ecosystem"));
        assert!(!menu.contains("Aura Reed"));
        assert!(!menu.contains("Créditos:"));
    }

    #[test]
    fn explicit_menu_classic_restores_original_category_overview() {
        let options = Preferences::default();
        let original = render("menu", options).unwrap();
        let classic = render("menu classic", options).unwrap();
        let spanish = render("menú classic", options).unwrap();
        assert_eq!(render("menu matter", options).unwrap(), classic);
        let all = render("menu all", options).unwrap();
        assert_eq!(classic, original);
        assert_eq!(spanish, original);
        assert!(classic.contains("Proyecto: Matter Ecosystem"));
        assert!(classic.contains("[menu ia]"));
        assert!(classic.contains("[menu anti]"));
        assert_ne!(classic, all);
    }

    #[test]
    fn category_submenus_are_paginated_and_describe_commands() {
        let options = Preferences::default();
        let set = render("menu set", options).unwrap();
        assert!(set.contains("set style matter|none"));
        assert!(set.contains("set lang es|en"));
        assert!(render("menu antis 4", options).is_ok());
        assert!(render("menu antis 9", options).is_err());
        assert!(render("menu unknown", options).is_err());
        assert!(!render("menu gacha", options).unwrap().contains("Gacha"));
        assert!(!render("menu", options).unwrap().contains("[menu group]"));
        assert!(render("menu anti 4", options).is_ok());
        assert!(render("menu config", options)
            .unwrap()
            .contains("=/>set lang es|en"));
    }

    #[test]
    fn none_style_is_plain_and_supports_english() {
        let options = Preferences {
            style: Style::None,
            language: Language::En,
        };
        let plain = render("menu set", options).unwrap();
        assert!(plain.starts_with("Global control menu"));
        assert!(plain.contains("=/>set style matter|none"));
        assert!(!plain.contains('╞'));
        assert!(!plain.contains('│'));
        assert!(!plain.contains('♪'));
    }

    #[test]
    fn persisted_preferences_are_independent() {
        let store = MenuPreferences::open(":memory:").unwrap();
        assert_eq!(store.get().unwrap(), Preferences::default());
        assert_eq!(store.get_mode().unwrap(), MenuMode::Web);
        store.set_mode(MenuMode::Classic).unwrap();
        assert_eq!(store.get_mode().unwrap(), MenuMode::Classic);
        assert_eq!(MenuMode::parse("classic").unwrap(), MenuMode::Classic);
        assert_eq!(MenuMode::parse("matter").unwrap(), MenuMode::Classic);
        assert_eq!(MenuMode::Classic.as_str(), "classic");
        assert_eq!(MenuMode::parse("modern").unwrap(), MenuMode::Web);
        store.set_mode(MenuMode::Buttons).unwrap();
        assert_eq!(store.get_mode().unwrap(), MenuMode::Buttons);
        store.set_mode(MenuMode::Web).unwrap();
        assert_eq!(store.get_mode().unwrap(), MenuMode::Web);
        assert!(MenuMode::parse("html").is_err());
        store.set_style(Style::None).unwrap();
        store.set_language(Language::En).unwrap();
        assert_eq!(
            store.get().unwrap(),
            Preferences {
                style: Style::None,
                language: Language::En
            }
        );
        assert_eq!(Style::parse("matter").unwrap(), Style::Classic);
        assert!(Style::parse("classic").is_err());
        store.set_style(Style::Classic).unwrap();
        assert_eq!(store.get().unwrap().language, Language::En);
        assert_eq!(store.get_mode().unwrap(), MenuMode::Web);
    }

    #[test]
    fn web_and_classic_are_explicit_readonly_render_variants() {
        let options = Preferences::default();
        let original = render("menu", options).unwrap();
        assert_eq!(render("menu web", options).unwrap(), original);
        assert_eq!(render("menu modern", options).unwrap(), original);
        assert_eq!(render("menu buttons", options).unwrap(), original);
        assert_eq!(render("menu matter", options).unwrap(), original);
        assert_ne!(render("menu all", options).unwrap(), original);
        assert!(render("menu unknown", options).is_err());
    }

    #[test]
    fn existing_two_mode_schema_is_upgraded_with_preserved_selection() {
        let file = std::env::temp_dir().join(format!(
            "matter-menu-three-modes-{}-{}.sqlite",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        ));
        {
            let conn = Connection::open(&file).unwrap();
            conn.execute_batch(
                "CREATE TABLE menu_preferences(
                    singleton INTEGER PRIMARY KEY CHECK(singleton=1),
                    style TEXT NOT NULL CHECK(style IN ('classic','none')),
                    language TEXT NOT NULL CHECK(language IN ('es','en')),
                    mode TEXT NOT NULL DEFAULT 'web' CHECK(mode IN ('web','classic'))
                );
                INSERT INTO menu_preferences VALUES(1,'none','en','classic');",
            )
            .unwrap();
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o600)).unwrap();
        }
        {
            let prefs = MenuPreferences::open(&file).unwrap();
            assert_eq!(prefs.get_mode().unwrap(), MenuMode::Classic);
            assert_eq!(prefs.get().unwrap().style, Style::None);
            assert_eq!(prefs.get().unwrap().language, Language::En);
            prefs.set_mode(MenuMode::Buttons).unwrap();
        }
        {
            let prefs = MenuPreferences::open(&file).unwrap();
            assert_eq!(prefs.get_mode().unwrap(), MenuMode::Buttons);
            assert_eq!(prefs.get().unwrap().language, Language::En);
        }
        std::fs::remove_file(&file).unwrap();
    }

    #[test]
    fn old_menu_sqlite_schema_is_migrated_without_losing_existing_options() {
        let file = std::env::temp_dir().join(format!(
            "matter-menu-migration-{}-{}.sqlite",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        ));
        {
            let conn = Connection::open(&file).unwrap();
            conn.execute_batch(
                "CREATE TABLE menu_preferences(
                   singleton INTEGER PRIMARY KEY CHECK(singleton=1),
                   style TEXT NOT NULL CHECK(style IN ('classic','none')),
                   language TEXT NOT NULL CHECK(language IN ('es','en'))
                 );
                 INSERT INTO menu_preferences VALUES(1,'none','en');",
            )
            .unwrap();
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o600)).unwrap();
        }
        {
            let prefs = MenuPreferences::open(&file).unwrap();
            assert_eq!(prefs.get_mode().unwrap(), MenuMode::Web);
            assert_eq!(prefs.get().unwrap().style, Style::None);
            assert_eq!(prefs.get().unwrap().language, Language::En);
            prefs.set_mode(MenuMode::Buttons).unwrap();
            prefs.set_language(Language::Es).unwrap();
        }
        {
            let prefs = MenuPreferences::open(&file).unwrap();
            assert_eq!(prefs.get_mode().unwrap(), MenuMode::Buttons);
            assert_eq!(prefs.get().unwrap().style, Style::None);
            assert_eq!(prefs.get().unwrap().language, Language::Es);
        }
        std::fs::remove_file(&file).unwrap();
    }
}
