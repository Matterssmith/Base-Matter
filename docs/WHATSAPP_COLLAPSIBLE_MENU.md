# Menú Matter desplegable para WhatsApp — alpha

## 2026-11-24 — Tema del menú HTML basado en Matter Register

El propietario pidió usar Matter Register como referencia visual directa. Se revisó `web/matter-register/app.js` desde la rama `feat/matter-register-web`: variables originales `--bg:#070b14`, `--a:#46e7ff`, `--b:#7386ff`, `--ink:#e8f8ff`, `--muted:#abc3d4`; paneles de vidrio oscuro, bordes luminosos, marca con símbolo ✦, partículas sutiles, chips y tratamiento de `prefers-reduced-motion`.

Se adaptaron **sin copiar formularios ni identidad civil** esos elementos a `providers/whatsapp-rust/src/rich_preview.rs`: `menu_inline_html` crea una tarjeta compacta Matterssmith con `<details>` cerrado, texto escapado, estética cian/índigo, encabezado y crédito. `menu_html_preview` para navegador conserva sus pruebas de audio y tarjetas desactivadas y ahora usa la misma familia visual. Las regresiones en `providers/whatsapp-rust/tests/rich_preview.rs` comprueban paleta, cierre por defecto y escape de HTML.

El envío enriquecido sigue opt-in mediante `MATTER_WA_RICH_HTML=1` y la validación Android/CI sigue pendiente; no se ha actualizado un binario en idkbot ni interrumpido WhatsApp o Vault. El HTML local colapsado **no ejecuta comandos de chat por sí solo**: los comandos reales permanecen en NativeFlow y en el dispatcher autenticado. Si un cliente no interpreta Rich UI, usar `=/>menu all` o el selector nativo.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].


## 2026-11-24 — HTML enriquecido dentro del chat (experimento real)

Se revisó el código de `jbeckforddev/Aura-Bot-2.0` (rama `master`): `cmd/socket/menu.ts` aún envía texto con previsualización; en cambio `cmd/funy/xd1.ts`, `cmd/system/Testxd3.ts` y los juegos usan `GenAIaeacdsnwHtmlPrimitive` dentro de `AIRichResponseMessage`, habitualmente anidado en `botForwardedMessage` (protobuf 104). El formato es **experimental**, no una interfaz HTML pública oficial estable ni una prueba de renderizado en cada teléfono.

Se adaptó el protocolo a Rust, sin copiar diseños de Aura: `rich_preview::menu_inline_html` crea una tarjeta Matterssmith original con HTML/CSS y `<details>` cerrado por defecto, escapando el texto; `experimental_rich_schema` incluye el HTML directo en `sections` y en el contenedor de pestaña; `rich_transport` lo empaqueta en `wa::message::FutureProofMessage`/ `bot_forwarded_message` como envoltorio. No se fabrican comprobantes, nombres o JIDs de Meta AI. La prueba `=/>test html` sigue limitada al propietario en su chat propio.

Con `MATTER_WA_RICH_HTML=1` en la configuración **privada ya existente del proveedor** (nunca en Git), `=/>menu` envía la tarjeta HTML a un chat previamente autorizado por el controlador y, además, el selector NativeFlow pequeño para accionar categorías de lectura. Este segundo mensaje es necesario porque pulsar un elemento del HTML no autoriza ni envía por sí solo un comando WhatsApp. No se crean permisos nuevos. Con la variable ausente u OFF queda la interfaz nativa/texto. Un éxito de `send_message` solo acredita envío; el móvil debe comprobar el contenido visible.

**Pruebas requeridas antes de despliegue**:
`cargo fmt --manifest-path providers/whatsapp-rust/Cargo.toml -- --check`;
`cargo test --manifest-path providers/whatsapp-rust/Cargo.toml --test rich_preview --test rich_transport --test ui`.
Después compilar el proveedor de forma aislada, comparar el binario y hacer cambio controlado con respaldo y `--resume-only`, sin duplicar WhatsApp ni tocar Vault.

Estado actual: código y regresiones actualizados en `alpha`. Sin validación Cargo, CI ni prueba real en WhatsApp en esta conversación. Si el cliente no muestra HTML, conservar el selector nativo y usar `=/>menu all` para texto.


Estado: cambio de código en `alpha`, **sin despliegue en idkbot ni verificación visual en teléfono**. Fecha: 2026-11-24.

## Contrato visible solicitado

- `=/>menu` / `=/>menú` presenta una tarjeta **breve** con dos controles nativos: selector `Categorías` y respuesta rápida `Ver texto`. No muestra inmediatamente el menú largo, las métricas del propietario ni el catálogo completo.
- Al seleccionar una fila del selector, el ID autorizado se resuelve mediante la lista fija en `providers/whatsapp-rust/src/ui.rs`; la respuesta ejecuta un comando **de lectura** como `=/>menu anti`, que entrega únicamente el detalle pedido. No se permite ejecutar una acción a partir de los títulos de las filas.
- El botón `Ver texto` usa exclusivamente `matter.read.menu.all` → `=/>menu all`. El menú completo aparece **solo cuando se solicita**.
- Las páginas del selector mantienen un cuerpo breve y las secciones con comandos no inventados. La selección está sujeta al estado del chat, identidad, HMAC y protección antirreplay existentes; las respuestas de texto siguen siendo fallback.
- La presentación de los mensajes nativos queda habilitada por defecto **solo al actualizar al binario de alpha**. Para desactivarla por incompatibilidad visual, definir `MATTER_WA_NATIVE_BUTTONS=0` y reiniciar el proveedor **con el procedimiento de recuperación existente** (sin crear otra sesión).
- Si el envío del selector falla, se envía el texto autenticado normal. Si WhatsApp acepta el envío pero **no dibuja** el botón, el fallback **manual** es escribir `=/>menu all` o `=/>menu anti`. No podemos detectar de forma fiable el renderizado en el móvil.

## Diferencia frente a HTML

WhatsApp no ejecuta HTML arbitrario dentro de mensajes de un bot. `native_flow_message` genera controles **de WhatsApp**, no un documento HTML. Las experiencias WhatsApp Flows oficiales requieren su infraestructura y cuentas habilitadas; sus pantallas se describen en Flow JSON y tampoco son HTML libre.

`providers/whatsapp-rust/src/rich_preview.rs` ofrece una **vista previa HTML para navegador**, con `<details>` cerrado por defecto y el texto visible al pulsar «Mostrar menú de texto». Se obtiene con `--rich-preview-html` y no se publica automáticamente como mensaje. El transporte experimental `AIRichResponseMessage` existente es solo un ensayo de propietario en chat propio, nunca una función oficial garantizada. No suplantar Meta AI, no activar HTML experimental en grupos y no inferir renderizado a partir del éxito del envío.

## Validación y despliegue seguro

1. Desde `Base-Matter/alpha`, compilar y ejecutar pruebas del proveedor con Rust:
   `cargo test --manifest-path providers/whatsapp-rust/Cargo.toml --test ui`
   y `cargo test --manifest-path providers/whatsapp-rust/Cargo.toml --test rich_preview`.
2. Probar en Android / iOS / Web `=/>menu`, selector, cambio de página, categoría y botón «Ver texto»; confirmar que **la respuesta del botón** llega al controlador y que el ID no concede permisos.
3. Probar `MATTER_WA_NATIVE_BUTTONS=0`: el menú completo en texto se debe conservar. Verificar también `=/>help` independiente del selector, menús de propietario sin fugas y chat desactivado sin nuevos permisos.
4. Revisar CI Linux/Windows, formato Rust y pruebas de seguridad. El cambio en GitHub **no modifica el binario ya ejecutándose**.
5. Para cambiar el proveedor de `idkbot`, usar el procedimiento documentado de `scripts/whatsapp-alpha-existing.sh`, una sola sesión existente y `--resume-only` cuando corresponda. No borrar almacén de sesión, no generar pairing nuevo, no duplicar proveedor ni reiniciar Vault. Hacer rollback al binario anterior si no se muestra el selector.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
