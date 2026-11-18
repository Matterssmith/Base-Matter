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
