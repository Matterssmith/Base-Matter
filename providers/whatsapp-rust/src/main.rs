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
use whatsapp_matter_rust_provider::{parse_control_ack, Config, ControlAck, Event, SessionLease};
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
        stream.take(1025).read_to_end(&mut response).await?;
        if response.len() > 1024 {
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
                    || p.lid.as_ref().is_some_and(|jid| jid.to_string() == owner_jid))
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

#[tokio::main]
async fn main() -> Result<()> {
    match std::env::args().nth(1).as_deref() {
        Some("--help") => {
            println!("WhatsApp-Matter Rust provider: --check or run for pairing code, local authenticated Base-Matter bridge.");
            return Ok(());
        }
        Some("--check") => {
            let config = Config::from_env()?;
            SessionLease::check_available(&config.session_db)?;
            println!("whatsapp-rust pairing settings checked; no session was opened.");
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
                let chat = source.chat.to_string();
                let sender = source.sender.to_string();
                let data = Event {
                    chat_jid: &chat,
                    sender_jid: &sender,
                    message_id: &ctx.info.id,
                    text: ctx.message.text_content().unwrap_or_default(),
                    from_me: source.is_from_me,
                    is_group: source.is_group,
                    timestamp: ctx.info.timestamp.timestamp(),
                };
                match config.signed_control(&data) {
                    Ok(Some(frame)) => match deliver(config.bridge_addr, &frame).await {
                        Ok(ack @ (ControlAck::Applied | ControlAck::AppliedWithMessage(_))) => {
                            println!("Base-Matter owner control applied");
                            // Only an authenticated private self-chat may receive
                            // an authorized response. Text has no control prefix.
                            let response = if frame.text == "=/>start" {
                                match timeout(
                                    Duration::from_secs(20),
                                    discover_register(&ctx, &config.owner_jid),
                                )
                                .await
                                {
                                    Ok(Ok(report)) => report,
                                    _ => "Base-Matter REGISTER: inspección no disponible o incompleta; no se hicieron cambios.".to_owned(),
                                }
                            } else if frame.text == "=/>add++" {
                                match timeout(
                                    Duration::from_secs(20),
                                    preview_community_recovery(&ctx, &config.owner_jid),
                                )
                                .await
                                {
                                    Ok(Ok(report)) => report,
                                    _ => "Base-Matter: inventario de recuperación incompleto; no se copiaron miembros.".to_owned(),
                                }
                            } else if frame.text.starts_with("=/>add++ snapshot ") {
                                "Base-Matter: instantánea cifrada aún pendiente de implementar. Ningún respaldo fue guardado.".to_owned()
                            } else {
                                match ack {
                                    ControlAck::AppliedWithMessage(text) => text,
                                    _ => "Base-Matter: solicitud procesada.".to_owned(),
                                }
                            };
                            if ctx.reply(response).await.is_err() {
                                eprintln!("Base-Matter control applied; chat acknowledgment unavailable");
                            }
                        }
                        Ok(ControlAck::Duplicate) => {
                            println!("Base-Matter duplicate control ignored (no new effect)");
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
