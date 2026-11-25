/*
 * WhatsApp-Matter — offline-only research of experimental Rich HTML envelopes.
 *
 * Créditos:
 * Hecho por Lournyx Matterssmith [Lou-Nyx].
 */
use anyhow::{bail, Result};
use base64::Engine;
use serde_json::{json, Value};

const MAX_TEXT_BYTES: usize = 3800;
const MAX_HTML_BYTES: usize = 16 * 1024;

/// Build a safe, fully offline HTML preview of the verified original menu.
///
/// This is a browser preview, NOT an HTML message sent to WhatsApp.
pub fn menu_html_preview(menu: &str) -> Result<String> {
    if menu.is_empty()
        || menu.len() > MAX_TEXT_BYTES
        || menu.chars().any(|c| c.is_control() && c != '\n')
    {
        bail!("Invalid Matterssmith menu for HTML preview");
    }
    let mut escaped = String::with_capacity(menu.len() + 128);
    for ch in menu.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    let audio_data =
        base64::engine::general_purpose::STANDARD.encode(crate::menu_sound::menu_tone_bytes());
    let result = format!(
        r#"<!doctype html>
<html lang="es">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>Base-Matter · Vista previa experimental</title>
<style>
:root{{color-scheme:dark}}
*{{box-sizing:border-box}}
body{{margin:0;background:#090d18;color:#f3f7ff;font-family:system-ui,sans-serif;padding:18px}}
main{{max-width:470px;margin:0 auto;padding:22px 16px;border:1px solid #31466d;border-radius:17px;background:linear-gradient(150deg,#101c38,#14111f)}}
pre{{font-family:inherit;font-size:12px;line-height:1.55;white-space:pre-wrap;overflow-wrap:anywhere;margin:0}}
h1{{font-size:20px;margin:0 0 8px}}
p{{margin:0 0 16px;color:#b7c8e7;font-size:13px}}
details{{border:1px solid #536c99;border-radius:12px;background:#172440;overflow:hidden}}
summary{{cursor:pointer;padding:14px;font-weight:700;list-style:revert}}
details pre{{padding:16px;border-top:1px solid #536c99}}
.controls{{display:grid;grid-template-columns:repeat(3,1fr);gap:8px;margin-top:18px}}
button{{border-radius:10px;border:1px solid #576d9d;background:#273554;color:#fff;padding:11px 3px;opacity:.8}}
small{{display:block;margin:15px 0;color:#a5b8d8}}
</style>
</head>
<body><main>
<h1>Matterssmith · Menú</h1>
<p>Abre esta sección para consultar los comandos. WhatsApp usa su selector nativo, no este documento HTML.</p>
<details><summary>Mostrar menú de texto</summary><pre>{escaped}</pre></details>
<div class="controls">
<button type="button" disabled>Audio</button>
<button type="button" disabled>MP4</button>
<button type="button" disabled>Enlace</button>
</div>
<audio controls preload="none" aria-label="Sonido original"><source src="data:audio/ogg;base64,{audio_data}" type="audio/ogg; codecs=opus"></audio>
<small>Vista HTML experimental. Audio pulsable; nunca autoplay. Los botones activos son mensajes nativos separados.</small>
</main></body></html>"#
    );
    if result.len() > MAX_HTML_BYTES {
        bail!("HTML preview exceeds offline budget");
    }
    Ok(result)
}

/// Compact, self-contained HTML/CSS fragment for supported in-chat Rich UI.
/// No external resources, JavaScript, automatic audio, or executable commands.
/// The authenticated NativeFlow selector handles real chat commands separately.
pub fn menu_inline_html(menu: &str) -> Result<String> {
    if menu.is_empty()
        || menu.len() > MAX_TEXT_BYTES
        || menu.chars().any(|c| c.is_control() && c != '\n')
    {
        bail!("Invalid menu for inline HTML");
    }
    let mut escaped = String::with_capacity(menu.len() + 128);
    for ch in menu.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    let html = format!(
        r#"<style>
*{{box-sizing:border-box}}
html,body{{margin:0;background:transparent;color:#f3f6ff;font-family:system-ui,sans-serif}}
.matter-card{{border:1px solid #4a628f;border-radius:16px;background:linear-gradient(145deg,#111f3c,#17121f);padding:14px;max-width:480px}}
.matter-title{{font-weight:800;color:#d9e9ff;font-size:17px}}
.matter-tip{{margin:6px 0 12px;color:#acbfdc;font-size:12px}}
details{{border:1px solid #3e5881;border-radius:10px;background:#1a2a49}}
summary{{cursor:pointer;padding:12px;font-weight:700;font-size:13px}}
pre{{white-space:pre-wrap;overflow-wrap:anywhere;font-family:inherit;font-size:12px;line-height:1.5;padding:12px;border-top:1px solid #3e5881;margin:0}}
small{{display:block;margin-top:10px;font-size:10px;color:#9caecf}}
</style><section class="matter-card">
<div class="matter-title">Matterssmith · Menú</div>
<p class="matter-tip">Toca el encabezado para mostrar u ocultar los comandos.</p>
<details><summary>Mostrar menú de texto</summary><pre>{escaped}</pre></details>
<small>Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].</small>
</section>"#
    );
    if html.len() > MAX_HTML_BYTES {
        bail!("Inline HTML menu exceeds safe bounds");
    }
    Ok(html)
}

/// Reconstruct *only* the research JSON hierarchy present in the supplied
/// Go/JS examples. This function never makes or sends a WA Message.
///
/// The experimental Meta AI names are preserved for compatibility analysis.
/// In particular we never wrap the result in botForwardedMessage or assert
/// that the sender is `0@bot`.
pub fn experimental_rich_schema(menu: &str) -> Result<Value> {
    let html = menu_inline_html(menu)?;
    let output = json!({
        "response_id": "matterssmith-preview-v1",
        "sections": [{
            "view_model": {
                "__typename": "GenAISingleLayoutViewModel",
                "primitive": {
                    "__typename": "GenAIaeacdsnwHtmlPrimitive",
                    "payload": html,
                    "trusted_sources": []
                }
            }
        }],
        "embedded_screens": [{
            "title": "Base-Matter · Vista previa",
            "content": [{
                "__typename": "FOAIDNixelButtonSheets",
                "tabs": [{
                    "id": "matterssmith-control",
                    "tab_header": "Matterssmith",
                    "sections": [{
                        "view_model": {
                            "__typename": "GenAISingleLayoutViewModel",
                            "primitive": {
                                "__typename": "GenAIaeacdsnwHtmlPrimitive",
                                "payload": html,
                                "trusted_sources": []
                            }
                        }
                    }]
                }]
            }]
        }]
    });
    // Defensive size bound for future local tooling that consumes the
    // serializer; no external network, scripts or dynamic embeds.
    if output.to_string().len() > MAX_HTML_BYTES * 2 {
        bail!("Rich preview structure exceeds offline budget");
    }
    Ok(output)
}

pub fn original_menu() -> &'static str {
    include_str!("../../../templates/matterssmith-menu.txt").trim_ascii_end()
}
