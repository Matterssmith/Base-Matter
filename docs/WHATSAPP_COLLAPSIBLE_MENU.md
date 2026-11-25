# Menú Matter desplegable para WhatsApp — alpha

## 2026-11-24 — Corrección del menú observado en WhatsApp

El propietario confirmó que `=/>menu` mostraba una tarjeta NativeFlow larga («Menú interactivo», «Categorías», «Ver texto», «Página 1/2») **sin HTML visible** en su cliente. Este comportamiento observado demuestra que el selector nativo sí se renderiza, pero **no** que el cliente haya renderizado `AIRichResponseMessage`.

Nuevo comportamiento deseado e implementado en `alpha`:
- `=/>menu`: tarjeta de dos líneas (identidad Matterssmith y «Menú») con **dos respuestas rápidas** del cliente: «Ver más» → `matter.read.menu.page.1` → `=/>menu page 1`, y «Menú clásico» → `matter.read.menu.all` → `=/>menu all`. No muestra páginas, listas ni textos instructivos en el mensaje inicial.
- `=/>menu page 1/2`: selector nativo con categorías implementadas y opción «Menú clásico». Las páginas se muestran únicamente tras seleccionar «Ver más». Todas las acciones se traducen mediante la tabla allowlist autenticada; ninguna acción concederá permisos.
- `=/>menu all`: ahora se envía **como texto convencional de WhatsApp** usando la respuesta completa del controlador `ControlThemes`, sin otro selector. El controlador mantiene el catálogo dinámico y el renderizado original.
- Si `MATTER_WA_RICH_HTML=1`, la tarjeta experimental usa `rich_preview::original_menu()` para evitar rebasar el máximo de bytes con menús dinámicos extensos. El HTML emplea el tema de Matter Register y contiene `<details>` inicialmente cerrado. El envío RichResponse, aun aceptado, **no confirma representación en Android**; el envío nativo breve sigue como respaldo.
- El color de las letras de botones no puede imponerse mediante el protocolo NativeFlow: lo decide el cliente de WhatsApp. Las etiquetas son «Ver más» y «Menú clásico», pero no se promete azul exacto.

`whatsapp-rust 0.7.0` ofrece las estructuras protobuf `AIRichResponseMessage` y `botForwardedMessage`, y las pruebas del empaquetado habían pasado. Eso **no basta para afirmar compatibilidad visual del HTML**. Baileys/AIRich usa el mismo formato experimental y sus distintas implementaciones dependen de la versión y política del cliente; no se debe sustituir la sesión activa ni migrar SQLite por este único síntoma. Si `=/>test html` en chat propio no muestra una tarjeta (aunque el proveedor indique «send accepted»), registrar el resultado y probar un adaptador separado sin tocar `wa-alpha`.

Verificación pendiente de esta revisión: `cargo test --manifest-path providers/whatsapp-rust/Cargo.toml --test ui --test rich_preview --test rich_transport`, `cargo build --manifest-path providers/whatsapp-rust/Cargo.toml`, y prueba visual con la sesión existente. No se ha desplegado desde esta conversación.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].


## 2026-11-24 — Corrección tras ejecución real de pruebas en idkbot

El propietario ejecutó `cargo test --manifest-path providers/whatsapp-rust/Cargo.toml --test ui --test rich_preview --test rich_transport` sobre `alpha`. Se reportaron **dos fallos de assertions** en `--test rich_preview` (2 passed / 2 failed): `inline_html_menu_has_clickable_collapsed_text_without_script` buscaba `=/>status` literal y `sample_rich_screen_shape_is_offline_and_does_not_spoof_meta_ai` buscaba el título plano `Matterssmith · Menú`.

Diagnóstico a partir del código y plantilla originales: el escapado HTML transforma `>` en `&gt;` dentro de `<pre>`, por lo que el HTML serializado contiene `=/&gt;status`, aunque el navegador lo visualiza como `=/>status`. La marca visible también está separada en `Matter<span>ssmith</span>` por el estilo de Matter Register. Se corrigieron **solo los assertions** correspondientes en `tests/rich_preview.rs` y `tests/rich_transport.rs`: exigen el comando escapado (rechazan el sin escapar) y la marca estilizada, conservando las validaciones de contenido, cierre `<details>`, seguridad y transporte.

**Pendiente:** repetir las tres pruebas en `idkbot` después del `git pull --ff-only origin alpha`. No reportar éxito de CI, teléfono o binario por una corrección de tests no ejecutada. Mantener `Cargo.lock` local y el flujo de sesión `--resume-only` existente.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].


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
