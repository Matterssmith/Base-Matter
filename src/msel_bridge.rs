/*
 * Base-Matter — Matter-MSEL host adapter for the ORIGINAL MSEL native engine.
 * The MSEL language and compiler stay in the private MSEL repository.
 * No credentials, Vault keys, WhatsApp sessions or platform send privileges.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Context, Result};
use serde::Serialize;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const MAX_EVENT_WIRE: usize = 32768;
const MAX_OUTPUT_WIRE: u64 = 131072;
const MAX_SOURCE_BYTES: u64 = 1024 * 1024;
const MAX_REPLIES: usize = 4;
const MAX_TEXT_CHARS: usize = 4096;

/// Construct only from identities authenticated by InboundGateway.
/// A normalized provider event alone is NOT authorization.
#[derive(Clone, Debug, Serialize)]
pub struct MselMessageEvent<'a> {
    pub provider: &'a str,
    pub chat: &'a str,
    pub sender: &'a str,
    pub message_id: &'a str,
    pub text: &'a str,
}

impl MselMessageEvent<'_> {
    pub fn validate(&self) -> Result<()> {
        for id in [self.provider, self.chat, self.sender, self.message_id] {
            if id.is_empty() || id.len() > 256 || id.chars().any(char::is_control) {
                bail!("Invalid MSEL message identity");
            }
        }
        if self.text.chars().count() > MAX_TEXT_CHARS || self.text.contains('\0') {
            bail!("Invalid MSEL message content");
        }
        if serde_json::to_vec(self)?.len() > MAX_EVENT_WIRE {
            bail!("MSEL event exceeds the protocol limit");
        }
        Ok(())
    }
}

/// Explicit host-controlled runner. Defaults to READ ONLY (no bot replies).
///
/// One subprocess per event. The Python CLI only marshals JSON into MSEL's
/// original C++ ABI (msel_bot_message_v1); it does not interpret .msel itself.
/// This is NOT a sandbox. Run only trusted scripts under a restricted OS user.
#[derive(Clone, Debug)]
pub struct MselCommandRunner {
    pub python: PathBuf,
    pub msel_checkout: PathBuf,
    pub bot_source: PathBuf,
    pub native_library: PathBuf,
    pub timeout: Duration,
    allow_reply: bool,
}

impl MselCommandRunner {
    pub fn new(python: PathBuf, checkout: PathBuf, source: PathBuf, library: PathBuf) -> Self {
        Self {
            python,
            msel_checkout: checkout,
            bot_source: source,
            native_library: library,
            timeout: Duration::from_secs(3),
            allow_reply: false,
        }
    }

    /// Call only after explicit authorization for message.reply at the host.
    pub fn allow_message_reply(mut self) -> Self {
        self.allow_reply = true;
        self
    }

    pub fn execute(&self, event: &MselMessageEvent<'_>) -> Result<Vec<String>> {
        event.validate()?;
        if self.timeout.is_zero() || self.timeout > Duration::from_secs(30) {
            bail!("Invalid MSEL execution timeout");
        }
        let checkout = self.msel_checkout.canonicalize().context("MSEL checkout unavailable")?;
        if !checkout.is_dir() {
            bail!("MSEL checkout is not a directory");
        }
        let script = self.bot_source.canonicalize().context("MSEL bot unavailable")?;
        if !script.is_file() || !script.to_string_lossy().ends_with(".msel")
            || script.metadata()?.len() > MAX_SOURCE_BYTES
        {
            bail!("Invalid trusted MSEL source");
        }
        let native = self.native_library.canonicalize().context("Native MSEL library unavailable")?;
        if !native.is_file() {
            bail!("Native MSEL library is not a file");
        }
        let payload = serde_json::to_vec(event)?;
        if payload.len() > MAX_EVENT_WIRE {
            bail!("MSEL request too large");
        }

        let mut cmd = Command::new(&self.python);
        cmd.arg("-m").arg("msel").arg("bot-event").arg(&script)
            .arg("--native-lib").arg(&native)
            .current_dir(&checkout)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .env_clear()
            .env("PYTHONDONTWRITEBYTECODE", "1")
            .env("PYTHONIOENCODING", "utf-8");
        if let Some(path) = std::env::var_os("PATH") {
            cmd.env("PATH", path);
        }
        if !self.allow_reply {
            cmd.arg("--read-only");
        }
        let mut child = cmd.spawn().context("MSEL process unavailable")?;
        let mut stdin = child.stdin.take().context("MSEL stdin unavailable")?;
        let stdout = child.stdout.take().context("MSEL stdout unavailable")?;
        let writer = thread::spawn(move || -> std::io::Result<()> {
            stdin.write_all(&payload)?;
            stdin.flush()
        });
        let reader = thread::spawn(move || -> std::io::Result<Vec<u8>> {
            let mut bytes = Vec::new();
            stdout.take(MAX_OUTPUT_WIRE + 1).read_to_end(&mut bytes)?;
            Ok(bytes)
        });

        let start = Instant::now();
        let status = loop {
            if let Some(status) = child.try_wait()? {
                break status;
            }
            if start.elapsed() >= self.timeout {
                let _ = child.kill();
                let _ = child.wait();
                let _ = writer.join();
                let _ = reader.join();
                bail!("MSEL execution timed out");
            }
            thread::sleep(Duration::from_millis(10));
        };
        writer.join().map_err(|_| anyhow::anyhow!("MSEL stdin thread failed"))??;
        let bytes = reader.join().map_err(|_| anyhow::anyhow!("MSEL stdout thread failed"))??;
        if !status.success() || bytes.len() as u64 > MAX_OUTPUT_WIRE {
            bail!("MSEL native execution rejected");
        }
        validate_replies(&bytes, self.allow_reply)
    }
}

pub fn validate_replies(bytes: &[u8], can_reply: bool) -> Result<Vec<String>> {
    if bytes.len() as u64 > MAX_OUTPUT_WIRE {
        bail!("MSEL reply payload exceeds limit");
    }
    let replies: Vec<String> = serde_json::from_slice(bytes).context("Invalid MSEL reply JSON")?;
    if replies.len() > MAX_REPLIES || (!can_reply && !replies.is_empty()) {
        bail!("MSEL reply permission or count violated");
    }
    for reply in &replies {
        if reply.chars().count() > MAX_TEXT_CHARS || reply.contains('\0') {
            bail!("Invalid MSEL reply content");
        }
    }
    Ok(replies)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wire_contract_is_exactly_the_native_msel_event_shape() {
        let event = MselMessageEvent {
            provider: "whatsapp",
            chat: "room@g.us",
            sender: "owner@s.whatsapp.net",
            message_id: "msg1",
            text: "¡hola!",
        };
        event.validate().unwrap();
        let value = serde_json::to_value(&event).unwrap();
        assert_eq!(value["provider"], "whatsapp");
        assert_eq!(value["chat"], "room@g.us");
        assert_eq!(value["text"], "¡hola!");
        assert_eq!(value.as_object().unwrap().len(), 5);
    }

    #[test]
    fn input_and_output_fail_closed() {
        let invalid = MselMessageEvent {
            provider: "whatsapp",
            chat: "room\nother",
            sender: "s",
            message_id: "id",
            text: "hello",
        };
        assert!(invalid.validate().is_err());
        assert!(validate_replies(br#"["hello"]"#, false).is_err());
        assert_eq!(validate_replies(br#"["hello"]"#, true).unwrap(), vec!["hello"]);
        assert!(validate_replies(br#"["1","2","3","4","5"]"#, true).is_err());
        assert!(validate_replies(br#"{"error":"bad"}"#, true).is_err());
        assert!(validate_replies(b"not-json", true).is_err());
        assert!(validate_replies(br#"["\u0000"]"#, true).is_err());
        assert!(validate_replies(serde_json::to_string(&vec!["x".repeat(4097)]).unwrap().as_bytes(), true).is_err());
    }
}

// Fin: Hecho por Lournyx Matterssmith [Lou-Nyx].
