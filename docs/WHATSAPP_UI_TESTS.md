## 2026-11-19 — Privacidad y semántica de ediciones Anti

- Base previo ffb8a7a344d010f870e195571e8d0d14285e6478. La pasarela de inspección Anti registra metadatos de mensajes de grupos **solo si el propietario habilitó ese grupo** según política actual del socket. Grupos apagados se omiten; la pasarela no introduce una excepción de activación.
- Una edición clásica sin `edited_message.text_content()` ya no se marca como descifrada/legible: se considera **opaca sin inspeccionar**, igual que un sobre E2EE sin clave original.
- Sin dar permisos, ejecutar comandos normalizados ni realizar moderación. `anti audit` conserva solo conteos para grupo activo.
- Ambos README revisados; CI del SHA siguiente pendiente.

## 2026-11-19 — Auditoría de mensajes editados y camuflados

- Base alpha previo: 1aaf76e48930959f45f72bede16e4e33228d6ae4. `=/>anti audit` permite consultar contadores de ediciones legibles, cifradas sin inspeccionar, comandos disimulados y enlaces disimulados del grupo. `=/>anti --global audit` en privado del dueño agrega el socket.
- Se audita simultáneamente edición y camuflaje, sin conservar cuerpo, remitente ni identificador de mensaje. En un mismo evento pueden aumentar varios contadores; son indicadores, no personas culpables.
- Filtros OFF y acciones automáticas desconectadas, no hay borrado ni expulsión. Comandos alterados no se ejecutan.
- Clippy del test con carácter invisible: se usa escape explícito `\\u{200B}`.
- READMEs 2/2 revisados, CI del nuevo commit pendiente. Precaución: ediciones cifradas del cliente sin secreto original se clasifican como no inspeccionadas.

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

## 2026-11-19 — AntiLink por lista de dominios, sin filtros de redes sociales

- Base alpha anterior: `8b0fc5516e6d8ab7e313469bfbd05bd1103d7f30`.
- Se eliminaron del registro y del menú `antilinkwa`, `antilinktelegram`, `antilinkdiscord`, `antilinkinstagram`, `antilinkfacebook`, `antilinktiktok`, `antilinkyoutube`, `antilinkx`, `antilinkshort` y `antilinkphishing`. Las tablas antiguas conservan sus filas sin ejecutarlas ni eliminarlas; no se reactivan tras reiniciar.
- Queda solo el detector genérico `antilink`. Su lista permite dominios como `t.me`, `instagram.com` o `youtube.com` bajo un **único** filtro. Compara host exacto y subdominios (no substrings); lista vacía => no detecciones.
- Nuevo comando `=/>set anti link add|del|list <dominios>` para el **chat actual** firmado, en grupos o privado, y `=/>set anti link --global add|del|list ...` para todo el socket; alias `=/>set antilink`. El global hereda hacia cada chat. Cambios persistentes SQLite privados, auditados y autorizados al propietario; hasta 12 entradas por comando y 64 por ámbito. No altera encendido. Un global `del` no borra override local.
- En grupos `=/>anti link on|off|inherit` sigue separando configuración de habilitación. Todos los antis OFF inicialmente; aun si está ON el detector es solo lectura y NO ejecuta moderación, expulsión ni borrado automático.
- Pruebas: persistencia, herencia, aislamiento, URL con dominio exacto, rechazo de sintaxis, no ejecución de nombres retirados y menú sin entradas de redes; CI de este SHA pendiente.
- README revisados 2/2. Bot-Matter sigue bloqueado. Siguiente: verificar CI y pedir prueba WhatsApp de `set anti link` en el chat propio y un grupo autorizado.


## 2026-11-19 — Burbuja de selección y tarjetas compatibles

- Reporte visual Android: al pulsar «Categorías», el cliente publicaba «☾︎ Anti / Abrir menú» en vez del comando elegido. El selector ahora usa `title: "=/>menu anti"` y elimina `description`. El ID opaco y su enrutamiento HMAC siguen intactos; la burbuja depende de cómo WhatsApp represente esa fila.
- `MATTER_WA_MENU_LAB=1` añade tarjetas NativeFlow de prueba TS18/TS19 en menús raíz de **chats privados habilitados**. Sus botones únicamente emiten ACK de lectura y pasan por las comprobaciones existentes. Si el envío nativo falla, se intenta la tarjeta legacy.
- El chat privado del propietario compara adicionalmente el transporte legacy y sigue siendo el único lugar que puede ensayar Rich HTML y audio. No se envía HTML, multimedia ni comparaciones de legacy a lectores externos o grupos.
- RichResponse HTML no equivale a HTML web y el cliente puede descartarlo sin dar error: estado **experimental sin visualización confirmada**. El HTML offline continúa disponible.
- No se modifican roles, antis, activación, secretos ni derechos de ejecución. Bot-Matter permanece bloqueado. CI Linux/Windows/proveedor y prueba de teléfono para este SHA pendientes. README revisados: 2/2.

# Base-Matter alpha — probar el menú en WhatsApp

## Nueva prueba de cita e instrucciones compactas (2026-11-19)

En el teléfono, abre `=/>menu` y comprueba que «Instrucciones» tiene **cuatro líneas** breves, sin espacios grandes entre ellas. Todas comienzan con `> ` en el texto enviado; WhatsApp debería representarlas como una cita. El contenido resume status con bot apagado, identidad administrativa, externos OFF y sockets/auditoría. En `style none` el contenido sigue breve, sin decoración ni citas.

Las capturas aportadas verifican el menú y el botón en Android anteriores a este cambio; falta confirmar el nuevo formato citado. Los botones TS18/TS19, audio y HTML experimental no cambian.



## Inicio en Debian (solo una instancia por sesión)

```bash
cd /home/Sakura/Base-Matter
git status --short --branch
git fetch origin
git switch alpha
git pull --ff-only origin alpha
export MATTER_WA_NATIVE_BUTTONS=1
export MATTER_WA_MENU_LAB=1
export MATTER_WA_RICH_HTML=1
cargo run --locked
```

Si Git encuentra cambios/conflictos, no descartarlos. Si otra instancia
usa el mismo SQLite, detenerla con Ctrl+C antes. El lanzador inicia
controlador Rust y proveedor Rust, recupera la sesión o muestra el
pairing code de 8 caracteres. No se usa QR ni Node.js.

## Prueba en el chat propio de WhatsApp

```text
=/>status
=/>bot on
=/>menu
=/>menu anti
=/>menu all
=/>menu config
=/>help
=/>help antilink
```

`=/>menu` muestra primero el diseño del usuario y únicamente las categorías con comandos implementados,
con un solo botón **Categorías**. Las 21 entradas se conservan en el
catálogo interno; las vacías no aparecen en el menú ni en el selector.
Las rutas históricas de paginación siguen aceptándose para compatibilidad.

Con `MATTER_WA_MENU_LAB=1`, el menú raíz en un **chat privado habilitado**
puede añadir dos tarjetas NativeFlow: **TS18** (Audio, MP4) y **TS19**
(Audio, MP4, Enlace). Esas respuestas son únicamente de diagnóstico.
En el **self-chat autenticado del dueño** se añade la comparación
con tarjetas clásicas ButtonsMessage y un sonido Ogg/Opus original
reproducible al tocarlo. Solo allí se intenta HTML RichResponse
si `MATTER_WA_RICH_HTML=1`; WhatsApp puede ignorarlo aun si el
envío se acepta. Ni HTML ni sonido se ejecutan automáticamente.
No se envían tarjetas experimentales automáticamente a grupos.

Las pulsaciones deben devolver `=/>test ack audio|mp4|link`.
Son pruebas de transporte. No descargan archivos, no ejecutan código,
no alteran roles. `=/>test ui` y `=/>test row` siguen funcionando
como diagnósticos independientes.

El menú no explica la implementación; `=/>help` indica únicamente
cómo solicitar ayuda. `style none` conserva el menú sin decoración.
No se muestran categorías vacías. Los nombres visibles están en negritas;
las rutas de comandos, entre acentos graves de código WhatsApp.
Los cinco alfabetos y la plantilla reutilizable están en
`templates/message-alphabets.json` y `templates/message-frame.txt`.

**Aceptación real pendiente:** capturas del selector, TS18, TS19,
HTML, y audio en Android. Las pruebas GitHub Actions verifican
compilación y contratos offline, no lo que muestra el cliente.
Si el renderizado falla, el menú de texto permanece disponible.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
