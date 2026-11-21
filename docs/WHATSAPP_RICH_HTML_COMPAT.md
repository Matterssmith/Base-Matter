## Auditoría LuferOS/Baileys-Rust (2026-11-21)

Se realizó la lectura de [`src/Utils/experiments.rs`](https://github.com/LuferOS/Baileys-Rust/blob/main/src/Utils/experiments.rs), commit fijado `d8e1d41d1b2bb2301385aaa28e196c0fb6855367`. Se conservó la copia exacta en [`third_party/luferos-baileys-rust/experiments.rs`](../third_party/luferos-baileys-rust/experiments.rs), con LICENSE y NOTICE Apache-2.0 y la atribución de **Luis Fernando Guzmán Niño (LuferOS)**. Los tres blobs Git coinciden con los originales.

**Hallazgo:** los builders `NativeButtonInput`, `InteractiveMessage`, `CarouselMessage`, `AiRichResponseMessage` y `extract_experimental_nodes` ofrecen botones y metadatos protobuf experimentales; `WebView` produce `open_webview` con una **URL**, no renderiza directamente HTML arbitrario. Los tipos RichResponse incluyen texto, código, tablas y LaTeX; en este archivo **no hay motor de HTML/CSS/JS**. Los dos unit tests del upstream comprueban la construcción de nodos `biz`/`bot`, no un envío ni una captura del cliente.

**Incompatibilidad:** el provider actual usa `whatsapp-rust =0.7.0` y `buffa`; la muestra usa `baileys-rs` con protobuf `prost`. No se debe hacer `use` directo en el proveedor actual. `Custom` recibe JSON opaco, `WebView` no valida HTTPS por sí solo y no hay límites generales de entrada; antes de portar se requieren validadores de esquema/longitud, URLs HTTPS, permisos, trazabilidad y fallback.

**Estado:** copia externa aislada, no compilada ni desplegada; laboratorio HTML/NativeFlow previo sin cambios. La visualización Android/iOS/Web sigue **sin confirmar**. En implementación futura: adaptación explícita al protobuf del provider elegido, envío en self-chat autorizado y captura de pantalla con versión del cliente; nunca fingir origen Meta AI ni otorgar privilegios por payload.

[Provenance, licencia y límites](../third_party/luferos-baileys-rust/PROVENANCE.md) · [Plan y selector universal](BAILEYS_RUST_EXPERIMENTS_BOT_FACTORY_PLAN.md)

## 2026-11-19 — Burbuja de selección y tarjetas compatibles

- Reporte visual Android: al pulsar «Categorías», el cliente publicaba «☾︎ Anti / Abrir menú» en vez del comando elegido. El selector ahora usa `title: "=/>menu anti"` y elimina `description`. El ID opaco y su enrutamiento HMAC siguen intactos; la burbuja depende de cómo WhatsApp represente esa fila.
- `MATTER_WA_MENU_LAB=1` añade tarjetas NativeFlow de prueba TS18/TS19 en menús raíz de **chats privados habilitados**. Sus botones únicamente emiten ACK de lectura y pasan por las comprobaciones existentes. Si el envío nativo falla, se intenta la tarjeta legacy.
- El chat privado del propietario compara adicionalmente el transporte legacy y sigue siendo el único lugar que puede ensayar Rich HTML y audio. No se envía HTML, multimedia ni comparaciones de legacy a lectores externos o grupos.
- RichResponse HTML no equivale a HTML web y el cliente puede descartarlo sin dar error: estado **experimental sin visualización confirmada**. El HTML offline continúa disponible.
- No se modifican roles, antis, activación, secretos ni derechos de ejecución. Bot-Matter permanece bloqueado. CI Linux/Windows/proveedor y prueba de teléfono para este SHA pendientes. README revisados: 2/2.

## 2026-11-18 — Menú y laboratorio de interfaz en WhatsApp

El menú autorizado conserva las 21 entradas del contrato cerrado,
un botón de categorías y respuesta textual alternativa. Sólo en
self-chat verificado del propietario se activa MATTER_WA_MENU_LAB=1
para enviar tarjetas TS18 (Audio/MP4), TS19 (Audio/MP4/Enlace) y un
tono original de audio Ogg/Opus, además del HTML RichResponse cuando
MATTER_WA_RICH_HTML=1. El audio requiere pulsar reproducir;
HTML no ejecuta JavaScript ni garantiza soporte en WhatsApp.
Ningún click concede privilegios ni ejecuta descargas. Los comandos
en submenús sólo muestran sintaxis; help contiene los detalles.
Leer [WHATSAPP_UI_TESTS.md](WHATSAPP_UI_TESTS.md) para los pasos.

# Base-Matter — Compatibilidad de HTML enriquecido en WhatsApp

**Estado:** vista previa offline validada en `alpha` y ensayo de
transporte RichResponse de propietario **sin compatibilidad visual verificada**.
El ensayo permanece desactivado por defecto.

## Continuidad: intento RichResponse no suplantado (solo self-chat)

El módulo `rich_preview.rs` sigue sin acceso a red: construye una
vista HTML local con los botones de ejemplo deshabilitados y un esquema
JSON para validar campos y límites. La nueva capa
`rich_transport.rs` puede envolver ese esquema como
`wa::AIRichResponseMessage` con ID aleatorio por mensaje, pero
no incluye `botForwardedMessage`, `0@bot`, indicadores de
reenviado o procedencia Meta AI inventada.

`MATTER_WA_RICH_HTML=1` activa **únicamente una prueba** después
de la autorización de `=/>menu` del dueño en su self-chat.
No cambia la ruta de grupos ni externos, no concede funciones
de administración y no abre sesiones adicionales. Si el
transporte falla, intenta NativeFlow y después el texto.
Si el cliente no lo renderiza después de aceptar el transporte,
no existe señal fiable para detectar el fallo. Sigue sin
demostrarse compatibilidad en Android/iOS/Web.

No se copiaron los códigos sin licencia conocida ni los diseños
de la otra marca aportada en el ZIP.

## Qué reveló el material facilitado

El archivo de prueba `xd.zip` del 18-11-2026 incluye Go, JavaScript y HTML
para **menús, juegos y pantallas Rich**. Los principales componentes observados:

- `help_rich.go` genera una plantilla HTML con `html/template`; intenta
  representar una hoja mediante `embedded_screens` y
  `FOAIDNixelButtonSheets`. Advierte que el menú animado no está disponible
  en WhatsApp Web/iPhone en esa implementación.
- `commands_juegos_rich.go` monta `GenAIaeacdsnwHtmlPrimitive` dentro
  de `sections` y `embedded_screens`. Depende de seis HTML en
  `assets/` que **no** están incluidos en el ZIP suministrado.
- `testeos.js` construye `unifiedResponse.data` (JSON codificado en
  base64), con contenidos de HTML/markdown y media. Para enviar, envuelve
  la respuesta con `botForwardedMessage.richResponseMessage`, marca el
  contenido como reenviado y atribuye un supuesto bot `0@bot`.
  Ese origen **no se debe reproducir** en Base-Matter.
- `menu_rich.html` puede servir de inspiración visual. Los HTML y JS
  independientes son muestras de interfaz, no plugins de Base-Matter.

Los nombres `GenAIaeacdsnwHtmlPrimitive` y
`FOAIDNixelButtonSheets` son experimentales; que existan en un JSON
no prueba que un cliente WhatsApp los renderice, ni que una cuenta ajena
a Meta AI tenga permiso para enviarlos.

## Lo que realmente se puede aprovechar

| Componente | Reutilización en Base-Matter |
| --- | --- |
| Menú, CSS y diseño responsive | Vista previa HTML **local** y estática |
| Escape de HTML de plantillas | Escape propio en Rust, sin ejecutar JS |
| JSON con `embedded_screens` y `sections` | Prueba de esquema, **sin enviar** |
| NativeFlow protobuf de `whatsapp-rust` 0.7 | Tres botones reales de solo lectura, opt-in |
| BotJID `0@bot` y `isForwarded` falsificados | **Rechazado**, no suplantar origen |
| Scripts HTML, solicitudes HTTP, audio y juegos | Fuera del núcleo y sin ejecución |
| Supuesta detección Android/iOS por JID | **Rechazada**, no es prueba de dispositivo |

## Prototipo sin WhatsApp

`providers/whatsapp-rust/src/rich_preview.rs` construye dos resultados
**sin requerir variables de entorno, llaves, bases de sesión ni red**:

```bash
cd /home/Sakura/Base-Matter
cargo run --manifest-path providers/whatsapp-rust/Cargo.toml -- --rich-preview-html > matter-preview.html
cargo run --manifest-path providers/whatsapp-rust/Cargo.toml -- --rich-preview-json > matter-preview.json
```

La primera salida es HTML estático, con el **menú original completo**,
texto escapado y tres botones de apariencia deshabilitados. La segunda es
JSON experimental `embedded_screens`/`FOAIDNixelButtonSheets`, sin
`botForwardedMessage`, sin fingir un bot de Meta AI y sin ejecutarse
como mensaje WhatsApp.

Las pruebas impiden scripts, atributos inyectados, datos peligrosos,
sobrecargas y encabezados falsificados en el JSON de ejemplo.
**No se copia ni ejecuta el código del ZIP.** El HTML de vista previa
es un documento local; los botones NativeFlow se prueban por separado.

## Prueba futura legítima

Para poder prometer HTML dentro de WhatsApp hace falta confirmar una
API documentada o un mecanismo legítimo de renderizado para terceros.
El simple envío de `richResponseMessage` no demuestra soporte de
HTML interactivo. Una prueba real deberá incluir Android, iPhone y
Web, sin suplantación, con fallback al menú de texto original.

```
Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
```
