/*!
 * Matter-Rust: safe localhost-only static capability explorer in Rust.
 *
 * This interface intentionally cannot register accounts, send messages,
 * edit Vault or access authenticated provider sessions.
 * Deploy behind a trusted HTTPS reverse proxy if remote preview is needed.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use std::{
    io::{self, Read, Write},
    net::{Ipv4Addr, Shutdown, SocketAddrV4, TcpListener, TcpStream},
    time::Duration,
};

const HTML: &str = include_str!("../web/index.html");
const APP: &str = include_str!("../web/app.js");
const CATALOG: &str = include_str!("../web/catalog.json");
const HEALTH: &str = "matter-rust-preview-ok";
const MAX_HEADERS: usize = 8192;
pub const DEFAULT_PREVIEW_PORT: u16 = 39841;

#[derive(Clone, Copy)]
struct Response {
    status: &'static str,
    mime: &'static str,
    bytes: &'static [u8],
}

fn resolve(method: &str, path: &str) -> Response {
    if method != "GET" && method != "HEAD" {
        return Response {
            status: "405 Method Not Allowed",
            mime: "text/plain; charset=utf-8",
            bytes: b"method_not_allowed",
        };
    }
    let (status, mime, bytes) = match path {
        "/" | "/index.html" => ("200 OK", "text/html; charset=utf-8", HTML.as_bytes()),
        "/app.js" => ("200 OK", "text/javascript; charset=utf-8", APP.as_bytes()),
        "/api/catalog" => ("200 OK", "application/json; charset=utf-8", CATALOG.as_bytes()),
        "/healthz" => ("200 OK", "text/plain; charset=utf-8", HEALTH.as_bytes()),
        _ => ("404 Not Found", "text/plain; charset=utf-8", b"not_found".as_slice()),
    };
    Response {
        status,
        mime,
        bytes,
    }
}

fn parse_request(input: &str) -> Response {
    let Some(line) = input.split("\r\n").next() else {
        return bad_request();
    };
    let mut parts = line.split_whitespace();
    let (Some(method), Some(path), Some(version), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return bad_request();
    };
    if !input.contains("\r\n\r\n")
        || !(version == "HTTP/1.1" || version == "HTTP/1.0")
        || !path.starts_with('/')
    {
        return bad_request();
    }
    resolve(method, path)
}

fn bad_request() -> Response {
    Response {
        status: "400 Bad Request",
        mime: "text/plain; charset=utf-8",
        bytes: b"bad_request",
    }
}

fn handle(mut stream: TcpStream) -> io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    stream.set_write_timeout(Some(Duration::from_secs(4)))?;
    let mut buf = [0_u8; MAX_HEADERS];
    let mut used = 0;
    loop {
        if used == MAX_HEADERS {
            // Header overflow cannot influence application handlers.
            return Ok(());
        }
        let n = stream.read(&mut buf[used..])?;
        if n == 0 {
            return Ok(());
        }
        used += n;
        if buf[..used].windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
    }
    let request = std::str::from_utf8(&buf[..used]).unwrap_or_default();
    let response = parse_request(request);
    let method = request.split_ascii_whitespace().next().unwrap_or_default();
    let header = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nAllow: GET, HEAD\r\nX-Content-Type-Options: nosniff\r\nReferrer-Policy: no-referrer\r\nX-Frame-Options: DENY\r\nCross-Origin-Resource-Policy: same-origin\r\nPermissions-Policy: camera=(), microphone=(), geolocation=()\r\nContent-Security-Policy: default-src 'none'; script-src 'self'; style-src 'unsafe-inline'; connect-src 'self'; img-src 'self' data:; base-uri 'none'; form-action 'none'; frame-ancestors 'none'; object-src 'none'\r\nConnection: close\r\n\r\n",
        response.status,
        response.mime,
        response.bytes.len()
    );
    stream.write_all(header.as_bytes())?;
    if method != "HEAD" {
        stream.write_all(response.bytes)?;
    }
    stream.flush()?;
    let _ = stream.shutdown(Shutdown::Write);
    Ok(())
}

/// Only loopback is accepted; no public bind flag exists.
/// Does not start any platform transports or read credentials.
pub fn serve_preview(port: u16) -> io::Result<()> {
    if port < 1024 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "preview port must be >= 1024",
        ));
    }
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port))?;
    println!("Matter-Rust preview: http://127.0.0.1:{port}/");
    println!("No live sessions. No Vault access. No registration endpoint.");
    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                if handle(stream).is_err() {
                    eprintln!("Matter-Rust: local HTTP request failed");
                }
            }
            Err(_) => eprintln!("Matter-Rust: local connection failed"),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_serves_whitelisted_readonly_paths() {
        assert_eq!(resolve("GET", "/").status, "200 OK");
        assert_eq!(resolve("GET", "/app.js").status, "200 OK");
        assert_eq!(resolve("HEAD", "/api/catalog").status, "200 OK");
        assert_eq!(resolve("GET", "/healthz").status, "200 OK");
        assert_eq!(resolve("GET", "/.git/config").status, "404 Not Found");
        assert_eq!(resolve("GET", "/api/roles").status, "404 Not Found");
        assert_eq!(resolve("GET", "/api/catalog?token=test").status, "404 Not Found");
        assert_eq!(resolve("POST", "/api/roles").status, "405 Method Not Allowed");
        assert_eq!(resolve("POST", "/api/register/sessions").status, "405 Method Not Allowed");
    }

    #[test]
    fn invalid_protocol_and_unfinished_headers_rejected() {
        assert_eq!(parse_request("GET / HTTP/1.1\r\n\r\n").status, "200 OK");
        assert_eq!(parse_request("HEAD /app.js HTTP/1.0\r\n\r\n").status, "200 OK");
        assert_eq!(parse_request("GET / HTTP/1.1\r\n").status, "400 Bad Request");
        assert_eq!(parse_request("GET / HTTP/2.0\r\n\r\n").status, "400 Bad Request");
        assert_eq!(parse_request("GET http://test/ HTTP/1.1\r\n\r\n").status, "400 Bad Request");
    }

    #[test]
    fn html_is_only_javascript_loader_and_catalog_is_readonly_reference() {
        assert!(HTML.contains("<script defer src=\"./app.js\"></script>"));
        assert!(!HTML.contains("<form"));
        assert!(!HTML.contains("<style"));
        assert!(!HTML.contains("<section"));
        assert!(APP.contains("document.body.appendChild(app)"));
        assert!(CATALOG.contains("\"schema_version\": 1"));
        assert!(CATALOG.contains("Hecho por Lournyx Matterssmith [Lou-Nyx]"));
        assert!(CATALOG.contains("\"planned\""));
        assert!(CATALOG.contains("\"reference\""));
    }
}
