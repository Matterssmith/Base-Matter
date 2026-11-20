/*
 * WhatsApp-Matter — embedded read-only Matter Register HTTP preview.
 * Serves exact bundled static bytes via localhost; does not collect PII,
 * perform identity checks, or write anything to Vault/GitHub.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Result};
use buffa::MessageField;
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    task::JoinHandle,
    time::{timeout, Duration},
};
use whatsapp_rust::prelude::wa;

const HTML: &str = include_str!("../../../web/matter-register/index.html");
const APP: &str = include_str!("../../../web/matter-register/app.js");
const CAPABILITIES: &str = r#"{"ready":false,"verification_provider_ready":false,"authenticated_session":false,"legal_documents_published":false}"#;
const MAX_REQUEST_BYTES: usize = 8192;

/// The publication URL is a host-configured public HTTPS location. Never
/// permit user-selected hosts, tokens, credentials, query strings or fragments.
pub fn trusted_preview_url(value: &str) -> Result<&str> {
    if value.len() > 512 || !value.starts_with("https://") || !value.ends_with('/') {
        bail!("untrusted_register_preview_url");
    }
    let tail = &value["https://".len()..];
    let (host, path) = tail
        .split_once('/')
        .ok_or_else(|| anyhow::anyhow!("untrusted_register_preview_url"))?;
    if host.len() < 4
        || host.len() > 253
        || !host.contains('.')
        || host.starts_with('.')
        || host.ends_with('.')
        || host.contains("..")
        || host == "127.0.0.1"
        || host == "0.0.0.0"
        || host == "localhost"
        || !host
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b".-".contains(&b))
        || path.len() > 200
        || !path
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"/-_".contains(&b))
    {
        bail!("untrusted_register_preview_url");
    }
    if host
        .split('.')
        .any(|label| label.is_empty() || label.starts_with('-') || label.ends_with('-'))
    {
        bail!("untrusted_register_preview_url");
    }
    Ok(value)
}

/// A WhatsApp URL button never authorizes registration; the web page is a
/// public demo. Successful send does not prove that phone rendered the button.
pub fn registration_preview_card(value: &str) -> Result<wa::Message> {
    let url = trusted_preview_url(value)?;
    let button = wa::message::interactive_message::native_flow_message::NativeFlowButton {
        name: Some("cta_url".to_owned()),
        button_params_json: Some(
            serde_json::json!({
                "display_text": "Abrir Matter Register",
                "url": url
            })
            .to_string(),
        ),
    };
    Ok(wa::Message {
        interactive_message: MessageField::some(wa::message::InteractiveMessage {
            body: MessageField::some(wa::message::interactive_message::Body {
                text: Some(
                    "✦ Matter Register\nAbre la página interactiva de demostración.\n\
                     No hay registro civil ni verificación biométrica activa.\n\
                     No envíes identificaciones por WhatsApp."
                        .to_owned(),
                ),
            }),
            footer: MessageField::some(wa::message::interactive_message::Footer {
                text: Some("Matterssmith · vista previa pública".to_owned()),
                ..Default::default()
            }),
            interactive_message: Some(
                wa::message::interactive_message::InteractiveMessage::NativeFlowMessage(Box::new(
                    wa::message::interactive_message::NativeFlowMessage {
                        buttons: vec![button],
                        message_version: Some(1),
                        ..Default::default()
                    },
                )),
            ),
            ..Default::default()
        }),
        ..Default::default()
    })
}

struct Response {
    status: &'static str,
    mime: &'static str,
    body: &'static [u8],
    allow: &'static str,
}

fn resolve(method: &str, path: &str) -> Response {
    if method != "GET" && method != "HEAD" {
        return Response {
            status: "405 Method Not Allowed",
            mime: "text/plain; charset=utf-8",
            body: b"method_not_allowed",
            allow: "GET, HEAD",
        };
    }
    let (status, mime, body) = match path {
        "/" | "/index.html" => ("200 OK", "text/html; charset=utf-8", HTML.as_bytes()),
        "/app.js" => ("200 OK", "text/javascript; charset=utf-8", APP.as_bytes()),
        "/api/register/capabilities" => (
            "200 OK",
            "application/json; charset=utf-8",
            CAPABILITIES.as_bytes(),
        ),
        "/healthz" => (
            "200 OK",
            "text/plain; charset=utf-8",
            b"matter-register-preview-ok".as_slice(),
        ),
        _ => (
            "404 Not Found",
            "text/plain; charset=utf-8",
            b"not_found".as_slice(),
        ),
    };
    Response {
        status,
        mime,
        body,
        allow: "GET, HEAD",
    }
}

async fn reply(mut stream: TcpStream) -> Result<()> {
    let mut request = [0u8; MAX_REQUEST_BYTES];
    // Read a complete HTTP/1.1 header, even when TCP splits it into frames.
    // Both total time and total bytes are bounded to prevent exhaustion.
    let count = match timeout(Duration::from_secs(3), async {
        let mut used = 0;
        loop {
            if used == request.len() {
                return Ok::<usize, std::io::Error>(0);
            }
            let n = stream.read(&mut request[used..]).await?;
            if n == 0 {
                return Ok(0);
            }
            used += n;
            if request[..used]
                .windows(4)
                .any(|window| window == b"\r\n\r\n")
            {
                return Ok(used);
            }
        }
    })
    .await
    {
        Ok(Ok(n)) if n > 0 => n,
        _ => return Ok(()),
    };
    let input = std::str::from_utf8(&request[..count]).unwrap_or_default();
    // Refuse unfinished headers and malformed request lines. Never log request
    // paths, headers or remote addresses (they may contain tokens).
    let first = input.lines().next().unwrap_or_default();
    let mut parts = first.split_whitespace();
    let method = parts.next().unwrap_or_default();
    let path = parts.next().unwrap_or_default();
    let protocol = parts.next().unwrap_or_default();
    let result = if input.contains("\r\n\r\n")
        && protocol.starts_with("HTTP/1.")
        && parts.next().is_none()
        && path.starts_with('/')
    {
        resolve(method, path)
    } else {
        Response {
            status: "400 Bad Request",
            mime: "text/plain; charset=utf-8",
            body: b"bad_request",
            allow: "GET, HEAD",
        }
    };
    let headers = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nAllow: {}\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nReferrer-Policy: no-referrer\r\nX-Frame-Options: DENY\r\nCross-Origin-Resource-Policy: same-origin\r\nPermissions-Policy: camera=(), microphone=(), geolocation=()\r\nContent-Security-Policy: default-src 'none'; script-src 'self'; style-src 'unsafe-inline'; img-src 'self' data:; connect-src 'self'; base-uri 'none'; form-action 'self'; frame-ancestors 'none'\r\nConnection: close\r\n\r\n",
        result.status,
        result.mime,
        result.body.len(),
        result.allow
    );
    timeout(Duration::from_secs(4), async {
        stream.write_all(headers.as_bytes()).await?;
        if method != "HEAD" {
            stream.write_all(result.body).await?;
        }
        stream.shutdown().await
    })
    .await??;
    Ok(())
}

/// Starts once alongside the authenticated provider process. The binding is
/// always loopback and isolated from the WhatsApp IPC and Vault sockets.
/// The reverse proxy (not this HTTP listener) must terminate public TLS.
pub async fn start_with_controller_port(controller_port: u16) -> Result<JoinHandle<()>> {
    let port = controller_port
        .checked_add(10)
        .filter(|p| *p >= 1024)
        .ok_or_else(|| anyhow::anyhow!("register_port_invalid"))?;
    start_loopback(port).await
}

pub async fn start_loopback(port: u16) -> Result<JoinHandle<()>> {
    if port < 1024 {
        bail!("register_port_invalid");
    }
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port)).await?;
    println!("MATTER_REGISTER_LOCAL=http://127.0.0.1:{port}/");
    Ok(tokio::spawn(async move {
        loop {
            let Ok((stream, _)) = listener.accept().await else {
                break;
            };
            // One local HTTP request at a time. No unbounded task creation;
            // slow/oversize clients have a hard timeout.
            if reply(stream).await.is_err() {
                eprintln!("Matter Register: preview response failed");
            }
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_only_configured_public_https_link_shape() {
        assert_eq!(
            trusted_preview_url("https://matter.example/register/").unwrap(),
            "https://matter.example/register/"
        );
        for bad in [
            "https://matterssmith.github.io@evil.example/register/",
            "http://matter.example/register/",
            "https://127.0.0.1/register/",
            "https://LOCALHOST/register/",
            "https://bad..example/register/",
            "https://matter.example/register/?token=x",
            "https://matter.example/register/#secret",
            "https://matter.example/../",
            "https://matter.example/%2f/",
            "",
        ] {
            assert!(trusted_preview_url(bad).is_err(), "unexpected valid URL");
        }
    }

    #[test]
    fn static_responses_and_unsafe_methods_fail_closed() {
        assert_eq!(resolve("GET", "/").status, "200 OK");
        assert_eq!(resolve("GET", "/app.js").status, "200 OK");
        assert_eq!(resolve("HEAD", "/healthz").status, "200 OK");
        assert_eq!(
            resolve("POST", "/api/register/sessions").status,
            "405 Method Not Allowed"
        );
        assert_eq!(resolve("GET", "/.git/config").status, "404 Not Found");
        assert_eq!(resolve("GET", "/app.js?token=1").status, "404 Not Found");
        assert!(
            std::str::from_utf8(resolve("GET", "/api/register/capabilities").body)
                .unwrap()
                .contains(r#""ready":false"#)
        );
    }

    #[test]
    fn native_card_never_contains_data_capture() {
        let message = registration_preview_card("https://matter.example/register/").unwrap();
        assert!(message.interactive_message.body.text.is_some());
    }
}
