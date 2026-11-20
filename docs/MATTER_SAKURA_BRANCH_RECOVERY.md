# Sakura + continuidad de ramas Matter

**Fecha:** 2026-11-20. **Ubicación:** `Base-Matter/workstreams/02-base`. **Estado:** scripts en GitHub, pendientes de instalación en la laptop Ubuntu.

## Cuentas y autenticación

`tools/setup-sakura.sh` crea el usuario Linux **Sakura**, directorio `/home/Sakura` y estructura privada:

- `/home/Sakura/Matter/workspaces/` para un checkout aislado por línea de trabajo.
- `/home/Sakura/Matter/checkpoints/` para metadatos privados de recuperación.
- `/home/Sakura/.config/matter-ai/` para configuración del agente, modo `0700`.

### Modos de inicio de sesión

1. **Sin contraseña configurada (predeterminado):** `useradd` deja la contraseña bloqueada. Esto **no** habilita contraseñas vacías: el administrador puede entrar con `sudo -iu Sakura`; para entrar por SSH hay que configurar clave pública.
2. **Con contraseña opcional:** `sudo bash tools/setup-sakura.sh --password-prompt`. El sistema ejecuta `passwd Sakura` para pedirla dos veces sin mostrarla; no se guarda en Git.
3. **Con clave pública SSH:** `sudo bash tools/setup-sakura.sh --authorized-key-file /ruta/clave.pub`. Instala una sola clave **pública** en `/home/Sakura/.ssh/authorized_keys`, permisos restringidos. Conservar privada fuera del servidor. No se modifica `sshd_config`.

**No se crea un usuario sin autenticación remota.** La cuenta nueva sin contraseña no puede iniciar por contraseña. Si la cuenta ya existe, el script conserva UID, datos y contraseña; no borra su contenido. El administrador de Ubuntu y root aún pueden acceder; no es posible garantizar acceso exclusivo absoluto a un usuario local.

El script **no otorga sudo sin contraseña** y no escribe en `/etc/sudoers`. Cualquier gestión de sistemas requiere el administrador y aprobación.

## Inicio mínimo en consola

Ejecuta como tu usuario Ubuntu actual con capacidad de `sudo`:

```bash
git clone -b workstreams/02-base https://github.com/Matterssmith/Base-Matter.git ~/Base-Matter-bootstrap
sudo bash ~/Base-Matter-bootstrap/tools/setup-sakura.sh
sudo -iu Sakura
```

Ahora estarás en Sakura (`whoami` debería mostrar `Sakura`). Continúa desde esa sesión:

```bash
git clone -b workstreams/02-base https://github.com/Matterssmith/Base-Matter.git ~/Base-Matter
cd ~/Base-Matter
bash tools/install-matter-ai.sh
~/.local/bin/matter-ai setup
```

Para establecer contraseña, añade `--password-prompt` a la ejecución como administrador. Una contraseña vacía no es una opción de acceso remoto seguro. Si solo quieres operar en la consola local de la laptop, la opción sin contraseña y `sudo -iu Sakura` funciona.

Instala y autoriza `gh` dentro de la sesión Sakura para repositorios privados y creación de PR. La API key de OpenAI se guarda dentro de la cuenta Sakura con permisos `0600`, nunca en Git.

## Qué significa guardar progreso

Hay tres lugares distintos; no deben confundirse:

1. **Archivos sin commit:** trabajo local solamente. Si falla el disco, no hay respaldo GitHub.
2. **Commit local:** historial de Git en la laptop; conserva cambios después de cerrar la sesión, pero no protege ante pérdida de disco.
3. **Push a rama `workstreams/NN-id`:** versión confirmada en GitHub. Es el respaldo remoto del código (no el respaldo de secretos/volúmenes privados).

`checkpoint` **sólo guarda metadatos** (SHA, lista de archivos, rama, hora) en `~/Matter/checkpoints/ID`, no copia el contenido sin commit. Guarda código de verdad usando `save` y confirma explícitamente el push.

## Comandos Matter-AI

```bash
matter-ai projects
matter-ai sync base
matter-ai sync matter-rust
matter-ai sync vault
matter-ai workstatus matter-rust
matter-ai branches matter-rust
matter-ai checkpoint matter-rust
matter-ai save matter-rust
matter-ai checkpoint all
matter-ai save all
matter-ai promote matter-rust alpha
matter-ai promote matter-rust beta
matter-ai promote matter-rust stable
```

Las operaciones son por proyecto. `matter-ai save matter-rust --local` crea sólo el commit local y no intenta push. El guardado estándar pregunta dos veces: crear commit y publicar rama. Si GitHub rechaza el push, se conserva el commit local sin forzar nada.

`matter-ai sync ID` clona el workstream correspondiente. Cuando el clon está limpio, intenta actualizar **solo por fast-forward**; ante conflicto o cambios locales no altera el directorio. Nunca usa `reset --hard`.

`matter-ai promote ID DESTINO` consulta ramas reales de ese repositorio y crea un **PR en borrador** usando GitHub CLI (`gh`). Los destinos admitidos son `alpha`, `beta`, `stable`, `matter`, `main` y ramas especiales que comiencen con `feat/`, `release/` o `hotfix/`. Un destino inexistente se rechaza. No todas las ramas existen en todos los repositorios. **Ninguna promoción fusiona cambios automáticamente.**

La ruta adecuada es trabajo `workstreams/*` → commit y push → PR a `alpha` → pruebas → PR a `beta` cuando exista → pruebas → PR a `stable` cuando exista. El orden de promoción puede ajustarse por repositorio; no se inventan ramas estables.

## Invariantes

- MSEL, Matter-Rust, Register, Social y demás proyectos trabajan en checkouts separados, aunque compartan repositorio.
- Guardado restringido a la rama exacta `workstreams/*` de cada tarea; `alpha`, `beta`, `stable`, `matter` y `main` reciben cambios por revisión de PR.
- `Bot-Matter` bloqueado; no se permite escribir o publicar durante esta etapa.
- Tokens, archivos `.env`, claves, sesiones, ficheros binarios y datos de `vault/` se excluyen del guardado automático; el código de Vault puede registrarse con validación de archivos, pero su almacenamiento cifrado requiere su propio proceso.
- Los cambios de `.github/` requieren revisión manual para evitar introducir workflows con permisos inesperados.
- Los scripts no realizan reinicios, instalaciones de servicios, movimientos de sesiones ni acceso root sin ejecutar explícitamente la provisión de cuenta.
- Antes de cada commit: revisar README, AGENTS y `docs/WORK_CONTINUITY.md`, ejecutar pruebas disponibles y dejar el avance escrito. La compilación/CI es una comprobación diferente del simple guardado.

## Pruebas y pendiente

Tests offline `tests/test_matter_ai_cli.py` y `tests/test_matter_branches.py` en GitHub Actions Linux/Windows; `bash -n` sobre instaladores. Estos tests no crean Sakura en una máquina real, no verifican credenciales GitHub ni prueban la API de OpenAI.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
