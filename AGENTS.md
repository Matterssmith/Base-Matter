# Base-Matter — repository instructions

## Referencia prioritaria del propietario — botones de fila del ejemplo Go (2026-11-18)

El propietario proporcionó código fuente `package dl` con comandos `ts18` y
`ts19`. Conservar el original en `docs/references/ts18_ts19_buttons.go.txt`.
Señal visual deseada: tarjeta con miniatura JPEG cuadrada, título y texto
descriptivo, con botones INFERIORES horizontales (Audio, MP4 y, opcional,
Enlace). La función Go que muestra esa tarjeta es `c.SendButtonsRow`.
El ejemplo no acredita soporte de HTML nativo ni del selector de países de
otra imagen. Solo sirve como referencia de interfaz; Rust sigue siendo el
lenguaje del proveedor y núcleo. No incorporar código Go ni habilitar
descargas o acciones desde botones sin verificar contratos, autorización
y cliente WhatsApp real. La observación del propietario de que los botones
actuales no funcionan y el menú antis es ilegible es un bug de UX abierto.
Ver `docs/WHATSAPP_BUTTON_CARD_REFERENCE.md`.


## Contrato de menú de Base-Matter, 2026-11-18

`src/menu.rs` contiene contratos de presentación:
nueve carpetas y un módulo por comando, sin
autoridad de ejecución. Los menús se generan
automáticamente y muestran `[pendiente]` cuando
no existe ejecutor. Estilo classic con
`╞═╦═`, `│♪║`, `│♪╠`, `│♪╚`;
estilo none sin símbolos. Idiomas es/en.
El prefijo global aparece una sola vez y no
se agrega marca personal ni créditos al menú.
Solo el dueño autenticado cambia el estilo
desde el privado; persistir en SQLite separado.

Las respuestas rápidas solo ejecutan consultas
en el puente HMAC/PN/LID, nunca modifican
activación ni roles. El HTML completo es
experimental, sin pruebas de renderizado móvil.
No borrar la plantilla antigua hasta auditar
sus consumidores, pero no usarla en el
controlador de producción. El permiso de
Alya Core continúa pendiente; ninguna clave
se importa de Aura Reed ni se transmite por
WhatsApp. Ver `docs/MENU_CATALOG.md`.

## Contrato de permisos de IA concedidos por titulares — 2026-11-18

Cada clave externa pertenece al titular de la API. Su disponibilidad
en un repositorio compartido o una sesión GitHub **no autoriza su uso**.
Obtener consentimiento específico, alcance, modelos, cuotas, datos
permitidos y vencimiento. Vault-Matter conservará credenciales
cifradas únicamente y aplicará esas condiciones por código antes
de invocar modelos; además pondrá las restricciones del titular en
las instrucciones prioritarias de la IA para **cada llamada**.
No publicar credenciales ni en código, README, mensajes o logs.

Base-Matter no debe manejar claves directas ni invocar APIs con
fallos de Vault: usar solo `src/vault_ai.rs` con Unix socket y
`MATTER_VAULT_AI_UID` del broker de confianza. Rechazar mensajes
sin `policy_applied=true`, concesión válida, revisión positiva
y capacidad exacta. El broker y el verificador de licencias
no están desplegados, por tanto se falla cerrado. Los
permisos del modelo no autorizan ejecución directa, sandbox,
roles, acceso a dispositivos ni publicación de código.
No añadir créditos ni marca personal al menú de Base-Matter.

## Contrato 2026-11-18 — `matter` es programación asistida

El usuario reasignó `matter` desde su reserva anterior:
ahora es un asistente tipo Codex que pregunta y propone
planes, **no ejecuta comandos ni cambia archivos**.
Nunca volver a registrarlo como alias del sandbox o consola.
Se verificó `jbeckforddev/Aura-Bot-2.0` en master/test;
no se encontró una integración Venice/Claude en los
archivos consultados. No copiar ni publicar secretos de
ese bot. Credenciales solo del servicio privado.

Jerarquía de proveedores: Claude > Venice > OpenAI,
según clave y modelo explícitos. El protocolo y los
endpoints están en `src/matter_ai.rs`. API no equivale a
permiso Vault, rol global o escritura GitHub. Solo PN/LID
propio, contexto privado, HMAC y antirreplay autorizan
el comando; las peticiones externas fallan cerrado.
Sin prueba en cuenta real no declarar el agente listo.
Los comandos directos `console`/`eval`/`test` y las
alternativas `sandbox`/`sandbox test` son independientes.
No incluir marcas personales ni créditos automáticos
en respuestas o menús de Base-Matter.

## Invariante más reciente — ensayo RichResponse del propietario, solo self-chat

El archivo `rich_preview.rs` y el CLI `--rich-preview-*` conservan
su naturaleza offline. Como continuación separada, `rich_transport.rs`
puede empaquetar el JSON validado como `wa::AIRichResponseMessage`,
**sin** `botForwardedMessage`, `0@bot`, `forwardOrigin` ni
`isForwarded` inventados. Solo se intenta enviar si el operador fijó
`MATTER_WA_RICH_HTML=1`, el control HMAC ya fue autorizado y el
mensaje es del propietario en su chat privado propio. No habilitar
grupos, privados externos ni funciones administrativas por este modo.
Ante error de envío, fallback NativeFlow/texto. Un envío aceptado no
demuestra que el cliente renderice la hoja. No se declara soporte
Android/iOS hasta la prueba manual. Mantener todo desactivado por defecto.

## Invariante adicional — compatibilidad de Rich HTML (alpha, 2026-11-18)

Se revisaron ejemplos no confiables de Go/JS/HTML recibidos en
`xd.zip`, usados exclusivamente como referencia de datos.
Nunca copiar el origen falso Meta AI `0@bot`, marcar como
reenviado un mensaje que no lo fue ni activar scripts HTML
mediante el protocolo de WhatsApp. No incorporar juegos/medios
externos a Base-Matter sin contratos, licencias y pruebas.

El módulo `providers/whatsapp-rust/src/rich_preview.rs` construye
HTML estático y estructura JSON experimental **solo offline**.
El CLI `--rich-preview-html/json` no debe tocar sesión,
credenciales ni red. HTML en WhatsApp **no está demostrado**
y no se debe confundir con botones NativeFlow (opt-in).
Bot-Matter permanece bloqueado. READMEs 2/2 antes de commits.

## Contrato nuevo — botones NativeFlow y HTML breve (alpha, 2026-11-18)

Solicitud expresa: preparar controles interactivos reales al estilo de botones
de Telegram, conservando íntegro el menú original en **un solo mensaje**.
El proveedor Rust `whatsapp-rust` 0.7.0 ofrece envío de protobuf
`InteractiveMessage.NativeFlowMessage` y respuestas `quick_reply`;
integrar sin Node y con un único proceso dueño de la sesión.

El envío de NativeFlow es **opt-in** mediante
`MATTER_WA_NATIVE_BUTTONS=1`, desactivado por defecto hasta probar
recepción y visualización reales, con fallback texto si el envío falla.
Los tres botones iniciales son exclusivamente
`status`, `menu`, `credits`: toda respuesta de botón vuelve a
pasar por la comprobación de identidad y firma HMAC de Base-Matter.
No activar `bot on/off` ni roles mediante IDs de botón.

HTML breve de estilo debe convertirse a texto WhatsApp con etiquetas
permitidas y validación cerrada. No enviar `FOAHtmlPrimitiveDemoDONOTUSE`,
HTML/CSS/JS ejecutable ni afirmar WebView nativo. **No se puede
garantizar la distinción Android/iPhone por el evento actual.**
No usar heurísticas de JID; botones específicos por sistema deberán
esperar metadatos autenticados y pruebas en ambos clientes.

## Invariante de usuario actualizada — 2026-11-18: restaurar diseño original y alcance de status

Esta instrucción expresa del propietario prevalece sobre la sección antigua de
menú abreviado y prohibición de créditos en el **menú**. El menú completo de
`templates/matterssmith-menu.txt` conserva exactamente el encabezado ornamental,
`Matterssmith`, `Base-Matter`, las 12 líneas de comandos, el cierre con 🌀 y
`Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].` una sola vez.
No repetir ese pie en respuestas ordinarias de `status`, ajustes ni errores.
Conservar las decoraciones por chat para las otras respuestas.

`=/>status` es consulta de estado del chat actual, permitida a cualquier
usuario con identidad WhatsApp verificada, también con el bot apagado, en
grupos y privados externos. No concede poderes, roles ni acceso a otro socket.
`=/>status --global` expone políticas de socket solamente al propietario
autenticado, incluso si lo envía desde un grupo; las lecturas externas reciben
una explicación sin revelar el estado global. Mutaciones siempre reservadas al
dueño. Es obligatorio probar ambas rutas firmadas y el renderizador real,
manteniendo Bot-Matter bloqueado y antis apagados por defecto.

## Invariante vigente de interfaz — 2026-11-18

- Los créditos `Hecho por Lournyx Matterssmith [Lou-Nyx].`
  siguen en todos los códigos y documentos correspondientes,
  **nunca** como pie automático de cada respuesta WhatsApp.
  El único comando visible de atribución es `=/>credits`
  (alias `creditos` y `créditos`), sin añadir autoridad
  a lectores externos.
- El menú único presenta el prefijo global `=/>` una sola vez;
  las variantes comparten una entrada abreviada
  (`status [--global]`, `bot [ámbito] on|off|inherit`).
  El marco común no repite los títulos Matterssmith/Base-Matter.
- El tema por chat puede usar `{{TITLE}}` y `{{ACCENT}}`
  de forma opcional, pero el marco base debe continuar sin ambos
  títulos. `{{BODY}}` es obligatorio, único y acotado.
- HTML incrustado no es equivalente a HTML web normal:
  Aura usa tipos experimentales no validados para Rust/iOS.
  No enviar scripts no confiables en mensajes ni convertir
  a Node. No inferir Android/iPhone por IDs de mensajes;
  usar una señal autenticada y compatibilidad probada
  antes de seleccionar botones por dispositivo.
- Esta revisión es solo `alpha`. Repetir Rustfmt/Clippy/tests
  en Debian y probar el menú en WhatsApp antes de promover.


Read `docs/DEVELOPMENT_STATUS.md`, the root README and architecture before work.
Inspect actual code, Git status, branch, ignored files and applicable nested instructions.
Do not assume a past report is current. Preserve useful existing implementations.

### Invariante de plantilla por chat (alpha; 2026-11-18)

El registro `ControlThemes` debe compartirse entre el menú y el
IPC. El identificador del chat solo se usa para seleccionar estilo
**después** de que el puente valide HMAC, ámbito, actor y replay.
Cada resultado con texto —incluidos estado, políticas, auditoría,
ayuda y previsualizaciones— se representa mediante el renderizador
de `src/control_style.rs`. No insertar marcos Unicode directamente
en cada comando. El proveedor `whatsapp-rust` **no** aplica otra
plantilla a un mensaje ya formateado por IPC, aunque tenga tema
de grupo distinto. La ruta de salida comprueba longitud y controles
antes del reconocimiento idempotente del tema.

La plantilla de estilo por grupo puede configurarse mediante
`ControlThemes::set_community` con `ControlTheme::with_template`
validado; todavía no hay comando persistente `=/>set style`,
y no debe afirmarse lo contrario. `community` en las políticas
de activación es un tipo de chat distinto del ámbito de
personalización de temas; su encendido sigue sin aplicarse
sin metadatos atestiguados de WhatsApp.

## Contrato prioritario del propietario — 2026-11-18 (sustituye comandos anteriores)

- En WhatsApp, **solo `=/>bot on/off/inherit`** gestiona la activación. Retirar
  `=/>on/off` de proveedor, puente, ayuda y menús; el gate de power queda
  interno. `bot on` activa en una transacción la política autorizada y el
  gate operativo, sin necesitar una segunda acción; `bot off` nunca apaga
  chats ajenos. Los viejos APIs de power del núcleo son compatibilidad,
  **nunca comandos de WhatsApp**.
- `=/>bot on/off` es el chat exacto; `=/>bot --global on/off` es la política
  por defecto del socket; `=/>bot group/private/channel/community on/off`
  son políticas por tipo. Comunidad significa *grupo de avisos* y **solo**
  debe aplicarse cuando el proveedor atestigüe el tipo mediante metadatos;
  nunca asumirlo a partir de la terminación `@g.us`. Canales también
  requieren identidad de proveedor verificada. Ámbitos externos apagados
  por defecto, estado `=/>status` siempre disponible.
- Menú global **sin categorías**. Tomar el marco, símbolos y espaciado
  del ejemplo del propietario, NO importar sus categorías ni comandos `#`.
  Los diseños viven en `templates/matterssmith-control.txt` y
  `templates/matterssmith-menu.txt`, fuera de manejadores de comandos.
  Cada respuesta, incluidas las operaciones REGISTER y vistas previas
  de WhatsApp, debe pasar por el renderizador compartido de
  `src/control_style.rs` con `\n` reales. Permitir sustitución controlada
  por alcance sin modificar seguridad, rutas ni handlers.
- Roles: Vault-Matter sigue siendo autoridad exclusiva; sin lector remoto
  autenticado, no existe ninguna concesión operativa de rango local.
- Publicar cambios primero en alpha. Promover a beta únicamente revisiones
  concretas con compilación, pruebas y uso real verificados; beta→stable
  después de estabilidad repetible. Las ramas ya divergen: no hacer merges
  ciegos ni sobrescribir sus commits propios.

## Identity

Preserve the project's visual identity and existing credits. Where credits apply:

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

Do not add a separate author line or attribute project authorship to automation.
Preserve upstream copyright/license notices when importing provider code.

## Git and verification

Default development: `alpha` (experimental), `beta` (integration), `stable` (releases). GitHub is the source of truth. `stable` is the repository default. Only these THREE branches should remain after migration. Legacy references `matter`, `matter-beta`, `stable-hardening-20261117` and `stable-socket-owner-control-20261117` have no exclusive commits relative to their replacements; an authorized repository administrator may delete their refs after updating consumers and preserving tags or release references as appropriate. Never rewrite unique commits or delete session data.
Inspect all local differences first. When dirty, protect tracked and new
nonignored files with `git stash push -u`. Fetch origin, switch to the explicitly selected `alpha` or `beta` branch,
reset ONLY to the verified remote head of the explicitly selected working branch, restore the stash and inspect conflicts. Do not mix `alpha`, `beta` and legacy `matter-beta` during recovery. If fetch
or stash restoration fails, stop dependent modifications/verification, retain
recovery data and report the failure. Never use git clean or remove ignored
sessions, databases or credentials. Preserve unrelated stashes and user changes.

Run cargo fmt --all, cargo check --all-targets, cargo test and git diff --check
for implementation checkpoints. Report actual results. Do not change global
linker/toolchain configuration without demonstrated need.

The user has authorized committing and pushing verified development checkpoints
to alpha for experimental checkpoints. Promote reviewed, independently verified subsets by PR to beta, then stable. Review EVERY repository README before EACH commit, report total,
reviewed, updated, unchanged, findings, fixes and pending issues. Update affected
README/code/contracts together; preserve correct content and design. Check new
files, full diff, examples, internal links, credits and accidental secrets.
Confirm the pushed SHA against origin. The owner has authorized promoting independently verified stable subsets to
`stable`, not a bulk merge of incomplete alpha/beta. Require green CI on the exact
candidate SHA and preserve divergent stable commits (including Cargo.lock
history). Leave all unverified work in alpha.

## Scope and continuity

Finish Matter-Base before starting Matter-Bot. Continue H audit/corrections,
I integration/resilience, J security, K contracts and L release readiness.
Missing earlier prerequisites must be implemented and tested; do not call
interfaces, mocks or isolated tests a completed external integration.
Record exact next work, changes, tests and blockers in docs/DEVELOPMENT_STATUS.md
at checkpoints and before ending a session. Preserve its historical entries.

Priority is Matter-Base with the real Matter-Vault contract and global control
commands. Vault is independent and must not store platform credentials/sessions.
Do not change cryptographic formats without compatibility/migration review.
Intercept `=` and `/>` control prefixes before the normal router. Only the socket
owner can power it on/off. A global owner label is not authority over another
socket; an explicit socket ID still needs authorization. Operational power is
independent of network connection. Powered-off sockets execute no normal commands.
Policy precedence when powered on: Group > Socket > Global > None (deny).
Preserve socket isolation and persist/audit administrative changes atomically.

## Owner correction — real group operations and encrypted socket roles (2026-11-17)

- A real WhatsApp `=/>status`, `=/>on`, `=/>help` and `=/>menu`
  round-trip was observed in the self-chat after PN/LID correction.
  This proves the private owner bridge works, not group functionality.
- WhatsApp private self-chat accepts **only socket-wide configuration**:
  power, status, socket-role changes, read-only role catalog and audit.
  Menus, commands targeting groups/communities, recovery previews and
  normal bot functionality must not be handled in private.
- WhatsApp group events need a **verified actual group JID and sender**
  from the authenticated Rust connection and authenticated HMAC binding.
  Owner-only `=/>group status/on/off/inherit`, `=/>group role`,
  menus and appropriately scoped preview commands are being integrated
  in alpha; never accept a text-supplied group ID as authority.
- Only the real socket owner can change power and group policy. A global
  project owner, `co-owner` or `staff` label is **not** a bypass.
  Project-level ranks, per-bot ranks and per-socket/per-group ranks are
  separate security namespaces; do not infer one from another.
- Socket/group ranks: `owner` is the immutable validated socket owner;
  other available ranks are `co-owner`, `admin`, `moderator`,
  `operator`, `trusted`, `member`, `restricted`, `banned`.
  `user` remains implicit and never creates a role-grant entry.
  All changes must be owner-authorized, audited and persisted.
- A local SQLite role record is **not** GitHub durability. Before
  claiming Vault-Matter integration, implement verified encrypted
  role snapshot publication and recovery through an authenticated
  GitHub/Vault provider with CAS, key isolation and audit. No plaintext
  roles or secrets in Git, and no WhatsApp session files in Vault.
  Fail closed on inaccessible Vault; do not fabricate successful sync.
- Alpha is for regressions and trials; promote isolated, verified
  changes by review to beta then stable, never bulk-merge divergent
  histories. Operate from a single Git clone and shared Cargo build
  cache in the owner's one-terminal environment. Preserve sessions.

## Safety invariant: external chat surfaces default OFF

The `group`, external `private`, and `channel` kinds must remain
disabled unless the verified socket owner explicitly enables that kind
or an exact supported chat scope. A socket-wide `--global on`
**must not** make all joined groups or unknown private DMs respond
automatically. `inherit` removes an override, falling back to the
kind default; missing kind default means deny. The owner's own
private self-chat uses a separate `kind:self` scope and can
inherit the socket fallback. Never conflate external-private
with self-chat settings. New regression tests must prove default
denial even when the socket-global policy is enabled.

## Verified-public-read exception (alpha; identity boundary)

The earlier owner-only HMAC path remains exclusive for mutations.
Incoming `from_me=false` events may enter a **different**
HMAC-signed path only for `status/estado`, `menu/menú`
(with one category), and argument-free `bot`.
The signer must obtain actor and destination JIDs from the
authenticated WhatsApp event and verify the linked client
own PN/LID; never elevate an external sender to socket owner.
The controller must verify signature, exact context, nonce/
timestamp and replay, then use a **read-only** policy/menu
dispatcher. Status is available when bot-off, while menu
is silent. Any control/mutation/debug/global exposure by
this path is forbidden. No arbitrary extension callbacks.

The owner's self-chat has an independent `kind:self`
activation setting. `kind:private` applies to external
private chats only. Do not infer newsletter/channel
identity from group/private and do not claim it works until
its provider source is verified. Tests must include incoming
group and external-private traffic plus attempted privilege
escalation. `beta/stable` remain untouched.

## Owner update — activation commands and menu design (current alpha contract)

This section supersedes the older 2026-11-17 statement that menus
must be group-only and that help/h remain valid. The user now requires:
- `=/>status` as the **effective current-chat bot policy** even
  while disabled; `=/>status --global` reports actual default
  policy and socket power independently. The socket is not the bot.
- `=/>bot on/off/inherit` for the chat, bot defaults for
  `group/private/channel` in owner-private configuration,
  `=/>bot --global on/off` for the entire socket's fallback.
  Specific chat > kind > socket global > deny, with socket power
  always a separate outer gate.
- `=/>menu` in any authenticated, bot-enabled context, with
  a distinct Matterssmith Unicode theme and categorized entries.
  `=/>help` and `=/>h` are retired. When the bot is disabled,
  menu and normal commands are silent; status and the authenticated
  owner's necessary recovery/configuration actions remain available.
- `=/>set` without arguments describes configuration;
  `=/>set role` never performs local grants. Translation aliases
  must be explicitly mapped, not arbitrary text matches. The
  optional final `--debug` may expose safe technical context to
  the verified owner but never secrets, raw identities or role data.
- `--global` specifies a **policy scope**, not a role.
  `mod` is a group-moderation authorization requirement,
  `self` is socket-owner plus explicitly Vault-authorized delegates,
  and `selfmod` requires both capabilities. None are implicit
  authority; delegated execution requires a verified Vault reader.
- `=/>set style/theme`, explicitly approved `#menu`
  visual adaptation and AI-backed `--auto` are future
  capabilities. Do not copy commands, users, source
  identities, third-party links, tokens or permissions from
  another bot's message; preview before applying.
- `menu/status` for external actors and WhatsApp channels requires
  a new signed ingress model. Never simply remove the
  `from_me` check to make unauthenticated messages work.
- Develop/test on alpha only; do not promote incomplete
  features to beta or stable. Full command semantics and
  limitations: `docs/BOT_POLICY_V2.md`.

## New authoritative policy: Vault-Matter owns ALL socket/group role state

- Vault-Matter is the **sole authoritative role manager** for socket
  and group roles. Base-Matter has **read-only** decision/query access
  to Vault and no GitHub Vault write credentials or role mutation API.
- A role only takes effect after fresh authenticated Vault confirmation;
  no record, unreadable Vault, bad scope/actor, expired proof or
  revoked revision => deny. No local default grants or offline copy.
- Vault-Matter alone may approve commands from its own authenticated
  WhatsApp/Telegram/Discord gateways, write encrypted role state and
  audit records, and recover from its private repository. A forwarded
  message or Base-generated relay is not itself verified user consent.
- Base's original authenticated socket owner can power down the socket
  without delegated role checks; project staff/owners never inherit
  socket rank permissions implicitly.
- `user` is an implicit absence of a grant; every explicit non-default
  rank is Vault-owned. Local `RoleStore` and encrypted outbox remain
  test utilities, never installed as a live second role authority.
- Alpha WhatsApp controller MUST NOT install local socket role stores
  or allow `set role` to mutate SQLite pending Vault service.
  Keep `=/>roles` honest about missing authority.
- Build a separate Vault Rust service and read-only verified Base
  interface, not a plaintext roles file or GitHub token in the Base
  process. See `docs/VAULT_ROLE_AUTHORITY.md`.
- Continue alpha -> verified beta -> independently validated stable,
  without declaring a working integration from schema-only metadata.

## Current hard constraints — owner clarification 2026-11-17

- Every \`*-Matter\` repository is part of the Matter ecosystem; Bot-Matter
  will consume its components in a later, separate stage.
- DO NOT implement, refactor or launch Bot-Matter until Base-Matter is complete.
  Freeze separate platform prototypes until core readiness, unless the owner
  explicitly reauthorizes them.
- Historical WhatsApp pairing-code connection/ping tests were undertaken
  before this beta version. Do not erase that history. However, do not claim
  the current remote Base repository contains a verified live WhatsApp
  connection without actual source and an end-to-end test.
- WhatsApp linking MUST use pairing code only. QR must not be exposed,
  advertised or used as fallback. Never overwrite existing local sessions.
- The former ping is a test only; it is NOT a permanent global command.
  Global control is implemented by Base-Matter using canonical \`=/>\`,
  independent from the bot's own command prefix and even when the bot is off.
  Legacy \`=\` and \`/>\` aliases are retained until an intentional migration.
- Preserve every ecosystem repository's history and respect the fact that
  provider modules are ultimately consumed by Bot-Matter, not developed as
  an excuse to start Bot-Matter early.

## Lenguajes y WhatsApp — corrección expresa del propietario

- Lenguajes principales: Python, C++ y Rust. Mantener Rust como núcleo y proveedor de WhatsApp ya trabajado. C y Go son secundarios; JavaScript/Node.js es terciario y NO sustituye ni dirige la implementación `whatsapp-rust`.
- Recuperar el código histórico `src/providers/whatsapp/mod.rs` del commit `3c36d97bb` para continuidad; conservar licencias, datos de sesiones y progreso existente.
- Prioridad actual: completar comandos globales, seguridad, verificación y componentes pendientes de Base-Matter sobre Rust. No seguir desarrollando la sonda Baileys/Node ni Bot-Matter.
- La vinculación WhatsApp es exclusivamente por pairing code; el `ping` anterior era una prueba y se retira de comandos durables.
- Atribuir comandos/archivos con el crédito exacto: `Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].` (en comentarios o notas discretas), sin línea duplicada de Autor.
- El estado `powered` de un socket NO implica que esté habilitada su política de ejecución. Las pruebas deben mantener esa separación.

## Updated ecosystem direction (user instruction)

- Target name WhatsApp-Matter replaces Matter-Baileys.
- Target name Telegram-Matter replaces Matter-Telegraf.
- All other repositories follow <Component>-Matter, with Base-Matter and
  Vault-Matter first. Preserve Matterssmith credits and stable branches.
- Inspect and rename the actual GitHub repositories when ecosystem work begins;
  documenting these targets does not mean remote repositories were renamed.
- Each platform repository may contain multiple provider methods in separate
  directories, e.g. providers/baileys and providers/telegraf. Verify the official
  upstream source, pin its revision, preserve history/licenses/notices and keep
  local modifications traceable. Do not clone unverified lookalike repositories.
- Adapters implement universal Matter-Base contracts. Platform libraries and
  provider-specific changes remain in their platform repository, outside Base.
- Full ecosystem management does not imply bypassing a platform's permissions.
- Real WhatsApp verification uses a secondary test account and a single process
  owning its connection; never overwrite an existing session. Then verify
  equivalent Telegram and Discord functionality where supported.
- This scope does not relax Matter-Base readiness criteria. External service
  availability/account linking must be reported honestly when blocked.

## Proveedor Rust verificable

El proveedor de WhatsApp seleccionado es whatsapp-rust, ejecutado desde providers/whatsapp-rust con biblioteca fijada, sesión SQLite privada y pairing code. No utilizar Node/Baileys en sustitución. No afirmar estabilidad hasta comprobar compilación y una sesión real autorizada. Revisar su README y el README raíz antes de cada commit.

## Estado de ramas (instrucción del propietario 2026-11-17)

- Nuevas ramas canónicas: `stable` (release; creada desde `matter`), `beta` (integración; creada desde `matter-beta`) y `alpha` (experimental; creada desde `matter-beta`).
- `matter` y `matter-beta` se conservan temporalmente sin borrar ni reescribir su historial; el predeterminado sigue siendo `matter` hasta cambio separado en GitHub.
- Flujo: cambios en `alpha` -> CI -> revisión por PR a `beta` -> comprobación real externa -> PR a `stable`. No hacer merge global de ramas divergentes. Ver `docs/BRANCH_STRATEGY.md`.
- En cada commit leer los DOS README (raíz y `providers/whatsapp-rust/README.md`), actualizar los afectados y reportar revisión, resultados y problemas.

## Comunidad oficial REGISTER y =/>start

- La orden canónica `=/>start` pertenece a Base-Matter. El controlador autentica el socket y su propietario; el adaptador `whatsapp-rust` verifica el contexto privado del dueño y consulta las comunidades de la sesión autorizada.
- Fase primera (segura): inventario de comunidades, prueba de rol admin del dueño y búsqueda de subgrupos oficiales mediante el encabezado EXACTO `MATTER:OFFICIAL:register:<community-id>` situado en la primera línea de la descripción. El título por sí solo no identifica un grupo oficial.
- La relación comunidad/subgrupo debe venir del proveedor autenticado; un texto o ID de WhatsApp enviado por terceros jamás confiere autoridad. Resolver PN/LID sin adivinar identidades.
- Fase de plan: comparar identidad oficial, nombre, descripción, foto, permisos y enlaces; preparar acciones idempotentes. No crear grupos ni cambiar administradores por la mera ejecución de `start`.
- Fase de aplicación separada: verificar admin de comunidad Y admin del grupo, consentimiento/autorización para cada mutación, aprobación específica del plan vigente, revisión de conflictos, auditoría transaccional, límites de tasa y recuperación. Permitir crear el grupo REGISTER si no existe; vincularlo a la comunidad con autorización explícita y sin duplicados.
- Las DOS cuentas de confianza se deben configurar mediante identidad validada y participación aprobada; nunca inventar números, agregar ni promover usuarios sin verificar consentimientos y permisos de WhatsApp.
- Los administradores existentes solo recibirán una solicitud moderada de enlaces de invitación, evitando menciones masivas, y solamente si la política del grupo lo autoriza. Esos enlaces se guardan cifrados, con caducidad, revocación y uso explícito para búsqueda de asistencia Matter. No publicar enlaces privados sin consentimiento.
- Menú único y diseño comunitario: separar componentes visuales reutilizables (encabezado, cuerpo y pie corto sin atribución automática) de la lógica de autorización. Configurar tema por comunidad; mantener el crédito Matterssmith en código y en `=/>credits` exclusivamente.
- NO anunciar que hay un escaneo o mutación real si el proveedor no confirma su resultado. Conservar pruebas y diferencias respecto a la biblioteca `whatsapp-rust`.

## Interoperabilidad con otros bots: #matter

- `#matter` es comando voluntario de ENTRADA de bots ajenos; NO sustituye `=/>`, que sigue como control canónico en Base-Matter. Los bots externos deben integrarse por contrato versionado de adaptador/IPC autenticado y permisos de mínimo privilegio.
- Identidad de actor, socket, bot externo, canal, alcance, expiración y nonce deben venir de enlace autentificado. No confiar en nombres, números o texto como prueba de identidad, ni permitir acceso a sockets ajenos.
- Sin compartir tokens, claves maestras ni archivos de sesión por defecto. Si un proveedor incompatible requiere migración, crear un EXPORT LOCAL ENCRIPTADO, voluntario y autorizado por dueño, con bloqueo de proceso y controles de origen/destino; jamás duplicar sesiones activas o publicar credenciales en Git/Vault.
- Solo mapear capacidades compatibles; Telegram, Discord y plataformas externas pueden carecer de algunas operaciones y deben reportarlo.

## Menú universal y copia de estética (instrucción correctiva 2026-11-17)

- `=/>menu`, `=/>help` y `=/>h` muestran exclusivamente los comandos
  globales reales en texto plano, sin estética inventada por Base-Matter.
- La intención es leer una respuesta real del comando `menu` de OTRO
  bot, con fuente identificada y permiso del propietario, y extraer
  SOLAMENTE su diseño decorativo. No copiar su lista de comandos,
  identidad, credenciales, enlaces, base de datos, sesiones ni lógica.
- No afirmar que Base observa otros bots sin una sesión autorizada,
  fuente real y control de mensajes. Los estilos importados deben
  validarse y aislarse por comunidad antes de activarse.
- `#matter` queda como interfaz opcional para integraciones externas
  autenticadas; no sustituye el prefijo `=/>` ni copia sesiones activas.

## Recuperación comunitaria =/>add++ (solicitud 2026-11-17)

- Objetivo: anticipar pérdida de una comunidad, preservando un inventario privado de sus integrantes y la posibilidad de ofrecerles entrar voluntariamente en una comunidad reemplazo.
- La sintaxis exacta es `=/>add++` (dos signos más). Requerir autorización global del propietario del socket y verificación admin en el origen y destino. El bot normal apagado no modifica ese control.
- Separar la lectura de miembros, instantánea cifrada y plan de invitación de las mutaciones. Sin consentimiento, límites de WhatsApp o permisos no añadir de golpe a terceros. Evitar reintentos intensivos y suspender ante señales de restricciones.
- Poner en cuarentena `apply` hasta tener verificación de consentimiento, plan revisado, idempotencia y pruebas externas. Las instantáneas cifradas no son sesiones WhatsApp y no se publican en Git.
- El grupo de emergencia puede llevar el nombre de la comunidad, acompañado de `Respaldo`, para evitar suplantar el original. Si la comunidad fue suspendida antes del respaldo y no hay acceso, no afirmar recuperación posible.

## Gramática de =/>add (petición 2026-11-17)

- Las órdenes universales nuevas son `=/>add user <número|@usuario|reply> [group|community <id>]`, `=/>add group [--hide] community <id>`, `=/>add community <id>`, `=/>add community new --Nombre`, `=/>add community backup <origen> to <destino> [--hide]`.
- `<id>` admite `official` y, para el subgrupo oficial configurado, `official --register`. La resolución debe emplear IDs realmente autenticados, nunca coincidencias de título.
- El bot solo debe planificar y reportar, NO añadir usuarios sin su consentimiento ni eludir ajustes de privacidad, cuotas o suspensiones. La promoción de un grupo a otra comunidad exige autorización admin, verificación del chat origen, destino y capacidad API. `--hide` es una preferencia, no una técnica para ocultar usuarios.
- Un mensaje citado se resuelve desde metadata confiable. Un alias @usuario solo se acepta si el proveedor da una resolución de identidad comprobable. Los comandos de grupo aún no cuentan con admisión de mensajes grupales: no declarar mutaciones completas.
- `=/>add++` sigue siendo diferente: inventario y respaldo cifrado preventivo de participantes; no es autorización para incorporaciones masivas.
