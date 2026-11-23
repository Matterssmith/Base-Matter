# Activar la conexión Vault en Base-Matter — idkbot / alpha / wa-alpha

**Ámbito:** sesión WhatsApp ya existente `wa-alpha`, Debian 13, usuario `Sakura`. Este documento activa únicamente el **enlace IPC de configuración** con Vault-Matter, no el registro civil, el gestor global de roles ni Matter-Bot.

> Antes de actuar: el historial del proyecto registra que `base-wa-alpha` estuvo enrolado y el ping/self-test cifrado funcionó anteriormente, pero el servidor puede haber cambiado. **No repetir el enrolamiento, no borrar archivos ni generar nuevas sesiones**. Las credenciales nunca van a Git. El repositorio privado Vault-Matter conserva su propia guía `docs/ACTIVAR_VAULT_ALPHA.md`.

## 1. Ubicar el checkout alpha y verificar el código

En `idkbot`, como `Sakura`:

```bash
cd /home/Sakura/Matter-Ecosystem/Base-Matter
git branch --show-current
git status --short
git log -1 --oneline
```

Debe responder `alpha`. Si el checkout está en otra rama, **no** ejecutar el lanzador alpha allí. Para traer código sin sobrescribir archivos locales ni perder la sesión, solamente dentro del checkout alpha limpio:

```bash
git fetch origin alpha
git pull --ff-only origin alpha
```

**Fuente de verdad:** GitHub. No ejecutar `git reset --hard`, no hacer `cargo clean` ni compilar con el bot activo. Antes de reemplazar binarios, verificar el SHA de la revisión, los dos workflows de GitHub Actions (Linux y proveedor WhatsApp), `BUILD_INFO` y `SHA256SUMS`; ambos binarios deben corresponder a una revisión autorizada. Un `git pull` **no** actualiza un ejecutable ya en uso.

## 2. Diagnóstico de Vault y de la sesión existente

```bash
id -nG
systemctl is-active matter-vault.service
test -S /run/matter-vault/vault.sock && echo "VAULT_IPC_SOCKET=EXISTS"
test -f /etc/base-matter/vault-alpha.env && echo "VAULT_ALPHA_ENV=EXISTS"
test -f /etc/base-matter/alpha-client.key && echo "VAULT_ALPHA_CLIENT_KEY=EXISTS"
test -f "$HOME/.local/share/base-matter/branches/alpha/whatsapp-test/runtime.env" && echo "ALPHA_RUNTIME=EXISTS"
pgrep -af 'whatsapp-matter-control|whatsapp-matter-rust-provider' || true
ss -ltn '( sport = :39821 )'
```

`/etc/base-matter/vault-alpha.env` debe estar bajo usuario autorizado y modo `0600`; no imprimir su contenido. `MATTER_WA_SOCKET_ID` del runtime tiene que ser `wa-alpha` y conservar su propia `session.db`, `activation.db`, `replay.db` y puerto `127.0.0.1:39821`. El lanzador comprueba esas condiciones sin mostrar secretos. Si `Sakura` no es miembro de `matter-vault-ipc`, se requiere corrección administrativa y **nueva sesión Unix**, no un bypass de permisos.

No confundir con el estado legado `~/.local/share/base-matter/whatsapp-test` (`wa-test-1`). Nunca copiar su keyfile ni SQLite al entorno alpha.

## 3. Probar Vault sin desconectar WhatsApp

El siguiente comando prueba solo el canal cifrado para `wa-alpha`; no solicita pairing, no inicia otro socket, ni consulta datos de usuarios:

```bash
/usr/local/bin/matter-vault-client ping /run/matter-vault/vault.sock base-wa-alpha /etc/base-matter/alpha-client.key matterssmith base-matter wa-alpha
```

Si falla, revisar en Vault-Matter la vigencia del grant de siete días, el servicio y el grupo IPC. **No** generar otra clave o reenrolar automáticamente. Es posible que `=/>vault` siga fallando incluso si este ping es correcto si el controlador instalado no incluye el soporte adecuado; eso requiere reemplazar el binario con CI verificado, no crear otra sesión.

Para verificar escritura y lectura cifrada únicamente sobre el marcador sintético reservado, de manera opcional:

```bash
/usr/local/bin/matter-vault-client self-test /run/matter-vault/vault.sock base-wa-alpha /etc/base-matter/alpha-client.key matterssmith base-matter wa-alpha
```

Resultado esperado si está autorizado: `vault_roundtrip_verified`. No usar este subcomando para registros reales.

## 4. Preflight del lanzador con opt-in explícito

Usar los binarios ya verificados. Estas rutas son las del esquema de despliegue conocido; sustituirlas por las rutas efectivas si el operador usa otra carpeta:

```bash
cd /home/Sakura/Matter-Ecosystem/Base-Matter
CONTROLLER=/home/Sakura/matter-ci-logout/controller/whatsapp-matter-control
PROVIDER=/home/Sakura/matter-ci-logout/provider/whatsapp-matter-rust-provider
test -x "$CONTROLLER" && test -x "$PROVIDER" && echo "BINARIOS=EXISTEN"
bash scripts/whatsapp-alpha-existing.sh --check "$CONTROLLER" "$PROVIDER" --vault-env /etc/base-matter/vault-alpha.env
```

Esperado: `ALPHA_PREFLIGHT=OK`, `ALPHA_VAULT=AUTHENTICATED_CONFIGURATION_ONLY` y `ALPHA_PAIRING=DISABLED`. El script realiza ping auténtico con `base-wa-alpha` y verifica el socket existente.

**Importante:** `--check` verifica disponibilidad del lock del proveedor. Si WhatsApp continúa ejecutándose, puede rechazar la comprobación porque la sesión está en uso; **esto no autoriza a matar procesos**. Hacer la verificación final en una ventana de mantenimiento, una vez detenido **solo el proveedor alpha** desde su consola original con Ctrl+C. No iniciar otra instancia mientras la actual se mantenga activa.

## 5. Activar Vault con la sesión WhatsApp existente

Solo después de confirmar: CI de ambos binarios verificado; ping y preflight correctos; respaldos locales de SQLite revisados; ausencia de otro controlador/proveedor alpha en ejecución; y la consola original lista para reanudar.

```bash
cd /home/Sakura/Matter-Ecosystem/Base-Matter
CONTROLLER=/home/Sakura/matter-ci-logout/controller/whatsapp-matter-control
PROVIDER=/home/Sakura/matter-ci-logout/provider/whatsapp-matter-rust-provider
pgrep -af 'whatsapp-matter-control|whatsapp-matter-rust-provider' || true
bash scripts/whatsapp-alpha-existing.sh --resume-only "$CONTROLLER" "$PROVIDER" --vault-env /etc/base-matter/vault-alpha.env
```

El comando se ejecuta **en primer plano**, mantiene la sesión SQLite existente, crea snapshots privados, usa las credenciales Vault dedicadas y **no pide un código nuevo**. Detenerlo con Ctrl+C cuando corresponda. Si aparece un intento inesperado de pairing, cancelar antes de suministrar cualquier código. Jamás iniciar en paralelo con el proceso anterior.

No emplear `scripts/whatsapp-one-terminal.sh` como sustituto automático: ese lanzador puede iniciar rutas distintas y generar configuración si falta.

## 6. Verificación por WhatsApp y alcance real

Desde el **chat privado propio** del dueño autenticado del socket, enviar:

```text
=/>vault
=/>vault test
```

`=/>vault` consulta estado; `=/>vault test` debe utilizar exclusivamente el registro sintético de prueba y no otorga roles. No probar comandos privilegiados en grupos ni con usuarios externos. Si retorna `AUTH_REJECTED`/denegado, guardar el diagnóstico no sensible, comprobar versión del controlador, grant, socket y JID verificado; no desactivar políticas de seguridad.

**Conectar Vault no habilita `=/>register start`.** El registro web necesita un gateway privado, consentimiento publicado y autenticación/verificación adicionales; la vista previa pública de Matter Register es otro componente. No pedir CURP, fotografías ni biometría a través de este flujo.

## 7. Fallos frecuentes y reglas

- `matter-vault.service` inactivo: revisar unidad, socket y permisos con el administrador. No iniciar un servicio de Vault como `Sakura`.
- `missing alpha-client.key` o `vault-alpha.env`: acudir a la guía privada de Vault-Matter. No copiar `client.key` legado ni generar otro par automáticamente.
- `unauthorized` o grant caducado: tramitar renovación y auditoría con Vault-Matter; los grants no son permanentes.
- `Control plane already owned` o lease ocupado: el bot ya puede estar ejecutándose; no usar `kill -9` ni eliminar locks o sesiones SQLite.
- Código Git y binarios pueden tener SHAs diferentes; solo reemplazar desde artefactos y hashes verificados. No asumir que `git pull` recompiló un binario.
- No modificar `beta`, `stable`, `master.key`, `/etc/base-matter/client.key`, ni respaldos ajenos.

**Referencias:** `scripts/whatsapp-alpha-existing.sh`, `docs/VAULT_CONNECTION_READY.md`, `docs/WHATSAPP_VAULT_PREBUILT.md` y guía privada de `Vault-Matter/docs/ACTIVAR_VAULT_ALPHA.md`.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
