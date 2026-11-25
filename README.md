## 2026-11-25 — Selector exclusivo «Otros menús»

Corrección solicitada por el propietario: la tarjeta conserva su contenido y un único selector final «Otros menús». No se muestran categorías, páginas ni accesos a categorías en ese selector. Cada formato explícito ofrece los otros dos; la vista sin formato específico ofrece los tres. Las categorías y los comandos manuales del catálogo se conservan. El proveedor conserva el contenido real del controlador en web/buttons como respaldo textual, no HTML real, y resuelve correctamente el modo clásico persistido. El receptor admite variantes de ID de selección y nombre NativeFlow ausente, siempre limitados a la lista de comandos de lectura existente; mantiene HMAC, identidad y antirreplay. Se agregaron pruebas de ida y vuelta de cada formato, variantes de respuesta y rechazo de mutación. El diagnóstico no registra contenido ni identificadores privados. README revisados: 2/2. CI, instalación y prueba de selección real en WhatsApp pendientes; no se cambian sesiones ni Vault.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-24 — Auditoría CI: enlaces del README del proveedor

La última ejecución pasó formato, compilación, pruebas del núcleo, documentación, Clippy y registros de continuidad; la auditoría de enlaces detectó una ruta relativa incorrecta en `providers/whatsapp-rust/README.md`: `docs/ENGINE_EXTENSION_REFACTOR_PLAN.md` no se resuelve desde la carpeta del proveedor. Se corrigió a `../../docs/ENGINE_EXTENSION_REFACTOR_PLAN.md`, cuyo objetivo sí existe. Sin cambios de permisos, sesiones, Vault, conexión ni proveedor de WhatsApp. El menú sigue unificado en una sola tarjeta con el selector final y respaldo visible para `menu web`. CI de este commit pendiente.

README revisados: 2/2.

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

## 2026-11-24 — Menú WhatsApp simplificado tras prueba en Android

En `providers/whatsapp-rust` se reemplazó la tarjeta inicial «Control · Categorías 1/2» por dos respuestas rápidas: **«Ver más»** (`=/>menu page 1`, abre selector de categorías) y **«Menú clásico»** (`=/>menu all`, entrega el menú completo **como texto**, sin otro selector). No se fuerzan colores: WhatsApp controla la apariencia de los botones. El ensayo de HTML Rich queda separado con `MATTER_WA_RICH_HTML=1`: el proveedor puede serializar y enviar la estructura, pero la compilación y las pruebas de protobuf **no prueban representación en el cliente**. Para el HTML se usa el catálogo original acotado, evitando rechazos por tamaño del menú dinámico. Conservar `wa-alpha` y Vault, no reemplazar el proveedor sin prueba de dispositivo. Estado: código y pruebas actualizados en `alpha`; **no desplegado ni verificado en idkbot**. Detalles: [docs/WHATSAPP_COLLAPSIBLE_MENU.md](docs/WHATSAPP_COLLAPSIBLE_MENU.md).

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-24 — Tema del menú HTML basado en Matter Register

El propietario pidió usar Matter Register como referencia visual directa. Se revisó `web/matter-register/app.js` desde la rama `feat/matter-register-web`: variables originales `--bg:#070b14`, `--a:#46e7ff`, `--b:#7386ff`, `--ink:#e8f8ff`, `--muted:#abc3d4`; paneles de vidrio oscuro, bordes luminosos, marca con símbolo ✦, partículas sutiles, chips y tratamiento de `prefers-reduced-motion`.

Se adaptaron **sin copiar formularios ni identidad civil** esos elementos a `providers/whatsapp-rust/src/rich_preview.rs`: `menu_inline_html` crea una tarjeta compacta Matterssmith con `<details>` cerrado, texto escapado, estética cian/índigo, encabezado y crédito. `menu_html_preview` para navegador conserva sus pruebas de audio y tarjetas desactivadas y ahora usa la misma familia visual. Las regresiones en `providers/whatsapp-rust/tests/rich_preview.rs` comprueban paleta, cierre por defecto y escape de HTML.

El envío enriquecido sigue opt-in mediante `MATTER_WA_RICH_HTML=1` y la validación Android/CI sigue pendiente; no se ha actualizado un binario en idkbot ni interrumpido WhatsApp o Vault. El HTML local colapsado **no ejecuta comandos de chat por sí solo**: los comandos reales permanecen en NativeFlow y en el dispatcher autenticado. Si un cliente no interpreta Rich UI, usar `=/>menu all` o el selector nativo.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-24 — Menú WhatsApp compacto y desplegable (alpha)

Se implementó en el proveedor `whatsapp-rust` una tarjeta raíz `=/>menu` **breve** con selector nativo de categorías y botón `Ver texto`, sin mostrar el catálogo completo hasta el clic. Los IDs existentes devuelven `=/>menu <categoría>` o `=/>menu all` tras autorización, HMAC y comprobación antirreplay, sin nuevos privilegios; `=/>help` conserva su comportamiento. En alpha el selector queda habilitado por defecto con opción `MATTER_WA_NATIVE_BUTTONS=0` para regresar al texto; si el cliente acepta un envío pero no dibuja botones, el usuario puede escribir `=/>menu all` manualmente. La vista HTML offline utiliza `<details>` cerrado por defecto; **WhatsApp no interpreta HTML arbitrario** y su RichResponse experimental no equivale a soporte del cliente. Código y pruebas escritos en alpha; **CI/compilación, renderizado Android y despliegue en idkbot todavía no verificados**. Sesión WhatsApp, Vault y beta/stable no modificados. Detalles: [docs/WHATSAPP_COLLAPSIBLE_MENU.md](docs/WHATSAPP_COLLAPSIBLE_MENU.md).

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Arquitectura aprobada: Engine-Matter y Engine-Extensions-Matter

El propietario dispone que WhatsApp-Matter se renombre a **Engine-Matter** (núcleo de sistemas Matter sin soporte directo WhatsApp/Telegram/Discord) y Telegram-Matter se renombre a **Engine-Extensions-Matter** (conectores externos WhatsApp/Telegram/Discord y futuras redes). Discord-Matter será retirado de forma controlada con respaldo. **Matter-MSEL/** tendrá su ubicación en Engine-Matter como integración del lenguaje oficial independiente MSEL. Social-Matter es sistema propio del motor; Bot-Matter sigue bloqueado. **Extensions-Matter** (sin prefijo Engine) no es el nuevo Engine-Extensions-Matter y continúa separado. Se conserva Base-Matter como biblioteca y contratos sin dependencias cíclicas.

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

## 2026-11-23 — Menú por diseño de plantilla, no por estilo prestado

Por rectificación expresa del propietario, no se imita ninguna interfaz externa. La representación de menu/menu categoría/menu all se ha separado del código: src/menu/theme.rs resuelve @Commands, @CategoriesActive, @CategoryName, @CategoryIcon, @Usage, @Branch y datos verificados; toda estética configurable está en templates/menu/theme.json y templates/menu/classic o plain. La etiqueta de categoría viene de metadatos registrados (Category.es/en), no carpetas. Los contadores se calculan del catálogo y no se escriben en la plantilla. RSS/uptime del controlador se capturan aparte en telemetry.rs; RAM del proveedor y sub-bots/premium siguen sin fuente y se identifican como no verificados. Plantilla binaria incorporada con include_str (requiere recompilar al cambiarla). Contrato completo docs/MENU_TEMPLATE_ENGINE.md. Validaciones CI y teléfono pendientes; el binario de WhatsApp no se ha desplegado. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Validación y decisión sobre telemetría del menú

Conservamos en alpha el bloque de información dinámica (fecha, versión, prefijo, conteo de catálogo) y solo para propietario autenticado las métricas del host y proceso controlador. El CI del SHA 73311eba pasó rustfmt, compilación, todas las pruebas predeterminadas y Clippy en Linux, pero exigió sincronizar los cuatro documentos obligatorios en cada commit; esta revisión los actualiza simultáneamente. Los contadores y métricas de proveedor siguen sin registro; no inferir datos de Aura Reed. Prueba en WhatsApp y despliegue no realizados. Véase docs/MENU_TELEMETRY_AUTOMATION.md. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Datos automáticos en el menú de Matter

El menú `=/>menu` ahora incorpora en `alpha` un bloque informativo propio con fecha y versión real de Base-Matter, comando de control y número de comandos del catálogo; el propietario del socket verificado puede consultar hostname, SO, RAM del host, RSS y uptime del proceso controlador. Los contadores de sub-bots/premium y las métricas del proveedor WhatsApp **no están conectados todavía** y no se muestran números inventados. Es diseño derivado de campos sugeridos por el propietario, nunca copia de identidad o datos de Aura Reed. Contrato, límites, privacidad, pendientes y pruebas en [docs/MENU_TELEMETRY_AUTOMATION.md](docs/MENU_TELEMETRY_AUTOMATION.md). CI y verificación WhatsApp en dispositivo pendientes; Bot-Matter sigue bloqueado. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-21 — Windows Clippy MSEL test imports

En CI de Base-Matter Windows, `cargo check` y todas las pruebas Rust terminaron correctamente para el SHA `1cb11f4`, pero Clippy `-D warnings` detectó dos imports sin uso (`MselCommandRunner` y `PathBuf`) en `tests/msel_bridge.rs`, ya que el test que los usa es solo Linux. Se limitan los imports a `#[cfg(target_os = "linux")]`, conservando las pruebas multiplataforma y el test original opcional. No cambia el motor MSEL, sesiones WhatsApp ni Vault. README revisados: 2/2. [CI de diagnóstico](https://github.com/Matterssmith/Base-Matter/actions/runs/44954149724). Nuevo CI pendiente.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-21 — Test real opt-in Base → lenguaje MSEL

El pipeline Base-Matter Rust en Linux finalizó **SUCCESS** para el commit `1cb11f4b2f36fe963e3d00a6e324c7f6af2c1e0c` ([ejecución 44954149724](https://github.com/Matterssmith/Base-Matter/actions/runs/44954149724), job Linux). Se añade test explícitamente ignorado por defecto `original_msel_checkout_e2e` para conectar **el host Rust de Base** con la **biblioteca nativa C++ real de MSEL privado** en el servidor del dueño, sin copiar ni exponer su código. Requiere `MATTER_MSEL_CHECKOUT` y compilación de `build/libmsel_embed.so`. El nuevo test no afirma ejecución verificada hasta que se ejecute. Ninguna sesión, llave ni transporte WhatsApp se abre o cambia. README revisados: 2/2. Bot-Matter bloqueado.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-21 — Ejecución de bots MSEL con motor original

Base-Matter `alpha` incorpora `MselCommandRunner` y `InboundGateway.ingest_msel`: entrada autenticada, autorización, replay y ejecución de un `.msel` por el **motor nativo original del repositorio MSEL** mediante `msel_bot_message_v1`. `WhatsApp-Matter/matter` aporta la forma neutral del evento sin heredar permisos. Por defecto el puente no envía respuestas; no se cambian sesiones ni se inicia otro proveedor. Pruebas Rust añadidas, CI/e2e aún pendientes. [Manual de integración](docs/MSEL_WHATSAPP_INTEGRATION.md).

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-21 — Bot Factory v2: MSEL/Social-Matter por defecto + auditoría Baileys-Rust

- **Nueva creación de bots (declarativa, no despliega):** `matter-bot-init` usa lenguaje `msel` / motor `matter-msel`, destino `social-matter`, sistema `host` y proveedor `default`. Permite `--language rust` (`matter-rust`), `--platform` repetido para WhatsApp/Telegram/Discord/Social-Matter y `--provider-for plataforma=biblioteca` para proveedores como `baileys-rust`, `whatsmeow` o bibliotecas propias `xyzbaileys`. El selector v1 antiguo permanece en `--legacy-selector` con su default histórico Rust.
- **Seguridad:** el catálogo privado y autorizado decide qué adaptadores/lenguajes existen; el selector no instala servicios, concede roles, usa Vault, migra sesiones ni abre conexiones. Social-Matter y Matter-MSEL siguen sin integración ejecutora certificada desde Base.
- **Auditoría WhatsApp:** `LuferOS/Baileys-Rust/src/Utils/experiments.rs` copiado **sin cambios**, con licencia Apache-2.0 y NOTICE, en `third_party/luferos-baileys-rust/`. No se importó al provider activo (`whatsapp-rust` 0.7.0); `open_webview` abre una URL, no ejecuta HTML arbitrario inline. Renderizado real en WhatsApp sin confirmar.
- **Pruebas:** 8 nuevos tests Rust de contrato creados, pero Cargo/CI aún no verificados en esta revisión. Plan y límites: [BOT_SETUP](docs/BOT_SETUP.md) · [Auditoría/plan](docs/BAILEYS_RUST_EXPERIMENTS_BOT_FACTORY_PLAN.md) · [HTML experimental](docs/WHATSAPP_RICH_HTML_COMPAT.md). La prioridad principal del ecosistema continúa siendo **MSEL P0 → Matter-MSEL P1**.

# MSEL — lenguaje oficial Matter | Prioridad de desarrollo P0

**MSEL (Matter Script Extension Language)** es el **lenguaje de programación oficial y propio de Matter Ecosystem**. La prioridad máxima actual es **diseñar, especificar, implementar y probar MSEL hasta que funcione de forma autónoma**: gramática, lexer, parser, análisis semántico, tipos, MSEL IR, intérprete/compilador, CLI, pruebas y compatibilidad multilingüe.

**Matter-MSEL no es el lenguaje ni la prioridad P0.** Es el nombre de la **integración/soporte de MSEL dentro del ecosistema Matter**, comparable a Matter-Rust: contratos, SDK, host y adaptadores que consumirán MSEL cuando el núcleo ya sea funcional.

**Orden obligatorio:** MSEL funcional (P0) → Matter-MSEL e integración con bots/sistemas (P1) → ampliación de capacidades y plataformas (P2). Las tareas críticas de mantenimiento y seguridad existentes de Base-Matter/Vault-Matter no se abandonan. No se declara compilador o integración implementada sin pruebas.

**Documentación de referencia:** [Especificación de MSEL y matriz tecnológica](docs/MATTER_MSEL_MASTER_SPEC.md) · [Plan de lenguaje funcional v0](docs/MSEL_LANGUAGE_V0_DESIGN.md).

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

---

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

## 2026-11-20 — Conexión Vault opcional, aislada y verificable para wa-alpha

- Comandos WhatsApp `=/>status`, `=/>menu` y `=/>help` confirmados operativos por el propietario en la sesión alpha. La autenticación previa de la sesión es real.
- `scripts/whatsapp-alpha-existing.sh` mantiene **Vault desconectado por defecto** y habilita sólo con argumento explícito `--vault-env /etc/base-matter/vault-alpha.env`. Verifica credencial distinta `base-wa-alpha`, socket `wa-alpha`, llave 0600 exclusiva, grupo IPC y handshake real antes de lanzar proveedor. Sin pairing, sin copiar `session.db` ni usar claves legacy.
- El enrolamiento se hace exclusivamente por script privado de Vault `scripts/enroll-alpha-client-linux.sh`; instrucciones privadas `docs/ALPHA_VAULT_AND_GITHUB_SYNC.md` en Vault-Matter. Todavía **no se ha aplicado ese enrolamiento en el host ni probado `=/>vault`**. Los roles y registros permanecen bloqueados y Base no recibe clave maestra ni token GitHub.
- La sincronización GitHub del ciphertext pertenece a Vault-Matter; no ejecutarla desde Base ni promover alpha a beta/stable sin CI y pruebas reales. Bot-Matter bloqueado.
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

## 2026-11-19 — Corrección de rustfmt del selector y módulos (alpha)

- Error reproducido en CI Linux: únicamente diferencias de formato en src/bot_setup.rs y orden de pub mod en src/lib.rs. Se aplicaron las diferencias exactas del log; no cambia lógica de elección provider/language/system, permisos, Vault ni bot.
- Windows y el proveedor offline pasaban en el SHA anterior; Linux quedó bloqueado por rustfmt antes de ejecutar otras etapas. CI de nuevo SHA por comprobar; ambos README revisados, beta/stable intactos, Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Frontera de servicio Vault compartida sin claves en consumidores

- Base y Bot tendrán identidades independientes frente a la autoridad Vault. Base público no depende de Bot ni lleva URLs privadas, llaves maestras o source de servicios privados. Ningún bot accede al repo cifrado ni sustituye a Vault como proxy privilegiado.
- El almacenamiento local Rust cifrado del custodio pasó CI offline inicial. El gateway mTLS/IPC, roles remotos, revocación y auditoría cifrada aún no existen; por ello el puente continúa denegando. Documento: docs/VAULT_SERVICE_BOUNDARY.md.
- Revisados ambos README; cambios exclusivamente documentales. Alpha, beta/stable intactos; Bot-Matter bloqueado. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

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

## 2026-11-19 — Corrección de rustfmt en puente privado (alpha)

- El CI Rust de SHA 5d0897cd8c71a895d5e7dc76777494e2ce7ffceb falló exclusivamente en etapa inicial rustfmt del puente `src/capabilities/mod.rs` y las pruebas `tests/private_capability_bridge.rs`; Windows y proveedor independiente estaban verdes. Se aplican únicamente las diferencias exactas del registro GitHub Actions, sin cambiar funcionalidad, reglas, autorización ni secretos.
- Continuidad: mantener `on|off|inherit`, `set` solo configuración; migración funcional de modos Anti pendiente. WhatsApp-Matter ya contiene copia separada de proveedor Rust y contratos Go, TS, JS; el corte de sesión no está autorizado ni probado.
- Revisados README raíz y proveedor. CI de este SHA nuevo pendiente, beta/stable intactos; Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Corrección de sintaxis Anti: on|off|inherit

- **Decisión actual:** los tres operadores de activación son exclusivamente `on|off|inherit`, tanto por filtro como por modo. El guion `-` queda DESCARTADO como operador; no implementarlo, mostrarlo ni usarlo como alias.
- `inherit` elimina únicamente el override del ámbito autorizado y hereda el valor superior. `set` permanece reservado a configuración; `anti` a consulta y activación. Se conserva `--include` / `--except` para selección dentro de modos.
- Especificación rectoría: [docs/ANTI_COMMAND_CONVENTIONS.md](docs/ANTI_COMMAND_CONVENTIONS.md). Corrección documental, **no** parser Rust, ni cambios de permisos, ni pruebas WhatsApp. Migrar runtime, help y menú antes de declarar el comando funcional. Beta/stable intactas. Bot-Matter sigue bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Reglas definitivas propuestas: Set configura; Anti activa y hereda

- **Diseño pendiente, sin cambio Rust:** `set` se reserva para configuración. `anti [regla] on|off|-` controla activación o retiro de override para heredar el valor superior; `set anti link on` queda descartado de la gramática objetivo.
- `anti --<mode> on|off|-` opera sobre un modo completo, opcionalmente limitado por `--include` y `--except` a filtros registrados (no archivos del servidor). Modos previstos: `--security` como subconjunto Anti y `--multimedia`, sin confundir Matter Security con filtros Anti.
- `--mod` conserva ámbito del socket; `--global` es predeterminado general y no puede editarse en modo vivo sin Vault. `-` nunca borra la política global. Actuaciones masivas requieren autorización y transacción/auditoría; en fallo se deniegan.
- Documento rector de continuidad: [docs/ANTI_COMMAND_CONVENTIONS.md](docs/ANTI_COMMAND_CONVENTIONS.md). Hay que actualizar router, ayuda, menú y pruebas en tarea posterior. No declarar implementado ni probado; `alpha` documentación solamente, sin modificar `beta`/`stable` y Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Puente público para capacidades protegidas (alpha, no desplegado)

- Se creó la API Rust de invocación anti.link en src/capabilities/anti_link.rs; sólo arma peticiones, no contiene implementación privada, claves ni permisos. Ausencia de gateway = rechazo. Los antis y =/>anti link existentes no cambian.
- Integradores podrán asociar su comando #antilink al puente mediante un gateway autenticado cuando exista, sin recibir código de ejecución ni acceso a repositorios privados. El ejecutor protegido deberá comprobar decisiones actuales y verificadas por alcance; los datos públicos de contexto no autentican a nadie.
- Scope: grupo, socket (--mod) y valores generales de lectura (--global). No se permiten mutaciones generales. Especificación: docs/PRIVATE_CAPABILITY_BRIDGE.md.
- README raíz y del proveedor revisados; pruebas unitarias de contrato añadidas. CI y pruebas reales pendientes; gateway privado y autoridad remota todavía NO desplegados. Alpha solamente, beta/stable intactos; Bot-Matter bloqueado.
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

## 2026-11-19 — Verificación de formato del test Anti

Se aplicó el ajuste exacto solicitado por Rustfmt en dos aserciones de herencia de `tests/anti.rs`. Semántica del comando sin cambios: `anti` solo lectura, `set anti` escritura autorizada, `--mod` socket, `--global` solo valores base generales no sincronizados. CI del nuevo SHA pendiente; no actualizar el servidor hasta pasar los verificadores.

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


## Iniciativa futura Marketplace-Matter: límites de Base-Matter

[Contrato de integración sin dependencia comercial](docs/COMMERCE_ECOSYSTEM_BOUNDARY.md) · [Requisitos globales](https://github.com/Matterssmith/Marketplace-Matter/blob/matter/docs/REQUISITOS_ECOSISTEMA_COMERCIO.md). Esta actualización no habilita funciones de tienda ni de Bot-Matter.

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

## 2026-11-18 — Se corrigió la posición de la prueba de Grupo
El commit anterior sustituyó por error el caso POSITIVO "menu group"
y dejó Grupo en la lista NEGATIVA. Ahora la lista positiva contiene
Grupo (una de las 21 entradas) y la negativa prueba "menu unknown".
No se modificó el signer ni se ampliaron permisos. README
revisados: 2/2. Revalidación del nuevo SHA pendiente.

## 2026-11-18 — Corrección de test y checkpoint tras CI

CI del SHA 97de8ac mostró:
- La suite del proveedor marcó fallida una aserción heredada que
  rechazaba `=/>menu group`, aunque «Grupo» es una de las 21
  categorías autorizadas. El caso negativo ahora es
  `=/>menu unknown`; todas las categorías siguen probándose.
- La verificación documental detectó que el commit previo omitió
  modificar docs/WORK_CONTINUITY.md. Este commit lo corrige
  y refuerza la regla de registrar el checkpoint SIEMPRE.
- Los cambios son tests/documentación, no permisos de escritura.
- README revisados: 2/2. CI nuevo pendiente.

## 2026-11-18 — Test real de 21 categorías y documentos de WhatsApp

La plantilla antigua de pruebas sólo renderiza "menu" raíz y
rechaza submenús. Se movió la prueba de las 21 categorías y páginas
a un fixture dinámico con MenuPreferences en memoria, sin alterar
la política de producción. Corrección Rustfmt del provider lib.rs.
Se actualizaron las guías del menú TS18/TS19, RichResponse y sonido.
README revisados: 2/2. CI y prueba real del teléfono pendientes.

## 2026-11-18 — Corrección Rustfmt laboratorio y listas canónicas

El CI detectó formato no estándar en los módulos del proveedor, en
el catálogo de 21 rutas y en sus pruebas. Se aplicó exactamente el
estilo solicitado a imports, match, tarjetas y arrays de test.
No se modificó la autorización de escritura ni se habilitó IA/Vault.
README revisados: 2/2. CI del nuevo SHA por confirmar.

## 2026-11-18 — Selector: 21 categorías y páginas admitidas (pruebas)
El proveedor de entrada de lectores acepta exclusivamente las 21
categorías autorizadas y páginas 1–3, además de aliases antiguos,
siempre como lectura y sujetos a identidad PN/LID, HMAC, antirreplay
y activación. Se añadieron pruebas para cada opción y para botones
TS18/TS19 Audio/MP4/Enlace y HTML con audio no automático.
La prueba de `=/>menu group` ahora debe pasar (es una categoría
autorizada); comandos no registrados siguen rechazados.
README revisados: 2/2. CI de este SHA pendiente.

### 2026-11-18 — Prueba opcional HTML, sonido y botones en menú
En alpha, el primer =/>menu sigue mostrando 21 categorías y el
selector Categorías. MATTER_WA_MENU_LAB=1 añade sólo en self-chat
autenticado tarjetas TS18 y TS19, tono original en audio
cifrado Ogg/Opus y HTML RichResponse cuando
MATTER_WA_RICH_HTML=1. El audio requiere pulsar reproducir;
HTML no ejecuta JavaScript y puede no aparecer en WhatsApp.
README revisados: 2/2. CI y teléfono pendientes.

### 2026-11-18 — Continuidad obligatoria y permiso Aura Reed comunicado

El usuario exige registrar cada idea y cambio en instrucciones, un
historial de decisiones y un checkpoint exacto para reiniciar tras
desconexiones. [AGENTS.md](AGENTS.md) es la norma vinculante;
[continuidad](docs/WORK_CONTINUITY.md) y
[decisiones](docs/DECISION_LOG.md) se actualizan en cada commit.
El workflow Rust lo exige. La documentación de IA
[docs/MATTER_AI.md](docs/MATTER_AI.md) remite al permiso comunicado
por Jeriel, registrado en Vault-Matter (sin límites específicos
comunicados, pero sin clave o broker activo). Se corrigen una
aserción de menu/help y una colisión temporal Windows.
**README revisados: 2/2**. CI de este nuevo SHA pendiente.

### 2026-11-18 — Rustfmt y ayuda sin categorías
Se aplicó formato Rustfmt en la ayuda por página, la disponibilidad de comandos, las variantes del menú y las pruebas. `=/>help` sin argumentos muestra exclusivamente formas de ayuda, sin copiar el menú. Ambos README revisados 2/2; se requiere nueva verificación.

### 2026-11-18 — Menú de atributos separado de ayuda contextual
Las páginas de `=/>menu` sólo enumeran variantes reales, sin descripciones ni pendientes; `=/>menu all` es un índice compacto. `=/>help` sin argumentos indica la sintaxis de ayuda y no copia categorías. `=/>help <categoria|comando>` mantiene los detalles. Invocaciones sin una operación de cero argumentos asignada, como `=/>set` y `=/>bot`, muestran uso. Ambos estilos (`classic` y `none`) siguen disponibles. READMEs revisados 2/2. Validar Linux, Windows y proveedor antes de usar en WhatsApp.

### 2026-11-18 — Compilación Windows: nombre local del rótulo de categoría
El compilador de Windows detectó que la variable local label ocultaba la función label del renderizador. Se renombró el local a decorated_label sin variar presentación ni permisos. Ambos README revisados; se requieren verificaciones completas del nuevo commit.

### 2026-11-18 — Formato final del menú sin símbolos
El CI detectó una única diferencia de Rustfmt en el nombre de Todo del estilo none; se aplicó literalmente sin cambiar comportamiento. Se mantienen 21 rutas, sección de autoridad no otorgante y botón de categorías. README principal y proveedor revisados 2/2; revalidación por SHA.

### 2026-11-18 — Rustfmt de menú autenticado y texto sin diseño
CI detectó formato en control_ipc.rs, control_style.rs, src/menu.rs y tests/menu_catalog.rs. Se aplicó el formato exacto. `style none` ahora enumera nombres simples de las 21 entradas, sin glifos de la lista clásica. El formato visual clásico, la autoridad por socket y los controles de seguridad no cambiaron. READMEs revisados 2/2; verificar nuevo SHA.

### 2026-11-18 — Corrección de saltos y delimitadores del menú completo
Se corrigió la serialización UTF-8 para que el marco use saltos de línea reales, no secuencias escapadas, y se agregó la cenefa completa al detector de respuestas ya decoradas. El test de límite NativeFlow se ajustó a 3900 bytes; nueva prueba valida formato clásico, identidad verificada, variante sin diseño, ausencia de rótulos vacíos y separación de la sección de autoridad para lectores externos. Revisados ambos README; CI del SHA anterior detectará/confirmará diferencias adicionales.

### 2026-11-18 — Formato completo de menú global y variante sin diseño
Se adoptó la cenefa, encabezado, 21 filas inmutables, sección descriptiva de autoridad, campos dinámicos de identidad verificada, comandos, rango, estado y socket, instrucciones y cierre. La ruta de IPC sólo rellena datos después de HMAC/antirreplay/admisión, no deriva Owner Global de un número. Se suprimió «Sin comandos registrados» y el menú all omite encabezados sin comandos. `style none` sigue mostrando categorías y datos. El selector conserva un único botón; se aumentó su límite de texto a 3800 bytes sin alterar límites de transporte ni permisos. Revisados ambos README y AGENTS; la comprobación CI y teléfono real siguen siendo necesarias.

### 2026-11-18 — Formato de prueba de catálogo cerrado
CI del SHA 653b6ec marcó únicamente formato de la prueba Rust del catálogo; se aplicaron exactamente los saltos de línea solicitados por Rustfmt. La aserción de IDs compara explícitamente slices en orden. README principal y proveedor revisados (2/2); se vuelve a validar. No hay cambios de transporte, sesión, Vault ni permisos.

### 2026-11-18 — Invariante de 21 entradas y comandos únicos
Se añadió una prueba que compara el orden exacto de las 21 filas con el contrato cerrado, exige 20 destinos de comandos distintos de Todo y evita duplicados. El comando general help se ubica en Principal, no IA; matter permanece en IA. Los módulos internos siguen conservando rutas históricas sin exponerse como categorías. READMEs revisados 2/2; validar CI del nuevo SHA.

### 2026-11-18 — Catálogo de 21 entradas aplicado a la presentación Rust
Se reorganizaron los descriptores existentes dentro de IA, Anti, Configuración, Información, Principal, Modo, Propietario, Sistema y Utilidades. Otras entradas autorizadas no contienen comandos inventados. Todo sigue siendo una vista agregada. Aliases anteriores permanecen sólo como compatibilidad. Un único selector «Categorías», paginado en tres pantallas por límites del cliente, mantiene 21 entradas accesibles. Rustfmt corrigió las diferencias que reportó CI del SHA 457de96; se vuelve a validar Linux, Windows y el proveedor. Ver docs/CATEGORY_CONTRACT.md.

### 2026-11-18 — Catálogo de categorías cerrado por instrucción del propietario
El contrato [docs/CATEGORY_CONTRACT.md](docs/CATEGORY_CONTRACT.md) registra las 21 entradas exactas del menú original y prohíbe categorías inventadas. Todo = vista agregada. Establece destino de cada comando histórico, aliases de compatibilidad y un único botón «Categorías». La documentación **no equivale a migración completa del código**; pendiente adecuar el renderizador y las pruebas.

### 2026-11-18 — Diagnóstico sin argumentos incluso con bot OFF
La ruta pública de =/>test ahora devuelve un informe mínimo cuando
el bot está apagado, en lugar de descartarlo sin respuesta.
Las pruebas con botones siguen inactivas hasta que el bot esté
habilitado; no ejecutan código. El proveedor añade logs redaccionados
para distinguir falta de firma, rechazo y bot desactivado, sin
imprimir números, JID ni cuerpos de mensajes. Se incorporaron tests
de bot OFF. Los dos README se revisaron (2/2). CI requerido.

### 2026-11-18 — Rustfmt del menú de categorías y regresión de test
El CI del SHA bf6fe38 encontró dos diferencias de Rustfmt:
el iterador de categorías y un salto de línea en tests/whatsapp_bridge.rs.
Se aplicó exactamente el formato requerido sin alterar comportamiento.
READMEs raíz y proveedor revisados (2/2), nuevo SHA necesita CI.

### 2026-11-18 — Restauración del menú visual de categorías
Se adopta exclusivamente el DISEÑO aportado por el propietario:
«╭─💠 𝗖𝗮𝘁𝗲𝗴𝗼𝗿í𝗮𝘀», renglones «│·♪·╠[»,
último «│·♪·╚[» y cierre «╰───────────⟨ 🌀».
Presenta nueve categorías registradas y la opción «Todo»
para =/>menu all. No se inventaron comandos de Economía,
Gacha, Juegos u otras secciones del ejemplo. NativeFlow ofrece
UN SOLO botón «Categorías» (single_select), sin botones
separados Ayuda/Ver todo. Todo está dentro del selector.
El ensayo =/>test ui y el clásico =/>test row conservan
sus botones de diagnóstico independientes.
Vault, secretos, configuración de antis y ramas beta/stable intactos.
Dos README revisados antes del commit (2/2).

### 2026-11-18 — Corrección: =/>test silencioso desde WhatsApp

El proveedor y puente sólo aceptaban =/>test desde el propio chat
privado, por lo que los mensajes en grupos y de otras cuentas no
alcanzaban el handler. Ahora las pruebas visuales de solo lectura
(=/>test, ui, row y ack cerrados) funcionan:
- propietario autenticado PN/LID: propio chat y grupos;
- otros miembros: únicamente chats habilitados, con identidad
  verificada por la misma ruta pública que =/>menu.
HTML sigue restringido al propio privado del propietario y ninguna
prueba concede roles ni ejecuta archivos.
Envío NativeFlow/ButtonsRow permitido tras un ACK firmado para esas
pruebas; el emisor recibe fallback de texto si el transporte falla.
Pruebas de regresión cubren owner grupo/privado, replay, público off/on,
y rechazo de HTML externo. No verificar renderizado físico desde CI.
README (2/2) revisados; beta/stable no se modificaron.


### 2026-11-18 — Formato confirmado para tarjeta de prueba ButtonsRow

El CI del SHA 62399fa se detuvo antes de compilar por Rustfmt:
tres diferencias en tests/ui.rs y una en el match de src/control.rs.
Se aplicó la salida literal de Rustfmt, sin modificar el comportamiento
del ensayo clásico ni el protocolo. Dos README revisados (2/2).
Revalidar compilación/test/Clippy sobre el nuevo SHA.

### 2026-11-18 — Comparación WhatsApp: NativeFlow y ButtonsRow clásico

Se agregó =/>test row, una tarjeta de texto basada en la familia
protobuf ButtonsMessage con tres botones (Audio, MP4, Enlace),
equivalentes al patrón Go ts19 recibido. Las respuestas de
ButtonsResponseMessage se traducen a acuses de solo lectura bajo
el puente autenticado, sin descargas ni ejecución de código.
No se agrega imagen de cabecera: necesita un upload cifrado real,
que será una etapa posterior, ni se da por probado el renderizado
en el cliente. README 2/2 revisados, CI del SHA requerido.

### 2026-11-18 — CI de pruebas visuales: Rustfmt y Clippy

En el SHA 1291d43, la compilación y los tests por defecto de Linux
pasaron; el proveedor se detuvo en un formato de importación y
un salto de línea adicional en tests/ui.rs. Clippy detectó una
comparación booleana innecesaria en tests/menu_catalog.rs.
Se aplicó el formato exacto de Rustfmt y se sustituyó la aserción
por la comprobación real del uso =/>antilink on|off|inherit.
READMEs raíz y proveedor revisados (2/2), sin cambios de runtime.

### 2026-11-18 — Pruebas de menú y botones ejecutables desde WhatsApp (alpha)

El comando privado del propietario =/>test ui construye una prueba NativeFlow
con dos botones y dos filas; al seleccionarlos, el puente de control
normal verifica identidad de la sesión, HMAC y antirreplay, y emite un
acuse de solo lectura. =/>test html usa únicamente el transporte
experimental ya existente y requiere MATTER_WA_RICH_HTML=1.
Se documentan instrucciones de verificación visual manual en
docs/WHATSAPP_UI_TESTS.md.

Se corrigió también el filtro de ayuda pública para permitir nombres
de comando seguros como =/>help antilink y se admite help antis 2
en el renderizador. No se autoriza ninguna mutación pública.
El campo "test" del menú corresponde solo a pruebas visuales;
test <código> y ejecución remota permanecen pendientes.
Dos README revisados (2/2). No se alteran sesiones, Vault ni ramas beta/stable.

### 2026-11-18 — Clippy final del prototipo de sticker

CI confirmó compilación y pruebas del proveedor; Clippy -D warnings
encontró únicamente una importación no usada en el test
sticker_badge.rs. Se eliminó. El transporte productivo, roles,
llaves y bases no cambiaron. Dos README revisados (2/2).
Revalidar CI para este SHA antes de elevar a beta.

### 2026-11-18 — Formato requerido para prototipo de insignias

El CI del prototipo se detuvo únicamente en cuatro diferencias de
Rustfmt: orden de módulos, agrupación de dominios, imports del test
y array de comandos legado. Se aplicaron los cambios exactos, sin
alterar autorización, protocolo ni flujo de ejecución.
READMEs raíz y proveedor revisados 2/2; nuevo SHA requiere CI.

### 2026-11-18 — Prototipo Rust offline: sticker asociado al mensaje

Se añadió providers/whatsapp-rust/src/sticker_badge.rs y sus pruebas
offline. La función construye un mensaje protobuf con
MessageAssociation/STICKER_ANNOTATION (associationType 11), valida
la clave real de destino y metadatos cifrados del sticker y no envía
mensajes. Está aislada del receptor productivo y no autoriza rangos.
Vault-Matter contiene únicamente el contrato documental/JSON de las
insignias, sin gateway de roles en producción. Boom/boom2 NO se habilitan.
CI del nuevo SHA obligatoria, seguida de compatibilidad en teléfono.
README raíz y proveedor revisados antes del commit (2/2).

### 2026-11-18 — Proveedor: expectativas corregidas para help privado

El último CI del proveedor compiló, pero la prueba provider.rs todavía
prohibía =/>help y =/>h desde el chat propio, aunque la nueva ruta
firmada y autorizada de ayuda ya los permite. Se trasladan esos
comandos a los casos positivos, incluidas help all y help antis.
Las órdenes on/off retiradas siguen prohibidas. No se cambió
la función de autorización ni se amplió a terceros; es actualización
de prueba acorde al contrato de ayuda previamente incorporado.
READMEs revisados 2/2, nuevas pruebas CI por SHA necesarias.

### 2026-11-18 — Corrección de tipado en selector público

El CI del SHA 8c4c79d superó el formato del proveedor pero reveló
dos comparaciones inválidas de referencias dobles en src/lib.rs:
&&str frente a str. Se aplicó la corrección de tipos *category,
sin ampliar la lista de categorías ni los permisos.
README raíz y proveedor revisados (2/2); revalidar el nuevo SHA.

### 2026-11-18 — CI: corrección de formato UI y colisiones Windows

El SHA fb90999e falló en formato del proveedor UI, espacios en blanco
del archivo de tests y una prueba de SQLite en Windows porque el nombre
temporal usaba reloj/PID y podía colisionar tras reiniciar el test.
Se aplicó el formato exacto indicado por Rustfmt a los dos archivos
de NativeFlow; el directorio de prueba de replay ahora usa 128 bits
aleatorios, reintento de colisión y limpieza por RAII.
El menú/código de ejecución, Vault, roles y sesiones permanecen igual.
Se revisaron los dos README (2/2). CI debe confirmar este nuevo SHA;
no inferir compatibilidad del teléfono por pasar pruebas offline.

# ꧁✪⃢.✪ 𝐌𝖆𝖙𝖙𝖊𝖗𝖘𝖘𝖒𝖎𝖙𝖍۞₪»̶ ༄☾︎𝐌𝐒☽︎༄

<div align="center">

<img
src="https://capsule-render.vercel.app/api?type=waving&color=gradient&customColorList=12,19,24&height=240&section=header&text=𝐌𝖆𝖙𝖙𝖊𝖗%20𝐁𝖆𝖘𝖊&desc=The%20Core%20Architecture&fontSize=52&fontColor=FFFFFF&animation=fadeIn&fontAlignY=38&descAlignY=58&descSize=20"
width="100%"
/>

# Base-Matter (antes Matter-Base)

### The Core Architecture.

**Powered by Matterssmith.**  
**Engineered for the Future.**

*A reusable Rust foundation for the Matterssmith ecosystem.*

</div>

---

### Selección interactiva por categorías (alpha; prueba real pendiente)

El proveedor `whatsapp-rust` ahora puede generar un botón `single_select` de nueve categorías y dos respuestas rápidas de Ayuda/Ver todo; las selecciones se traducen exclusivamente a `menu`/`help` firmados. También lee `ButtonsResponseMessage.selectedButtonId`, compatible con el patrón de botón de fila del ejemplo Go, sin habilitar descargas. Ningún botón muta bot/roles/antis. El envío aceptado sigue sin probar renderizado real. Dos README revisados.


### Corrección de formato Rustfmt — navegación y ayuda (alpha)

Se aplicaron los cambios de formato exactos emitidos por GitHub Actions en `src/menu.rs`, `src/bin/whatsapp_control.rs` y el proveedor Rust. Sin cambios semánticos ni de sesión. README 2/2 revisados y actualizados; se requiere nuevo CI para el commit.


### Corregidos permisos de ayuda y menú completo

Se normalizaron los verbos de ayuda en grupos y privados, retirando duplicados y permitiendo `menu all` en lectura pública de chats activos. Dos README inspeccionados y actualizados. Sin cambios de sesión, Vault ni rangos.


### 2026-11-18 — Primer corte de UX: menú corto, menú completo y ayuda

El menú principal ahora muestra categorías legibles. `menu all` expone el catálogo íntegro; `menu antis` resume las protecciones sin enumerar redes sociales. Se añadió `help` y `help <comando>` al controlador y al puente de WhatsApp; sigue siendo lectura sin ejecución. `set antilink add/del` y los botones/HTML reales requieren la siguiente implementación. README raíz y del proveedor revisados (2/2); no se tocaron sesiones ni Vault.


### 2026-11-18 — Referencia del propietario: tarjetas con botones de fila (Go, solo documentación)

Se preservó literalmente el ejemplo Go `ts18`/`ts19` recibido del propietario en
`docs/references/ts18_ts19_buttons.go.txt`. Su punto de interés es
`SendButtonsRow(title, text, footer, thumbnail, buttons)` con dos botones
para `ts18` (Audio/MP4) y tres para `ts19` (Audio/MP4/Enlace).
**Este código es referencia de interfaz, no ejecutable ni integración de Go.**
No implica que el proveedor Rust haya reproducido esos botones ni prueba
compatibilidad HTML, tarjetas o selectores grandes en WhatsApp.
Documentación: [referencia técnica](docs/WHATSAPP_BUTTON_CARD_REFERENCE.md).
READMEs raíz y proveedor revisados antes de publicar; no se cambia
sesión, Vault, roles, antis, seguridad ni Bot-Matter.


### 2026-11-18 — Corrección final de formato del proveedor

Para `dcf39b2`, la verificación Linux del núcleo quedó
**verde**: Rustfmt, check, suite de pruebas y Clippy.
El proveedor independiente rechazó únicamente la
distribución de líneas del match de nueve categorías
permitidas; se aplicó el formato exacto requerido
sin ampliar permisos ni modificar la allowlist.
Windows seguía ejecutándose en la consulta anterior.
Volver a verificar todos los flujos para el siguiente
SHA antes de probar en WhatsApp o pasar a beta.
Sesiones, roles, Vault y claves sin modificar.
Se revisaron ambos README antes del commit.

### Control de pruebas de WhatsApp — validación de submenús (alpha)

En `578c680` el CI de Rust pasó el formato inicial
pero la verificación del proveedor detectó una regresión:
el test antiguo rechazaba `menu <categoría>` mientras el
nuevo diseño ya admite esas categorías. Se corrigió sin
abrir un comodín de comandos públicos: el proveedor
firma únicamente las **nueve categorías registradas**
y páginas numéricas entre 1 y 8; continúa rechazando
categorías arbitrarias y operaciones administrativas.
Las pruebas de solo lectura fueron ampliadas para
exigir el reconocimiento de `menu set`, `menu antis`,
`menu antis 2` y `menu system`. Se aplicaron dos
reordenamientos Rustfmt, manteniendo cada comando
en su propio archivo. No se tocan sesiones, APIs ni roles.
La versión nueva aún necesita CI verde y verificación
real de los botones en WhatsApp. README raíz y del
proveedor revisados antes de esta publicación.

### CI del menú modular — Clippy y proveedor independiente

El CI de `2943cedd` pasó formato, `cargo check`
y `cargo test` del núcleo Linux, pero Clippy
rechazó módulos cuyo nombre coincidía con el
directorio (regla `module_inception`). Se
conservan los archivos individuales solicitados,
usando identificadores Rust `main_command` y
`listing_command` mediante `#[path]` en los
`mod.rs` correspondientes. No se cambian nombres
de comandos visibles, roles ni sus permisos.
El proveedor requiere dos ajustes menores de
Rustfmt en su decorador autónomo; aplicados.
README raíz y de proveedor revisados para
este commit; validar nuevo SHA antes de promover.

### Revisión CI del catálogo — separación del proveedor Rust

En el commit inicial `44ab5b35`, Linux y el proveedor independiente
detectaron diferencias Rustfmt. Se aplicaron exactamente las
normalizaciones solicitadas al controlador, 55 descriptores,
renderizador, pruebas y proveedor. El proveedor ya no compila
`src/control_style.rs` entero, que ahora requiere el registro
`crate::menu`; dispone de su decorador de respuestas autónomo
en `providers/whatsapp-rust/src/provider_control_style.rs`,
que conserva el mismo formato del marco de respaldo.
El nuevo commit se validará en Linux/Windows/proveedor antes
de considerarlo listo para WhatsApp. No se alteran
activación, sesiones, roles, APIs ni `beta`.

## 2026-11-18 — Catálogo de menú automático por categoría (alpha)

Se integra el catálogo público `src/menu.rs` con nueve
directorios en `src/menu/`, un archivo Rust independiente
por cada comando (55 descriptores), incluidos los 32 antis
de `anti::RULES`. Cada categoría se genera desde su
`mod.rs` y publica descripción, variantes, traducción
es/en y el estado de implementación. **El catálogo no
concede permisos ni ejecuta código**. Las entradas
sin ejecución están etiquetadas `[pendiente]`.

El menú de producción usa el estilo solicitado
`╞═╦═`, `│♪║`, `│♪╠` y `│♪╚`, sin
marca ni créditos automáticos. El prefijo `=/>`
aparece solo en el encabezado. Las categorías
admiten submenús y páginas: `menu set`,
`menu antis 2`, `menu system`.
`set style classic|none` y `set lang es|en`
son opciones privadas de propietario, persistentes
en SQLite propio, independiente de la sesión.

Los botones NativeFlow Estado/Menú/Antis mapean
a `=/>status`, `=/>menu` y `=/>menu antis`;
el evento vuelve al puente autenticado y solo
permite consulta. El formato HTML limitado se
convierte a texto WhatsApp en el mismo mensaje.
La hoja HTML experimental no tiene garantía
de visualización real en móvil.

El puente privado acepta `matter` en su lista
autenticada; la API sigue bloqueada hasta
existir el gateway Vault y concesiones del titular.
Alya Core/Aura Reed: **PERMISO PENDIENTE**,
sin credenciales provisionadas. Sistema, sandbox,
test, console, eval, adjuntos citados y ejecución
real de moderación siguen pendientes.

Se revisaron ambos README; `beta` y `stable`
no se modifican hasta CI y prueba WhatsApp.
Ver [catálogo](docs/MENU_CATALOG.md).

## 2026-11-18 — Uso de IA limitado por licencias de sus titulares (alpha)

`matter` cambia a un **cliente del futuro broker Vault-Matter**: ya
no recibe claves de Claude, Venice, OpenAI, Alya ni repositorios externos.
Las credenciales deben aprovisionarse **exclusivamente en Vault**, cifradas
y mediante autorización explícita de cada titular. El acceso al repositorio
de un tercero no concede uso de su clave ni permiso para compartirla.

La petición exige `policy_required=true`. La respuesta exige `approved`,
`policy_applied`, `grant_id`, revisión positiva, capacidad coincidente
y nonce correspondiente. La ruta Unix y el UID esperado del servicio se
configuran por separado. En ausencia de un broker con comprobaciones reales,
**no se hace ninguna llamada a IA**. Toda licencia deberá incluir sus
límites como instrucciones que el modelo recibe *en cada petición*;
Vault debe imponer las mismas restricciones por código, sin delegar
la autorización al modelo. `matter` no ejecuta código ni administra roles.

Vault-Matter rama `matter`, commit `427b169`, incorpora la plantilla
`vault/ai/AI_READ_FIRST.md`, el esquema de concesiones
`schemas/ai-usage-grant.schema.json` y el contrato cifrado de credenciales.
Se revisaron README principal y del proveedor, y se conservó su
contenido anterior como historial. Consulte `docs/MATTER_AI.md`.
No se promueve a beta mientras falten CI/gateway y prueba real.

## 2026-11-18 — Matter AI: agente de programación por consulta (alpha)

Se reservó el comando `matter` para programación asistida y
**no** para alias de consola. El nuevo módulo Rust
`src/matter_ai.rs` integra los formatos Claude Messages,
Venice Chat Completions y OpenAI Responses en orden
Claude > Venice > OpenAI, con claves/modelos **solo del entorno
del controlador**. No se copió ningún secreto desde
`jbeckforddev/Aura-Bot-2.0`: no se encontró una integración
identificable de Claude/Venice en las rutas de IA verificadas.

`matter ask`, `matter plan` y `matter status` son comandos
del propietario autenticado en su privado, sin acceso a
ejecución ni edición automática de archivos. El proveedor
WhatsApp habilita únicamente el verbo privado y conserva
HMAC/antirreplay; no hay acceso público ni en grupos.
El IPC tiene un plazo de espera especial para peticiones IA.
En ausencia de credenciales, la respuesta es explicativa y
sin red. Sigue pendiente prueba real con API configurada.

La categoría individual de `matter` no da poderes de
administración ni sustituye el sandbox; las otras funciones
solicitadas (`test`, sandbox, sistema, categorías, estilo)
se validarán en commits separados antes de promover `beta`.
Vault-Matter no está conectado de forma productiva:
la lectura autenticada de roles continúa bloqueada.
Detalle: [Matter AI](docs/MATTER_AI.md).
Ambos README revisados, sin añadir créditos al menú.

## 2026-11-18 — Rustfmt del ensayo RichResponse (alpha)

El primer CI del ensayo `d8c3857` detectó solo diferencias
de formato en `src/main.rs` y `tests/rich_transport.rs`
del proveedor antes de compilar los tipos protobuf nuevos.
Se aplicaron los cambios exactos de Rustfmt sin alterar
comandos, la limitación self-chat, flags ni sesiones.
README raíz/proveedor revisados (2/2), CI nuevo pendiente.

## 2026-11-18 — Extensión segura sobre la vista previa Rich (alpha)

Se conservaron los commits paralelos de `alpha` con vista previa
HTML/JSON offline y CI verde (`a6cbb19`). A partir de ellos, la
implementación Rust añade un ensayo **separado**, opt-in y acotado al
**chat propio**: `providers/whatsapp-rust/src/rich_transport.rs`
envuelve el JSON generado por el módulo offline en el protobuf
`AIRichResponseMessage`, sin fingir procedencia de Meta AI ni
usar `botForwardedMessage`. El HTML procede exclusivamente del
menú original escapado; no se importan archivos Go/JS ni juegos.

`MATTER_WA_RICH_HTML=1` permite **intentar** la entrega RichResponse
solo después de una autorización HMAC de `=/>menu` en el self-chat
del propietario. Apagado por defecto. Error de envío -> botones
NativeFlow, luego texto. Si WhatsApp recibe un mensaje que no puede
mostrar, el programa no lo detecta: requiere prueba real. No
declarar HTML compatible sin evidencias. `beta` y `stable`
intactas; `Bot-Matter` sigue bloqueado. Tests de payload, origen,
límites e IDs nuevos; CI del commit aún por verificar. READMEs
revisados íntegramente 2/2.
Ver [evaluación Rich](docs/WHATSAPP_RICH_HTML_COMPAT.md).

## 2026-11-18 — Rustfmt de pruebas Rich HTML (alpha)

El primer CI para `2dfc214` encontró tres diferencias de Rustfmt
en `providers/whatsapp-rust/tests/rich_preview.rs`, antes de compilar
el proveedor. Se aplicaron las posiciones exactas emitidas por
Rustfmt sin alterar casos ni permisos. El motor de vista previa sigue
siendo offline. READMEs revisados 2/2 y actualizados; CI de la
nueva revisión pendiente.

## 2026-11-18 — Análisis de ejemplos Go/JS de HTML enriquecido (alpha)

Se revisó el material `xd.zip` (Go, JavaScript, plantillas HTML y muestras
de pruebas Rich). Sirve para estudiar el esquema `embedded_screens` y
`FOAIDNixelButtonSheets`, pero su modo de envío finge ser un bot Meta AI
(`0@bot`) y **no** se integró ni habilitó. También faltan recursos HTML
de juegos que ese código espera encontrar.

Nuevo módulo Rust `providers/whatsapp-rust/src/rich_preview.rs`: vista previa
HTML **local**, menú original escapado, botones decorativos claramente
deshabilitados, generador JSON offline del esquema experimental y tests
de contenido/seguridad. El proveedor ofrece `--rich-preview-html` y
`--rich-preview-json`, **sin abrir sesión**. La ruta real NativeFlow
continúa como función experimental separada; el HTML del navegador
**no se envía** a WhatsApp ni se declara compatible con iPhone/Web.

Consulta [evaluación de Rich HTML](docs/WHATSAPP_RICH_HTML_COMPAT.md).
Documentación y READMEs auditados 2/2; las ramas beta/stable, secretos y
sesiones WhatsApp permanecen intactos. CI y prueba real pendientes.

## 2026-11-18 — Formato de importación en proveedor (alpha)

El CI de `3178e32` detectó un único orden de imports distinto
del solicitado por Rustfmt en `providers/whatsapp-rust/src/ui.rs`.
Se aplicó la disposición exacta. No hay cambios de API, seguridad,
estilo, botones, HTML ni sesión. El resto de la suite se validará
sobre el nuevo SHA. Ambos README revisados y actualizados (2/2).

## 2026-11-18 — Clippy del proveedor NativeFlow (alpha)

El CI `44379206555` sobre `d6dd6f1` pasó formato, compilación y
las 3 pruebas nuevas de botones/HTML junto con el resto del proveedor.
Solo falló Clippy por una actualización innecesaria de `Body` y un
`match` irrefutable de respuesta NativeFlow. Se retiró la actualización
sobrante y se sustituyó por el `let` deconstructivo tipado.
No cambian IDs, permisos, mensajes, sesiones ni el opt-in. Pendiente
CI del siguiente SHA. README revisados 2/2.

## 2026-11-18 — Rustfmt: corrección de la fase de UI NativeFlow (alpha)

El workflow independiente del proveedor para `d532ef2` se detuvo en
`cargo fmt --check` por formato en `ui.rs`, `main.rs` y
`tests/ui.rs`. Aplicados los cambios exactos de Rustfmt. El
lector `InteractiveResponseMessage` utiliza una coincidencia exhaustiva
sobre el único caso protobuf en vez de un patrón irrefutable con `else`.
No se modifican sesiones ni alcances. Nuevo CI obligatorio.

## 2026-11-18 — Botones nativos experimentales y estilo HTML mínimo (alpha)

- El proveedor independiente Rust ahora tiene un constructor de
  `NativeFlowMessage` con tres botones reales `quick_reply`:
  Estado, Menú, Créditos. El cuerpo conserva **exactamente** el
  diseño original Matterssmith y su crédito en un solo mensaje.
- Las pulsaciones de botón, si el cliente las entrega como
  `InteractiveResponseMessage`, se transforman únicamente en lecturas
  ya autorizadas y vuelven al puente autenticado HMAC y antirreplay.
  Ningún botón ejecuta `bot on/off` o concede rango.
- El HTML de estilo muy breve acepta solo etiquetas conocidas
  (negrita, cursiva, tachado, código, salto de línea) y las convierte
  al formato de texto de WhatsApp. Rechaza scripts, atributos y enlaces
  HTML arbitrarios; **no** envía HTML de navegador ni JavaScript.
- Hasta disponer de prueba real, botones **apagados por defecto**.
  Para probarlos: `export MATTER_WA_NATIVE_BUTTONS=1` antes de
  iniciar `cargo run --locked`. Cuando el envío nativo devuelve error,
  se envía el menú original por texto. Si el cliente ignora un mensaje
  enviado correctamente, puede no mostrarlo: requiere validación humana.
- No se presenta una detección fiable Android/iPhone: los tres botones
  actuales son comunes a todos los clientes. La segmentación por SO y
  botones específicos siguen pendientes. Solo `alpha`; no cambiar
  `beta`/`stable` ni datos de sesión. README revisados: 2/2.

## 2026-11-18 — Regresión del proveedor tras el menú original (alpha)

- GitHub Actions del primer cambio `ce57169`: núcleo Linux en verde
  (rustfmt, check, tests, docs, Clippy y auditoría). El proveedor compila,
  pero un caso de `tests/provider.rs` exigía incorrectamente que el lector
  externo no pudiera firmar `status --global`.
- La nueva interfaz solo permite firmar esa lectura inocua: el controlador
  devuelve "información privada del propietario" y no las políticas reales.
  Actualizada la prueba para confirmar esta separación. Sin nuevos permisos
  de escritura ni modificación de sesiones. Pendiente CI de esta revisión.

## 2026-11-18 — Restitución del menú original y corrección de `status` (alpha)

- Se restituye el mensaje visual aportado por la propietaria: encabezado
  Matterssmith, Base-Matter, 12 comandos independientes, símbolos, marco y
  crédito `Hecho por Lournyx Matterssmith [Lou-Nyx].` al pie del **menú**
  solamente. El renderer evita duplicar marcos o créditos en el mensaje final.
- `=/>status` permanece disponible para cualquier remitente verificado,
  en grupos y privados externos, con el bot apagado. No enciende chats.
- `=/>status --global` responde con la configuración completa del socket
  solo cuando el origen es el propietario HMAC autenticado; funciona desde
  chat propio o grupo verificado. A terceros responde únicamente que la
  información global es privada. Nunca entrega llaves, identidades ni permisos.
- Regresiones sobre menús, autorización, recepción pública y estado global
  añadidas. Requiere CI del nuevo SHA y prueba real de recepción WhatsApp
  antes de considerar la fase lista para promoción. `beta` y `stable` no se
  modifican. README raíz y README del proveedor revisados (2/2).

## 2026-11-18 — Corrección de tres advertencias Clippy en pruebas anti (alpha)

- En `7de9a0d`, el CI Linux aprobó formato y alcanzó Clippy estricto, que rechazó tres comparaciones `assert_eq!(..., false)` en `tests/anti.rs`.
- Sustituidas por `assert!(!...)` sin cambiar reglas de activación, credenciales, decisiones de autorización ni lógica anti. Se conservan las afirmaciones de protecciones apagadas por defecto.
- El proveedor Rust pasó su workflow de verificación en `7de9a0d`. No implica moderación automática ni prueba de una sesión real.
- La nueva revisión aún requiere CI completo Linux/Windows y prueba real de WhatsApp antes de promover a `beta` o `stable`.

## 2026-11-18 — Corrección de CI para antis y créditos (alpha; ejecución pendiente)

- La ejecución de GitHub Actions del commit `6142c6c` detectó formato Rust pendiente en seis archivos, incluido el módulo de protecciones y la plantilla compartida.
- La suite Windows detectó una prueba de `=/>credits` desactualizada. Se mantiene el crédito obligatorio `Hecho por Lournyx Matterssmith [Lou-Nyx].` y la atribución explícita de fundadora y creadora, sin atribuir rangos no verificados a colaboradores.
- Se normalizaron las diferencias publicadas por Rustfmt y se actualizó la expectativa estricta de la prueba. **CI aún no verificado para este cambio**; no se promueve a `beta` ni `stable`.
- El almacenamiento/configuración `anti` está en fase de prueba: la moderación automática no está conectada al flujo real de mensajes. No se afirma funcionamiento operativo de expulsión/borrado ni se habilita por defecto.

## 2026-11-18 — Menú global compacto, créditos explícitos y referencias Aura/YoSoyYo (alpha)

Tras la validación local íntegra del operador en `61b45ee`
(Clippy estricto, suite Base, 21 pruebas del proveedor Rust y auditoría),
se implementó una nueva revisión visual en `alpha`, **aún sin
compilación Debian ni prueba de recepción real de este cambio**.

- La plantilla `templates/matterssmith-control.txt` contiene un solo
  marco corto de control. Ya no imprime el nombre Matterssmith, Base-Matter
  ni un pie de autor en cada mensaje.
- `templates/matterssmith-menu.txt` presenta `=/>` una sola vez como
  prefijo global. Agrupa `status [--global]` y
  `bot [ámbito] on|off|inherit` en una línea por comando, sin categorías
  ni instrucciones duplicadas. Las variantes siguen ejecutándose como
  comandos completos al enviarse a WhatsApp.
- La atribución permanece intacta en fuentes y README. Solo
  `=/>credits` (o `=/>creditos`) muestra los créditos al solicitarlos.
  El controlador autoriza la lectura del propietario incluso con el bot
  apagado; terceros solo pueden consultar cuando su chat está activado.
  Los filtros HMAC de Base y el proveedor Rust permiten únicamente ese
  nuevo comando de lectura; no conceden permisos.
- `ControlTheme::with_template` conserva los campos de personalización
  `{{TITLE}}` y `{{ACCENT}}` cuando un tema los solicita, pero deja
  de obligarlos en la plantilla común. Rechaza un pie automático de
  créditos y sigue exigiendo un marco válido alrededor de `{{BODY}}`.
- Inspección de `jbeckforddev/Aura-Bot-2.0` (lectura): el menú Aura
  utiliza metadatos de miniatura y un mensaje enriquecido; otros
  comandos usan el tipo experimental `FOAHtmlPrimitiveDemoDONOTUSE`.
  El ZIP proporcionado `YoSoyYo-BaseV3.2-ofc.zip` incluye
  `interactiveMessage/nativeFlowMessage` con botones
  `quick_reply/cta_url` y una heurística del SO a partir del ID,
  **no una prueba fiable Android/iPhone**.
  No se copió Node, Baileys, contenido HTML ejecutable ni botones
  experimentales al runtime Rust.
- Pendiente: ejecutar `cargo fmt --all`, Clippy estricto,
  suites completas y prueba de visualización real en Android/iPhone.
  No hay clasificación de SO autenticada en `whatsapp-rust` 0.7.0;
  no anunciar botones separados por plataforma hasta disponer de
  un mecanismo fiable y una recepción confirmada. Sin promoción
  a `beta` ni `stable`.

## 2026-11-18 — Seguimiento de Clippy: segunda referencia posicional (alpha)

Tras actualizar `alpha` hasta `810c445`, la compilación estricta
detectó `E0609` en `tests/whatsapp_native.rs:82`:
`inspect_bot_defaults(...).unwrap().0` ya no es válido porque
el resultado es `BotDefaultPolicies`.

La prueba ahora consulta el campo `socket_powered`. Se revisaron
también los consumidores del snapshot en el controlador y las
pruebas de puente, IPC, estado y activación; no se hallaron otras
referencias posicionales en los archivos revisados. El nuevo commit
aún requiere `cargo clippy --all-targets --locked -- -D warnings`
y el conjunto de pruebas en Debian. No hay cambios en la sesión,
los archivos SQLite ni los comandos WhatsApp.

## 2026-11-18 — Consumidor residual de `BotDefaultPolicies` (alpha)

El operador volvió a ejecutar Clippy estricto y encontró un fallo
de compilación de `tests/whatsapp_bridge.rs:111`: la prueba leía
`.unwrap().0` después de que `inspect_bot_defaults()`
cambiara de tupla a la estructura `BotDefaultPolicies`.

Se sustituyó la referencia por `.socket_powered` y se normalizó
el formato del `assert!`. No cambia el comportamiento del puente,
la política de activación, las sesiones ni la persistencia.
La corrección está publicada exclusivamente en `alpha`;
las pruebas y Clippy de este commit siguen pendientes de nueva
ejecución en Debian.

## 2026-11-18 — Estructura de políticas para Clippy estricto (alpha)

El operador confirmó `cargo test --all-targets --locked` sin errores
en las suites visibles, incluida la conexión WhatsApp, los controles,
el menú temático y los comandos `=/>bot on/off`. El proveedor
independiente pasó **21 pruebas** (1 + 16 + 3 + 1). La fase de
`cargo clippy --all-targets --locked -- -D warnings` se detuvo
por `clippy::type_complexity` en `src/activation.rs`: el método
`inspect_bot_defaults` devolvía una tupla de seis elementos.

Se reemplazó por `BotDefaultPolicies`, con seis campos
explícitos: `socket_powered`, `default_policy`,
`group_policy`, `private_policy`, `channel_policy`
y `community_policy`. `src/control.rs` y la regresión
`tests/live_unified_controls.rs` se ajustaron para consultar
los campos nombrados, sin cambios de almacenamiento ni seguridad.

**Pendiente:** volver a ejecutar Clippy estricto y pruebas en Debian
para este commit, ya que todavía no está verificado. `beta/stable`
permanecen intactas.

## 2026-11-18 — Ingreso WhatsApp nativo: validar tamaño antes de alias retirados (alpha)

El operador confirmó las suites de Base-Matter hasta `tests/whatsapp_native.rs`.
Allí falló `duplicated_expired_and_malformed_events_fail_closed` porque
el test exigía un error para `=/>on` con 4096 espacios anexos, pero el
ingreso lo marcaba `Ignored` **antes de aplicar el límite de longitud**.
Se ajustó `src/whatsapp_native.rs` para validar primero el tamaño y
el identificador del evento, después ignorar `=/>on/off` retirados.

Se añadió una regresión equivalente para `=/>bot on` excesivamente
largo y para garantizar que el rechazo no consuma el ID de replay.
Los comandos retirados siguen bloqueados y la activación única
sigue siendo `=/>bot on/off`. Pruebas de esta corrección todavía
pendientes de ejecución en Debian. No se promueve a `beta` ni `stable`.

## 2026-11-18 — Ajuste de prueba de tema visual (alpha; nueva validación pendiente)

La ejecución Debian del operador en el commit `19b5fc6` aprobó
`cargo check --all-targets --locked` y las pruebas mostradas
hasta `tests/control_style.rs`. El caso
`community_styles_are_independent_and_untrusted_fields_fail_closed`
falló porque verificaba la presencia textual de `community-one`,
un identificador interno de bot ya excluido del menú.
No es un fallo de la plantilla visual ni motivo para mostrar
identificadores ocultos dentro del diseño.

Se actualizó **solo la prueba** para exigir los elementos
presentados al usuario (`*Register* · ✦` frente a
`*Base* · *`) y verificar que la misma plantilla de grupo
también se aplica a `status`, sin contaminación entre ámbitos.
La suite completa posterior a `control_style` y el proveedor
todavía **no se verificaron en esa ejecución**. Las modificaciones
de formato locales del operador deben conservarse antes de
actualizar Git: no borrar ni sobrescribir archivos de sesión,
y mantener `providers/whatsapp-rust/Cargo.lock` sin seguimiento.

### Seguridad del renderizador y avisos

La plantilla debe conservar el marcador del cuerpo y un marco válido; desde esta revisión el crédito no se imprime automáticamente, solo con `=/>credits`.
Las respuestas, incluso si parecen ya formateadas, se someten
a límite de tamaño y rechazo de caracteres de control.
Los avisos comunitarios y canales siguen deshabilitados en
la admisión real salvo identificación explícita y verificada
de la plataforma. La política `community` no activa
automáticamente un grupo `@g.us` ordinario.

## 2026-11-18 — Selección de plantilla por chat para todos los comandos (alpha; pruebas Debian pendientes)

Se corrigió la selección de diseño: `ControlThemes` ahora ofrece
`decorate(verified_group, response)` y el IPC selecciona la plantilla
de cada grupo **solo después de validar el HMAC** de la petición.
Se comparte el mismo registro de estilos entre `menu`, `status`,
`bot on/off/inherit`, auditoría y demás respuestas de control.

El proveedor Rust conserva las respuestas que el IPC ya formateó,
incluidas las plantillas personalizadas: únicamente aplica el
diseño predeterminado a sus respuestas propias de REGISTER, recuperación
o acuses sin contenido del IPC. Las plantillas siguen almacenadas en
`templates/matterssmith-control.txt` y
`templates/matterssmith-menu.txt`; las credenciales y roles no
se copian al registro de temas.

Nueva regresión en `tests/whatsapp_theme_scopes.rs`: verifica que
una plantilla de grupo distinta se utilice en estado y cambios de
activación sin doble encabezado ni texto literal `\\n`.
**No se ha ejecutado todavía Cargo en Debian para estos commits.**
No hay promoción a beta ni stable.

## Control global Matterssmith — contrato vigente (alpha; pruebas posteriores pendientes)

El comando de activación para WhatsApp es **`=/>bot on/off`**, no
`=/>on/off`. En el chat actual basta **un comando**: al activar,
el estado operativo del socket despierta junto con la política en
una transacción auditada. Desactivar un grupo no afecta otros grupos.

Los alcances adicionales son `=/>bot --global on/off`,
`=/>bot group on/off`, `=/>bot private on/off`,
`=/>bot channel on/off` y `=/>bot community on/off`.
Las dos últimas opciones solo guardan parámetros de tipo: su efecto
real está bloqueado hasta que el proveedor identifique canales y
grupos de avisos comunitarios mediante metadatos autenticados.
`=/>status` consulta siempre el chat actual aunque esté apagado.

**Un único menú corto, sin categorías.** Toda respuesta del controlador
(estado, menú, configuración, auditoría y ayuda) y toda vista previa
directa del proveedor Rust se representan mediante el renderizador
compartido `src/control_style.rs`. El diseño predeterminado se configura
en archivos independientes:
`templates/matterssmith-control.txt` y
`templates/matterssmith-menu.txt`. Usan saltos de línea reales
y conservan los créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
La API de temas admite reemplazos validados por comunidad;
su configuración persistente vía comando todavía no está terminada.

**Estado real:** el operador confirmó anteriormente todas las pruebas
locales y 20 pruebas del proveedor sobre `dfd9dab` (antes de esta
corrección visual y de activación). Estas nuevas modificaciones en
alpha **requieren volver a pasar Cargo, rustfmt y WhatsApp real**.
Vault-Matter todavía no autoriza rangos en producción. No se han
promovido estos cambios a beta ni a stable.

## Project status

Base-Matter es una **biblioteca de desarrollo incompleta**. The current repository
contains a runtime/context foundation, in-memory event and service
registries, translation lookup, a persistent bot identity registry, an
optionally persistent shared-user registry with explicitly authorized external
identity links and bot-scoped user data, a persistent legal-consent store, and
cryptographic key/envelope and provider-authentication components, a command
router, and persistent socket activation policies. The bot
capabilities are only partially implemented: a separate Rust WhatsApp
provider and a loopback HMAC control process exist in beta with offline tests,
but actual phone pairing, live revocation, production Vault, plugin isolation,
Telegram and Discord adapters are not release-qualified. See
[the development status](docs/DEVELOPMENT_STATUS.md) for the
verified stage assessment.

## 2026-11-18 — Prueba grupal: status es lectura, incluso con socket apagado

El operador confirmó en Debian que las **6 pruebas de
`tests/control_ipc.rs`** pasaron tras separar correctamente
`kind:self` de `kind:private`. La suite principal avanzó
hasta `tests/group_control.rs`, con 1 prueba aprobada y 1
regresión antigua: el test todavía esperaba un error al enviar
`=/>status` desde un grupo con el propietario autenticado.

El contrato vigente exige lo contrario: **status del chat actual
siempre responde**, independientemente del estado operativo del
socket y del bot. Se actualizó el test para comprobar
`DispatchResult::GroupStatus { enabled: false,
source: PoweredOff }`, manteniendo **el rechazo de status
en el dispatcher administrativo para actores ajenos**.
Los usuarios externos tienen su propia ruta pública,
firmada, limitada a consultas y sin autoridad del dueño.

No se ha alterado el funcionamiento del controlador.
`tests/group_control.rs` y la suite completa todavía
requieren verificación en Debian para este nuevo commit;
`beta` y `stable` siguen sin modificaciones.

## 2026-11-18 — Corrección de prueba IPC: privado externo ≠ chat propio

La verificación Debian del commit `46ffe7d` completó `cargo check
--all-targets --locked` y la suite independiente del proveedor
WhatsApp Rust (20 pruebas aprobadas). La suite de Base se detuvo en
`tests/control_ipc.rs::disabled_chat_menu_is_silent_but_status_and_reactivation_work`:
el caso esperaba `Ignored` tras `=/>bot private off`, pero el
controlador devolvió correctamente `Applied` al mostrar `=/>menu`
en el **chat propio del propietario**. La política `kind:private`
rige únicamente privados **externos** y nunca desactiva `kind:self`.

Se corrigió la regresión para exigir el comportamiento real:
`bot private off` no afecta al chat propio, mientras
`bot off` ejecutado en él sí silencia el menú;
`status` sigue respondiendo y `bot inherit` restaura
su política heredada. Ninguna validación de permisos, almacén
de Vault, sesión o proveedor se relajó. Reejecutar primero
`cargo test --test control_ipc --locked` y después toda la
suite; **el resultado de esta revisión aún no está confirmado**
en Debian.

## Protección predeterminada de chats externos (alpha)

Los grupos, canales y chats privados externos quedan **apagados por
defecto**. Un `=/>bot --global on` NO activa automáticamente todos
los grupos ni conversaciones de otras personas. Cada tipo debe
habilitarse expresamente con `=/>bot group on` o
`=/>bot private on`, o mediante una excepción de grupo autenticada.
El chat propio del dueño está aislado de los valores `private`.
Una política de grupo con `inherit` vuelve al tipo de chat; si
el tipo está sin habilitar, prevalece apagado.

`=/>status --global` presenta las políticas ausentes para tipos
externos como "apagado por defecto", no como un permiso implícito.
Los rangos siguen perteneciendo exclusivamente a Vault-Matter;
estas políticas de encendido no son asignaciones de roles.

## 2026-11-18 — Consultas públicas verificadas, sin control administrativo (alpha; ensayo Debian pendiente)

Se amplió el puente HMAC Rust para aceptar **solo lecturas públicas**
`=/>status`, `=/>estado`, `=/>menu`, `=/>menú <categoría>`
y `=/>bot` (sin argumentos) desde participantes de grupos o
chats privados externos. La firma identifica al remitente real
(`@lid` o `@s.whatsapp.net`) obtenido del evento de WhatsApp;
nunca sustituye esa identidad por el propietario global.
El despachador usa una ruta independiente que no tiene acceso
a mutaciones, roles, extensiones arbitrarias ni ejecución de bots.

Las consultas `status` funcionan aunque el bot esté apagado;
`menu/bot` solo responden si la política efectiva del chat
lo habilita. Se añadieron regresiones para replay, firma
alterada, intento de `=/>on` y cambios de rol desde tercero.
Los estados `private` controlan únicamente privados **externos**;
el chat del propietario tiene su propio estado `self`.

**No comprobado todavía en un grupo real con otra cuenta:** la
llegada y respuesta de estos mensajes en el proveedor Rust. El
soporte de canales y las delegaciones con Vault-Matter continúan
pendientes. No se cambió el control exclusivo del propietario ni
se habilitó Bot-Matter. Revisión: `alpha`, sin promoción.

## 2026-11-18 — Control de bot por contexto (alpha, pruebas pendientes)

**Actualización de la prueba real:** en WhatsApp, el dueño observó
`=/>help` y `=/>group on/status` responder dentro de un grupo real.
`PoweredOff` se observó porque `=/>off` había apagado el socket:
activar un grupo nunca enciende automáticamente ese socket.

Este checkpoint cambia el control de acceso, sin iniciar Bot-Matter:

- `=/>status` y `=/>estado`: estado efectivo del chat actual,
  disponible aunque el bot esté deshabilitado;
  `=/>status --global`: configuración del socket y valores
  predeterminados de grupos, privados y canales, incluso si el
  socket operativo está apagado.
- `=/>bot on/off/inherit`: excepción del chat actual;
  `=/>bot group/private/channel on/off/inherit`: valores por tipo
  (configuración del dueño en privado);
  `=/>bot --global on/off/inherit` o `=/>bot on/off --global`:
  predeterminado para los chats del mismo socket.
  El encendido del socket sigue siendo `=/>on/off`, separado.
- `=/>menu` y `=/>menú`: nuevo tema independiente Matterssmith
  con categorías. Se muestra en contextos habilitados. Los
  comandos normales y el menú no responden cuando están
  deshabilitados. `=/>help` y `=/>h` se retiran.
- `=/>set`: explicación de configuración. `=/>set role` solo
  explica la autoridad de Vault, sin crear asignaciones locales.
  El modificador final `--debug` muestra detalles técnicos
  depurados al propietario sin cambiar permisos.
- El `ControlDispatcher` genérico conserva sus extensiones
  owner-only de antes, aun con socket apagado. La política de
  silencio se aplica exclusivamente al controlador de chat
  WhatsApp mediante una opción explícita de construcción.

**Alcance real de alpha:** el puente de WhatsApp sigue aceptando
únicamente eventos propios del dueño verificados por PN/LID.
No afirma que los mensajes de terceros, chats privados externos
o canales ya puedan usar `status/menu`: requieren una nueva
versión autenticada de ingreso con identidad individual.
`=/>set style/theme`, importación de `#menu` ajeno, selección
automática por IA y `self/selfmod/mod` con rangos delegados
siguen **pendientes**, sin permisos implícitos de Vault.
El `Cargo.lock` separado del proveedor debe revisarse para una
compilación reproducible; no borrar el archivo de sesión.
Ver [contrato de política por chat](docs/BOT_POLICY_V2.md).

## 2026-11-18 — Verificación del proveedor WhatsApp Rust

Las pruebas generales de `cargo test --all-targets --locked` de Base-Matter
pasaron en el servidor Debian del operador, incluyendo las pruebas nuevas
de grupos, roles, control HMAC y denegación sin Vault. La suite separada del
proveedor `providers/whatsapp-rust` no se completó: en
`tests/recovery.rs` persistía un `env!(CARGO_BIN_EXE_...)` que impide
compilar la prueba cuando Cargo no proporciona esa variable durante
`cargo check` / `cargo test --all-targets`. Se aplicó el mismo patrón
`option_env!` con alternativa de variable de entorno de ejecución usado
en `tests/provider.rs`, conservando la prueba real del ejecutable de
`--recovery-verify`. También se ajustó una prueba de alias LID que
todavía esperaba `=/>menu` en chat privado; la configuración privada
solo admite `=/>status` y otros controles de socket, mientras el menú
pertenece a grupos. El resultado de la suite del proveedor para estos
cambios todavía requiere ejecución en Debian.

## Vault-Matter: única autoridad de roles (decisión actual)

**Cambio de arquitectura:** los rangos del socket/grupo NO pertenecen a
Base-Matter ni a su SQLite local. La autoridad exclusiva es
`Vault-Matter`; Base-Matter realiza únicamente consultas autenticadas,
de lectura, asociadas a la identidad verificada del actor y al ámbito
exacto. Si Vault no confirma un permiso vigente, el permiso se deniega.

WhatsApp, Telegram y Discord pueden ser puertas de administración
de Vault, pero no almacenes ni autoridades independientes. Solo el
servicio de Vault valida la orden, registra la operación, cifra el
estado y publica en su repositorio privado usando permisos de escritura
propios. Base nunca posee una credencial GitHub de escritura de Vault.
El rango `user` es implícito. Roles de proyecto, bots, socket y grupo
son espacios de autorización separados.

**Alpha operativo:** se deshabilitó la instalación del antiguo
`RoleStore` local en el controlador WhatsApp. Los comandos de
asignación de rango siguen **bloqueados** hasta disponer de un lector
autenticado de Vault; no hay escrituras locales alternativas. El
control real del propietario sobre encendido y política de grupos se
mantiene separado de los rangos delegados. Ver
[contrato Vault como única autoridad](docs/VAULT_ROLE_AUTHORITY.md).

## Archivo histórico — Alcances anteriores privado/grupo (superados por BOT_POLICY_V2)

A successful real-device self-chat `=/>status` followed by `=/>on`,
`=/>help` and `=/>menu` confirmed that PN/LID owner identity and the
HMAC response round trip work. The previous behavior incorrectly exposed
group-oriented menus in the private self-chat and ignored every group.

- **Private self-chat** is reserved for owner-authorized **socket-wide**
  settings: `=/>status`, `=/>on`, `=/>off`,
  `=/>set role <PN/LID> <rank>`, `=/>role <PN/LID> <rank>`,
  `=/>unset role <PN/LID> <rank>`, `=/>roles`, `=/>audit`.
  It no longer executes `menu/help/start/add/add++`.
- **Authenticated group chats** handle group-scoped owner operations:
  `=/>group status`, `=/>group on`, `=/>group off`,
  `=/>group inherit`, `=/>group role <PN/LID> <rank>`,
  `=/>menu`, `=/>help`, `=/>h`; the existing `start`, `add`,
  `add++` remain read-only/previews or explicitly constrained actions.
  Spanish aliases `=/>grupo encender/apagar/heredar/estado/rango`
  are supported.
- Group command ingress requires an outgoing authenticated event from the
  paired socket owner, a real `@g.us` chat, verified PN/LID sender and
  an HMAC-signed group JID. The controller derives the group scope from
  that signed JID, not user-supplied text. Third-party group commands,
  standard bot handlers and automatic delegate execution remain blocked.
  A socket must be powered separately from any group policy.
- **Socket ranks** are independent of project owner/co-owner/staff
  and bot-level grants. Test ranks are `co-owner`, `admin`,
  `moderator`, `operator`, `trusted`, `member`,
  `restricted`, `banned`. The real socket `owner` is not
  assignable using a role command. `user` is implicit and never stored
  as an explicit grant. Only the verified socket owner can change grants.
  In alpha these are locally durable SQLite grants and do **not**
  yet confer group command delegation or bot execution.
- **Vault-Matter remains incomplete.** The private
  `Matterssmith/Vault-Matter` repository contains an encrypted-envelope
  schema and metadata. Base has `EncryptedControlOutbox` and
  `RoleStore::export_snapshot` for authenticated encryption, but no
  installed production key provider, GitHub App writer, remote CAS/
  recovery, or reliable encrypted role synchronization. No plaintext
  assignments, WhatsApp sessions, master keys or GitHub credentials may
  be pushed to any repository. A successful local role change is not
  proof that the remote Vault received it.
- Earlier requested `bots`, `bot use`, `bot default`,
  `group use`, group bot inheritance, `config set/get` and
  delegated administration require separately implemented and verified
  bot registry, group binding and Vault transport; they are **not**
  advertised as functioning by this alpha checkpoint.

Promote this subsystem from `alpha` to `beta`, then `stable`,
only after relevant CI and live group/owner regression tests pass.
A live WhatsApp connection is not proof of production safety.

## Single-terminal WhatsApp development trial (alpha)

### Authenticated self-chat PN/LID owner mapping (alpha)

The WhatsApp Rust 0.7 API exposes the linked client's own PN/LID values
through `client.pn()` and `client.lid()`. The provider now compares its
configured pairing phone against the authenticated PN, checks that the
self-chat sender and destination match that same account in either namespace,
and signs a canonical owner frame for the HMAC controller. A foreign chat,
incoming message, group, or unverified identity remains denied. Devices
may add suffixes such as `:device`, which are stripped using the upstream
`Jid::to_non_ad_string()` API. No new keys or SQLite session are generated.

To test a linked alpha session after updating the repo, stop the current
foreground `cargo run` with Ctrl+C, use `git pull --ff-only origin alpha`,
then `cargo run` again. From the authenticated phone's "message yourself"
chat, send `=/>status` and `=/>menu`. The first live response still needs
verification in Debian; offline unit tests do not establish delivery.

### Low-disk single-clone development

Use one Git checkout instead of cloning alpha, beta and stable into separate
directories. Switch branches only after stopping live provider/controller
processes and inspecting `git status`. For a shared build cache, export an
**absolute** `CARGO_TARGET_DIR` such as
`$HOME/.cache/matterssmith/target` before running Cargo. The alpha launcher
honors this variable for both Rust binaries. `CARGO_HOME` already shares crate
downloads by default; no extra Cargo installation per branch is necessary.
For restricted disks, `CARGO_INCREMENTAL=0`,
`CARGO_PROFILE_DEV_DEBUG=0` and `CARGO_PROFILE_TEST_DEBUG=0` reduce build
artifacts, at the expense of debugging information. Different profiles or
dependencies can still generate distinct compilation products.

Sessions remain independently stored under
`~/.local/share/base-matter/branches/{alpha,beta,stable}/whatsapp-test/`;
they never belong in the shared Cargo target directory. The alpha launcher
currently manages a branch-isolated session; beta and stable do not yet have
this launcher or a qualified live WhatsApp provider. Do not treat their
offline Rust tests as live WhatsApp pairing tests. A removed private session
cannot be reconstructed from GitHub.


The original WhatsApp Rust pairing implementation was recovered from
`Matter-Base-matter-1.zip` and historical `src/providers/whatsapp/mod.rs`.
That reference had one-process startup and a temporary `#ping` reply.
The production control prefix remains `=/>`; `#ping` is not restored.

On the `alpha` checkout, `cargo run` now starts a local Rust
controller and the separate Rust WhatsApp provider in one terminal,
without merging their incompatible SQLite dependency graphs.
If no branch-specific configuration exists, it requests a phone number
once and generates private credentials. Each branch is reserved its own
configuration/session path under
`~/.local/share/base-matter/branches/<branch>/whatsapp-test/`
and a distinct loopback port. Only this `alpha` checkpoint currently
contains the launcher; do not assume `beta` or `stable` do.

The initial `@s.whatsapp.net` owner identity is provisional: WhatsApp
may supply a linked-device `@lid` identity. The provider emits a generic
rejection diagnostic without leaking JIDs, and authorization continues
to fail closed until a verified mapping is configured. No existing
session is overwritten or erased. Live message receipt and reconnect
testing are still required prior to beta promotion.

## Responsibilities and boundaries

Matter-Base is intended to host reusable Rust capabilities consumed through
public APIs. It must remain independent of Matter-Bot and platform adapters.
This repository includes a separate `whatsapp-rust` provider executable and
a loopback-only Rust control receiver (both development/beta, not live verified).
The core remains independent of provider-specific libraries. Telegram and
Discord connection drivers are not implemented. Plugin contracts are present,
but the real execution sandbox is not. Scoped role authorization is opt-in;
full provider admission and audited cross-store lifecycle are not yet complete.

Bot registration requires an owner ID. Consumers must derive actor IDs from
authenticated requests before calling owner-management APIs. Owners can manage
co-owners and transfer/delete bots; co-owners can manage bot-scoped operations
but cannot change ownership or delete. Each bot supports at most 256
co-owners; additions beyond the limit are rejected in memory and SQLite.
Legacy unowned records require an
injected `OwnerRecoveryAuthorizer` before ownership can be claimed. Ownership
changes are written to the registry audit history.
Reading a bot's audit history requires the current owner ID and is checked by
the identity store when records are read. Access to the retained audit history
of a deleted bot is not exposed through the current registry API.

The security module defines provider and key-provider contracts. It does not
ship a production Matter-Vault integration or store platform sessions or
credentials. The tests use local test providers and do not verify a live Vault
service. [Repository contract evidence](docs/vault-integration.md) is available
separately and does not establish a live integration.

## Implemented components

| Component | Current behavior | State |
| --- | --- | --- |
| `core::config` | Default application name, environment, debug flag, and locale | Partial; no validation/loading |
| `core::runtime` | Opens legal storage, builds a context, owns module initialize/start/shutdown lifecycle; can inject bot and user registries | Partial; default registries are in memory, no built-in modules are registered |
| `core::context` | Shares event bus, typed service registry, legal store, bot registry, and user registry | Implemented at this scope |
| `bots` | Family/release/instance identity, unique display names, in-memory or SQLite-backed registration and recovery with atomic schema versioning | Implemented owner/co-owner enforcement and audited ownership changes; general roles remain pending |
| `users` | Global application user IDs, explicit unique external identity links, and per-bot/user key-value data in memory or SQLite | Implemented with an injected `UserAuthorizer`; the application authenticates principals and enforces ownership and bot scope |
| `commands` | Prefixes, canonical aliases, authorization, middleware and handlers | Optional SQLite attempt/outcome audit; socket gating via ControlDispatcher; no platform parsing |
| `activation` | SQLite owner-scoped power, group/socket overrides, persisted global fallback and atomic audit | Trusted provisioning, durable bot binding, owner transfer/recovery; no network connections |
| `events` | Synchronous named callbacks; listeners are copied before invocation so callbacks run outside the registry lock | Basic; in-memory and not durable; a callback panic propagates from `publish`; `publish_checked` catches unwinding panics per listener, continues dispatch, and returns an error |
| `services` | In-memory type-indexed service registration and retrieval | Basic; no lifecycle management |
| `i18n` | In-memory translation catalogs and key fallback | Basic; no bundled catalogs or locale validation |
| `legal` | SQLite documents, user IDs, immutable decision history, and access checks | Implemented at this scope |
| `security` | Authenticated encryption envelopes, key management, provider identity/policy, replay-aware assertions, binding contracts | Implemented contracts/primitives; external integration unverified |
| `modules` | `CoreModule` plus ordered initialize/start and reverse stop with failure cleanup; unwinding callback panics become errors | Implemented and coordinated by `MatterRuntime`; consumers register their own modules |

These states describe this repository's present behavior, not the full planned
Matter-Base platform.

## Repository layout

```text
src/
  activation.rs  Persistent socket activation policy and owner checks
  command_audit.rs  Persistent command attempts/outcomes and owner-scoped history
  commands.rs    Universal command routing
  control.rs     Control-prefix interception and socket-gated dispatch
  control_ipc.rs Loopback-only authenticated control service
  whatsapp_bridge.rs Signed WhatsApp owner-control bridge
  platforms.rs   Shared platform driver/transport registries
  bots/       Bot identity contracts and in-memory/SQLite registry
  users/      Shared users, explicit external identity links, scoped user data
  core/       Configuration, shared context, logging, runtime
  events/     In-memory event bus
  i18n/       Translation catalog lookup
  legal/      SQLite legal-document and consent store
  modules/    CoreModule contract
  security/   Encryption, key management, provider authentication/contracts
  services/   Typed in-memory service registry
tests/        Integration-style tests for the implemented contracts
docs/         Architecture and continuity documentation
providers/whatsapp-rust/  Standalone Rust pairing-code provider
```

There are two tracked README files: this root `README.md` and
[`providers/whatsapp-rust/README.md`](providers/whatsapp-rust/README.md).
Both require review before every development commit.

Clippy's first strict CI run identified three pre-existing lint failures in legal-document row typing, ciphertext size arithmetic, and user schema-range validation. The follow-up changes preserve public APIs and require a passing CI run before they are considered verified.

## Persistent roles and resource authorization

The optional role store, in [src/authorization.rs](src/authorization.rs),
provides SQLite-backed bot-scope role definitions, single-parent inheritance,
owner-only administration, bounded memberships and rules, explicit deny
precedence, default denial, and immutable audit records. Provisioning an
owner/scope pair is a **trusted application-only operation**. Actor IDs must
come from authenticated ingress; passing a claimed ID is not authentication.
Scope administration grants no implicit command execution, socket power or
access to another scope.

[RoleCommandAuthorizer](src/authorization.rs) can be injected into
CommandRouter; it checks the canonical name as resource
`command:<name>` with action `execute`. A successful authorization is
only a decision at admission time. Callers must separately enforce the
socket/activation gate, consent, and any platform security policy. The store
does not persist sessions, platform credentials or encryption keys.

## Session metadata and transport boundary

The [session module](src/sessions.rs) provides a SQLite-backed lifecycle for
socket connection **metadata**: owner, platform, connection state, optimistic
revision and immutable audit. It does not store login tokens, session files,
private platform credentials, WhatsApp keys or a live network connection.
A trusted application provisions session metadata and drives valid lifecycle
transitions. Socket power and bot activation are independent.

MessageService accepts a required application MessageAdmission policy and
a TransportDriver supplied by a platform adapter. Its sender refuses
disconnected sessions, malformed/oversized messages and denied requests.
The application must enforce authenticated actor, consent, socket activation,
resource scope and group policy; this library does not infer those permissions.
A driver acknowledgement is not a delivery guarantee, and in-flight sends
can race with connection changes. No automatic retry or exactly-once
semantics are provided. A built-in ControlPlaneMessageAdmission can combine the persisted socket
activation/bot binding snapshot with role rules and an additional mandatory
application policy. Group destinations must be explicitly identified using
OutboundMessage.group_id; private destinations use None. This still does not
make separate stores or network effects atomic.
See [session contracts](docs/sessions.md).

## Reserved role-control commands

With ControlDispatcher::with_roles, =, /> and =/> support "set role
USER_ID ROLE_ID" and "unset role USER_ID ROLE_ID" for the bot bound to the
receiving socket. The socket owner must remain the bot and role-scope owner.
Role controls work even when the socket is off; the ordinary router cannot
intercept them. Missing role configuration and foreign owners are denied.
The host must serialize bot transfer with cross-store role administration.


## Encrypted control-plane synchronization

EncryptedControlOutbox holds only authenticated encrypted envelopes and a keyed
change fingerprint. It can export a consistent, authorized RoleStore snapshot,
encrypt it with the active KeyManager key and queue it durably. An application
must install a trusted EncryptedEnvelopeSink (for example a correctly scoped
GitHub App writing to the private Matter-Vault repository). It MUST use
compare-and-swap revisions, deduplicate envelope IDs and never write plaintext.
The same content under the same key is not queued again; key rotation queues
fresh ciphertext. Failed publication keeps the encrypted record pending.

This is **not** a live GitHub/Matter-Vault adapter. No GitHub credential,
private key, master key or external session is stored here. An optional EncryptedEnvelopeReader supports lost-acknowledgment recovery:
a pending record is marked published only if the returned remote envelope
matches every field of the locally queued ciphertext. Conflicting remote
records remain pending; they are never overwritten automatically. Multiple
concurrent publishers, full restore/import and actual authenticated GitHub
commits remain integration requirements. See
[Vault integration](docs/vault-integration.md).

## Trusted plugin manifests and sandbox contracts

The [plugin host](src/plugins.rs) accepts bounded JSON manifests for
Rust/C++/Go/Python/Node workers. It rejects path traversal, unsupported
runtimes, duplicate IDs/commands and oversized input. Installation requires
an explicit PluginTrust provider to authenticate publishers; execution checks
the policy again and goes through an injected PluginSandbox. Without an
external sandbox, DenyPluginSandbox refuses all plugin execution. The host
does **not** execute an entrypoint directly or expose system credentials.

Loading manifests from a directory prevalidates the batch, but registration
into the command router is not atomic against concurrent installations.
No OS-level sandbox backend, signed artifact verification service or live
cross-language process is supplied. No in-process mock should be treated as
real isolation. See [plugin security contract](docs/plugins.md).

## External connection lifecycle

ConnectionService orchestrates an authenticated session owner, an
application-provided ConnectionAdmission policy, a SessionCredentialSource
returning zeroizing secret bytes and a separate ConnectionDriver implemented
by a platform adapter. Failures fault the session metadata and are not retried
automatically. The service exposes guarded connect/disconnect, live health
checks and explicit recovery for uncertain states. **No real WhatsApp,
Telegram or Discord adapter is bundled.**

Credential bytes are transient and never persisted in Matter-Base; the
external source is responsible for authentication, storage, rotation and
retrieval. A connected database state cannot prove the external provider is
online. Network operations across processes need provider-side generation
fencing and revocation. See [connections and sessions](docs/sessions.md).


## Readiness gates, licensing and tracked-file hygiene

Matter-Base is not yet a production-ready release. The exact remaining
requirements are maintained in [release readiness](docs/RELEASE_READINESS.md).
The [MIT license](LICENSE) is included for this repository's own code;
third-party provider imports must retain their original notices and licenses.
The beta CI also verifies all tracked README links/credits, rejects
sensitive private-key/session filenames and runs a second Rust job on
Windows. Passing Linux and Windows CI is not a Termux or provider test. Windows
also checks that SQLite-backed test files are closed before cleanup.


## Authenticated platform ingress

InboundGateway receives untrusted provider text/events but does not accept
claimed actor IDs from those messages. A required external InboundAuthenticator
verifies the provider connection/webhook and derives the authenticated socket,
sender, source and group. A separate InboundAdmission enforces legal consent
and platform policy before routing to ControlDispatcher. A SQLite-backed,
HMAC-keyed InboundReplayStore refuses duplicate event IDs across connections
and keeps only encrypted-style keyed fingerprints, never raw provider IDs.

Replay claims occur before handler effects. A crash after claiming can leave
an uncertain result; blind retry is prohibited. Replay binding keys must be
provisioned externally and shared across processes; key rotation and pruning
require coordinated retention. **No live WhatsApp/Telegram/Discord identity
authenticator ships with Base.** See [inbound contracts](docs/ingress.md).

## Vinculación WhatsApp y control global — contrato corregido

Se reconoce la prueba anterior de conexión a WhatsApp y `ping` en el
desarrollo histórico. La rama publicada actualmente no contiene el antiguo
adaptador ejecutable; conservar su historial no equivale a comprobar una
conexión real hoy.

`PairingCodeService` añade un contrato en Base-Matter para **solicitar
únicamente un código de vinculación**, con autorización del propietario,
número internacional de 8–15 dígitos, código temporal redactado y sin
persistencia en SQLite. **No implementa QR, no registra un comando ping y
no abre por sí mismo una conexión WhatsApp.** El proveedor Rust independiente está implementado en
`providers/whatsapp-rust`, pero la conexión real y recuperación en teléfono
siguen pendientes de validación; no se deben borrar sesiones.
No se modifica ni se inicia Bot-Matter.

`ControlDispatcher` procesa el prefijo global canónico `=/>` antes del
router del bot. Los prefijos `=` y `/>` permanecen como alias de
compatibilidad. Ejemplos: `=/>on`, `=/>off`,
`=/>set role USER_ID ROLE_ID`. El dueño del socket sigue siendo el único
autorizado para manipularlo, y el control funciona con el bot apagado.
No se transfiere poder al propietario de otros sockets.

Todos los repositorios de la organización pertenecen al ecosistema Matter;
el futuro `Bot-Matter` consumirá contratos de sus componentes **solo
cuando Base-Matter termine**. Véase [pairing code](docs/PAIRING_CODE.md).

## WhatsApp owner control — authenticated bridge v1 (beta)

The opt-in `WhatsAppControlBridge` validates a short-lived HMAC-SHA256
provider frame bound to a **specific socket ID, owner ID, owner JID, private
chat, event ID and message body**. It requires the WhatsApp Web provider to
authenticate the original `fromMe` event. Only canonical `=\/>` control
verbs are admitted; the old ping and bot routing are unreachable.

The library accepts a signed JSON frame and dispatches through
ControlDispatcher. The separate `whatsapp-matter-control` Rust executable
listens exclusively on 127.0.0.1, while the independent
`providers/whatsapp-rust` process owns the platform session.
Neither process by itself proves a live WhatsApp connection.
The transport must keep its bridge key local, reject group/nonowner messages
and use the shared replay guard. The bridge rejects
signatures, timestamps beyond two minutes, duplicate event IDs and attempts
to control another owner's socket. The protocol is documented in
[WhatsApp bridge v1](docs/WHATSAPP_BRIDGE.md).


## Continuous verification

GitHub Actions workflow [Rust verification](.github/workflows/rust-ci.yml)
is configured for pushes and pull requests targeting `matter-beta`, and can
also be dispatched manually. It runs formatting, all-target compilation,
unit/integration and documentation tests, Clippy with warnings denied, and a
commit whitespace check. Workflow presence does **not** establish that a run
has passed; inspect the latest run and its logs before treating any commit
as verified. The opt-in external Matter-Vault repository test is not run by
default and still requires a separately available checkout.

## Requirements and build

Install a Rust toolchain with Cargo, then run from the repository root:

```sh
cargo fmt --all
cargo check --all-targets
cargo test
git diff --check
```

The SQLite dependency uses its bundled SQLite feature. No Node.js `npm start`
workflow or JavaScript runtime is required for the Rust library. The legacy
installer scripts remain in the repository but are not the build or test
workflow for the current Cargo crate.

## Minimal library use

```rust,no_run
use matter_base::{CoreConfig, MatterRuntime};

fn main() -> anyhow::Result<()> {
    // Use a path owned by the application. The legal database contains
    // consent records; do not put secrets or platform sessions in it.
    let runtime = MatterRuntime::open(
        CoreConfig::default(),
        "data/legal/registry.db",
    )?;
    runtime.initialize()?;
    runtime.start()?;
    runtime.shutdown()?;
    Ok(())
}
```

The default `MatterRuntime::new` path is `data/legal/registry.db`. Tests and
applications that manage storage separately can provide a `LegalStore` with
`MatterRuntime::with_legal_store`. Use `open_with_bot_registry` with
`BotRegistry::open_sqlite` when bot identities must survive restarts. Use
`open_with_registries` with `UserRegistry::open_sqlite` to persist shared users,
explicit external identity links, and bot-scoped user data. User operations need
a `UserPrincipal` backed by the consuming application's `UserAuthorizer`; never
link platform identities by matching names or IDs automatically. Keep secrets,
platform credentials, and sessions out of the user data store.

## Security and persistence notes

- Legal documents and consent decisions use SQLite; document registration and
  consent validation/writes run in transactions. Decision history is append-only
  through the store's database triggers. Schema initialization and component
  version registration are transactional; unversioned existing tables and their
  data are preserved, failed DDL rolls back, and unsupported future versions
  are rejected without rewriting the database.
- Legal access checks evaluate the user and required documents in one SQLite
  statement, so the result uses one database snapshot.
- Consent writes, legal-user lookups/history, document administration, and
  access checks require a `LegalPrincipal` backed by the consuming
  application's `LegalAuthorizer`. The application must authenticate the
  principal and authorize the requested action/resource; Matter-Base does not
  authenticate platform identities; the shared user directory is provided by `users`.
  Authorization failures are returned as a generic denial without forwarding
  policy-provider diagnostics.
- The standard runtime persists only the legal store. Bot registrations and
  shared users have independent SQLite stores that must be explicitly injected
  to survive restarts. Socket activation policies have a separate SQLite store in `activation`.
  Network connections, configuration, events, and service metadata do not have
  persistence implementations here.
- Shared user data is a generic, bounded text value store. The consuming
  application must reject unauthorized owners and unknown bot instance IDs in
  its `UserAuthorizer`; the user store does not itself query the bot registry.
  Do not put credentials, sessions, secrets, or sensitive personal data in it.
- Envelope encryption validates metadata, uses the existing
  XChaCha20-Poly1305 format, and caps plaintext/ciphertext at 16 MiB before
  encryption or Base64 decoding. Keep compatibility when changing that contract.
- Key-provider and binding-store interfaces are injection points. A live,
  production Vault connection has not been verified by this repository.
- Authenticated key loading rechecks the Vault identity when the key set is
  staged; a mismatch is rejected before replacing the current key manager
  contents. Authenticated key reads also reject a key manager scoped to a
  different Vault.
- `ProviderAuthenticator::new` uses a bounded in-memory replay store.
  Applications can inject `SqliteProviderAssertionReplayStore` through
  `new_with_replay_store` to reject assertion replays across verifier restarts
  and independent database connections. Expired rows are cleaned across scopes;
  both per-scope and global capacity are bounded. It is not a distributed store.
- Authenticated provider assertions are limited to 16 KiB, and a key provider
  must return 1–4,096 key IDs per load. Empty and oversized lists fail closed,
  preserving the current key set.
- The key manager caps its total key count at 4,096 for provider loads and
  manual insertion/rotation alike. Capacity errors leave existing keys intact.
- Errors returned by injected key and binding providers are replaced with
  operation-level messages before they cross the integration boundary; this
  prevents provider-supplied diagnostics from exposing secrets through logs.
- Never add platform credentials, WhatsApp sessions, master keys, or private
  provider material to source, documentation, or logs.

See [architecture](docs/architecture.md), [services](docs/services.md),
[internationalization](docs/i18n.md), and [security module source](src/security/)
for component-level details.

## Bot ownership

```rust,no_run
use matter_base::{BotAction, BotRegistry};

fn register_bot(bots: &BotRegistry, authenticated_owner_id: &str) -> anyhow::Result<()> {
    let bot = bots.register("family-key", "release-key", "matter", authenticated_owner_id)?;
    bots.authorize(bot.instance_id(), authenticated_owner_id, BotAction::Manage)?;
    let _owned_bot = bots.get(bot.instance_id(), authenticated_owner_id)?;
    let _owned_bots = bots.list(authenticated_owner_id)?;
    let _audit = bots.audit_history(bot.instance_id(), authenticated_owner_id)?;
    Ok(())
}
```

Pass an owner ID derived from the application's authenticated request. Do not
accept it directly from an untrusted request field. `get` and `list` also
require that authenticated actor ID and return only bots owned by or shared
with that actor; inaccessible instances are returned as absent. Legacy
unowned records must be claimed through the recovery authorization flow before
they become visible to a regular owner query.

## Universal command routing

`CommandRouter` matches configured prefixes, resolves case-insensitive command
names and aliases, and passes a platform-neutral invocation to an injected
authorization policy, middleware, and handler. Authorization runs before
middleware or handler code. Message size, command count, aliases, middleware,
and argument count are bounded. Arguments are split on whitespace; adapters
must handle platform-specific quoting and message parsing.

`CommandRouter::new_with_audit` injects a `CommandAuditStore` for durable
recognized-command attempts and outcomes. The default `new` constructor remains
unaudited. `ControlDispatcher` adds socket activation checks before normal
command routing; power changes use the separate activation audit.
The injected authorizer must enforce authenticated identity, bot scope,
permissions, and operational activation before dispatch.

```rust,no_run
use anyhow::Result;
use matter_base::{CommandAuthorizer, CommandInvocation, CommandRouter};
use std::sync::Arc;

struct Policy;

impl CommandAuthorizer for Policy {
    fn authorize(&self, invocation: &CommandInvocation) -> Result<()> {
        if invocation.bot_instance_id == "known-bot" {
            Ok(())
        } else {
            anyhow::bail!("command denied")
        }
    }
}

fn make_router() -> Result<CommandRouter> {
    let router = CommandRouter::new(vec!["#".to_owned()], Arc::new(Policy))?;
    router.register("help", &["h".to_owned()], |invocation| {
        Ok(format!("{} {}", invocation.command, invocation.args.join(" ")))
    })?;
    Ok(router)
}
```

For persistent command auditing, create an application-owned SQLite database
and pass `Arc::new(CommandAuditStore::open(path)?)` to
`CommandRouter::new_with_audit(prefixes, authorizer, audit)`. Attempts persist
before authorization/middleware/handlers run. Audit write failures return an
error; an outcome-write failure cannot undo handler effects and leaves a
`Pending` attempt. Never automatically retry based only on that status.
`audit.history(bot_id, authenticated_owner, &bots, after_id, limit)` returns
owner-scoped pages of at most 1,000 records. Arguments, responses, message bodies
and callback diagnostics are omitted. See [command audit contracts](docs/command-audit.md)
for coverage, recovery and storage boundaries.

## Socket activation policies

`activation::SocketActivationStore` persists socket ownership, operational power,
and group/socket overrides with atomic audit writes. New sockets start powered
off. When powered on, resolution uses Group > Socket > trusted global default >
deny. Only the stored owner can change policies or read its audit history.
The application must authenticate provisioning and actors; this store does not
authenticate platform sessions. `bind_bot` establishes a durable bot association;
`ControlDispatcher` intercepts `=on`, `=off` (also `/>` and `=/>`) and gates
normal commands. Explicit socket targets still require the target owner.
Socket power controls work before bot binding and remain available to the socket
owner after bot ownership changes; normal commands still require a valid binding
and current bot authority.

For durable global policy, trusted setup calls `provision_global(admin_id)` and
the authenticated administrator calls `set_global_policy(actor, Some(true))`,
`Some(false)` or `None` (remove fallback). Construct the dispatcher with
`ControlDispatcher::new_with_persistent_policy` to observe committed changes
on subsequent admissions. Global administration grants no socket power rights;
group/socket overrides still take precedence and power-off always denies.
The existing `new(..., global_default)` constructor retains a static fallback.
Socket owners can call `transfer_owner(socket, actor, new_owner)` or
`release_binding(socket, actor)` to hand over a socket or recover a stale bot
association. Both reset power/local overrides and release the binding in the
same transaction as their audit; the new binding and power-on must be explicit.
Normal admission reads owner, binding and policy together to preserve isolation.
See [activation contracts](docs/activation.md).

## Promoción estable y pruebas WhatsApp — 17 nov 2026

La rama `matter` ya recibió el PR #3 con **solo** endurecimiento verificado
de seguridad, compilación y pruebas SQLite/Windows. Su commit
`e509cc4f8c53eafbb1233359bc2ae63e0dde7f48` pasó la
[CI estable](https://github.com/Matterssmith/Matter-Base/actions/runs/44139981479)
en Ubuntu y Windows. No se trasladaron los módulos de integración incompletos.

El proveedor elegido es **whatsapp-rust**, recuperable desde el commit histórico
[3c36d97bb](https://github.com/Matterssmith/Matter-Base/blob/3c36d97bb/src/providers/whatsapp/mod.rs).
La vinculación por código y la prueba anterior de ping pertenecen al historial
del desarrollo Rust. No se han repetido sobre la revisión actual de Base-Matter.

El repositorio Matter-Baileys conserva un experimento separado, **congelado y
no seleccionado** para este proyecto. No se usará para sustituir la integración
Rust ni se desarrollará Bot-Matter antes de completar Base. En la beta actual
están verificados los contratos Rust de pairing y el control global, pero no
un nuevo emparejamiento real de WhatsApp.

## Global socket status without Bot-Matter

The reserved `=/>status [SOCKET_ID]` control now returns the
authenticated socket owner's current policy decision and bound bot identifier
from one consistent database snapshot, even while the bot is powered off.
The response is an application-level `DispatchResult::Status`:
integrations must render it only to the authenticated requesting owner.
It is not a network connection-health probe. It never starts Bot-Matter,
invokes the ordinary command router, or creates a WhatsApp ping handler.
Other users and owners of different sockets are denied.

## Roadmap boundary

Scoped roles, persistent session metadata, guarded messaging contracts and
plugin validation exist and have passed CI. Remaining readiness work includes
complete authenticated Vault synchronization/recovery, externally verified
provider connections, real sandbox isolation, cross-store owner handover
coordination, and full release and security audits. Matter-Bot must not be built on
this crate until those core contracts are implemented, integrated, documented,
and tested.

## Credits

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## Nombres del ecosistema — Matter al final

La convención solicitada es `Componente-Matter`: `Base-Matter`,
`Vault-Matter`, `Bot-Matter`, `WhatsApp-Matter`,
`Telegram-Matter`, `Discord-Matter` y equivalentes. El inventario
de los 22 repositorios y las comprobaciones previas a su renombrado
están en [ecosystem rename](docs/ECOSYSTEM_RENAME.md).

Los identificadores Rust `matter-base` / `matter_base` se mantienen
temporalmente por compatibilidad. La renominación de los repositorios de
GitHub requiere una operación administrativa aparte y no se ha ejecutado.

## Ecosystem names and provider layout

The requested target repositories are **WhatsApp-Matter** (currently
Matter-Baileys) and **Telegram-Matter** (currently Matter-Telegraf).
El protocolo para WhatsApp exige **solo código de vinculación**, sin QR.
El antiguo ping fue una prueba temporal, no el bot global. La rama beta del
repositorio de proveedor conserva un prototipo no apto para este flujo y queda
congelada mientras se termina Base-Matter; no constituye prueba de conexión.
El control global usa `=/>` desde Base, antes de cualquier comando del bot.
Administrative repository renames and full provider imports remain outstanding. Provider methods belong in
separate directories such as `providers/baileys` and `providers/telegraf`, using
verified official upstream revisions and preserved licenses/history. Base and
Vault readiness remains the immediate priority. See [repository instructions](AGENTS.md).

## Proveedor de WhatsApp: Rust exclusivamente

El proveedor previamente probado era **`whatsapp-rust`** con `PairCodeOptions` y SQLite, recuperable desde el commit histórico `3c36d97bb`. Esta es la implementación que se debe continuar, sin sustituirla por Node.js, Baileys o un bot nuevo. La vinculación es exclusivamente por código, sin QR.

El archivo `src/whatsapp_bridge.rs` es una frontera de verificación escrita en Rust; **no es un adaptador Node**, no establece conexión y no debe confundirse con una prueba real de WhatsApp. El antiguo `ping` no forma parte de los comandos permanentes. El control de Base usa `=/>` con las autorizaciones del propietario del socket.

Los códigos deben conservar los créditos exigidos en `AGENTS.md` y sus pruebas deben verificar la separación entre encendido operativo y política de ejecución.

## Extensiones del control global en Rust

Base-Matter contiene ControlExtensionRegistry, que admite comandos adicionales bajo el prefijo canónico =/> y alias declarados por un host Rust confiable. El núcleo comprueba al propietario del socket antes de ejecutar la extensión; los comandos continúan independientes de Bot-Matter y están disponibles incluso con el bot apagado. Los verbos administrativos on/off/status/set/unset y el ping histórico son reservados y no pueden sobrescribirse. Los prefijos antiguos = y /> solo mantienen los controles incorporados.

Los controladores de extensiones deben incluir comprobaciones adicionales de permisos y auditoría donde sus efectos lo requieran. Los fallos no exponen datos internos; los argumentos y resultados están acotados. Consulte docs/CONTROL_EXTENSIONS.md. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## Ingreso nativo de WhatsApp en Rust

El modulo WhatsAppNativeIngress acepta eventos autenticados exclusivamente dentro del proceso de whatsapp-rust, comprueba el JID privado propio, from_me, ausencia de grupos, expiracion y replay persistente antes de ejecutar controles canónicos =/>. No activa Bot-Matter, no incluye QR ni registra un ping permanente. El proveedor externo real, el emparejamiento y las pruebas contra un telefono siguen pendientes. Ver docs/WHATSAPP_NATIVE.md.

## Proveedor Rust ejecutable (pruebas aisladas)

El directorio providers/whatsapp-rust contiene un binario Rust real que integra whatsapp-rust 0.7.0 con la autorización, replay y control de Base-Matter, sin iniciar Bot-Matter, Node ni mostrar QR. Se requiere provisionar el socket desde el host seguro y utilizar un archivo SQLite privado, con bloqueo de sesión. Ver providers/whatsapp-rust/README.md para variables y pruebas. La vinculación de un teléfono real todavía no ha sido validada sobre esta versión, por lo que la promoción live a matter continúa bloqueada.

## Separación de procesos Rust para WhatsApp

El proveedor whatsapp-rust 0.7.0 y el núcleo actual utilizan versiones de SQLite nativas incompatibles. Para evitar alterar la persistencia estable del núcleo, el proveedor corre como proceso Rust independiente y entrega controles firmados HMAC por TCP local (127.0.0.1). El verificador Rust ya implementado en src/whatsapp_bridge.rs autentica esos eventos. No interviene Node. Ver providers/whatsapp-rust/README.md. El receptor TCP aún debe probarse e integrarse antes de su uso estable.

## Receptor Rust de comandos de WhatsApp

El binario de Base-Matter whatsapp-matter-control abre exclusivamente un socket TCP de loopback para eventos WhatsApp firmados por el proceso Rust whatsapp-rust independiente. Primero se ejecuta --init una sola vez para provisionar la identidad del socket; --serve exige dueño existente y claves de transporte/replay diferentes, y rechaza datos no autenticados. No ejecuta Bot-Matter, no admite QR ni ping. Los comandos de prueba on/off/status pasan por el mismo ControlDispatcher con persistencia y auditoria. Ver docs/WHATSAPP_IPC.md. La comunicacion entre procesos necesita pruebas reales con telefono antes de denominarse estable.

La prueba local del puente TCP Rust comprueba una sesión de lazo local: on, repetición del mismo ID y off, sin enviar mensajes a WhatsApp. No sustituye las pruebas con un teléfono real.

## Prueba de configuración del puente Rust

Los dos ejecutables Rust incorporan `--check` para revisar variables de entorno, secretos diferentes, acceso a directorios privados y dirección TCP local antes de abrir una sesión. No crean bases SQLite ni inician WhatsApp en este modo. Las instrucciones de emparejamiento deben seguir siendo exclusivamente por pairing code. No utilizar el antiguo ping ni desarrollar Bot-Matter.

## Prueba real de WhatsApp — dos procesos Rust

Para probar el emparejamiento exclusivamente por código en `matter-beta`, sigue la [guía paso a paso](docs/WHATSAPP_TRIAL.md). Incluye creación de un entorno privado y comandos separados del receptor Base-Matter y del proveedor `whatsapp-rust`. Sin sesión real verificada, no declarar estabilidad ni promover el proveedor a `matter`.

## Contrato común para WhatsApp, Telegram y Discord

La infraestructura de conexiones y mensajes conserva los controles de Base-Matter para todas las plataformas. El [registro de controladores Rust](docs/PLATFORM_CONTRACT.md) dirige cada sesión exclusivamente a su proveedor canónico, sin fallback cruzado, sin compartir credenciales y sin suplantar identidad. Los adaptadores de Telegram-Matter y Discord-Matter permanecen pendientes; este contrato no afirma que estén conectados. WhatsApp continúa usando `whatsapp-rust` y pairing code; la prueba con teléfono y las puertas de estabilidad siguen pendientes.

## Protección de archivos de bloqueo en Rust

Los dos procesos de WhatsApp crean sus archivos de bloqueo de propiedad de sesión con permisos privados `0600` desde la apertura en Unix. Conservan la exclusión por `create_new`, no sustituyen bases SQLite ni recuperan automáticamente locks obsoletos. Requiere CI y evaluación de la sesión real; no cambia el control global `=/>` ni habilita Bot-Matter.

## Límite de recepción del puente WhatsApp

El receptor TCP de controles `=/>` corta los frames de más de 16 KiB y los envíos incompletos o demasiado lentos. El plazo máximo de lectura es 4 segundos por conexión, no un temporizador reiniciado por cada fragmento. Esta es una protección local; no sustituye la verificación de sesión WhatsApp, HMAC ni las pruebas con un teléfono.

## Verificación del resultado del control WhatsApp

El proveedor Rust ahora distingue el acuse de control `applied` del resultado
`duplicate`. Los frames repetidos no se muestran como ejecuciones nuevas;
`ignored`, `denied`, errores de JSON, claves extrañas y acuses incompletos
fallan cerrados. Esto mejora la observabilidad de las pruebas sin alterar
el protocolo HMAC. El proveedor añade posteriormente una confirmación
genérica solo para `applied`, documentada abajo. La conexión real y
la identidad del JID siguen pendientes de prueba.

## Recuperación de procesos WhatsApp

Los archivos de bloqueo de sesión y del controlador usan a partir de este
checkpoint un marcador de formato v2 y el bloqueo exclusivo nativo de archivos
de Rust 1.89 o posterior. El lock no se elimina al apagar; al terminar un
proceso se libera automáticamente el bloqueo del sistema operativo, permitiendo
reiniciar sin borrar la sesión SQLite. Los locks v1 con PID, sin formato v2,
se rechazan hasta que el operador confirme que el proceso anterior se detuvo:
**nunca se retiran ni se sobrescriben automáticamente**. La validación
`--check` verifica un lock v2 sin modificar sesiones ni archivos.

## Prueba real de procesos Rust sin dispositivo

La suite de integración incluye un test que ejecuta el binario controlador
`whatsapp-matter-control` en otro proceso, le envía comandos HMAC firmados
por TCP local, corta bruscamente el proceso y verifica el reinicio con el lock
v2. Un ID de mensaje usado antes de la caída sigue rechazado como replay
gracias a SQLite. La prueba no conecta ni vincula ningún teléfono; la
validación de WhatsApp real continúa pendiente.

## Receptor obligatorio antes de WhatsApp

El arranque del proveedor Rust exige que su controlador Base-Matter esté
escuchando en el loopback configurado antes de abrir SQLite o iniciar el
pairing code. Si no hay conexión TCP, el proveedor cancela el arranque con
un error genérico, sin modificar la sesión. Esta comprobación no autentica
el receptor ni sustituye HMAC: la firma se verifica en cada orden posterior.
`--check` continúa sin conexiones ni cambios de archivos.

## Confirmación privada del control global en WhatsApp

Después de verificar un comando `=/>` del dueño en su propio chat privado,
el proveedor Rust responde únicamente si Base-Matter devuelve `applied`,
con el texto fijo «Base-Matter: solicitud procesada.». Un replay
`duplicate`, un comando no autorizado o un fallo de IPC no generan respuesta.
La respuesta no empieza por ningún prefijo de comando y, por tanto, no se
auto-ejecuta. La función no devuelve todavía información detallada del estado,
y requiere prueba con teléfono y recepción real antes de declararse estable.

## Rama de desarrollo y la comunidad REGISTER

La estrategia de ramas canónicas es [`stable` / `beta` / `alpha`](docs/BRANCH_STRATEGY.md).
Las ramas anteriores `matter` y `matter-beta` se conservan sin forzar
fusiones mientras se migran las referencias, la rama por defecto y la CI.

La especificación de [`=/>start` y grupos oficiales REGISTER](docs/COMMUNITY_REGISTER.md)
separa **detección y auditoría** de los cambios reales (imagen, nombre,
descripción, ajustes, creación y administración). Los subgrupos requieren
coincidencia exacta del identificador al inicio de la descripción, pertenencia
verificada a la comunidad y autorización del dueño. No hay creación masiva
ni cambio de permisos sin revisión del plan.

El [contrato externo `#matter`](docs/EXTERNAL_BOT_COMPAT.md) permite que
bots ajenos actúen como clientes autorizados del control `=/>`, sin copiar
sesiones WhatsApp ni conceder privilegios por compartir un prefijo.

## Comando alpha =/>start

El proceso de control Base-Matter registra la orden `=/>start` y comprueba
autorización del dueño del socket antes de admitir el inventario de
comunidades. Funciona aunque el socket esté apagado. Las operaciones
`=/>start apply` se rechazan mientras no exista un plan de cambios
verificado; no se modifica ninguna comunidad por autorizar la lectura.

## =/>start — inventario comunitario de solo lectura

Desde `alpha`, la confirmación del controlador Base-Matter permite que el
proveedor Rust consulte las comunidades de la sesión WhatsApp, compruebe
qué comunidades tienen admin verificado para el propio JID PN/LID, enumere
subgrupos vinculados y compare la primera línea exacta de la descripción
con `MATTER:OFFICIAL:register:<community-id>`. Responde únicamente al
chat propio con el resultado o un fallo de lectura. Los cambios de nombre,
imagen, permisos, links y creación permanecen bloqueados.

## Estética reutilizable de menú y ayuda

El módulo Rust `control_style` renderiza `menu`, `help` y `h`
mediante una plantilla compartida con título, bot ID, encabezado,
comandos globales y créditos. `ControlThemes` aísla overrides
por identificador de comunidad y utiliza un diseño por defecto.
Es solo presentación y no atribuye roles ni permisos. La persistencia,
la entrega al chat y los estilos de bots externos se integrarán
por el contrato público; no hay una copia exacta de una estética
ajena sin su diseño fuente.

## Respuestas de estado y extensiones del control

El receptor `whatsapp-matter-control` puede adjuntar texto informativo
limitado a 760 bytes al resultado `applied`, para `status`,
encendido/apagado y extensiones autorizadas. Nunca devuelve estos datos
en un rechazo ni un replay. El proveedor verificará ese campo antes de
enviarlo al chat; el cambio no altera la firma HMAC de entrada.

## Archivo histórico — Menús anteriores de WhatsApp (sustituidos por menú categorizado)

Las extensiones `=/>menu`, `=/>help` y `=/>h` usan el renderizador
Rust de `ControlTheme`, tras validar el propietario del socket por HMAC.
El receptor devuelve texto limitado por el IPC y el proveedor lo responde
únicamente al chat privado propio. La plantilla incorpora nombre,
bot ID y créditos. El diseño puede variar por comunidad a través de
`ControlThemes`; todavía no hay almacenamiento ni administración de temas
desde WhatsApp. En paralelo `=/>start` mantiene el informe de lectura.

## Menú universal sin diseño inventado

Los comandos `=/>menu`, `=/>help` y `=/>h` muestran exclusivamente los comandos globales existentes, con texto plano y sin un marco decorativo inventado. La estética de otro bot solo puede reutilizarse tras observar un menú real autorizado y extraer sus componentes visuales, nunca sus comandos o identidad. No se afirma que el proveedor pueda leer automáticamente menús de terceros.

## Ramas definitivas y estética del menú

La rama predeterminada GitHub ya es `stable`; el desarrollo continúa en `alpha` y la integración validada se prepara en `beta`. Se retirarán referencias heredadas sin descartar commits exclusivos. La salida de `=/>menu` y `=/>help` es deliberadamente un listado sencillo. [La estética real de otros bots](docs/MENU_STYLE_IMPORT.md) se extraerá únicamente de un mensaje fuente autenticado, sin copiar sus comandos ni contenido.

## Respaldo de recuperación comunitaria `=/>add++`

El controlador Rust reconoce la orden global exacta `=/>add++` del dueño del socket, con independencia de que el bot esté encendido. Por defecto solo admite una previsualización; `=/>add++ snapshot <community-jid>` reserva una operación explícita de inventario privado cifrado. No existe modo `apply` de incorporación masiva y ningún comando agrega personas sin autorización o consentimiento. El objetivo es preparar una copia protegida de la lista accesible **antes** de una eventual suspensión; no garantiza acceso posterior ni eludir restricciones de WhatsApp. [Plan de recuperación](docs/COMMUNITY_RECOVERY.md).

## Vista previa real de `=/>add++` en WhatsApp Rust

En `alpha` el proveedor consulta `get_linked_groups_participants` solo para comunidades con administrador verificado, deduplica los identificadores PN/LID conocidos y muestra el número aproximado y el nombre sugerido para un grupo de respaldo. No transmite listas de números por chat ni agrega a ninguna persona. La modalidad `snapshot` aún indica explícitamente que no se ha guardado un respaldo cifrado. No está aprobada para una comunidad suspendida.

## Cifrado del respaldo comunitario

El proveedor Rust incorpora instantáneas de recuperación cifradas con XChaCha20-Poly1305, un nonce aleatorio y una clave dedicada de 256 bits distinta del puente WhatsApp. Cada instantánea contiene los identificadores PN/LID accesibles para recuperación futura, pero se guarda únicamente como ciphertext local con permisos 0600 en el directorio privado de la sesión. No se copia ninguna sesión ni se publican números en GitHub. Para configuraciones anteriores se habilita la clave sin reemplazar los valores existentes con `bash scripts/whatsapp-test-setup.sh --enable-recovery`. La conexión de este almacén con `=/>add++ snapshot` queda sujeta al próximo checkpoint de CI.

## `=/>add++ snapshot` — Respaldo cifrado conectado

Cuando el dueño emite `=/>add++ snapshot <community-jid>` en su propio chat, Base-Matter primero valida la firma HMAC y el propietario del socket; el proveedor revisa después que la sesión sea administradora de una comunidad activa, solicita los participantes vinculados y crea un archivo cifrado XChaCha20-Poly1305, sin publicar números de teléfono ni modificar grupos. Se requiere una clave adicional generada con `scripts/whatsapp-test-setup.sh --enable-recovery` para configuraciones existentes. Si falla la consulta, el guardado o la autorización, no se declara una copia completada. La recuperación por invitaciones sigue pendiente; una instantánea no fuerza a miembros a unirse a otra comunidad.

## Comprobación offline de respaldos

El proveedor expone `--recovery-verify <archivo>.enc.json` para comprobar autenticidad del snapshot cifrado, dueño y socket, sin conectarse a WhatsApp. Solo muestra el total de registros, nunca identificadores privados. El comando de instantánea es exclusivo de comunidades activas con admin verificado; no constituye autorización para agregar masivamente a terceros. [Guía de prueba](docs/WHATSAPP_TRIAL.md).

## Prueba CLI del respaldo sin WhatsApp

Las pruebas automatizadas ejecutan el binario real del proveedor con `--recovery-verify` y verifican que puede validar la instantánea sin abrir la sesión, sin crear SQLite y sin mostrar PN/LID. Se rechazan llaves equivocadas y nombres de archivo inválidos. La prueba sigue siendo offline y no certifica que una cuenta real permita el inventario.

## Sintaxis de =/>add (alpha)

Se ha integrado un parser Rust independiente y se ha reservado el comando global `=/>add` con autorización del propietario del socket. Admite `add user` (número, nombre de usuario o remitente del mensaje respondido), `add group [--hide] community <id>`, `add community new --Nombre`, `add community backup <origen> to <destino> [--hide]` y referencias `official --register`. Todos son **planes sintácticos sin mutaciones**: la lectura de reply, el traslado de grupos y la incorporación a comunidades todavía requieren contexto WhatsApp autenticado, permisos administrativos y aceptación de los usuarios. Ver [contrato de comandos](docs/ADD_COMMANDS.md).

## Pruebas Rust con Cargo en Debian

El controlador Rust se prueba con `cargo check --all-targets --locked` y
`cargo test --all-targets --locked`. Los tests de integración del binario
`whatsapp-matter-control` deben compilar incluso cuando `cargo check` no expone
`CARGO_BIN_EXE_whatsapp-matter-control`; al ejecutar las pruebas, la ruta del
binario se obtiene del entorno de integración de Cargo. Estas pruebas son
offline y no acreditan una conexión real de WhatsApp.
