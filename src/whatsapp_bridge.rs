/*
 * Base-Matter — authenticated WhatsApp owner and public read-only bridge (v1).
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */

use crate::anti_guard::EditState;
use crate::control::{ControlDispatcher, DispatchResult};
use crate::ingress::InboundReplayStore;
use crate::vault_ipc::VaultClient;
use anyhow::{anyhow, bail, Result};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::sync::Arc;
use time::OffsetDateTime;
use zeroize::Zeroizing;

const VERSION: u8 = 1;
const MAX_FRAME_BYTES: usize = 16 * 1024;
const MAX_TEXT_BYTES: usize = 4096;
const MAX_ID_BYTES: usize = 256;
const MAX_SKEW_SECONDS: u64 = 120;
const DOMAIN: &[u8] = b"Base-Matter:WhatsApp-Control:v1\0";

/// Untrusted bytes received from a provider process. The signature covers
/// every field except signature. The authenticated provider must obtain all
/// routing identifiers from its established WhatsApp connection, not from text.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WhatsAppControlFrame {
    pub version: u8,
    pub socket_id: String,
    pub actor_id: String,
    pub owner_jid: String,
    pub chat_jid: String,
    pub message_id: String,
    pub text: String,
    pub from_me: bool,
    pub timestamp: i64,
    pub signature: String,
}

#[derive(Clone, Debug)]
pub struct WhatsAppBridgeConfig {
    pub socket_id: String,
    pub owner_actor_id: String,
    pub owner_jid: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WhatsAppBridgeOutcome {
    AntiObserved,
    Ignored,
    Duplicate,
    Dispatched(DispatchResult),
}

/// Distinguishes trusted owner-origin administrative controls from signed,
/// read-only incoming status/menu/bot requests in verified groups or external
/// private chats. Never routes public messages to Bot-Matter or role mutation.
pub struct WhatsAppControlBridge {
    config: WhatsAppBridgeConfig,
    key: Zeroizing<[u8; 32]>,
    replay: Arc<InboundReplayStore>,
    control: Arc<ControlDispatcher>,
    global_vault: Option<VaultClient>,
}

impl WhatsAppControlBridge {
    pub fn new(
        config: WhatsAppBridgeConfig,
        key: [u8; 32],
        replay: Arc<InboundReplayStore>,
        control: Arc<ControlDispatcher>,
    ) -> Result<Self> {
        validate_identifier(&config.socket_id)?;
        validate_identifier(&config.owner_actor_id)?;
        validate_owner_jid(&config.owner_jid)?;
        if key.iter().all(|byte| *byte == 0) {
            bail!("Invalid WhatsApp bridge binding key");
        }
        Ok(Self {
            config,
            key: Zeroizing::new(key),
            replay,
            control,
            global_vault: None,
        })
    }

    /// Only the trusted controller may install a per-socket authenticated
    /// Vault client. No sender may select an IPC key, tenant or socket.
    pub fn with_global_vault(mut self, client: Option<VaultClient>) -> Result<Self> {
        if client
            .as_ref()
            .is_some_and(|c| c.socket != self.config.socket_id)
        {
            bail!("Vault credential does not belong to current socket");
        }
        self.global_vault = client;
        Ok(self)
    }

    pub fn ingest_json(&self, incoming: &str) -> Result<WhatsAppBridgeOutcome> {
        if incoming.len() > MAX_FRAME_BYTES {
            bail!("WhatsApp control frame exceeds limit");
        }
        let frame: WhatsAppControlFrame = serde_json::from_str(incoming)
            .map_err(|_| anyhow!("Invalid WhatsApp control frame"))?;
        self.ingest(&frame)
    }

    fn verify_frame(&self, frame: &WhatsAppControlFrame) -> Result<(bool, bool)> {
        // Two disjoint identities, both backed by the provider's HMAC:
        // (1) verified socket-owner controls; (2) incoming read-only
        // requests bound to a real WhatsApp group or sender's private chat.
        let is_owner = frame.from_me
            && frame.actor_id == self.config.owner_actor_id
            && (frame.chat_jid == self.config.owner_jid || valid_group_jid(&frame.chat_jid));
        let is_public_reader = !frame.from_me
            && valid_user_jid(&frame.actor_id)
            && (valid_group_jid(&frame.chat_jid) || frame.chat_jid == frame.actor_id);
        if frame.version != VERSION
            || frame.socket_id != self.config.socket_id
            || frame.owner_jid != self.config.owner_jid
            || !(is_owner || is_public_reader)
        {
            bail!("WhatsApp control identity denied");
        }
        if frame.message_id.is_empty()
            || frame.message_id.len() > MAX_ID_BYTES
            || frame.message_id.chars().any(char::is_control)
            || frame.text.is_empty()
            || frame.text.len() > MAX_TEXT_BYTES
        {
            bail!("Invalid WhatsApp control payload");
        }
        let now = OffsetDateTime::now_utc().unix_timestamp();
        if now.abs_diff(frame.timestamp) > MAX_SKEW_SECONDS {
            bail!("Expired WhatsApp control frame");
        }
        let supplied =
            hex::decode(&frame.signature).map_err(|_| anyhow!("Invalid WhatsApp control proof"))?;
        if supplied.len() != 32 {
            bail!("Invalid WhatsApp control proof");
        }
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.key[..])
            .map_err(|_| anyhow!("WhatsApp control verifier unavailable"))?;
        mac.update(DOMAIN);
        let fields = [
            VERSION.to_string(),
            frame.socket_id.clone(),
            frame.actor_id.clone(),
            frame.owner_jid.clone(),
            frame.chat_jid.clone(),
            frame.message_id.clone(),
            frame.text.clone(),
        ];
        for field in &fields {
            let bytes = field.as_bytes();
            mac.update(&(u32::try_from(bytes.len())?).to_be_bytes());
            mac.update(bytes);
        }
        mac.update(&[u8::from(frame.from_me)]);
        mac.update(&frame.timestamp.to_be_bytes());
        mac.verify_slice(&supplied)
            .map_err(|_| anyhow!("Invalid WhatsApp control proof"))?;
        Ok((is_owner, is_public_reader))
    }

    /// Diagnostic only: verify the actual signed sender before requesting a
    /// fresh, bounded Vault decision. Never grant access from this result.
    pub(crate) fn console_global_owner(&self, frame: &WhatsAppControlFrame) -> bool {
        if !(frame.text.starts_with("=/>")
            || frame.text.starts_with("\u{001f}matter-group-style:=/>")
            || frame.text.starts_with("\u{001f}matter-presentation:"))
        {
            return false;
        }
        let Ok((is_owner, _)) = self.verify_frame(frame) else {
            return false;
        };
        let principal = if is_owner {
            self.config.owner_jid.as_str()
        } else {
            frame.actor_id.as_str()
        };
        self.global_vault
            .as_ref()
            .is_some_and(|vault| vault.console_global_owner(principal))
    }

    pub fn ingest(&self, frame: &WhatsAppControlFrame) -> Result<WhatsAppBridgeOutcome> {
        let (is_owner, is_public_reader) = self.verify_frame(frame)?;

        // Anti observations have a separate signed, read-only path.
        // Do NOT call dispatch() here: an incoming group member cannot
        // become the socket owner through text, mentions or Unicode.
        if let Some(body) = frame.text.strip_prefix("\u{001f}matter-anti:") {
            let Some((state, text)) = body.split_once(':') else {
                bail!("Invalid anti observation state");
            };
            let edit = match state {
                "original" => EditState::Original,
                "edited" => EditState::EditedReadable,
                "opaque" if text.is_empty() => EditState::EditedEncryptedUnavailable,
                _ => bail!("Invalid anti edit state"),
            };
            if !valid_group_jid(&frame.chat_jid)
                || frame.message_id.len() > 180
                || (!is_owner && !valid_user_jid(&frame.actor_id))
            {
                bail!("Anti observation identity denied");
            }
            let replay_id = format!("{state}:{}", frame.message_id);
            if !self
                .replay
                .claim("whatsapp-anti", &self.config.socket_id, &replay_id)?
            {
                return Ok(WhatsAppBridgeOutcome::Duplicate);
            }
            self.control.inspect_anti_observation(
                &self.config.socket_id,
                &frame.chat_jid,
                text,
                edit,
            )?;
            return Ok(WhatsAppBridgeOutcome::AntiObserved);
        }
        // Only the authenticated adapter can attach fresh group/target metadata.
        let (style_text, admin_attested, attested_target) =
            if let Some(body) = frame.text.strip_prefix("\u{001f}matter-presentation:") {
                let (admin, rest) = body
                    .split_once(':')
                    .ok_or_else(|| anyhow!("Invalid presentation proof"))?;
                let (target, command) = rest
                    .split_once(':')
                    .ok_or_else(|| anyhow!("Invalid presentation target"))?;
                if !matches!(admin, "0" | "1")
                    || (admin == "1" && !valid_group_jid(&frame.chat_jid))
                    || !command.starts_with("=/>set ")
                {
                    bail!("Invalid presentation attestation");
                }
                (command, admin == "1", Some(target))
            } else if let Some(command) = frame.text.strip_prefix("\u{001f}matter-group-style:") {
                if !is_public_reader
                    || !valid_group_jid(&frame.chat_jid)
                    || !matches!(
                        command.split_whitespace().collect::<Vec<_>>().as_slice(),
                        ["=/>set", "style", "matter" | "none" | "inherit"]
                    )
                {
                    bail!("Invalid group style attestation");
                }
                (command, true, None)
            } else {
                (frame.text.as_str(), false, None)
            };
        let style_parts: Vec<_> = style_text.split_whitespace().collect();
        let personal = style_parts.contains(&"--user");
        let presentation = style_parts.first() == Some(&"=/>set")
            && style_parts
                .iter()
                .skip(1)
                .any(|part| matches!(*part, "style" | "priority"));
        if presentation || personal || attested_target.is_some() {
            let group = valid_group_jid(&frame.chat_jid).then_some(frame.chat_jid.as_str());
            let kind = if group.is_some() {
                "group"
            } else if is_owner {
                "self"
            } else {
                "private"
            };
            if !self
                .replay
                .claim("whatsapp", &self.config.socket_id, &frame.message_id)?
            {
                return Ok(WhatsAppBridgeOutcome::Duplicate);
            }
            if !self.control.chat_command_allowed(
                &self.config.socket_id,
                group,
                kind,
                style_text,
            )? {
                return Ok(WhatsAppBridgeOutcome::Dispatched(DispatchResult::Inactive));
            }
            let principal = if is_owner {
                self.config.owner_jid.as_str()
            } else {
                frame.actor_id.as_str()
            };
            let reply = |output: String| -> Result<WhatsAppBridgeOutcome> {
                Ok(WhatsAppBridgeOutcome::Dispatched(
                    DispatchResult::Extension {
                        command: "set".into(),
                        output,
                    },
                ))
            };
            if !presentation {
                return reply("El ámbito --user aún no está disponible para este comando; no se realizó ningún cambio.".into());
            }
            use crate::control_scope::{parse_scope, CommandScope};
            let user_count = style_parts.iter().filter(|part| **part == "--user").count();
            let words: Vec<_> = style_parts[1..]
                .iter()
                .copied()
                .filter(|part| *part != "--user")
                .collect();
            if user_count > 1
                || (personal
                    && words
                        .iter()
                        .any(|word| matches!(*word, "--chat" | "--mod" | "--socket" | "--global")))
            {
                return reply("Solo se permite un ámbito por comando.".into());
            }
            let scoped = match parse_scope(&words) {
                Ok(scoped) => scoped,
                Err(_) => return reply("Ámbito inválido o repetido.".into()),
            };
            let [setting, tail @ ..] = scoped.arguments.as_slice() else {
                return reply("Uso: =/>set style [ámbito] [matter|none|inherit]".into());
            };
            if tail.len() > 1 || !matches!(*setting, "style" | "priority") {
                return reply("Argumentos inválidos. Selecciona un único usuario y valor.".into());
            }
            let value = tail.first().copied();
            if !personal && attested_target.is_some_and(|target| target != "-") {
                return reply("Objetivo de usuario inválido para este ámbito.".into());
            }
            if *setting == "style" && !personal && scoped.scope != CommandScope::Chat {
                let global = value.is_some()
                    && self
                        .global_vault
                        .as_ref()
                        .is_some_and(|vault| vault.require_global_owner(principal).is_ok());
                let can_change = match scoped.scope {
                    CommandScope::Socket => is_owner || global,
                    CommandScope::Global => global,
                    CommandScope::Chat => unreachable!(),
                };
                let outcome = self.control.scoped_style_reply(
                    scoped.scope,
                    &frame.chat_jid,
                    principal,
                    value,
                    can_change,
                )?;
                return Ok(WhatsAppBridgeOutcome::Dispatched(outcome));
            }
            if *setting == "priority"
                && (personal || scoped.scope != CommandScope::Chat || group.is_none())
            {
                return reply(
                    "La prioridad de ámbitos se configura en el grupo con =/>set priority --chat."
                        .into(),
                );
            }
            let target = if personal {
                match attested_target.unwrap_or("-") {
                    "-" => Some(principal),
                    target if valid_user_jid(target) => Some(target),
                    _ => return reply("Usuario no verificable. Usa una única mención o responde a un mensaje del chat.".into()),
                }
            } else {
                None
            };
            let record_kind = if *setting == "priority" {
                "policy"
            } else if personal {
                "user-style"
            } else {
                "chat-style"
            };
            let Some(vault) = self.global_vault.as_ref() else {
                return reply("Vault-Matter no disponible. No se confirmó ningún cambio.".into());
            };
            let request = crate::vault_ipc::PresentationRequest {
                chat: &frame.chat_jid,
                actor: principal,
                target,
                kind: record_kind,
                admin: admin_attested,
                value,
            };
            let label = match record_kind {
                "policy" => "Prioridad del grupo",
                "user-style" => "Estilo del usuario en este chat",
                _ => "Estilo del chat",
            };
            let result = match vault.presentation_config(&request) {
                Ok(value) => format!("{label}: {}. {}", value.as_deref().unwrap_or("inherit"),
                    if request.value.is_some() { "Guardado en Vault-Matter." } else { "Lectura autenticada de Vault-Matter." }),
                Err(_) => "Vault-Matter denegó la configuración o no está disponible. No se confirmó ningún cambio.".into(),
            };
            return reply(result);
        }
        // No bot commands, including historic #ping, are admitted here.
        if !frame.text.starts_with("=/>") {
            return Ok(WhatsAppBridgeOutcome::Ignored);
        }
        let group_id = if valid_group_jid(&frame.chat_jid) {
            Some(frame.chat_jid.as_str())
        } else {
            None
        };
        // Codex has a narrower authority than ordinary global maintenance.
        // Private chat only: no prompt or repository report is sent to a group.
        if frame.text.split_whitespace().next() == Some("=/>codex") {
            if group_id.is_some() || !valid_user_jid(&frame.chat_jid) {
                bail!("Codex is private-only");
            }
            let principal = if is_owner {
                self.config.owner_jid.as_str()
            } else {
                frame.actor_id.as_str()
            };
            self.global_vault
                .as_ref()
                .ok_or_else(|| anyhow!("Vault primary authority unavailable"))?
                .require_primary_owner(principal)?;
            if !self
                .replay
                .claim("whatsapp", &self.config.socket_id, &frame.message_id)?
            {
                return Ok(WhatsAppBridgeOutcome::Duplicate);
            }
            // Slot 1 has a private management surface without opening
            // ordinary external private chats. Socket power still applies.
            if !self.control.chat_command_allowed(
                &self.config.socket_id,
                None,
                "self",
                &frame.text,
            )? {
                return Ok(WhatsAppBridgeOutcome::Dispatched(DispatchResult::Inactive));
            }
            return Ok(WhatsAppBridgeOutcome::Dispatched(
                DispatchResult::Extension {
                    command: "codex".into(),
                    output: "Codex autorizado para el Owner principal.".into(),
                },
            ));
        }
        // Check the authenticated receiving surface before any command effect,
        // including independent global Vault routes. A denied request is consumed
        // so it cannot be replayed after the chat is enabled.
        let kind = if group_id.is_some() {
            "group"
        } else if is_owner {
            "self"
        } else {
            "private"
        };
        if !self.control.chat_command_allowed(
            &self.config.socket_id,
            group_id,
            kind,
            &frame.text,
        )? {
            if !self
                .replay
                .claim("whatsapp", &self.config.socket_id, &frame.message_id)?
            {
                return Ok(WhatsAppBridgeOutcome::Duplicate);
            }
            return Ok(WhatsAppBridgeOutcome::Dispatched(DispatchResult::Inactive));
        }
        // Socket-local alert settings never grant a role or alter admission.
        if frame
            .text
            .split_whitespace()
            .take(2)
            .eq(["=/>set", "alerts"])
        {
            let words: Vec<_> = frame.text.split_whitespace().collect();
            if !matches!(
                words.as_slice(),
                ["=/>set", "alerts"]
                    | ["=/>set", "alerts", "reaction" | "text" | "off"]
                    | [
                        "=/>set",
                        "alerts",
                        "processing" | "success" | "admission" | "denied" | "error" | "inactive",
                        "reaction" | "text" | "off"
                    ]
            ) {
                bail!("Invalid alert configuration");
            }
            if !is_owner {
                self.global_vault
                    .as_ref()
                    .ok_or_else(|| anyhow!("Vault authority unavailable"))?
                    .require_global_owner(&frame.actor_id)?;
            }
            if !self
                .replay
                .claim("whatsapp", &self.config.socket_id, &frame.message_id)?
            {
                return Ok(WhatsAppBridgeOutcome::Duplicate);
            }
            return Ok(WhatsAppBridgeOutcome::Dispatched(
                DispatchResult::Extension {
                    command: "set alerts".into(),
                    output: "Configuración de alertas autorizada.".into(),
                },
            ));
        }
        // Reload is restricted to this socket; it cannot activate compiled code.
        if frame.text.split_whitespace().eq(["=/>reload"]) {
            if !is_owner {
                self.global_vault
                    .as_ref()
                    .ok_or_else(|| anyhow!("Vault authority unavailable"))?
                    .require_global_owner(&frame.actor_id)?;
            }
            if !self
                .replay
                .claim("whatsapp", &self.config.socket_id, &frame.message_id)?
            {
                return Ok(WhatsAppBridgeOutcome::Duplicate);
            }
            return Ok(WhatsAppBridgeOutcome::Dispatched(
                DispatchResult::Extension {
                    command: "reload".into(),
                    output: "Recarga de vistas del socket autorizada.".into(),
                },
            ));
        }
        // Maintenance affects the entire local bot runtime. Only a fresh
        // Vault global-owner decision grants it, even for the socket owner.
        let maintenance: Vec<_> = frame.text.split_whitespace().collect();
        if matches!(maintenance.first(), Some(&"=/>update" | &"=/>restart"))
            || maintenance.starts_with(&["=/>set", "update"])
        {
            if !matches!(
                maintenance.as_slice(),
                ["=/>update" | "=/>restart"]
                    | ["=/>set", "update"]
                    | ["=/>set", "update", "manual" | "restart" | "reload" | "none"]
            ) {
                bail!("Invalid maintenance command");
            }
            let principal = if is_owner {
                self.config.owner_jid.as_str()
            } else {
                frame.actor_id.as_str()
            };
            self.global_vault
                .as_ref()
                .ok_or_else(|| anyhow!("Vault global authority unavailable"))?
                .require_global_owner(principal)?;
            if !self
                .replay
                .claim("whatsapp", &self.config.socket_id, &frame.message_id)?
            {
                return Ok(WhatsAppBridgeOutcome::Duplicate);
            }
            let command = maintenance[0].trim_start_matches("=/>");
            return Ok(WhatsAppBridgeOutcome::Dispatched(
                DispatchResult::Extension {
                    command: command.to_owned(),
                    output:
                        "Mantenimiento autorizado; el proveedor comprobará el supervisor local."
                            .to_owned(),
                },
            ));
        }
        // Exact global Vault health commands have their own HMAC-verified
        // path. Neither being the owner of this bot/socket nor typing an
        // owner claim in a message grants access: the private Vault authority
        // independently checks the actual WhatsApp principal every time.
        let mut tokens = frame.text.split_whitespace();
        if tokens.next() == Some("=/>vault") {
            if group_id.is_some() {
                bail!("Vault is private-only");
            }
            let client = self
                .global_vault
                .as_ref()
                .ok_or_else(|| anyhow!("Vault global authority unavailable"))?;
            let principal = if is_owner {
                self.config.owner_jid.as_str()
            } else {
                frame.actor_id.as_str()
            };
            let args = tokens.collect::<Vec<_>>();
            // Identity authorization must precede replay admission.
            // Replay admission must precede the synthetic write in "test".
            client.require_global_owner(principal)?;
            if !self
                .replay
                .claim("whatsapp", &self.config.socket_id, &frame.message_id)?
            {
                return Ok(WhatsAppBridgeOutcome::Duplicate);
            }
            let answer = client.global_owner_vault_command(principal, &args)?;
            return Ok(WhatsAppBridgeOutcome::Dispatched(
                DispatchResult::Extension {
                    command: "vault".to_owned(),
                    output: answer,
                },
            ));
        }
        // Every enrolled global owner can assign/replace slot 2. This is
        // authorized in private encrypted Vault regardless of bot ownership;
        // neither role is subordinate. The HMAC-authenticated principal is
        // checked against the encrypted owner registry on every request.
        let parts = frame.text.split_whitespace().collect::<Vec<_>>();
        if matches!(parts.as_slice(), ["=/>set", "role", "owner", "--global"])
            || matches!(parts.as_slice(), ["=/>set", "role", "owner", "--global", _])
        {
            if group_id.is_some() {
                bail!("Los owners globales se asignan sólo en chat privado");
            }
            let client = self
                .global_vault
                .as_ref()
                .ok_or_else(|| anyhow!("Vault global no configurado"))?;
            let principal = if is_owner {
                self.config.owner_jid.as_str()
            } else {
                frame.actor_id.as_str()
            };
            client.require_global_owner(principal)?;
            if !self
                .replay
                .claim("whatsapp", &self.config.socket_id, &frame.message_id)?
            {
                return Ok(WhatsAppBridgeOutcome::Duplicate);
            }
            let output = if parts.len() == 4 {
                "Uso: =/>set role owner --global NUMERO. Ambos owners globales pueden asignarlo en privado."
                    .to_owned()
            } else {
                client.grant_secondary_global_owner(principal, parts[4])?;
                "Vault-Matter: owner global secundario actualizado. Registro cifrado; identidad reservada.".to_owned()
            };
            return Ok(WhatsAppBridgeOutcome::Dispatched(
                DispatchResult::Extension {
                    command: "set-role-global-owner".to_owned(),
                    output,
                },
            ));
        }
        if is_public_reader {
            // A signed, provider-verified incoming WhatsApp sender may be an
            // owner of the entire ecosystem even if they do NOT own this
            // particular bot/socket. Vault is the independent authority.
            // The privileged local actor is substituted only AFTER Vault
            // authorizes the actual platform sender. Ordinary users never
            // enter this branch and retain the read-only policy below.
            let globally_authorized = self
                .global_vault
                .as_ref()
                .is_some_and(|vault| vault.require_global_owner(&frame.actor_id).is_ok());
            if globally_authorized
                && !matches!(
                    frame.text.split_whitespace().next(),
                    Some("=/>status" | "=/>estado")
                )
            {
                if !self
                    .replay
                    .claim("whatsapp", &self.config.socket_id, &frame.message_id)?
                {
                    return Ok(WhatsAppBridgeOutcome::Duplicate);
                }
                let outcome = self.control.dispatch(
                    &self.config.socket_id,
                    &self.config.owner_actor_id,
                    &frame.chat_jid,
                    group_id,
                    &frame.text,
                )?;
                return Ok(WhatsAppBridgeOutcome::Dispatched(outcome));
            }
            // Anonymous/public sender: read-only. No role, socket or Vault
            // mutation regardless of the command text, mention or username.
            let outcome = self.control.dispatch_public_readonly(
                &self.config.socket_id,
                &frame.actor_id,
                &frame.chat_jid,
                group_id,
                &frame.text,
            )?;
            if !self
                .replay
                .claim("whatsapp", &self.config.socket_id, &frame.message_id)?
            {
                return Ok(WhatsAppBridgeOutcome::Duplicate);
            }
            return Ok(WhatsAppBridgeOutcome::Dispatched(outcome));
        }
        let verb = frame.text.split_whitespace().next().unwrap_or_default();
        if group_id.is_some() {
            if !matches!(
                verb,
                "=/>group"
                    | "=/>grupo"
                    | "=/>menu"
                    | "=/>menú"
                    | "=/>help"
                    | "=/>h"
                    | "=/>ayuda"
                    | "=/>status"
                    | "=/>ping"
                    | "=/>estado"
                    | "=/>bot"
                    | "=/>start"
                    | "=/>add"
                    | "=/>add++"
                    | "=/>anti"
                    | "=/>set"
                    | "=/>credits"
                    | "=/>creditos"
                    | "=/>créditos"
                    | "=/>test"
                    | "=/>register"
            ) {
                bail!("Only group-scoped controls are available in groups");
            }
        } else if !matches!(
            verb,
            "=/>status"
                | "=/>ping"
                | "=/>estado"
                | "=/>bot"
                | "=/>menu"
                | "=/>menú"
                | "=/>help"
                | "=/>h"
                | "=/>ayuda"
                | "=/>set"
                | "=/>unset"
                | "=/>role"
                | "=/>roles"
                | "=/>audit"
                | "=/>vault"
                | "=/>matter"
                | "=/>test"
                | "=/>register"
                | "=/>anti"
                | "=/>credits"
                | "=/>creditos"
                | "=/>créditos"
        ) {
            bail!("Private chat is reserved for socket configuration");
        }
        if !self
            .replay
            .claim("whatsapp", &self.config.socket_id, &frame.message_id)?
        {
            return Ok(WhatsAppBridgeOutcome::Duplicate);
        }
        let outcome = self.control.dispatch(
            &self.config.socket_id,
            &self.config.owner_actor_id,
            &frame.chat_jid,
            group_id,
            &frame.text,
        )?;
        Ok(WhatsAppBridgeOutcome::Dispatched(outcome))
    }
}

fn valid_user_jid(value: &str) -> bool {
    let account = value
        .strip_suffix("@s.whatsapp.net")
        .or_else(|| value.strip_suffix("@lid"));
    account.is_some_and(|digits| {
        (8..=20).contains(&digits.len()) && digits.bytes().all(|byte| byte.is_ascii_digit())
    })
}

fn valid_group_jid(value: &str) -> bool {
    value.ends_with("@g.us")
        && !value.starts_with('@')
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"@._:-".contains(&byte))
}

fn validate_identifier(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > 128
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._:-".contains(&byte))
    {
        bail!("Invalid bridge identifier");
    }
    Ok(())
}

fn validate_owner_jid(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > 128
        || !(value.ends_with("@s.whatsapp.net") || value.ends_with("@lid"))
        || !value
            .split('@')
            .next()
            .unwrap_or_default()
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._:-".contains(&byte))
    {
        bail!("Invalid WhatsApp owner JID");
    }
    Ok(())
}
