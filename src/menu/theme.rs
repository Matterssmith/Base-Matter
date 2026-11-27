/*
 * Matter menu presentation engine: HTML-like separation of catalog,
 * runtime metrics and visual templates. No frames or labels in Rust.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use super::{
    command_count, label, visible_category_ids, Category, Language, MenuViewer, Preferences, Style,
    CATEGORIES,
};
use crate::message_typography::font_text;
use anyhow::{anyhow, bail, Result};
use serde::Deserialize;
use std::collections::BTreeMap;

type Vars = BTreeMap<&'static str, String>;

#[derive(Deserialize)]
struct Identity {
    project: String,
    owner: String,
    bot: String,
    prefix: String,
}
#[derive(Deserialize)]
struct Fonts {
    header: String,
    category: String,
    command: String,
}
#[derive(Deserialize)]
struct Decoration {
    branch: String,
    last_branch: String,
    all_icon: String,
}
#[derive(Deserialize)]
struct Labels {
    user: String,
    bot: String,
    project: String,
    owner: String,
    prefix: String,
    commands: String,
    categories_active: String,
    version: String,
    date: String,
    role: String,
    status: String,
    socket: String,
    sub_bots: String,
    premium_bots: String,
    url: String,
    host: String,
    os: String,
    uptime: String,
    server_ram: String,
    controller_ram: String,
    provider_ram: String,
}
#[derive(Deserialize)]
struct Locale {
    title: String,
    root_title: String,
    categories: String,
    information: String,
    system: String,
    instructions: String,
    all: String,
    unavailable: String,
    labels: Labels,
    notes: Vec<String>,
}
#[derive(Deserialize)]
struct Theme {
    identity: Identity,
    fonts: Fonts,
    decoration: Decoration,
    es: Locale,
    en: Locale,
}
impl Theme {
    fn load() -> Result<Self> {
        let theme: Self = serde_json::from_str(include_str!("../../templates/menu/theme.json"))?;
        for font in [
            &theme.fonts.header,
            &theme.fonts.category,
            &theme.fonts.command,
        ] {
            font_text("Aa0", font)?;
        }
        if theme.identity.prefix.is_empty()
            || theme.identity.prefix.len() > 16
            || theme.identity.prefix.chars().any(char::is_whitespace)
        {
            bail!("Invalid menu control prefix");
        }
        Ok(theme)
    }
    fn locale(&self, language: Language) -> &Locale {
        match language {
            Language::Es => &self.es,
            Language::En => &self.en,
        }
    }
    fn layout(style: Style, section: &str) -> Result<&'static str> {
        let source = match (style, section) {
            (Style::Classic, section) => crate::classic_template::menu_layout(section)
                .ok_or_else(|| anyhow::anyhow!("Unknown menu layout"))?,
            (Style::None, "root") => include_str!("../../templates/menu/plain/root.txt"),
            (Style::None, "category-row") => {
                include_str!("../../templates/menu/plain/category-row.txt")
            }
            (Style::None, "category") => include_str!("../../templates/menu/plain/category.txt"),
            (Style::None, "command-row") => {
                include_str!("../../templates/menu/plain/command-row.txt")
            }
            (Style::None, "all") => include_str!("../../templates/menu/plain/all.txt"),
            (Style::None, "all-category-row") => {
                include_str!("../../templates/menu/plain/all-category-row.txt")
            }
            (Style::None, "information") => {
                include_str!("../../templates/menu/plain/information.txt")
            }
            (Style::None, "system") => include_str!("../../templates/menu/plain/system.txt"),
            (Style::None, "empty") => include_str!("../../templates/menu/plain/empty.txt"),
            (Style::None, "instruction-row") => {
                include_str!("../../templates/menu/plain/instruction-row.txt")
            }
            (Style::None, "pagination") => {
                include_str!("../../templates/menu/plain/pagination.txt")
            }
            _ => bail!("Unknown menu layout"),
        };
        Ok(source)
    }
}

fn validated(value: String, limit: usize) -> Result<String> {
    let value = value.trim_end_matches('\n').to_owned();
    if value.is_empty()
        || value.len() > limit
        || value.chars().any(|ch| ch.is_control() && ch != '\n')
    {
        bail!("Menu output exceeds safe text bounds");
    }
    Ok(value)
}

/// Single-pass replacement. Data that happens to contain @Tokens is never expanded.
fn fill(source: &str, values: &Vars) -> Result<String> {
    let source = crate::classic_template::expand(source);
    let mut output = String::new();
    let bytes = source.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'@' {
            if bytes.get(i + 1) == Some(&b'@') {
                output.push('@');
                i += 2;
                continue;
            }
            let start = i + 1;
            let mut end = start;
            while end < bytes.len() && (bytes[end].is_ascii_alphanumeric() || bytes[end] == b'_') {
                end += 1;
            }
            if end > start {
                output.push_str(
                    values
                        .get(&source[start..end])
                        .ok_or_else(|| anyhow!("Unbound menu template variable"))?,
                );
                i = end;
                continue;
            }
        }
        let ch = source[i..]
            .chars()
            .next()
            .ok_or_else(|| anyhow!("Invalid menu template"))?;
        output.push(ch);
        i += ch.len_utf8();
    }
    Ok(output.replace("\r\n", "\n"))
}

fn values(theme: &Theme, options: Preferences) -> Result<Vars> {
    let language = theme.locale(options.language);
    let telemetry = super::telemetry::MenuTelemetry::capture();
    let mut v = Vars::new();
    for (key, value) in [
        ("Title", language.title.as_str()),
        ("CategoriesTitle", language.categories.as_str()),
        ("InformationTitle", language.information.as_str()),
        ("SystemTitle", language.system.as_str()),
        ("InstructionsTitle", language.instructions.as_str()),
    ] {
        v.insert(
            key,
            if options.style == Style::Classic {
                font_text(value, &theme.fonts.header)?
            } else {
                value.to_owned()
            },
        );
    }
    v.insert("AllTitle", language.all.clone());
    v.insert("RootTitle", language.root_title.clone());
    v.insert("MenuCommand", format!("{}menu", theme.identity.prefix));
    v.insert("Bot", theme.identity.bot.clone());
    v.insert("Project", theme.identity.project.clone());
    v.insert("Owner", theme.identity.owner.clone());
    v.insert("Prefix", theme.identity.prefix.clone());
    v.insert("Version", telemetry.version.to_owned());
    v.insert("Date", telemetry.date);
    v.insert("Commands", command_count().to_string());
    v.insert(
        "CategoriesActive",
        visible_category_ids()
            .iter()
            .filter(|id| **id != "all")
            .count()
            .to_string(),
    );
    v.insert("SubBots", language.unavailable.clone());
    v.insert("PremiumBots", language.unavailable.clone());
    v.insert("URL", language.unavailable.clone());
    v.insert("Host", telemetry.host);
    v.insert("OS", telemetry.operating_system);
    v.insert("Uptime", telemetry.process_uptime);
    v.insert("ServerRAM", telemetry.server_memory);
    v.insert("ControllerRAM", telemetry.control_memory);
    v.insert("ProviderRAM", language.unavailable.clone());
    for (key, value) in [
        ("LabelUser", &language.labels.user),
        ("LabelBot", &language.labels.bot),
        ("LabelProject", &language.labels.project),
        ("LabelOwner", &language.labels.owner),
        ("LabelPrefix", &language.labels.prefix),
        ("LabelCommands", &language.labels.commands),
        ("LabelCategoriesActive", &language.labels.categories_active),
        ("LabelVersion", &language.labels.version),
        ("LabelDate", &language.labels.date),
        ("LabelRole", &language.labels.role),
        ("LabelStatus", &language.labels.status),
        ("LabelSocket", &language.labels.socket),
        ("LabelSubBots", &language.labels.sub_bots),
        ("LabelPremiumBots", &language.labels.premium_bots),
        ("LabelURL", &language.labels.url),
        ("LabelHost", &language.labels.host),
        ("LabelOS", &language.labels.os),
        ("LabelUptime", &language.labels.uptime),
        ("LabelServerRAM", &language.labels.server_ram),
        ("LabelControllerRAM", &language.labels.controller_ram),
        ("LabelProviderRAM", &language.labels.provider_ram),
    ] {
        v.insert(key, value.clone());
    }
    Ok(v)
}

fn display_name(theme: &Theme, category: &Category, options: Preferences) -> Result<String> {
    // Names belong to the registered command category data (Category.es/en),
    // never to the source directory path or the Rust module name.
    let name = label(category, options.language);
    if options.style == Style::Classic {
        font_text(name, &theme.fonts.category)
    } else {
        Ok(name.to_owned())
    }
}

fn category_values(
    theme: &Theme,
    category: &Category,
    options: Preferences,
    base: &Vars,
) -> Result<Vars> {
    let mut v = base.clone();
    v.insert("CategoryId", category.id.to_owned());
    v.insert("CategoryIcon", category.icon.to_owned());
    v.insert("CategoryName", display_name(theme, category, options)?);
    v.insert(
        "CategoryCommands",
        category
            .commands
            .iter()
            .filter(|cmd| cmd.implemented)
            .count()
            .to_string(),
    );
    Ok(v)
}

fn command_rows(
    theme: &Theme,
    options: Preferences,
    usages: &[&str],
    base: &Vars,
) -> Result<String> {
    let mut rows = Vec::new();
    for (index, usage) in usages.iter().enumerate() {
        let mut v = base.clone();
        v.insert("Usage", font_text(usage, &theme.fonts.command)?);
        v.insert(
            "Branch",
            if index + 1 == usages.len() {
                theme.decoration.last_branch.clone()
            } else {
                theme.decoration.branch.clone()
            },
        );
        rows.push(
            fill(Theme::layout(options.style, "command-row")?, &v)?
                .trim_end()
                .to_owned(),
        );
    }
    Ok(rows.join("\n"))
}

pub fn render_root(options: Preferences, viewer: MenuViewer<'_>) -> Result<String> {
    let theme = Theme::load()?;
    let language = theme.locale(options.language);
    let mut v = values(&theme, options)?;
    let visible = visible_category_ids();
    let mut rows = Vec::new();
    for (index, id) in visible.iter().enumerate() {
        let mut row = if *id == "all" {
            let mut special = v.clone();
            special.insert("CategoryId", "all".to_owned());
            special.insert("CategoryIcon", theme.decoration.all_icon.clone());
            special.insert(
                "CategoryName",
                if options.style == Style::Classic {
                    font_text(&language.all, &theme.fonts.category)?
                } else {
                    language.all.clone()
                },
            );
            special
        } else {
            let item = CATEGORIES
                .iter()
                .find(|cat| cat.id == *id)
                .ok_or_else(|| anyhow!("Unknown registered menu category"))?;
            category_values(&theme, item, options, &v)?
        };
        row.insert(
            "Branch",
            if index + 1 == visible.len() {
                theme.decoration.last_branch.clone()
            } else {
                theme.decoration.branch.clone()
            },
        );
        rows.push(
            fill(Theme::layout(options.style, "category-row")?, &row)?
                .trim_end()
                .to_owned(),
        );
    }
    v.insert("CategoryRows", rows.join("\n"));
    v.insert("User", viewer.user.to_owned());
    v.insert("Role", viewer.role.to_owned());
    v.insert("Status", viewer.status.to_owned());
    v.insert("Socket", viewer.socket.to_owned());
    v.insert(
        "Information",
        fill(Theme::layout(options.style, "information")?, &v)?
            .trim_end()
            .to_owned(),
    );
    v.insert(
        "OwnerBlock",
        if viewer.socket_owner && options.style == Style::Classic {
            format!(
                "{}\n\n",
                crate::classic_template::expand(Theme::layout(options.style, "owner")?).trim_end()
            )
        } else {
            String::new()
        },
    );
    v.insert(
        "SystemBlock",
        if viewer.socket_owner {
            format!(
                "{}\n\n",
                fill(Theme::layout(options.style, "system")?, &v)?.trim_end()
            )
        } else {
            String::new()
        },
    );
    let mut instructions = Vec::new();
    for note in &language.notes {
        let mut item = v.clone();
        item.insert("Instruction", note.clone());
        instructions.push(
            fill(Theme::layout(options.style, "instruction-row")?, &item)?
                .trim_end()
                .to_owned(),
        );
    }
    v.insert("Instructions", instructions.join("\n"));
    validated(fill(Theme::layout(options.style, "root")?, &v)?, 3800)
}

pub fn render_category(category: &Category, page: usize, options: Preferences) -> Result<String> {
    let theme = Theme::load()?;
    let visible: Vec<_> = category
        .commands
        .iter()
        .filter(|cmd| cmd.implemented)
        .collect();
    if visible.is_empty() {
        return validated(
            fill(
                Theme::layout(options.style, "empty")?,
                &values(&theme, options)?,
            )?,
            3450,
        );
    }
    const PAGE_SIZE: usize = 7;
    let pages = visible.len().div_ceil(PAGE_SIZE);
    if page == 0 || page > pages {
        bail!("Menu page does not exist");
    }
    let shown = &visible[(page - 1) * PAGE_SIZE..visible.len().min(page * PAGE_SIZE)];
    let usages: Vec<_> = shown
        .iter()
        .flat_map(|cmd| cmd.usages.iter().copied())
        .collect();
    let mut v = category_values(&theme, category, options, &values(&theme, options)?)?;
    v.insert("CommandRows", command_rows(&theme, options, &usages, &v)?);
    v.insert("Page", page.to_string());
    v.insert("Pages", pages.to_string());
    let pagination = if pages > 1 {
        v.insert(
            "NextPage",
            (if page == pages { 1 } else { page + 1 }).to_string(),
        );
        fill(Theme::layout(options.style, "pagination")?, &v)?
            .trim_end()
            .to_owned()
    } else {
        String::new()
    };
    v.insert("Pagination", pagination);
    validated(fill(Theme::layout(options.style, "category")?, &v)?, 3450)
}

pub fn render_all(options: Preferences) -> Result<String> {
    let theme = Theme::load()?;
    let mut v = values(&theme, options)?;
    let mut rows = Vec::new();
    for category in CATEGORIES {
        let visible: Vec<_> = category
            .commands
            .iter()
            .filter(|cmd| cmd.implemented)
            .collect();
        if visible.is_empty() {
            continue;
        }
        let mut c = category_values(&theme, category, options, &v)?;
        let names: Vec<_> = visible.iter().map(|cmd| cmd.name).collect();
        c.insert("CommandRows", command_rows(&theme, options, &names, &c)?);
        rows.push(
            fill(Theme::layout(options.style, "all-category-row")?, &c)?
                .trim_end()
                .to_owned(),
        );
    }
    v.insert("AllRows", rows.join("\n"));
    validated(fill(Theme::layout(options.style, "all")?, &v)?, 3450)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variables_are_single_pass_and_unknown_tokens_fail_closed() {
        let mut v = Vars::new();
        v.insert("Commands", "@Owner".to_owned());
        v.insert("Owner", "private".to_owned());
        assert_eq!(fill("Commands: @Commands", &v).unwrap(), "Commands: @Owner");
        assert_eq!(fill("@@Commands", &v).unwrap(), "@Commands");
        assert!(fill("@Unknown", &v).is_err());
    }
    #[test]
    fn category_label_comes_from_metadata_not_folder_name() {
        let theme = Theme::load().unwrap();
        let fake = Category {
            id: "different_path",
            es: "Etiqueta Personal",
            en: "Custom label",
            icon: "✪",
            commands: &[],
        };
        let options = Preferences::default();
        assert!(display_name(&theme, &fake, options).unwrap().contains("𝙴"));
        let plain_en = Preferences {
            style: Style::None,
            language: Language::En,
        };
        assert_eq!(
            display_name(&theme, &fake, plain_en).unwrap(),
            "Custom label"
        );
    }
    #[test]
    fn auto_counts_match_registered_commands_and_visible_categories() {
        let theme = Theme::load().unwrap();
        let v = values(&theme, Preferences::default()).unwrap();
        assert_eq!(v["Commands"], command_count().to_string());
        assert_eq!(
            v["CategoriesActive"],
            CATEGORIES
                .iter()
                .filter(|category| category.commands.iter().any(|cmd| cmd.implemented))
                .count()
                .to_string()
        );
    }
}
