/*
 * WhatsApp-Matter — isolated whatsapp-rust 0.7 pairing-code provider.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Result};
use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::{timeout, Duration},
};
use whatsapp_matter_rust_provider::{
    help_reply::help_from_quoted_message,
    menu_sound, parse_control_ack,
    recovery::{
        read_encrypted, recovery_key_from_env, save_encrypted, RecoveryMember, RecoverySnapshot,
    },
    render_provider_control,
    rich_transport::experimental_self_chat_rich_message,
    ui::{
        legacy_row_probe_message, menu_lab_card, native_menu_lab_card, native_navigation_message,
        native_ui_probe_message, readonly_button_command, MenuCardVariant,
    },
    Config, ControlAck, Event, SessionLease,
};
use whatsapp_rust::{
    download::MediaType,
    media::{self, AudioOptions},
    pair_code::PairCodeOptions,
    prelude::*,
    types::events::ConnectFailureReason,
    upload::UploadOptions,
};

async fn deliver(
    address: std::net::SocketAddrV4,
    frame: &whatsapp_matter_rust_provider::SignedFrame,
) -> Result<ControlAck> {
    let body = serde_json::to_vec(frame)?;
    if body.len() >= 16 * 1024 {
        bail!("Control frame exceeds limit");
    }
    let transport = async {
        let mut stream = TcpStream::connect(address).await?;
        stream.write_all(&body).await?;
        stream.shutdown().await?;
        let mut response = Vec::new();
        stream.take(8193).read_to_end(&mut response).await?;
        if response.len() > 8192 {
            bail!("Invalid bridge reply length");
        }
        parse_control_ack(&response)
    };
    let deadline = if frame.from_me
        && frame.text.starts_with("=/>matter ")
        && !frame.chat_jid.ends_with("@g.us")
    {
        Duration::from_secs(83)
    } else if (frame.text.split_whitespace().next() == Some("=/>vault")
        || frame.text.starts_with("=/>set role owner --global"))
        && !frame.chat_jid.ends_with("@g.us")
    {
        // Both owner-linked and external verified-owner private messages
        // require a private Vault authority check before any command.
        Duration::from_secs(12)
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

// Optional laboratory: native buttons are read-only and may be tested
// in an activated private chat. Experimental Rich HTML, legacy comparison
// and audio stay limited to a verified owner self-chat.
async fn send_menu_lab(
    ctx: &whatsapp_rust::bot::MessageContext,
    menu: &str,
    rich_html: bool,
    owner_self_chat: bool,
) {
    if owner_self_chat && rich_html {
        if let Ok(message) = experimental_self_chat_rich_message(menu) {
            if ctx.send_message(message).await.is_err() {
                eprintln!("Base-Matter: rich HTML menu experiment rejected");
            } else {
                eprintln!("Base-Matter: rich HTML send accepted; mobile rendering unverified");
            }
        } else {
            eprintln!("Base-Matter: rich HTML preview exceeds bounds");
        }
    }
    for variant in [MenuCardVariant::Ts18, MenuCardVariant::Ts19] {
        if ctx
            .send_message(native_menu_lab_card(variant))
            .await
            .is_err()
        {
            eprintln!("Base-Matter: NativeFlow test card rejected; trying legacy buttons");
            if ctx.send_message(menu_lab_card(variant)).await.is_err() {
                eprintln!("Base-Matter: legacy button card rejected");
            }
        } else if owner_self_chat {
            // Self-chat only: compare transports to identify clients that
            // silently accept a message but fail to display its buttons.
            if ctx.send_message(menu_lab_card(variant)).await.is_err() {
                eprintln!("Base-Matter: legacy comparison card rejected");
            }
        }
    }
    if !owner_self_chat {
        return;
    }
    let upload = timeout(
        Duration::from_secs(15),
        ctx.client.upload(
            menu_sound::menu_tone_bytes().to_vec(),
            MediaType::Audio,
            UploadOptions::default(),
        ),
    )
    .await;
    match upload {
        Ok(Ok(uploaded)) => {
            let sound = media::audio_message(
                uploaded,
                AudioOptions {
                    duration_seconds: Some(1),
                    ptt: Some(false),
                    ..Default::default()
                },
            );
            if ctx.send_message(sound).await.is_err() {
                eprintln!("Base-Matter: menu audio message rejected");
            }
        }
        _ => eprintln!("Base-Matter: menu audio upload unavailable"),
    }
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
async fn main() -> Result<()> {
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
            return Ok(());
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
            return Ok(());
        }
        Some("--help") => {
            println!("WhatsApp-Matter Rust provider: --check, --resume-only (restore stored identity; no pairing-code request), --rich-preview-html, --rich-preview-json, --recovery-verify <encrypted-filename>, or run for pairing code. Local Base-Matter IPC; no QR.");
            return Ok(());
        }
        Some("--check") => {
            let config = Config::from_env()?;
            SessionLease::check_available(&config.session_db)?;
            println!("whatsapp-rust pairing settings checked; no session was opened.");
            return Ok(());
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
            return Ok(());
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
    // Unverified rich-HTML transport, explicitly enabled only for
    // an authenticated owner's self-chat. No forged Meta AI envelope.
    let rich_html_trial = matches!(std::env::var("MATTER_WA_RICH_HTML").as_deref(), Ok("1"));
    let menu_lab = matches!(std::env::var("MATTER_WA_MENU_LAB").as_deref(), Ok("1"));
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
        .on_connected(move |_client| {
            let observed = Arc::clone(&connected_flag);
            async move {
                observed.store(true, Ordering::Relaxed);
                println!("WhatsApp session connected. Base-Matter owner control available.");
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
            async move {
                let source = &ctx.info.source;
                // Match across PN/LID with the linked client's verified
                // identities, stripping the per-device suffix on all JIDs.
                let chat = source.chat.to_non_ad_string();
                let sender = source.sender.to_non_ad_string();
                let own_pn = ctx.client.pn().map(|jid| jid.to_non_ad_string());
                let own_lid = ctx.client.lid().map(|jid| jid.to_non_ad_string());
                let selected_button = readonly_button_command(&ctx.message);
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
                    match config.signed_anti_observation(&observation, edit_state) {
                        Ok(Some(frame)) => {
                            if !matches!(
                                deliver(config.bridge_addr, &frame).await,
                                Ok(ControlAck::Applied | ControlAck::Duplicate)
                            ) {
                                eprintln!("Base-Matter: anti observation not confirmed");
                            }
                        }
                        Ok(None) => {}
                        Err(_) => eprintln!("Base-Matter: anti observation rejected before signing"),
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
                        .or(reply_help.as_deref())
                        .or_else(|| ctx.message.text_content())
                        .unwrap_or_default(),
                    from_me: source.is_from_me,
                    is_group: source.is_group,
                    timestamp: ctx.info.timestamp.timestamp(),
                };
                let verified = if data.from_me {
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
                // Whitelisted UI diagnostic ingress logs do not include
                // message IDs, chat IDs, phone numbers or message bodies.
                if matches!(
                    data.text,
                    "=/>test" | "=/>test ui" | "=/>test row" | "=/>test html"
                ) {
                    match &verified {
                        Ok(Some(_)) => println!("Base-Matter: visual test authenticated for current chat"),
                        Ok(None) => eprintln!("Base-Matter: visual test ignored before signing"),
                        Err(_) => eprintln!("Base-Matter: visual test signer rejected current chat"),
                    }
                }
                // Log rejections without leaking phone numbers, LIDs,
                // message bodies, sessions or cryptographic keys.
                if data.from_me && !data.is_group && data.text.starts_with("=/>") {
                    match &verified {
                        Ok(None) => eprintln!(
                            "Base-Matter: self-chat control ignored (not this account's private chat)"
                        ),
                        Err(_) => eprintln!(
                            "Base-Matter: self-chat control denied (connected account identity mismatch or invalid event)"
                        ),
                        Ok(Some(_)) => {}
                    }
                }
                match verified {
                    Ok(Some(frame)) => match deliver(config.bridge_addr, &frame).await {
                        Ok(ack @ (ControlAck::Applied | ControlAck::AppliedWithMessage(_))) => {
                            println!("Base-Matter owner control applied");
                            // Only an authorized signed control may receive a
                            // response. External senders require independent
                            // Vault-global-owner authorization for mutations.
                            let ipc_already_templated =
                                matches!(&ack, ControlAck::AppliedWithMessage(_));
                            let debug_requested = frame.text.ends_with(" --debug");
                            let actual_command = frame.text
                                .strip_suffix(" --debug")
                                .unwrap_or(&frame.text);
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
                            let response = if ipc_already_templated && !provider_generated {
                                response
                            } else {
                                match render_provider_control(&response) {
                                    Ok(formatted) => formatted,
                                    Err(_) => {
                                        eprintln!("Base-Matter: control response rejected by theme");
                                        return;
                                    }
                                }
                            };
                            if actual_command == "=/>test ui"
                                && native_buttons
                            {
                                if let Ok(probe) = native_ui_probe_message() {
                                    if ctx.send_message(probe).await.is_ok() {
                                        return;
                                    }
                                }
                                eprintln!("Base-Matter: UI test unavailable; sending text instructions");
                            }
                            if actual_command == "=/>test row"
                            {
                                if ctx.send_message(legacy_row_probe_message()).await.is_ok() {
                                    return;
                                }
                                eprintln!("Base-Matter: classic ButtonsRow test rejected; using text");
                            }
                            if actual_command == "=/>test html"
                                && rich_html_trial
                                && frame.from_me
                                && frame.chat_jid == config.owner_jid
                            {
                                if let Ok(message) = experimental_self_chat_rich_message(&response)
                                {
                                    if ctx.send_message(message).await.is_ok() {
                                        return;
                                    }
                                }
                                eprintln!("Base-Matter: HTML test unavailable; sending text instructions");
                            }
                            let is_menu = matches!(actual_command, "=/>menu" | "=/>menú")
                                || actual_command.starts_with("=/>menu ")
                                || actual_command.starts_with("=/>menú ");
                            let is_root = matches!(actual_command, "=/>menu" | "=/>menú");
                            // Help is text-only. A help reply must not display a category selector.
                            let mut sent = false;
                            if native_buttons && is_menu {
                                if let Ok(interactive) = native_navigation_message(&response, actual_command) {
                                    sent = ctx.send_message(interactive).await.is_ok();
                                }
                                if !sent {
                                    eprintln!("Base-Matter: native selector unavailable; using text");
                                }
                            }
                            if !sent {
                                sent = ctx.reply(&response).await.is_ok();
                                if !sent {
                                    eprintln!("Base-Matter: control reply unavailable");
                                }
                            }
                            if sent && is_private_vault_command(&frame, &config.owner_jid) {
                                println!("Base-Matter: Vault owner response submitted to WhatsApp");
                            }
                            // Private enabled chats may inspect read-only
                            // NativeFlow test buttons when the operator opted in.
                            // HTML, audio and legacy comparison remain owner-self only.
                            if sent && selected_button.is_some() {
                                println!("Base-Matter: interactive control received and authenticated");
                            }
                            if sent && menu_lab && is_root && !data.is_group {
                                let owner_self_chat =
                                    frame.from_me && frame.chat_jid == config.owner_jid;
                                send_menu_lab(
                                    &ctx,
                                    &response,
                                    rich_html_trial,
                                    owner_self_chat,
                                )
                                .await;
                            }
                        }
                        Ok(ControlAck::Duplicate) => {
                            println!("Base-Matter duplicate control ignored (no new effect)");
                        }
                        Ok(ControlAck::Ignored) => {
                            // The bot is intentionally silent while disabled.
                            // Only no-argument =/>test and status explain why.
                            if frame.text.starts_with("=/>test") {
                                eprintln!("Base-Matter: visual test ignored by chat admission");
                            }
                        }
                        Err(_) => {
                            eprintln!("Base-Matter control denied or unreachable");
                            // The verified owner receives a generic Vault error,
                            // without exposing IPC details, keys or private data.
                            if is_private_vault_command(&frame, &config.owner_jid) {
                                let _ = ctx
                                    .reply("Vault-Matter: consulta no completada. Revisa el controlador y la credencial del socket; no se modificaron roles.")
                                    .await;
                            } else if frame.from_me
                                && frame.text.starts_with("=/>set role owner --global")
                                && !frame.chat_jid.ends_with("@g.us")
                            {
                                let _ = ctx
                                    .reply("Vault-Matter: asignación denegada. Verifica la identidad principal y el servicio privado.")
                                    .await;
                            } else if frame.from_me && frame.text.starts_with("=/>test") {
                                let _ = ctx
                                    .reply("Base-Matter: prueba rechazada por el controlador. Revisa permisos del chat y el registro local.")
                                    .await;
                            }
                        }
                    },
                    Ok(None) => {}
                    Err(_) => eprintln!("Unsafe or expired owner control ignored"),
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
    bot.run().await;
    if resume_only {
        if connected_once.load(Ordering::Relaxed) {
            bail!("Resume-only: WhatsApp disconnected after authentication. No pairing was requested.");
        }
        bail!("Resume-only: WhatsApp stopped before authentication. Inspect the device identity locally and check phone linked devices. No pairing was requested.");
    }
    Ok(())
}

#[cfg(test)]
mod readiness_tests {
    use super::*;

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
