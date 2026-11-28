/*
 * WhatsApp-Matter — isolated whatsapp-rust 0.7 pairing-code provider.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
mod codex_bridge;
mod command_alerts;
mod maintenance;

use anyhow::{bail, Result};
use std::{
    collections::{HashMap, HashSet},
    io::IsTerminal,
    process::ExitCode,
    sync::{
        atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering},
        Arc, Mutex, OnceLock,
    },
    time::Instant,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::{timeout, Duration},
};
use whatsapp_matter_rust_provider::{
    classic_template,
    help_reply::help_from_quoted_message,
    parse_control_ack_with_presentation,
    recovery::{
        read_encrypted, recovery_key_from_env, save_encrypted, RecoveryMember, RecoverySnapshot,
    },
    render_provider_control,
    rich_transport::{experimental_self_chat_rich_message, styled_rich_message},
    ui::{
        button_menu_cards, legacy_row_probe_message, menu_button_selection,
        native_navigation_message_with_web_fallback, native_ui_probe_message,
        web_navigation_notice,
    },
    Config, ControlAck, Event, SessionLease,
};
use whatsapp_rust::{pair_code::PairCodeOptions, prelude::*, types::events::ConnectFailureReason};

// Command-only diagnostics; identity metadata stays in the local operator console.
const CONSOLE_ARGS_LIMIT: usize = 24;

fn console_field(value: &str, limit: usize) -> String {
    let mut chars = value.chars().filter(|c| {
        !c.is_control()
            && !matches!(*c, '\u{061c}' | '\u{200b}'..='\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2060}'..='\u{206f}')
    });
    let mut cleaned: String = chars.by_ref().take(limit).collect();
    if chars.next().is_some() && limit > 0 {
        cleaned.pop();
        cleaned.push('…');
    }
    if cleaned.trim().is_empty() {
        "-".to_owned()
    } else {
        cleaned
    }
}

fn console_identity(value: &str) -> String {
    // Only canonical routing identifiers, never arbitrary message content.
    let Some((account, domain)) = value.split_once('@') else {
        return "-".to_owned();
    };
    if account.is_empty()
        || value.len() > 128
        || !account
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._:-".contains(&b))
        || !matches!(domain, "s.whatsapp.net" | "g.us" | "lid" | "newsletter")
    {
        return "-".to_owned();
    }
    value.to_owned()
}

fn verified_group_sender_pn(
    sender: &str,
    alternate: Option<&str>,
    participants: impl IntoIterator<Item = [String; 3]>,
) -> Option<String> {
    let valid_number = |account: &str| {
        (8..=20).contains(&account.len()) && account.bytes().all(|b| b.is_ascii_digit())
    };
    if !sender.strip_suffix("@lid").is_some_and(valid_number) {
        return None;
    }
    // The exact event sender must occur in exactly one authenticated group
    // participant row. Never infer a mapping from names, text or similar IDs.
    let mut matching = participants
        .into_iter()
        .filter(|ids| ids.iter().any(|id| id == sender));
    let ids = matching.next()?;
    if matching.next().is_some() {
        return None;
    }
    let mut phones = ids
        .iter()
        .filter(|id| id.strip_suffix("@s.whatsapp.net").is_some_and(valid_number));
    let phone = phones.next()?;
    if phones.any(|id| id != phone)
        || alternate.is_some_and(|id| id != sender && id != phone.as_str())
    {
        return None;
    }
    Some(phone.clone())
}

// Unknown verbs are not commands: no IPC, metadata lookup, trace or reaction.
fn recognized_command(text: &str) -> bool {
    if !text.starts_with("=/>") {
        return false;
    }
    let verb = text.split_whitespace().next().unwrap_or_default();
    matches!(
        verb,
        "=/>menu"
            | "=/>menú"
            | "=/>status"
            | "=/>estado"
            | "=/>vault"
            | "=/>register"
            | "=/>help"
            | "=/>ayuda"
            | "=/>h"
            | "=/>set"
            | "=/>unset"
            | "=/>role"
            | "=/>roles"
            | "=/>audit"
            | "=/>start"
            | "=/>add"
            | "=/>add++"
            | "=/>test"
            | "=/>matter"
            | "=/>update"
            | "=/>restart"
            | "=/>reload"
            | "=/>codex"
            | "=/>anti"
            | "=/>ping"
            | "=/>bot"
            | "=/>group"
            | "=/>grupo"
            | "=/>credits"
            | "=/>creditos"
            | "=/>créditos"
    )
}

fn console_command(text: &str) -> String {
    let mut parts = text.split_whitespace();
    let verb = parts.next().unwrap_or_default();
    if !recognized_command(text) {
        return String::new();
    }
    // Never print arbitrary tokens, quoted payloads, targets or credentials.
    // Only a finite vocabulary of public command syntax is eligible.
    let mut preview = String::new();
    for arg in parts {
        let safe = match verb {
            "=/>menu" | "=/>menú" | "=/>help" | "=/>ayuda" | "=/>h" => matches!(
                arg,
                "matter"
                    | "classic"
                    | "modern"
                    | "web"
                    | "buttons"
                    | "all"
                    | "todo"
                    | "anti"
                    | "control"
                    | "system"
                    | "config"
                    | "owner"
                    | "help"
                    | "ia"
                    | "admin"
                    | "economy"
                    | "downloads"
                    | "fun"
                    | "gacha"
                    | "games"
                    | "group"
                    | "info"
                    | "links"
                    | "main"
                    | "mode"
                    | "profile"
                    | "reactions"
                    | "search"
                    | "sockets"
                    | "utils"
                    | "page"
                    | "1"
                    | "2"
                    | "3"
                    | "4"
                    | "5"
                    | "6"
                    | "7"
                    | "8"
                    | "status"
                    | "ping"
                    | "set"
                    | "role"
            ),
            "=/>bot" | "=/>group" | "=/>grupo" | "=/>status" | "=/>estado" => matches!(
                arg,
                "on" | "off"
                    | "inherit"
                    | "encender"
                    | "apagar"
                    | "heredar"
                    | "group"
                    | "private"
                    | "channel"
                    | "community"
                    | "--mod"
                    | "--global"
                    | "--chat"
                    | "--socket"
                    | "--user"
                    | "--debug"
                    | "status"
                    | "estado"
            ),
            "=/>set" | "=/>unset" | "=/>anti" => matches!(
                arg,
                "role"
                    | "owner"
                    | "admin"
                    | "anti"
                    | "link"
                    | "on"
                    | "off"
                    | "inherit"
                    | "--mod"
                    | "--global"
                    | "--debug"
                    | "menu"
                    | "style"
                    | "lang"
                    | "language"
                    | "matter"
                    | "classic"
                    | "modern"
                    | "none"
                    | "web"
                    | "buttons"
                    | "es"
                    | "en"
                    | "audit"
                    | "--except"
                    | "--include"
            ),
            "=/>test" => matches!(
                arg,
                "ui" | "row"
                    | "html"
                    | "ack"
                    | "a"
                    | "b"
                    | "one"
                    | "two"
                    | "audio"
                    | "mp4"
                    | "link"
            ),
            "=/>vault" => matches!(arg, "status" | "test"),
            "=/>register" => arg == "start",
            "=/>add++" => arg == "snapshot",
            _ => false,
        };
        if !preview.is_empty() {
            preview.push(' ');
        }
        if !safe {
            preview.push('…');
            break;
        }
        preview.push_str(arg);
        if preview.chars().count() > CONSOLE_ARGS_LIMIT {
            break;
        }
    }
    if preview.is_empty() {
        verb.to_owned()
    } else {
        format!("{verb} {}", console_field(&preview, CONSOLE_ARGS_LIMIT))
    }
}

type ConsoleMetadata = (String, String, String, &'static str);
type ConsoleCache = HashMap<(String, String), (Instant, ConsoleMetadata)>;

async fn console_metadata(
    ctx: &whatsapp_rust::bot::MessageContext,
    data: &Event<'_>,
) -> ConsoleMetadata {
    let alt = ctx
        .info
        .source
        .sender_alt
        .as_ref()
        .map(|j| j.to_non_ad_string());
    let lid = [Some(data.sender_jid), alt.as_deref()]
        .into_iter()
        .flatten()
        .find(|j| j.ends_with("@lid"))
        .unwrap_or("-");
    let fallback = (
        "-".to_owned(),
        "-".to_owned(),
        console_identity(lid),
        "Sin verificar",
    );
    if !data.is_group {
        return fallback;
    }
    static CACHE: OnceLock<Mutex<ConsoleCache>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let key = (data.chat_jid.to_owned(), data.sender_jid.to_owned());
    if let Some(value) = cache.lock().ok().and_then(|c| {
        c.get(&key)
            .filter(|(at, _)| at.elapsed() < Duration::from_secs(300))
            .map(|(_, value)| value.clone())
    }) {
        return value;
    }
    // Best effort only: no indefinite network wait or authorization by profile fields.
    let value = match timeout(
        Duration::from_millis(500),
        ctx.client.groups().get_metadata(&ctx.info.source.chat),
    )
    .await
    {
        Ok(Ok(metadata)) => {
            let participant = metadata.participants.iter().find(|p| {
                [
                    &p.jid,
                    p.phone_number.as_ref().unwrap_or(&p.jid),
                    p.lid.as_ref().unwrap_or(&p.jid),
                ]
                .iter()
                .any(|j| j.to_non_ad_string() == data.sender_jid)
            });
            let username = participant
                .and_then(|p| p.username.as_ref())
                .map(|u| console_field(u.as_str(), 12))
                .unwrap_or_else(|| "-".to_owned());
            let lid = participant
                .and_then(|p| p.lid.as_ref())
                .map(|j| console_identity(&j.to_non_ad_string()))
                .unwrap_or_else(|| fallback.2.clone());
            let role = match participant {
                Some(p) if p.is_admin() => "Admin WA",
                Some(_) => "Miembro WA",
                None => "Sin verificar",
            };
            (console_field(&metadata.subject, 20), username, lid, role)
        }
        _ => fallback,
    };
    if let Ok(mut c) = cache.lock() {
        if c.len() >= 256 {
            c.clear();
        }
        c.insert(key, (Instant::now(), value.clone()));
    }
    value
}

fn console_color(value: &str, palette: &str) -> String {
    if std::io::stdout().is_terminal()
        && std::env::var_os("NO_COLOR").is_none()
        && !matches!(std::env::var("TERM").as_deref(), Ok("dumb"))
    {
        let color = classic_template::console_palette(palette);
        format!("\x1b[{color}m{value}\x1b[0m")
    } else {
        value.to_owned()
    }
}

fn console_row(body: &str, last: bool) -> String {
    let template = if last {
        classic_template::console_template("last-row")
    } else {
        classic_template::console_template("row")
    };
    classic_template::render(template, &[("BODY", body)])
        .trim_end_matches('\n')
        .to_owned()
}

// Call only after an authenticated Applied ACK for a menu request.
// Group menus are public read-only; external private chats stay excluded.
fn web_html_skip_reason(
    enabled: bool,
    autosend: bool,
    root: bool,
    group: bool,
    from_me: bool,
    owner_chat: bool,
) -> Option<&'static str> {
    if !enabled {
        Some("HTML DESACTIVADO")
    } else if root && !autosend {
        Some("HTML AUTOSEND DESACTIVADO")
    } else if !group && !(from_me && owner_chat) {
        Some("HTML PRIVADO NO AUTORIZADO")
    } else {
        None
    }
}

struct ConsoleTraceHeader {
    header: String,
    rows: [String; 2],
    scope: String,
    wa_role: &'static str,
    socket_owner: bool,
}

fn console_matter_role(global_owner: bool, socket_owner: bool) -> &'static str {
    if global_owner {
        "Owner global"
    } else if socket_owner {
        "Owner socket"
    } else {
        "Sin verificar"
    }
}

struct CommandTrace {
    id: u64,
    started: Instant,
    enabled: bool,
    ipc: Mutex<String>,
    tx: Mutex<Vec<String>>,
    processing_reaction: AtomicBool,
    header: Option<ConsoleTraceHeader>,
}

impl CommandTrace {
    fn new(enabled: bool, started: Instant) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        Self {
            id: NEXT.fetch_add(1, Ordering::Relaxed),
            started,
            enabled,
            ipc: Mutex::new("PEND".to_owned()),
            tx: Mutex::new(Vec::new()),
            processing_reaction: AtomicBool::new(false),
            header: None,
        }
    }

    async fn populate_header(
        &mut self,
        ctx: &whatsapp_rust::bot::MessageContext,
        data: &Event<'_>,
        origin: &str,
        socket_owner: bool,
    ) {
        if self.enabled {
            let clock = data.timestamp.rem_euclid(86400);
            let (h, m, s) = (clock / 3600, clock / 60 % 60, clock % 60);
            let name = console_field(&ctx.info.push_name, 18);
            let sender = console_identity(data.sender_jid);
            let chat = console_identity(data.chat_jid);
            let command = console_color(&console_command(data.text), "command");
            let (group, username, lid, group_role) = console_metadata(ctx, data).await;
            let identity = if lid == "-" { sender } else { lid };
            let scope = if data.is_group {
                classic_template::console_layout(
                    "group",
                    &[
                        (
                            "LABEL",
                            &console_color(classic_template::console_label("group"), "scope"),
                        ),
                        ("GROUP", &group),
                        ("CHAT", &chat),
                    ],
                )
            } else {
                classic_template::console_layout(
                    "private",
                    &[
                        (
                            "LABEL",
                            &console_color(classic_template::console_label("private"), "scope"),
                        ),
                        ("CHAT", &chat),
                    ],
                )
            };
            let user = if username == "-" {
                name
            } else {
                classic_template::console_layout(
                    "username",
                    &[("NAME", &name), ("USERNAME", &username)],
                )
            };
            let header = classic_template::render(
                classic_template::console_template("header"),
                &[
                    ("EVENT", &format!("#{:04}", self.id)),
                    ("CLOCK", &format!("{h:02}:{m:02}:{s:02}Z")),
                    ("ORIGIN", origin),
                ],
            );
            let rows = [
                classic_template::console_layout("command", &[("COMMAND", &command)]),
                classic_template::console_layout(
                    "user_identity",
                    &[
                        (
                            "LABEL",
                            &console_color(classic_template::console_label("user"), "label"),
                        ),
                        ("USER", &console_color(&user, "value")),
                        ("ID", &console_color(&identity, "identity")),
                    ],
                ),
            ];
            self.header = Some(ConsoleTraceHeader {
                header,
                rows,
                scope,
                wa_role: group_role,
                socket_owner,
            });
        }
    }

    fn print_header(&self, global_owner: bool) {
        let Some(header) = &self.header else {
            return;
        };
        let matter_role = console_matter_role(global_owner, header.socket_owner);
        let matter_color = if global_owner || header.socket_owner {
            "owner"
        } else {
            "muted"
        };
        let wa_color = match header.wa_role {
            "Admin WA" => "admin",
            "Miembro WA" => "member",
            _ => "muted",
        };
        let ranks = classic_template::console_layout(
            "ranks",
            &[
                ("WA", &console_color(header.wa_role, wa_color)),
                ("MATTER", &console_color(matter_role, matter_color)),
            ],
        );
        let rows = [&header.rows[0], &header.rows[1], &ranks, &header.scope];
        println!(
            "{}\n{}",
            console_color(header.header.trim_end_matches('\n'), "frame"),
            rows.iter()
                .map(|row| console_row(row, false))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }

    fn stage(&self, stage: &str) {
        if !self.enabled {
            return;
        }
        if let Some(value) = stage.strip_prefix("IPC: ") {
            if let Ok(mut ipc) = self.ipc.lock() {
                *ipc = value.to_owned();
            }
        } else if let Ok(mut tx) = self.tx.lock() {
            tx.push(stage.to_owned());
        }
    }

    fn finish(&self, state: &str) {
        if self.enabled {
            let ipc = self.ipc.lock().map(|v| v.clone()).unwrap_or_default();
            let tx = self.tx.lock().map(|v| v.clone()).unwrap_or_default();
            let (label, color) = if state == "MANTENIMIENTO AUTORIZADO" {
                ("AUTORIZADO", "success")
            } else if state == "MANTENIMIENTO COMPLETADO" {
                ("COMPLETADO", "success")
            } else if state == "CODEX EN CURSO" {
                ("EN CURSO", "muted")
            } else if matches!(state, "RECARGA PARCIAL" | "CODEX EN REVISIÓN") {
                (state, "warning")
            } else if state == "MANTENIMIENTO BLOQUEADO" {
                ("BLOQUEADO", "warning")
            } else if state == "MANTENIMIENTO SIN CAMBIOS" {
                ("SIN CAMBIOS", "muted")
            } else if state.starts_with("ENVÍO ACEPTADO")
                || state.starts_with("ENVIADO")
                || state.starts_with("HTML ENVIADO")
            {
                ("ACEPTADO", "success")
            } else if state == "IGNORADO" || state == "DUPLICADO" {
                (state, "muted")
            } else if state.starts_with("TIMEOUT") || state.starts_with("SIN ENVÍO") {
                ("INCIERTO", "warning")
            } else {
                ("FALLO", "error")
            };
            let ipc_color = if ipc.starts_with("OK") {
                "tx_ok"
            } else if ipc == "IGNORADO" || ipc == "DUPLICADO" {
                "muted"
            } else {
                "tx_error"
            };
            let ipc_short = match ipc.as_str() {
                "DENEGADO O NO DISPONIBLE" => "DENEG./NO DISP.",
                "NO ADMITIDO" => "NO ADMITIDO",
                _ => &ipc,
            };
            let mut lines = vec![classic_template::console_layout(
                "ipc",
                &[
                    ("IPC", &console_color(ipc_short, ipc_color)),
                    ("STATE", &console_color(label, color)),
                    ("ELAPSED", &self.started.elapsed().as_millis().to_string()),
                ],
            )];
            if tx.is_empty() {
                lines.push(classic_template::console_layout("tx", &[("STAGE", "-")]));
            } else {
                for item in tx.iter().take(4) {
                    lines.push(classic_template::console_layout("tx", &[("STAGE", item)]));
                }
                if tx.len() > 4 {
                    lines.push(classic_template::console_layout(
                        "tx_more",
                        &[("COUNT", &(tx.len() - 4).to_string())],
                    ));
                }
            }
            if label == "ACEPTADO" {
                lines.push(classic_template::console_layout("client", &[]));
            }
            let last = lines.len().saturating_sub(1);
            let rows = lines
                .iter()
                .enumerate()
                .map(|(index, row)| console_row(row, index == last))
                .collect::<Vec<_>>()
                .join("\n");
            let footer = classic_template::expand(classic_template::console_template("footer"));
            println!(
                "{rows}\n{}",
                console_color(footer.trim_end_matches('\n'), "frame")
            );
        }
    }

    async fn send<T, E>(
        &self,
        kind: &str,
        future: impl std::future::Future<Output = std::result::Result<T, E>>,
    ) -> std::result::Result<std::result::Result<T, E>, tokio::time::error::Elapsed> {
        let start = Instant::now();
        let result = timeout(Duration::from_secs(8), future).await;
        let (status, color) = match &result {
            Ok(Ok(_)) => ("OK", "tx_ok"),
            Ok(Err(_)) => ("ERROR", "tx_error"),
            Err(_) => ("TIMEOUT?", "tx_timeout"),
        };
        if self.enabled {
            let kind = match kind {
                "NAVEGACIÓN" => "NAV",
                "TARJETA" => "CARD",
                other => other,
            };
            self.stage(&format!(
                "{kind}:{} {}ms",
                console_color(status, color),
                start.elapsed().as_millis()
            ));
        }
        result
    }
}

/// The official Matter-Bot Pong frame, with measured values only.
/// Socket is the local authenticated controller RTT, not WhatsApp network RTT.
fn matter_ping_message(socket: Duration, process: Duration) -> String {
    let socket_ms = socket.as_secs_f64() * 1000.0;
    let process_ms = process.as_secs_f64() * 1000.0;
    let total_ms = (socket + process).as_secs_f64() * 1000.0;
    classic_template::render(
        classic_template::layout("pong"),
        &[
            ("SOCKET", &format!("{socket_ms:.3}")),
            ("PROCESS", &format!("{process_ms:.3}")),
            ("TOTAL", &format!("{total_ms:.3}")),
        ],
    )
    .trim_end_matches('\n')
    .to_owned()
}

fn plain_ping_message(socket: Duration, process: Duration) -> String {
    format!(
        "Pong\nSocket: {:.3} ms\nProcess: {:.3} ms\nTotal: {:.3} ms",
        socket.as_secs_f64() * 1000.0,
        process.as_secs_f64() * 1000.0,
        (socket + process).as_secs_f64() * 1000.0,
    )
}

async fn deliver(
    address: std::net::SocketAddrV4,
    frame: &whatsapp_matter_rust_provider::SignedFrame,
    verified_rank: Option<&mut bool>,
    plain_style: Option<&mut bool>,
) -> Result<ControlAck> {
    let body = serde_json::to_vec(frame)?;
    if body.len() >= 16 * 1024 {
        bail!("Control frame exceeds limit");
    }
    let transport = async {
        let mut stream = TcpStream::connect(address).await?;
        stream.set_nodelay(true)?;
        stream.write_all(&body).await?;
        stream.shutdown().await?;
        let mut response = Vec::new();
        stream.take(8193).read_to_end(&mut response).await?;
        if response.len() > 8192 {
            bail!("Invalid bridge reply length");
        }
        let (ack, rank, plain) = parse_control_ack_with_presentation(&response);
        if let Some(plain_style) = plain_style {
            *plain_style = plain;
        }
        if let Some(verified_rank) = verified_rank {
            *verified_rank = rank;
        }
        ack
    };
    let deadline = if frame.from_me
        && frame.text.starts_with("=/>matter ")
        && !frame.chat_jid.ends_with("@g.us")
    {
        Duration::from_secs(83)
    } else if (matches!(
        frame.text.split_whitespace().next(),
        Some("=/>vault" | "=/>codex")
    ) || frame.text.starts_with("=/>set role owner --global"))
        && !frame.chat_jid.ends_with("@g.us")
    {
        // Both owner-linked and external verified-owner private messages
        // require a private Vault authority check before any command.
        Duration::from_secs(12)
    } else if frame.text.starts_with("=/>set ")
        || frame.text.starts_with("\u{001f}matter-group-style:")
    {
        Duration::from_secs(30)
    } else {
        Duration::from_secs(4)
    };
    timeout(deadline, transport).await?
}

// Return only a generic diagnostic for the verified socket owner's private
// Vault command, never for groups or outside senders.
fn is_private_vault_command(
    frame: &whatsapp_matter_rust_provider::SignedFrame,
    verified_owner_jid: &str,
) -> bool {
    frame.from_me
        && frame.chat_jid == verified_owner_jid
        && frame.text.split_whitespace().next() == Some("=/>vault")
}

async fn command_alert(
    trace: &CommandTrace,
    ctx: &whatsapp_rust::bot::MessageContext,
    session: &std::path::Path,
    kind: command_alerts::Kind,
) {
    let mode = command_alerts::mode(session, kind);
    if kind != command_alerts::Kind::Processing
        && mode != command_alerts::Mode::Reaction
        && trace.processing_reaction.swap(false, Ordering::Relaxed)
    {
        let _ = trace.send("REACCIÓN", ctx.react("")).await;
    }
    match mode {
        command_alerts::Mode::Off => {}
        command_alerts::Mode::Reaction => {
            // Native reactions replace the previous reaction on the same message.
            // Never fall back to text when a reaction cannot be sent.
            let sent = matches!(
                trace.send("REACCIÓN", ctx.react(kind.emoji())).await,
                Ok(Ok(_))
            );
            if sent {
                trace
                    .processing_reaction
                    .store(kind == command_alerts::Kind::Processing, Ordering::Relaxed);
            }
        }
        command_alerts::Mode::Text => {
            if let Ok(body) = maintenance::styled("Alerta", kind.message(), false) {
                let _ = trace.send("AVISO", ctx.reply(&body)).await;
            }
        }
    }
}

async fn finish_command(
    trace: &CommandTrace,
    ctx: &whatsapp_rust::bot::MessageContext,
    session: &std::path::Path,
    state: &str,
) {
    if let Some(kind) = command_alerts::completion(state) {
        command_alert(trace, ctx, session, kind).await;
    }
    trace.finish(state);
}

async fn require_local_controller(address: std::net::SocketAddrV4) -> Result<()> {
    // No SQLite, pairing request, login or authenticated command is sent.
    // A successful TCP connection only proves that the local port listens.
    timeout(Duration::from_secs(3), TcpStream::connect(address))
        .await
        .map_err(|_| anyhow::anyhow!("Base-Matter local controller unreachable"))?
        .map_err(|_| anyhow::anyhow!("Base-Matter local controller unreachable"))?;
    Ok(())
}

async fn discover_register(
    ctx: &whatsapp_rust::bot::MessageContext,
    owner_jid: &str,
) -> Result<String> {
    let participating = ctx
        .client
        .community()
        .get_participating()
        .await
        .map_err(|_| anyhow::anyhow!("Community inventory unavailable"))?;
    if participating.len() > 12 {
        bail!("Too many communities to inspect safely");
    }
    let mut parents: Vec<_> = participating.into_iter().collect();
    parents.sort_by_key(|(jid, _)| jid.to_string());
    let mut managed = 0_usize;
    let mut summaries = Vec::new();
    for (parent_id, metadata) in parents {
        if !metadata.is_parent_group {
            continue;
        }
        let is_admin = metadata.participants.iter().any(|participant| {
            participant.is_admin()
                && (participant.jid.to_string() == owner_jid
                    || participant
                        .phone_number
                        .as_ref()
                        .is_some_and(|jid| jid.to_string() == owner_jid)
                    || participant
                        .lid
                        .as_ref()
                        .is_some_and(|jid| jid.to_string() == owner_jid))
        });
        if !is_admin {
            continue;
        }
        managed += 1;
        let subgroups = ctx
            .client
            .community()
            .get_subgroups(&parent_id)
            .await
            .map_err(|_| anyhow::anyhow!("Subgroup inventory unavailable"))?;
        if subgroups.len() > 64 {
            bail!("Community has too many subgroups for one scan");
        }
        let mut matches = Vec::new();
        let mut examined = 0_usize;
        for subgroup in subgroups {
            if subgroup.is_default_sub_group || subgroup.is_general_chat {
                continue;
            }
            let group = ctx
                .client
                .community()
                .query_linked_group(parent_id.clone(), subgroup.id)
                .await
                .map_err(|_| anyhow::anyhow!("Linked group metadata unavailable"))?;
            if group.parent_group_jid.as_ref() != Some(&parent_id) {
                continue;
            }
            examined += 1;
            if group.description.as_deref().is_some_and(|description| {
                whatsapp_matter_rust_provider::official_register_description(
                    description,
                    &parent_id.to_string(),
                )
            }) {
                matches.push(group.subject);
            }
        }
        let register = if matches.len() > 1 {
            "COLISION: multiples grupos oficiales".to_owned()
        } else if let Some(name) = matches.first() {
            format!("Encontrado: {}", name.chars().take(60).collect::<String>())
        } else {
            "No existe: se requiere plan de creación autorizado".to_owned()
        };
        let community_name: String = metadata.subject.chars().take(60).collect();
        summaries.push(format!(
            "{} | grupos: {} | REGISTER: {}",
            community_name, examined, register
        ));
    }
    if managed == 0 {
        return Ok(
            "Base-Matter REGISTER: ninguna comunidad con admin verificado para esta identidad PN/LID. No se hicieron cambios."
                .to_owned(),
        );
    }
    let report = format!(
        "Base-Matter REGISTER - Solo lectura\nComunidades admin: {}\n{}\nSin cambios administrativos.",
        managed,
        summaries.join("\n")
    );
    Ok(report.chars().take(3000).collect())
}

async fn preview_community_recovery(
    ctx: &whatsapp_rust::bot::MessageContext,
    owner_jid: &str,
) -> Result<String> {
    let participating = ctx
        .client
        .community()
        .get_participating()
        .await
        .map_err(|_| anyhow::anyhow!("Community inventory unavailable"))?;
    if participating.len() > 12 {
        bail!("Community inventory exceeds safe review limit");
    }
    let mut parents: Vec<_> = participating.into_iter().collect();
    parents.sort_by_key(|(jid, _)| jid.to_string());
    let mut results = Vec::new();
    for (community_id, metadata) in parents {
        if !metadata.is_parent_group {
            continue;
        }
        let is_admin = metadata.participants.iter().any(|p| {
            p.is_admin()
                && (p.jid.to_string() == owner_jid
                    || p.phone_number
                        .as_ref()
                        .is_some_and(|jid| jid.to_string() == owner_jid)
                    || p.lid
                        .as_ref()
                        .is_some_and(|jid| jid.to_string() == owner_jid))
        });
        if !is_admin {
            continue;
        }
        if metadata.is_suspended {
            results.push("Comunidad suspendida: consulta de miembros no disponible".to_owned());
            continue;
        }
        let participants = ctx
            .client
            .community()
            .get_linked_groups_participants(community_id.clone())
            .await
            .map_err(|_| anyhow::anyhow!("Community participant inventory unavailable"))?;
        if participants.len() > 15_000 {
            bail!("Participant inventory exceeds safe limit");
        }
        let mut unique = HashSet::new();
        for participant in participants {
            let id = participant
                .lid
                .or(participant.phone_number)
                .unwrap_or(participant.jid);
            unique.insert(id.to_string());
        }
        let name: String = metadata.subject.chars().take(38).collect();
        let summary = format!(
            "Comunidad: {name}\nID: {community_id}\nMiembros aprox.: {}\nRespaldo sugerido: {name} - Respaldo",
            unique.len()
        );
        results.push(summary);
    }
    if results.is_empty() {
        return Ok(
            "Base-Matter: sin comunidades con admin PN/LID verificado; no se copió ningún miembro."
                .to_owned(),
        );
    }
    let answer = format!(
        "Base-Matter =/>add++ (VISTA PREVIA)\n{}\nNo se creó grupo, no se añadieron personas.\nPara conservar datos antes de la suspensión se necesita una instantánea cifrada.",
        results.join("\n\n")
    );
    Ok(answer.chars().take(1600).collect())
}

async fn snapshot_community_recovery(
    ctx: &whatsapp_rust::bot::MessageContext,
    config: &Config,
    requested_community: &str,
) -> Result<String> {
    if requested_community.is_empty()
        || requested_community.len() > 128
        || !requested_community.ends_with("@g.us")
        || !requested_community
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"@._:-".contains(&byte))
    {
        bail!("Invalid community recovery scope");
    }
    let participating = ctx
        .client
        .community()
        .get_participating()
        .await
        .map_err(|_| anyhow::anyhow!("Community metadata unavailable"))?;
    let (source_id, metadata) = participating
        .into_iter()
        .find(|(jid, group)| jid.to_string() == requested_community && group.is_parent_group)
        .ok_or_else(|| anyhow::anyhow!("Community is not accessible"))?;
    if metadata.is_suspended {
        bail!("Suspended community may not be backed up");
    }
    let owner_is_admin = metadata.participants.iter().any(|member| {
        member.is_admin()
            && (member.jid.to_string() == config.owner_jid
                || member
                    .phone_number
                    .as_ref()
                    .is_some_and(|jid| jid.to_string() == config.owner_jid)
                || member
                    .lid
                    .as_ref()
                    .is_some_and(|jid| jid.to_string() == config.owner_jid))
    });
    if !owner_is_admin {
        bail!("Authenticated community admin required");
    }
    let participants = ctx
        .client
        .community()
        .get_linked_groups_participants(source_id.clone())
        .await
        .map_err(|_| anyhow::anyhow!("Community participant inventory unavailable"))?;
    if participants.len() > 15_000 {
        bail!("Participant count exceeds recovery snapshot limit");
    }
    let key = recovery_key_from_env(&config.bridge_key)?;
    let mut seen = HashSet::new();
    let mut members = Vec::new();
    for participant in participants {
        let candidate = participant
            .lid
            .as_ref()
            .or(participant.phone_number.as_ref())
            .unwrap_or(&participant.jid)
            .to_string();
        if !seen.insert(candidate) {
            continue;
        }
        members.push(RecoveryMember {
            jid: participant.jid.to_string(),
            phone_jid: participant.phone_number.map(|jid| jid.to_string()),
            lid_jid: participant.lid.map(|jid| jid.to_string()),
        });
    }
    let created = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    let name: String = metadata
        .subject
        .chars()
        .filter(|character| !character.is_control())
        .take(80)
        .collect();
    let snapshot = RecoverySnapshot {
        version: 1,
        socket_id: config.socket_id.clone(),
        owner_jid: config.owner_jid.clone(),
        community_jid: source_id.to_string(),
        community_name: name,
        captured_at: created,
        members,
    };
    let count = snapshot.members.len();
    let file = save_encrypted(&config.session_db, &key, &snapshot)?;
    let filename = file
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("Encrypted recovery path unavailable"))?;
    Ok(format!(
        "Base-Matter: snapshot cifrada guardada. Miembros: {count}. Archivo local: {filename}. No se añadió nadie. Protege el archivo y su clave por separado."
    ))
}

fn classify_logout(on_connect: bool, reason: ConnectFailureReason) -> &'static str {
    // Distinguish server rejection during handshake from a stream-level
    // removal. Never print account identifiers, raw node contents, or tokens.
    match (on_connect, reason) {
        (_, ConnectFailureReason::AccountLocked) => {
            "WA_LOGOUT_CLASS=ACCOUNT_LOCKED; inspect official WhatsApp account status, do not retry."
        }
        (true, ConnectFailureReason::LoggedOut) => {
            "WA_LOGOUT_CLASS=AUTH_REJECTED_LOGGED_OUT; stored device rejected at login."
        }
        (true, ConnectFailureReason::UnknownLogout) => {
            "WA_LOGOUT_CLASS=AUTH_REJECTED_UNKNOWN_LOGOUT; stored device rejected at login."
        }
        (false, ConnectFailureReason::LoggedOut) => {
            "WA_LOGOUT_CLASS=STREAM_LOGGED_OUT; device removed or unauthorized during stream."
        }
        (_, _) => {
            "WA_LOGOUT_CLASS=OTHER; preserve local credentials and inspect linked devices."
        }
    }
}

fn safe_resume_event(kind: EventKind) -> Option<&'static str> {
    match kind {
        EventKind::Disconnected => Some(
            "WhatsApp connection interrupted. No pairing requested; existing session retained.",
        ),
        EventKind::ConnectFailure => Some(
            "WhatsApp server rejected a connection attempt. Check linked devices and network.",
        ),
        EventKind::ClientOutdated => Some(
            "WhatsApp client version rejected. Update the Rust provider before trying a new pairing.",
        ),
        EventKind::TemporaryBan => Some(
            "WhatsApp temporary restriction reported. Stop attempts; do not request pairing.",
        ),
        EventKind::StreamError => Some(
            "WhatsApp stream error reported. Existing session retained.",
        ),
        _ => None,
    }
}

fn pairing_options(resume_only: bool, phone: String) -> Option<PairCodeOptions> {
    if resume_only {
        None
    } else {
        Some(PairCodeOptions {
            phone_number: phone,
            ..Default::default()
        })
    }
}

#[tokio::main]
async fn main() -> Result<ExitCode> {
    let resume_only = matches!(std::env::args().nth(1).as_deref(), Some("--resume-only"));
    match std::env::args().nth(1).as_deref() {
        Some("--rich-preview-html") => {
            if std::env::args().nth(2).is_some() {
                bail!("HTML preview accepts no parameters");
            }
            println!(
                "{}",
                whatsapp_matter_rust_provider::rich_preview::menu_html_preview(
                    whatsapp_matter_rust_provider::rich_preview::original_menu()
                )?
            );
            return Ok(ExitCode::SUCCESS);
        }
        Some("--rich-preview-json") => {
            if std::env::args().nth(2).is_some() {
                bail!("Rich preview accepts no parameters");
            }
            println!(
                "{}",
                serde_json::to_string_pretty(
                    &whatsapp_matter_rust_provider::rich_preview::experimental_rich_schema(
                        whatsapp_matter_rust_provider::rich_preview::original_menu()
                    )?
                )?
            );
            return Ok(ExitCode::SUCCESS);
        }
        Some("--help") => {
            println!("WhatsApp-Matter Rust provider: --check, --resume-only (restore stored identity; no pairing-code request), --rich-preview-html, --rich-preview-json, --recovery-verify <encrypted-filename>, or run for pairing code. Local Base-Matter IPC; no QR.");
            return Ok(ExitCode::SUCCESS);
        }
        Some("--check") => {
            let config = Config::from_env()?;
            SessionLease::check_available(&config.session_db)?;
            println!("whatsapp-rust pairing settings checked; no session was opened.");
            return Ok(ExitCode::SUCCESS);
        }
        Some("--recovery-verify") => {
            let filename = std::env::args()
                .nth(2)
                .ok_or_else(|| anyhow::anyhow!("Recovery archive name is required"))?;
            if !filename.starts_with("community-recovery-")
                || !filename.ends_with(".enc.json")
                || filename.len() > 160
                || !filename
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || b"._-".contains(&byte))
                || std::env::args().nth(3).is_some()
            {
                bail!("Invalid recovery archive name");
            }
            let config = Config::from_env()?;
            let key = recovery_key_from_env(&config.bridge_key)?;
            let path = config
                .session_db
                .parent()
                .ok_or_else(|| anyhow::anyhow!("Private session directory missing"))?
                .join(filename);
            let snapshot = read_encrypted(&path, &key, &config.socket_id, &config.owner_jid)?;
            println!(
                "Recovery snapshot authenticated: {} member records; no session was opened.",
                snapshot.members.len()
            );
            return Ok(ExitCode::SUCCESS);
        }
        Some("--resume-only") => {
            if std::env::args().nth(2).is_some() {
                bail!("Resume-only mode accepts no additional arguments");
            }
        }
        None => {}
        Some(_) => bail!("Unsupported provider operation"),
    }
    let config = Arc::new(Config::from_env()?);
    // Opt-out native navigation in alpha: the new root menu is intentionally
    // compact and sends only read-only selectors after authenticated IPC.
    // Operators can set MATTER_WA_NATIVE_BUTTONS=0 when a client fails to
    // render it; a transport send error still falls back to normal text.
    let native_buttons = !matches!(
        std::env::var("MATTER_WA_NATIVE_BUTTONS").as_deref(),
        Ok("0" | "off" | "false")
    );
    // Experimental in-chat HTML Rich UI: explicit operator opt-in, and only
    // after controller authentication/authorization of =/>menu.
    // No claims about phone rendering or Meta AI identity.
    let rich_html_trial = matches!(std::env::var("MATTER_WA_RICH_HTML").as_deref(), Ok("1"));
    let rich_html_autosend = matches!(std::env::var("MATTER_WA_HTML_AUTOSEND").as_deref(), Ok("1"));
    // Web attempts experimental HTML after an admitted menu command,
    // with independent NativeFlow navigation and explicit fallback.
    // Opt-in local command diagnostics: sender/chat JIDs, display name and stages; no raw bodies or secrets.
    let diagnostic_messages = matches!(std::env::var("MATTER_WA_DIAG").as_deref(), Ok("1"));
    // Presentation context only; scoped to authenticated chat and sender.
    let menu_views = Arc::new(Mutex::new(HashMap::<(String, String), bool>::new()));
    let supervised = resume_only && std::env::var("MATTER_WA_SUPERVISED").as_deref() == Ok("1");
    let maintenance = Arc::new(AtomicU8::new(0));
    let maintenance_for_messages = Arc::clone(&maintenance);
    let codex_session = config.session_db.clone();
    require_local_controller(config.bridge_addr).await?;
    let _lease = SessionLease::acquire(&config.session_db)?;
    let database = config
        .session_db
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Session database path is not Unicode"))?;
    let backend = SqliteStore::new(database).await?;
    let refresh_phone = config.phone.clone();
    let connected_once = Arc::new(AtomicBool::new(false));
    let connected_flag = Arc::clone(&connected_once);
    let maintenance_config = Arc::clone(&config);
    let report_in_flight = Arc::new(AtomicBool::new(false));
    let mut builder = Bot::builder().with_backend(backend).skip_history_sync();
    // Resume-only never starts the pair-code request. The server may still
    // reject stale session credentials; do not replace the original store.
    if let Some(options) = pairing_options(resume_only, config.phone.clone()) {
        builder = builder.with_pair_code(options);
    }
    let bot = builder
        // No QR event callback. Upstream may still create an internal QR
        // challenge, but this provider never displays or exports it.
        .on_pair_code(|code, expiry| async move {
            if code.len() == 8 && code.bytes().all(|b| b.is_ascii_alphanumeric()) {
                println!(
                    "Authorized pairing code: {code} ({} seconds)",
                    expiry.as_secs()
                );
            } else {
                eprintln!("Provider returned an invalid pairing code");
            }
        })
        .on_pair_code_error(|failure, _client| async move {
            // Never dump raw provider errors: they may contain account identifiers.
            // A rejected code is NOT proof of throttling unless the server says so.
            use whatsapp_rust::pair_code::PairCodeRejection;
            match failure.rejection {
                Some(PairCodeRejection::BadRequest | PairCodeRejection::RateOverlimit) => {
                    if let Some(wait) = failure.backoff {
                        eprintln!(
                            "Pairing code rejected: possible rate limit; server backoff={}s. Stop retrying.",
                            wait.as_secs()
                        );
                    } else {
                        eprintln!("Pairing code rejected: possible rate limit. Stop retrying.");
                    }
                }
                Some(PairCodeRejection::FeatureNotAvailable) => {
                    eprintln!("Phone pairing is not available for this account.");
                }
                Some(PairCodeRejection::Forbidden) => {
                    eprintln!("Pairing code forbidden by the server. Do not retry.");
                }
                Some(_) => {
                    eprintln!("Pairing code rejected by the server; reason classified as other.");
                }
                None => {
                    eprintln!("Pairing code failed without a classified server rejection; check connection and existing session before retrying.");
                }
            }
        })
        .on_pair_code_refresh(move |_manual, client| {
            let phone = refresh_phone.clone();
            async move {
                if resume_only {
                    eprintln!("Resume-only: pairing-code refresh ignored; session data retained.");
                    return;
                }
                if client
                    .pair_with_code(PairCodeOptions {
                        phone_number: phone,
                        ..Default::default()
                    })
                    .await
                    .is_err()
                {
                    eprintln!("Pairing code refresh failed");
                }
            }
        })
        .on_connected(move |client| {
            let observed = Arc::clone(&connected_flag);
            let config = Arc::clone(&maintenance_config);
            let report_in_flight = Arc::clone(&report_in_flight);
            async move {
                observed.store(true, Ordering::Relaxed);
                println!("WhatsApp session connected. Base-Matter owner control available.");
                if supervised && !report_in_flight.swap(true, Ordering::AcqRel) {
                    if let Ok(Some(report)) = maintenance::restart_report(&config.session_db) {
                        if let Ok(jid) = report.chat.parse::<whatsapp_rust::Jid>() {
                            if matches!(
                                timeout(
                                    Duration::from_secs(8),
                                    client.send_text(jid, report.body)
                                )
                                .await,
                                Ok(Ok(_))
                            ) {
                                command_alerts::deliver_deferred(
                                    &client,
                                    &config.session_db,
                                    &report.chat,
                                    report.target.as_ref(),
                                    report.kind,
                                )
                                .await;
                                maintenance::clear_restart(&config.session_db);
                            }
                        }
                    }
                    report_in_flight.store(false, Ordering::Release);
                }
            }
        })
        .on_logged_out(|info| async move {
            eprintln!("{}", classify_logout(info.on_connect, info.reason));
            eprintln!("WhatsApp-Matter: session rejected or removed. Check linked devices on the phone. Preserve session.db and stop pairing attempts.");
        })
        // Only report event types from the authenticated upstream event bus.
        // No raw event payloads, phone numbers, server error bodies, or keys.
        .on_event_for(
            &[
                EventKind::Disconnected,
                EventKind::ConnectFailure,
                EventKind::ClientOutdated,
                EventKind::TemporaryBan,
                EventKind::StreamError,
            ],
            |event, _client| async move {
                if let Some(label) = safe_resume_event(event.kind()) {
                    eprintln!("{label}");
                }
            },
        )
        .on_message(move |ctx| {
            let config = Arc::clone(&config);
            let menu_views = Arc::clone(&menu_views);
            let maintenance = Arc::clone(&maintenance_for_messages);
            async move {
                let received_at = Instant::now();
                let source = &ctx.info.source;
                // Match across PN/LID with the linked client's verified
                // identities, stripping the per-device suffix on all JIDs.
                let chat = source.chat.to_non_ad_string();
                let sender = source.sender.to_non_ad_string();
                let own_pn = ctx.client.pn().map(|jid| jid.to_non_ad_string());
                let own_lid = ctx.client.lid().map(|jid| jid.to_non_ad_string());
                let selected_button = menu_button_selection(&ctx.message);
                let reply_help = help_from_quoted_message(
                    &ctx.message,
                    ctx.message.text_content().unwrap_or_default(),
                    &chat,
                );

                // Legacy edits provide edited_message directly. Newer clients
                // may carry an encrypted MESSAGE_EDIT envelope which requires
                // the parent message's messageSecret to decrypt. Never claim
                // that an unreadable edit was inspected.
                let (edit_state, inspected_text) = if let Some(protocol) =
                    ctx.message.protocol_message.as_option()
                {
                    if protocol.r#type
                        == Some(wa::message::protocol_message::Type::MESSAGE_EDIT)
                    {
                        if let Some(content) = protocol
                            .edited_message
                            .as_option()
                            .and_then(|message| message.text_content())
                        {
                            ("edited", content)
                        } else {
                            // Missing plaintext is not evidence of a
                            // successfully decoded edit; keep it opaque.
                            ("opaque", "")
                        }
                    } else {
                        ("original", ctx.message.text_content().unwrap_or_default())
                    }
                } else if whatsapp_rust::features::message_edit::extract_envelope(&ctx.message)
                    .is_some()
                {
                    ("opaque", "")
                } else {
                    ("original", ctx.message.text_content().unwrap_or_default())
                };

                if source.is_group {
                    let observation = Event {
                        chat_jid: &chat,
                        sender_jid: &sender,
                        message_id: &ctx.info.id,
                        text: inspected_text,
                        from_me: source.is_from_me,
                        is_group: true,
                        timestamp: ctx.info.timestamp.timestamp(),
                    };
                    // Passive observations are not commands. Never print a
                    // line for each group message or an expected ignored ACK.
                    // Report a transport outage once until recovery.
                    static ANTI_TRANSPORT_WARNING: AtomicBool = AtomicBool::new(false);
                    if let Ok(Some(frame)) = config.signed_anti_observation(&observation, edit_state) {
                        match deliver(config.bridge_addr, &frame, None, None).await {
                            Ok(_) => {
                                ANTI_TRANSPORT_WARNING.store(false, Ordering::Relaxed);
                            }
                            Err(_) => {
                                if !ANTI_TRANSPORT_WARNING.swap(true, Ordering::Relaxed) {
                                    eprintln!("Base-Matter: anti observation transport unavailable");
                                }
                            }
                        }
                    }
                }
                // Edited messages may be inspected but must never trigger
                // the normal control dispatcher as new executable commands.
                if edit_state != "original" {
                    return;
                }
                let data = Event {
                    chat_jid: &chat,
                    sender_jid: &sender,
                    message_id: &ctx.info.id,
                    text: selected_button
                        .as_ref()
                        .map(|(command, _)| command.as_str())
                        .or(reply_help.as_deref())
                        .or_else(|| ctx.message.text_content())
                        .unwrap_or_default(),
                    from_me: source.is_from_me,
                    is_group: source.is_group,
                    timestamp: ctx.info.timestamp.timestamp(),
                };
                if !recognized_command(data.text) {
                    return;
                }
                let origin = if selected_button.is_some() {
                    "botón"
                } else if reply_help.is_some() {
                    "cita"
                } else {
                    "texto"
                };
                let fast_ping = data.text.split_whitespace().eq(["=/>ping"]);
                let mut verified = if data.from_me {
                    config.signed_verified_control(
                        &data, own_pn.as_deref(), own_lid.as_deref(),
                    )
                } else {
                    // Sender JID is signed from WhatsApp metadata. Mutating
                    // controls remain denied unless the controller confirms
                    // this exact sender is a global owner in private Vault.
                    config.signed_public_readonly(
                        &data, own_pn.as_deref(), own_lid.as_deref(),
                    )
                };
                // For an incoming group command, resolve a LID to a PN only
                // from the authenticated group's exact participant metadata.
                // Preserve the original event/chat for routing and diagnostics;
                // sign the resolved actor again before forwarding to Base.
                // Ping is read-only for every admitted sender. Do not make
                // its answer wait for a privileged identity alias lookup.
                // Vault still verifies the signed LID directly for console rank.
                if !fast_ping && data.is_group && !data.from_me && data.sender_jid.ends_with("@lid") {
                    if let Ok(Some(frame)) = &mut verified {
                        if let Ok(Ok(metadata)) = timeout(
                            Duration::from_secs(2),
                            ctx.client.groups().get_metadata(&source.chat),
                        )
                        .await
                        {
                            let alternate = source
                                .sender_alt
                                .as_ref()
                                .map(|jid| jid.to_non_ad_string());
                            let participants = metadata.participants.iter().map(|p| {
                                [
                                    p.jid.to_non_ad_string(),
                                    p.phone_number
                                        .as_ref()
                                        .map(|j| j.to_non_ad_string())
                                        .unwrap_or_default(),
                                    p.lid
                                        .as_ref()
                                        .map(|j| j.to_non_ad_string())
                                        .unwrap_or_default(),
                                ]
                            });
                            if let Some(pn) = verified_group_sender_pn(
                                data.sender_jid,
                                alternate.as_deref(),
                                participants,
                            ) {
                                frame.actor_id = pn;
                                match whatsapp_matter_rust_provider::sign_frame(frame, &config.bridge_key) {
                                    Ok(signature) => frame.signature = signature,
                                    Err(_) => return,
                                }
                            }
                        }
                    }
                }
                // An explicit --user selects the sender, one actual mention,
                // or a quoted participant. A failed resolution must never
                // silently fall back to the sender.
                let presentation_words: Vec<_> = data.text.split_whitespace().collect();
                let presentation = presentation_words.first() == Some(&"=/>set")
                    && presentation_words.iter().skip(1).any(|word| matches!(*word, "style" | "priority"));
                if presentation {
                    if let Ok(Some(frame)) = &mut verified {
                        let personal = presentation_words.contains(&"--user");
                        let mut target = "-".to_owned();
                        let mut admin = false;
                        let mut cleaned = presentation_words.clone();
                        let base = ctx.message.get_base_message();
                        let context = base.extended_text_message.as_option().and_then(|message| message.context_info.as_option());
                        let mentions: Vec<_> = context.map(|info| info.mentioned_jid.iter().map(String::as_str).collect()).unwrap_or_default();
                        let quoted = context.filter(|info| info.quoted_message.as_option().is_some());
                        let typed_mentions = cleaned.iter().filter(|word| word.starts_with('@')).count();
                        let selected = if personal {
                            match (mentions.as_slice(), quoted) {
                                ([mention], None) if typed_mentions == 1 => {
                                    cleaned.retain(|word| !word.starts_with('@'));
                                    Some(*mention)
                                }
                                ([], Some(info)) if typed_mentions == 0
                                    && info.remote_jid.as_deref().is_none_or(|chat| chat == data.chat_jid)
                                    && info.stanza_id.as_deref().is_some_and(|id| !id.is_empty()) => {
                                    Some(info.participant.as_deref().unwrap_or("!"))
                                }
                                ([], None) if typed_mentions == 0 => None,
                                _ => Some("!"),
                            }
                        } else { None };
                        if let Some(selected) = selected { target = selected.to_owned(); }
                        if data.is_group && (selected.is_some() || !personal) {
                            match timeout(Duration::from_secs(2), ctx.client.groups().get_metadata(&source.chat)).await {
                                Ok(Ok(metadata)) => {
                                    let rows: Vec<_> = metadata.participants.iter().map(|p| {
                                        [p.jid.to_non_ad_string(),
                                            p.phone_number.as_ref().map(|j| j.to_non_ad_string()).unwrap_or_default(),
                                            p.lid.as_ref().map(|j| j.to_non_ad_string()).unwrap_or_default()]
                                    }).collect();
                                    admin = metadata.participants.iter().zip(&rows).any(|(p, ids)| p.is_admin() && ids.iter().any(|id| id == data.sender_jid));
                                    if let Some(selected) = selected {
                                        let mut matches = rows.iter().filter(|ids| ids.iter().any(|id| id == selected));
                                        target = match (matches.next(), matches.next()) {
                                            (Some(ids), None) if selected != "!" && !selected.is_empty() => {
                                                verified_group_sender_pn(selected, None, [(*ids).clone()]).unwrap_or_else(|| selected.to_owned())
                                            }
                                            _ => "!".into(),
                                        };
                                    }
                                }
                                _ => { if selected.is_some() { target = "!".into(); } }
                            }
                        } else if let Some(selected) = selected {
                            let actor = if frame.from_me { &frame.owner_jid } else { &frame.actor_id };
                            if selected != actor.as_str() { target = "!".into(); }
                        }
                        // Prevent delimiter injection even from malformed metadata.
                        if target != "-" && target != "!" && !target.strip_suffix("@lid").or_else(|| target.strip_suffix("@s.whatsapp.net")).is_some_and(|id| (8..=20).contains(&id.len()) && id.bytes().all(|b| b.is_ascii_digit())) {
                            target = "!".into();
                        }
                        frame.text = format!("\u{001f}matter-presentation:{}:{}:{}", u8::from(admin), target, cleaned.join(" "));
                        match whatsapp_matter_rust_provider::sign_frame(frame, &config.bridge_key) {
                            Ok(signature) => frame.signature = signature,
                            Err(_) => return,
                        }
                    }
                }
                let socket_owner = matches!(&verified, Ok(Some(frame)) if frame.from_me);
                let mut trace = CommandTrace::new(diagnostic_messages, received_at);
                if !fast_ping {
                    trace.populate_header(&ctx, &data, origin, socket_owner).await;
                }
                if let Ok(Some(frame)) = verified {
                    trace.stage("IPC: ENVIANDO");
                    let ipc_started = Instant::now();
                    let mut global_owner = false;
                    let mut plain_style = false;
                    let ack = deliver(config.bridge_addr, &frame, Some(&mut global_owner), Some(&mut plain_style)).await;
                    let ipc_elapsed = ipc_started.elapsed();
                    if fast_ping && matches!(
                        &ack,
                        Ok(ControlAck::Applied
                            | ControlAck::AppliedWithMessage(_)
                            | ControlAck::AppliedWithMenuMode { .. }
                            | ControlAck::AppliedWithMenuItems { .. })
                    ) {
                        trace.stage(&format!("IPC: OK {}ms", ipc_elapsed.as_millis()));
                        // Measure the entire callback up to response preparation,
                        // including Anti observation, signing and controller RTT.
                        // Diagnostic metadata and the reaction are sent afterward.
                        let process = received_at.elapsed().saturating_sub(ipc_elapsed);
                        let pong = if plain_style {
                            plain_ping_message(ipc_elapsed, process)
                        } else {
                            matter_ping_message(ipc_elapsed, process)
                        };
                        let sent = matches!(
                            trace.send("PING", ctx.reply(&pong)).await,
                            Ok(Ok(_))
                        );
                        let state = if sent {
                            "ENVÍO ACEPTADO (render no verificado)"
                        } else {
                            "SIN ENVÍO CONFIRMADO"
                        };
                        if let Some(kind) = command_alerts::completion(state) {
                            command_alert(&trace, &ctx, &config.session_db, kind).await;
                        }
                        // Fresh Vault rank only for local console diagnostics,
                        // after Pong. The HMAC binds the original sender/chat.
                        if diagnostic_messages {
                            let mut rank_frame = frame.clone();
                            rank_frame.text = "\u{001f}matter-ping-rank".to_owned();
                            if let Ok(signature) = whatsapp_matter_rust_provider::sign_frame(&rank_frame, &config.bridge_key) {
                                rank_frame.signature = signature;
                                let _ = deliver(config.bridge_addr, &rank_frame, Some(&mut global_owner), None).await;
                            }
                        }
                        trace.populate_header(&ctx, &data, origin, socket_owner).await;
                        trace.print_header(global_owner);
                        trace.finish(state);
                        return;
                    }
                    if fast_ping {
                        trace.populate_header(&ctx, &data, origin, socket_owner).await;
                    }
                    trace.print_header(global_owner);
                    match ack {
                        Ok(ack @ (
                            ControlAck::Applied
                            | ControlAck::AppliedWithMessage(_)
                            | ControlAck::AppliedWithMenuMode { .. }
                            | ControlAck::AppliedWithMenuItems { .. }
                        )) => {
                            let ipc_ms = ipc_elapsed.as_millis();
                            trace.stage(&format!("IPC: OK {ipc_ms}ms"));
                            if !diagnostic_messages {
                                println!("Base-Matter: comando aceptado");
                            }
                            // Begin only after a successful authenticated admission.
                            // Alert settings themselves use the newly selected policy.
                            if !frame.text.split_whitespace().take(2).eq(["=/>set", "alerts"]) {
                                command_alert(&trace, &ctx, &config.session_db, command_alerts::Kind::Processing).await;
                            }
                            // Only the signed, admitted controller ACK authorizes these paths.
                            if frame.text.split_whitespace().take(2).eq(["=/>set", "alerts"]) {
                                match command_alerts::configure(&config.session_db, &frame.text)
                                    .and_then(|body| maintenance::styled("Set Alerts", &body, plain_style))
                                {
                                    Ok(body) => {
                                        let sent = matches!(trace.send("TEXTO", ctx.reply(&body)).await, Ok(Ok(_)));
                                        finish_command(&trace, &ctx, &config.session_db, if sent { "ENVÍO ACEPTADO" } else { "SIN ENVÍO CONFIRMADO" }).await;
                                    }
                                    Err(_) => {

                                        finish_command(&trace, &ctx, &config.session_db, "ERROR CONFIGURACIÓN").await;
                                    }
                                }
                                return;
                            }
                            if frame.text.split_whitespace().eq(["=/>reload"]) {
                                let body = match menu_views.lock() {
                                    Ok(mut views) => {
                                        let count = views.len();
                                        views.clear();
                                        format!("Estado: Recarga parcial\nÁmbito: socket actual\nVistas de comandos renovadas: {count}\nCódigo Rust nuevo: requiere =/>restart\nSesión: sin reinicio.")
                                    }
                                    Err(_) => "Estado: Fallido\nMotivo: registro de vistas no disponible.".into(),
                                };
                                if body.starts_with("Estado: Fallido") {

                                    finish_command(&trace, &ctx, &config.session_db, "MANTENIMIENTO FALLIDO").await;
                                    return;
                                }
                                if let Ok(message) = maintenance::styled("Reload", &body, plain_style) {
                                    let sent = matches!(trace.send("RELOAD", ctx.reply(&message)).await, Ok(Ok(_)));
                                    finish_command(&trace, &ctx, &config.session_db, if sent { "RECARGA PARCIAL" } else { "SIN ENVÍO CONFIRMADO" }).await;
                                } else {
                                    finish_command(&trace, &ctx, &config.session_db, "ERROR TEMA").await;
                                }
                                return;
                            }
                            if frame.text.split_whitespace().next() == Some("=/>codex") {
                                let instruction = frame.text.strip_prefix("=/>codex").unwrap_or("").trim().to_owned();
                                let path = config.session_db.clone();
                                let chat = frame.chat_jid.clone();
                                let body = if instruction.is_empty() {
                                    "Uso: =/>codex <instrucción>\nConsultar: =/>codex status\nDestino: Base-Matter / alpha.".into()
                                } else if resume_only {
                                    let target = command_alerts::DeferredTarget::capture(&ctx, trace.processing_reaction.load(Ordering::Relaxed));
                                    tokio::task::spawn_blocking(move || codex_bridge::command(&path, &chat, &instruction, plain_style, target))
                                        .await.ok().and_then(|result| result.ok())
                                        .unwrap_or_else(|| "Estado: Bloqueado\nTrabajador aislado no disponible o tarea pendiente.\nNo se confirma ninguna modificación.".into())
                                } else {
                                    "Estado: Bloqueado\nRequiere el runtime alpha existente.".into()
                                };
                                if body.starts_with("Estado: Bloqueado") || body.starts_with("Estado: Fallido") || body.starts_with("Estado: Envío no confirmado") {
                                    trace.stage("CODEX: TRABAJADOR NO DISPONIBLE O TAREA NO CONFIRMADA");

                                    finish_command(&trace, &ctx, &config.session_db, "CODEX FALLÓ").await;
                                    return;
                                }
                                if let Ok(message) = maintenance::styled("Codex", &body, plain_style) {
                                    let pending = data.text.split_whitespace().nth(1) != Some("status")
                                        && (body.starts_with("Estado: En cola")
                                            || body.starts_with("Estado: Codex trabajando"));
                                    let warning = body.starts_with("Estado: Tarea pendiente")
                                        || body.starts_with("Estado: Pull request cerrado")
                                        || body.starts_with("Estado: Tarea no encontrada");
                                    let review = body.starts_with("Estado: Pull request preparado");
                                    let sent = matches!(trace.send("CODEX", ctx.reply(&message)).await, Ok(Ok(_)));
                                    finish_command(&trace, &ctx, &config.session_db, if !sent { "SIN ENVÍO CONFIRMADO" } else if pending { "CODEX EN CURSO" } else if review || warning { "CODEX EN REVISIÓN" } else { "ENVÍO ACEPTADO" }).await;
                                } else {
                                    finish_command(&trace, &ctx, &config.session_db, "ERROR TEMA").await;
                                }
                                return;
                            }
                            // The signed, admitted global-owner command is the only entry.
                            if frame.text.split_whitespace().take(2).eq(["=/>set", "update"]) {
                                let result = if maintenance.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                                    let result = maintenance::set_update(&config.session_db, &frame.text);
                                    maintenance.store(0, Ordering::Release);
                                    result
                                } else {
                                    Err(anyhow::anyhow!("Maintenance busy"))
                                };
                                let (body, state) = match result {
                                    Ok(body) => (body, "MANTENIMIENTO COMPLETADO"),
                                    Err(_) => ("Estado: Bloqueado\nMotivo: mantenimiento en curso o configuración privada no disponible.".into(), "MANTENIMIENTO BLOQUEADO"),
                                };
                                if state == "MANTENIMIENTO BLOQUEADO" {

                                    finish_command(&trace, &ctx, &config.session_db, state).await;
                                    return;
                                }
                                if let Ok(body) = maintenance::styled("Set Update", &body, plain_style) {
                                    let sent = matches!(trace.send("MANTENIMIENTO", ctx.reply(&body)).await, Ok(Ok(_)));
                                    finish_command(&trace, &ctx, &config.session_db, if sent { state } else { "SIN ENVÍO CONFIRMADO" }).await;
                                } else {
                                    finish_command(&trace, &ctx, &config.session_db, "ERROR TEMA").await;
                                }
                                return;
                            }
                            if let Some(code) = maintenance_code(&frame.text) {
                                let mut action = maintenance::UpdateAction::None;
                                let (body, state) = if !supervised {
                                    ("Estado: Bloqueado\nMotivo: supervisor local no disponible.".to_owned(), "MANTENIMIENTO BLOQUEADO")
                                } else if maintenance.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_err() {
                                    ("Estado: Bloqueado\nMotivo: mantenimiento en curso.".to_owned(), "MANTENIMIENTO BLOQUEADO")
                                } else if code == 76 {
                                    let policy = maintenance::update_policy(&config.session_db);
                                    let report = if policy.is_ok() {
                                        tokio::task::spawn_blocking(maintenance::update_source).await.ok()
                                    } else {
                                        None
                                    };
                                    let (body, state) = report.unwrap_or_else(|| ("Estado: Fallido\nMotivo: configuración o tarea de actualización no disponible.".into(), "MANTENIMIENTO FALLIDO"));
                                    let eligible = matches!(state, "MANTENIMIENTO COMPLETADO" | "MANTENIMIENTO SIN CAMBIOS")
                                        && maintenance::activation_pending();
                                    action = match policy.as_deref() {
                                        Ok("restart") if eligible => {
                                            match maintenance::request_restart(&config.session_db, &frame.chat_jid, plain_style, Some(&body), command_alerts::DeferredTarget::capture(&ctx, trace.processing_reaction.load(Ordering::Relaxed))) {
                                                Ok(()) => {
                                                    let action = maintenance::UpdateAction::Note("Activación: Reinicio autorizado.".into());
                                                    let verification = (state == "MANTENIMIENTO COMPLETADO").then_some("Integridad: OK");
                                                    if diagnostic_messages {
                                                        print!("{}", maintenance::update_console_rows(&body, verification, &action));
                                                    }
                                                    let sent = match maintenance::update_message(&body, verification, &action, plain_style) {
                                                        Ok(message) => matches!(trace.send("MANTENIMIENTO", ctx.reply(&message)).await, Ok(Ok(_))),
                                                        Err(_) => false,
                                                    };
                                                    if sent {
                                                        finish_command(&trace, &ctx, &config.session_db, "MANTENIMIENTO AUTORIZADO").await;
                                                        maintenance.store(75, Ordering::Release);
                                                    } else {
                                                        maintenance::clear_restart(&config.session_db);
                                                        maintenance.store(0, Ordering::Release);
                                                        finish_command(&trace, &ctx, &config.session_db, "SIN ENVÍO CONFIRMADO").await;
                                                    }
                                                    return;
                                                }
                                                Err(_) => maintenance::UpdateAction::Note("Activación: Bloqueada; informe pendiente o estado privado no disponible.".into()),
                                            }
                                        }
                                        Ok("reload") if eligible => {
                                            let count = menu_views.lock().map(|mut views| {
                                                let count = views.len();
                                                views.clear();
                                                count
                                            });
                                            match count {
                                                Ok(count) => maintenance::UpdateAction::Reload(format!("Estado: Recarga parcial\nVistas renovadas: {count}\nCódigo Rust: pendiente de =/>restart.")),
                                                Err(_) => maintenance::UpdateAction::Note("Recarga: Fallida; no se renovaron las vistas.".into()),
                                            }
                                        }
                                        Ok(_) if eligible => maintenance::UpdateAction::Note("Activar: =/>restart".into()),
                                        Ok(_) => maintenance::UpdateAction::None,
                                        Err(_) => maintenance::UpdateAction::Note("Activación: Bloqueada; política no disponible.".into()),
                                    };
                                    maintenance.store(0, Ordering::Release);
                                    (body, state)
                                } else {
                                    match maintenance::request_restart(&config.session_db, &frame.chat_jid, plain_style, None, command_alerts::DeferredTarget::capture(&ctx, trace.processing_reaction.load(Ordering::Relaxed))) {
                                        Ok(()) => {
                                            let body = maintenance::styled("Restart", "Estado: Autorizado\nSe verificará la versión pendiente y se conservará la sesión WA.", plain_style);
                                            let sent = match body {
                                                Ok(body) => matches!(trace.send("MANTENIMIENTO", ctx.reply(&body)).await, Ok(Ok(_))),
                                                Err(_) => false,
                                            };
                                            if sent {
                                                finish_command(&trace, &ctx, &config.session_db, "MANTENIMIENTO AUTORIZADO").await;
                                                maintenance.store(75, Ordering::Release);
                                            } else {
                                                maintenance::clear_restart(&config.session_db);
                                                maintenance.store(0, Ordering::Release);
                                                finish_command(&trace, &ctx, &config.session_db, "SIN ENVÍO CONFIRMADO").await;
                                            }
                                            return;
                                        }
                                        Err(_) => {
                                            maintenance.store(0, Ordering::Release);
                                            ("Estado: Bloqueado\nMotivo: informe anterior pendiente o estado privado no disponible.".into(), "MANTENIMIENTO BLOQUEADO")
                                        }
                                    }
                                };
                                if matches!(state, "MANTENIMIENTO FALLIDO" | "MANTENIMIENTO BLOQUEADO") {
                                    if diagnostic_messages {
                                        for line in body.lines() {
                                            println!("{}", console_row(line, false));
                                        }
                                    }

                                    finish_command(&trace, &ctx, &config.session_db, state).await;
                                    return;
                                }
                                let partial_reload = matches!(&action, maintenance::UpdateAction::Reload(_));
                                let title = if code == 76 { "Update" } else { "Restart" };
                                let message = if code == 76 {
                                    let verification = (state == "MANTENIMIENTO COMPLETADO").then_some("Integridad: OK");
                                    if diagnostic_messages {
                                        print!("{}", maintenance::update_console_rows(&body, verification, &action));
                                    }
                                    maintenance::update_message(&body, verification, &action, plain_style)
                                } else {
                                    if diagnostic_messages {
                                        for line in body.lines() {
                                            println!("{}", console_row(line, false));
                                        }
                                    }
                                    maintenance::styled(title, &body, plain_style)
                                };
                                if let Ok(body) = message {
                                    let sent = matches!(trace.send("MANTENIMIENTO", ctx.reply(&body)).await, Ok(Ok(_)));
                                    finish_command(&trace, &ctx, &config.session_db, if !sent { "SIN ENVÍO CONFIRMADO" } else if partial_reload { "RECARGA PARCIAL" } else { state }).await;
                                } else {
                                    finish_command(&trace, &ctx, &config.session_db, "ERROR TEMA").await;
                                }
                                return;
                            }
                            // Only an authorized signed control may receive a
                            // response. External senders require independent
                            // Vault-global-owner authorization for mutations.
                            let ipc_already_templated = matches!(
                                &ack,
                                ControlAck::AppliedWithMessage(_)
                                    | ControlAck::AppliedWithMenuMode { .. }
                                    | ControlAck::AppliedWithMenuItems { .. }
                            );
                            // This value comes from a successful authenticated IPC reply,
                            // never from user-authored WhatsApp text.
                            let preferred_menu_mode = match &ack {
                                ControlAck::AppliedWithMenuMode { mode, .. } => Some(mode.clone()),
                                ControlAck::AppliedWithMenuItems { mode, .. } => mode.clone(),
                                _ => None,
                            };
                            let menu_items = match &ack {
                                ControlAck::AppliedWithMenuItems { items, .. } => items.clone(),
                                _ => Vec::new(),
                            };
                            let debug_requested = frame.text.ends_with(" --debug");
                            let command_text = frame.text
                                .strip_prefix("\u{001f}matter-group-style:")
                                .unwrap_or(&frame.text);
                            let actual_command = command_text
                                .strip_suffix(" --debug")
                                .unwrap_or(command_text);
                            let response = if actual_command == "=/>start" {
                                match timeout(
                                    Duration::from_secs(20),
                                    discover_register(&ctx, &config.owner_jid),
                                )
                                .await
                                {
                                    Ok(Ok(report)) => report,
                                    _ => "Base-Matter REGISTER: inspección no disponible o incompleta; no se hicieron cambios.".to_owned(),
                                }
                            } else if actual_command == "=/>add++" {
                                match timeout(
                                    Duration::from_secs(20),
                                    preview_community_recovery(&ctx, &config.owner_jid),
                                )
                                .await
                                {
                                    Ok(Ok(report)) => report,
                                    _ => "Base-Matter: inventario de recuperación incompleto; no se copiaron miembros.".to_owned(),
                                }
                            } else if let Some(community_id) =
                                actual_command.strip_prefix("=/>add++ snapshot ")
                            {
                                match timeout(
                                    Duration::from_secs(45),
                                    snapshot_community_recovery(&ctx, &config, community_id),
                                )
                                .await
                                {
                                    Ok(Ok(report)) => report,
                                    _ => "Base-Matter: no se guardó la instantánea. Verifica permisos admin, comunidad activa y clave de recuperación.".to_owned(),
                                }
                            } else {
                                match ack {
                                    ControlAck::AppliedWithMessage(text) => text,
                                    ControlAck::AppliedWithMenuMode { message, .. }
                                    | ControlAck::AppliedWithMenuItems { message, .. } => message,
                                    _ => "Base-Matter: solicitud procesada.".to_owned(),
                                }
                            };
                            // Never mistake a typed ACK for a real button click.
                            // The controller already verified this callback.
                            let response = if selected_button.is_some()
                                && actual_command.starts_with("=/>test ack ")
                            {
                                format!(
                                    "✅ Botón verificado: `{actual_command}`\n                                     WhatsApp entregó una respuesta interactiva                                      autenticada. Sin ejecutar acciones."
                                )
                            } else {
                                response
                            };
                            let response = if debug_requested
                                && (actual_command == "=/>start"
                                    || actual_command == "=/>add++"
                                    || actual_command.starts_with("=/>add++ snapshot "))
                            {
                                format!(
                                    "{response}\n[DEBUG] plataforma=WhatsApp; contexto={};                                      identidad=verificada; transporte=HMAC; secretos=ocultos",
                                    if data.is_group { "grupo" } else { "privado" }
                                )
                            } else {
                                response
                            };
                            // The IPC already used the verified chat's selected
                            // template. Never overwrite a customized group layout
                            // with the provider's fallback frame. Only responses
                            // generated here (REGISTER/recovery/fallback) need it.
                            let provider_generated =
                                actual_command == "=/>start"
                                    || actual_command == "=/>add++"
                                    || actual_command.starts_with("=/>add++ snapshot ");
                            let response = if plain_style || (ipc_already_templated && !provider_generated) {
                                response
                            } else {
                                match render_provider_control(&response) {
                                    Ok(formatted) => formatted,
                                    Err(_) => {
                                        eprintln!("Base-Matter: control response rejected by theme");
                                        finish_command(&trace, &ctx, &config.session_db, "ERROR TEMA").await;
                                        return;
                                    }
                                }
                            };
                            if actual_command == "=/>test ui"
                                && native_buttons
                            {
                                if let Ok(probe) = native_ui_probe_message() {
                                    if matches!(
                                        trace.send("UI", ctx.send_message(probe)).await,
                                        Ok(Ok(_))
                                    ) {
                                        finish_command(&trace, &ctx, &config.session_db, "ENVIADO (UI)").await;
                                        return;
                                    }
                                }
                                eprintln!("Base-Matter: UI test unavailable; sending text instructions");
                            }
                            if actual_command == "=/>test row"
                            {
                                if matches!(
                                    trace.send("ROW", ctx.send_message(legacy_row_probe_message())).await,
                                    Ok(Ok(_))
                                ) {
                                    finish_command(&trace, &ctx, &config.session_db, "ENVIADO (ROW)").await;
                                    return;
                                }
                                eprintln!("Base-Matter: classic ButtonsRow test rejected; using text");
                            }
                            // The controller has already admitted a signed
                            // group test, or the verified owner self-chat.
                            // No private third-party HTML injection is allowed.
                            if actual_command == "=/>test html"
                                && rich_html_trial
                                && frame.from_me
                                && (data.is_group || frame.chat_jid == config.owner_jid)
                            {
                                if let Ok(message) = experimental_self_chat_rich_message(&response)
                                {
                                    match trace.send("HTML TEST", ctx.send_message(message)).await {
                                        Ok(Ok(_)) => {
                                            // A successful WhatsApp send is not proof that
                                            // Android rendered the experimental HTML.
                                            // Every admitted visual test receives a
                                            // diagnostic, including enabled groups.
                                            println!("Base-Matter: Rich HTML test submitted; client rendering unverified");
                                            let _ = trace.send("AVISO HTML", ctx.reply("Matter HTML: envío aceptado por el proveedor. Confirma si ves la tarjeta HTML; WhatsApp puede ocultarla.")).await;
                                            finish_command(&trace, &ctx, &config.session_db, "HTML ENVIADO (render no verificado)").await;
                                            return;
                                        }
                                        Ok(Err(_)) => eprintln!("Base-Matter: HTML test send rejected"),
                                        Err(_) => eprintln!("Base-Matter: HTML test send timed out"),
                                    }
                                }
                                eprintln!("Base-Matter: HTML test unavailable; sending text instructions");
                            }
                            let is_menu = matches!(actual_command, "=/>menu" | "=/>menú")
                                || actual_command.starts_with("=/>menu ")
                                || actual_command.starts_with("=/>menú ");
                            let is_root = matches!(actual_command, "=/>menu" | "=/>menú");
                            // Classic and buttons use one NativeFlow card.
                            // Web uses actual RichResponse plus independent native
                            // navigation; do not disguise classic text as HTML.
                            let explicit_classic = matches!(actual_command, "=/>menu classic" | "=/>menú classic" | "=/>menu matter" | "=/>menú matter");
                            let explicit_web = matches!(actual_command, "=/>menu web" | "=/>menú web" | "=/>menu modern" | "=/>menú modern");
                            let explicit_buttons = matches!(actual_command, "=/>menu buttons" | "=/>menú buttons");
                            let view_key = (chat.clone(), sender.clone());
                            let prior_buttons = menu_views.lock().ok()
                                .and_then(|views| views.get(&view_key).copied())
                                .unwrap_or(false);
                            let category_buttons = is_menu
                                && !is_root
                                && !explicit_classic
                                && !explicit_web
                                && !explicit_buttons
                                && (prior_buttons || selected_button.as_ref().is_some_and(|(_, buttons)| *buttons));
                            let is_buttons = explicit_buttons
                                || (is_root && preferred_menu_mode.as_deref() == Some("buttons"))
                                || category_buttons;
                            let is_web = explicit_web
                                || (is_root && preferred_menu_mode.as_deref().unwrap_or("web") == "web");
                            if is_menu {
                                if let Ok(mut views) = menu_views.lock() {
                                    if views.len() >= 1024 && !views.contains_key(&view_key) {
                                        views.clear();
                                    }
                                    views.insert(view_key, is_buttons);
                                }
                            }
                            let is_classic = explicit_classic
                                || (is_root && matches!(preferred_menu_mode.as_deref(), Some("classic" | "matter")));
                            let mut web_submitted = false;
                            let mut web_uncertain = false;
                            if is_web {
                                if let Some(reason) = web_html_skip_reason(
                                    rich_html_trial,
                                    rich_html_autosend,
                                    is_root,
                                    data.is_group,
                                    frame.from_me,
                                    frame.chat_jid == config.owner_jid,
                                ) {
                                    trace.stage(&format!("WEB: {reason}"));
                                } else {
                                    match styled_rich_message(&response, plain_style) {
                                        Ok(message) => match trace.send("HTML", ctx.send_message(message)).await {
                                            Ok(Ok(_)) => web_submitted = true,
                                            Ok(Err(_)) => trace.stage("WEB: HTML RECHAZADO"),
                                            Err(_) => {
                                                web_uncertain = true;
                                                trace.stage("WEB: HTML TIMEOUT (sin reintento)");
                                            }
                                        },
                                        Err(_) => trace.stage("WEB: HTML NO CONSTRUIDO (validación/tamaño)"),
                                    }
                                }
                            }
                            let web_fallback = is_web && !web_submitted && !web_uncertain;
                            // Buttons never copy the classic body. The last card is
                            // the independent format selector, not another category list.
                            if is_buttons && native_buttons {
                                let title = if is_root || explicit_buttons {
                                    "Menu Buttons"
                                } else {
                                    actual_command.strip_prefix("=/>menu ")
                                        .or_else(|| actual_command.strip_prefix("=/>menú "))
                                        .unwrap_or("Menu Buttons")
                                };
                                for card in button_menu_cards(&menu_items, title) {
                                    match trace.send("TARJETA", ctx.send_message(card)).await {
                                        Ok(Ok(_)) => {}
                                        _ => {
                                            eprintln!("Base-Matter: menu button card unavailable");
                                            break;
                                        }
                                    }
                                }
                            }
                            let sent = if is_menu && native_buttons {
                                let navigation_request = if is_buttons {
                                    "=/>menu buttons"
                                } else if is_classic && is_root {
                                    "=/>menu classic"
                                } else if is_web {
                                    "=/>menu web"
                                } else {
                                    actual_command
                                };
                                let navigation_body = if is_web || is_buttons {
                                    "Matter"
                                } else {
                                    &response
                                };
                                match native_navigation_message_with_web_fallback(
                                    navigation_body,
                                    navigation_request,
                                    web_fallback,
                                ) {
                                    Ok(message) => match trace.send("NAVEGACIÓN", ctx.send_message(message)).await {
                                        Ok(Ok(_)) => true,
                                        Ok(Err(_)) => {
                                            eprintln!("Base-Matter: unified menu rejected; attempting a single text fallback");
                                            false
                                        }
                                        Err(_) => {
                                            eprintln!("Base-Matter: unified menu timed out; refusing duplicate reply");
                                            // A timed-out send may already have been accepted.
                                            finish_command(&trace, &ctx, &config.session_db, "TIMEOUT NAVEGACIÓN (sin duplicar)").await;
                                            return;
                                        }
                                    },
                                    Err(_) => {
                                        eprintln!("Base-Matter: unified menu could not be constructed");
                                        false
                                    }
                                }
                            } else {
                                false
                            };
                            let sent = if !sent {
                                if is_web {
                                    matches!(
                                        trace.send("AVISO WEB", ctx.reply(web_navigation_notice(web_fallback))).await,
                                        Ok(Ok(_))
                                    ) || web_submitted
                                } else {
                                    let fallback = if is_buttons {
                                        "Los botones no están disponibles en este cliente."
                                    } else {
                                        &response
                                    };
                                    matches!(
                                        trace.send("TEXTO", ctx.reply(fallback)).await,
                                        Ok(Ok(_))
                                    )
                                }
                            } else {
                                true
                            };
                            if web_fallback && sent {
                                trace.stage("WEB: AVISO Y ACCESO A CLASSIC");
                            }
                            finish_command(&trace, &ctx, &config.session_db, if sent {
                                "ENVÍO ACEPTADO (render no verificado)"
                            } else {
                                "SIN ENVÍO CONFIRMADO"
                            }).await;
                            if sent && is_private_vault_command(&frame, &config.owner_jid) {
                                println!("Base-Matter: Vault owner response submitted to WhatsApp");
                            }
                        }
                        Ok(ControlAck::Duplicate) => {
                            trace.stage("IPC: DUPLICADO");
                            finish_command(&trace, &ctx, &config.session_db, "DUPLICADO").await;
                        }
                        Ok(ControlAck::Ignored) => {
                            trace.stage("IPC: IGNORADO");

                            finish_command(&trace, &ctx, &config.session_db, "IGNORADO").await;
                        }
                        Err(error) => {
                            trace.stage("IPC: DENEGADO O NO DISPONIBLE");
                            if !diagnostic_messages {
                                eprintln!("Base-Matter control denied or unreachable");
                            }
                            if frame.text.split_whitespace().next() == Some("=/>codex") {
                                trace.stage("CODEX: COMPROBAR OWNER PRINCIPAL Y VAULT ACTUALIZADO");
                            }

                            finish_command(&trace, &ctx, &config.session_db, if error.downcast_ref::<std::io::Error>().is_some() {
                                "IPC ERROR TRANSPORTE"
                            } else {
                                "IPC FALLÓ"
                            }).await;
                        }
                    }
                } else {
                    if fast_ping {
                        trace.populate_header(&ctx, &data, origin, socket_owner).await;
                    }
                    trace.print_header(false);
                    trace.stage("IPC: NO ADMITIDO");
                    // Native reaction to the original callback; no command
                    // execution, disclosure of permissions, or text fallback.

                    finish_command(&trace, &ctx, &config.session_db, "NO ADMITIDO (firma o política local)").await;
                }
            }
        })
        .build()
        .await?;
    if resume_only {
        let local_identity = bot.client();
        println!(
            "WhatsApp-Matter Rust: attempting existing session only; no pairing code requested."
        );
        println!(
            "WhatsApp stored identity signals: PN={}, LID={}; server authentication unverified.",
            local_identity.pn().is_some(),
            local_identity.lid().is_some()
        );
    } else {
        println!("WhatsApp-Matter Rust session starting. Pairing code only.");
    }
    let maintenance_exit = async {
        loop {
            let code = maintenance.load(Ordering::Acquire);
            if matches!(code, 75 | 76) {
                return code;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    };
    let codex_client = bot.client();
    let codex_notices = async {
        loop {
            tokio::time::sleep(Duration::from_secs(20)).await;
            if !resume_only || !connected_once.load(Ordering::Relaxed) {
                continue;
            }
            let path = codex_session.clone();
            let notice = tokio::task::spawn_blocking(move || codex_bridge::notice(&path))
                .await
                .ok()
                .and_then(|result| result.ok())
                .flatten();
            if let Some(notice) = notice {
                if let Ok(jid) = notice.chat.parse::<whatsapp_rust::Jid>() {
                    if matches!(
                        timeout(
                            Duration::from_secs(8),
                            codex_client.send_text(jid, notice.body)
                        )
                        .await,
                        Ok(Ok(_))
                    ) {
                        if let Some(kind) = notice.kind {
                            command_alerts::deliver_deferred(
                                &codex_client,
                                &codex_session,
                                &notice.chat,
                                notice.target.as_ref(),
                                kind,
                            )
                            .await;
                        }
                        let _ = codex_bridge::ack_notice(&codex_session, &notice.marker);
                    }
                }
            }
        }
    };
    tokio::select! {
        _ = bot.run() => {},
        code = maintenance_exit => return Ok(ExitCode::from(code)),
        _ = codex_notices => {},
    }
    if resume_only {
        if connected_once.load(Ordering::Relaxed) {
            bail!("Resume-only: WhatsApp disconnected after authentication. No pairing was requested.");
        }
        bail!("Resume-only: WhatsApp stopped before authentication. Inspect the device identity locally and check phone linked devices. No pairing was requested.");
    }
    Ok(ExitCode::SUCCESS)
}

fn maintenance_code(command: &str) -> Option<u8> {
    match command.split_whitespace().collect::<Vec<_>>().as_slice() {
        ["=/>restart"] => Some(75),
        ["=/>update"] => Some(76),
        _ => None,
    }
}

#[cfg(test)]
mod readiness_tests {
    use super::*;

    #[test]
    fn maintenance_codes_accept_only_fixed_verbs() {
        assert_eq!(maintenance_code("=/>restart"), Some(75));
        assert_eq!(maintenance_code("=/>update"), Some(76));
        assert_eq!(maintenance_code("=/>update beta"), None);
        assert_eq!(maintenance_code("=/>restart --global"), None);
        assert_eq!(maintenance_code("=/>help update"), None);
    }

    #[test]
    fn console_rows_use_independent_templates_without_reinterpreting_body() {
        assert_eq!(console_row("CMD =/>ping", false), "│·♪·╠[ CMD =/>ping");
        assert_eq!(console_row("TX: OK", true), "│·♪·╚[ TX: OK");
        assert_eq!(
            console_row("{{CLASSIC_HEADER}}", false),
            "│·♪·╠[ {{CLASSIC_HEADER}}"
        );
    }

    #[test]
    fn console_footer_aligns_with_header_without_changing_chat_frame() {
        let header = classic_template::console_template("header");
        let footer = classic_template::console_template("footer").trim_end();
        assert_eq!(header.chars().position(|c| c == '💠'), Some(3));
        assert_eq!(footer, "╰──╮╭───────⟨ 🌀");
        assert_eq!(
            classic_template::expand("{{CLASSIC_JOINED_CLOSE}}"),
            "╰─╮╭────────⟨ 🌀"
        );
    }

    #[test]
    fn admitted_group_menu_can_attempt_html_for_non_socket_owner() {
        for (enabled, autosend, root, group, from_me, owner_chat, allowed) in [
            (true, true, true, true, false, false, true),
            (true, false, false, true, false, false, true),
            (true, true, true, false, true, true, true),
            (true, true, true, false, false, false, false),
            (true, false, true, true, false, false, false),
            (false, true, false, true, false, false, false),
        ] {
            let reason = web_html_skip_reason(enabled, autosend, root, group, from_me, owner_chat);
            assert_eq!(reason.is_none(), allowed);
        }
    }

    #[test]
    fn plain_ping_keeps_all_measured_values() {
        assert_eq!(
            plain_ping_message(Duration::from_micros(12_345), Duration::from_micros(678)),
            "Pong\nSocket: 12.345 ms\nProcess: 0.678 ms\nTotal: 13.023 ms"
        );
    }

    #[test]
    fn official_ping_uses_measured_durations_and_one_frame() {
        let pong = matter_ping_message(Duration::from_micros(12_345), Duration::from_micros(678));
        assert_eq!(
            pong,
            "╭─ 💠 𝐏𝐨𝐧𝐠\n│·♪·╠[ ⴵ · 𝐒𝐨𝐜𝐤𝐞𝐭 ···· *12.345 ms*\n│·♪·╠[ ⚙︎  𝐏𝐫𝐨𝐜𝐞𝐬𝐬 · *0.678 ms*\n│·♪·╚[ ✧  𝐓𝐨𝐭𝐚𝐥 ········ *13.023 ms*\n╰───────────⟨ 🌀"
        );
        assert_eq!(pong.lines().count(), 5);
    }

    #[test]
    fn console_keeps_whatsapp_and_matter_ranks_independent() {
        assert_eq!(console_matter_role(true, false), "Owner global");
        assert_eq!(console_matter_role(true, true), "Owner global");
        assert_eq!(console_matter_role(false, true), "Owner socket");
        assert_eq!(console_matter_role(false, false), "Sin verificar");
        for wa_role in ["Admin WA", "Miembro WA", "Sin verificar"] {
            assert_eq!(
                classic_template::console_layout(
                    "ranks",
                    &[
                        ("WA", wa_role),
                        ("MATTER", console_matter_role(true, false))
                    ]
                ),
                format!("WA: {wa_role} · Matter: Owner global")
            );
        }
    }

    #[test]
    fn console_compact_rows_do_not_repeat_event_id() {
        let ipc = classic_template::console_layout(
            "ipc",
            &[
                ("IPC", "OK 21ms"),
                ("STATE", "ACEPTADO"),
                ("ELAPSED", "175"),
            ],
        );
        assert_eq!(ipc, "IPC:OK 21ms · ACEPTADO · 175ms");
        assert!(!ipc.contains("{{EVENT}}"));
        assert_eq!(
            classic_template::console_layout(
                "user_identity",
                &[("LABEL", "USR"), ("USER", "Test"), ("ID", "123@lid")]
            ),
            "USR Test · ID 123@lid"
        );
    }

    #[test]
    fn console_diagnostics_redact_arguments_and_control_characters() {
        assert_eq!(
            console_command("=/>set role owner --global SECRET"),
            "=/>set role owner --global …"
        );
        assert_eq!(console_command("=/>menu anti 2"), "=/>menu anti 2");
        assert_eq!(console_command("=/>help --quoted SECRET"), "=/>help …");
        assert_eq!(console_command("=/>bot group on"), "=/>bot group on");
        assert_eq!(console_command("=/>vault token=SECRET"), "=/>vault …");
        assert_eq!(
            console_identity("123456789012345@lid"),
            "123456789012345@lid"
        );
        assert_eq!(
            console_identity("120363123456789012@g.us"),
            "120363123456789012@g.us"
        );
        assert_eq!(
            console_identity("11111111111@s.whatsapp.net"),
            "11111111111@s.whatsapp.net"
        );
        assert_eq!(console_identity("bad\n@lid"), "-");
        assert_eq!(console_field("abcdefghij", 5), "abcd…");
        assert!(
            console_command("=/>menu downloads downloads downloads")
                .chars()
                .count()
                <= "=/>menu ".chars().count() + CONSOLE_ARGS_LIMIT
        );
        assert_eq!(console_command("=/>unknown-secret"), "");
        assert!(!recognized_command("=/>comando-inexistente"));
        assert!(!recognized_command("=/>"));
        assert!(!recognized_command("texto =/>ping"));
        assert!(recognized_command("=/>codex status"));
        assert!(recognized_command("=/>set alerts"));
        assert!(recognized_command("=/>menú"));
        assert_eq!(console_field("a\n\u{001b}\u{202e}b", 8), "ab");
    }

    #[test]
    fn group_pn_requires_exact_unambiguous_participant_binding() {
        let lid = "123456789012345@lid";
        let pn = "11111111111@s.whatsapp.net";
        let other = "22222222222@s.whatsapp.net";
        let row = |phone: &str| [lid.to_owned(), phone.to_owned(), lid.to_owned()];
        assert_eq!(
            verified_group_sender_pn(lid, Some(pn), [row(pn)]),
            Some(pn.to_owned())
        );
        assert_eq!(
            verified_group_sender_pn(lid, None, [row(pn)]),
            Some(pn.to_owned())
        );
        assert!(verified_group_sender_pn(lid, Some(other), [row(pn)]).is_none());
        assert!(verified_group_sender_pn(lid, None, [row(pn), row(other)]).is_none());
        assert!(verified_group_sender_pn(lid, Some(pn), [row("")]).is_none());
        assert!(verified_group_sender_pn("99999999999@lid", Some(pn), [row(pn)]).is_none());
        assert!(verified_group_sender_pn(
            lid,
            None,
            [[pn.to_owned(), other.to_owned(), lid.to_owned(),]]
        )
        .is_none());
    }

    #[test]
    fn vault_diagnostic_never_exposes_service_to_group_or_other_sender() {
        use whatsapp_matter_rust_provider::SignedFrame;
        let mut frame = SignedFrame {
            version: 1,
            socket_id: "wa-alpha".to_owned(),
            actor_id: "owner".to_owned(),
            owner_jid: "owner@s.whatsapp.net".to_owned(),
            chat_jid: "owner@s.whatsapp.net".to_owned(),
            message_id: "synthetic".to_owned(),
            text: "=/>vault".to_owned(),
            from_me: true,
            timestamp: 0,
            signature: String::new(),
        };
        assert!(is_private_vault_command(&frame, &frame.owner_jid));
        frame.text = "=/>vault test".to_owned();
        assert!(is_private_vault_command(&frame, &frame.owner_jid));
        frame.text = "=/>vaulting".to_owned();
        assert!(!is_private_vault_command(&frame, &frame.owner_jid));
        frame.text = "=/>vault".to_owned();
        frame.chat_jid = "example@g.us".to_owned();
        assert!(!is_private_vault_command(&frame, &frame.owner_jid));
        frame.chat_jid = "owner@s.whatsapp.net".to_owned();
        frame.from_me = false;
        assert!(!is_private_vault_command(&frame, &frame.owner_jid));
        frame.from_me = true;
        assert!(!is_private_vault_command(
            &frame,
            "different@s.whatsapp.net"
        ));
    }

    #[test]
    fn safe_logout_categories_do_not_expose_server_data() {
        let auth = classify_logout(true, ConnectFailureReason::LoggedOut);
        assert!(auth.starts_with("WA_LOGOUT_CLASS=AUTH_REJECTED_LOGGED_OUT"));
        let removed = classify_logout(false, ConnectFailureReason::LoggedOut);
        assert!(removed.starts_with("WA_LOGOUT_CLASS=STREAM_LOGGED_OUT"));
        let locked = classify_logout(true, ConnectFailureReason::AccountLocked);
        assert!(locked.starts_with("WA_LOGOUT_CLASS=ACCOUNT_LOCKED"));
        for label in [auth, removed, locked] {
            assert!(!label.contains('@'));
            assert!(!label.contains("token="));
            assert!(!label.contains("key="));
        }
    }

    #[test]
    fn safe_resume_event_never_emits_remote_payloads() {
        for kind in [
            EventKind::Disconnected,
            EventKind::ConnectFailure,
            EventKind::ClientOutdated,
            EventKind::TemporaryBan,
            EventKind::StreamError,
        ] {
            let label = safe_resume_event(kind).expect("diagnostic should exist");
            assert!(!label.is_empty());
            assert!(!label.contains('@'));
            assert!(!label.contains("key="));
        }
        assert!(safe_resume_event(EventKind::Messages).is_none());
    }

    #[test]
    fn resume_only_never_configures_pairing_code_requests() {
        assert!(pairing_options(true, "5215512345678".to_owned()).is_none());
        assert!(pairing_options(false, "5215512345678".to_owned()).is_some());
    }

    #[tokio::test]
    async fn controller_must_listen_before_a_whatsapp_session_is_opened() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = match listener.local_addr().unwrap() {
            std::net::SocketAddr::V4(address) => address,
            _ => unreachable!(),
        };
        require_local_controller(address).await.unwrap();
        drop(listener);
        assert!(require_local_controller(address).await.is_err());
    }
}
