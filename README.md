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
