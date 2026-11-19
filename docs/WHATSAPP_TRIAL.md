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

## Prueba de menú modular por WhatsApp (alpha 2026-11-18)

Detén antes la instancia anterior con Ctrl+C.
No ejecutes dos proveedores con el mismo almacén
de sesión. Mantén el emparejamiento por código.
Para habilitar botones y deshabilitar la hoja
HTML experimental:

```bash
export MATTER_WA_NATIVE_BUTTONS=1
export MATTER_WA_RICH_HTML=0
cargo run --locked
```

Desde tu chat propio autenticado, con bot activo:

```text
=/>status
=/>menu
=/>menu set
=/>menu antis
=/>menu antis 2
=/>menu system
=/>set style none
=/>menu set
=/>set lang en
=/>menu
=/>set style classic
=/>set lang es
=/>menu
=/>matter status
```

El modo none debe tener texto sin dibujos.
El idioma inglés debe traducir las descripciones.
Las preferencias sobreviven al reinicio.
El botón Antis intenta `=/>menu antis`,
y Estado y Menú ejecutan consultas equivalentes.
El control público solo funciona para menús
de chats encendidos; status sigue público
aunque el bot esté apagado. No probar
comandos `[pendiente]` ni introducir claves
en WhatsApp. El envío Rich/NativeFlow
aceptado no garantiza renderizado del cliente.

## Prueba experimental RichResponse en self-chat — alpha 2026-11-18

No cambia el menú predeterminado. `--rich-preview-html/json`
continúan sin conexión, sin sesiones y sin enviar WhatsApp.
Además, un ensayo separado puede **intentar** presentar la
hoja en el chat del propio propietario; su compatibilidad no
está demostrada y no funciona en grupos por diseño.

Con una sola instancia autorizada, tras detener la anterior y
actualizar `alpha` sin borrar la sesión:

```bash
cd /home/Sakura/Base-Matter
export MATTER_WA_RICH_HTML=1
export MATTER_WA_NATIVE_BUTTONS=1
cargo run --locked
```

Enviar `=/>menu` en el self-chat. El proveedor intenta una
respuesta RichResponse sin atribución falsa a Meta AI.
Si el transporte la rechaza, prueba NativeFlow y finalmente
texto. Si el servidor acepta el envío pero la app no lo
renderiza, puede no verse nada: registrar el comportamiento
del teléfono y revisar logs sin compartir secretos.
Los grupos siguen usando solo NativeFlow/texto. Para desactivar
la hoja: `export MATTER_WA_RICH_HTML=0` y reiniciar.
**No afirmar soporte Android/iOS/Web ni producción.**

## Vista previa HTML local del menú original (alpha, 2026-11-18)

Sin abrir sesión ni usar claves WhatsApp, el proveedor Rust permite
inspeccionar un diseño web móvil inspirado en los ejemplos Rich,
con el **menú original exacto** y controles deshabilitados:
```bash
cd /home/Sakura/Base-Matter
cargo run --manifest-path providers/whatsapp-rust/Cargo.toml -- --rich-preview-html > matter-preview.html
cargo run --manifest-path providers/whatsapp-rust/Cargo.toml -- --rich-preview-json > matter-preview.json
```
No enviar el JSON de prueba como mensaje firmado ni simular Meta AI.
El HTML estático no es el HTML que WhatsApp muestra dentro del chat.
Los botones de consulta NativeFlow se prueban separadamente mediante
la opción `MATTER_WA_NATIVE_BUTTONS=1`, con sesión propia y fallback
al diseño textual. Los metadatos de Android/iOS aún no están verificados.

## Menú con botones experimentales — alpha 2026-11-18

El menú original en texto es el valor predeterminado. Para probar la
variante con **tres botones nativos** (`Estado`, `Menú`, `Créditos`)
sin modificar credenciales ni SQLite, en la terminal activa como Sakura:

```bash
cd /home/Sakura/Base-Matter
export MATTER_WA_NATIVE_BUTTONS=1
cargo run --locked
```

El mensaje `=/>menu` se envía entonces como NativeFlow con el
cuerpo original del menú, o como texto si el proveedor rechaza el
envío. Las pulsaciones realizan solamente consultas. Verifica que los
botones aparecen en Android y iPhone, que la selección devuelve una
respuesta del controlador y que el menú de texto permanece disponible
desactivando la variable. La versión actual **no** sabe la plataforma
del remitente y no debe simular que filtra botones Android/iOS.

HTML de estilo limitado se transforma a formato de WhatsApp; no se
envía HTML web, CSS, JavaScript ni contenido ejecutable. No iniciar
dos instancias de la misma sesión para comparar teléfonos.

## Corrección vigente — menú original y estado global (alpha, 2026-11-18)

Desde el chat propio y desde un grupo en el que eres el remitente autenticado:
`=/>status` consulta solamente el estado efectivo de ese chat;
`=/>status --global` consulta la configuración del socket, incluso desde
grupo. Otros miembros del grupo y remitentes privados externos pueden enviar
`=/>status` cuando el bot está apagado, sin permisos administrativos.
Si solicitan `=/>status --global`, reciben una explicación de acceso
reservado, **no** las políticas del socket.

`=/>menu` vuelve a usar el diseño original completo con 12 comandos,
marco ornamental y crédito Matterssmith una sola vez. Esta documentación
prevalece sobre menciones históricas del menú abreviado, ausencia de créditos
o permisos de estado restringidos. Las acciones de cambiar políticas siguen
reservadas al dueño. El menú externo sigue silencioso con el bot apagado.
No se activa ninguna regla anti automáticamente.

## Actualización alpha — menú corto y créditos

La rama `alpha` muestra ahora un único marco corto de
control, sin dos títulos o footer de autor. La lista del menú
incluye `Prefijo global: =/>` solo una vez. Las entradas
`status [--global]` y `bot [ámbito] on|off|inherit`
son **notación abreviada**, no comandos sin prefijo.

En WhatsApp continúas enviando `=/>status`, `=/>status --global`,
`=/>bot on`, `=/>bot --global off` o `=/>bot group on`
según el alcance autorizado. Para ver la firma del proyecto,
envía `=/>credits`. El comando es de solo lectura.

Se examinó el estilo HTML experimental de Aura y los
botones del ZIP YoSoyYo. No se ha conectado ningún
envío de HTML ejecutable ni selección automática Android/iPhone
a WhatsApp-Matter Rust: requieren soporte real de cliente
y una señal de dispositivo fiable. El menú actual siempre
viaja como una sola respuesta segura de texto.

# Prueba WhatsApp Rust de Base-Matter — pairing code

## Contrato vigente: bot único y plantilla Matterssmith (alpha)

En WhatsApp **no existe `=/>on/off`**. `=/>bot on` habilita el chat
y despierta el socket automáticamente en una transacción. `=/>bot off`
deshabilita exclusivamente ese chat; no apaga otros grupos.
El menú global tiene una lista corta, sin categorías.

El diseño está separado del código en
`templates/matterssmith-control.txt` y `templates/matterssmith-menu.txt`.
Todas las respuestas del controlador y las vistas previas del
proveedor Rust pasan por el mismo renderizador. Se usan saltos de línea
reales, no secuencias literales `\n`.

**En tu propio chat:**

```text
=/>status
=/>bot on
=/>menu
=/>status --global
```

**En un solo grupo de pruebas:**

```text
=/>status
=/>bot on
=/>menu
=/>bot off
=/>status
=/>menu
=/>bot inherit
```

Tras `bot off`, `menu` no responde; `status` sigue funcionando.
Otro participante solo puede consultar `status/menu` si el chat está
habilitado; no debe poder modificar políticas.

**Desde el chat propio, por alcance:**

```text
=/>bot --global on
=/>bot group on
=/>bot private on
=/>bot channel on
=/>bot community on
```

`group` y `private` afectan a todos los chats de ese tipo: no usarlos
para probar un grupo individual. `community` guarda la política de
avisos comunitarios y `channel` la de canales, pero **no se aplican**
hasta recibir una clasificación verificada del proveedor.
Los rangos de Vault-Matter continúan bloqueados sin autoridad remota.

Las instrucciones históricas posteriores son referencias de diagnóstico;
no vuelvas a usar los comandos retirados `=/>on/off` ni menús con categorías.
## Procedimiento vigente en alpha: una sola terminal

```bash
cd /home/Sakura/Base-Matter
source "$HOME/.config/matterssmith/rust-env.sh"
git status --short --branch
git pull --ff-only origin alpha
cargo test --all-targets --locked
cargo test --manifest-path providers/whatsapp-rust/Cargo.toml --all-targets
cargo run
```

No volver a generar `runtime.env` ni borrar SQLite, llaves o locks
para probar las correcciones. El lanzador `cargo run` gestiona los
dos procesos Rust bajo una sola terminal y usa `CARGO_TARGET_DIR`.
Las instrucciones de dos terminales incluidas más abajo documentan
únicamente la ejecución manual anterior; no son necesarias en alpha.

**Control actual:** `=/>status`, `=/>bot on/off/inherit` y `=/>menu`
en el chat verificado; `=/>status --global`, `=/>bot --global`,
`=/>bot group/private/channel/community`, `=/>roles` y `=/>audit`
solo en el chat propio. `=/>set role` exige Vault-Matter remoto.
# Prueba WhatsApp Rust de Base-Matter — pairing code

Este procedimiento se puede ejecutar desde la rama `beta` con
una cuenta secundaria propia autorizada. Es una prueba, **no una
certificación de estabilidad**. Solo se usa Rust y vinculación por código.
No se muestra QR, no se usa Node/Baileys y no se inicia Bot-Matter.

## Preparación

Los dos procesos Rust deben funcionar en el MISMO servidor. Requisitos:
Rust/Cargo, OpenSSL, Git y las herramientas para compilar librerías Rust.
No compartas en GitHub ni en este chat los secretos ni el teléfono.

Comprueba antes tus cambios locales. Nunca sobrescribas una sesión activa:

```bash
cd /home/Sakura/Base-Matter
git status --short --branch
git fetch origin
git switch beta
git pull --ff-only origin beta
cargo check --all-targets --locked
cargo test --all-targets --locked
cargo test --manifest-path providers/whatsapp-rust/Cargo.toml --all-targets
```

Si hay cambios locales que quieras conservar, utiliza `git stash push -u`
y verifica su resultado ANTES de cambiar de rama. No uses un reset sobre
una copia que tenga trabajo sin publicar.

## Generar configuración privada una sola vez

```bash
bash scripts/whatsapp-test-setup.sh
```

El script solicitará el teléfono, JID propio y nombres internos. Genera
dos claves independientes para autenticación del canal y replay. Crea
un archivo local con permisos restringidos fuera del repositorio en:

`$HOME/.local/share/base-matter/whatsapp-test/runtime.env`

**No lo vuelvas a generar para una sesión ya enlazada.** El script se
niega a sobrescribir la configuración existente. Si no conoces el JID
real, `NUMERO@s.whatsapp.net` es solo una suposición: WhatsApp puede
usar identificadores `@lid`, y entonces el ingreso de controles será
rechazado hasta configurar la identidad autenticada correcta. No relajes
las comprobaciones para hacerlo funcionar.

## Terminal A: receptor Base-Matter

```bash
cd /home/Sakura/Base-Matter
source "$HOME/.local/share/base-matter/whatsapp-test/runtime.env"
cargo run --locked --bin whatsapp-matter-control -- --check
cargo run --locked --bin whatsapp-matter-control -- --init
cargo run --locked --bin whatsapp-matter-control -- --serve
```

`--check` valida sin crear SQLite ni conectar dispositivos.
`--init` se ejecuta **una sola vez** para registrar el socket y su dueño.
Si el socket ya existe, no repitas `--init`. `--serve` queda atendiendo
el puerto 127.0.0.1:39819 en esta terminal.

## Terminal B: proveedor Rust whatsapp-rust

```bash
cd /home/Sakura/Base-Matter
source "$HOME/.local/share/base-matter/whatsapp-test/runtime.env"
cargo run --manifest-path providers/whatsapp-rust/Cargo.toml -- --check
cargo run --manifest-path providers/whatsapp-rust/Cargo.toml
```

El proveedor solicita el **pairing code** a WhatsApp. Cuando aparezca
en su terminal, entra desde tu propio teléfono a:

WhatsApp → Dispositivos vinculados → Vincular dispositivo →
Vincular con número de teléfono.

Introduce el código que te muestra Rust. No publiques ese código.
El upstream podría crear internamente un desafío QR, pero el programa
no lo muestra ni ofrece vinculación QR.

## Comprobar comandos globales

Tras confirmar que la sesión está conectada, envía desde la misma
cuenta vinculada, al chat privado propio:

```text
=/>status
=/>menu
=/>help
=/>h
=/>start
=/>on
=/>status
=/>off
```

La aplicación responde en el chat propio con una lista textual sencilla
para `menu/help/h`, resultados informativos de `status/on/off`, y un
informe de comunidades para `start` cuando el control sea autorizado.
El resultado de `start` depende de metadatos reales de WhatsApp y
solo realiza lecturas; nunca crea, promueve ni modifica grupos.
También registra aceptación, duplicación o rechazo en la terminal.
No existe comando `ping` de producción ni se inicia Bot-Matter.

`=/>on` enciende el estado operativo del socket, pero no concede
una política para ejecutar comandos normales. Los grupos y terceros
no tienen autoridad para controlar ese socket.

## Bloqueos para promoción a stable

Validar emparejamiento real, JID/LID de la cuenta, comandos recibidos,
persistencia de SQLite, reinicio controlado, bloqueo por proceso, replay,
caducidad, revocación y compatibilidad upstream. La CI solo cubre pruebas
de código y transporte local sin dispositivo.

Si hay errores de vinculación o de archivo owner.lock, **no borres
el SQLite ni el lock a ciegas**. Comprueba primero que no exista otro
proceso usando la misma sesión. Haz una copia consistente antes de
cualquier migración.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## Resultados en terminal

El proveedor registra `applied` cuando el controlador ejecuta una operación
autorizada, `duplicate` cuando no se repite ningún efecto por replay, o un
rechazo genérico si el mensaje fue denegado o el controlador no respondió.
No representa estos tres estados como una misma ejecución aceptada. Tampoco
responde a WhatsApp; los resultados solo se observan en la terminal.

## Recuperación y archivos de bloqueo v2

Con la revisión v2, los procesos nuevos mantienen archivos lock permanentes
que incorporan un marcador de formato y usan el bloqueo nativo de archivos
del sistema operativo (Rust 1.89+). Si un proceso termina, otro puede
recuperar el mismo lock v2 después de comprobar que está libre, sin borrar
el SQLite y sin tener que borrar manualmente el lock. Si el lock está en
uso, se rechaza la segunda instancia.

Un archivo antiguo con PID o con contenido desconocido no se migra
automáticamente. Si aparece un error de lock v1, se debe confirmar que
el proceso anterior terminó antes de una migración controlada; no borrar
archivos a ciegas ni perder sesiones existentes.

## Comprobación del receptor al arrancar

Inicia primero el receptor Base-Matter (Terminal A). El proveedor de la
Terminal B comprueba que su puerto TCP está escuchando antes de abrir
sesión y solicitar pairing code. Si el receptor está caído, el proveedor
falla sin modificar el SQLite ni solicitar un código a WhatsApp. Esto
no demuestra por sí solo que la clave HMAC coincida, ni que el teléfono
esté conectado.

## Confirmación de control en el chat

Si el dispositivo realmente entrega al proveedor un `=/>on` válido desde
la cuenta vinculada, se espera una respuesta corta de confirmación en el
mismo chat propio. No aparece por duplicados, comandos inválidos, grupos o
otros remitentes. Una respuesta fallida después de `applied` no deshace el
efecto; la terminal indica ese fallo. No usar la respuesta como prueba
absoluta de entrega a otros dispositivos.

## Estética de otros bots

Durante esta beta `=/>menu` permanece en texto plano. No lee por
sí solo la respuesta de otros bots: para reproducir un estilo es
necesario observar un mensaje real del bot fuente y autorizar la
extracción de su marco decorativo. No se importarán comandos ajenos,
enlaces, identidades ni sesiones. Ver `docs/MENU_STYLE_IMPORT.md`.

## Recuperación de la comunidad: =/>add++ (alpha/beta experimental)

Una vez vinculada la cuenta y verificado que administra una comunidad activa, usar `=/>add++` para inspeccionar el inventario aproximado sin escribir archivos. Para conservar una copia cifrada antes de una suspensión, primero habilitar la clave dedicada en una configuración existente:

```bash
bash scripts/whatsapp-test-setup.sh --enable-recovery
```

Reabrir las dos terminales y volver a cargar `runtime.env` para que el proveedor lea la clave. Con una comunidad real identificada por `=/>start` o `=/>add++`, emitir desde el chat propio:

```text
=/>add++
=/>add++ snapshot 120363EXAMPLE@g.us
```

Sustituye el ejemplo por el ID real de la comunidad, que debe terminar en `@g.us`. El resultado de `snapshot` debe confirmar el nombre de un archivo `community-recovery-*.enc.json` en la carpeta privada de la sesión y el conteo. **No añade integrantes**. Para verificar su integridad sin abrir WhatsApp:

```bash
cargo run --manifest-path providers/whatsapp-rust/Cargo.toml -- --recovery-verify ARCHIVO.enc.json
```

Antes de usarlo, cargar `runtime.env` como en la Terminal B. El argumento debe ser solo el nombre que devuelva el comando, sin directorios. No compartir la clave ni publicar el respaldo cifrado; guardarlos separadamente en medios seguros. Si tu servidor es temporal, conserva ambas copias fuera del servidor antes de perder acceso.

## Diagnóstico de sesión inexistente

Si el directorio `$HOME/.local/share/base-matter/whatsapp-test/` no existe,
no presupongas que la sesión se perdió ni ejecutes un reinicio destructivo.
Busca primero `runtime.env` y `session.db` en las rutas locales que usaste
antes, incluidos otros usuarios y la ubicación previa del repositorio.
No regeneres llaves, no borres locks, no sobrescribas SQLite y no comuniques
secretos ni códigos de vinculación. El cambio de carpeta del repositorio no
mueve automáticamente los archivos de sesión de otra ruta.
