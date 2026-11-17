/*
 * WhatsApp-Matter — actual whatsapp-rust pairing-code-only provider runner.
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Context, Result};
use matter_base::{
    activation::SocketActivationStore, control::ControlDispatcher, BotRegistry, CommandAuthorizer,
    CommandInvocation, CommandRouter, InboundReplayStore, WhatsAppNativeConfig,
    WhatsAppNativeIngress, WhatsAppNativeMessage, WhatsAppNativeOutcome,
};
use std::sync::Arc;
use whatsapp_matter_rust_provider::{Config, SessionLease};
use whatsapp_rust::{pair_code::PairCodeOptions, prelude::*};

struct NoBot;
impl CommandAuthorizer for NoBot {
    fn authorize(&self, _: &CommandInvocation) -> Result<()> {
        bail!("Bot-Matter cannot execute in the provider")
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    match std::env::args().nth(1).as_deref() {
        Some("--help") => {
            println!("WhatsApp-Matter (Rust): --init to provision a private socket, then run with configured pairing-code values. No QR or Bot-Matter commands.");
            return Ok(());
        }
        None | Some("--init") => {}
        Some(_) => bail!("Unsupported provider operation"),
    }
    let init = std::env::args().nth(1).as_deref() == Some("--init");
    let config = Config::from_env()?;
    let _lease = SessionLease::acquire(&config.session_db)?;
    let activation = Arc::new(SocketActivationStore::open(&config.activation_db)?);
    if init {
        activation
            .provision(&config.socket_id, &config.actor_id)
            .context("Socket must not already be provisioned")?;
        println!("Socket metadata provisioned. No WhatsApp connection opened.");
        return Ok(());
    }
    // The operator must explicitly provision the socket before connecting.
    // audit_history checks the current owner, so a changed owner denies.
    activation.audit_history(&config.socket_id, &config.actor_id)?;
    let replay = Arc::new(InboundReplayStore::open(
        &config.replay_db,
        *config.replay_key,
    )?);
    let router = Arc::new(CommandRouter::new(vec!["#".into()], Arc::new(NoBot))?);
    let control = Arc::new(ControlDispatcher::new(
        activation,
        Arc::new(BotRegistry::in_memory()),
        router,
        None,
    ));
    let ingress = Arc::new(WhatsAppNativeIngress::new(
        WhatsAppNativeConfig {
            socket_id: config.socket_id,
            actor_id: config.actor_id,
            owner_jid: config.owner_jid,
        },
        replay,
        control,
    )?);
    // User-selected private SQLite path. No implicit session recreation or
    // deletion. Existing databases require an offline backup before upgrade.
    let database = config
        .session_db
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Session database path must be Unicode"))?;
    let store = SqliteStore::new(database).await?;
    let phone = config.phone;
    let refresh = phone.clone();
    let bot = Bot::builder()
        .with_backend(store)
        .skip_history_sync()
        .with_pair_code(PairCodeOptions {
            phone_number: phone,
            ..Default::default()
        })
        // Deliberately NO on_qr_code callback. Only owner terminal receives
        // the short-lived pairing code. Upstream can still generate an
        // internal QR challenge; never render or export its payload.
        .on_pair_code(|code, timeout| async move {
            if code.len() == 8 && code.bytes().all(|b| b.is_ascii_alphanumeric()) {
                println!(
                    "Owner pairing code: {code} (valid {} seconds)",
                    timeout.as_secs()
                );
            } else {
                eprintln!("WhatsApp returned a nonconforming pairing code");
            }
        })
        .on_pair_code_error(|_error, _client| async move {
            eprintln!("WhatsApp rejected code pairing; check provider policy or rate limits");
        })
        .on_pair_code_refresh(move |_manual, client| {
            let phone = refresh.clone();
            async move {
                if client
                    .pair_with_code(PairCodeOptions {
                        phone_number: phone,
                        ..Default::default()
                    })
                    .await
                    .is_err()
                {
                    eprintln!("WhatsApp pairing code refresh failed");
                }
            }
        })
        .on_connected(|_client| async move {
            println!("WhatsApp session connected. Global owner control enabled.");
        })
        .on_message(move |ctx| {
            let ingress = ingress.clone();
            async move {
                let source = &ctx.info.source;
                let chat = source.chat.to_string();
                let sender = source.sender.to_string();
                let text = ctx.message.text_content().unwrap_or_default();
                let event = WhatsAppNativeMessage {
                    chat_jid: &chat,
                    sender_jid: &sender,
                    message_id: &ctx.info.id,
                    text,
                    from_me: source.is_from_me,
                    is_group: source.is_group,
                    timestamp: ctx.info.timestamp.timestamp(),
                };
                match ingress.ingest(&event) {
                    Ok(WhatsAppNativeOutcome::Dispatched(_)) => {
                        println!("Base-Matter owner control accepted")
                    }
                    Ok(WhatsAppNativeOutcome::Ignored | WhatsAppNativeOutcome::Duplicate) => {}
                    Err(_) => eprintln!("Base-Matter global control rejected"),
                }
            }
        })
        .build()
        .await?;
    println!("WhatsApp-Matter Rust session starting; pairing code only.");
    bot.run().await;
    Ok(())
}
