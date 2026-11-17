/*
 * WhatsApp-Matter — isolated whatsapp-rust 0.7 pairing-code provider.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Result};
use std::sync::Arc;
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
    timeout(Duration::from_secs(4), transport).await??;
    Ok(())
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
            if config.session_db.with_extension("owner.lock").exists() {
                bail!("Session already owned or lock recovery required");
            }
            println!("whatsapp-rust pairing settings checked; no session was opened.");
            return Ok(());
        }
        None => {}
        Some(_) => bail!("Unsupported provider operation"),
    }
    let config = Arc::new(Config::from_env()?);
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
                        Ok(ControlAck::Applied) => {
                            println!("Base-Matter owner control applied");
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
