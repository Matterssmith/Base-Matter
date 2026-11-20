# Base-Matter alpha — prueba WhatsApp/Vault sin Cargo ni una sesión nueva

## Estado de despliegue

En `instabox-se` se confirmó el mensaje del instalador privado `CONEXIÓN REAL CONFIRMADA` después de descargar el artefacto GitHub Actions Vault `44664373277` y comprobar SHA-256; el cliente registrado es `base-wa-test-1` con `wa-test-1`. El servicio `matter-vault.service` es real. El comando `self-test` y la prueba WhatsApp `=/>vault` aún no aparecen en la evidencia remitida, por lo que **no están certificados**.

El servidor Debian 13 posee 16 GB de disco. Se recuperaron 4.7 GB borrando exclusivamente dependencias Rust generadas `debug/deps`. Volver a invocar `cargo run` o el lanzador normal puede regenerar varios gigabytes.

## Precaución sobre el socket legado

`scripts/whatsapp-one-terminal.sh` inicia una sesión `branches/alpha/whatsapp-test` con su propio ID (típicamente `wa-alpha`) y crea una configuración si no existe. Vault, en cambio, se enroló contra el `wa-test-1` del archivo preexistente `~/.local/share/base-matter/whatsapp-test/runtime.env`. **No fusionar estos estados, ni relajar validaciones, ni emparejar un teléfono de nuevo.**

## Binarios CI publicados en GitHub Actions

Dos workflows independientes publican únicamente para `alpha` en `push`, tras sus suites:
- `base-alpha-control-linux-x86_64-SHA`: controlador `whatsapp-matter-control`, `SHA256SUMS`, `BUILD_INFO`.
- `base-alpha-provider-linux-x86_64-SHA`: proveedor `whatsapp-matter-rust-provider`, `SHA256SUMS`, `BUILD_INFO`.

Ambos deben presentar **el mismo SHA del commit Base alpha** y `TARGET=x86_64-unknown-linux-gnu`. Ninguno contiene claves, sesiones, SQLite o `runtime.env`. Caducan a los 7 días. Nunca instalar si los dos CI no finalizaron correctamente.

Desde una sesión nueva `su - Sakura` (para que herede el grupo `matter-vault-ipc`) descargar por `gh run download` a directorios separados bajo `/home/Sakura/matter-ci-binaries`, validar SHA256SUMS y BUILD_INFO en ambos. No compartir `gh auth` tokens ni claves de cliente.

## Ejecutar sobre la sesión original

Sólo si los binarios son verificables y el estado original existe:

```bash
bash /home/Sakura/Base-Matter/scripts/whatsapp-vault-existing.sh \
  /home/Sakura/matter-ci-binaries/controller/whatsapp-matter-control \
  /home/Sakura/matter-ci-binaries/provider/whatsapp-matter-rust-provider
```

El lanzador independiente lee los env privados existentes, exige coincidencia `MATTER_WA_SOCKET_ID == MATTER_VAULT_SOCKET_ID`, revisa el grupo IPC, conserva el mismo SQLite y evita `cargo build`, `--init`, generación de credenciales y sesiones paralelas. Primero verifica `--check` de ambos binarios; si la sesión original no existe, rechaza. Un mensaje de pairing inesperado es motivo para cancelar antes de introducir ningún código.

En el chat propio autorizado enviar `=/>vault` y `=/>vault test`; no emplear en grupos. El `self-test` CLI independiente debe arrojar `vault_roundtrip_verified` antes. Sin autorización por actor todavía, `roles` y Bot-Matter se mantienen bloqueados.

## Riesgos pendientes

La credencial de cliente tiene autorización de siete días; hace falta política de renovación sin regenerar claves automáticamente. No existe backup remoto automático de registros cifrados ni respaldo independiente de la clave maestra. No publicar tokens, archivos de sesión ni cambiar ramas beta/stable.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
