## 2026-11-24 — Un solo mensaje por menú, selector final y fallback web visible

**Cambio solicitado:** eliminar los botones rápidos duplicados de cambio de menú y el envío por separado de las tarjetas `menu buttons`. Todos los modos `=/>menu`, `=/>menu classic`, `=/>menu web`, `=/>menu buttons` y las categorías ahora envían **una sola tarjeta NativeFlow** con el contenido en el cuerpo y exactamente **un botón al final: «Seleccionar categoría»**. Su lista separa «Categorías» de «Otros menús», donde figuran `Menu Classic`, `Menu Web` y `Menu Buttons`. Los ID reales continúan limitados al mapeo `readonly_button_command` y pasan por autorización firmada; seleccionar una opción no concede privilegios.

La lista de WhatsApp tiene hasta diez filas por tarjeta. Para las diez categorías implementadas, la primera página incluye seis categorías, «Más categorías» y los tres formatos; la segunda, las cuatro categorías restantes, «Anterior» y los tres formatos. Cada selección genera la respuesta del menú elegido en su propio **único** mensaje; ya no se producen cuatro tarjetas para un solo `menu buttons`. El selector permanece al final en ambas páginas y todos los submenús. Se eliminó también el laboratorio automático de menú, que creaba mensajes adicionales y audio en `menu`; los comandos `=/>test ui`, `=/>test row` y `=/>test html` siguen separados.

**HTML experimental:** se identificó que el antiguo menú `web` enviaba un `AIRichResponseMessage` opcional y a continuación un NativeFlow cuyo cuerpo era solo U+200B, con resultado aparentemente vacío cuando el cliente no representaba el rich HTML. El nuevo menú `web` usa una **única tarjeta nativa con cuerpo visible en WhatsApp**, tema textual Matter y el selector final. El HTML real `GenAIaeacdsnwHtmlPrimitive` queda disponible únicamente para diagnóstico explícito con `=/>test html`, sin enviar tarjetas ocultas ni adicionales durante `menu web`. **Esto es un respaldo visible, no HTML renderizado.** No atribuir compatibilidad visual a una confirmación de envío. Si el usuario necesita HTML real, deberá abrirse en un navegador/webview autorizado y desplegado, no asumir que WhatsApp admite contenido HTML arbitrario.

Los mensajes de consola `Base-Matter DIAG: buttons category card 1 sent` hasta `card 4 sent` provienen del **binario anterior todavía en ejecución** y desaparecerán una vez compilada, verificada y reanudada la nueva versión. Mantener Vault y la sesión sin cambios y pasar por `cargo run` protegido al actualizar. No reiniciar antes de aprobar las suites de núcleo y proveedor.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-24 — Pruebas visuales en grupos y `cargo run` verificado

### Pruebas en grupos, sin saltarse la identidad ni el permiso del chat

En la rama `alpha`, los comandos `=/>test`, `=/>test ui`, `=/>test row`, `=/>test html` y `=/>test ack a|b|one|two|audio|mp4|link` se procesan en **grupos habilitados** por el socket, pero con permisos separados: `ui`, `row` y los ACK de solo lectura pueden utilizarlos remitentes de grupo autenticados; `html` solo puede solicitarlo el **propietario identificado por la sesión** desde el grupo o su chat propio. El controlador sigue aplicando la política del grupo. Las pruebas no cambian roles, secretos, la política de activación ni Vault. Un comando de terceros para `=/>test html` en chat privado se rechaza.

`=/>test html` envía `AIRichResponseMessage` experimental al **mismo chat de grupo** después del ACK autenticado del propietario, siempre que `MATTER_WA_RICH_HTML=1`; `=/>menu web` también puede intentar ese HTML en grupos habilitados cuando lo solicita el propietario y conserva el selector de categorías como respaldo. Otros remitentes pueden solicitar el selector, pero no provocar envíos de HTML experimental. El éxito del transporte no confirma visualización HTML en Android. Las pruebas `ui`/`row` incorporan timeout de 8 segundos para no bloquear el loop principal si la plataforma se demora. Las respuestas visuales se limitan a los comandos explícitos; no se envían automáticamente en todos los mensajes del grupo.

### Arranque seguro con `cargo run` desde la raíz de Base-Matter

El `default-run = "base-matter"` de `Cargo.toml` invoca `src/bin/base_matter.rs`, que ahora llama a `scripts/matter-verified-run.sh` **en lugar del lanzador con pairing**. El nuevo lanzamiento ejecuta, y se detiene ante el primer error:

1. `cargo fmt --all -- --check` para núcleo y proveedor.
2. `cargo check --all-targets` para núcleo y proveedor.
3. `cargo test --all-targets` para núcleo y proveedor.
4. `cargo build --bin whatsapp-matter-control` y `cargo build --manifest-path providers/whatsapp-rust/Cargo.toml --bin whatsapp-matter-rust-provider`.
5. `scripts/whatsapp-alpha-existing.sh --check`, con binarios absolutos derivados de `cargo metadata` y `--vault-env /etc/base-matter/vault-alpha.env`.
6. Si pasó todo y no corre otra instancia: `--resume-only` de la sesión WhatsApp alfa existente **sin solicitar pairing**.

`cargo run -- --check-only` ejecuta las mismas verificaciones pero **no inicia WhatsApp**. `cargo run -- --help` únicamente imprime instrucciones. El script requiere rama `alpha`, Cargo, Python3, `pgrep`, los archivos Vault dedicados, no ejecuta un segundo bot cuando ve uno activo y no baja a modo sin Vault silenciosamente. Usa `CARGO_BUILD_JOBS=2` de forma predeterminada y preserva opciones explícitas de HTML, NativeFlow y diagnósticos. Las pruebas son una barrera contra regresiones, **no una prueba absoluta de seguridad**; la autenticación y autorización siguen dentro del proveedor, control y Vault.

Uso desde `/home/Sakura/Matter-Ecosystem/Base-Matter`, en `tmux` después de detener la instancia anterior del bot (no detener Vault): `export PATH="$HOME/.cargo/bin:$PATH"; cargo run`. Con el bot activo, usar `cargo run -- --check-only` puede fallar en el chequeo de sesión activa; la revisión se diseña para realizarse antes del reinicio, no para sobrescribir ejecutables en uso.

**Estado:** código publicado en `alpha`, CI y validación en servidor pendientes de confirmar para esta revisión; no se ha alterado `runtime.env`, las bases de datos reales ni la clave Vault. No declarar HTML visualmente funcional hasta verlo en el cliente.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-24 — Corrección de flags visuales en el lanzador alfa

Se identificó que `scripts/whatsapp-alpha-existing.sh` ejecutaba `source runtime.env` después de recibir las variables de entorno para HTML y botones. Si el archivo privado tenía opciones anteriores, podía sustituir silenciosamente `MATTER_WA_RICH_HTML=1`, `MATTER_WA_HTML_AUTOSEND=1` o `MATTER_WA_NATIVE_BUTTONS=1`. Ahora solo se preservan **valores explícitos 0/1** de estas cinco variables de presentación: `MATTER_WA_RICH_HTML`, `MATTER_WA_HTML_AUTOSEND`, `MATTER_WA_NATIVE_BUTTONS`, `MATTER_WA_DIAG` y `MATTER_WA_MENU_LAB`. No se permite sobrescribir claves, identificadores, sesiones, puente IPC, ni configuración de Vault.

La salida de `--check` muestra `ALPHA_UI_HTML`, `ALPHA_UI_NATIVE_BUTTONS`, `ALPHA_UI_DIAG` y `ALPHA_HTML_CLIENT_RENDER=UNVERIFIED` sin secretos. Un `ALPHA_UI_HTML=1` demuestra que el binario recibió el permiso para intentar el envío, **no** que Android lo renderice.

Comprobar antes del reinicio: `bash -n scripts/whatsapp-alpha-existing.sh`, `--check` con los binarios nuevos, y solo entonces `--resume-only` tras detener únicamente la instancia anterior. Mantener intactos Vault y las bases de datos alfa.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

# Menú Matter desplegable para WhatsApp — alpha

## 2026-11-24 — Tres menús: web (predeterminado), classic y buttons

El propietario solicitó tres vistas del menú WhatsApp sin instrucciones visibles en la tarjeta principal:

| Comando | Resultado |
| --- | --- |
| `=/>menu` | Modo guardado del socket (predeterminado `web`) |
| `=/>menu web` | Intenta tarjeta HTML experimental en chat propio y envía controles nativos: «Seleccionar categoría», «Menú clásico», «Menu Buttons» |
| `=/>menu classic` | Envía menú tradicional completo **en texto** y una tarjeta de controles «Seleccionar categoría», «Menú web», «Menu Buttons» |
| `=/>menu buttons` | Envía primero una tarjeta con botones **Menu Classic** y **Menu Web**, además de «Seleccionar categoría»; después cuatro tarjetas de categorías con hasta tres botones reales cada una |
| `=/>menu all` | Inventario tradicional completo más controles de navegación, sin alterar sus comandos |
| `=/>set menu` | Consultar la preferencia actual; solo propietario autorizado, chat privado |
| `=/>set menu web|classic|buttons` | Guardar la preferencia para las siguientes llamadas a `=/>menu`, manteniendo `web` como predeterminado de fábrica |

Las categorías salen del manifiesto `templates/menu-visible-categories.json`: **únicamente las categorías implementadas**, con los mismos IDs autorizados por el controlador. Todos los controles de respuesta rápida siguen siendo solo lectura y llegan al dispatcher autenticado; **no** se crea un atajo de rol. La preferencia SQLite se migra de `CHECK('web','classic')` a `CHECK('web','classic','buttons')` transaccionalmente y preserva el estado anterior. El código de envío incorpora límites de espera y detiene las tarjetas adicionales en el primer fallo; la tarjeta principal de selección queda disponible para navegar incluso en ese caso.

**Límite de WhatsApp:** los botones rápidos permiten normalmente hasta tres por mensaje, y las listas permiten hasta diez filas. Por eso el modo `buttons` no intenta meter diez botones en una sola tarjeta. En determinados clientes, `NativeFlow` puede aceptarse sin que el dispositivo dibuje controles; el resultado del envío no equivale a una confirmación de representación. La opción «Seleccionar categoría» permanece en todas las vistas.

**HTML aún sin confirmación:** `=/>menu web` transporta `AIRichResponseMessage`/ `GenAIaeacdsnwHtmlPrimitive` experimental y solo en chat propio verificado con `MATTER_WA_RICH_HTML=1`. Según la investigación pública de implementaciones Baileys, ese primitivo depende del cliente Android y no tiene equivalencia garantizada en Web/Desktop/iOS. El proveedor no debe suplantar identidades de bots ni fabricar firmas de Meta AI para forzar su aparición. El HTML no puede considerarse funcional en el teléfono hasta que el usuario confirme que visualiza la tarjeta y pulsa su contenido. El comando `=/>test html` envía una confirmación de diagnóstico; `MATTER_WA_DIAG=1` deja registro de aceptación o rechazo del transporte, sin secretos.

**Verificación necesaria antes de despliegue:** `cargo fmt --all -- --check` y `cargo fmt --manifest-path providers/whatsapp-rust/Cargo.toml --all -- --check`; `cargo test --lib menu::tests`; `cargo test --test control_ipc`; `cargo test --manifest-path providers/whatsapp-rust/Cargo.toml --test provider --test ui --test rich_preview --test rich_transport` y compilación de ambos binarios. No detener Vault ni solicitar pairing.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].


## 2026-11-24 — Menú web predeterminado y elección persistente por propietario

Nuevos comandos de WhatsApp en el controlador `Base-Matter/alpha`:

| Comando | Función |
| --- | --- |
| `=/>menu` | Abre el modo predeterminado del socket; **web** en instalaciones nuevas |
| `=/>menu web` | Solicita explícitamente HTML experimental `AIRichResponseMessage` (el selector de categorías y botón clásico permanecen como respaldo) |
| `=/>menu classic` | Envía **el menú original de categorías en texto** |
| `=/>menu all` | Envía el inventario completo de comandos en texto |
| `=/>set menu` | Consulta el modo guardado; requiere chat privado y propietario autenticado |
| `=/>set menu web` | Guarda `web` como predeterminado; requiere propietario autenticado |
| `=/>set menu classic` | Guarda `classic` como predeterminado; requiere propietario autenticado |

**Persistencia:** `src/menu.rs` extiende el SQLite privado de preferencias existente con la columna `mode TEXT NOT NULL DEFAULT 'web' CHECK(mode IN ('web','classic'))`. Los archivos con esquema previo reciben una migración `ALTER TABLE`, preservando estilo `classic|none` e idioma `es|en` y los demás datos. El controlador expone `current_menu_mode` tras autorización, y únicamente para un `=/>menu` autenticado incluye el atributo `menu_mode` en su respuesta IPC. El proveedor valida exactamente `web|classic`; jamás utiliza texto escrito por terceros para escoger privilegios. `=/>set menu` es **configuración de transporte**, diferente de `=/>set style`.

**Presentación:** en modo web, la tarjeta NativeFlow de respaldo tiene **cuerpo invisible**, sin instrucciones ni pie y muestra un selector de categorías (únicamente las implementadas) y una respuesta rápida `Menú clásico` mapeada a `=/>menu classic`. El menú HTML usa `<details>` cerrado con tema Matter Register. La visualización del HTML en Android **no está confirmada**: el envío protobuf correcto no obliga al cliente WhatsApp a representarlo. El envío enriquecido necesita `MATTER_WA_RICH_HTML=1` y, por seguridad, se limita por ahora al chat privado propio del propietario autenticado; otros chats siguen teniendo el selector nativo. `MATTER_WA_HTML_AUTOSEND=0` permite suspender solo el envío HTML automático para diagnóstico sin alterar la preferencia persistida. La consola mantiene `MATTER_WA_DIAG=1` opcional y límites de espera.

**Estado:** cambios guardados en GitHub `alpha`. Compilación Rust, CI y representación real en `idkbot` siguen pendientes de ejecutar para esta revisión; no se modificó la sesión ni Vault. No intentar pairing ni crear una segunda sesión.

Pruebas sugeridas después de compilar controlador y proveedor: `cargo test --lib menu::tests`, `cargo test --test control_ipc`, `cargo test --manifest-path providers/whatsapp-rust/Cargo.toml --test provider --test ui --test rich_preview --test rich_transport` y las dos compilaciones; primero `=/>set menu`, `=/>set menu classic`, `=/>menu`, `=/>set menu web`, `=/>menu`, `=/>menu web`, `=/>menu classic`, `=/>test html`.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].


## 2026-11-24 — Prueba de todas las interfaces sin desactivar HTML

Aclaración del propietario: desea mantener **HTML RichResponse y NativeFlow habilitados** mientras se diagnostican las respuestas intermitentes. Los valores recomendados para probar simultáneamente son:

- `MATTER_WA_RICH_HTML=1`: habilita transporte experimental `AIRichResponseMessage`, incluida la prueba explícita `=/>test html`.
- `MATTER_WA_HTML_AUTOSEND=1`: permite enviar la tarjeta HTML junto al menú raíz `=/>menu`, **solo en chat privado de propietario autenticado**; el menú nativo sigue saliendo como segunda tarjeta y el texto clásico sigue accesible.
- `MATTER_WA_NATIVE_BUTTONS=1`: mantiene los botones «Ver más» y «Menú clásico» y los experimentos `=/>test ui`.
- `MATTER_WA_DIAG=1`: logs de recepción/validación/transporte sin IDs, números ni secretos.
- El experimento de estrés `MATTER_WA_MENU_LAB=1` adicional envía varias tarjetas de prueba TS18/TS19 y audio **en cada** `=/>menu` de chat privado, por lo que no debe confundirse con habilitar HTML: dejarlo sin activar durante pruebas de recepción de `status`, y usar `=/>test html`, `=/>test ui`, `=/>test row` para ejercitar los formatos sin tráfico repetido.

Se añadieron timeouts de ocho segundos a los envíos del laboratorio HTML/NativeFlow/ButtonsRow/audio en `providers/whatsapp-rust/src/main.rs`, además de los ya presentes en menú y respuestas. Si el HTML automático ya fue enviado, el laboratorio evita repetirlo. Sigue pendiente compilar y probar en `idkbot` y verificar que el cliente real represente HTML.

**Advertencia:** el éxito de la compilación/protobuf o del envío no demuestra que WhatsApp dibuje el contenido HTML, ni que `status` tenga confiabilidad restaurada. Mantener la sesión existente y el Vault sin cambios.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].


## 2026-11-24 — Diagnóstico de estado WhatsApp: un comando y luego silencio

Incidente comunicado por el propietario en `idkbot`: tras actualizar `wa-alpha`, el proveedor sigue conectado, pero `=/>status` responde una vez y los comandos posteriores pueden quedar sin respuesta, sin error visible. **Causa no demostrada en un dispositivo todavía**.

Análisis del código: `providers/whatsapp-rust/src/main.rs` recibía eventos en `on_message` y esperaba sin plazo para `ctx.reply` / `ctx.send_message`. Con `MATTER_WA_RICH_HTML=1`, la raíz `=/>menu` podía hacer dos envíos (HTML experimental + NativeFlow) durante un mismo evento. Un envío pendiente es una causa plausible de bloqueo silencioso; alternativamente, el proveedor puede no recibir más eventos o el filtro de identidad/antirreplay puede descartar mensajes. Los mensajes `Ignored` eran principalmente silenciosos.

Cambios exclusivamente en `Base-Matter/alpha`: se agregó `tokio::time::timeout(Duration::from_secs(8), ...)` para mensajes normales, menú nativo, texto clásico y prueba/envío de HTML enriquecido; se separó el HTML automático del menú bajo `MATTER_WA_HTML_AUTOSEND=1` **además** de `MATTER_WA_RICH_HTML=1`, solo para chat propio del propietario. Por defecto se **suspende el HTML automático** para estabilizar `status`, sin quitar la ruta explícita de `=/>test html`. Con `MATTER_WA_DIAG=1` el proveedor imprime exclusivamente etapas `callback received`, `control candidate received`, `control signed`, `control not admitted`, `controller replied duplicate`, `controller ignored`, `reply timed out`, o fallos de envío: **nunca contenido, JID, IDs de mensajes ni claves**.

Prueba en servidor (se debe compilar el **nuevo** binario del proveedor antes de retomar):
```bash
cd /home/Sakura/Matter-Ecosystem/Base-Matter
git fetch origin alpha && git merge --ff-only origin/alpha
CARGO_BUILD_JOBS=2 cargo test --manifest-path providers/whatsapp-rust/Cargo.toml --test ui --test rich_preview --test rich_transport
CARGO_BUILD_JOBS=2 cargo build --manifest-path providers/whatsapp-rust/Cargo.toml
```
Tras detener exclusivamente el proceso alpha original de manera controlada, usar la sesión `--resume-only`, con el `CONTROLLER` y `PROVIDER` **reales**, Vault habilitado como antes y `MATTER_WA_DIAG=1 MATTER_WA_HTML_AUTOSEND=0 MATTER_WA_RICH_HTML=1 MATTER_WA_NATIVE_BUTTONS=1`. Probar en el mismo chat `=/>status` tres veces con mensajes nuevos, después `=/>menu`, `=/>status`. No borrar `session.db`, `runtime.env`, el almacén de replay, ni reiniciar Vault; no permitir otra instancia paralela.

Interpretación del log: sin nuevos `callback received`, investigar entrega upstream/session; si hay callback sin `candidate`, inspeccionar tipo de mensaje; con `not admitted`, revisar identidad y alcance sin desactivar permisos; con `duplicate`, verificar IDs reales localmente bajo estrictas condiciones de privacidad; con `ignored`, revisar estado y autorización; con `timed out`, examinar envíos y consumidor WhatsApp. **No llamar al bug resuelto sin confirmación de teléfono y pruebas.**

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].


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
