## 2026-11-19 — Estado cruzado de integración con Vault-Matter privado

- Cliente IPC real Base: `matter-vault-client` y `=/>vault` / `=/>vault test`, restringidos a identidad/socket autorizado. La conexión Vault/Base bajo UIDs Unix separados ya está verificada **en GitHub Actions**, pero **NO instalada en el Debian del propietario** ni probada mediante mensaje real de WhatsApp.
- Vault-Matter rama `matter`, checkpoint `2e87ec545073c7772c2ebc4f3c5a28d75ddbad3e`: añadió exportación/restauración local de archivos cifrados, `verify-root` y nuevas pruebas de recuperación. Resultado final de CI de ese SHA pendiente en el momento del checkpoint.
- La exportación local no es un backup remoto: está pendiente la réplica de ciphertext a GitHub privado y la custodia independiente de llave maestra. Base-Matter público **no recibe** la maestra ni GitHub token, y Bot-Matter permanece bloqueado.
- Paso operativo: instalar el servicio y el cliente en Debian bajo cuentas separadas usando la guía privada Vault; comprobar `ping` y `self-test` desde UID Base, integrar las seis variables de entorno en el controlador WhatsApp y entonces probar `=/>vault` en chat propio. Si faltan privilegios o concesiones: denegado, sin fallback.
- No se ha habilitado control privado, roles globales ni autoridad por actor WhatsApp. No promocionar `beta` / `stable` hasta las pruebas y políticas pendientes.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

# Base-Matter — conexión real a servicio Vault (confirmada en CI)

**Alcance implementado:** `VaultClient`, binario `matter-vault-client`, `=/>vault` y `=/>vault test` (solo propietario y chat propio, aun con bot OFF). `=/>vault` sin argumentos sólo informa estado. `test` modifica exclusivamente el registro sintético reservado `connection-health-v1`.

El cliente Base no contiene la clave maestra del repositorio privado. Recibe únicamente un secreto de transporte externo de 32 bytes, propio de su bot/socket, almacenado fuera de Git con modo 0600 y nunca impreso. El servidor privado rechaza mensajes no autenticados por AEAD, permisos de otro socket/tenant, claves erróneas, revisiones obsoletas y concesiones revocadas o vencidas.

## Evidencia concreta

- Base alpha SHA `98231581533bf641e46ce7f5d53a3978b95eb7ab`: Rust Linux y Windows `https://github.com/Matterssmith/Base-Matter/actions/runs/44659925103` **success**.
- Proveedor whatsapp-rust `https://github.com/Matterssmith/Base-Matter/actions/runs/44659925089` **success**.
- E2E real del protocolo en runner Linux Ubuntu con servidor Vault privado, cliente Base, systemd y UIDs separados: `https://github.com/Matterssmith/Vault-Matter/actions/runs/44660096430` **success**. Es un enlace entre dos ejecutables y secretos efímeros fuera de Git, NO una simulación de funciones.

## Para activar el enlace en un host Debian

1. Instalar el servicio Vault bajo un UID independiente de la cuenta Base con el procedimiento privado de aprovisionamiento aprobado. Se necesitan privilegios administrativos; no degradar el aislamiento a un único usuario.
2. El runtime Base debe recibir estas variables: `MATTER_VAULT_IPC_SOCKET`, `MATTER_VAULT_CLIENT_ID`, `MATTER_VAULT_CLIENT_KEY_FILE`, `MATTER_VAULT_TENANT`, `MATTER_VAULT_BOT`, `MATTER_VAULT_SOCKET_ID`. Sólo rutas e IDs; nunca bytes de la clave.
3. El valor `MATTER_VAULT_SOCKET_ID` debe coincidir exactamente con `MATTER_WA_SOCKET_ID`. El proceso de Base debe iniciar con acceso al grupo IPC y una nueva sesión Unix para heredar membresía; no se logra reiniciando solamente el comando en una shell antigua.
4. En chat privado del dueño verificado: `=/>vault` verifica el handshake; `=/>vault test` realiza `get -> put CAS -> get` sobre un marcador sintético. En grupo o cuenta externa denegar.
5. El almacén cifrado del host NO se replica todavía a GitHub. No usar el servicio con registros irremplazables hasta implantar backups remotos cifrados y recuperación verificable.

## Límites por los que Matter-Bot permanece bloqueado

La conexión **no** habilita `=/>set role`, `anti` privado, delegación de comandos de bot, roles globales ni claves de IA: requieren políticas por actor/grupo/operación y gateway privado de acciones con auditoría, revocación y controles independientes. No está desplegada en el servidor del dueño ni comprobada por un dispositivo WhatsApp; el proveedor está probado offline.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
