/*
 * Base-Matter — typed, side-effect-free grammar for global add commands.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resource {
    Id(String),
    Official(Option<String>),
    New,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Destination {
    Group,
    Community,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum User {
    Number(String),
    Username(String),
    Reply,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddIntent {
    User { user: User, destination: Option<(Destination, Resource)> },
    Group { community: Resource, hide: bool },
    Community { community: Resource, name: Option<String> },
    Backup { source: Resource, destination: Resource, hide: bool },
}

pub fn parse_add_text(value: &str) -> Result<AddIntent> {
    let rest = value
        .strip_prefix("=/>add ")
        .ok_or_else(|| anyhow::anyhow!("Canonical add prefix required"))?;
    if rest.is_empty() || rest.len() > 512 || rest.chars().any(char::is_control) {
        bail!("Invalid add command");
    }
    parse_add_args(&rest.split_whitespace().map(str::to_owned).collect::<Vec<_>>())
}

/// Parses syntax only; does not resolve aliases, approve plans or invoke WhatsApp.
/// The authenticated adapter must determine the source chat and replied sender.
pub fn parse_add_args(args: &[String]) -> Result<AddIntent> {
    if args.is_empty() || args.len() > 16 || args.iter().any(|a| {
        a.is_empty() || a.len() > 128 || a.chars().any(char::is_control)
    }) {
        bail!("Invalid add command arguments");
    }
    let tokens: Vec<&str> = args.iter().map(String::as_str).collect();
    match tokens[0] {
        "user" => user(&tokens[1..]),
        "group" => group(&tokens[1..]),
        "community" => community(&tokens[1..]),
        _ => bail!("Unknown add operation"),
    }
}

fn user(parts: &[&str]) -> Result<AddIntent> {
    let (who, remaining) = match parts.first().copied() {
        None | Some("group" | "community") => (User::Reply, parts),
        Some("reply") => (User::Reply, &parts[1..]),
        Some(value) if value.starts_with('@') => {
            let name = &value[1..];
            if name.is_empty() || name.len() > 64
                || !name.bytes().all(|b| b.is_ascii_alphanumeric() || b"._".contains(&b))
            {
                bail!("Invalid username");
            }
            (User::Username(name.to_owned()), &parts[1..])
        }
        Some(value) => {
            let digits = value.strip_prefix('+').unwrap_or(value);
            if !(7..=15).contains(&digits.len()) || !digits.bytes().all(|b| b.is_ascii_digit()) {
                bail!("Invalid phone identity");
            }
            (User::Number(digits.to_owned()), &parts[1..])
        }
    };
    let destination = if remaining.is_empty() {
        None
    } else {
        let scope = match remaining[0] {
            "group" => Destination::Group,
            "community" => Destination::Community,
            _ => bail!("Unknown destination type"),
        };
        let (value, extras) = reference(&remaining[1..], false)?;
        if !extras.is_empty() {
            bail!("Unexpected destination parameters");
        }
        Some((scope, value))
    };
    Ok(AddIntent::User { user: who, destination })
}

fn group(parts: &[&str]) -> Result<AddIntent> {
    let mut parts = parts;
    let mut hide = false;
    if parts.first() == Some(&"--hide") {
        hide = true;
        parts = &parts[1..];
    }
    if parts.first() != Some(&"community") {
        bail!("Group transfer requires a community");
    }
    let (community, extras) = reference(&parts[1..], false)?;
    let extras = if !hide && extras == ["--hide"] {
        hide = true;
        &[][..]
    } else {
        extras
    };
    if !extras.is_empty() {
        bail!("Invalid group transfer flags");
    }
    Ok(AddIntent::Group { community, hide })
}

fn community(parts: &[&str]) -> Result<AddIntent> {
    if parts.first() == Some(&"backup") {
        let (source, extras) = reference(&parts[1..], false)?;
        if extras.first() != Some(&"to") {
            bail!("Backup requires 'to' destination");
        }
        let (destination, trailing) = reference(&extras[1..], true)?;
        let hide = trailing == ["--hide"];
        if !trailing.is_empty() && !hide {
            bail!("Invalid backup flags");
        }
        return Ok(AddIntent::Backup {
            source, destination, hide,
        });
    }
    if parts.first() == Some(&"new") {
        if parts.len() < 2 || !parts[1].starts_with("--") || parts[1] == "--hide" {
            bail!("New community requires --<name>");
        }
        let mut name = parts[1..].join(" ");
        name.drain(..2);
        if name.trim().is_empty() || name.len() > 100
            || parts[1..].contains(&"--hide")
        {
            bail!("Invalid new community name");
        }
        return Ok(AddIntent::Community {
            community: Resource::New, name: Some(name),
        });
    }
    let (community, extras) = reference(parts, false)?;
    if !extras.is_empty() {
        bail!("Invalid community arguments");
    }
    Ok(AddIntent::Community { community, name: None })
}

fn reference<'a>(
    input: &'a [&'a str],
    allow_new: bool,
) -> Result<(Resource, &'a [&'a str])> {
    let Some(&value) = input.first() else {
        bail!("Resource identifier missing");
    };
    if value == "new" {
        if !allow_new {
            bail!("New resource not permitted here");
        }
        return Ok((Resource::New, &input[1..]));
    }
    if value == "official" {
        if let Some(tag) = input.get(1).and_then(|s| s.strip_prefix("--")) {
            if tag != "hide" && valid_tag(tag) {
                return Ok((Resource::Official(Some(tag.to_owned())), &input[2..]));
            }
        }
        return Ok((Resource::Official(None), &input[1..]));
    }
    if value.starts_with("--") || !valid_id(value) {
        bail!("Invalid resource ID");
    }
    Ok((Resource::Id(value.to_owned()), &input[1..]))
}

fn valid_tag(value: &str) -> bool {
    !value.is_empty() && value.len() <= 64
        && value.bytes().all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
}

fn valid_id(value: &str) -> bool {
    !value.is_empty() && value.len() <= 128
        && value.bytes().all(|b| b.is_ascii_alphanumeric() || b"@._:-".contains(&b))
}
