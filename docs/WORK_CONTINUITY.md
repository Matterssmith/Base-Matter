## 2026-11-25 — Rustfmt del catálogo compartido

CI del SHA 0a2b1e6 y servidor idkbot detuvieron el arranque en formato. Aplicadas todas las diferencias exactas de rustfmt de los dos workflows Linux: src/menu.rs, src/control_ipc.rs y proveedor src/lib.rs y src/ui.rs. Sin cambios de comportamiento, permisos, sesión ni Vault. README revisados: 2/2. Nuevo CI pendiente; no declarar aprobación hasta comprobar resultados.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-25 — Vistas independientes y categoría compartida

Solicitud del propietario: web sin texto de respaldo ni marca en el selector; Buttons no debe copiar el clásico; un ID canónico por categoría con presentación según el formato. El controlador deriva descriptores de botones del mismo CATEGORIES/CommandSpec y los adjunta únicamente al ACK autorizado. El proveedor muestra respuestas rápidas de hasta tres por tarjeta y selector final exclusivo «Otros menús». Los ID de navegación Buttons contienen el ID canónico y conservan el contexto al pulsarlos incluso desde una tarjeta anterior; el último formato se recuerda por chat/remitente en memoria acotada para los comandos escritos. Los botones de comandos sin argumentos seguros muestran ayuda explícita, no mutan configuración. Todo agrega navegación por categorías; las páginas de una categoría conservan su identificador. El catálogo web por categoría queda pendiente: las consultas desde web siguen usando clásico mientras no se verifique el HTML. Web no tiene cuerpo nativo textual; HTML y selector son transportes separados, sin garantía de renderizado del selector sin cuerpo. No se modifica Vault, autenticación ni sesiones. README revisados: 2/2. CI, formato y prueba real pendientes.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-25 — Clippy del proveedor y verificación del formato web

CI del SHA e076cfd: núcleo aprobado y proveedor pasó formato, compilación y pruebas antes de fallar en Clippy por single_match (main.rs). Se sustituye el match exterior por if let sin cambiar la admisión, los ACK ni el silencio ante rechazos. Se mantiene la corrección rustfmt de tests/ui.rs. README revisados: 2/2. Nuevo CI pendiente; no se modifica la sesión ni Vault.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-25 — Formato de la prueba de menú web

El propietario reportó bloqueo MATTER_GATE=FORMAT en idkbot. Se aplica exactamente la diferencia de rustfmt en providers/whatsapp-rust/tests/ui.rs; no cambia el comportamiento, la sesión ni Vault. README revisados: 2/2. CI y arranque pendientes de confirmación.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-25 — Separar HTML real del menú clásico

El propietario informa que web muestra el marco clásico precedido por «Menú web». Se retira ese encabezado y la copia del cuerpo clásico del NativeFlow web. El proveedor vuelve a intentar el RichResponse HTML real con el contenido autorizado del controlador, solo para el propietario de la sesión autenticada en chat propio o grupo habilitado; requiere MATTER_WA_RICH_HTML=1 y, para el menú raíz automático, MATTER_WA_HTML_AUTOSEND=1. El selector nativo «Otros menús» conserva exclusivamente los formatos alternativos y un cuerpo mínimo de marca. **Excepción de transporte:** web puede enviar dos mensajes (HTML experimental y tarjeta nativa de navegación), ya que no hay representación combinada verificada; classic/buttons siguen con una sola tarjeta. Si HTML no se representa, no se suplanta con el clásico. La aceptación del envío no confirma el renderizado del cliente. Prueba de regresión del cuerpo web añadido; CI y dispositivo pendientes. Sin cambios en Vault, permisos ni sesiones. README revisados: 2/2.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-25 — Consola sin spam de recepción

Por solicitud del propietario, el proveedor WhatsApp ya no imprime callbacks generales, respuestas interactivas desconocidas, ediciones, candidatos, rechazos previos de firma ni ACK ignorados o duplicados. El flujo de comandos retorna temprano para texto sin prefijo de control y registra una sola confirmación al recibir ACK aplicado del controlador; el diagnóstico distingue una selección interactiva aceptada sin registrar contenido ni identificadores. La observación pasiva necesaria para antis permanece intacta y silenciosa en operación normal; una caída de transporte se informa una vez hasta recuperación. Se conservan errores operativos de comandos y pruebas explícitas, identidad, firma, antirreplay, Vault y sesión. README revisados: 2/2. CI y despliegue pendientes.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-25 — Selector exclusivo «Otros menús»

Corrección solicitada por el propietario: la tarjeta conserva su contenido y un único selector final «Otros menús». No se muestran categorías, páginas ni accesos a categorías en ese selector. Cada formato explícito ofrece los otros dos; la vista sin formato específico ofrece los tres. Las categorías y los comandos manuales del catálogo se conservan. El proveedor conserva el contenido real del controlador en web/buttons como respaldo textual, no HTML real, y resuelve correctamente el modo clásico persistido. El receptor admite variantes de ID de selección y nombre NativeFlow ausente, siempre limitados a la lista de comandos de lectura existente; mantiene HMAC, identidad y antirreplay. Se agregaron pruebas de ida y vuelta de cada formato, variantes de respuesta y rechazo de mutación. El diagnóstico no registra contenido ni identificadores privados. README revisados: 2/2. CI, instalación y prueba de selección real en WhatsApp pendientes; no se cambian sesiones ni Vault.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-24 — Auditoría CI: enlaces del README del proveedor

La última ejecución pasó formato, compilación, pruebas del núcleo, documentación, Clippy y registros de continuidad; la auditoría de enlaces detectó una ruta relativa incorrecta en `providers/whatsapp-rust/README.md`: `docs/ENGINE_EXTENSION_REFACTOR_PLAN.md` no se resuelve desde la carpeta del proveedor. Se corrigió a `../../docs/ENGINE_EXTENSION_REFACTOR_PLAN.md`, cuyo objetivo sí existe. Sin cambios de permisos, sesiones, Vault, conexión ni proveedor de WhatsApp. El menú sigue unificado en una sola tarjeta con el selector final y respaldo visible para `menu web`. CI de este commit pendiente.

README revisados: 2/2.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-24 — Menú WhatsApp unificado: un mensaje y selector al final

**Decisión:** cada vista (`=/>menu`, `=/>menu web`, `=/>menu classic`, `=/>menu buttons`, `=/>menu all` y categorías) responde con **un único NativeFlow**; elimina tarjetas extra, quick replies de cambio de formato y audio automático. Un solo botón «Seleccionar categoría» al final abre una lista de categorías implementadas y, en sección «Otros menús», `Menu Classic`, `Menu Web`, `Menu Buttons`; paginación 6+4 categorías para respetar las diez filas por lista. Los IDs son los mismos allowlist de lectura y cada selección conserva autorización del controlador y protección antirreproducción.

**Corrección de visibilidad:** el modo web ya no utiliza cuerpo invisible U+200B ni envía automáticamente `AIRichResponseMessage`; muestra texto compatible en la misma tarjeta que contiene el selector. HTML arbitrario no está garantizado por WhatsApp y el único experimento explícito sigue siendo `=/>test html`. Un `send_message` aceptado no equivale a renderizado. Los cuatro logs anteriores `buttons category card X sent` indican que en `idkbot` sigue cargado el **binario viejo**, no esta revisión.

**Despliegue:** rama `alpha`, sin tocar Vault ni sesiones. `cargo run` solo reanuda cuando formato, check, test, build y preflight pasen; no iniciar otra instancia. README revisados: 2/2. Pruebas CI de la última revisión pendientes de resultado final.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

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

## 2026-11-24 — Menú buttons adicional, navegación completa y HTML experimental

El menú principal ahora admite los modos `web` (predeterminado), `classic` y `buttons`. `=/>menu` usa la preferencia SQLite del socket; `=/>menu web`, `=/>menu classic` y `=/>menu buttons` son variantes explícitas. El propietario cambia con `=/>set menu web|classic|buttons` y consulta con `=/>set menu` en chat privado verificado. SQLite migra transaccionalmente del anterior `CHECK(mode IN ('web','classic'))` a tres valores, conservando estilo e idioma.

**Interacción solicitada:** cada modo contiene botón «Seleccionar categoría» (`single_select` con la lista autorizada de categorías implementadas) y accesos a los otros formatos. `classic` entrega el antiguo texto más botones hacia `web` y `buttons`; `web` intenta la tarjeta HTML experimental y conserva el selector nativo más `classic`/`buttons`; `buttons` comienza con «Menu Classic» y «Menu Web», luego «Seleccionar categoría», y continúa con botones reales para cada una de las diez categorías implementadas en cuatro tarjetas de hasta tres respuestas rápidas. Las tarjetas usan timeout de cuatro segundos; ante el primer fallo se detienen las restantes y queda el selector. Ningún botón ejecuta directamente mutaciones o concede roles.

**Límite funcional:** la vista HTML `GenAIaeacdsnwHtmlPrimitive` no se ha demostrado visible en el cliente WhatsApp del usuario. Se mantiene como experimento para el chat propio con `MATTER_WA_RICH_HTML=1`, **no** se falsifican pruebas de identidad Meta ni firmas, y la vista nativa funciona como respaldo. Envío aceptado != HTML representado. Confirmar el resultado de `=/>test html` y los diagnósticos `MATTER_WA_DIAG=1`.

Guía detallada: `docs/WHATSAPP_COLLAPSIBLE_MENU.md`. Las suites y compilaciones deben pasar antes de reiniciar `idkbot`. Los cambios están en la rama `alpha`, sin modificación de sesiones ni Vault.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-24 — Pruebas `provider`: plantilla y firma externa sin privilegios

En `idkbot` se reportaron dos fallos de `cargo test --manifest-path providers/whatsapp-rust/Cargo.toml --test provider`: `standalone_provider_uses_exact_same_external_matterssmith_template` y `incoming_readonly_commands_bind_real_sender_and_never_gain_owner_authority` (17 pruebas sí pasaban).

La primera aserción duplicaba un prefijo obsoleto `╭─💠`, diferente de la plantilla oficial `templates/matterssmith-control.txt` con `╭─ 💠`. El test ahora carga la plantilla con `include_str!` y comprueba prefijo/sufijo exactos, texto, saltos de línea e idempotencia.

La segunda aserción consideraba inválidos `=/>group on`, `=/>bot on` y `=/>set role 12345678901 admin` a nivel de firma. Sin embargo, el proveedor permite enviar estos intentos como eventos **externos y no privilegiados** para que el controlador compruebe después si el actor es un owner global verificado por Vault. La prueba se corrigió para exigir `from_me=false`, `actor_id` igual al remitente real y distinto de `config.actor_id`, JID de destino intacto y HMAC íntegro. La lista negativa conserva comandos desconocidos, variantes inválidas y el probe HTML del propietario. **Firmar no concede autoridad; Vault sigue siendo obligatorio para ejecutar acciones.** No se modificó el código de autorización.

Commit correctivo en `Base-Matter/alpha`: `61d1eea64c102f17a1bf828388a9e4e0bfddf101`. Validación pendiente en `idkbot`:

```bash
cd /home/Sakura/Matter-Ecosystem/Base-Matter && export PATH="$HOME/.cargo/bin:$PATH" && git fetch origin alpha && git merge --ff-only origin/alpha && CARGO_BUILD_JOBS=2 cargo test --manifest-path providers/whatsapp-rust/Cargo.toml --test provider
```

Solo después de obtener el resultado verde continuar con el resto de pruebas/compilación. No borrar `runtime.env`, `session.db` ni Vault.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-24 — Reparación de regresión control_ipc en idkbot

La prueba `local_tcp_loopback_roundtrip_rejects_replay_before_second_effect` falló porque esperaba el prefijo literal `╭─💠 𝗖𝗼𝗻𝘁𝗿𝗼𝗹`, mientras la plantilla oficial `templates/matterssmith-control.txt` contiene `╭─ 💠 𝗖𝗼𝗻𝘁𝗿𝗼𝗹` (espacio intencional). La prueba de integración ahora importa la plantilla real con `include_str!`, divide por `{{BODY}}` y verifica prefijo, sufijo, contenido y saltos de línea sin asumir un espaciado duplicado. No se cambiaron la lógica de producción, la autenticación ni Vault por este error.

Corrección publicada en `Base-Matter/alpha`: commit `2962d25f8574a1e45b1088f03b3f335a70baba97` (o SHA posterior si hubo más commits). Validación pendiente de `cargo test --test control_ipc` en `idkbot`; no afirmar pruebas verdes antes de ejecutar.

Comando local: `cd /home/Sakura/Matter-Ecosystem/Base-Matter && git fetch origin alpha && git merge --ff-only origin/alpha && CARGO_BUILD_JOBS=2 cargo test --test control_ipc`.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-24 — Preferencia del menú WhatsApp: web por defecto

En `Base-Matter/alpha` se agregaron `=/>menu web`, `=/>menu classic`, `=/>set menu`, `=/>set menu web` y `=/>set menu classic`. La preferencia `web|classic` se guarda en el SQLite privado de menús del controlador (predeterminado **web**), se migra desde esquemas existentes sin alterar el idioma/estilo, y se comunica al proveedor como metadato de una respuesta IPC previamente autenticada para `=/>menu`. En el modo web se intenta HTML experimental en chat propio cuando `MATTER_WA_RICH_HTML=1`, junto con el respaldo NativeFlow: categorías y botón «Menú clásico», sin cuerpo visible. El modo clásico devuelve el antiguo menú de texto. `menu all` conserva su catálogo completo.

No se ha comprobado que WhatsApp Android represente el HTML, ni compilado/deplegado esta revisión en `idkbot`. Mantener sesión alfa y Vault sin modificar. Guía completa: [docs/WHATSAPP_COLLAPSIBLE_MENU.md](docs/WHATSAPP_COLLAPSIBLE_MENU.md).

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

## 2026-11-24 — Tema del menú HTML basado en Matter Register

El propietario pidió usar Matter Register como referencia visual directa. Se revisó `web/matter-register/app.js` desde la rama `feat/matter-register-web`: variables originales `--bg:#070b14`, `--a:#46e7ff`, `--b:#7386ff`, `--ink:#e8f8ff`, `--muted:#abc3d4`; paneles de vidrio oscuro, bordes luminosos, marca con símbolo ✦, partículas sutiles, chips y tratamiento de `prefers-reduced-motion`.

Se adaptaron **sin copiar formularios ni identidad civil** esos elementos a `providers/whatsapp-rust/src/rich_preview.rs`: `menu_inline_html` crea una tarjeta compacta Matterssmith con `<details>` cerrado, texto escapado, estética cian/índigo, encabezado y crédito. `menu_html_preview` para navegador conserva sus pruebas de audio y tarjetas desactivadas y ahora usa la misma familia visual. Las regresiones en `providers/whatsapp-rust/tests/rich_preview.rs` comprueban paleta, cierre por defecto y escape de HTML.

El envío enriquecido sigue opt-in mediante `MATTER_WA_RICH_HTML=1` y la validación Android/CI sigue pendiente; no se ha actualizado un binario en idkbot ni interrumpido WhatsApp o Vault. El HTML local colapsado **no ejecuta comandos de chat por sí solo**: los comandos reales permanecen en NativeFlow y en el dispatcher autenticado. Si un cliente no interpreta Rich UI, usar `=/>menu all` o el selector nativo.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-24 — Menú WhatsApp compacto y desplegable (alpha)

Se implementó en el proveedor `whatsapp-rust` una tarjeta raíz `=/>menu` **breve** con selector nativo de categorías y botón `Ver texto`, sin mostrar el catálogo completo hasta el clic. Los IDs existentes devuelven `=/>menu <categoría>` o `=/>menu all` tras autorización, HMAC y comprobación antirreplay, sin nuevos privilegios; `=/>help` conserva su comportamiento. En alpha el selector queda habilitado por defecto con opción `MATTER_WA_NATIVE_BUTTONS=0` para regresar al texto; si el cliente acepta un envío pero no dibuja botones, el usuario puede escribir `=/>menu all` manualmente. La vista HTML offline utiliza `<details>` cerrado por defecto; **WhatsApp no interpreta HTML arbitrario** y su RichResponse experimental no equivale a soporte del cliente. Código y pruebas escritos en alpha; **CI/compilación, renderizado Android y despliegue en idkbot todavía no verificados**. Sesión WhatsApp, Vault y beta/stable no modificados. Detalles: [docs/WHATSAPP_COLLAPSIBLE_MENU.md](docs/WHATSAPP_COLLAPSIBLE_MENU.md).

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Arquitectura aprobada: Engine-Matter y Extension-Matter

El propietario dispone que WhatsApp-Matter se renombre a **Engine-Matter** (núcleo de sistemas Matter sin soporte directo WhatsApp/Telegram/Discord) y Telegram-Matter se renombre a **Extension-Matter** (conectores externos WhatsApp/Telegram/Discord y futuras redes). Discord-Matter será retirado de forma controlada con respaldo. **Matter-MSEL/** tendrá su ubicación en Engine-Matter como integración del lenguaje oficial independiente MSEL. Social-Matter es sistema propio del motor; Bot-Matter sigue bloqueado. **Extensions-Matter** (plural) no es el nuevo Extension-Matter y continúa separado. Se conserva Base-Matter como biblioteca y contratos sin dependencias cíclicas.

La preparación actual modifica solo documentación de `alpha`: no mover el runtime/proveedor WhatsApp, no tocar sesiones, cifrado, comandos ni generar nuevos sockets. Plan completo en [docs/ENGINE_EXTENSION_REFACTOR_PLAN.md](docs/ENGINE_EXTENSION_REFACTOR_PLAN.md). Revisados README de raíz y de proveedor; ningún despliegue realizado.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Activación segura de Vault alpha documentada para idkbot

Nueva guía `docs/ACTIVAR_VAULT_ALPHA.md`: diagnosticar el servicio y `base-wa-alpha`, probar ping y self-test sintético, verificar el lanzador `--check --vault-env`, y reiniciar de manera controlada el único proveedor alpha existente con `--resume-only --vault-env`. Preserva `wa-alpha`, evita reenrolar, mezclar `wa-test-1`, generar pairing o tocar las claves. Hace explícita la validación CI de binarios, revisión de credenciales y limitación del grant de configuración. La activación no desbloquea `=/>register start`, roles ni registro civil. Documentación reflejada en Vault-Matter alpha; no hay despliegue realizado desde esta conversación. README raíz y README proveedor revisados (2/2); sólo documentación alpha.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Corrección del test de cierre visual del menú tras CI

El commit 4b1a307 cambió correctamente el espacio de «╰─╮╭────────⟨ 🌀», pero la prueba unitaria src/menu.rs continuaba buscando el texto anterior sin el espacio. Los tests predeterminados de Linux y Windows fallaron por esa aserción obsoleta; formato y compilación pasaron. Se actualiza únicamente la expectativa del test para igualarla al diseño aprobado, sin modificar renderizador, sesiones, Vault, proveedores ni procesos en ejecución. CI del SHA resultante pendiente. README revisados: 2/2; documentación de desarrollo y continuidad sincronizada.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Orden y espacios del menú clásico de control global

- Orden aprobado de la vista clásica pública: Categorías → Información Matter → ficha de Usuario → Instrucciones, conservando el marco ornamental y la identidad Matter.
- Espaciado literal verificado: encabezados con «╭─ 💠», cierre de categorías con «⟨ 🌀», sin línea vacía entre el conector «││» y la información; una línea vacía entre información/ficha de usuario y entre ficha/instrucciones. Sin triples saltos de línea.
- Los bloques exclusivos del propietario verificado («Control del socket» y «Sistema · Host») continúan disponibles después de la ficha; no se revelan al público. No se cambian permisos, sesiones ni lógica de comandos.
- Los campos dinámicos se conservan (fecha, versión, cantidades y datos del usuario). Se actualizan las plantillas, el formateo de los bloques opcionales y las aserciones de tests/menu_catalog.rs. Cambios solo en alpha; CI y prueba real en WhatsApp pendientes.
- Revisión documental de ambos README, docs/DEVELOPMENT_STATUS.md, docs/WORK_CONTINUITY.md y docs/MENU_TEMPLATE_ENGINE.md.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Control del ingreso WhatsApp: rechazo estricto de variantes de lectura

El CI del SHA cec5997 reportó 17/18 pruebas del proveedor correctas y fallo en incoming_readonly_commands_bind_real_sender_and_never_gain_owner_authority. Causa: el fallback genérico de signed_public_readonly admitía variantes rechazadas por el catálogo (por ejemplo =/>menu unset, =/>status --debug, =/>test run). Se limita ese fallback a verbos con potencial de mutación que el controlador y Vault autentican; todas las rutas de menú, estado, ayuda, prueba visual, Vault y crédito usan patrones explícitos de admisión. Se conserva verificación de remitente, HMAC, alcance de socket y ausencia de privilegios automáticos para terceros. Prueba real de vinculación no realizada, nuevo CI pendiente. No tocar sesiones, bases SQLite ni servicios idkbot. README revisados (2/2).

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Test de menú corregido para etiqueta de plantilla

CI Linux de 0fa6284 verificó Rustfmt y compiló; 6/7 pruebas de menu_catalog pasaron y una conservaba la etiqueta literal obsoleta «Comandos de catálogo» pese al contrato nuevo @LabelCommands: @Commands. La prueba ahora valida Comandos: [menu::command_count()] en vez de un texto anterior. No cambia fuente de datos ni estilos, no hay despliegue WhatsApp. README revisados: 2/2. Mantener registros continuos y MSEL P0. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Compilación del motor de menú: corrección de préstamo mutable

GitHub Actions para 8bd87f verificó cargo fmt, pero cargo check identificó E0499 en src/menu/theme.rs: el argumento de v.insert("Pagination", if...) intentaba modificar v dentro de otro préstamo mutable. Se calcula pagination primero y se inserta después, sin cambiar estilos, comandos ni permisos. CI del nuevo SHA pendiente; no se instaló en WhatsApp. README revisados: 2/2, continuidad sincronizada por commit. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Ajuste exacto de rustfmt al motor de plantillas

El CI Linux del SHA 7606e5a mostró únicamente diferencias de formato en src/menu.rs, src/menu/theme.rs y src/menu/telemetry.rs; este commit incorpora los 28 cambios exactos de rustfmt de GitHub Actions. Se conserva la separación de diseño en templates/menu/, datos dinámicos y seguridad; no hay cambios en sesiones WhatsApp, Vault o permisos. No declarar aprobación hasta verificar CI del SHA actualizado. README revisados: 2/2. Continúa MSEL P0. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Continuidad obligatoria: menú estructurado con tokens

Se reemplazó el diseño añadido en el paso previo, rechazado por el propietario, por motor de plantillas. Leer docs/MENU_TEMPLATE_ENGINE.md ANTES de tocar estilos. Archivos clave: templates/menu/theme.json, templates/menu/classic/*.txt, templates/menu/plain/*.txt, src/menu/theme.rs. Menú raíz/submenús/lista all se componen de @Variables y Category.es/en; no asumir categoría por carpeta ni codificar cantidad de comandos. Se conservó el contrato de permisos, apagado por defecto, categorías vacías ocultas. tests/menu_catalog.rs y pruebas del módulo actualizadas. Datos sin fuente quedan no verificados. README revisados: 2/2 (raíz y provider). Próximo: comprobar CI exacto de alpha; si falla, reparar código/formato y mantener simultáneamente los cuatro documentos AGENTS.md, DECISION_LOG.md, DEVELOPMENT_STATUS.md, WORK_CONTINUITY.md en cada commit. Prohibido actualizar procesos idkbot live antes de validar y coordinar. MSEL sigue siendo P0.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Reanudación del menú y cierre de continuidad per-commit

Fuente original: idea del usuario inspirada en estructura informativa externa. Implementados campos con valores del proceso controlador y host, privacidad de propietario socket, fecha, versión, prefijo, comandos de catálogo; sub-bots/premium/URL/provider RSS y uptime siguen sin fuente real. Código src/menu/telemetry.rs, src/menu.rs; prueba tests/menu_catalog.rs; especificación docs/MENU_TELEMETRY_AUTOMATION.md. A 73311eba CI Linux fmt/check/tests/Clippy correctos, pero verificador de cuatro documentos falló; ahora se publican en commit conjunto AGENTS.md, docs/DECISION_LOG.md, docs/DEVELOPMENT_STATUS.md y docs/WORK_CONTINUITY.md. README revisados: 2/2 (README.md y providers/whatsapp-rust/README.md). Próximo: CI completo del SHA, en especial Windows/proveedor y control de continuidad, después sincronizar clon alpha en idkbot. Sin desplegar binarios ni tocar servicios existentes. Conservar Matter-MSEL y MSEL P0 según sus normas.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Punto de reanudación: menú con información dinámica

Base-Matter/alpha agregó `src/menu/telemetry.rs`, integró telemetría en `src/menu.rs`, amplió `tests/menu_catalog.rs` y definió el contrato en `docs/MENU_TELEMETRY_AUTOMATION.md`. Campos reales implementados: fecha local, versión Base, comando de catálogo, host, sistema, RAM host, RAM y uptime **del controlador**. Host/recursos solo en menú de propietario autenticado. Sub-bots/premium y RAM/uptime reales del proveedor siguen pendientes de un registro firmado. Ver CI y ejecutar `cargo test --test menu_catalog` antes de desplegar. El servidor `idkbot` solo tiene el clon en `alpha`; no se ha iniciado ni actualizado ningún proveedor real. Conservar MSEL como P0 y la rama estable intacta. Revisar ambos README por commit, conservar créditos.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-21 — Rustfmt CI import ordering

La verificación Linux del SHA `aaea9f45205c` detectó únicamente el orden rustfmt de imports condicionados a Linux en `tests/msel_bridge.rs`. Aplicada la corrección exacta de GitHub Actions `44954393053`. Se conservan importaciones condicionales en Windows, pruebas de host MSEL real opt-in, permisos y scripts originales. README revisados: 2/2. Nuevo SHA pendiente de confirmación CI.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-21 — Windows Clippy MSEL test imports

En CI de Base-Matter Windows, `cargo check` y todas las pruebas Rust terminaron correctamente para el SHA `1cb11f4`, pero Clippy `-D warnings` detectó dos imports sin uso (`MselCommandRunner` y `PathBuf`) en `tests/msel_bridge.rs`, ya que el test que los usa es solo Linux. Se limitan los imports a `#[cfg(target_os = "linux")]`, conservando las pruebas multiplataforma y el test original opcional. No cambia el motor MSEL, sesiones WhatsApp ni Vault. README revisados: 2/2. [CI de diagnóstico](https://github.com/Matterssmith/Base-Matter/actions/runs/44954149724). Nuevo CI pendiente.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-21 — Test real opt-in Base → lenguaje MSEL

El pipeline Base-Matter Rust en Linux finalizó **SUCCESS** para el commit `1cb11f4b2f36fe963e3d00a6e324c7f6af2c1e0c` ([ejecución 44954149724](https://github.com/Matterssmith/Base-Matter/actions/runs/44954149724), job Linux). Se añade test explícitamente ignorado por defecto `original_msel_checkout_e2e` para conectar **el host Rust de Base** con la **biblioteca nativa C++ real de MSEL privado** en el servidor del dueño, sin copiar ni exponer su código. Requiere `MATTER_MSEL_CHECKOUT` y compilación de `build/libmsel_embed.so`. El nuevo test no afirma ejecución verificada hasta que se ejecute. Ninguna sesión, llave ni transporte WhatsApp se abre o cambia. README revisados: 2/2. Bot-Matter bloqueado.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-21 — Reparación de compilación del puente MSEL

GitHub Actions detectó error Rust E0451 en `tests/msel_bridge.rs`: el test intentaba usar sintaxis de actualización de struct que accede al campo privado `allow_reply`. Corregido asignando `timeout = Duration::ZERO` a una instancia creada mediante el constructor público y conservando `allow_reply` privado para evitar concesión de permisos por campos. Es corrección de compilación/test, no modifica sesión ni comandos de WhatsApp. Pruebas del nuevo SHA y CI pendientes de confirmar. README revisados: 2/2. Se preservan todas las instrucciones previas y créditos.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-21 — Integración Base/WhatsApp con lenguaje original MSEL

Prioridad solicitada tras respaldo de Base. Fuente de verdad MSEL/alpha privado con `msel_bot_message_v1` y nuevo CLI `bot-event` JSON; Base-Matter/alpha añade `src/msel_bridge.rs`, `InboundGateway.ingest_msel` y pruebas; WhatsApp-Matter/matter agrega conversión de evento normalizado al contrato ABI. No se copia un intérprete ni se tocan sesiones reales; no se habilita respuesta live. Revisión de README revisados: 2/2 (raíz y proveedor whatsapp-rust). Scripts, entrada y límites en `docs/MSEL_WHATSAPP_INTEGRATION.md`. Los commit intermedios de código deberán considerarse de desarrollo; la verificación de continuidad final queda en este commit atómico. Problemas originales de formato Rust corregidos siguiendo logs de CI. Pendientes: CI Linux/Windows real, cablear adaptador autenticado y transporte autorizado, prueba aislada con bot ON/OFF, inspección de logs sin datos privados. Bot-Matter sigue bloqueado.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-21 — Rectificación de prioridades: MSEL es el lenguaje oficial (P0)

- **MSEL (Matter Script Extension Language)** es el **lenguaje de programación propio de Matter**, no un sinónimo de Matter-MSEL. **Prioridad P0 actual: diseñar, implementar y probar MSEL como lenguaje autónomo hasta que ejecute programas reales.**
- **Matter-MSEL** es el componente posterior de **integración/soporte de MSEL con Matter Ecosystem**, comparable a Matter-Rust; no sustituye el núcleo del lenguaje ni recibe la etiqueta P0.
- **Orden obligatorio de trabajo nuevo:** L0 especificación y gramática → L1 lexer/parser localizado → L2 AST, tipos e IR → L3 intérprete/compilador y CLI → L4 ejecución independiente verificada → I0 Matter-MSEL y SDK → I1 bots con Base-Matter → I2/I3 interoperabilidad con 32 entradas tecnológicas.
- Mantener la matriz de 32 entradas y las extensiones .msel y .<idioma>.msel; Rust se repite en la lista original. No declarar ninguna integración terminada sin tests. Los servicios y correcciones de seguridad existentes de Base-Matter/Vault-Matter siguen siendo atendidos.
- **Documentación normativa:** [README.md](../README.md), [MATTER_MSEL_MASTER_SPEC.md](MATTER_MSEL_MASTER_SPEC.md), [MSEL_LANGUAGE_V0_DESIGN.md](MSEL_LANGUAGE_V0_DESIGN.md). En AGENTS.md de raíz los enlaces de esta línea son informativos; las rutas canónicas son README.md y docs/*.md.
- **Estado:** documentos corregidos en rama alpha; no se ha escrito, compilado o probado un intérprete de MSEL en este cambio. README revisados: raíz y proveedor WhatsApp. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — CI: enlace de permisos globales con WhatsApp

- Se aplicaron las diferencias exactas `rustfmt` al filtro de comandos del proveedor. El controlador Linux ya superó formato, pruebas y Clippy.
- Los owners 1 y 2 poseen igual autoridad una vez autenticados mediante JID firmado y cliente Vault. Los remitentes no autorizados permanecen read-only.
- README revisados: 2/2. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Igualdad real de privilegios de ambos owners globales

- La autoridad de global owner 1 y 2 es idéntica. No existe rango subordinado: ambos pueden ejecutar los controles de Base en cualquier socket Matter cuyo proveedor valide el JID y cuyo cliente privado Vault verifique la membresía global del remitente.
- `WhatsAppControlBridge` añade un adaptador global verificado **antes** de la ruta de lectura pública. Sólo el sender externo autenticado con HMAC + `VaultClient::require_global_owner` obtiene permiso delegado para los controles del socket; el resto queda en `dispatch_public_readonly` sin mutación. Los controles de administración y grupo siguen las reglas funcionales existentes, como destino/argumentos y estado del bot.
- `whatsapp-rust` puede firmar comandos administrativos recibidos de otros usuarios en chats/grupos sin dar privilegios automáticamente: sólo Vault puede elevar a uno de los dos owners globales. Ningún comando de texto, alias o rol de grupo supera la comprobación.
- `=/>set role owner --global NUMERO` acepta owner global 1 o 2, con alcance de chat privado para esta modificación. El cliente deja de informar erróneamente que los roles/controles globales están siempre bloqueados. Revocación cruzada en el servicio Vault en rotación de owner 2.
- **Límite real:** sólo controles que existen en Base quedan ejecutables. Telegram/Discord no tienen todavía proveedores operativos ni callback firmado; no se puede proclamar que ya manejen comandos. No se ha desplegado este SHA en instabox-se.
- Pruebas de seguridad deniegan escalamiento por un sender firmado sin membresía Vault. README revisados: 2/2, los cuatro documentos de continuidad actualizados de forma atómica.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Rustfmt: concesión global owner 2

- Se aplicaron automáticamente los cambios exactos de rustfmt detectados en CI al cliente Vault, puente WhatsApp, pruebas de aislamiento y proveedor Rust. Sin nuevas capacidades ni cambios de secretos.
- El comando `=/>set role owner --global NUMERO` se autoriza sólo desde el owner 1 autenticado por Vault en chat privado. Tests y binarios nuevos deben terminar con CI success antes de instalarlos.
- README revisados: 2/2; documentación de continuidad actualizada. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Real Vault =/>set role owner --global

- Ruta global tipada: `=/>set role owner --global NUMERO` en chat privado, con PN auténtico firmado por WhatsApp y validado en el puente; la operación se envía exclusivamente al servicio privado Vault como `owner_grant_secondary`. El owner del socket no hereda autoridad global.
- El servicio Vault restringe la mutación al principal global slot 1 previamente enrolado, asignando sólo slot 2 / WhatsApp. Rechaza autoasignación, dueño de bot, otros remitentes, números no canónicos y grupos. El slot 1 no se puede modificar desde WhatsApp.
- Sin número, devuelve el uso al principal autorizado, sin cambio de roles. La respuesta satisfactoria no muestra el número destino; la prueba procesal sin Vault debe denegarlo. No se incluyen números reales en repositorios.
- **No se ha instalado la versión nueva en instabox-se**. Se requiere compilar Vault y Base desde los SHAs CI aprobados, reemplazar control/proveedor con sesión detenida y registrar primero el PN global 1 de forma privada. No se han implementado otros roles globales ni los adaptadores Telegram/Discord.
- README revisados: 2/2, cuatro documentos de continuidad actualizados; créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Política owner global principal en todas las redes

- Ancla privada: el teléfono público del propietario es el principal global 1 en Vault, sin colocarlo en el repositorio público. Cada bot y plataforma deben comprobar la identidad real del evento y consultar `owner_check` cifrado antes de responder `=/>vault`.
- WhatsApp prueba su JID; Telegram y Discord requieren vincular sus identificadores estables una vez, no conceder privilegios por un número o nombre que alguien escriba. El owner 2 mantiene identidades diferentes, seleccionables por plataforma.
- Se aplican dos ajustes rustfmt detectados por CI para el cliente v2 de plataforma. La compatibilidad de cliente v1 sigue limitada a WhatsApp y Vault se mantiene privado. El servidor no se ha actualizado.
- README revisados: 2/2. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Cliente global de Vault con identidad inmutable por red

- Base-Matter introduce `require_global_owner_for_platform(platform, subject_id)` para el contrato v2 de Vault, con `whatsapp` PN/LID real, `telegram` user.id y `discord` user.id. Username/nickname es solo un alias visual y no se convierte en principal por texto.
- `require_global_owner` existente sigue soportando WhatsApp. Toda autorización usa paquete cifrado autenticado al servicio privado y compara la respuesta de forma estricta. Identidades erróneas o plataformas no autorizadas fallan sin filtrar nombres.
- Nuevo diagnóstico CLI `matter-vault-client owner-check-platform SOCKET CLIENT KEY TENANT BOT SOCKET_ID PLATFORM` lee únicamente `subject_id` por stdin, no imprime números ni IDs. La cuenta pública del operador se introduce al registrar Vault local; no existe número fijo en código.
- Contratos disponibles para Telegram/Discord sin afirmar integraciones en ejecución. En WhatsApp el proveedor usa los JID obtenidos de eventos autenticados. Nunca confiar en un @username escrito en un mensaje.
- Requiere actualizar binarios de Vault y Base tras CI y enlazar cada bot adicional con su propio grant. README revisados: 2/2 y cuatro documentos de continuidad sincronizados. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Verificación host-local de owner global desde stdin

- El cliente público `matter-vault-client owner-check` lee un JID canónico de stdin, verifica por AEAD contra el registro cifrado de Vault-Matter y devuelve solo `global_owner_authorized` o error genérico. No acepta el JID en argv, no imprime números, no consulta registros de roles.
- Usado para E2E con dos identidades sintéticas. En producción las dos identidades reales se enrolan exclusivamente bajo el UID privado `matter-vault`; no introducirlas en Git ni en chat.
- README revisados: 2/2 y cuatro documentos de continuidad actualizados en el mismo commit; créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Protección de replay en comandos Vault globales

- El puente autentica al JID global con el servicio privado, reclama `message_id` durable contra replay, y **solo después** permite `=/>vault test` (escritura sintética). Repite autorización antes de ejecutar para negar si se revocó el principal entre ambos pasos. Replays y terceros no pueden provocar un nuevo write.
- Sin cambios de scopes, roles, secretos, permisos de grupos ni registro público. README revisados 2/2, documentos de continuidad sincronizados en este commit; créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Correcciones de formato Rust de autoridad global

- CI original: Rustfmt falló antes de compilar en las rutas del puente/cliente y una prueba; se aplicaron los diffs exactos del log. La comparación de la prueba de respuesta AEAD usa `is_some_and` para evitar ambigüedad del patrón de slice.
- Ningún cambio de privilegios, números, claves ni roles. Servicio Vault privado y controlador público continúan pendiente de despliegue coordinado y enrolamiento de dos identidades.
- README revisados: 2/2 y documentación de continuidad actualizada en este mismo commit. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Acceso Vault reservado al owner global de dos números

- El servicio privado Vault-Matter implementó `set-global-owners` y `owner_check` bajo autoridad cifrada (commit `e7134ce4d81193f0f34e3c434c81d77c860baa9e`); ningún número aparece en Base público ni en docs. Requiere enrolar **exactamente dos JID verificados**, en el host, antes de habilitar.
- Base incorpora `VaultClient::require_global_owner`: petición AEAD de exacta identidad WhatsApp al servicio privado, sin permitir que el cliente elija su socket; no concede rangos ni otras acciones.
- `WhatsAppControlBridge` dirige sólo `=/>vault [status|test]` a ese cliente en chats privados: permite al owner global desde el chat propio de cualquier bot Matter enrolado, incluso como remitente externo, siempre con HMAC firmado por el proveedor y autorización positiva del Vault. Dueño del socket ≠ owner global. Grupos, terceros, alias y permisos de roles continúan denegados.
- El proveedor whitelist permite mensajes privados entrantes `=/>vault` sin conceder control de socket. La respuesta negativa a un tercero es silencio; ninguna lista de números, secretos o configuraciones se expone. La ejecución de pruebas de escritura sigue limitada al registro sintético del grant del bot/socket.
- Se sustituyó el antiguo handler local de Vault por la ruta de autorización global. Las pruebas de controlador sin Vault ahora exigen `denied` en lugar de aceptar un status falso. Un sender PN y otro LID no se equiparan automáticamente: el operador debe verificar la correspondencia en WhatsApp y enrolar el identificador que realmente llega, sin asumirla.
- Sin implementación ni integración automática de nuevos bots: cada bot deberá enrolar su propio cliente Vault (sin copiar el secreto de alpha) antes de usar el comando. No hay despliegue en instabox-se de esta revisión ni números enrolados.
- README revisados: 2/2; documentación de continuidad AGENTS, DECISION_LOG, WORK_CONTINUITY y DEVELOPMENT_STATUS sincronizada en este commit. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Corrección real de =/>vault en WhatsApp: allowlist del puente

- Diagnóstico confirmado con usuario: `wa-alpha` está autenticada a WhatsApp, nuevo cliente Vault `base-wa-alpha` pasó `vault_roundtrip_verified revision=1`, y el preflight imprime `ALPHA_VAULT=AUTHENTICATED_CONFIGURATION_ONLY`. Sin embargo, `=/>vault` y `=/>vault test` devolvían rechazo genérico.
- **Causa raíz identificada:** `providers/whatsapp-rust/src/lib.rs` ya autorizaba `=/>vault` en el chat propio, y `src/bin/whatsapp_control.rs` lo registraba; sin embargo, **`src/whatsapp_bridge.rs` omitía `=/>vault` de su allowlist de controles privados**, rechazándolo antes de ejecutar el handler o cualquier IPC. El cambio de timeout en el proveedor no podía solucionarlo.
- Solución: admitir exactamente el verbo `=/>vault` en el allowlist de mensajes privados ya autenticados por HMAC y dueño del socket. No admitirlo en grupos, invitados, bots externos ni prefijos alternativos. `=/>vault` y `=/>vault test` siguen en estado de configuración únicamente; roles, registro y comandos privados bloqueados.
- Regresión: `tests/whatsapp_process.rs` verifica **proceso controlador Rust real** y respuesta `applied` + texto sin configurar en ambos comandos aun con bot OFF; elimina toda variable Vault heredada para no utilizar secretos del runner. `tests/whatsapp_bridge.rs` prueba rechazo a grupo y remitente no dueño.
- **Despliegue pendiente:** reconstruir/descargar desde GitHub Actions el controlador `whatsapp-matter-control` del nuevo SHA, validar hash y `BUILD_INFO`; detener único proveedor, sustituir únicamente el controlador y reanudar `wa-alpha` con `--vault-env`. No re-enrolar, no tocar SQLite ni claves. Confirmar respuesta en chat propio tras CI.
- README revisados: 2/2 (`README.md` y `providers/whatsapp-rust/README.md`); AGENTS/DECISION_LOG/WORK_CONTINUITY/DEVELOPMENT_STATUS actualizados conjuntamente en este commit.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Formato CI del diagnóstico Vault

- El CI del commit anterior rechazó únicamente el formato de una línea de la prueba de aislamiento en `providers/whatsapp-rust/src/main.rs`; se aplicó el diff exacto de rustfmt.
- No se alteran claves, políticas, estado de WhatsApp, autoridad Vault ni comportamiento del comando. Se revalida CI y solo se instala un proveedor si el workflow finaliza success.
- README revisados: 2/2; cuatro documentos de continuidad actualizados en el mismo commit. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Diagnóstico de respuesta =/>vault por WhatsApp

- La conexión real Vault del socket wa-alpha se confirmó por el operador: `ALPHA_VAULT_ENROLLED=YES`, `ALPHA_VAULT_IPC=AUTHENTICATED`, `vault_roundtrip_verified revision=1`. El preflight alpha verificó `ALPHA_VAULT=AUTHENTICATED_CONFIGURATION_ONLY`. WhatsApp reconectó y respondió `WhatsApp session connected. Base-Matter owner control available.`.
- Problema reportado: `=/>vault` no devuelve respuesta visible. El proveedor actual registraba solamente `Base-Matter control denied or unreachable` si el IPC fallaba, sin notificar al propietario. También fijaba 4 segundos para cualquier consulta no-IA.
- Corregido `providers/whatsapp-rust/src/main.rs`: sólo para `=/>vault` originado en el chat privado autenticado del propietario, el IPC dispone de 12 segundos; errores reciben un aviso genérico sin secretos ni roles; el envío aceptado se registra sin el contenido. El acceso a grupos y a terceros sigue denegado. Agregada prueba Rust de aislamiento contra comandos falsos, grupos y remitentes externos.
- Estas modificaciones son código en GitHub; **requieren descargar/verificar el binario nuevo del proveedor desde CI y reemplazarlo sólo con la sesión detenida**. No se ha ejecutado esta sustitución ni demostrado todavía la respuesta por WhatsApp. No tocar credenciales ni sesión SQLite.
- README revisados: 2/2 (`README.md` y `providers/whatsapp-rust/README.md`), cuatro documentos de continuidad actualizados atómicamente. Bot-Matter permanece bloqueado, roles y Github sync independientes.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Etapa Vault: continuidad y revisión documental del cliente wa-alpha

- WhatsApp real: el propietario confirmó que `=/>status`, `=/>menu` y `=/>help` funcionan en la sesión alpha conectada.
- Base-Matter alpha modifica únicamente el lanzador existente `scripts/whatsapp-alpha-existing.sh` con opt-in `--vault-env /etc/base-matter/vault-alpha.env`. Sin opt-in, Vault permanece aislado. Con opt-in: `base-wa-alpha`, llave `alpha-client.key`, `wa-alpha`, ruta IPC y ping autenticado. Nunca mezclar con `base-wa-test-1` o con la base de sesión original.
- Vault-Matter privado incorpora instalador para el nuevo grant y sincronización de tar de ciphertext a GitHub usando cuenta separada, token externo, validación y timer; el repositorio público no recibe código ni token Vault.
- EVIDENCIA LIMITADA: los commits están en GitHub. No consta todavía ejecución del nuevo enrolamiento en instabox-se, `=/>vault` real en alpha ni subida remota de una copia cifrada. La autoridad de roles, registros y ejecución privada continúa bloqueada.
- Pruebas requeridas antes de declarar conexión: CI Linux/Windows y proveedor, enrolamiento real sólo de cliente alpha, self-test y preflight (sin segundo proveedor), reconexión controlada y WhatsApp self-chat `=/>vault`.
- README revisados: 2/2 (`README.md` y `providers/whatsapp-rust/README.md`); cambios y decisiones sincronizados en AGENTS/DECISION_LOG/WORK_CONTINUITY/DEVELOPMENT_STATUS en el mismo commit.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Sesión WhatsApp alpha recuperada y autenticada en host real

- Evidencia enviada desde `instabox-se`: en `/home/Sakura/Base-Matter`, el script `scripts/whatsapp-alpha-existing.sh --check` pasó `Base-Matter control settings checked`, `whatsapp-rust pairing settings checked`, `ALPHA_PREFLIGHT=OK`, `ALPHA_VAULT=ISOLATED_NOT_ENROLLED`, `ALPHA_PAIRING=DISABLED`.
- Luego se ejecutó una única prueba `--resume-only` con los binarios previamente verificados `e073b1b5ebc3ad89cec5a9ce8a71647312aebb64`: `ALPHA_SNAPSHOT=VERIFIED_PRIVATE`, `ALPHA_CONTROLLER=STARTED`, `ALPHA_PROVIDER=RESUME_ONLY`, `PN=true`, `LID=true`, **`WhatsApp session connected. Base-Matter owner control available.`**. El proceso quedó en primer plano. No se solicitó código ni QR.
- Esta señal **acredita autenticación y conexión real de la sesión aislada alpha**, distinta del `wa-test-1` original que recibió `AUTH_REJECTED_LOGGED_OUT`. No acredita aún respuesta del bot a un comando ni integración Vault para `wa-alpha`. Conservar originales, respaldos privados, puertos y archivos separados. No copiar archivos entre sesiones.
- Próxima prueba SIN detener el proveedor en curso: en WhatsApp > chat propio, enviar `=/>status`, luego `=/>menu` o `=/>help`; comprobar respuesta real y evento `Base-Matter owner control applied`. Si no responde, inspeccionar sólo errores genéricos del terminal sin publicar JID, teléfono o tokens. No probar `=/>vault test` en alpha hasta enrolar su cliente IPC independiente y verificar concesiones.
- Vault-Matter en `wa-test-1` continúa con última evidencia `vault_roundtrip_verified revision=2`; **no atribuir** esa concesión al socket `wa-alpha`. Ramas beta/stable intactas y Bot-Matter bloqueado hasta criterios de preparación.
- README revisados: 2/2.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Recuperación segura y separada de dispositivo alpha

- Base alpha y socket original tienen PN/LID de la misma cuenta, pero las claves y JID de dispositivo difieren. La sesión original fue rechazada; alpha todavía no está probada en el servidor.
- Creado `scripts/whatsapp-alpha-existing.sh` exclusivamente para el estado `branches/alpha/whatsapp-test`, con `--check` sin cambios y `--resume-only` que exige bases/configuración existentes, puerto y ruta aislados, crea snapshot SQLite privado, desactiva entorno Vault heredado y jamás solicita códigos ni Cargo ni `--init`.
- CI prueba sintaxis y escenarios con binarios simulados: check seguro, snapshot, rutas equivocadas, socket original y configuración ausente. WhatsApp móvil no se ha reconectado.
- README revisados: 2/2. Actualizada continuidad y decisión. Bot-Matter sigue bloqueado; ramas beta/stable no modificadas.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Comparación de cuenta sin sufijos de dispositivo

- Las bases WhatsApp original `whatsapp-test` y aislada `branches/alpha/whatsapp-test` tienen `device.id=1`, pero PN, LID, claves y account difieren. No tratarlas como respaldos intercambiables. Un JID distinto puede deberse a `:n` y representar otro dispositivo de la MISMA cuenta.
- Comparador offline amplía solo el informe con `SAME_PN_ACCOUNT` y `SAME_LID_ACCOUNT` calculados en SQLite (no salen JIDs/secretos), con `unknown` en formato no reconocido. Cero emparejamientos, cero cambios a sesiones, cero cambios Vault; actualizar ambos README y continuidad.
- Próximo paso: ejecutar comparador tras `git pull`, verificar si ambas identidades pertenecen a una misma cuenta y a partir de allí diseñar recuperación aislada; Bot-Matter bloqueado.
- README revisados: 2/2.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Dos bases WhatsApp distintas: comparar sin mover credenciales

- Inventario operador: `whatsapp-test/session.db` original de 475136 bytes y `branches/alpha/whatsapp-test/session.db` de 630784 bytes. La segunda no es una copia verificada ni evidencia de conexión.
- Herramienta Python de lectura `scripts/whatsapp-stores-compare.py` con pruebas CI compara en SQLite los dos registros `device.id=1` sólo por flags de PN/LID/claves/cuenta. Nunca imprime secretos ni copia archivos.
- No mezclar rutas, sesiones, proveedores ni claves. La anterior rechazó autenticación; el teléfono todavía señalaba dispositivo vinculado. La siguiente acción depende de la comparación. Vault sigue OK; Bot-Matter bloqueado; beta/stable intactos.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 UTC — Confirmación: un solo dispositivo SQLite, id predeterminado 1

- Auditoría offline de `/home/Sakura/.local/share/base-matter/whatsapp-test/session.db` en `instabox-se`: `DEVICE_TOTAL=1`; `DEFAULT_DEVICE_EXISTS=True`; `OTHER_DEVICES=0`; `DEFAULT_DEVICE_HAS_IDENTITY=True`. Comprobación SQL de sólo lectura, sin volcar PN, LID, claves ni JID.
- Fuente de implementación `whatsapp-rust 0.7.0` en `storages/sqlite-storage/src/sqlite_store.rs`: `SqliteStore::new(database_url)` llama `Self::build(database_url, 1, ...)`. Por tanto **la selección del registro equivocado queda descartada**: `device.id=1` es el único registro y tiene identidad.
- Pruebas anteriores: PN/LID/account/keys presentes, SQLite íntegra, DNS/TLS a web.whatsapp.com correctos; `--resume-only` emite `WA_LOGOUT_CLASS=AUTH_REJECTED_LOGGED_OUT` antes de autenticación, sin solicitar código. En el teléfono del operador la sesión todavía aparecía en Dispositivos vinculados, con última actividad aproximadamente a las 20:00 hora México (19/11). La UI móvil puede no reflejar de inmediato el estado servidor; ni presencia ni fecha confirman vigencia de credenciales.
- Pendiente: revisar copias anteriores auténticas del almacén *sin abrir ni imprimir secretos* y verificar el estado de vinculación; no repetir conexiones de prueba ni forzar nuevo pairing. Si se decide relink, sólo mediante base de sesión nueva y aislada, respaldo privado autenticado y autorización del propietario. La sesión anterior no debe eliminarse, sobreescribirse ni subirse al repositorio público.
- Vault independiente sigue validado con `vault_roundtrip_verified revision=2`; no confundir rechazo de WhatsApp con fallo de Vault. Bot-Matter continúa bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Cierre de sesión WhatsApp observado con identidad y TLS válidos

- Evidencia directa: inspección de SQLite `device` con `DEVICE_HAS_PN=yes`, `DEVICE_HAS_LID=yes`, `DEVICE_HAS_ACCOUNT=yes`, `DEVICE_HAS_KEY_MATERIAL=yes`, integridad OK. `DNS_WHATSAPP=OK`, `TLS_WHATSAPP=OK` (`TLSv1.3`). El CI `0259d9f0c722dc01abfa3aa6529cdafde5eb196d` publicó binarios firmados por SHA-256 y operador los verificó.
- Lanzador existente sobre `wa-test-1`: `--resume-only` imprimió `PN=true, LID=true`, disparó **evento `LoggedOut` antes de `on_connected`** y terminó. El servicio Vault independiente permanece verificado con `vault_roundtrip_verified revision=2`.
- Revisado el código upstream `whatsapp-rust 0.7.0`: `LoggedOut` puede ser rechazo `<failure>` durante conexión (401/403/406) o stream error de remoción/conflicto (401/516). Se añade categoría anónima `WA_LOGOUT_CLASS` usando sólo `on_connect` y `ConnectFailureReason` (sin raw node, JIDs, claves, mensajes de servidor o número telefónico); pruebas Rust correspondientes.
- **No inferir** que se deba volver a emparejar sólo por `LoggedOut`; primero verificar en el teléfono la sesión de Dispositivos vinculados y si el estado de la cuenta está restringido. No repetir prueba de conexión hasta actualizar a nuevo binario de CI y validar hashes, sin tocar `session.db*`, llaves, Vault ni ramas beta/stable.
- README y continuidad auditados; Bot-Matter continúa bloqueado. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Señales de sesión local existentes; diagnóstico anónimo de transporte

- Inspector de la base real confirmó `device` singular con PN, LID, cuenta y material de claves presentes, SQLite OK; no implica autenticación WhatsApp activa. Eliminada impresión duplicada del estado offline.
- El proveedor Rust, únicamente en nuevo commit alpha después de CI, informa eventos de desconexión, rechazo de conexión, versión obsoleta, restricción temporal y error del stream sin datos personales ni claves. `--resume-only` también distingue identidad almacenada en memoria de conexión efectiva.
- La siguiente decisión depende de comprobar WhatsApp → Dispositivos vinculados y conectividad del host; no se destruye ni cambia ninguna sesión, ni se generan nuevos códigos. Vault continúa con roundtrip autenticado `revision=2`.
- Actualizados README y continuidad. Bot-Matter bloqueado; beta/stable intactos.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Diagnóstico de sesión WhatsApp (tabla singular) y salida del proveedor

- En `instabox-se`, `--resume-only` con binarios SHA verificados `6fc993b7f335f62dae8c84d91109e929a502a332` terminó inmediatamente sin `on_connected`; no se solicita más código ni se declara WA funcional.
- Inspector SQLite corregido tras revisar implementación upstream `whatsapp-rust 0.7.0`: la identidad vinculada es la tabla `device`, **no inferir autenticación de `devices` plural**. Sólo banderas de presencia de `pn/lid/account` y claves, sin extraer sus valores; prueba Python offline añadida.
- El proveedor informa sin secretos cuándo `--resume-only` termina antes o después de autenticación, y evento de desvinculación cuando es emitido. No se añaden reintentos, QR ni cambios de sesión. Manual `docs/WHATSAPP_PAIRING_DIAGNOSTIC.md`.
- Vault (proceso aislado) sigue verificado con `vault_roundtrip_verified revision=2`; no modificar repos privados/llaves. README revisados y continuidad actualizada. CI de esta revisión pendiente; beta/stable y Bot-Matter bloqueados.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Corrección rustfmt del modo resume-only

- CI del proveedor `4af6078bba1ea2dbd4885bcc4345790b54ecd78b` detectó exclusivamente el `println!` de entrada de `--resume-only` que requería múltiples líneas. Corregido de forma idéntica a rustfmt; no se modifica lógica, sesión, concesiones Vault, permisos o código de vinculación.
- Reiniciar CI Linux/Windows/proveedor para nuevo SHA y distribuir artefactos Linux emparejados sólo tras éxito completo.
- README inspeccionados y continuidad actualizada. Bot-Matter bloqueado. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Recuperación segura WhatsApp y self-test Vault en host

- Instalación en instabox-se ya acreditada con IPC autenticado y ahora `vault_roundtrip_verified revision=2` del cliente real Base; servicio `matter-vault.service` `active`.
- Inspector sin secretos confirmó sesión `session.db` SQLite `OK`, 20 tablas y fila de dispositivo existente, no autenticación vigente de WhatsApp.
- Agregado modo `--resume-only` al proveedor Rust; el lanzador seguro del socket legado `wa-test-1` lo usa por defecto, omitiendo peticiones de código y refresh. Rechazo si condiciones no se cumplen, nunca borrar sesión, jamás generar QR. Prueba Rust para presencia/ausencia de opción de emparejamiento.
- CI nuevo de alpha aún por confirmar; binarios anteriores del proveedor no conocen `--resume-only`. Sólo distribuir los artefactos del SHA que pase Linux/Windows/proveedor. A continuación comprobar `WhatsApp session connected` y mandar `=/>vault` en self-chat. No hay prueba WhatsApp positiva aún.
- Guía: `docs/WHATSAPP_PAIRING_DIAGNOSTIC.md`. README raíz y proveedor inspeccionados; beta/stable y Bot-Matter intactos. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Formato CI de prueba Vault y comando diagnóstico correcto

- `whatsapp-rust-ci` del commit `83668775362311ea3c87a6fcc933b910083556b3` falló únicamente en `cargo fmt --check` por una expresión `assert_eq!` larga en prueba nueva. Aplicado exactamente formato indicado por rustfmt. Los tests de lógica todavía deben pasar en nuevo CI.
- Guía de pairing corregida para ejecutar únicamente el inspector Python de sesión, sin proponer `--help` al lanzador productivo. No se ha relanzado ni regenerado WhatsApp en Debian, y el servicio Vault sigue operativo.
- Ambos README revisados, documentación de continuidad y seguridad preservada. CI nuevo pendiente, beta/stable y Bot-Matter intactos.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — WhatsApp pairing no completado; defecto del whitelist Vault corregido

- El operador ejecutó `scripts/whatsapp-vault-existing.sh` como Sakura con grupo IPC heredado. Ambas comprobaciones `--check` pasaron y el controlador inició, pero el proveedor informó solicitud de pairing fallida. No se logró reconexión WhatsApp ni prueba `=/>vault` en teléfono; NO confundir con fallo IPC Vault ya autenticado.
- Corregida lista de comandos firmables: `=/>vault` ahora sólo en self-chat propietario, nunca grupos/terceros. Prueba de firma/rechazo añadida al proveedor.
- Diagnóstico de rechazo de pairing categorizado sin imprimir secretos; inventario Python de SQLite/archivos de sesión **sólo lectura** y sin filas. No se borran ni regeneran credenciales, no se automatiza re-pairing.
- Archivo operativo `docs/WHATSAPP_PAIRING_DIAGNOSTIC.md`. CI de nuevo commit pendiente, no desplegar proveedor hasta pasar CI y recompilar artefacto en GitHub, no en Debian de 16 GB.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Corrección estricta de final de YAML CI

- El job Linux en el commit anterior llegó a `Verify whitespace` y falló al detectar un salto vacío al final del archivo workflow (verificación `git diff --check HEAD^ HEAD`). Se eliminó el salto excedente, sin cambiar código Rust ni sesiones.
- Se solicita un nuevo CI Linux, Windows y proveedor sobre el mismo SHA de empaquetado; ningún artefacto se declara aprobado antes de `success`.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Ajuste de workflow del empaquetado Base CI

- Se detectó que los pasos de empaquetado del controlador quedaron bajo el job Windows por ubicación YAML. Trasladados al job Linux `rust` **después de las pruebas** y añadida comprobación de sintaxis del lanzador sin Cargo. El proveedor revalida su mismo SHA con cambio mínimo en workflow.
- Sin cambio en runtime, sesión ni instalación Vault del servidor. No se consideran listos los nuevos binarios hasta que CI Linux/Windows y proveedor finalicen correctamente y ambos artefactos compartan SHA.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Binarios CI y prueba WhatsApp/Vault en servidor sin espacio

- Evidencia del usuario: `instabox-se` recuperó 4.7 GB, descargó y verificó SHA-256 de Vault/cliente del CI `44664373277` y ejecutó instalador root con resultado `CONEXIÓN REAL CONFIRMADA`, para `wa-test-1`. **La autenticación IPC en servidor está confirmada; `self-test` y WhatsApp reales pendientes.**
- Desacople identificado: lanzador Cargo por rama usa ruta `branches/alpha` y socket distinto; se prohíbe mezclar con el estado original `whatsapp-test` asociado a Vault.
- CI de Base alpha emite dos artefactos x86_64 por SHA del commit, después de pruebas: controlador y proveedor Rust sin claves/sesiones; lanzador opcional `scripts/whatsapp-vault-existing.sh` sólo reutiliza el entorno original, exige identidad Vault igual al socket y nunca compila, registra otro socket ni inicia segundo proveedor activo.
- Ver `docs/WHATSAPP_VAULT_PREBUILT.md`. CI de este commit aún por verificar; beta/stable intactos, Bot-Matter bloqueado. Autoridad Vault de roles y backup remoto siguen pendientes.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Estado cruzado de integración con Vault-Matter privado

- Cliente IPC real Base: `matter-vault-client` y `=/>vault` / `=/>vault test`, restringidos a identidad/socket autorizado. La conexión Vault/Base bajo UIDs Unix separados ya está verificada **en GitHub Actions**, pero **NO instalada en el Debian del propietario** ni probada mediante mensaje real de WhatsApp.
- Vault-Matter rama `matter`, checkpoint `2e87ec545073c7772c2ebc4f3c5a28d75ddbad3e`: añadió exportación/restauración local de archivos cifrados, `verify-root` y nuevas pruebas de recuperación. Resultado final de CI de ese SHA pendiente en el momento del checkpoint.
- La exportación local no es un backup remoto: está pendiente la réplica de ciphertext a GitHub privado y la custodia independiente de llave maestra. Base-Matter público **no recibe** la maestra ni GitHub token, y Bot-Matter permanece bloqueado.
- Paso operativo: instalar el servicio y el cliente en Debian bajo cuentas separadas usando la guía privada Vault; comprobar `ping` y `self-test` desde UID Base, integrar las seis variables de entorno en el controlador WhatsApp y entonces probar `=/>vault` en chat propio. Si faltan privilegios o concesiones: denegado, sin fallback.
- No se ha habilitado control privado, roles globales ni autoridad por actor WhatsApp. No promocionar `beta` / `stable` hasta las pruebas y políticas pendientes.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Conexión Base/Vault verificada en CI y paso al host pendiente

- Base SHA `98231581533bf641e46ce7f5d53a3978b95eb7ab`: Rust Windows/Linux CI `44659925103` verde, whatsapp-rust `44659925089` verde. Vault E2E `44660096430` verde para socket Unix real, cifrado, get/put CAS, revocación y arranque systemd con UIDs distintos; Base no pudo leer master.
- `=/>vault` informa estado; `=/>vault test` prueba get/put/get cifrados de un marcador reservado, propietario self-chat solamente. Necesita entorno y grupo IPC en el proceso real.
- NO está instalado en Debian del dueño; no hay copia segura de ciphertext runtime a GitHub; no habilitar roles, control global ni comandos privados. Guía pública docs/VAULT_CONNECTION_READY.md, referencia operativa en documentación privada Vault.
- Actualizados ambos README, AGENTS, DECISION_LOG, WORK_CONTINUITY y DEVELOPMENT_STATUS sin cambiar Rust; beta/stable intactos, Bot-Matter bloqueado. CI de este commit documental pendiente.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Vault test: prueba de escritura/lectura cifrada desde Base y WhatsApp

- `VaultClient::verify_roundtrip` verifica `get -> put(CAS) -> get` sobre un registro sintético reservado, con revisión comprobada y sin mostrar el payload. No permite roles ni otra categoría de registros; se conserva la concesión tenant/bot/socket vigente y validación del gateway.
- Nuevo operador CLI `matter-vault-client self-test` y comando nativo `=/>vault test` exclusivamente en chat privado del propietario autenticado. `=/>vault` sin argumentos continúa devolviendo estado. No modificar ni probar con secretos reales.
- Requiere servicio y secretos de cliente ya provisionados bajo UIDs diferentes. Pruebas de CI Rust y E2E privado del nuevo SHA pendientes. README revisados: 2/2; AGENTS/decisiones/continuidad actualizados, alpha únicamente, beta/stable intactos y Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Corrección rustfmt del selector del proveedor WhatsApp

- CI del proveedor para SHA 15baba4be23d4bb316eedbc70a3ae2c0d89c5ff3 detectó una sola diferencia rustfmt en tests/ui.rs; se aplicó exactamente, sin modificar ni categorías ni lógica de paginación, seguridad o transporte. El test offline ahora puede ejecutarse tras fmt.
- README revisados: raíz y proveedor. AGENTS, DECISION_LOG, WORK_CONTINUITY y DEVELOPMENT_STATUS actualizados. Nuevo CI pendiente. Solo alpha; beta/stable intactos y Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Selector WhatsApp de 10 categorías (alpha)

- CI anterior del proveedor fallaba en dos pruebas: el catálogo ahora tiene 10 categorías por activar Sistema (=/>vault), pero NativeFlow mantiene un límite de 9 categorías por página. No se cambia el límite ni se eliminan categorías; las pruebas ahora comprueban la primera página con navegación y la segunda con Utils + regreso.
- Seguridad: selector continúa ofreciendo exclusivamente IDs de consulta permitidos. No se habilitan comandos de escritura por botones. Cargo del núcleo Base ya había pasado; nuevo CI provider pendiente.
- README revisados: 2/2; AGENTS, DECISION_LOG, WORK_CONTINUITY y DEVELOPMENT_STATUS actualizados. alpha únicamente; beta/stable intactos; Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Corrección final rustfmt en catálogo y prueba Vault

- Se aplicaron las dos diferencias exactas del CI Linux (array de categorías y última coma en la llamada de prueba de status). No se modificó el contrato ni la autorización del comando =/>vault.
- README revisados: 2/2; estado, decisiones, continuidad y AGENTS actualizados. Este commit requiere nuevo CI Linux y Windows; beta/stable intactos, Bot-Matter bloqueado. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Test del comando Vault y categoría Sistema

- Corregidos rustfmt en la prueba owner-only de Vault y catálogo/menu raíz de 10 categorías, incluyendo Sistema ahora no vacía. El manifiesto templates/menu-visible-categories.json refleja la misma lista real. No se amplían permisos Vault.
- README revisados: 2/2; AGENTS, DECISION_LOG, WORK_CONTINUITY y DEVELOPMENT_STATUS cambiados en el mismo commit. CI nuevo pendiente; beta/stable intactos. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Comando de estado Vault protegido en WhatsApp-Rust

- El controlador =/> integra exclusivamente lectura de salud del canal Vault mediante =/>vault y =/>vault status, en el self-chat verificado y por actor dueño del socket. Opera incluso con bot apagado; NO concede roles, configura secretos ni ejecuta comandos privados.
- La configuración del cliente toma seis variables de entorno con rutas/identificadores, NUNCA la llave maestra ni contenido de la clave. Configuración incompleta bloquea startup. Errores y salida son mínimos, sin identidad ni rutas.
- Enlaces relativos README del proveedor corregidos. README revisados: 2/2. Cambios sólo alpha, con tests y CI por verificar; Bot-Matter sigue bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Ajuste exacto de formato rustfmt en CLI Vault

- El CI Rust de Base detectó un único orden de imports en src/bin/vault_client.rs; se aplicó corrección exacta sin cambios de funcionalidad. Windows ya compila el cliente limitado; CI nuevo por verificar.
- README revisados: 2/2. Decisiones y continuidad actualizadas, sin tocar beta/stable. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Base Vault IPC: corrección de Windows y formato

- Se corrigió orden de imports rustfmt y se separó secure_write por plataforma para no introducir codigo inaccesible ni warnings en Windows. Windows deniega el transporte IPC Unix y la escritura de salida protegida; Linux mantiene la implementación real.
- README revisados: 2/2. CI del SHA nuevo pendiente; actualizados decisiones y continuidad, sin tocar beta/stable ni Bot-Matter. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Corrección de formato y compilación Windows del cliente IPC

- Modulo vault_ipc y CLI ajustados a rustfmt; imports y estructuras Unix protegidas con cfg(unix), manteniendo denegación del transporte no disponible en Windows. No se alteró la identidad, cifrado o la llave maestra.
- README revisados: 2/2. Registros DECISION_LOG, WORK_CONTINUITY, DEVELOPMENT_STATUS y AGENTS actualizados. CI nuevo pendiente; beta/stable intactos.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Cliente IPC cifrado de Vault para Base alpha

- Nuevo modulo Rust vault_ipc y CLI matter-vault-client con canal Unix AEAD, identificador de cliente y secreto de transporte independiente de la llave maestra; ping/get/put restringidos a configuration, bot y socket vinculados al grant Vault.
- Las cargas se leen/escriben en archivos y nunca via argumentos de CLI. Base publico no conoce repos privados ni importa crypto store. Falta integrar autorizacion de actor WhatsApp y despliegue bajo UIDs separados.
- Cambios en codigo y docs/VAULT_IPC_CLIENT.md; README revisados: 2/2. Se actualizaron AGENTS, DECISION_LOG, WORK_CONTINUITY y DEVELOPMENT_STATUS. CI de esta revision pendiente; beta y stable intactas; Bot-Matter bloqueado.
- Creditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Política obligatoria de cifrado para capacidades protegidas

- Toda información operativa que atraviese una capacidad protegida se clasifica como credencial: cifrado autenticado en tránsito/almacenamiento, claves externas y segregación por bot/socket/recurso. Sin cifrado o autoridad real se **deniega**, nunca se permite fallback en texto claro.
- Base-Matter público expone solo contratos, identificadores y solicitudes; **no conoce rutas ni nombres de repositorios privados**, no descarga implementación interna y no debe poseer claves para descifrar material privilegiado. El host confiable autoriza y transporta ciphertext opaco.
- Regla vinculante: `docs/PROTECTED_CAPABILITY_ENCRYPTION.md`. Contrato existente aún no implementa canal operativo cifrado. HMAC aislado no proporciona confidencialidad. Los datos operativos y auditorías deben seguir cifrados; el descifrado temporal solo ocurre en el proceso autorizado.
- Corrección documental, no modifica runtime/WhatsApp ni demuestra cumplimiento criptográfico. No promover `alpha` hasta pruebas Linux/Windows, seguridad y proveedor. Bot-Matter sigue bloqueado. Ambos README y registros de continuidad revisados.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Política obligatoria Vault-D019: protección equivalente a credenciales

- **Regla permanente del ecosistema:** TODOS los activos administrados por Vault — roles, permisos, decisiones, credenciales, políticas, metadatos, evidencia, auditoría, caché, respaldos y estados — deben cifrarse/autenticarse en reposo y transmitirse por canales cifrados y autenticados. Descifrado temporal sólo en memoria confiable; claves fuera de Git, nunca textos sensibles en logs, respuestas públicas ni archivos temporales.
- Base pública: sin rutas, URLs ni dependencias a repositorios privados. El selector provider/language/system y los puentes emiten solicitudes sin credenciales; todo dato o decisión privada del servicio Vault deberá viajar por un canal autenticado y cifrado, y jamás se persistirá en claro. No cambiar el código de runtime hasta existir gateway verificado. Al recibir un resultado mínimo autorizado, no volcarlo a logs, tests ni mensajes. `alpha` solamente; beta/stable intactos.
- La norma principal se mantiene en la política privada VAULT-D019. Esta actualización es documental y no acredita gateway Vault, ejecución privada o migración de sesiones. No conceder permisos sin verificación y no afirmar pruebas funcionales.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Selector público neutral para crear bots (alpha)

- Se añadió `src/bot_setup.rs`: contrato de selección `provider/language/system` con defaults virtuales `default/rust/host`. El núcleo NO lleva URLs, rutas ni dependencias hacia implementaciones privadas; proveedor real y permisos los resuelve un catálogo autenticado del host. Sin catálogo se deniega.
- Se añadió `matter-bot-init` como CLI **offline de solo descriptor JSON**; no genera runtime, no concede credenciales y no importa fuentes privadas. Guía: `docs/BOT_SETUP.md`.
- WhatsApp-Matter ya tiene extracción separada del proveedor Rust; no se realizó corte de sesiones ni borrado del proveedor que hoy utiliza el lanzador de Base. El gateway privado no existe aún. Bot-Matter continúa bloqueado.
- Revisar `cargo fmt --all -- --check`, `cargo check --all-targets --locked`, `cargo test --all-targets --locked`, CI Linux/Windows y proveedor tras este commit. README raíz y del proveedor actualizados, sin modificación beta/stable.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Corrección del menú estático por ámbito

- Base previa c8554033bb0594886443a90c14dd0811576021c8. La plantilla `templates/matterssmith-menu.txt` mostraba el socket como `status --global`, heredado de una convención retirada. Se sustituyó por `status --mod`, se añadió consulta de predeterminado general `status --global` y se renombró el encendido por defecto del socket como `bot --mod on/off`.
- El cambio es de presentación; no modifica estado, privilegios ni políticas. `tests/control_style.rs` comprueba el nuevo encabezado; el contrato `help` mantiene los ámbitos equivalentes. Estado sin argumentos de los comandos implementados continúa siendo consulta, no guía.
- README revisados: 2/2. CI de este SHA pendiente; no desplegar en WhatsApp hasta completar Linux, Windows y proveedor. Beta/stable no se modifican. Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Ajuste de Rustfmt en pruebas públicas Anti

- Base previa 01822a6d32e43839c809ba0580b2b9564d931087. Se aplicaron las dos diferencias exactas de Rustfmt en `tests/whatsapp_public_readonly.rs` para las llamadas de propietario y comprobaciones de rechazo; sin cambios de lógica, autoría ni autorización.
- El controlador mantiene el estado sin argumentos para comandos realmente implementados, con ámbitos `--mod` de socket y `--global` de predeterminado general en los módulos que los soportan. Consultas públicas Anti no ejecutan acciones.
- README revisados: 2/2. CI del SHA nuevo pendiente y validación WhatsApp real pendiente. Beta/stable intactos. Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Verificación de consultas públicas Anti y estado sin argumentos

- SHA anterior f3c308ab3683693b80b05cdae1d569bcc04a2112. Las suites Linux/Windows habían superado el transporte y la mayor parte de los comandos, pero la nueva prueba de estado sin argumentos esperaba un encabezado anterior. Se ajustó la aserción a «Antis del grupo» conservando una respuesta real y distinguible del chat privado.
- Se incorporó una prueba de WhatsApp que habilita un grupo firmado y verifica que un usuario normal puede consultar `anti`, `anti link`, `anti --mod link` y `anti --global link` sin concesión de permisos, y que `anti link on` / `set anti link on` son denegados.
- Persistencia de políticas: todos OFF de fábrica, `--mod` socket, `--global` solo general sin Vault configurado. Ninguna consulta crea escrituras o acciones de moderación.
- README revisados: 2/2; CI del nuevo SHA pendiente, beta/stable sin cambios, Bot-Matter bloqueado. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Migración de pruebas HMAC/IPC al ámbito del socket

- Base previa baa59b01be636593b6742f78e66ddfcb53804db6. Se ajustaron las pruebas de `tests/control_ipc.rs` y `tests/whatsapp_bridge.rs` que enviaban el alias heredado `bot --global on` y consultaban `status --global` como si pertenecieran al socket. Las nuevas comprobaciones usan `bot --mod on` y `status --mod`, sin alterar el transporte HMAC, el antirreplay ni las firmas.
- La opción `--global` queda reservada al valor predeterminado general; en modo vivo no concede escritura global y requiere backend autenticado para guardar cambios. `--mod` es el socket propio, nunca otro.
- README revisados: 2/2. La integración de un resolvedor seguro de usuarios por respuesta/mención sigue pendiente. CI del nuevo SHA pendiente; beta/stable intactos, Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Corrección compatibilidad de estado Anti y proveedor

- Base previa: 83e0a17b47c80c595a43b0d0168d0df74cdf6675. CI Linux y Windows encontraron una sola aserción heredada en `tests/anti.rs`: la etiqueta del grupo dejó de decir «Antis del grupo». Se restaura esa etiqueta; «chat privado» distingue DM, y «socket actual» sustituye al antiguo término ambiguo «socket global».
- Se aplicó la diferencia exacta de Rustfmt del proveedor `whatsapp-rust/src/lib.rs`, sin cambiar autorización ni contenidos.
- Consultas sin argumentos devuelven estado cuando existe: bot, anti, status, set y ajustes implementados. No crear respuestas ficticias de módulos pendientes o de usuarios sin metadatos verificados.
- README revisados: 2/2. CI del nuevo SHA pendiente; no integrar en beta/stable. Bot-Matter sigue bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — CI de consultas sin argumentos, formato y controles globales

- SHA anterior: 5b229870eea827864f48961fefb2278beca8bf33. Se aplicaron 21 ajustes exigidos por Rustfmt (despachador, consultas de estado y pruebas); `bot --mod on` ahora usa el tipo Rust correcto del token `action`.
- En modo WhatsApp unificado quedan bloqueadas **ambas** formas de escritura global: `bot --global on` y `bot on --global`; `bot --mod on` es la escritura local del socket autenticado. La prueba cubre el bypass de orden de argumentos.
- Se mantiene el comportamiento: comandos con estado, sin argumentos, muestran su valor actual; `help` explica la sintaxis sin ejecutar comandos. Anti permite consulta read-only en chat activo y registra solo configuración, no moderación.
- README revisados: 2/2. CI del nuevo SHA aún pendiente; no promover a beta/stable ni confirmar WhatsApp real. Bot-Matter sigue bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Consultas de estado sin argumentos y ayuda por ámbito

- La consulta de estado es el comportamiento predeterminado de las familias **con estado**: `=/>bot`, `=/>anti`, `=/>status`, `=/>set`, `=/>set style`, `=/>set lang`, `=/>set anti` y `=/>set anti link`. En un chat privado verificado, Anti consulta la configuración de ese chat. En un chat de grupo, la de ese grupo. Comandos meramente informativos o pendientes sin estado real mantienen sintaxis de ayuda o indisponibilidad.
- `--mod` identifica **este socket**; `--global`, **el valor predeterminado general**. No mezclar ambos: el socket no es global. Para Anti, `--global` muestra el fallback inmutable OFF/warn y no un registro Vault. Para status/bot en modo vivo, `--global` solo informa la disponibilidad del fallback general y jamás el socket. `bot --global on/off` se rechaza en WhatsApp vivo; `bot --mod on/off` configura el socket.
- `=/>set` y sus variantes sin valor consultan preferencias reales de menú; `=/>set --global` reporta los valores de fábrica, no una preferencia remota configurable. Los comandos `set anti` con argumentos son escrituras verificadas; sin argumentos solo consultan. `set anti link` sin operación muestra estado y lista de dominios actual.
- En chats habilitados, usuarios normales pueden leer estado Anti mediante un canal dedicado **sin escritura**, para `anti`, `anti <tipo>`, `anti --mod`, `anti --global`; el proveedor solo firma consultas limitadas, nunca `anti link on/off`. Los detalles privados del socket y los rangos globales no se divulgan.
- Los nombres/respuestas/menciones/números para consultar usuarios siguen **pendientes de resolvedor autenticado**; este cambio no simula estados de terceros.
- Los cambios de catálogo describen `--mod` cuando la familia lo soporta y eliminan la documentación errónea «--global = socket». No anunciar soporte universal de ámbitos en comandos pendientes.
- Pruebas añadidas/ajustadas: estado sin argumentos, ámbitos, no escalada, consultas públicas, contrato de help y compatibilidad. CI Linux/Windows y proveedor siguen siendo requisito antes de activar en servidor.
- README revisados: 2/2; sin promocionar a beta/stable, Bot-Matter bloqueado. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Control de ámbito estandarizado y seguridad de consulta por usuario

- Base anterior: `c584b3d55569`. Se aplicaron ajustes de Rustfmt a `src/control_scope.rs` y orden de `src/lib.rs` sin cambiar la autorización.
- El contrato `docs/CONTROL_SCOPE_CONTRACT.md` define: chat actual, `--mod` socket actual, `--global` predeterminado general; dos banderas son inválidas. Un ámbito no otorga permisos.
- El resolver de usuario por respuesta, mención, número o `user` es un pendiente de integración con metadatos verificados, no una función pública activa. No conceder por texto un rol ni divulgar identidad privada.
- Continuidad actualizada y READMEs revisados: 2/2. Verificación CI de este SHA pendiente. `beta`/`stable` intactos. Bot-Matter bloqueado.

## 2026-11-19 — Ámbitos y estado de usuario (contrato de migración)

`--mod` identifica el socket autenticado; `--global` los valores predeterminados generales (no un alias del socket). Consulta sin bandera = chat actual. Ver `docs/CONTROL_SCOPE_CONTRACT.md`. Cualquier usuario podrá consultar solo campos públicos de otro participante por respuesta, mención, número o `user <id>` cuando exista un resolvedor de identidad verificado; esa ruta **aún no está conectada**. Se introdujo el parser común y la protección Anti lo usa; otras familias requieren migración explícita, pruebas y permisos por comando. Las escrituras globales no están implementadas y siguen bloqueadas.

## 2026-11-19 — Comandos Anti separados por consulta, edición y ámbito

- `=/>anti`: inventario de filtros del grupo actual, sin modificar valores.
- `=/>anti <tipo>`: estado efectivo de un filtro del grupo; `=/>anti audit` inspecciona contadores pasivos de ese grupo.
- `=/>anti --mod` y `=/>anti --mod <tipo>`: valores predeterminados del **socket actual**, incluidos sus overrides.
- `=/>anti --global` y `=/>anti --global <tipo>`: solo consulta del valor base general **OFF/warn**, no una lectura de configuración centralizada; todavía no existe una política global compartida de Vault. No muestra los cambios particulares de otros sockets.
- `=/>set anti <tipo> on|off|inherit`: habilitación individual por grupo; `=/>set anti --mod <tipo> on|off`: predeterminado del socket. Acciones: `set anti <tipo> action warn|delete|kick` (solo guardan preferencia, no moderan).
- Dominios: `=/>set anti link add|del|list` selecciona el chat firmado, y `=/>set anti --mod link add|del|list` el socket. Añadir un dominio no habilita el filtro.
- No se aceptan `=/>antis`, `=/>antilink on` ni `=/>anti link on` como operaciones. No se permite `set anti --global` hasta disponer de autoridad global autenticada y almacenamiento transaccional central.
- Estado de seguridad: escritura restringida al propietario autenticado del socket, solo grupos activados observados, sin ejecutar mensajes editados, sin borrados o expulsiones automáticas. Bot-Matter sigue bloqueado.
- Objetivo: revisar CI Linux/Windows/proveedor sobre el SHA final y repetir la prueba de menú por pairing-code. Esta anotación no es una certificación de WhatsApp real.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Formato de resolver de comandos en ayuda

- Base alpha anterior 97472dbdca9a5a8a7f6f535fa91d73bccb9abbe4. La verificación Rustfmt solicitó reducir el filtro de nombre exacto o prefijo en `src/menu.rs` a una línea.
- Corrección puramente de estilo; ya está escrita la prueba de la ayuda por cita y de los subcomandos, y el transporte Whatsapp sigue sin ejecutar mensajes citados.
- README 2/2 revisados. CI del nuevo commit pendiente.

## 2026-11-19 — Resolver nombres de subcomandos en help

- SHA previo 42c95c8452528d70dcce19ae82b0dad187545610. El test de ayuda citado reveló que `help set role` intentaba interpretar `role` como un número de página de categoría.
- Solo una segunda palabra numérica selecciona página. Las consultas `help anti audit`, `help set role` y las citas con subcomandos ahora buscan el descriptor específico más largo; no ejecutan nada.
- Test ampliado. READMEs 2/2 revisados; CI de nuevo SHA pendiente.

## 2026-11-19 — Corrección final de formato y Clippy en ayuda citada

- Base alpha anterior 61d7af72039aa9b810a29396e263bc553ed71162. Se aplicaron diferencias exactas de Rustfmt en `src/menu.rs` y `clippy::unnecessary_lazy_evaluations` en el proveedor WhatsApp Rust (respuesta citada).
- `=/>help`, `=/>help anti`, `=/>help set anti link` y el uso respondiendo a un comando conservan seguridad y requisitos de habilitación.
- Archivos READMEs revisados 2/2; CI del nuevo SHA pendiente, aún sin verificar teléfono físico.

## 2026-11-19 — Prioridad de comandos sobre categoría de igual nombre

- SHA anterior: c41fdeb6f2fdefbaf1329376dc0f6418cc9c9a3e. Prueba real de Linux identificó un conflicto: `help anti` tomaba la categoría Anti y no la ficha de comando; también la antigua aserción `Ayuda: antilink` ya no corresponde a la nueva ficha `Ayuda completa`.
- Exactitud: el nombre exacto de comando tiene prioridad en `help`; para categorías siguen `help antis`, `help antis 1`, etc. El menú mantiene categorías sin cambios. Un comando citado se resuelve a metadata, no a ejecución.
- READMEs 2/2 actualizados y pruebas del cambio pendientes. El acceso sin registro y Anti OFF se mantienen.

## 2026-11-19 — Compatibilidad estricta del test de citas

- Base alpha anterior: 425bb2df3ac7a4541ce5a06f6eb4867e32259918. Rust provider 0.7 requiere el trait `MessageBuilderExt` para `Message::text`; en prueba `no_quote` se usa directamente el constructor explícito de `wa::Message`, sin dependencia adicional.
- READMEs 2/2 revisados. Comportamiento de consulta de ayuda citado sin cambios, y sin conceder ejecución por texto citado.
- Pruebas Linux/Windows/proveedor pendientes para el SHA de esta corrección.

## 2026-11-19 — Finalización de Rustfmt en prueba de ayuda por cita

- SHA anterior: 79b3335c521eaab335ad8f8167ae91aef34c847d. El comprobador solicitó solo el orden de imports en `tests/menu_catalog.rs`; se aplicó sin alterar lógica.
- `help <comando>` y `help` respondiendo al mensaje consultan metadatos, nunca ejecutan el comando citado. Anti sigue con ediciones/camuflaje en modo pasivo.
- README revisados 2/2; CI nuevo pendiente. No actualizar Debian hasta Linux, Windows y provider verdes.

## 2026-11-19 — Corrección CI de ayuda en mensaje citado

- SHA anterior 6e0a2956a035e7fb1f05d4f3217cb322b7aac7e7. Se corrigió la importación local de `matter_base::menu` en la prueba de ayuda por cita, sin cambiar el contrato.
- Se aplicó Rustfmt del proveedor: módulo help_reply, match del firmante read-only y orden de imports en main.rs.
- `help` citado devuelve **solo metadatos** y no ejecuta órdenes; se mantiene control por chat, tipo de remitente, HMAC, antirreplay y filtrado de manipulación.
- READMEs 2/2; esperar CI Linux/Windows/proveedor del SHA nuevo antes de probar en WhatsApp.

## 2026-11-19 — Formato y cobertura de ayuda contextual

- Base `alpha` anterior: 80eb18be86e5d35c87101164508e618906575176. Rustfmt exacto aplicado en `src/menu.rs` y `tests/menu_catalog.rs` según CI.
- `help` en respuesta a un mensaje usa citas de WhatsApp Rust sin ejecutarlas; quitada lógica duplicada en el proveedor y dependencia de test aislada bajo `#[cfg(test)]`.
- El acceso público a ayuda específica acepta `help set anti link`, `help anti audit`, `help set role`; las respuestas son solo metadatos. Reglas de bot OFF y Vault sin cambios.
- READMEs 2/2 revisados; esperar CI antes de actualizar servidor.

## 2026-11-19 — Ayuda completa y respuesta al mensaje citado

- Base alpha anterior: 9aa97bdda1113181785768da2d8ae7869f8dec52. `=/>help` solo ofrece el uso y explica la opción de responder a un comando; no muestra categorías.
- `=/>help <comando>` ahora expone categoría, descripción, estado real de catálogo, todas las formas de uso y restricciones documentadas; `help set anti link`, `help anti`, `help register` y `help set role` destacan su estado operativo.
- Proveedor WhatsApp Rust 0.7: al RESPONDER a un mensaje que contenga un comando con `=/>help`, extrae `extended_text_message.context_info.quoted_message` del mensaje recibido, rechaza otra conversación, limita longitud y envía solo `=/>help --quoted <base64url>` firmado por HMAC. El núcleo busca el nombre más específico en el catálogo, nunca ejecuta texto citado.
- Casos de prueba: quote de `set anti link`, `anti audit`, bot, registro, rol, cita cross-chat, cita ausente, contenido inválido y lectura directa. No hay delegación de permisos ni activación de comandos desde citas.
- READMEs revisados 2/2; CI SHA nuevo pendiente. Guardrails Anti conservados y Bot-Matter sigue bloqueado.

## 2026-11-19 — Ajuste Rustfmt de política Anti por grupos activos

- SHA previo d0afffdff09a5f73965f7f640b31dd7141cdf6f8. Apliqué formato de CI para `bot_admission(...).decision.enabled` de `inspect_anti_observation`; no cambia lógica.
- Monitoreo pasivo solo en grupos activados por propietario, edición cifrada sin clave permanece no inspeccionada, comandos camuflados nunca se ejecutan.
- README revisados 2/2. Pendiente CI del nuevo SHA y confirmación real en WhatsApp.

## 2026-11-19 — Privacidad y semántica de ediciones Anti

- Base previo ffb8a7a344d010f870e195571e8d0d14285e6478. La pasarela de inspección Anti registra metadatos de mensajes de grupos **solo si el propietario habilitó ese grupo** según política actual del socket. Grupos apagados se omiten; la pasarela no introduce una excepción de activación.
- Una edición clásica sin `edited_message.text_content()` ya no se marca como descifrada/legible: se considera **opaca sin inspeccionar**, igual que un sobre E2EE sin clave original.
- Sin dar permisos, ejecutar comandos normalizados ni realizar moderación. `anti audit` conserva solo conteos para grupo activo.
- Ambos README revisados; CI del SHA siguiente pendiente.

## 2026-11-19 — Corrección CI de auditoría y documentaciones del proveedor

- Base previo fdbad8252aa080c0988be136236a6a5620bc547e. Se aplicaron las diferencias de Rustfmt de la función `anti audit` y tests.
- Solucionado `clippy::empty_line_after_doc_comments` en `providers/whatsapp-rust/src/lib.rs`: documentos del firmante y del nuevo evento Anti quedan correctamente vinculados a sus métodos.
- `=/>anti audit` sólo muestra conteos, sin cuerpos/identificadores. El puente no ejecuta mensajes editados ni normalizados y todos los antis continúan apagados inicialmente.
- READMEs 2/2 revisados; CI del SHA siguiente pendiente. No habilitar el despliegue hasta tres suites verdes.

## 2026-11-19 — Auditoría de mensajes editados y camuflados

- Base alpha previo: 1aaf76e48930959f45f72bede16e4e33228d6ae4. `=/>anti audit` permite consultar contadores de ediciones legibles, cifradas sin inspeccionar, comandos disimulados y enlaces disimulados del grupo. `=/>anti --global audit` en privado del dueño agrega el socket.
- Se audita simultáneamente edición y camuflaje, sin conservar cuerpo, remitente ni identificador de mensaje. En un mismo evento pueden aumentar varios contadores; son indicadores, no personas culpables.
- Filtros OFF y acciones automáticas desconectadas, no hay borrado ni expulsión. Comandos alterados no se ejecutan.
- Clippy del test con carácter invisible: se usa escape explícito `\\u{200B}`.
- READMEs 2/2 revisados, CI del nuevo commit pendiente. Precaución: ediciones cifradas del cliente sin secreto original se clasifican como no inspeccionadas.

## 2026-11-19 — Corrección CI menú Anti y proveedor

- El SHA anterior `876d0b7420c8214937cdeb76916ab51ac3378622` ya pasó el formato del núcleo, pero Windows indicó una expectativa antigua: `set anti link` está en **página 4**, no en página 1 de Anti; test ajustado con página real. Las páginas 1–4 siguen cubiertas, sin redes sociales.
- El proveedor requirió un ajuste exacto de Rustfmt para la prueba de firma de Anti; no cambia HMAC ni código operativo.
- Menú sigue sin comandos sociales; lista chat/socket y observación Anti firmada con ediciones y camuflaje en estado pasivo. No hay moderación automática.
- README revisados 2/2; CI nuevo pendiente.

## 2026-11-19 — CI Rustfmt para puente de inspección WhatsApp

- SHA previo: 8f8d8981980aef7655651c15d9d7ea79ca978d35. Se aplicaron exactamente las diferencias de rustfmt detectadas en `src/control.rs`, `tests/anti.rs` y `tests/whatsapp_public_readonly.rs`.
- Transporte HMAC de observación Anti (solo grupo; sin acciones), estados original/editado legible/editado cifrado ilegible, antirreplay y normalización sin ejecutar texto.
- READMEs revisados 2/2; CI del nuevo SHA pendiente, todavía no autorizar actualización del servidor.

## 2026-11-19 — Observación Anti real desde WhatsApp, sin conceder permisos

- Base alpha inicial: `c4428b4e4fb9e781aea8a98a85d776d67edf2cbd`. El proveedor Rust ahora observa mensajes de grupos, incluidos textos originales, ediciones clásicas `ProtocolMessage.MESSAGE_EDIT` y sobres cifrados `MESSAGE_EDIT` identificables. Las ediciones cifradas sin clave del mensaje padre se registran como **ilegibles** y nunca se despachan como comandos.
- Un SignedFrame HMAC ya existente, con prefijo interno no disponible como orden pública, enlaza mensaje/grupo/remitente reales a un canal **exclusivamente de inspección**. El puente verifica HMAC, JID, timestamp, antirreplay y ámbito de grupo; no llama al dispatcher administrativo ni manda respuestas.
- El inspector `AntiStore::inspect_message` detecta dominios y camuflaje con metadatos de edición; solo guarda tipo/ámbito en SQLite privado, **no cuerpos ni JIDs de remitentes**. Todos los antis permanecen OFF y moderación aún no está conectada.
- Pruebas nuevas cubren firma del proveedor, edición recibida de otro participante, duplicados, firma alterada y rechazo de monitorización en privado. Se mantienen pruebas de desactivación por defecto y coincidencias de dominio.
- Edición cifrada E2EE moderna requiere recuperar `messageContextInfo.messageSecret` del mensaje padre; no se descifra automáticamente en whatsapp-rust 0.7. Una edición detectada pero ilegible debe tratarse como no inspeccionada.
- README revisados 2/2. CI de esta revisión pendiente; prueba real WhatsApp posterior, Bot-Matter bloqueado.

## 2026-11-19 — Contrato central de inspección editada y camuflaje

- Base SHA previo: 6941fa23706dc41a9005e9c37f820a7e692d18d2. Nuevo módulo `src/anti_guard.rs` con `EditState`: original, editado legible y editado cifrado NO legible. Normalización de análisis detecta separadores Unicode, caracteres invisibles, control bidi, cambios de puntuación, rutas malformadas y prefijos confusos.
- **Regla fundamental:** no despachar nunca un comando normalizado; únicamente inspeccionar y auditar. Un texto alterado o cifrado no representa permiso para ejecutarlo ni para moderar.
- `AntiStore::inspect_message` acepta el estado de edición verificado por el proveedor y aplica el detector `antilink` usando el contenido inspeccionable. Registra únicamente tipo de evento y ámbito en SQLite: ni cuerpo, ni identificador de persona, ni mensaje original.
- Los filtros sin detector específico aún NO reciben adaptadores multimedia (antiimage, antiabuse, etc.); compartir metadatos de edición será la fase de integración del proveedor. Ediciones cifradas modernas requieren clave de secreto del mensaje original en whatsapp-rust 0.7, por lo que se registran como NO inspeccionadas.
- Corrige fallo CI menú: la primera página de Anti contiene `antilink`, no la segunda tras retirar reglas de redes.
- README revisados 2/2. Código nuevo CI pendiente. Sin activar moderación, roles ni Bot-Matter.

## 2026-11-19 — Primer cierre de CI AntiLink

- SHA previo: b5b2615e6989a1fc1c29227bd921c8bab2c4bb07.
- CI detectó diferencias de formato Rust en `src/anti.rs`, `src/control.rs`, `tests/anti.rs` y `tests/menu_catalog.rs`. Se aplicaron las diferencias exactas, sin cambios de lógica.
- Los cambios funcionales anteriores: retiro de Anti por redes sociales, lista de dominios chat/socket, OFF predeterminado, SQLite privado y auditado.
- Próxima fase inmediata: análisis de mensajes editados, camuflaje y detección de protocolo. Historial y pruebas del proveedor; no ejecutar comandos alterados ni prometer lectura de ediciones cifradas sin su clave.
- README revisados 2/2; GitHub Actions nuevo SHA pendientes.

## 2026-11-19 — AntiLink por lista de dominios, sin filtros de redes sociales

- Base alpha anterior: `8b0fc5516e6d8ab7e313469bfbd05bd1103d7f30`.
- Se eliminaron del registro y del menú `antilinkwa`, `antilinktelegram`, `antilinkdiscord`, `antilinkinstagram`, `antilinkfacebook`, `antilinktiktok`, `antilinkyoutube`, `antilinkx`, `antilinkshort` y `antilinkphishing`. Las tablas antiguas conservan sus filas sin ejecutarlas ni eliminarlas; no se reactivan tras reiniciar.
- Queda solo el detector genérico `antilink`. Su lista permite dominios como `t.me`, `instagram.com` o `youtube.com` bajo un **único** filtro. Compara host exacto y subdominios (no substrings); lista vacía => no detecciones.
- Nuevo comando `=/>set anti link add|del|list <dominios>` para el **chat actual** firmado, en grupos o privado, y `=/>set anti link --global add|del|list ...` para todo el socket; alias `=/>set antilink`. El global hereda hacia cada chat. Cambios persistentes SQLite privados, auditados y autorizados al propietario; hasta 12 entradas por comando y 64 por ámbito. No altera encendido. Un global `del` no borra override local.
- En grupos `=/>anti link on|off|inherit` sigue separando configuración de habilitación. Todos los antis OFF inicialmente; aun si está ON el detector es solo lectura y NO ejecuta moderación, expulsión ni borrado automático.
- Pruebas: persistencia, herencia, aislamiento, URL con dominio exacto, rechazo de sintaxis, no ejecución de nombres retirados y menú sin entradas de redes; CI de este SHA pendiente.
- README revisados 2/2. Bot-Matter sigue bloqueado. Siguiente: verificar CI y pedir prueba WhatsApp de `set anti link` en el chat propio y un grupo autorizado.


## 2026-11-19 — Integración futura Matterssmith Commerce (solo contrato)

- SHA Base `alpha` anterior: `24349b0ae29c88ebecda571de01821fe310b8800`.
- Matriz MAT-COM-001..018 en `https://github.com/Matterssmith/Marketplace-Matter/blob/matter/docs/REQUISITOS_ECOSISTEMA_COMERCIO.md`; función de Base: interfaces neutrales, eventos y seguridad, no negocio de tienda.
- Documento `docs/COMMERCE_ECOSYSTEM_BOUNDARY.md`. Sin cambios en código, proveedores, gating, sesiones ni bloqueo de Bot-Matter.
- ZIP del propietario revisado; distingue marketplace de software, generador experimental, relevadores y Tri-Red. No se copian datos privados ni archivos completos.
- README revisado: 1/1 en este repo. Sin pruebas funcionales nuevas; solo verificación de documentación/commit.
- Próximo: mantener prioridades originales de Base-Matter, sin despliegue comercial hasta que se apruebe y se valide.

## 2026-11-19 — Corrección de formato en acceso invitado

- Base alpha anterior: 3c1fa6f325df63718eb67eefb9ce1a66ff4424e4.
- Proveedor WhatsApp verificó PASS para el primer cambio. CI núcleo detuvo por rustfmt: orden alfabético de módulos y dos bloques de tests con espaciado incorrecto. Diferencias exactas aplicadas.
- Política intacta: lectura para invitados sin registro cuando el chat esté habilitado; `=/>register` informativo; Vault no recibe datos y no se asignan roles.
- Vault-Matter `matter` actualizó contrato de invitados en `850071e1bda6f62d68eea802666e45474d379825`.
- README revisados: 2/2. Nuevo CI pendiente; no afirmar teléfono probado.

## 2026-11-19 — Modo invitado estilo demo, registro voluntario

- HEAD remoto alpha antes del cambio: ea7c0da6c579f24406093940e7677fe6a349dfbd. El usuario confirmó que se debe poder usar el bot sin registro, como una versión de prueba: **no imponer CURP, biometría, género, edad ni inscripción Vault para comandos básicos**.
- Nuevo contrato Rust `src/guest.rs`: status e información de registro disponibles sin bot activo; menú, ayuda, diagnósticos de lectura, créditos y ayuda de bot disponibles solo donde el socket/chat fue habilitado; deniega escrituras, perfil Vault, verificación civil y ejecución de código a invitados.
- `=/>register` publica estado de prueba sin crear cuenta; `=/>register start` indica que el gateway de Vault aún no está listo y NO almacena datos. Ruta pública firmada en el proveedor y HMAC/antirreplay normales, incluyendo estado OFF. Descriptor en menú Principal.
- No se lanzó Bot-Matter: el contrato es reutilizable cuando se desbloquee. Ninguna concesión automática ni limitación arbitraria por tiempo. Los bot comandos `#` aún dependen de su runtime preparado y autorizado.
- Dos pruebas nuevas: acceso invitado/read-only con bot desactivado y matriz de capacidades. README revisados: 2/2. CI del nuevo SHA pendiente. Siguiente: confirmar UI de WhatsApp y contratos Vault del invitado; continuar corrección Anti pendiente.

## 2026-11-19 — Clippy Rust del receptor ListResponse

- CI del SHA 031ebb79ad1edc7c81d68a8997ba9c11918681b5: proveedor compiló y todos sus tests pasaron; Clippy estricta falló en tests/ui.rs por un `..Default::default()` innecesario en `SingleSelectReply`. Se quitó sin modificar protocolo ni permisos.
- README revisados 2/2. CI del nuevo SHA pendiente. Siguiente: lista de dominios personalizable en vez de reglas anti por red social, después prueba real con =/>test ui.

## 2026-11-19 — Respuestas de listas WhatsApp y verificación auténtica del botón

- Base alpha: 37698e3967af5b2857c252473d5ad792a289a3e1. Una selección del menú produce burbuja, pero no dispara respuesta como el texto escrito manualmente.
- Añadida lectura de ListResponseMessage.single_select_reply.selected_row_id y TemplateButtonReplyMessage.selected_id a NativeFlow/ButtonsResponse existentes; IDs bajo lista blanca estricta. Firma HMAC, identidad y antirrepetición se mantienen.
- El ACK manual es una prueba de comando, no de botón. Solo el evento interactivo decodificado y autorizado muestra «✅ Botón verificado»; un log sin teléfonos confirma el evento.
- Test de respuestas ListResponse y de rechazo de IDs no autorizados. README revisados 2/2; CI y prueba en móvil pendientes.

## 2026-11-19 — Corrección Rustfmt + tests TS18/TS19 nativos

- Base alpha: 351d4d4a416cf8aa6b15fc33a6607bf1150763b7; Linux y Windows del cambio de menú sin fallo conocido al iniciar esta corrección, proveedor falló por `cargo fmt --check` antes de compilar.
- Aplicadas las cuatro diferencias exactas de formato que reportó CI en `providers/whatsapp-rust/src/main.rs` y `src/ui.rs`.
- Prueba nueva `native_ts18_ts19_buttons_use_quick_reply_and_readonly_ack_ids`: verifica 2/3 quick replies, los IDs cerrados Audio/MP4/Enlace, el texto mostrado y el enrutamiento de ACK. No activa permisos ni descargas.
- Las filas del selector permanecen como títulos exactos `=/>menu <categoría>` sin `Abrir menú`. En grupos y chats sin activación se mantienen las restricciones existentes.
- CI del siguiente SHA pendiente. HTML RichResponse sigue sin representación móvil confirmada. README revisados: 2/2.

## 2026-11-19 — Burbuja de selección y tarjetas compatibles

- Reporte visual Android: al pulsar «Categorías», el cliente publicaba «☾︎ Anti / Abrir menú» en vez del comando elegido. El selector ahora usa `title: "=/>menu anti"` y elimina `description`. El ID opaco y su enrutamiento HMAC siguen intactos; la burbuja depende de cómo WhatsApp represente esa fila.
- `MATTER_WA_MENU_LAB=1` añade tarjetas NativeFlow de prueba TS18/TS19 en menús raíz de **chats privados habilitados**. Sus botones únicamente emiten ACK de lectura y pasan por las comprobaciones existentes. Si el envío nativo falla, se intenta la tarjeta legacy.
- El chat privado del propietario compara adicionalmente el transporte legacy y sigue siendo el único lugar que puede ensayar Rich HTML y audio. No se envía HTML, multimedia ni comparaciones de legacy a lectores externos o grupos.
- RichResponse HTML no equivale a HTML web y el cliente puede descartarlo sin dar error: estado **experimental sin visualización confirmada**. El HTML offline continúa disponible.
- No se modifican roles, antis, activación, secretos ni derechos de ejecución. Bot-Matter permanece bloqueado. CI Linux/Windows/proveedor y prueba de teléfono para este SHA pendientes. README revisados: 2/2.

## 2026-11-19 — Instrucciones compactas y citas WhatsApp (propuesta de Leito)

- Base GitHub alpha: `8a1176ff7cdfe1d86c5c5d689c4ec2e9cd1620c0`. Nueva sugerencia de Dev MS Leito Delfin 3 Siss respaldada por capturas del menú en Android.
- `=/>menu` en estilo `classic`: sustituir seis explicaciones extensas con marcos `│` por cuatro líneas breves con prefijo literal `> ` de cita WhatsApp. Conservar encabezado, cierre, categorías visibles, botón único y estado autenticado. `status` con bot OFF, identidad para admin, externos OFF y aislamiento/auditoría siguen expresados.
- `style none`: texto igualmente compacto pero sin tipografía ni cita decorativa.
- `TextFormat::Quote` permite reutilizar citas de una o varias líneas sin introducir HTML o alterar controles. Registrar `whatsapp.quote` en `templates/message-alphabets.json`; sumar pruebas de cuatro líneas compactas y renderizado reutilizable.
- Cambios sólo de presentación: ninguna operación, permiso, categoría, sesión, Vault ni laboratorio TS18/TS19 se altera. Seguimiento en `AGENTS.md`, `docs/DECISION_LOG.md`, `docs/WORK_CONTINUITY.md`, `docs/DEVELOPMENT_STATUS.md`.
- README revisados: 2/2. CI Linux, Windows y proveedor sobre el nuevo SHA: pendiente. Captura visual del formato cita en WhatsApp: pendiente.

## 2026-11-19 — Corrección de plantilla CRLF en Windows

- Base alpha: 96bd61fc8a61ca745f91e4b89a62d4bf3611832f.
- CI Windows del SHA anterior: el test de tipografía falló porque la plantilla de texto conservó CRLF en Windows, y el cierre terminaba en retorno de carro.
- Solución: normalizar `\r\n` a `\n` antes de renderizar mensajes arbitrarios. No afecta texto, caracteres o permisos.
- Linux había completado sus verificaciones; proveedor en validación del SHA previo. Repetir los tres trabajos para el nuevo SHA.
- README revisados: 2/2; WhatsApp real pendiente.

## 2026-11-19 — Rustfmt final de selector de categorías

- Base alpha: 605ae3bf3da68de49276f1ef9d68d6e1b77f8161; Linux Rust completó correctamente sus pruebas de formato, compilación, tests, Clippy y documentación.
- El proveedor Rust falló sólo en formato de la expresión que valida el manifiesto. Se aplicó el diff exacto del CI.
- Windows y proveedor del nuevo SHA requieren nueva confirmación. Sin cambios en permisos, sesiones, Vault o plantilla visual.
- README revisados: 2/2. Continuidad: validar CI para este SHA y probar =/>menu en WhatsApp real.

## 2026-11-19 — Rustfmt y tipado del cambio de menú

- Base alpha previa: 83cc0f993e24b1ea70659c6e11d5bf8277015302.
- GitHub Actions de la revisión inicial: formato FAIL en proveedor y Rust; se aplicaron exactamente las diferencias de rustfmt de ambos trabajos.
- Se hace explícita la comparación `&str` frente a `String` del manifiesto de categorías activas, y el índice de los alfabetos JSON.
- No cambian categorías, alcance, comandos, botones, permisos ni datos de Vault.
- Siguiente: comprobar CI Linux/Windows/proveedor y después pantalla real en WhatsApp.
- README revisados: 2/2. CI del siguiente SHA: pendiente.

## 2026-11-19 — Cambio visual solicitado: tipografías y categorías activas

- Base remota alpha: 8639b9e4beab5217f344766d74c1b85578b3daba.
- Idea de Leito: negritas para títulos, `código inline` para comandos, soporte de bloques ```código```; separar información y ornamento.
- Ocultar todas las categorías sin comandos implementados en menú principal, agregación, submenús y botón de WhatsApp. Catálogo contractual de 21 entradas preservado.
- Archivos funcionales: src/menu.rs, providers/whatsapp-rust/src/ui.rs, src/message_typography.rs, src/lib.rs, templates/menu-visible-categories.json, templates/message-alphabets.json y templates/message-frame.txt.
- Pruebas de catálogo, interfaz y puente ajustadas; menús heredados por compatibilidad conservan su autenticación HMAC y permisos.
- Continuidad: revisar CI de este commit (Linux/Windows/proveedor). Si falla, corregir en alpha y actualizar estos cuatro registros. Después realizar prueba de pantalla en WhatsApp de =/>menu y selector Categorías, junto a TS18/TS19, sonido y Rich HTML opcional. Beta/stable sin cambios.
- README revisados: 2/2. CI y pantalla real pendientes.

# Checkpoint 2026-11-18 — corrección exacta de test Grupo

- SHA base: ce9465a8cffaa1c7b17dc7596f8798a7fbfb83a8.
- Commit: "fix(alpha): correct positive and negative group menu tests".
- Próxima tarea: comprobar GitHub Actions Linux/Windows y
  provider offline para nuevo SHA; obtener logs en caso de fallo.
- Incidencia: test de provider rechazaba incorrectamente
  "menu unknown" en el conjunto autorizado (y viceversa).
- Estado de funcionalidad: menu HTML/audio/TS18/TS19 ya en
  alpha; falta confirmación de representación en WhatsApp real.
- Flags de ensayo: MATTER_WA_NATIVE_BUTTONS=1,
  MATTER_WA_MENU_LAB=1, MATTER_WA_RICH_HTML=1.
- Ejecución: cargo run --locked desde /home/Sakura/Base-Matter;
  antes git fetch y pull --ff-only en alpha.
- Seguridad: sólo self-chat de dueño para lab. Ninguna clave.
- README revisados: 2/2. beta/stable intactas.

# Punto vigente de recuperación — Base-Matter alpha (18-11-2026)

- SHA base: 97de8ac040148925baa194710a0d5dab82d90aff.
- Commit esperado: "fix(alpha): restore continuity checkpoint and valid readonly menu tests".
- Archivos: providers/whatsapp-rust/tests/provider.rs y los
  cuatro archivos obligatorios de seguimiento (AGENTS, DECISION_LOG,
  WORK_CONTINUITY, DEVELOPMENT_STATUS), ambos README.
- Corrección: prueba de rechazo del signer reemplaza "menu group"
  por "menu unknown". Grupo es categoría legítima.
- Incidencia de proceso: el commit anterior no actualizó este
  checkpoint; el CI lo detectó. Retomar la actualización estricta
  de los cuatro archivos en cada commit.
- CI SHA 97de8ac: proveedor FAIL en test de categoría Grupo,
  Rust Linux FAIL por checker de continuidad; Windows en ejecución.
- CI nuevo: pendiente. No afirmar que la interfaz se vio en móvil.
- Funcionalidad actual para probar: =/>menu en self-chat,
  21 entradas y un selector nativo; con flags MENU_LAB=1 y
  RICH_HTML=1 se prueban tarjetas TS18, TS19, RichResponse y
  audio Ogg/Opus 0.45s. El sonido requiere pulsar reproducir.
- Siguiente: verificar GitHub Actions Linux/Windows y proveedor,
  corregir fallas de pruebas o compilación, registrar un nuevo
  checkpoint en cada commit. Tras verde, iniciar WhatsApp Rust
  con pairing code o sesión existente y probar =/>menu.
- README revisados: 2/2. Rama beta/stable intacta.

# Checkpoint 2026-11-18 — 21 categorías y pruebas de transporte

- SHA base: 954aea37c2c92bc944ceb309780eaa9b1581a87d.
- Commit siguiente: "test(alpha): unlock 21 readonly menu categories and TS18 TS19 regression".
- Archivos: provider src/lib.rs, tests/provider.rs, tests/ui.rs,
  tests/rich_preview.rs, tests/rich_transport.rs, tests/whatsapp_public_readonly.rs.
- Seguridad: sin modificación de HMAC ni roles; sólo permite
  menu páginas 1–3 y las 21 categorías en lectura autenticada.
- Estado CI: pendiente de Linux, Windows y proveedor en el nuevo SHA.
- Siguiente: actualizar documentación de prueba de menú,
  comprobar CI y corregir fallos; luego probar WhatsApp real.
- README revisados: 2/2. beta/stable sin cambios.

# Checkpoint 2026-11-18 — Rustfmt de laboratorio

- SHA base: 05ec05c5bba5bb33b63dcb9aec457e2f8fcedda4.
- Commit: "fix(alpha): rustfmt TS18 TS19 HTML audio menu lab".
- Modificado: lib/main/ui/rich_preview del proveedor, pruebas de
  proveedor/UI/lector público, instrucciones y los README.
- Resultado de CI anterior: fallo de formato (sin ejecutar tests).
- Resultado nuevo: pendiente de GitHub Actions Rust Linux/Windows
  y proveedor independiente.
- Próximo: leer logs CI del nuevo commit, corregir errores de
  compilación o pruebas, actualizar registros por commit, probar
  menú con sonido y botones en WhatsApp real.
- README revisados: 2/2. beta/stable sin cambios.


## Checkpoint 2026-11-18 — experimento TS18/TS19 + HTML + audio
- SHA anterior: 206d50707ec556880fe4e65b912e40245bef722a.
- Rama: alpha. Commit: buscar mensaje "feat(alpha): start menu html audio TS18 TS19 lab".
- Cambio: fuente Ogg/Opus original, envío opt-in en self-chat
  de HTML experimental, tarjetas ts18/ts19, audio pulsable y
  menú normal primero. Flags MATTER_WA_NATIVE_BUTTONS=1,
  MATTER_WA_MENU_LAB=1, MATTER_WA_RICH_HTML=1.
- Estado CI de este cambio: pendiente. Bot-Matter bloqueado.
- Siguiente: corregir allowlist de las 21 categorías y navegación
  en lectores, añadir pruebas UI, revisar Rustfmt, CI y teléfono.
- README revisados: 2/2. beta/stable intactos.


## Punto de recuperación (18 nov 2026 México / 19 nov UTC)

- **Repositorio**: Matterssmith/Base-Matter.
- **Rama de trabajo autorizada**: alpha. No tocar beta/stable.
- **HEAD anterior a este commit**: d6ff4cb2ac9493f0a94a3354ce9a25b65c331787.
- **Nuevo commit**: identificar HEAD de alpha cuyo mensaje sea
  "fix(alpha): preserve decisions and recoverable checkpoints, record Aura consent".
  Consultar SHA real a través de GitHub después de publicarlo.
- **README revisados: 2/2** (README.md y providers/whatsapp-rust/README.md).
- **Cambios de esta revisión**: instrucciones permanentes, historial,
  checkpoint y desarrollo, expediente IA, ambos README,
  test menu/help, test de directorios temporales Windows,
  script y workflow de continuidad.
- **Estado de pruebas de la versión anterior d6ff4cb**:
  * Proveedor whatsapp-rust offline: SUCCESS.
  * Linux: FAILED en tests/menu_catalog.rs: la prueba esperaba
    "📖 Ayuda: Anti" en menu antis 2. El nuevo contrato exige
    que tal encabezado sólo aparezca en help antis 2.
  * Windows: FAILED en tests/command_audit.rs, error Os 183
    AlreadyExists por ruta temporal PID + timestamp.
  * Las demás suites no deben declararse completas por esos fallos.
- **Estado de pruebas del nuevo commit**: PENDIENTE de CI; verificar
  GitHub Actions Linux/Windows, proveedor, SHA y logs antes de
  informar éxito. La validación en teléfono WhatsApp sigue PENDIENTE.
- **Contrato de menú**: las 21 entradas exactas en
  docs/CATEGORY_CONTRACT.md, un botón «Categorías».
  menu: atributos/opciones disponibles, sin detalles.
  help (sin args): sólo sintaxis de ayuda; help con args:
  descripción/uso. No anunciar categorías vacías.
  style classic / none deben conservarse.
- **Permiso IA**: usuario comunica autorización de Jeriel para usar
  su clave de Aura Reed; sin límites específicos comunicados.
  Vault-Matter commit 8872f9c registra el permiso como declaración,
  no como licencia operativamente activa. No hay API key ni
  broker Vault de IA; no habilitar ejecución ni llamadas.
- **Bloqueos reales**: Vault runtime, grant autenticado,
  credencial cifrada/dedicada, pruebas reales de WhatsApp,
  clasificación física de módulos históricos, resto de Etapas H–L.
  Matter-Bot BLOQUEADO.

### Instrucciones concretas para la próxima sesión
1. Consultar AGENTS.md, este checkpoint y docs/DECISION_LOG.md.
2. Leer docs/CATEGORY_CONTRACT.md, docs/MATTER_AI.md y
   docs/DEVELOPMENT_STATUS.md, más ambos README.
3. Consultar rama alpha y SHA exacto de GitHub; inspeccionar
   los workflows Rust Linux/Windows y whatsapp-rust.
4. Si falla CI, abrir logs, arreglar la causa y hacer un nuevo
   commit **actualizando de nuevo los cuatro registros**.
5. Si CI está verde, solicitar prueba real móvil en chat propio:
   =/>menu, =/>menu anti, =/>menu all, =/>help,
   =/>help antilink, =/>menu config, set style none.
   El selector requiere MATTER_WA_NATIVE_BUTTONS=1.
6. Revisar Vault-Matter (rama matter) antes de añadir funcionalidad
   IA; no traer claves desde repositorios de terceros ni chats.
7. Mantener créditos, contrato HMAC/antirreplay/PN/LID, bot OFF
   externo por defecto, Vault autoridad única y ramas separadas.
8. Registrar resultado exacto en el siguiente commit; nunca
   proclamar compatibilidad del teléfono por CI offline.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## Checkpoint 2026-11-18 — experimento TS18/TS19 + HTML + audio
- SHA anterior: 206d50707ec556880fe4e65b912e40245bef722a.
- Rama: alpha. Commit: buscar mensaje "feat(alpha): start menu html audio TS18 TS19 lab".
- Cambio: fuente Ogg/Opus original, envío opt-in en self-chat
  de HTML experimental, tarjetas ts18/ts19, audio pulsable y
  menú normal primero. Flags MATTER_WA_NATIVE_BUTTONS=1,
  MATTER_WA_MENU_LAB=1, MATTER_WA_RICH_HTML=1.
- Estado CI de este cambio: pendiente. Bot-Matter bloqueado.
- Siguiente: corregir allowlist de las 21 categorías y navegación
  en lectores, añadir pruebas UI, revisar Rustfmt, CI y teléfono.
- README revisados: 2/2. beta/stable intactos.


## Punto de recuperación (18 nov 2026 México / 19 nov UTC)

- **Repositorio**: Matterssmith/Base-Matter.
- **Rama de trabajo autorizada**: alpha. No tocar beta/stable.
- **HEAD anterior a este commit**: d6ff4cb2ac9493f0a94a3354ce9a25b65c331787.
- **Nuevo commit**: identificar HEAD de alpha cuyo mensaje sea
  "fix(alpha): preserve decisions and recoverable checkpoints, record Aura consent".
  Consultar SHA real a través de GitHub después de publicarlo.
- **README revisados: 2/2** (README.md y providers/whatsapp-rust/README.md).
- **Cambios de esta revisión**: instrucciones permanentes, historial,
  checkpoint y desarrollo, expediente IA, ambos README,
  test menu/help, test de directorios temporales Windows,
  script y workflow de continuidad.
- **Estado de pruebas de la versión anterior d6ff4cb**:
  * Proveedor whatsapp-rust offline: SUCCESS.
  * Linux: FAILED en tests/menu_catalog.rs: la prueba esperaba
    "📖 Ayuda: Anti" en menu antis 2. El nuevo contrato exige
    que tal encabezado sólo aparezca en help antis 2.
  * Windows: FAILED en tests/command_audit.rs, error Os 183
    AlreadyExists por ruta temporal PID + timestamp.
  * Las demás suites no deben declararse completas por esos fallos.
- **Estado de pruebas del nuevo commit**: PENDIENTE de CI; verificar
  GitHub Actions Linux/Windows, proveedor, SHA y logs antes de
  informar éxito. La validación en teléfono WhatsApp sigue PENDIENTE.
- **Contrato de menú**: las 21 entradas exactas en
  docs/CATEGORY_CONTRACT.md, un botón «Categorías».
  menu: atributos/opciones disponibles, sin detalles.
  help (sin args): sólo sintaxis de ayuda; help con args:
  descripción/uso. No anunciar categorías vacías.
  style classic / none deben conservarse.
- **Permiso IA**: usuario comunica autorización de Jeriel para usar
  su clave de Aura Reed; sin límites específicos comunicados.
  Vault-Matter commit 8872f9c registra el permiso como declaración,
  no como licencia operativamente activa. No hay API key ni
  broker Vault de IA; no habilitar ejecución ni llamadas.
- **Bloqueos reales**: Vault runtime, grant autenticado,
  credencial cifrada/dedicada, pruebas reales de WhatsApp,
  clasificación física de módulos históricos, resto de Etapas H–L.
  Matter-Bot BLOQUEADO.

### Instrucciones concretas para la próxima sesión
1. Consultar AGENTS.md, este checkpoint y docs/DECISION_LOG.md.
2. Leer docs/CATEGORY_CONTRACT.md, docs/MATTER_AI.md y
   docs/DEVELOPMENT_STATUS.md, más ambos README.
3. Consultar rama alpha y SHA exacto de GitHub; inspeccionar
   los workflows Rust Linux/Windows y whatsapp-rust.
4. Si falla CI, abrir logs, arreglar la causa y hacer un nuevo
   commit **actualizando de nuevo los cuatro registros**.
5. Si CI está verde, solicitar prueba real móvil en chat propio:
   =/>menu, =/>menu anti, =/>menu all, =/>help,
   =/>help antilink, =/>menu config, set style none.
   El selector requiere MATTER_WA_NATIVE_BUTTONS=1.
6. Revisar Vault-Matter (rama matter) antes de añadir funcionalidad
   IA; no traer claves desde repositorios de terceros ni chats.
7. Mantener créditos, contrato HMAC/antirreplay/PN/LID, bot OFF
   externo por defecto, Vault autoridad única y ramas separadas.
8. Registrar resultado exacto en el siguiente commit; nunca
   proclamar compatibilidad del teléfono por CI offline.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-25 — Consola y ping alpha

Por solicitud del propietario, registro compacto con nombre de perfil, username y LID si disponibles, grupo/nombre/JID, IPC y TX con latencias y colores ANSI en terminal. `=/>ping` es lectura autenticada, no `#ping`; mide RTT local del controlador. No modificar sesiones ni Vault; metadatos no autorizan acciones. README revisados: 2/2. CI y verificación de formato/compilación pendientes, idkbot reportado caído. No iniciar dos proveedores con la misma identidad.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-25 — Ping de Matter-Bot y corrección CI alpha

El propietario define la plantilla original de Matter-Bot en `docs/MATTER_BOT_PING.md` (Socket, Process, Total). Separada del control autenticado `=/>ping`; `#ping` no se habilita en el proveedor de control. Se aplican diferencias rustfmt exactas de CI y se actualiza prueba IPC: ping autenticado aceptado, replay duplicado y argumentos extra denegados. CI anterior falló formato y expectativa histórica de ping; nueva ejecución pendiente. Error local instabox-se en whatsapp_theme_scopes sin causa inicial en la salida. No tocar sesiones/Vault. README revisados: 2/2.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
