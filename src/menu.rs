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
    antis::listing_command::COMMAND,
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
        commands: &[status::main_command::COMMAND, navigation::credits::COMMAND],
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
            "classic" | "default" => Ok(Self::Classic),
            "none" => Ok(Self::None),
            _ => bail!("Available styles: classic, none"),
        }
    }
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Classic => "classic",
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
        let db = Connection::open(path)?;
        db.busy_timeout(Duration::from_secs(3))?;
        db.execute_batch(
            "CREATE TABLE IF NOT EXISTS menu_preferences(
                singleton INTEGER PRIMARY KEY CHECK(singleton = 1),
                style TEXT NOT NULL CHECK(style IN ('classic', 'none')),
                language TEXT NOT NULL CHECK(language IN ('es', 'en'))
            )",
        )?;
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
             VALUES(1,COALESCE((SELECT style FROM menu_preferences WHERE singleton=1),'classic'),?1)
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
fn header(options: Preferences) -> Vec<String> {
    match (options.style, options.language) {
        (Style::Classic, _) => vec![
            "꒦꒷✰꒷꒦︶꒦꒷✧꒷꒦︶꒦꒷✰꒷꒦".to_owned(),
            "💬 𝗦𝗶𝘀𝘁𝗲𝗺𝗮 𝗱𝗲 𝗖𝗼𝗻𝘁𝗿𝗼𝗹 𝗚𝗹𝗼𝗯𝗮𝗹 ۞₪»̶⟩".to_owned(),
            "╭─💠 𝗠𝗲𝗻𝘂́ 𝗣𝗿𝗶𝗻𝗰𝗶𝗽𝗮𝗹".to_owned(),
            "│  Bot: Principal · Sistema: Global".to_owned(),
            format!(
                "│  Prefijo: [ =/> ] · Comandos: {} · Pendientes: {}",
                command_count(),
                pending_count()
            ),
            "│".to_owned(),
        ],
        (Style::None, Language::Es) => vec![
            "Menú de control global".to_owned(),
            "Bot: Principal".to_owned(),
            "Sistema: Global".to_owned(),
            "Prefijo: =/>".to_owned(),
            format!(
                "Comandos: {} · Pendientes: {}",
                command_count(),
                pending_count()
            ),
            String::new(),
        ],
        (Style::None, Language::En) => vec![
            "Global control menu".to_owned(),
            "Bot: Principal".to_owned(),
            "System: Global".to_owned(),
            "Prefix: =/>".to_owned(),
            format!(
                "Commands: {} · Pending: {}",
                command_count(),
                pending_count()
            ),
            String::new(),
        ],
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
    // Owner-provided visual layout. Never invent unregistered categories.
    const ENTRIES: [(&str, &str, &str); 21] = [
        ("ia", "ⴵ ·· 𝙸𝙰", "ⴵ ·· IA"),
        ("anti", "☾︎ ··· 𝙰𝚗𝚝𝚒", "☾︎ ··· Anti"),
        ("all", "✦ · 𝚃𝚘𝚍𝚘", "✦ · All"),
        ("config", "⚙︎ · 𝙲𝚘𝚗𝚏𝚒𝚐𝚞𝚛𝚊𝚌𝚒ó𝚗", "⚙︎ · Config"),
        ("downloads", "⇩ · 𝙳𝚎𝚜𝚌𝚊𝚛𝚐𝚊𝚜", "⇩ · Downloads"),
        ("economy", "◈ · 𝙴𝚌𝚘𝚗𝚘𝚖í𝚊", "◈ · Economy"),
        ("fun", "♫ · 𝙳𝚒𝚟𝚎𝚛𝚜𝚒ó𝚗", "♫ · Fun"),
        ("gacha", "✰ · 𝙶𝚊𝚌𝚑𝚊", "✰ · Gacha"),
        ("games", "🎮 𝙹𝚞𝚎𝚐𝚘𝚜", "🎮 · Games"),
        ("group", "♧ 𝙶𝚛𝚞𝚙𝚘", "♧ · Group"),
        ("info", "ⓘ 𝙸𝚗𝚏𝚘𝚛𝚖𝚊𝚌𝚒ó𝚗", "ⓘ · Information"),
        ("links", "🔗 𝙴𝚗𝚕𝚊𝚌𝚎𝚜", "🔗 · Links"),
        ("main", "✪ ·· 𝙿𝚛𝚒𝚗𝚌𝚒𝚙𝚊𝚕", "✪ ·· Main"),
        ("mode", "◉ · 𝙼𝚘𝚍𝚘", "◉ · Mode"),
        ("owner", "♛ · 𝙿𝚛𝚘𝚙𝚒𝚎𝚝𝚊𝚛𝚒𝚘", "♛ · Owner"),
        ("profile", "♙ · 𝙿𝚎𝚛𝚏𝚒𝚕", "♙ · Profile"),
        ("reactions", "♡ 𝚁𝚎𝚊𝚌𝚌𝚒𝚘𝚗𝚎𝚜", "♡ · Reactions"),
        ("search", "⌕ · 𝙱ú𝚜𝚚𝚞𝚎𝚍𝚊", "⌕ · Search"),
        ("sockets", "ⴵ · 𝙲𝚘𝚗𝚎𝚡𝚒𝚘𝚗𝚎𝚜", "ⴵ · Sockets"),
        ("system", "⚙︎ 𝚂𝚒𝚜𝚝𝚎𝚖𝚊", "⚙︎ System"),
        ("utils", "✧ 𝚄𝚝𝚒𝚕𝚒𝚍𝚊𝚍𝚎𝚜", "✧ · Utilities"),
    ];
    if ENTRIES
        .iter()
        .any(|(id, _, _)| *id != "all" && !CATEGORIES.iter().any(|category| category.id == *id))
    {
        bail!("Menu presentation contains unregistered categories");
    }
    let spanish = options.language == Language::Es;
    let classic = options.style == Style::Classic;
    let mut lines = if classic {
        vec![
            "*꒦꒷✰꒷꒦︶꒦꒷✧꒷꒦︶꒦꒷✰꒷꒦*".to_owned(),
            String::new(),
            "💬 𝗦𝗶𝘀𝘁𝗲𝗺𝗮 𝗱𝗲 𝗖𝗼𝗻𝘁𝗿𝗼𝗹 𝗚𝗹𝗼𝗯𝗮𝗹 ۞₪»̶⟩".to_owned(),
            String::new(),
            if spanish {
                "╭─💠 𝗖𝗮𝘁𝗲𝗴𝗼𝗿í𝗮𝘀".to_owned()
            } else {
                "╭─💠 Categories".to_owned()
            },
        ]
    } else {
        vec![if spanish {
            "Menú de control global".to_owned()
        } else {
            "Global control menu".to_owned()
        }]
    };
    let visible_ids = visible_category_ids();
    let visible_entries: Vec<_> = ENTRIES
        .iter()
        .filter(|(id, _, _)| visible_ids.contains(id))
        .collect();
    for (index, (id, es, en)) in visible_entries.iter().enumerate() {
        let decorated_label = if spanish { es } else { en };
        if classic {
            let branch = if index + 1 == visible_entries.len() {
                "╚"
            } else {
                "╠"
            };
            lines.push(format!("│·♪·{branch}[ *{decorated_label}* `[menu {id}]`"));
        } else {
            let plain_label = if *id == "all" {
                if spanish {
                    "Todo"
                } else {
                    "All"
                }
            } else {
                CATEGORIES
                    .iter()
                    .find(|category| category.id == *id)
                    .map(|category| label(category, options.language))
                    .ok_or_else(|| anyhow!("Unknown canonical category"))?
            };
            lines.push(format!("{plain_label}: menu {id}"));
        }
    }
    if classic {
        lines.extend(["╰─╮╭────────⟨🌀".to_owned(), "       ││".to_owned()]);
        if viewer.socket_owner {
            // Role description only. The viewer's actual verified rank
            // is printed separately; global ownership requires Vault proof.
            lines.extend([
                "╭─💠 𝗔𝘂𝘁𝗼𝗿𝗶𝗱𝗮𝗱 𝗚𝗹𝗼𝗯𝗮𝗹".to_owned(),
                "│".to_owned(),
                "│ ♛ Owner Global".to_owned(),
                "│    Prioridad máxima.".to_owned(),
                "│".to_owned(),
                "│ ◈ Jerarquía: Superior".to_owned(),
                "│ ⚙︎ Alcance: Ecosistema Matter".to_owned(),
                "│ ✦ Bot principal: Administración global".to_owned(),
                "│ ⴵ Cambios: Control y auditoría".to_owned(),
                "│ ♧ Seguridad: Identidad verificada".to_owned(),
                "│".to_owned(),
                "│ ♛ 𝗣𝗲𝗿𝗺𝗶𝘀𝗼𝘀".to_owned(),
                "│".to_owned(),
                "│ · Controlar el bot principal.".to_owned(),
                "│ · Consultar configuración global.".to_owned(),
                "│ · Administrar políticas autorizadas.".to_owned(),
                "│ · Supervisar sockets y servicios.".to_owned(),
                "│ · Gestionar jerarquías y permisos.".to_owned(),
                "│ · Prioridad sobre rangos inferiores.".to_owned(),
                "│ · Auditar cambios y operaciones.".to_owned(),
                "│".to_owned(),
                "╰───────────⟨ 🌀".to_owned(),
                String::new(),
            ]);
        }
        lines.extend([
            format!("✧☽︎╦[ 𝐔𝐬𝐮𝐚𝐫𝐢𝐨: *{}* 🌀 ]═☾︎✧", viewer.user),
            "     ╠[ ◈ 𝐁𝐨𝐭: *Principal*".to_owned(),
            "     ╠[ ⚙︎ 𝐒𝐢𝐬𝐭𝐞𝐦𝐚: *Global*".to_owned(),
            "     ╠[ ✦ 𝐏𝐫𝐞𝐟𝐢𝐣𝐨: *[ =/> ]*".to_owned(),
            format!("     ╠[ ☷ 𝐂𝐨𝐦𝐚𝐧𝐝𝐨𝐬: *{}*", command_count()),
            format!("     ╠[ ♛ 𝐑𝐚𝐧𝐠𝐨: *{}*", viewer.role),
            format!("     ╠[ ◉ 𝐄𝐬𝐭𝐚𝐝𝐨: *{}*", viewer.status),
            format!("✧☽︎╩[ ♧ 𝐒𝐨𝐜𝐤𝐞𝐭: *{}*", viewer.socket),
            String::new(),
            "╭─💠 𝗜𝗻𝘀𝘁𝗿𝘂𝗰𝗰𝗶𝗼𝗻𝗲𝘀".to_owned(),
        ]);
        // WhatsApp quotes distinguish operational notes from ornamental lines.
        // Keep every line short for narrow Android screens.
        for instruction in [
            "◉ `status` incluso con bot OFF.",
            "♛ Admin: identidad verificada.",
            "♧ Externos: OFF por defecto.",
            "◈ Sockets aislados · cambios auditados.",
        ] {
            lines.push(crate::message_typography::format_segment(
                instruction,
                crate::message_typography::TextFormat::Quote,
            )?);
        }
        lines.extend([
            "╰───────────⟨ 🌀".to_owned(),
            String::new(),
            "*꒦꒷✰꒷꒦︶꒦꒷✧꒷꒦︶꒦꒷✰꒷꒦*".to_owned(),
        ]);
    } else {
        lines.push(String::new());
        lines.push(format!("Usuario: {}", viewer.user));
        lines.push("Bot: Principal".to_owned());
        lines.push("Sistema: Global".to_owned());
        lines.push("Prefijo: =/>".to_owned());
        lines.push(format!("Comandos: {}", command_count()));
        lines.push(format!("Rango: {}", viewer.role));
        lines.push(format!("Estado: {}", viewer.status));
        lines.push(format!("Socket: {}", viewer.socket));
        lines.push(String::new());
        lines.push("Instrucciones:".to_owned());
        lines.push("status incluso con bot OFF.".to_owned());
        lines.push("Admin: identidad verificada.".to_owned());
        lines.push("Externos: OFF por defecto.".to_owned());
        lines.push("Sockets aislados; cambios auditados.".to_owned());
    }
    let result = lines.join("\n");
    if result.len() > 3800 || result.chars().any(|ch| ch.is_control() && ch != '\n') {
        bail!("Global menu exceeds safe WhatsApp bounds");
    }
    Ok(result)
}

fn verbose_menu(options: Preferences) -> Result<String> {
    let mut lines = vec![if options.language == Language::Es {
        "💠 Todo"
    } else {
        "💠 All commands"
    }
    .to_owned()];
    for category in CATEGORIES {
        let commands: Vec<_> = category
            .commands
            .iter()
            .filter(|command| command.implemented)
            .collect();
        if commands.is_empty() {
            continue;
        }
        lines.push(format!("*{}*:", label(category, options.language)));
        for command in commands {
            lines.push(format!("  `=/>{}`", command.name));
        }
    }
    let output = lines.join("\n");
    if output.len() > 3450 {
        bail!("Full command index exceeds safe message size");
    }
    Ok(output)
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
        ("set anti link", true) => "Ámbito: chat actual; --global aplica al socket. Solo propietario verificado. Los dominios se agregan a una lista, sin activar la moderación. Revisa la lista con «list».",
        ("set anti link", false) => "Scope: current chat; --global selects the socket. Verified owner only. Adding domains does not enable moderation.",
        ("anti", true) => "Los filtros están apagados inicialmente. «anti audit» muestra ediciones y camuflajes observados. La detección no elimina ni expulsa.",
        ("anti", false) => "All filters initially OFF. «anti audit» reports edits and disguises; no automatic moderation.",
        ("antilink", true) => "Protección única de dominios: configura la lista con «set anti link». Actívala aparte; detección de solo lectura.",
        ("antilink", false) => "Single domain filter: configure with «set anti link». Enable separately; detection remains read-only.",
        ("help", true) => "También puedes responder a un mensaje con un comando escribiendo «=/>help». Se interpreta como consulta de ayuda, nunca como ejecución.",
        ("help", false) => "You can also reply to a message containing a command with «=/>help». Quoted commands are never executed.",
        ("register", true) => "Registro voluntario, aún sin alta real en Vault. El acceso invitado a funciones básicas no exige identificación civil.",
        ("register", false) => "Optional enrollment; Vault account creation is not live. Basic guest features require no civil ID.",
        ("set role", true) => "El cambio de rangos requiere Vault-Matter autenticado; actualmente las escrituras de roles permanecen bloqueadas.",
        ("set role", false) => "Role writes require authenticated Vault-Matter and are currently blocked.",
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
        .filter(|command| {
            command.name == query || query.starts_with(&format!("{} ", command.name))
        })
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
        "menu page 1" | "menu page 2" | "menu page 3" => return concise_menu(options),
        _ => {}
    }

    let words: Vec<&str> = section.split_whitespace().collect();
    let (category_id, page) = match words.as_slice() {
        [] | ["menu" | "menú"] => (None, 1_usize),
        ["menu" | "menú", name] | [name] => (Some(*name), 1_usize),
        ["menu" | "menú", name, number] => (
            Some(*name),
            number
                .parse::<usize>()
                .map_err(|_| anyhow!("Invalid menu page"))?,
        ),
        _ => bail!("Usage: menu [category] [page]"),
    };
    if page == 0 {
        bail!("Menu page must be positive");
    }
    let category_id = category_id.map(canonical_category);
    let root = category_id.is_none();
    let shown: Vec<&Category> = if let Some(id) = category_id {
        vec![CATEGORIES
            .iter()
            .find(|category| category.id == id)
            .ok_or_else(|| anyhow!("Unknown menu category"))?]
    } else {
        CATEGORIES.iter().collect()
    };
    let mut out = header(options);
    for category in shown {
        let visible: Vec<_> = category
            .commands
            .iter()
            .filter(|command| command.implemented)
            .collect();
        if visible.is_empty() {
            continue;
        }
        if options.style == Style::Classic {
            out.push(format!(
                "╞═╦═ {} *{}*",
                category.icon,
                label(category, options.language)
            ));
        } else {
            out.push(label(category, options.language).to_owned());
        }
        if root {
            out.push(format!("menu {}", category.id));
            continue;
        }
        const PAGE_SIZE: usize = 7;
        let pages = visible.len().div_ceil(PAGE_SIZE);
        if page > pages {
            bail!("Menu page does not exist");
        }
        let start = (page - 1) * PAGE_SIZE;
        let current = &visible[start..visible.len().min(start + PAGE_SIZE)];
        let usages: Vec<_> = current
            .iter()
            .flat_map(|command| command.usages.iter().copied())
            .collect();
        for (index, usage) in usages.iter().enumerate() {
            if options.style == Style::Classic {
                let branch = if index + 1 == usages.len() {
                    "╚"
                } else {
                    "╠"
                };
                out.push(format!("│♪{branch}[ `=/>{usage}`"));
            } else {
                out.push(format!("=/>{usage}"));
            }
        }
        if pages > 1 {
            let next = if page < pages { page + 1 } else { 1 };
            out.push(format!("menu {} {next} ({page}/{pages})", category.id));
        }
        out.push(String::new());
    }
    if options.style == Style::Classic {
        out.push("╰─╮╭────────⟨🌀".to_owned());
        out.push("       ││".to_owned());
    }
    let output = out.join("\n");
    if output.len() > 3450 || output.chars().any(|ch| ch.is_control() && ch != '\n') {
        bail!("Menu exceeds safe message size");
    }
    Ok(output)
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
            "ia", "anti", "all", "config", "info", "main", "mode", "owner", "utils",
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
            assert!(antis::CATEGORY
                .commands
                .iter()
                .any(|entry| entry.name == *rule));
        }
    }

    #[test]
    fn classic_menu_has_single_prefix_without_brand_or_credits() {
        let menu = render("menu", Preferences::default()).unwrap();
        assert!(menu.starts_with("*꒦꒷✰꒷꒦︶"));
        assert!(menu.contains("│·♪·╠[ *ⴵ ·· 𝙸𝙰* `[menu ia]`"));
        assert!(menu.contains("│·♪·╚[ *✧ 𝚄𝚝𝚒𝚕𝚒𝚍𝚊𝚍𝚎𝚜* `[menu utils]`"));
        assert!(menu.contains("╰─╮╭────────⟨🌀"));
        assert!(menu.contains("menu all"));
        assert!(menu.contains("menu anti"));
        assert!(menu.contains("menu mode"));
        assert!(menu.matches("=/>").count() >= 1);
        assert!(!menu.contains("Matterssmith"));
        assert!(!menu.contains("Créditos:"));
    }

    #[test]
    fn category_submenus_are_paginated_and_describe_commands() {
        let options = Preferences::default();
        let set = render("menu set", options).unwrap();
        assert!(set.contains("set style classic|none"));
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
        assert!(plain.contains("=/>set style classic|none"));
        assert!(!plain.contains('╞'));
        assert!(!plain.contains('│'));
        assert!(!plain.contains('♪'));
    }

    #[test]
    fn persisted_preferences_are_independent() {
        let store = MenuPreferences::open(":memory:").unwrap();
        assert_eq!(store.get().unwrap(), Preferences::default());
        store.set_style(Style::None).unwrap();
        store.set_language(Language::En).unwrap();
        assert_eq!(
            store.get().unwrap(),
            Preferences {
                style: Style::None,
                language: Language::En
            }
        );
        store.set_style(Style::Classic).unwrap();
        assert_eq!(store.get().unwrap().language, Language::En);
    }
}
