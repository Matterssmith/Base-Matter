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

### Botón selector + Ayuda + Ver todo, y lectura de respuestas Go-style

Se reemplazaron los tres quick replies fijos por un `single_select` con nueve filas para categorías y dos botones de consulta. El parser reconoce tanto `NativeFlowResponseMessage` como `ButtonsResponseMessage` cuando los IDs pertenecen a la lista cerrada. Sin comandos administrativos, envíos espontáneos ni descarga de medios. El cliente real Android/iOS no ha sido validado. README revisado.

### Formato del puente para help

Se aplicó el bloque Rustfmt solicitado por CI a las ramas de ayuda pública. Sin cambios de comportamiento, permisos, claves ni sesión. README revisado.

### Corrección HMAC para navegación

La allowlist del proveedor incorpora `help/h/ayuda` en contextos verificados, y permite `menu all` públicamente si el chat está activo. Mantiene antirreplay y restricciones de propietario. README revisado.

### Actualización UX en alpha

El puente reconoce `help` además del menú, siempre con HMAC y permisos previos. Los botones actuales aún no replican el ejemplo Go. Compatibilidad HTML, audio y pantalla completa pendiente de prueba real; no se han alterado las sesiones.

### 2026-11-18 — Ejemplo original de tarjeta con botones de fila (sin integración)

Se conserva en el repositorio el código Go suministrado por el propietario
para `ts18`/`ts19`, incluida la llamada `SendButtonsRow` que presenta
miniatura, texto y botones inferiores Audio/MP4/Enlace. Es una referencia
visual; **no** se añadió un motor Go, descarga multimedia ni endpoints externos.
Los botones de menú en Rust actuales son un prototipo distinto y la prueba
del teléfono no demostró aún la interacción deseada.
Ver [`docs/WHATSAPP_BUTTON_CARD_REFERENCE.md`](../../docs/WHATSAPP_BUTTON_CARD_REFERENCE.md).

### CI final del menú por categorías

El commit `dcf39b2` pasó las verificaciones del
núcleo en Linux. El CI del proveedor señaló una
única diferencia Rustfmt para el match de
categorías en `src/lib.rs`; corregida exactamente.
La lista sigue cerrada a nueve categorías
registradas; no permite comandos administrativos
a terceros ni páginas arbitrarias. Sin cambios
de sesión o credenciales. Este README fue revisado
antes de publicar la corrección, pendiente de
nueva ejecución del CI y prueba en teléfono.

### CI de submenús públicos (alpha 2026-11-18)

La prueba offline de `578c680` detectó que el proveedor
firmaba submenús genéricos como `menu group`, antes
explícitamente prohibidos. Se sustituyó el comodín por
una lista exacta de categorías: status, bot, set, antis,
roles, system, sandbox, ai y navigation. Solo se
aceptan páginas 1..8; no se autoriza ninguna
mutación ni categoría inventada. Los casos positivos
de menú incluyen set, antis, antis 2 y system.
La lectura pública depende de la activación del chat;
las operaciones de propietario usan otra firma.
README del proveedor actualizado, sin credenciales.

### Verificación de código modular / independiente

La compilación de `2943cedd` llegó a Clippy
en el núcleo y detectó únicamente la regla
`module_inception` de nombres internos; se
mantiene un archivo por comando sin suprimir
advertencias. El proveedor standalone marcó
dos diferencias Rustfmt en
`provider_control_style.rs`; ambas corregidas.
Se conserva el decorador de respaldo y la
separación del menú (no reimportar
`crate::menu` a la biblioteca autónoma).
README actualizado; nueva compilación pendiente.

### Decoración independiente del proveedor (CI 2026-11-18)

La importación anterior de `../../../src/control_style.rs`
se reemplaza por el módulo aislado
`src/provider_control_style.rs` para los mensajes de
REGISTER/recuperación y fallback. Evita importar el
registro de comandos y SQLite del núcleo. El menú
dinámico recibido por IPC ya viene formateado; no
se envuelve otra vez. Se aplicaron los diffs
Rustfmt del verificador. Probar los mensajes
NativeFlow reales en Android/iOS tras confirmar
CI, conservando solo tres botones de lectura.
No se copió ningún secreto ni se modificaron sesiones.

## Menú modular y botones nativos, 2026-11-18 (alpha)

El controlador usa un catálogo Rust por carpeta
y por comando. Permite `menu`, `menu set`,
`menu antis`, `menu antis 2`, `menu system`,
`menu ai` y demás categorías. Estilos
`classic`/`none`, idiomas `es`/`en`,
preferencias persistidas fuera de la sesión.

`MATTER_WA_NATIVE_BUTTONS=1` intenta enviar
en un solo mensaje Estado, Menú y Antis.
Su respuesta se convierte exclusivamente en
`=/>status`, `=/>menu` o `=/>menu antis`,
y se verifica por HMAC, origen, antirreplay
y política de chat. Las llamadas a submenús
de otros usuarios siguen siendo solo lectura.
El pie usa marcas nativas generadas desde
HTML breve validado; no HTML/CSS arbitrario.
`MATTER_WA_RICH_HTML=1` es experimental,
self-chat y no demuestra visualización Android/iOS.

No se importan claves, no se ejecuta sandbox,
no se activan servicios ni se conceden rangos.
Ver `../../docs/MENU_CATALOG.md`.
README del proveedor revisado.

## 2026-11-18 — Matter AI: solo broker Vault y permiso del titular

El controlador `matter` sigue limitado al propietario autenticado
en su chat privado PN/LID, con HMAC y antirreplay. No se reutilizan
claves de IA obtenidas de Aura ni otros repositorios; Base-Matter
no acepta claves API directas. Solo un gateway Unix con ruta y UID
esperado configurados puede responder, tras verificar permisos y aplicar
la política del titular en cada solicitud. El cliente rechaza respuestas
sin `policy_applied=true`, `grant_id` y revisión positiva.
El gateway productivo **aún no existe**; no anunciar capacidad
de IA en vivo como verificada. Ver `../../docs/MATTER_AI.md`.

## Matter AI por WhatsApp: comando privado de solo análisis (alpha)

El propietario puede usar `matter status`, `matter ask` y
`matter plan` a través del puente firmado existente. El
proveedor solo acepta `matter` en su lista privada
**cuando el mensaje procede de la cuenta autenticada**;
no existe ruta pública ni de grupos. El controlador
aplica además permisos de propietario y aislamiento de
la ruta de extensiones. La consulta IA no es ejecución
de shell ni modificación de GitHub.

El controlador consume claves de entorno autorizadas
(Claude preferente, Venice después, OpenAI alternativo),
no claves de Aura Reed ni archivos de sesiones.
El plazo de entrega IPC se amplía solo para consultas
`matter`. Otras operaciones conservan 4 segundos.
No hay garantía de respuesta externa hasta probar
cuentas válidas. Ver `../../docs/MATTER_AI.md`.

## Normalización Rustfmt del transporte RichResponse (alpha)

La comprobación inicial del commit `d8c3857` se detuvo
en el formato de la bandera `MATTER_WA_RICH_HTML` y
tres aserciones del test `rich_transport`. Ajustado
conforme al diff de CI. Sin modificación funcional,
sin habilitar envío por defecto. Revalidar el nuevo SHA.

## Ensayo RichResponse opt-in únicamente en el chat propio (alpha)

Se conserva `rich_preview.rs` y sus comandos offline
`--rich-preview-html` / `--rich-preview-json`. El módulo
independiente `rich_transport.rs` empaqueta ese JSON como
`AIRichResponseMessage` nativo, sin alterar los datos ni
afirmar ser Meta AI. Sólo se intenta entregar tras recibir
`=/>menu` del propietario autenticado en su self-chat y con
`MATTER_WA_RICH_HTML=1`. En grupos o mensajes de terceros
esta opción no se usa, aun si está activada. Default apagado.

El proveedor hace fallback a NativeFlow o al menú original si
`send_message` rechaza la operación; no detecta si el cliente
ignora un envío aceptado. Es un **ensayo**, no HTML WhatsApp
garantizado. El modo de prueba no ejecuta scripts de `xd.zip`,
no atribuye origen Meta AI, no permite botones administrativos
ni abre una segunda sesión. La vista previa local conserva sus
botones deshabilitados; los únicos controles con interacción
real continúan siendo los tres NativeFlow de solo lectura.
Detalles: `../../docs/WHATSAPP_RICH_HTML_COMPAT.md`.

## Ajuste de formato del prototipo Rich (alpha)

El verificador de `2dfc214` rechazó el formato de dos índices
JSON y una aserción en `tests/rich_preview.rs`. Corregido conforme
al log de Rustfmt, sin alterar lógica ni habilitar envío Rich.
Nueva ejecución de verificación obligatoria.

## Compatibilidad experimental del material Rich HTML (2026-11-18)

Analizados ejemplos externos en Go/JavaScript/HTML: el contenedor
`embedded_screens`/`FOAIDNixelButtonSheets` puede estudiarse, pero
el envío proviene de un `botForwardedMessage` que aparenta tener
identidad `0@bot`. Ese método **no** se trasplanta a Base-Matter ni
se afirma que WhatsApp acepte HTML de terceros.

Ahora `rich_preview.rs` genera HTML estático y un JSON de compatibilidad
sin abrir el proveedor, con el menú original y sin JavaScript:
```bash
cargo run --manifest-path providers/whatsapp-rust/Cargo.toml -- --rich-preview-html > matter-preview.html
cargo run --manifest-path providers/whatsapp-rust/Cargo.toml -- --rich-preview-json > matter-preview.json
```
El proceso debe iniciarse desde la raíz del repositorio. Las salidas
son solamente **archivos locales de prueba**, no mensajes WhatsApp.
Los botones del HTML se muestran deshabilitados deliberadamente; los
únicos botones nativos reales siguen siendo NativeFlow opt-in. No se
usa Node, Baileys, spoof de Meta AI ni una segunda sesión.
Véase [análisis de compatibilidad](../../docs/WHATSAPP_RICH_HTML_COMPAT.md).

## Ajuste de orden de imports NativeFlow (alpha)

El paso rustfmt en `3178e32` exigió ordenar el alias
`NativeResponse` y `wa`. Corregido siguiendo el diff de CI.
Sin modificaciones de lógica y sujeto a CI completo en el nuevo SHA.

## Ajuste de Clippy para el experimento NativeFlow (alpha)

En la revisión `d6dd6f1`, las pruebas nativas y el convertidor de
HTML breve terminaron en verde. Clippy detectó exclusivamente el
`..Default::default()` redundante del cuerpo protobuf y un patrón
de coincidencia irrefutable. Corregidos sin modificar botones,
lógica de firma HMAC o interfaz de sesiones. Nuevas pruebas CI
pendientes antes de ensayar Android/iOS.

## Corrección de formato NativeFlow (alpha, 2026-11-18)

El primer workflow del experimento `d532ef2` indicó solo diferencias
de Rustfmt en los archivos nuevos y el `main.rs` modificado; aún no
alcanzaba las pruebas. Se aplicaron las diferencias del log y se evitó
un patrón potencialmente irrefutable en el lector de respuestas.
Nuevo CI es obligatorio para verificar firma, botones y parser.

## Experimento NativeFlow y HTML de estilo (alpha, 2026-11-18)

Para una prueba controlada de botones, ejecutar en la terminal del
operador `export MATTER_WA_NATIVE_BUTTONS=1` y luego
`cargo run --locked` desde la raíz. Por defecto `=/>menu`
continúa enviándose en texto, con la plantilla original.
Con la opción activada, el proveedor intenta enviar un único
`InteractiveMessage.NativeFlowMessage` con el cuerpo exacto del menú
más tres `quick_reply`: Estado, Menú y Créditos. Los botones tienen
IDs específicos solo de lectura; una selección se procesa por el mismo
puente firmado HMAC, autorización de chat y protección antirreplay.
Un error de envío provoca fallback al texto. No se garantiza que
Android/iOS rendericen el proto sin un ensayo real.

`src/ui.rs` permite HTML **solo como lenguaje abreviado de estilo**:
`b/strong`, `i/em`, `s`, `code`, `br`, algunas entidades,
sin atributos ni JavaScript, convertido a marcado de WhatsApp.
No se crea un WebView, no se usa el tipo experimental
`FOAHtmlPrimitiveDemoDONOTUSE`. La discriminación Android/iPhone
no está implementada: no existe una identidad de SO autenticada en
los eventos que recibe esta versión del proveedor.
Ninguna función nueva envía mensajes espontáneos, cambia roles o
activa el bot por defecto. Mantener respaldo de sesión y bloqueo.

## 2026-11-18 — Corrección de la prueba de `status --global` público

El CI del proveedor sobre `ce57169` compiló y pasó 15/16 pruebas
`tests/provider.rs`; la restante aún esperaba ignorar `status --global`
de un lector externo. Ahora esa consulta se firma pero el controlador
siempre devuelve una negativa de privacidad; nunca expone políticas
ni acepta cambios del socket. Actualizada la prueba y solicitada
verificación nueva sobre el SHA exacto.

## 2026-11-18 — Menú original y lectura contextual / global (alpha)

El proveedor `whatsapp-rust` conserva el mensaje original de Matterssmith en
`=/>menu`, incluido el crédito final, sin una segunda caja de control.
`=/>status` de un participante externo se firma para la lectura del chat
efectivo, aun con el bot apagado. `=/>status --global` del propietario funciona
desde el chat propio y grupos autenticados, con las políticas reales.
Cuando un tercero pide `--global`, se firma únicamente una consulta inocua
y el controlador responde con una negativa informativa; no entrega datos de
socket ni admite mutaciones. Los cambios de estado siguen siendo owner-only.
La vinculación sigue por pairing code, con procesos Rust independientes y sin
modificar bases de sesión. Son correcciones sujetas a CI y prueba WhatsApp real.

# WhatsApp-Matter: proveedor Rust independiente

El proveedor original es whatsapp-rust, ejecutado en un **proceso Rust
separado** del núcleo Base-Matter. No usa Node ni Baileys ni Bot-Matter.

La separación responde a versiones incompatibles de libsqlite3-sys entre
la biblioteca whatsapp-rust 0.7.0 y las dependencias actuales de Base.
Ambos procesos siguen escritos en Rust y se comunican por un canal TCP
en 127.0.0.1 protegido con HMAC-SHA256 de 256 bits.

## Verificación del proveedor Rust (alpha; 2026-11-18)

GitHub Actions `44367046939` aprobó la verificación independiente del
proveedor para `7de9a0d`, tras la corrección de formato del marco compartido.
Queda una advertencia de Clippy del núcleo en las pruebas de antis, corregida
en el siguiente checkpoint. Este resultado no prueba un reinicio real de
sesión WhatsApp ni una moderación automática activa.

## Corrección de verificación Rust (alpha; 2026-11-18)

El último CI (`6142c6c`) falló por formato de la plantilla compartida
`src/control_style.rs` en la comprobación del proveedor, no por errores
verificados de emparejamiento. El mismo cambio también corrige el formato
del núcleo y la prueba desactualizada de créditos. Falta nuevo CI sobre el
SHA exacto y prueba real de WhatsApp antes de declarar esta revisión estable.

## Clippy: segundo consumidor del snapshot (alpha)

En la nueva verificación de `alpha`, Clippy del núcleo se detuvo
en `tests/whatsapp_native.rs:82`, que aún accedía al campo
posicional `.0` de `inspect_bot_defaults`. Se reemplazó por
`.socket_powered`, sin alterar la autenticación, las respuestas
tematizadas o la vinculación por pairing code. La suite del proveedor
no falló; simplemente no llegó a ejecutarse en ese intento porque
`set -e` detuvo el bloque en Clippy. Falta repetir ambas suites.

## Menú compacto y comando de créditos (alpha; pruebas reales pendientes)

El proveedor Rust permite `=/>credits`, `=/>creditos` y
`=/>créditos` con la misma autenticación y verificación HMAC que
los demás controles. Para remitentes ajenos solo se permite lectura.
Los mensajes normales heredan el marco común corto, sin un footer
automático de autor ni nombres duplicados. La atribución completa
permanece en el código y se muestra exclusivamente mediante
el comando de créditos.

Referencias estudiadas: Aura-Bot-2.0 usa mensajes enriquecidos,
miniaturas y, en comandos concretos, una vista HTML experimental
(`FOAHtmlPrimitiveDemoDONOTUSE`). El ZIP YoSoyYo incluye
`nativeFlowMessage` y botones `quick_reply` / `cta_url`.
Ninguno ofrece una detección fiable del sistema operativo del destinatario.
**No se envían aún botones HTML ni versiones forzadas Android/iOS**
desde este proveedor; implementar eso sin detección fiable rompería
el requisito de no mostrar una variante equivocada. El transporte
Rust, el pairing code, los archivos SQLite y el control de sesión
permanecen sin modificaciones.

## Ajuste posterior a Clippy del núcleo (alpha)

La nueva compilación de Clippy encontró una referencia posicional
antigua en la prueba del puente `tests/whatsapp_bridge.rs`.
La referencia fue actualizada a `BotDefaultPolicies::socket_powered`;
el proveedor WhatsApp Rust no requiere cambios de protocolo ni
de sesión para este ajuste. La suite independiente de 21 pruebas
estaba aprobada en la ejecución anterior, no en el nuevo SHA.
Las verificaciones estrictas restantes deben repetirse.

## Resultado comunicado del proveedor Rust — 2026-11-18

El operador ejecutó la suite independiente del proveedor después
de corregir el ingreso nativo. Pasaron **21 pruebas**:
una del ejecutable principal, dieciséis de `tests/provider.rs`,
tres de recuperación y una de REGISTER. El bloqueo restante
reportado fue `clippy::type_complexity` del **núcleo Base**,
no una prueba fallida del proveedor.

La corrección sustituye la tupla de seis políticas por una
estructura Rust nombrada, sin alterar el protocolo HMAC,
las sesiones SQLite, pairing code ni las plantillas WhatsApp.
Falta la nueva ejecución de Clippy de ambos paquetes.

## Revisión del ingreso nativo (alpha; suite completa pendiente)

La suite Base identificó un caso donde `=/>on` retirado y
demasiado largo se ignoraba antes de validar el límite de entrada.
Se reordenó la validación en `src/whatsapp_native.rs`. El
proveedor independiente continúa firmando solo los comandos
autorizados y no recupera los alias antiguos. La prueba nueva
comprueba también un `=/>bot on` excesivo y que un rechazo
no consuma el identificador de replay. No modificar sesiones
WhatsApp, archivos SQLite ni el `Cargo.lock` local.
La nueva ejecución del proveedor aún no está confirmada.

## Validación por fases en Debian (alpha)

La última suite reportada del **núcleo** se detuvo por
una aserción antigua de `tests/control_style.rs`, que
buscaba un `bot_id` interno en el menú compacto.
La expectativa ya fue corregida en GitHub, sin añadir
dicho identificador a las respuestas. El proveedor Rust
no llegó a ejecutarse en ese lote: **no asumir su
aprobación para el nuevo commit**.

El operador tiene modificaciones locales generadas por
`cargo fmt` y un `providers/whatsapp-rust/Cargo.lock`
sin seguimiento. Preservar ambos al sincronizar; el lock
no contiene una sesión WhatsApp y no se debe borrar como
forma de corregir esta regresión. Repetir las pruebas del
proveedor después de confirmar la suite central.

## Respuestas propias y plantilla seleccionada de cada chat (alpha)

La respuesta de `WhatsAppControlIpc` llega ya renderizada con la
plantilla del chat verificado. El proceso `whatsapp-rust` conserva
esa respuesta y **no impone encima el diseño predeterminado**.
Para informes generados directamente en el proveedor (REGISTER,
recuperación comunitaria y respuestas de reserva), utiliza
`render_provider_control`, que carga exactamente el mismo
archivo de plantilla y nunca otra variante escrita dentro de
los handlers.

La prueba nueva `tests/whatsapp_theme_scopes.rs` verifica en
el núcleo la salida por grupo, con créditos y saltos de línea
reales. Falta probarlo con el binario WhatsApp conectado.
No se cambian SQLite, pairing code, claves ni sesiones.

## Contrato WhatsApp actual: un solo comando y una sola plantilla (alpha)

Los comandos `=/>on/off` se han retirado del proveedor y del puente
firmado. Usar solo `=/>bot on/off` en el chat autorizado; habilitar
despierta el socket sin un encendido adicional. Para predeterminados
usar `--global` o los alcances `group/private/channel/community`
en el chat propio. La variante `community` corresponde a avisos
comunitarios: no se aplica a grupos ordinarios sin metadatos de tipo.

El proveedor **reutiliza `src/control_style.rs`** para decorar también
respuestas directas REGISTER, recuperación y vistas previas. Ambos
procesos comparten las plantillas editables de la raíz:
`templates/matterssmith-control.txt` y
`templates/matterssmith-menu.txt`. No copiar diseños dentro de los
manejadores. El menú único no tiene subcategorías. El salto de línea
es un carácter real, jamás texto literal `\n`.

Esta revisión está publicada solo en alpha, pendiente de pruebas
reales en Debian; los 20 tests del proveedor informados previamente
corresponden al commit anterior `dfd9dab`, no certifican este cambio.

## Distinción en la suite Base: consulta grupal con bot apagado

La regresión siguiente, tras aprobar `control_ipc`, estaba
en `tests/group_control.rs`: la prueba rechazaba
`=/>status` desde un grupo del propietario. Se ajustó
la expectativa para exigir estado grupal `PoweredOff`
cuando el socket está apagado, sin permitir operaciones
administrativas de un participante externo. El proveedor
WhatsApp no necesita cambios para este caso; su suite
independiente ya había aprobado 20 pruebas.
La suite Base debe volver a ejecutarse para validar la
corrección antes de continuar con WhatsApp real.

## Resultado de compilación del proveedor (Debian, 2026-11-18)

El operador informó `cargo test --manifest-path
providers/whatsapp-rust/Cargo.toml --all-targets` con
**20 pruebas aprobadas y ninguna fallida**: una prueba del
binario principal, quince del protocolo/proveedor, tres de
recuperación y una de REGISTER. El defecto que persiste es una
expectativa en la suite **Base**, no en este proveedor:
`=/>bot private off` debe deshabilitar privados externos,
sin afectar al chat propio del dueño. La prueba IPC ya fue
alineada con esa separación, pero necesita ejecución Debian.
No eliminar `Cargo.lock`, sesiones, llaves ni caché compilada.

## Lectura pública autenticada sin rangos ni control de socket (alpha; pruebas pendientes)

En un grupo, el proveedor puede firmar `status/menu/bot` procedentes
de otros participantes usando el JID PN/LID real del remitente. En
un chat privado externo exige que el JID del chat coincida exactamente
con el del remitente. La sesión conectada debe coincidir además
con la identidad emparejada del socket; el HMAC vincula al actor,
chat, comando, mensaje y tiempo. En la capa Base una ruta especial
devuelve solo estado o menú, sin invocar extensiones generales.

No se acepta `=/>status --global`, `--debug`, `=/>on`,
`=/>bot on`, cambios de configuración ni `=/>set role`
desde terceros. Cuando está deshabilitado el bot, el estado
sigue respondiendo y el menú no responde. No asumir
funcionamiento de canales o privado PN/LID cruzado aún:
esas identidades necesitan pruebas verificadas del proveedor.
El chat propio usa política `self`, no el predeterminado
`private` de chats externos. No se modificaron sesiones,
llaves, ramas beta/stable ni repositorios de roles.

## Contratos recientes de chat, estado y silencio (alpha; build Debian pendiente)

La sesión real ya respondió comandos de propietario dentro de un
grupo, pero dejó claro que `PoweredOff` significa socket apagado,
no fallo de la política grupal. En alpha se incorporaron
`=/>status` (contexto actual), `=/>status --global`
(predeterminados), `=/>bot on/off/inherit` (chat),
`=/>bot group/private/channel on/off/inherit` (tipos) y
`=/>bot --global on/off/inherit` (socket).

`=/>menu` / `=/>menú` muestra un diseño Matterssmith propio por
categorías al habilitar el bot. Los comandos normales, el menú y
la ayuda de `bot` no responden cuando la política está apagada;
`status` sigue disponible. `=/>help` y `=/>h` se retiraron.
El proveedor acepta un acuse `ignored` auténtico y no lo responde
en WhatsApp. El límite acotado de respuestas IPC se amplió para
incluir el menú decorativo y `--debug` sin cargar datos secretos.

**Límite de seguridad:** `from_me` y PN/LID solo autorizan al
propietario vinculado; no abrir mensajes de otros usuarios
suplantando su identidad. El soporte de canales y privados
externos exige un nuevo ingreso autenticado. La conexión con
Vault-Matter y `selfmod` aún no están instalados.
Ver `../../docs/BOT_POLICY_V2.md`.

## Pruebas offline del binario y recuperación (alpha)

La suite del proveedor se ejecuta de forma independiente al núcleo:

```bash
cargo test --manifest-path providers/whatsapp-rust/Cargo.toml --all-targets
```

`tests/recovery.rs` resuelve el ejecutable
`whatsapp-matter-rust-provider` mediante
`option_env!("CARGO_BIN_EXE_whatsapp-matter-rust-provider")` y,
en su ausencia, `std::env::var_os` durante la prueba. La prueba
invoca realmente `--recovery-verify`, rechaza la clave equivocada
y rutas inseguras, y verifica que la sesión SQLite no se abre.
No crea una nueva vinculación de WhatsApp ni borra las sesiones.

`tests/provider.rs` utiliza el mismo mecanismo para `--check`.
Su prueba del JID `@lid` usa `=/>status` en privado, y confirma
que `=/>menu` se ignora en ese ámbito. Pruebas de compilación
y ejecución en el servidor: pendientes tras esta corrección.

## Fuente única de permisos (Vault-Matter)

Los rangos de socket y grupo no se pueden escribir localmente en
Base-Matter. El usuario dirige altas, cambios y revocaciones de
rango a un canal **propio y autenticado de Vault-Matter** en WhatsApp,
Telegram o Discord. No se aceptan mensajes reenviados o un nombre
visible como prueba de autorización. El servicio Vault es la única
autoridad, autentica cada actor, cifra las asignaciones y mantiene la
persistencia del repositorio privado. Base consulta permisos como
lector de solo lectura y deniega todo permiso no confirmado.

**Hasta que exista el servicio y el verificador:**
`=/>set role`, `=/>unset role`, `=/>role` y
`=/>group role` se rechazan sin modificar SQLite. El
`=/>roles` privado indica que falta la conexión con Vault.
La identidad propietaria original puede seguir apagando el socket
sin depender de un rango delegable.

## Archivo histórico — División privado/grupo anterior (sustituida por BOT_POLICY_V2)

El control remoto ya respondió en WhatsApp al propietario con `=/>status`
y `=/>on`. El chat privado propio se reserva desde ahora a los
ajustes globales del socket; `menu/help/start/add/add++` son
comandos de ámbito grupal. El proveedor verifica `from_me`, el
PN/LID real del cliente, el chat `@g.us` y el remitente propio.
El destino grupal queda firmado en el HMAC; el controlador determina
el grupo por ese dato autenticado, sin aceptar IDs en texto como
prueba de permisos. El controlador del socket no arranca Bot-Matter.
`=/>group on/off/inherit/status` afecta únicamente a ese grupo;
el encendido general `=/>on/off` sigue siendo privado.

`=/>set role <PN/LID> <rango>` es control privado del socket.
`=/>group role <PN/LID> <rango>` es una asignación específica
del grupo, todavía no una delegación de ejecución. Los rangos no
pertenecen al escalafón de administración del proyecto. `user`
es implícito; el almacén local persiste solamente asignaciones
no predeterminadas. Falta la publicación cifrada autenticada en
Vault-Matter y su recuperación; no asumir persistencia remota.
Las pruebas de grupos reales siguen pendientes en esta revisión.

## Corrección de identidad PN/LID autenticada (alpha)

La sesión vinculada se identifica ahora mediante `ctx.client.pn()` y
`ctx.client.lid()` de whatsapp-rust 0.7.0. Solo se aceptan mensajes
`=/>` propios en chat privado cuya identidad de chat y remitente
coincidan con PN o LID verificados del cliente, con JIDs de dispositivo
normalizados. El PN autenticado debe coincidir con el número configurado
y el JID del propietario debe ser uno de esos identificadores. El puente
HMAC conserva el JID canónico del propietario. Siguen denegados chats
ajenos, grupos, mensajes entrantes y prefijos antiguos `#ping`.
No es necesario regenerar la sesión ni el archivo `runtime.env`.
La prueba de recepción y respuesta real sigue pendiente.

## Ejecución en una sola terminal (alpha)

Para ahorrar espacio en Debian, utilizar **un solo clon Git** y exportar
`CARGO_TARGET_DIR` como ruta absoluta compartida antes de ejecutar
`cargo run`. El lanzador de `alpha` respeta la ruta tanto para
`whatsapp-matter-control` como para este proveedor Rust independiente.
Los artefactos compartidos no contienen las sesiones SQLite. Las sesiones,
claves y configuración son locales y particulares de cada rama. Cambiar de
rama requiere detener antes cualquier proceso WhatsApp activo. `stable` y
`beta` conservan sus propias verificaciones; este lanzador no ha sido
promovido y aún requiere prueba en un dispositivo real.


La rama `alpha` incorpora `cargo run` desde la raíz de Base-Matter.
El ejecutable `base-matter` inicia el controlador y este proveedor
Rust como procesos independientes bajo una sola sesión de terminal,
para evitar conflictos `libsqlite3-sys`. No usa Node ni QR.

Cada ejecución utiliza una ruta privada y un puerto reservados según
la rama Git: `alpha` usa 39821, `beta` 39822 y `stable` 39823.
El lanzador solo existe por ahora en `alpha`; las ramas `beta` y
`stable` requieren promoción y verificación por separado.
Sin configuración previa pregunta únicamente el teléfono y conserva
su SQLite y secretos para el siguiente arranque. Un JID derivado de
un número no garantiza coincidencia con el LID autenticado y los
comandos propios permanecen denegados ante cualquier discrepancia.
Una conexión en pantalla no demuestra que se haya aplicado `=/>status`.

## Parámetros locales

- MATTER_WA_PHONE: número para pairing code, 8–15 dígitos internacionales
- MATTER_WA_OWNER_JID: JID propio privado (dominio s.whatsapp.net o lid)
- MATTER_WA_SOCKET_ID: socket registrado en Base-Matter
- MATTER_WA_OWNER_ID: dueño registrado en Base-Matter
- MATTER_WA_SESSION_DB: ruta absoluta al SQLite privado de whatsapp-rust
- MATTER_WA_BRIDGE_ADDR: 127.0.0.1:puerto del controlador de Base
- MATTER_WA_BRIDGE_KEY_HEX: clave privada HMAC de 32 bytes, 64 hex

La carpeta del SQLite debe estar protegida para el usuario del servidor.
El propietario de la sesión se bloquea mediante archivo owner.lock para
impedir procesos duplicados. No se elimina la sesión de WhatsApp.
Realizar un respaldo SQLite consistente antes de actualizar una sesión
existente. Probar primero un número secundario.

Ejecución desde el repositorio, con las variables de entorno cargadas:

    cargo run --manifest-path providers/whatsapp-rust/Cargo.toml

El enlace se realiza exclusivamente mediante código de vinculación mostrado
en la terminal del operador. El proveedor no registra callback QR ni muestra
códigos QR; whatsapp-rust puede crear un desafío QR interno en paralelo,
sin que este proceso lo exponga.

El proceso valida el mensaje de la cuenta propia, JID de chat y remitente
exactos, fromMe y chat no grupal. Firma únicamente mensajes canónicos
=/> con timestamp reciente y los entrega al controlador Rust de Base.
Solo responde con el acuse genérico `applied` al chat propio autenticado;
no responde ping ni ejecuta Bot-Matter.

## Límite de estabilidad

El proveedor requiere un proceso receptor Base-Matter configurado con el
mismo secreto y socket, una sesión real vinculada mediante pairing code,
revisión de compatibilidad SQLite y pruebas de revocación y reconexión.
No se puede afirmar estabilidad por aprobar únicamente pruebas offline.

Biblioteca upstream: oxidezap/whatsapp-rust, versión 0.7.0 (tercero, no
oficial de WhatsApp). Conservar licencias y atribuciones upstream.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## Preflight Rust sin conexión

Con el entorno configurado ejecutar `cargo run --manifest-path providers/whatsapp-rust/Cargo.toml -- --check`. Comprueba el número internacional, JID propio, claves y ruta privada sin abrir sesiones ni conectarse a WhatsApp. El controlador independiente de Base ofrece `cargo run --locked --bin whatsapp-matter-control -- --check`. No publicar variables de entorno ni archivos SQLite.

## Guía de prueba real

La [guía de dos terminales y emparejamiento](../../docs/WHATSAPP_TRIAL.md) usa únicamente Rust. El script `scripts/whatsapp-test-setup.sh` prepara claves privadas diferentes sin sobrescribir sesiones. El modo `--check` valida el entorno antes de abrir una sesión. Nunca compartas el código de vinculación ni los secretos.

## Propiedad de sesión segura

En sistemas Unix, el archivo `owner.lock` se crea con permisos `0600` en la llamada de apertura, sin una ventana inicial con permisos de grupo/otros. La carpeta de sesión debe seguir siendo privada (`0700`). Un bloqueo residual exige recuperación manual verificada; no eliminarlo automáticamente ni borrar `session.db`.

## Resultados del control global

Al enviar `=/>`, el proveedor diferencia el efecto nuevo `applied` del
replay `duplicate` y los rechazos. Un resultado `duplicate` nunca se anuncia
como ejecución nueva; los acuses malformados o desconocidos se rechazan.
El control solo intenta una confirmación breve cuando se ejecuta una
operación; no devuelve todavía los detalles del estado. Requiere prueba
real autorizada antes de promoverlo a estable.

## Reinicio seguro tras una caída

En Rust 1.89 o posterior, la exclusión del archivo `owner.lock` utiliza un
bloqueo de sistema operativo y un marcador v2 permanente. Tras un cierre o
caída del proceso, el bloqueo se libera y el mismo archivo puede utilizarse
de nuevo sin tocar `session.db`. Si hay otro proceso activo, se deniega
el acceso. Los antiguos locks con PID (v1) se rechazan por seguridad hasta
la revisión manual del operador: la migración nunca borra locks ni sesiones.
`--check` verifica la disponibilidad del archivo de modo de solo inspección.

## Orden de arranque comprobado

Al ejecutar el proveedor para vincular WhatsApp, primero comprueba que el
receptor Base-Matter escucha en la dirección 127.0.0.1 configurada; si no,
aborta antes de abrir la sesión SQLite o solicitar un código. La comprobación
solo establece disponibilidad TCP, **no autentica el controlador**: el HMAC
protege los comandos. El modo `--check` sigue sin red ni escrituras.

## Confirmación en el chat privado

Tras un acuse `applied` autenticado, el proveedor intenta enviar al
chat propio «Base-Matter: solicitud procesada.». Los duplicados y errores no
generan respuestas. No se envía a grupos, terceros ni mensajes que no tengan
el prefijo `=/>` desde el JID autenticado. Es un aviso genérico: no implica
que `status` ya tenga salida detallada, ni sustituye las pruebas de entrega
con un teléfono autorizado.

## Comunidad REGISTER (alpha)

`=/>start` se reserva para inventario y verificación de comunidades
gestionadas por la sesión de WhatsApp. El diseño en
[`docs/COMMUNITY_REGISTER.md`](../../docs/COMMUNITY_REGISTER.md)
requiere verificar el rol admin y los identificadores exactos de subgrupo,
y distingue vista previa de operaciones con cambios. Ninguna actualización
administrativa se debe asumir completa por aprobar CI offline.

## Encabezado oficial REGISTER

La función de detección solo reconoce la primera línea exacta
`MATTER:OFFICIAL:register:<community-jid>`, validando formato de JID
comunitario. Las coincidencias parciales, colocadas en otra línea o con
ID de otra comunidad se rechazan. La descripción **no es autenticación**;
además se deben verificar metadatos de pertenencia y rol de administrador.

## Escaneo de comunidades reales con =/>start (alpha)

Tras pasar HMAC y permisos en Base, el callback de `whatsapp-rust` utiliza
`Community::get_participating`, `get_subgroups` y
`query_linked_group`. El escaneo verifica el admin del propio JID PN/LID,
pertenencia del subgrupo a la comunidad y encabezado REGISTER exacto.
Responde en el chat propio y no modifica grupos ni roles. Limita a 12
comunidades, 64 subgrupos por comunidad y 20 segundos por solicitud.
No se puede declarar probado en red sin un teléfono vinculado.

## Respuesta de comandos globales desde Base

El proveedor admite el acuse opcional `message` de una operación
`applied` para mostrar menús o estados en el self-chat. Exige menos
de 761 bytes, texto UTF-8 válido, sin NUL ni controles salvo saltos de
línea. Los `duplicate`, `denied` y campos inesperados no contienen
respuestas. La verificación no equivale a privilegio de administración.

## Ayuda y menú del controlador Base (alpha)

Los comandos `=/>menu`, `=/>help` y `=/>h` devuelven en el chat
privado propio el formato autorizado por Base-Matter. El texto se valida
y se limita a 760 bytes; los mensajes duplicados o no autorizados no
generan respuesta. El proveedor no interpreta comandos externos
`#matter` ni importa sesiones de bots ajenos.

## Menú simple para pruebas beta

El menú de control de Base-Matter es intencionalmente plano. `=/>menu`, `=/>help` y `=/>h` no incluyen una estética atribuida a otro bot sin una muestra observada y aprobada. Todas las respuestas siguen exigiendo validación del dueño del socket.

## Prueba desde la nueva rama beta

La guía `docs/WHATSAPP_TRIAL.md` corresponde ahora a `beta`. `=/>menu`, `=/>help` y `=/>h` muestran comandos del núcleo sin diseño preestablecido, y `=/>start` solo consulta comunidades. No copiar menús de otras cuentas sin una fuente verificable.

## Respaldo de comunidades `=/>add++` (fase alfa)

El controlador Base-Matter autoriza un inventario de recuperación, pero todavía no incorpora participantes ni crea un grupo espejo. El respaldo cifrado local y la lectura real del proveedor se integran de forma separada, conservando permisos, privacidad, exclusión de duplicados y protección contra nuevas suspensiones. La prueba actual de WhatsApp desde `beta` no debe declararse capaz de migrar usuarios automáticamente.

## Inventario de recuperación WhatsApp

Después de que Base-Matter autorice `=/>add++`, se consultan las comunidades donde la sesión actual figure como administradora. Se muestran conteos aproximados de integrantes de subgrupos (sin publicar identidades), el ID de comunidad y el nombre sugerido del grupo de respaldo. No se añaden participantes; no se extraen miembros de comunidades suspendidas.

## Almacén local de instantáneas cifradas

El módulo `recovery` cifra registros de participantes y la identidad de la comunidad con XChaCha20-Poly1305, usando una clave independiente `MATTER_WA_RECOVERY_KEY_HEX`. Se crean archivos nuevos `community-recovery-*.enc.json` con permisos 0600 en la carpeta privada del SQLite; no se sobrescriben snapshots anteriores. Se exige revalidar el socket y el dueño al descifrar. Para un entorno ya configurado, ejecutar `bash scripts/whatsapp-test-setup.sh --enable-recovery` sin borrar la sesión. No confundir snapshot de miembros con exportación de credenciales de WhatsApp.

## Comando real de instantánea (alpha)

`=/>add++ snapshot <ID_COMUNIDAD@g.us>` solicita un respaldo de identidades PN/LID desde los grupos enlazados solo cuando la cuenta es admin de esa comunidad y WhatsApp permite consultarla. Se deduplican las identidades conocidas, se cifran usando la clave independiente y se escribe un archivo privado nuevo. La respuesta enviada al chat muestra el conteo y el nombre del archivo, no la lista. No se agrega nadie ni se eluden suspensiones o privacidad. Para habilitar la clave en una configuración existente: `bash scripts/whatsapp-test-setup.sh --enable-recovery`.

## Verificar el respaldo sin sesión activa

Con el mismo `runtime.env` y una copia del archivo cifrado, `cargo run --manifest-path providers/whatsapp-rust/Cargo.toml -- --recovery-verify NOMBRE.enc.json` valida clave, integridad, socket y dueño sin abrir la sesión de WhatsApp. El nombre debe corresponder a un archivo del directorio privado y no puede contener rutas. El resultado muestra solo el conteo, nunca PN/LID.

## Verificación del binario real

Se añadió prueba offline ejecutando el binario `--recovery-verify`, comprobando integridad, ausencia de sesión SQLite nueva, rechazo con clave errónea y no filtración de JIDs por stdout. No se ejecuta ninguna operación de grupo ni conexión de WhatsApp en CI.

## Nuevo =/>add (alpha)

El controlador Base-Matter valida la gramática de `=/>add` y responde con un acuse de vista previa, sin realizar altas, traslados ni cambios de permisos. `reply` exigirá metadatos de un mensaje real; `official` deberá resolverse desde un registro verificado de grupos/comunidades. El proveedor actual admite controles globales desde el chat propio y no se declara listo para operar `add user` dentro de grupos.
