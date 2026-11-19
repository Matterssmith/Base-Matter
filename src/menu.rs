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
    antis::antilinkwa::COMMAND,
    antis::antilinktelegram::COMMAND,
    antis::antilinkdiscord::COMMAND,
    antis::antilinkinstagram::COMMAND,
    antis::antilinkfacebook::COMMAND,
    antis::antilinktiktok::COMMAND,
    antis::antilinkyoutube::COMMAND,
    antis::antilinkx::COMMAND,
    antis::antilinkshort::COMMAND,
    antis::antilinkphishing::COMMAND,
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
        commands: &[navigation::menu::COMMAND, ai::help::COMMAND],
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
    for (index, (id, es, en)) in ENTRIES.iter().enumerate() {
        let label = if spanish { es } else { en };
        if classic {
            let branch = if index + 1 == ENTRIES.len() {
                "╚"
            } else {
                "╠"
            };
            lines.push(format!("│·♪·{branch}[ {label} [menu {id}]"));
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
            "│".to_owned(),
            "│ ✦ Todos los comandos utilizan".to_owned(),
            "│   el prefijo global indicado.".to_owned(),
            "│".to_owned(),
            "│ ◉ status funciona incluso".to_owned(),
            "│   cuando el bot está apagado.".to_owned(),
            "│".to_owned(),
            "│ ♛ Los controles administrativos".to_owned(),
            "│   requieren identidad verificada.".to_owned(),
            "│".to_owned(),
            "│ ♧ Los chats externos permanecen".to_owned(),
            "│   desactivados por defecto.".to_owned(),
            "│".to_owned(),
            "│ ◈ Cada socket conserva sus".to_owned(),
            "│   configuraciones independientes.".to_owned(),
            "│".to_owned(),
            "│ ⴵ Los cambios administrativos".to_owned(),
            "│   deben registrarse en auditoría.".to_owned(),
            "│".to_owned(),
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
        lines.push("status funciona aunque el bot esté apagado.".to_owned());
        lines.push("Los controles administrativos requieren identidad verificada.".to_owned());
        lines.push("Los chats externos están desactivados por defecto.".to_owned());
        lines.push("Cada socket mantiene configuración independiente.".to_owned());
        lines.push("Los cambios administrativos deben auditarse.".to_owned());
    }
    let result = lines.join("\n");
    if result.len() > 3800 || result.chars().any(|ch| ch.is_control() && ch != '\n') {
        bail!("Global menu exceeds safe WhatsApp bounds");
    }
    Ok(result)
}

fn verbose_menu(options: Preferences) -> Result<String> {
    let mut lines = vec![
        if options.language == Language::Es {
            "💠 Menú completo"
        } else {
            "💠 Complete Menu"
        }
        .to_owned(),
        "Prefijo: =/>".to_owned(),
    ];
    for cat in CATEGORIES {
        if cat.commands.is_empty() {
            continue;
        }
        lines.push(format!("{}:", cat.id));
        for entry in cat.commands {
            lines.push(format!(
                "  {}{}",
                entry.name,
                if entry.implemented {
                    ""
                } else {
                    " [pendiente]"
                }
            ));
        }
    }
    lines.push("Detalle: help <comando>".to_owned());
    let result = lines.join("\n");
    if result.len() > 3450 {
        bail!("Menú completo demasiado largo");
    }
    Ok(result)
}

fn readable_antis() -> Result<String> {
    Ok([
        "🛡️ Protecciones · Base-Matter",
        "Prefijo: =/>",
        "Filtros desactivados por defecto.",
        "",
        "• antis — ver reglas",
        "• anti <tipo> — consultar o configurar una regla",
        "• antilink — filtro general de enlaces",
        "• antispam — mensajes repetidos",
        "• antiflood — exceso de mensajes",
        "• antimedia — control de archivos",
        "",
        "Detalles: help antilink",
        "Más protecciones: menu antis 2",
        "La lista de enlaces add/del está pendiente.",
    ]
    .join("\n"))
}

pub fn render_help(query: &str, options: Preferences) -> Result<String> {
    let query = query.trim();
    if query.is_empty() {
        return Ok(concise_menu(options)?
            .replacen("╭─💠 𝗖𝗮𝘁𝗲𝗴𝗼𝗿í𝗮𝘀", "╭─💠 𝗔𝘆𝘂𝗱𝗮", 1)
            .replacen("╭─💠 Categories", "╭─💠 Help", 1)
            .replacen("Menú de Base-Matter", "Ayuda de Base-Matter", 1)
            .replacen("Base-Matter Menu", "Base-Matter Help", 1));
    }
    if query == "all" {
        return verbose_menu(options);
    }
    if CATEGORIES
        .iter()
        .any(|cat| cat.id == canonical_category(query))
    {
        return render(&format!("menu {query}"), options);
    }
    if let [category, page] = query.split_whitespace().collect::<Vec<_>>().as_slice() {
        if CATEGORIES
            .iter()
            .any(|cat| cat.id == canonical_category(category))
        {
            return render(&format!("menu {category} {page}"), options);
        }
    }
    let item = CATEGORIES
        .iter()
        .flat_map(|cat| cat.commands.iter())
        .find(|item| item.name == query)
        .ok_or_else(|| anyhow!("Comando no encontrado. Usa help para las categorías"))?;
    let mut lines = vec![
        format!("📖 Ayuda: {}", item.name),
        description(item, options.language).to_owned(),
        if item.implemented {
            "Estado: registrado"
        } else {
            "Estado: pendiente"
        }
        .to_owned(),
        "Uso:".to_owned(),
    ];
    for usage in item.usages {
        lines.push(format!("=/>{usage}"));
    }
    Ok(lines.join("\n"))
}

pub fn render(section: &str, options: Preferences) -> Result<String> {
    match section.trim() {
        "menu" | "menú" => return concise_menu(options),
        "menu all" | "menú all" => return verbose_menu(options),
        "menu anti" | "menú anti" | "menu antis" | "menú antis" => return readable_antis(),
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
        if options.style == Style::Classic {
            out.push(format!(
                "╞═╦═ {} {}",
                category.icon,
                label(category, options.language)
            ));
            out.push("│♪║".to_owned());
        } else {
            out.push(label(category, options.language).to_owned());
        }
        if category.commands.is_empty() {
            // Keep the canonical navigation entry, never label it empty.
            out.push(String::new());
            continue;
        }
        if root {
            let item = category
                .commands
                .first()
                .ok_or_else(|| anyhow!("Empty menu category"))?;
            match options.style {
                Style::Classic => {
                    out.push(format!(
                        "│♪╚[ {} — {}",
                        item.name,
                        description(item, options.language)
                    ));
                    out.push(format!("│♪║  menu {}", category.id));
                }
                Style::None => {
                    out.push(item.name.to_owned());
                    out.push(description(item, options.language).to_owned());
                    out.push(format!("menu {}", category.id));
                }
            }
        } else {
            const PAGE_SIZE: usize = 7;
            let pages = category.commands.len().div_ceil(PAGE_SIZE);
            if page > pages {
                bail!("Menu page does not exist");
            }
            let start = (page - 1) * PAGE_SIZE;
            let slice = &category.commands[start..category.commands.len().min(start + PAGE_SIZE)];
            for (index, command) in slice.iter().enumerate() {
                let state = if command.implemented {
                    ""
                } else if options.language == Language::Es {
                    " [pendiente]"
                } else {
                    " [pending]"
                };
                match options.style {
                    Style::Classic => {
                        let branch = if index + 1 == slice.len() {
                            "╚"
                        } else {
                            "╠"
                        };
                        out.push(format!("│♪{branch}[ {}{state}", command.name));
                        out.push(format!("│♪║  {}", description(command, options.language)));
                        for usage in command.usages {
                            out.push(format!("│♪║  {usage}"));
                        }
                    }
                    Style::None => {
                        out.push(format!("{}{state}", command.name));
                        out.push(description(command, options.language).to_owned());
                        out.extend(command.usages.iter().map(|usage| (*usage).to_owned()));
                    }
                }
            }
            if pages > 1 {
                let next = if page < pages { page + 1 } else { 1 };
                let instruction = format!("menu {} {next} ({page}/{pages})", category.id);
                out.push(match options.style {
                    Style::Classic => format!("│♪║  {instruction}"),
                    Style::None => instruction,
                });
            }
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
        assert_eq!(rows.len(), 21);
        let canonical = [
            "ia",
            "anti",
            "all",
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
        assert!(command_count() >= 40);
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
        assert!(menu.contains("│·♪·╠[ ⴵ ·· 𝙸𝙰 [menu ia]"));
        assert!(menu.contains("│·♪·╚[ ✧ 𝚄𝚝𝚒𝚕𝚒𝚍𝚊𝚍𝚎𝚜 [menu utils]"));
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
        assert!(render("menu antis 5", options)
            .unwrap()
            .contains("antiflood"));
        assert!(render("menu antis 9", options).is_err());
        assert!(render("menu unknown", options).is_err());
        assert!(!render("menu gacha", options)
            .unwrap()
            .contains("Sin comandos registrados"));
        assert!(render("menu anti 5", options)
            .unwrap()
            .contains("antiflood"));
        assert!(render("menu config", options)
            .unwrap()
            .contains("set lang es|en"));
    }

    #[test]
    fn none_style_is_plain_and_supports_english() {
        let options = Preferences {
            style: Style::None,
            language: Language::En,
        };
        let plain = render("menu set", options).unwrap();
        assert!(plain.starts_with("Global control menu"));
        assert!(plain.contains("Change menu style"));
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
