/*
 * WhatsApp-Matter — isolated whatsapp-rust 0.7 pairing-code provider.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Result};
use std::{collections::HashSet, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::{timeout, Duration},
};
use whatsapp_matter_rust_provider::{
    parse_control_ack,
    recovery::{
        read_encrypted, recovery_key_from_env, save_encrypted, RecoveryMember, RecoverySnapshot,
    },
    Config, ControlAck, Event, SessionLease,
};
use whatsapp_rust::{pair_code::PairCodeOptions, prelude::*};

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
    timeout(Duration::from_secs(4), transport).await?
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

#[tokio::main]
async fn main() -> Result<()> {
    match std::env::args().nth(1).as_deref() {
        Some("--help") => {
            println!("WhatsApp-Matter Rust provider: --check, --recovery-verify <encrypted-filename>, or run for pairing code. Local Base-Matter IPC; no QR.");
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
        None => {}
        Some(_) => bail!("Unsupported provider operation"),
    }
    let config = Arc::new(Config::from_env()?);
    require_local_controller(config.bridge_addr).await?;
    let _lease = SessionLease::acquire(&config.session_db)?;
    let database = config
        .session_db
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Session database path is not Unicode"))?;
    let backend = SqliteStore::new(database).await?;
    let refresh_phone = config.phone.clone();
    let phone = config.phone.clone();
    let bot = Bot::builder()
        .with_backend(backend)
        .skip_history_sync()
        .with_pair_code(PairCodeOptions {
            phone_number: phone,
            ..Default::default()
        })
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
        .on_pair_code_error(|_error, _client| async move {
            eprintln!("Pairing code request failed or was rate limited");
        })
        .on_pair_code_refresh(move |_manual, client| {
            let phone = refresh_phone.clone();
            async move {
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
        .on_connected(|_client| async move {
            println!("WhatsApp session connected. Base-Matter owner control available.");
        })
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
                let data = Event {
                    chat_jid: &chat,
                    sender_jid: &sender,
                    message_id: &ctx.info.id,
                    text: ctx.message.text_content().unwrap_or_default(),
                    from_me: source.is_from_me,
                    is_group: source.is_group,
                    timestamp: ctx.info.timestamp.timestamp(),
                };
                let verified = config.signed_verified_control(
                    &data,
                    own_pn.as_deref(),
                    own_lid.as_deref(),
                );
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
                            // Only a verified owner-authored private/group event
                            // may receive an authorized response. No reply is sent
                            // for disabled bot functions or unauthorized senders.
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
                            if ctx.reply(response).await.is_err() {
                                eprintln!("Base-Matter control applied; chat acknowledgment unavailable");
                            }
                        }
                        Ok(ControlAck::Duplicate) => {
                            println!("Base-Matter duplicate control ignored (no new effect)");
                        }
                        Ok(ControlAck::Ignored) => {
                            // Bot disabled in this chat: intentionally silent.
                        }
                        Err(_) => {
                            eprintln!("Base-Matter owner control denied or unreachable");
                        }
                    },
                    Ok(None) => {}
                    Err(_) => eprintln!("Unsafe or expired owner control ignored"),
                }
            }
        })
        .build()
        .await?;
    println!("WhatsApp-Matter Rust session starting. Pairing code only.");
    bot.run().await;
    Ok(())
}

#[cfg(test)]
mod readiness_tests {
    use super::*;

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
