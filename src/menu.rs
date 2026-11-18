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
pub static CATEGORIES: &[Category] = &[
    status::CATEGORY,
    bot::CATEGORY,
    set::CATEGORY,
    antis::CATEGORY,
    roles::CATEGORY,
    system::CATEGORY,
    sandbox::CATEGORY,
    ai::CATEGORY,
    navigation::CATEGORY,
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
        Self { style: Style::Classic, language: Language::Es }
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
        let db = self.db.lock().map_err(|_| anyhow!("Menu database locked"))?;
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
        let db = self.db.lock().map_err(|_| anyhow!("Menu database locked"))?;
        db.execute(
            "INSERT INTO menu_preferences(singleton,style,language)
             VALUES(1,?1,COALESCE((SELECT language FROM menu_preferences WHERE singleton=1),'es'))
             ON CONFLICT(singleton) DO UPDATE SET style=excluded.style",
            params![style.as_str()],
        )?;
        Ok(())
    }
    pub fn set_language(&self, language: Language) -> Result<()> {
        let db = self.db.lock().map_err(|_| anyhow!("Menu database locked"))?;
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
            format!("│  Prefijo: [ =/> ] · Comandos: {} · Pendientes: {}", command_count(), pending_count()),
            "│".to_owned(),
        ],
        (Style::None, Language::Es) => vec![
            "Menú de control global".to_owned(),
            "Bot: Principal".to_owned(),
            "Sistema: Global".to_owned(),
            "Prefijo: =/>".to_owned(),
            format!("Comandos: {} · Pendientes: {}", command_count(), pending_count()),
            String::new(),
        ],
        (Style::None, Language::En) => vec![
            "Global control menu".to_owned(),
            "Bot: Principal".to_owned(),
            "System: Global".to_owned(),
            "Prefix: =/>".to_owned(),
            format!("Commands: {} · Pending: {}", command_count(), pending_count()),
            String::new(),
        ],
    }
}
pub fn render(section: &str, options: Preferences) -> Result<String> {
    let words: Vec<&str> = section.split_whitespace().collect();
    let (category_id, page) = match words.as_slice() {
        [] | ["menu" | "menú"] => (None, 1_usize),
        ["menu" | "menú", name] | [name] => (Some(*name), 1_usize),
        ["menu" | "menú", name, number] => (
            Some(*name),
            number.parse::<usize>().map_err(|_| anyhow!("Invalid menu page"))?,
        ),
        _ => bail!("Usage: menu [category] [page]"),
    };
    if page == 0 {
        bail!("Menu page must be positive");
    }
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
            out.push(format!("╞═╦═ {} {}", category.icon, label(category, options.language)));
            out.push("│♪║".to_owned());
        } else {
            out.push(label(category, options.language).to_owned());
        }
        if root {
            let item = category.commands.first().ok_or_else(|| anyhow!("Empty menu category"))?;
            match options.style {
                Style::Classic => {
                    out.push(format!("│♪╚[ {} — {}", item.name, description(item, options.language)));
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
                        let branch = if index + 1 == slice.len() { "╚" } else { "╠" };
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
    fn anti_catalog_matches_all_registered_rules() {
        assert_eq!(CATEGORIES.len(), 9);
        assert!(command_count() >= 40);
        assert!(pending_count() > 0);
        assert_eq!(antis::CATEGORY.commands.len(), crate::anti::RULES.len() + 2);
        for rule in crate::anti::RULES {
            assert!(antis::CATEGORY.commands.iter().any(|entry| entry.name == *rule));
        }
    }

    #[test]
    fn classic_menu_has_single_prefix_without_brand_or_credits() {
        let menu = render("menu", Preferences::default()).unwrap();
        assert!(menu.contains("╞═╦═"));
        assert!(menu.contains("│♪╚["));
        assert!(menu.contains("menu antis"));
        assert!(menu.contains("Bot: Principal"));
        assert_eq!(menu.matches("=/>").count(), 1);
        assert!(!menu.contains("Matterssmith"));
        assert!(!menu.contains("Créditos:"));
    }

    #[test]
    fn category_submenus_are_paginated_and_describe_commands() {
        let options = Preferences::default();
        let set = render("menu set", options).unwrap();
        assert!(set.contains("set style classic|none"));
        assert!(set.contains("set lang es|en"));
        assert!(render("menu antis 5", options).unwrap().contains("antiflood"));
        assert!(render("menu antis 9", options).is_err());
        assert!(render("menu unknown", options).is_err());
    }

    #[test]
    fn none_style_is_plain_and_supports_english() {
        let options = Preferences { style: Style::None, language: Language::En };
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
        assert_eq!(store.get().unwrap(), Preferences { style: Style::None, language: Language::En });
        store.set_style(Style::Classic).unwrap();
        assert_eq!(store.get().unwrap().language, Language::En);
    }
}
