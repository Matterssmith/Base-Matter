## 2026-11-19 — Conexión Base/Vault verificada en CI y paso al host pendiente

- Base SHA `98231581533bf641e46ce7f5d53a3978b95eb7ab`: Rust Windows/Linux CI `44659925103` verde, whatsapp-rust `44659925089` verde. Vault E2E `44660096430` verde para socket Unix real, cifrado, get/put CAS, revocación y arranque systemd con UIDs distintos; Base no pudo leer master.
- `=/>vault` informa estado; `=/>vault test` prueba get/put/get cifrados de un marcador reservado, propietario self-chat solamente. Necesita entorno y grupo IPC en el proceso real.
- NO está instalado en Debian del dueño; no hay copia segura de ciphertext runtime a GitHub; no habilitar roles, control global ni comandos privados. Guía pública docs/VAULT_CONNECTION_READY.md, referencia operativa en documentación privada Vault.
- Actualizados ambos README, AGENTS, DECISION_LOG, WORK_CONTINUITY y DEVELOPMENT_STATUS sin cambiar Rust; beta/stable intactos, Bot-Matter bloqueado. CI de este commit documental pendiente.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Vault test: prueba de escritura/lectura cifrada desde Base y WhatsApp

- `VaultClient::verify_roundtrip` verifica `get -> put(CAS) -> get` sobre un registro sintético reservado, con revisión comprobada y sin mostrar el payload. No permite roles ni otra categoría de registros; se conserva la concesión tenant/bot/socket vigente y validación del gateway.
- Nuevo operador CLI `matter-vault-client self-test` y comando nativo `=/>vault test` exclusivamente en chat privado del propietario autenticado. `=/>vault` sin argumentos continúa devolviendo estado. No modificar ni probar con secretos reales.
- Requiere servicio y secretos de cliente ya provisionados bajo UIDs diferentes. Pruebas de CI Rust y E2E privado del nuevo SHA pendientes. README revisados: 2/2; AGENTS/decisiones/continuidad actualizados, alpha únicamente, beta/stable intactos y Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Comando de estado Vault protegido en WhatsApp-Rust

- El controlador =/> integra exclusivamente lectura de salud del canal Vault mediante =/>vault y =/>vault status, en el self-chat verificado y por actor dueño del socket. Opera incluso con bot apagado; NO concede roles, configura secretos ni ejecuta comandos privados.
- La configuración del cliente toma seis variables de entorno con rutas/identificadores, NUNCA la llave maestra ni contenido de la clave. Configuración incompleta bloquea startup. Errores y salida son mínimos, sin identidad ni rutas.
- Enlaces relativos README del proveedor corregidos. README revisados: 2/2. Cambios sólo alpha, con tests y CI por verificar; Bot-Matter sigue bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

# Base-Matter — real encrypted Vault IPC client

Base now includes module vault_ipc::VaultClient and binary
matter-vault-client for a host-provisioned Unix-domain Vault service.
They do NOT receive or import the Vault master key. The host provisions
a separate 32-byte client transport credential under Unix mode 0600,
outside Git. A private Vault grant binds that credential to a tenant,
bot and socket for seven days and may be revoked.

The protocol encrypts and authenticates both request and response
with XChaCha20-Poly1305, independent nonces, AAD with direction/client,
and random request IDs. The server stores durable encrypted replay marks.
Only configuration records for the registered tenant/bot/socket are
supported, using compare-and-swap revisions. No global role grants.

Run matter-vault-client with:
- ping SOCKET_PATH CLIENT_ID KEY_FILE TENANT BOT SOCKET_ID
- read SOCKET_PATH CLIENT_ID KEY_FILE TENANT BOT SOCKET_ID RECORD OUTPUT_FILE
- write SOCKET_PATH CLIENT_ID KEY_FILE TENANT BOT SOCKET_ID RECORD INPUT_FILE new|REVISION

Input and output values are passed as files, never CLI arguments.
Output is create-new mode 0600 and payload never printed to stdout.

Limitations: requires a separate Vault host account, Unix socket with
restricted access, and securely provisioned client key. This is not
automatically attached to native WhatsApp control or roles. A real
socket owner and actor must be authenticated before binding any admin
command; an IPC key alone authenticates the service, not chat users.

Windows rejects this Unix IPC API; alternative secure transport is pending.
Backups/rotation/audit finalization, automated enrollment and production
deployment remain pending. Missing service or credentials is denied.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
