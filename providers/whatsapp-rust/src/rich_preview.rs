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
<title>Matterssmith · Menú HTML</title>
<style>
:root{{color-scheme:dark;--bg:#070b14;--surface:#111827;--ink:#e8f8ff;--muted:#abc3d4;--a:#46e7ff;--b:#7386ff;font:14px/1.45 system-ui,-apple-system,Segoe UI,Arial,sans-serif}}
*{{box-sizing:border-box}}
body{{margin:0;min-height:100vh;padding:20px 12px;background:radial-gradient(ellipse at 75% 5%,#46e7ff28,transparent 43%),radial-gradient(ellipse at 0 80%,#7039ae2b,transparent 48%),var(--bg);color:var(--ink)}}
main{{max-width:470px;margin:0 auto;padding:24px 18px;border:1px solid #46e7ff77;border-radius:24px;background:linear-gradient(155deg,#142037,#0b1221);box-shadow:0 16px 55px #0008,0 0 25px #46e7ff1a}}
pre{{font-family:inherit;font-size:12px;line-height:1.55;white-space:pre-wrap;overflow-wrap:anywhere;margin:0}}
h1{{font-size:22px;line-height:1.1;letter-spacing:-.03em;margin:0 0 9px;color:var(--a)}}
p{{margin:0 0 17px;color:var(--muted);font-size:13px}}
details{{border:1px solid #46e7ff60;border-radius:14px;background:#101b2c;overflow:hidden}}
details[open]{{box-shadow:inset 0 0 28px #46e7ff15}}
summary{{cursor:pointer;padding:15px;font-weight:800;list-style:revert;background:linear-gradient(125deg,#46e7ff15,#7386ff18)}}
summary:focus-visible{{outline:2px solid var(--a);outline-offset:-3px}}
details pre{{padding:16px;border-top:1px solid #364459;max-height:330px;overflow-y:auto}}
.controls{{display:grid;grid-template-columns:repeat(3,1fr);gap:8px;margin-top:18px}}
button{{border-radius:12px;border:1px solid #46e7ff5e;background:#142033;color:var(--ink);padding:11px 3px;opacity:.65}}
small{{display:block;margin:15px 0;color:var(--muted)}}
@media(prefers-reduced-motion:reduce){{*,*:before,*:after{{animation:none!important;transition:none!important}}}}
</style>
</head>
<body><main>
<h1>✦ Matterssmith · Menú</h1>
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
    // The visual language comes from Matter Register (web/matter-register/app.js,
    // feat/matter-register-web): cyan/indigo glow, dark layered glass, pills,
    // border highlights and reduced-motion-aware animation. Keep a compact
    // layout for an in-chat RichResponse card, not the registration form.
    let html = format!(
        r#"<style>
:root{{--matter-bg:#070b14;--matter-panel:#111827;--matter-ink:#e8f8ff;--matter-muted:#abc3d4;--matter-a:#46e7ff;--matter-b:#7386ff}}
*{{box-sizing:border-box}}
html,body{{margin:0;background:transparent;color:var(--matter-ink);font:14px/1.45 system-ui,-apple-system,Segoe UI,Arial,sans-serif}}
.matter-menu{{position:relative;isolation:isolate;overflow:hidden;max-width:440px;margin:0 auto;padding:17px;border-radius:22px;border:1px solid rgba(70,231,255,.47);background:radial-gradient(ellipse at 89% 8%,rgba(70,231,255,.16),transparent 55%),linear-gradient(155deg,#142037,#0b1221);box-shadow:0 12px 38px rgba(0,0,0,.35),0 0 24px rgba(70,231,255,.09)}}
.matter-menu:before{{content:"";position:absolute;z-index:-1;inset:0;pointer-events:none;opacity:.22;background-image:radial-gradient(1px 1px at 12px 20px,#fff,transparent),radial-gradient(2px 2px at 72px 63px,#46e7ff,transparent);background-size:121px 107px,181px 163px;animation:matter-stars 32s linear infinite}}
.matter-brand{{display:flex;gap:11px;align-items:center;margin-bottom:15px}}
.matter-mark{{display:grid;place-items:center;flex:none;width:42px;height:42px;border-radius:14px;border:2px solid var(--matter-a);background:rgba(70,231,255,.07);box-shadow:0 0 17px rgba(70,231,255,.25),inset 0 0 10px rgba(70,231,255,.13);color:var(--matter-a);font-size:24px;font-weight:900}}
.matter-name{{font-weight:900;letter-spacing:.015em;font-size:17px;line-height:1.1}}
.matter-name span{{color:var(--matter-a)}}
.matter-kicker{{color:var(--matter-muted);font-size:11px;margin-top:3px}}
.matter-eyebrow{{color:var(--matter-a);font-size:10px;font-weight:850;letter-spacing:.16em;text-transform:uppercase;margin-bottom:6px}}
.matter-heading{{font-size:22px;line-height:1.14;letter-spacing:-.045em;font-weight:900;margin:0}}
.matter-heading em{{font-style:normal;color:var(--matter-a);text-shadow:0 0 18px rgba(70,231,255,.29)}}
.matter-description{{color:var(--matter-muted);font-size:12px;margin:9px 0 14px}}
.matter-pill{{display:inline-block;border:1px solid #55657b;border-radius:99px;background:#0e1827;padding:5px 10px;font-size:10px;margin:0 5px 14px 0}}
.matter-pill b{{color:var(--matter-a)}}
.matter-details{{overflow:hidden;border-radius:15px;border:1px solid rgba(70,231,255,.37);background:#101b2c}}
.matter-details[open]{{box-shadow:inset 0 0 24px rgba(70,231,255,.08)}}
.matter-details summary{{display:flex;justify-content:space-between;align-items:center;gap:10px;cursor:pointer;list-style:none;padding:14px 15px;font-weight:800;color:var(--matter-ink);background:linear-gradient(125deg,rgba(70,231,255,.12),rgba(115,134,255,.10))}}
.matter-details summary::-webkit-details-marker{{display:none}}
.matter-details summary .matter-arrow{{color:var(--matter-a);font-size:19px;transition:transform .2s ease}}
.matter-details[open] summary .matter-arrow{{transform:rotate(90deg)}}
.matter-details summary:focus-visible{{outline:2px solid var(--matter-a);outline-offset:-3px}}
.matter-details pre{{white-space:pre-wrap;overflow-wrap:anywhere;max-height:330px;overflow-y:auto;margin:0;padding:14px 12px;border-top:1px solid #344355;background:#0b172a;color:var(--matter-ink);font:11px/1.55 ui-monospace,Consolas,monospace}}
.matter-footer{{margin-top:13px;padding-top:11px;border-top:1px solid #364459;color:var(--matter-muted);font-size:10px}}
@keyframes matter-stars{{to{{background-position:121px 107px,-181px -163px}}}}
@media(prefers-reduced-motion:reduce){{.matter-menu:before{{animation:none}}.matter-arrow{{transition:none!important}}}}
</style><section class="matter-menu" aria-label="Matterssmith menú">
<div class="matter-brand"><div class="matter-mark" aria-hidden="true">✦</div><div><div class="matter-name">Matter<span>ssmith</span></div><div class="matter-kicker">Matter Ecosystem · Control</div></div></div>
<div class="matter-eyebrow">Menú interactivo</div>
<h2 class="matter-heading">Tu mundo. <em>Tu Matter.</em></h2>
<p class="matter-description">Los comandos están ocultos hasta que abras la sección.</p>
<div class="matter-pill"><b>✦</b> Categorías</div><div class="matter-pill"><b>◆</b> Interactivo</div>
<details class="matter-details"><summary>Mostrar menú de texto <span class="matter-arrow" aria-hidden="true">›</span></summary><pre>{escaped}</pre></details>
<div class="matter-footer">Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].</div>
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
