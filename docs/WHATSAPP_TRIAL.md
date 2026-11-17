# Prueba WhatsApp Rust de Base-Matter — pairing code

Este procedimiento se puede ejecutar desde la rama `matter-beta` con
una cuenta secundaria propia autorizada. Es una prueba, **no una
certificación de estabilidad**. Solo se usa Rust y vinculación por código.
No se muestra QR, no se usa Node/Baileys y no se inicia Bot-Matter.

## Preparación

Los dos procesos Rust deben funcionar en el MISMO servidor. Requisitos:
Rust/Cargo, OpenSSL, Git y las herramientas para compilar librerías Rust.
No compartas en GitHub ni en este chat los secretos ni el teléfono.

Comprueba antes tus cambios locales. Nunca sobrescribas una sesión activa:

```bash
cd /home/Sakura/Matter-Base
git status --short --branch
git fetch origin
git switch matter-beta
git pull --ff-only origin matter-beta
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
cd /home/Sakura/Matter-Base
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
cd /home/Sakura/Matter-Base
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
=/>on
=/>status
=/>off
```

La aplicación **no responde por WhatsApp**: el proveedor muestra
aceptación o rechazo en su terminal. No existe comando `ping` de
producción ni ejecución de comandos del futuro Bot-Matter.

`=/>on` enciende el estado operativo del socket, pero no concede
una política para ejecutar comandos normales. Los grupos y terceros
no tienen autoridad para controlar ese socket.

## Bloqueos para promoción a matter

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
