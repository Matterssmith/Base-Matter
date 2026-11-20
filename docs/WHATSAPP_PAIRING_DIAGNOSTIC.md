## 2026-11-20 — Inspección corregida del esquema SQLite v0.7.0 y cierre de reconexión

- **Evidencia final del servidor**: el operador verificó ambos binarios SHA `6fc993b7f335f62dae8c84d91109e929a502a332`; `scripts/whatsapp-vault-existing.sh` pasó checks, inició el controlador y ejecutó `--resume-only`; el proceso volvió inmediatamente al shell sin disparar `on_connected`. No hubo emparejamiento ni autenticación de WhatsApp.
- La inspección anterior `DEVICE_ROW_EXISTS=yes` usaba preferentemente `devices` (plural); en `whatsapp-rust 0.7.0` el registro auténtico del `SqliteStore` está en `device` (singular) con `pn`, `lid`, `account` y material de claves. **La comprobación anterior NO prueba que el almacén tenga credenciales de una identidad vinculada**. Fuente: `oxidezap/whatsapp-rust` etiqueta `v0.7.0`, ruta `storages/sqlite-storage/migrations/2025-08-14-035031_initial/up.sql`.
- El inspector ahora consulta exclusivamente `device` para comprobar existencia y devuelve solo banderas `DEVICE_HAS_PN`, `DEVICE_HAS_LID`, `DEVICE_HAS_ACCOUNT`, `DEVICE_HAS_KEY_MATERIAL` y `SESSION_AUTHENTICATED=not_determined_offline`. Ninguna consulta extrae valores sensibles.
- El proveedor ahora comunica cuando `--resume-only` termina antes de autenticación, y distingue terminación tras autenticación; `on_logged_out` informa desconexión de dispositivo sin JID ni secretos. No agrega reintentos de pairing.
- **Antes de otro arranque**, ejecutar únicamente `python3 scripts/whatsapp-session-inspect.py` y revisar WhatsApp → Dispositivos vinculados. Los valores `no` no autorizan borrar DB. Si `account=no` junto con `pn=no` y `lid=no`, no hay prueba de identidad emparejada en el dispositivo predeterminado. Si todas indican `yes`, puede existir credencial histórica revocada o error de red; el servidor sigue siendo la autoridad de vínculo.
- No modificar `runtime.env`, `session.db*` ni datos de Vault. Sin CI verde de este commit no sustituir binarios instalados. No repetir emparejamiento a ciegas.

## 2026-11-20 — Recuperación de wa-test-1 con credenciales existentes (sin solicitar código)

- El operador confirmó `sudo systemctl is-active matter-vault.service` -> `active` y `matter-vault-client self-test` -> `vault_roundtrip_verified revision=2`. Esta **prueba de escritura y lectura cifradas desde el host real** cierra el pendiente IPC; NO certifica conexión WhatsApp.
- `python3 scripts/whatsapp-session-inspect.py`: `session.db` existe (475136 bytes), integridad SQLite `OK`, 20 tablas y `DEVICE_ROW_EXISTS=yes`. Se conservaron `runtime.env`, `activation.db` y `replay.db`. Las filas no prueban que WhatsApp admita la identidad actual.
- El mismo operador verificó SHA-256 y coincidencia de SHA `36cf35f5572c505fe48cd4249a64594ff861525a` en los binarios controlador/proveedor descargados de CI. Ningún contenido de sesión fue publicado en GitHub.
- Según implementación `whatsapp-rust 0.7.0`, `BotBuilder::with_pair_code` comprueba `client.is_logged_in()` antes de pedir código; el rechazo anterior indica que **no estaba autenticado en ese instante**, aunque exista la fila SQLite. Fuente técnica: https://docs.rs/whatsapp-rust/0.7.0/src/whatsapp_rust/bot.rs.html
- Modo nuevo `whatsapp-matter-rust-provider --resume-only`: conecta exclusivamente con credenciales existentes; no llama a `with_pair_code` ni a `pair_with_code` en refresh; no genera ni presenta códigos. El lanzador seguro `scripts/whatsapp-vault-existing.sh` invoca ahora siempre `--resume-only`.
- Sólo cuando el cliente imprima `WhatsApp session connected. Base-Matter owner control available.` probar en chat propio `=/>vault` y `=/>vault test`. Si no conecta, detener con Ctrl+C, conservar DB y confirmar el dispositivo en WhatsApp > Dispositivos vinculados. El archivo SQLite no debe eliminarse ni sustituirse.
- Un emparejamiento nuevo requiere comprobación de teléfono y autorización por separado. Sin autocompletar, sin reintentos y sin QR. Nada de Bot-Matter, roles o otras ramas.

# WhatsApp-Matter Rust: vínculo por código y recuperación del estado real

## 2026-11-20 — Incidente del servidor instabox-se

Se confirmó `matter-vault.service` con IPC autenticado al cliente `base-wa-test-1` de `wa-test-1`. Ya recuperados 4.7 GB, binarios controlador/proveedor para Base alpha `158eb4792cf1bf42b0b6aae70b1ffa90a4d4d8a3` con CI correctos. El lanzador `scripts/whatsapp-vault-existing.sh` pasó `--check` de ambos y arrancó controlador; el proveedor informó `WhatsApp-Matter Rust session starting. Pairing code only.` y `Pairing code request failed or was rate limited`. **No hubo `WhatsApp session connected`, por tanto el proveedor no autentica WhatsApp y no se ha probado el comando en el teléfono.**

La frase del error era ambigua: `on_pair_code_error` descartaba la categoría del proveedor. Ahora distingue `BadRequest/RateOverlimit` (posible límite de solicitudes), `FeatureNotAvailable`, `Forbidden`, otros rechazos y eventos sin clasificación; omite valores sensibles y no propone reintentos automáticos. El upstream `whatsapp-rust 0.7.0` salta solicitud de pairing si hay una sesión autenticada restaurada. Una base SQLite existente **no demuestra** credenciales de un dispositivo vinculado.

Se detectó otra omisión: `signed_verified_control` no permitía `=/>vault` para el chat propio, pese a que el controlador ya lo reconocía. Se agregó sólo a esa lista privada y se probó la firma del propietario y el rechazo de grupo/terceros. No se autoriza `=/>vault` públicamente.

## Diagnóstico offline obligatorio (sin contraseñas, JID ni registros)

Sin iniciar el proveedor ni pedir más códigos, desde Base-Matter alpha:

```bash
cd /home/Sakura/Base-Matter
python3 scripts/whatsapp-session-inspect.py
```

El inspector no carga `runtime.env`, no lee contraseñas ni publica datos de SQLite y abre la base en modo lectura. El informe indica tamaño de las bases, integridad y si hay registros de dispositivo sin imprimir su contenido. Es sólo una pista: no demuestra que WhatsApp acepte esas credenciales.

Si el almacén original no tiene dispositivo, o la vinculación fue invalidada en el teléfono, se necesitará autorización expresa del propietario y una vinculación por código **una sola vez después de esperar el intervalo que indique WhatsApp**; no recrear bases de datos ni repetir el proceso por adivinanza. Si existen credenciales, diagnosticar la versión/proveedor/conexión o un logout sin borrar contenido.

No ejecutar `cargo run`, no borrar `session.db*`, `runtime.env`, locks ni bases del controlador. No activar Bot-Matter ni editar ramas beta/stable. El servicio Vault independiente continúa operativo.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
