## 2026-11-20 — Diagnóstico de respuesta =/>vault por WhatsApp

- La conexión real Vault del socket wa-alpha se confirmó por el operador: `ALPHA_VAULT_ENROLLED=YES`, `ALPHA_VAULT_IPC=AUTHENTICATED`, `vault_roundtrip_verified revision=1`. El preflight alpha verificó `ALPHA_VAULT=AUTHENTICATED_CONFIGURATION_ONLY`. WhatsApp reconectó y respondió `WhatsApp session connected. Base-Matter owner control available.`.
- Problema reportado: `=/>vault` no devuelve respuesta visible. El proveedor actual registraba solamente `Base-Matter control denied or unreachable` si el IPC fallaba, sin notificar al propietario. También fijaba 4 segundos para cualquier consulta no-IA.
- Corregido `providers/whatsapp-rust/src/main.rs`: sólo para `=/>vault` originado en el chat privado autenticado del propietario, el IPC dispone de 12 segundos; errores reciben un aviso genérico sin secretos ni roles; el envío aceptado se registra sin el contenido. El acceso a grupos y a terceros sigue denegado. Agregada prueba Rust de aislamiento contra comandos falsos, grupos y remitentes externos.
- Estas modificaciones son código en GitHub; **requieren descargar/verificar el binario nuevo del proveedor desde CI y reemplazarlo sólo con la sesión detenida**. No se ha ejecutado esta sustitución ni demostrado todavía la respuesta por WhatsApp. No tocar credenciales ni sesión SQLite.
- README revisados: 2/2 (`README.md` y `providers/whatsapp-rust/README.md`), cuatro documentos de continuidad actualizados atómicamente. Bot-Matter permanece bloqueado, roles y Github sync independientes.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Etapa Vault: continuidad y revisión documental del cliente wa-alpha

- WhatsApp real: el propietario confirmó que `=/>status`, `=/>menu` y `=/>help` funcionan en la sesión alpha conectada.
- Base-Matter alpha modifica únicamente el lanzador existente `scripts/whatsapp-alpha-existing.sh` con opt-in `--vault-env /etc/base-matter/vault-alpha.env`. Sin opt-in, Vault permanece aislado. Con opt-in: `base-wa-alpha`, llave `alpha-client.key`, `wa-alpha`, ruta IPC y ping autenticado. Nunca mezclar con `base-wa-test-1` o con la base de sesión original.
- Vault-Matter privado incorpora instalador para el nuevo grant y sincronización de tar de ciphertext a GitHub usando cuenta separada, token externo, validación y timer; el repositorio público no recibe código ni token Vault.
- EVIDENCIA LIMITADA: los commits están en GitHub. No consta todavía ejecución del nuevo enrolamiento en instabox-se, `=/>vault` real en alpha ni subida remota de una copia cifrada. La autoridad de roles, registros y ejecución privada continúa bloqueada.
- Pruebas requeridas antes de declarar conexión: CI Linux/Windows y proveedor, enrolamiento real sólo de cliente alpha, self-test y preflight (sin segundo proveedor), reconexión controlada y WhatsApp self-chat `=/>vault`.
- README revisados: 2/2 (`README.md` y `providers/whatsapp-rust/README.md`); cambios y decisiones sincronizados en AGENTS/DECISION_LOG/WORK_CONTINUITY/DEVELOPMENT_STATUS en el mismo commit.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Sesión WhatsApp alpha recuperada y autenticada en host real

- Evidencia enviada desde `instabox-se`: en `/home/Sakura/Base-Matter`, el script `scripts/whatsapp-alpha-existing.sh --check` pasó `Base-Matter control settings checked`, `whatsapp-rust pairing settings checked`, `ALPHA_PREFLIGHT=OK`, `ALPHA_VAULT=ISOLATED_NOT_ENROLLED`, `ALPHA_PAIRING=DISABLED`.
- Luego se ejecutó una única prueba `--resume-only` con los binarios previamente verificados `e073b1b5ebc3ad89cec5a9ce8a71647312aebb64`: `ALPHA_SNAPSHOT=VERIFIED_PRIVATE`, `ALPHA_CONTROLLER=STARTED`, `ALPHA_PROVIDER=RESUME_ONLY`, `PN=true`, `LID=true`, **`WhatsApp session connected. Base-Matter owner control available.`**. El proceso quedó en primer plano. No se solicitó código ni QR.
- Esta señal **acredita autenticación y conexión real de la sesión aislada alpha**, distinta del `wa-test-1` original que recibió `AUTH_REJECTED_LOGGED_OUT`. No acredita aún respuesta del bot a un comando ni integración Vault para `wa-alpha`. Conservar originales, respaldos privados, puertos y archivos separados. No copiar archivos entre sesiones.
- Próxima prueba SIN detener el proveedor en curso: en WhatsApp > chat propio, enviar `=/>status`, luego `=/>menu` o `=/>help`; comprobar respuesta real y evento `Base-Matter owner control applied`. Si no responde, inspeccionar sólo errores genéricos del terminal sin publicar JID, teléfono o tokens. No probar `=/>vault test` en alpha hasta enrolar su cliente IPC independiente y verificar concesiones.
- Vault-Matter en `wa-test-1` continúa con última evidencia `vault_roundtrip_verified revision=2`; **no atribuir** esa concesión al socket `wa-alpha`. Ramas beta/stable intactas y Bot-Matter bloqueado hasta criterios de preparación.
- README revisados: 2/2.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Recuperación segura y separada de dispositivo alpha

- Base alpha y socket original tienen PN/LID de la misma cuenta, pero las claves y JID de dispositivo difieren. La sesión original fue rechazada; alpha todavía no está probada en el servidor.
- Creado `scripts/whatsapp-alpha-existing.sh` exclusivamente para el estado `branches/alpha/whatsapp-test`, con `--check` sin cambios y `--resume-only` que exige bases/configuración existentes, puerto y ruta aislados, crea snapshot SQLite privado, desactiva entorno Vault heredado y jamás solicita códigos ni Cargo ni `--init`.
- CI prueba sintaxis y escenarios con binarios simulados: check seguro, snapshot, rutas equivocadas, socket original y configuración ausente. WhatsApp móvil no se ha reconectado.
- README revisados: 2/2. Actualizada continuidad y decisión. Bot-Matter sigue bloqueado; ramas beta/stable no modificadas.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Comparación de cuenta sin sufijos de dispositivo

- Las bases WhatsApp original `whatsapp-test` y aislada `branches/alpha/whatsapp-test` tienen `device.id=1`, pero PN, LID, claves y account difieren. No tratarlas como respaldos intercambiables. Un JID distinto puede deberse a `:n` y representar otro dispositivo de la MISMA cuenta.
- Comparador offline amplía solo el informe con `SAME_PN_ACCOUNT` y `SAME_LID_ACCOUNT` calculados en SQLite (no salen JIDs/secretos), con `unknown` en formato no reconocido. Cero emparejamientos, cero cambios a sesiones, cero cambios Vault; actualizar ambos README y continuidad.
- Próximo paso: ejecutar comparador tras `git pull`, verificar si ambas identidades pertenecen a una misma cuenta y a partir de allí diseñar recuperación aislada; Bot-Matter bloqueado.
- README revisados: 2/2.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Dos bases WhatsApp distintas: comparar sin mover credenciales

- Inventario operador: `whatsapp-test/session.db` original de 475136 bytes y `branches/alpha/whatsapp-test/session.db` de 630784 bytes. La segunda no es una copia verificada ni evidencia de conexión.
- Herramienta Python de lectura `scripts/whatsapp-stores-compare.py` con pruebas CI compara en SQLite los dos registros `device.id=1` sólo por flags de PN/LID/claves/cuenta. Nunca imprime secretos ni copia archivos.
- No mezclar rutas, sesiones, proveedores ni claves. La anterior rechazó autenticación; el teléfono todavía señalaba dispositivo vinculado. La siguiente acción depende de la comparación. Vault sigue OK; Bot-Matter bloqueado; beta/stable intactos.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 UTC — Confirmación: un solo dispositivo SQLite, id predeterminado 1

- Auditoría offline de `/home/Sakura/.local/share/base-matter/whatsapp-test/session.db` en `instabox-se`: `DEVICE_TOTAL=1`; `DEFAULT_DEVICE_EXISTS=True`; `OTHER_DEVICES=0`; `DEFAULT_DEVICE_HAS_IDENTITY=True`. Comprobación SQL de sólo lectura, sin volcar PN, LID, claves ni JID.
- Fuente de implementación `whatsapp-rust 0.7.0` en `storages/sqlite-storage/src/sqlite_store.rs`: `SqliteStore::new(database_url)` llama `Self::build(database_url, 1, ...)`. Por tanto **la selección del registro equivocado queda descartada**: `device.id=1` es el único registro y tiene identidad.
- Pruebas anteriores: PN/LID/account/keys presentes, SQLite íntegra, DNS/TLS a web.whatsapp.com correctos; `--resume-only` emite `WA_LOGOUT_CLASS=AUTH_REJECTED_LOGGED_OUT` antes de autenticación, sin solicitar código. En el teléfono del operador la sesión todavía aparecía en Dispositivos vinculados, con última actividad aproximadamente a las 20:00 hora México (19/11). La UI móvil puede no reflejar de inmediato el estado servidor; ni presencia ni fecha confirman vigencia de credenciales.
- Pendiente: revisar copias anteriores auténticas del almacén *sin abrir ni imprimir secretos* y verificar el estado de vinculación; no repetir conexiones de prueba ni forzar nuevo pairing. Si se decide relink, sólo mediante base de sesión nueva y aislada, respaldo privado autenticado y autorización del propietario. La sesión anterior no debe eliminarse, sobreescribirse ni subirse al repositorio público.
- Vault independiente sigue validado con `vault_roundtrip_verified revision=2`; no confundir rechazo de WhatsApp con fallo de Vault. Bot-Matter continúa bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Cierre de sesión WhatsApp observado con identidad y TLS válidos

- Evidencia directa: inspección de SQLite `device` con `DEVICE_HAS_PN=yes`, `DEVICE_HAS_LID=yes`, `DEVICE_HAS_ACCOUNT=yes`, `DEVICE_HAS_KEY_MATERIAL=yes`, integridad OK. `DNS_WHATSAPP=OK`, `TLS_WHATSAPP=OK` (`TLSv1.3`). El CI `0259d9f0c722dc01abfa3aa6529cdafde5eb196d` publicó binarios firmados por SHA-256 y operador los verificó.
- Lanzador existente sobre `wa-test-1`: `--resume-only` imprimió `PN=true, LID=true`, disparó **evento `LoggedOut` antes de `on_connected`** y terminó. El servicio Vault independiente permanece verificado con `vault_roundtrip_verified revision=2`.
- Revisado el código upstream `whatsapp-rust 0.7.0`: `LoggedOut` puede ser rechazo `<failure>` durante conexión (401/403/406) o stream error de remoción/conflicto (401/516). Se añade categoría anónima `WA_LOGOUT_CLASS` usando sólo `on_connect` y `ConnectFailureReason` (sin raw node, JIDs, claves, mensajes de servidor o número telefónico); pruebas Rust correspondientes.
- **No inferir** que se deba volver a emparejar sólo por `LoggedOut`; primero verificar en el teléfono la sesión de Dispositivos vinculados y si el estado de la cuenta está restringido. No repetir prueba de conexión hasta actualizar a nuevo binario de CI y validar hashes, sin tocar `session.db*`, llaves, Vault ni ramas beta/stable.
- README y continuidad auditados; Bot-Matter continúa bloqueado. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Señales de sesión local existentes; diagnóstico anónimo de transporte

- Inspector de la base real confirmó `device` singular con PN, LID, cuenta y material de claves presentes, SQLite OK; no implica autenticación WhatsApp activa. Eliminada impresión duplicada del estado offline.
- El proveedor Rust, únicamente en nuevo commit alpha después de CI, informa eventos de desconexión, rechazo de conexión, versión obsoleta, restricción temporal y error del stream sin datos personales ni claves. `--resume-only` también distingue identidad almacenada en memoria de conexión efectiva.
- La siguiente decisión depende de comprobar WhatsApp → Dispositivos vinculados y conectividad del host; no se destruye ni cambia ninguna sesión, ni se generan nuevos códigos. Vault continúa con roundtrip autenticado `revision=2`.
- Actualizados README y continuidad. Bot-Matter bloqueado; beta/stable intactos.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Diagnóstico de sesión WhatsApp (tabla singular) y salida del proveedor

- En `instabox-se`, `--resume-only` con binarios SHA verificados `6fc993b7f335f62dae8c84d91109e929a502a332` terminó inmediatamente sin `on_connected`; no se solicita más código ni se declara WA funcional.
- Inspector SQLite corregido tras revisar implementación upstream `whatsapp-rust 0.7.0`: la identidad vinculada es la tabla `device`, **no inferir autenticación de `devices` plural**. Sólo banderas de presencia de `pn/lid/account` y claves, sin extraer sus valores; prueba Python offline añadida.
- El proveedor informa sin secretos cuándo `--resume-only` termina antes o después de autenticación, y evento de desvinculación cuando es emitido. No se añaden reintentos, QR ni cambios de sesión. Manual `docs/WHATSAPP_PAIRING_DIAGNOSTIC.md`.
- Vault (proceso aislado) sigue verificado con `vault_roundtrip_verified revision=2`; no modificar repos privados/llaves. README revisados y continuidad actualizada. CI de esta revisión pendiente; beta/stable y Bot-Matter bloqueados.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Corrección rustfmt del modo resume-only

- CI del proveedor `4af6078bba1ea2dbd4885bcc4345790b54ecd78b` detectó exclusivamente el `println!` de entrada de `--resume-only` que requería múltiples líneas. Corregido de forma idéntica a rustfmt; no se modifica lógica, sesión, concesiones Vault, permisos o código de vinculación.
- Reiniciar CI Linux/Windows/proveedor para nuevo SHA y distribuir artefactos Linux emparejados sólo tras éxito completo.
- README inspeccionados y continuidad actualizada. Bot-Matter bloqueado. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Recuperación segura WhatsApp y self-test Vault en host

- Instalación en instabox-se ya acreditada con IPC autenticado y ahora `vault_roundtrip_verified revision=2` del cliente real Base; servicio `matter-vault.service` `active`.
- Inspector sin secretos confirmó sesión `session.db` SQLite `OK`, 20 tablas y fila de dispositivo existente, no autenticación vigente de WhatsApp.
- Agregado modo `--resume-only` al proveedor Rust; el lanzador seguro del socket legado `wa-test-1` lo usa por defecto, omitiendo peticiones de código y refresh. Rechazo si condiciones no se cumplen, nunca borrar sesión, jamás generar QR. Prueba Rust para presencia/ausencia de opción de emparejamiento.
- CI nuevo de alpha aún por confirmar; binarios anteriores del proveedor no conocen `--resume-only`. Sólo distribuir los artefactos del SHA que pase Linux/Windows/proveedor. A continuación comprobar `WhatsApp session connected` y mandar `=/>vault` en self-chat. No hay prueba WhatsApp positiva aún.
- Guía: `docs/WHATSAPP_PAIRING_DIAGNOSTIC.md`. README raíz y proveedor inspeccionados; beta/stable y Bot-Matter intactos. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Formato CI de prueba Vault y comando diagnóstico correcto

- `whatsapp-rust-ci` del commit `83668775362311ea3c87a6fcc933b910083556b3` falló únicamente en `cargo fmt --check` por una expresión `assert_eq!` larga en prueba nueva. Aplicado exactamente formato indicado por rustfmt. Los tests de lógica todavía deben pasar en nuevo CI.
- Guía de pairing corregida para ejecutar únicamente el inspector Python de sesión, sin proponer `--help` al lanzador productivo. No se ha relanzado ni regenerado WhatsApp en Debian, y el servicio Vault sigue operativo.
- Ambos README revisados, documentación de continuidad y seguridad preservada. CI nuevo pendiente, beta/stable y Bot-Matter intactos.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — WhatsApp pairing no completado; defecto del whitelist Vault corregido

- El operador ejecutó `scripts/whatsapp-vault-existing.sh` como Sakura con grupo IPC heredado. Ambas comprobaciones `--check` pasaron y el controlador inició, pero el proveedor informó solicitud de pairing fallida. No se logró reconexión WhatsApp ni prueba `=/>vault` en teléfono; NO confundir con fallo IPC Vault ya autenticado.
- Corregida lista de comandos firmables: `=/>vault` ahora sólo en self-chat propietario, nunca grupos/terceros. Prueba de firma/rechazo añadida al proveedor.
- Diagnóstico de rechazo de pairing categorizado sin imprimir secretos; inventario Python de SQLite/archivos de sesión **sólo lectura** y sin filas. No se borran ni regeneran credenciales, no se automatiza re-pairing.
- Archivo operativo `docs/WHATSAPP_PAIRING_DIAGNOSTIC.md`. CI de nuevo commit pendiente, no desplegar proveedor hasta pasar CI y recompilar artefacto en GitHub, no en Debian de 16 GB.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Corrección estricta de final de YAML CI

- El job Linux en el commit anterior llegó a `Verify whitespace` y falló al detectar un salto vacío al final del archivo workflow (verificación `git diff --check HEAD^ HEAD`). Se eliminó el salto excedente, sin cambiar código Rust ni sesiones.
- Se solicita un nuevo CI Linux, Windows y proveedor sobre el mismo SHA de empaquetado; ningún artefacto se declara aprobado antes de `success`.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Ajuste de workflow del empaquetado Base CI

- Se detectó que los pasos de empaquetado del controlador quedaron bajo el job Windows por ubicación YAML. Trasladados al job Linux `rust` **después de las pruebas** y añadida comprobación de sintaxis del lanzador sin Cargo. El proveedor revalida su mismo SHA con cambio mínimo en workflow.
- Sin cambio en runtime, sesión ni instalación Vault del servidor. No se consideran listos los nuevos binarios hasta que CI Linux/Windows y proveedor finalicen correctamente y ambos artefactos compartan SHA.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Binarios CI y prueba WhatsApp/Vault en servidor sin espacio

- Evidencia del usuario: `instabox-se` recuperó 4.7 GB, descargó y verificó SHA-256 de Vault/cliente del CI `44664373277` y ejecutó instalador root con resultado `CONEXIÓN REAL CONFIRMADA`, para `wa-test-1`. **La autenticación IPC en servidor está confirmada; `self-test` y WhatsApp reales pendientes.**
- Desacople identificado: lanzador Cargo por rama usa ruta `branches/alpha` y socket distinto; se prohíbe mezclar con el estado original `whatsapp-test` asociado a Vault.
- CI de Base alpha emite dos artefactos x86_64 por SHA del commit, después de pruebas: controlador y proveedor Rust sin claves/sesiones; lanzador opcional `scripts/whatsapp-vault-existing.sh` sólo reutiliza el entorno original, exige identidad Vault igual al socket y nunca compila, registra otro socket ni inicia segundo proveedor activo.
- Ver `docs/WHATSAPP_VAULT_PREBUILT.md`. CI de este commit aún por verificar; beta/stable intactos, Bot-Matter bloqueado. Autoridad Vault de roles y backup remoto siguen pendientes.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Estado cruzado de integración con Vault-Matter privado

- Cliente IPC real Base: `matter-vault-client` y `=/>vault` / `=/>vault test`, restringidos a identidad/socket autorizado. La conexión Vault/Base bajo UIDs Unix separados ya está verificada **en GitHub Actions**, pero **NO instalada en el Debian del propietario** ni probada mediante mensaje real de WhatsApp.
- Vault-Matter rama `matter`, checkpoint `2e87ec545073c7772c2ebc4f3c5a28d75ddbad3e`: añadió exportación/restauración local de archivos cifrados, `verify-root` y nuevas pruebas de recuperación. Resultado final de CI de ese SHA pendiente en el momento del checkpoint.
- La exportación local no es un backup remoto: está pendiente la réplica de ciphertext a GitHub privado y la custodia independiente de llave maestra. Base-Matter público **no recibe** la maestra ni GitHub token, y Bot-Matter permanece bloqueado.
- Paso operativo: instalar el servicio y el cliente en Debian bajo cuentas separadas usando la guía privada Vault; comprobar `ping` y `self-test` desde UID Base, integrar las seis variables de entorno en el controlador WhatsApp y entonces probar `=/>vault` en chat propio. Si faltan privilegios o concesiones: denegado, sin fallback.
- No se ha habilitado control privado, roles globales ni autoridad por actor WhatsApp. No promocionar `beta` / `stable` hasta las pruebas y políticas pendientes.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

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

## 2026-11-19 — Corrección rustfmt del selector del proveedor WhatsApp

- CI del proveedor para SHA 15baba4be23d4bb316eedbc70a3ae2c0d89c5ff3 detectó una sola diferencia rustfmt en tests/ui.rs; se aplicó exactamente, sin modificar ni categorías ni lógica de paginación, seguridad o transporte. El test offline ahora puede ejecutarse tras fmt.
- README revisados: raíz y proveedor. AGENTS, DECISION_LOG, WORK_CONTINUITY y DEVELOPMENT_STATUS actualizados. Nuevo CI pendiente. Solo alpha; beta/stable intactos y Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Selector WhatsApp de 10 categorías (alpha)

- CI anterior del proveedor fallaba en dos pruebas: el catálogo ahora tiene 10 categorías por activar Sistema (=/>vault), pero NativeFlow mantiene un límite de 9 categorías por página. No se cambia el límite ni se eliminan categorías; las pruebas ahora comprueban la primera página con navegación y la segunda con Utils + regreso.
- Seguridad: selector continúa ofreciendo exclusivamente IDs de consulta permitidos. No se habilitan comandos de escritura por botones. Cargo del núcleo Base ya había pasado; nuevo CI provider pendiente.
- README revisados: 2/2; AGENTS, DECISION_LOG, WORK_CONTINUITY y DEVELOPMENT_STATUS actualizados. alpha únicamente; beta/stable intactos; Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Corrección final rustfmt en catálogo y prueba Vault

- Se aplicaron las dos diferencias exactas del CI Linux (array de categorías y última coma en la llamada de prueba de status). No se modificó el contrato ni la autorización del comando =/>vault.
- README revisados: 2/2; estado, decisiones, continuidad y AGENTS actualizados. Este commit requiere nuevo CI Linux y Windows; beta/stable intactos, Bot-Matter bloqueado. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Test del comando Vault y categoría Sistema

- Corregidos rustfmt en la prueba owner-only de Vault y catálogo/menu raíz de 10 categorías, incluyendo Sistema ahora no vacía. El manifiesto templates/menu-visible-categories.json refleja la misma lista real. No se amplían permisos Vault.
- README revisados: 2/2; AGENTS, DECISION_LOG, WORK_CONTINUITY y DEVELOPMENT_STATUS cambiados en el mismo commit. CI nuevo pendiente; beta/stable intactos. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Comando de estado Vault protegido en WhatsApp-Rust

- El controlador =/> integra exclusivamente lectura de salud del canal Vault mediante =/>vault y =/>vault status, en el self-chat verificado y por actor dueño del socket. Opera incluso con bot apagado; NO concede roles, configura secretos ni ejecuta comandos privados.
- La configuración del cliente toma seis variables de entorno con rutas/identificadores, NUNCA la llave maestra ni contenido de la clave. Configuración incompleta bloquea startup. Errores y salida son mínimos, sin identidad ni rutas.
- Enlaces relativos README del proveedor corregidos. README revisados: 2/2. Cambios sólo alpha, con tests y CI por verificar; Bot-Matter sigue bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Ajuste exacto de formato rustfmt en CLI Vault

- El CI Rust de Base detectó un único orden de imports en src/bin/vault_client.rs; se aplicó corrección exacta sin cambios de funcionalidad. Windows ya compila el cliente limitado; CI nuevo por verificar.
- README revisados: 2/2. Decisiones y continuidad actualizadas, sin tocar beta/stable. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Base Vault IPC: corrección de Windows y formato

- Se corrigió orden de imports rustfmt y se separó secure_write por plataforma para no introducir codigo inaccesible ni warnings en Windows. Windows deniega el transporte IPC Unix y la escritura de salida protegida; Linux mantiene la implementación real.
- README revisados: 2/2. CI del SHA nuevo pendiente; actualizados decisiones y continuidad, sin tocar beta/stable ni Bot-Matter. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Corrección de formato y compilación Windows del cliente IPC

- Modulo vault_ipc y CLI ajustados a rustfmt; imports y estructuras Unix protegidas con cfg(unix), manteniendo denegación del transporte no disponible en Windows. No se alteró la identidad, cifrado o la llave maestra.
- README revisados: 2/2. Registros DECISION_LOG, WORK_CONTINUITY, DEVELOPMENT_STATUS y AGENTS actualizados. CI nuevo pendiente; beta/stable intactos.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Cliente IPC cifrado de Vault para Base alpha

- Nuevo modulo Rust vault_ipc y CLI matter-vault-client con canal Unix AEAD, identificador de cliente y secreto de transporte independiente de la llave maestra; ping/get/put restringidos a configuration, bot y socket vinculados al grant Vault.
- Las cargas se leen/escriben en archivos y nunca via argumentos de CLI. Base publico no conoce repos privados ni importa crypto store. Falta integrar autorizacion de actor WhatsApp y despliegue bajo UIDs separados.
- Cambios en codigo y docs/VAULT_IPC_CLIENT.md; README revisados: 2/2. Se actualizaron AGENTS, DECISION_LOG, WORK_CONTINUITY y DEVELOPMENT_STATUS. CI de esta revision pendiente; beta y stable intactas; Bot-Matter bloqueado.
- Creditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Corrección de rustfmt del selector y módulos (alpha)

- Error reproducido en CI Linux: únicamente diferencias de formato en src/bot_setup.rs y orden de pub mod en src/lib.rs. Se aplicaron las diferencias exactas del log; no cambia lógica de elección provider/language/system, permisos, Vault ni bot.
- Windows y el proveedor offline pasaban en el SHA anterior; Linux quedó bloqueado por rustfmt antes de ejecutar otras etapas. CI de nuevo SHA por comprobar; ambos README revisados, beta/stable intactos, Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Frontera de servicio Vault compartida sin claves en consumidores

- Base y Bot tendrán identidades independientes frente a la autoridad Vault. Base público no depende de Bot ni lleva URLs privadas, llaves maestras o source de servicios privados. Ningún bot accede al repo cifrado ni sustituye a Vault como proxy privilegiado.
- El almacenamiento local Rust cifrado del custodio pasó CI offline inicial. El gateway mTLS/IPC, roles remotos, revocación y auditoría cifrada aún no existen; por ello el puente continúa denegando. Documento: docs/VAULT_SERVICE_BOUNDARY.md.
- Revisados ambos README; cambios exclusivamente documentales. Alpha, beta/stable intactos; Bot-Matter bloqueado. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Política obligatoria de cifrado para capacidades protegidas

- Toda información operativa que atraviese una capacidad protegida se clasifica como credencial: cifrado autenticado en tránsito/almacenamiento, claves externas y segregación por bot/socket/recurso. Sin cifrado o autoridad real se **deniega**, nunca se permite fallback en texto claro.
- Base-Matter público expone solo contratos, identificadores y solicitudes; **no conoce rutas ni nombres de repositorios privados**, no descarga implementación interna y no debe poseer claves para descifrar material privilegiado. El host confiable autoriza y transporta ciphertext opaco.
- Regla vinculante: `docs/PROTECTED_CAPABILITY_ENCRYPTION.md`. Contrato existente aún no implementa canal operativo cifrado. HMAC aislado no proporciona confidencialidad. Los datos operativos y auditorías deben seguir cifrados; el descifrado temporal solo ocurre en el proceso autorizado.
- Corrección documental, no modifica runtime/WhatsApp ni demuestra cumplimiento criptográfico. No promover `alpha` hasta pruebas Linux/Windows, seguridad y proveedor. Bot-Matter sigue bloqueado. Ambos README y registros de continuidad revisados.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Selector público neutral para crear bots (alpha)

- Se añadió `src/bot_setup.rs`: contrato de selección `provider/language/system` con defaults virtuales `default/rust/host`. El núcleo NO lleva URLs, rutas ni dependencias hacia implementaciones privadas; proveedor real y permisos los resuelve un catálogo autenticado del host. Sin catálogo se deniega.
- Se añadió `matter-bot-init` como CLI **offline de solo descriptor JSON**; no genera runtime, no concede credenciales y no importa fuentes privadas. Guía: `docs/BOT_SETUP.md`.
- WhatsApp-Matter ya tiene extracción separada del proveedor Rust; no se realizó corte de sesiones ni borrado del proveedor que hoy utiliza el lanzador de Base. El gateway privado no existe aún. Bot-Matter continúa bloqueado.
- Revisar `cargo fmt --all -- --check`, `cargo check --all-targets --locked`, `cargo test --all-targets --locked`, CI Linux/Windows y proveedor tras este commit. README raíz y del proveedor actualizados, sin modificación beta/stable.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Corrección de rustfmt en puente privado (alpha)

- El CI Rust de SHA 5d0897cd8c71a895d5e7dc76777494e2ce7ffceb falló exclusivamente en etapa inicial rustfmt del puente `src/capabilities/mod.rs` y las pruebas `tests/private_capability_bridge.rs`; Windows y proveedor independiente estaban verdes. Se aplican únicamente las diferencias exactas del registro GitHub Actions, sin cambiar funcionalidad, reglas, autorización ni secretos.
- Continuidad: mantener `on|off|inherit`, `set` solo configuración; migración funcional de modos Anti pendiente. WhatsApp-Matter ya contiene copia separada de proveedor Rust y contratos Go, TS, JS; el corte de sesión no está autorizado ni probado.
- Revisados README raíz y proveedor. CI de este SHA nuevo pendiente, beta/stable intactos; Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Corrección de sintaxis Anti: on|off|inherit

- **Decisión actual:** los tres operadores de activación son exclusivamente `on|off|inherit`, tanto por filtro como por modo. El guion `-` queda DESCARTADO como operador; no implementarlo, mostrarlo ni usarlo como alias.
- `inherit` elimina únicamente el override del ámbito autorizado y hereda el valor superior. `set` permanece reservado a configuración; `anti` a consulta y activación. Se conserva `--include` / `--except` para selección dentro de modos.
- Especificación rectoría: [docs/ANTI_COMMAND_CONVENTIONS.md](docs/ANTI_COMMAND_CONVENTIONS.md). Corrección documental, **no** parser Rust, ni cambios de permisos, ni pruebas WhatsApp. Migrar runtime, help y menú antes de declarar el comando funcional. Beta/stable intactas. Bot-Matter sigue bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Reglas definitivas propuestas: Set configura; Anti activa y hereda

- **Diseño pendiente, sin cambio Rust:** `set` se reserva para configuración. `anti [regla] on|off|-` controla activación o retiro de override para heredar el valor superior; `set anti link on` queda descartado de la gramática objetivo.
- `anti --<mode> on|off|-` opera sobre un modo completo, opcionalmente limitado por `--include` y `--except` a filtros registrados (no archivos del servidor). Modos previstos: `--security` como subconjunto Anti y `--multimedia`, sin confundir Matter Security con filtros Anti.
- `--mod` conserva ámbito del socket; `--global` es predeterminado general y no puede editarse en modo vivo sin Vault. `-` nunca borra la política global. Actuaciones masivas requieren autorización y transacción/auditoría; en fallo se deniegan.
- Documento rector de continuidad: [docs/ANTI_COMMAND_CONVENTIONS.md](docs/ANTI_COMMAND_CONVENTIONS.md). Hay que actualizar router, ayuda, menú y pruebas en tarea posterior. No declarar implementado ni probado; `alpha` documentación solamente, sin modificar `beta`/`stable` y Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Puente público para capacidades protegidas (alpha, no desplegado)

- Se creó la API Rust de invocación anti.link en src/capabilities/anti_link.rs; sólo arma peticiones, no contiene implementación privada, claves ni permisos. Ausencia de gateway = rechazo. Los antis y =/>anti link existentes no cambian.
- Integradores podrán asociar su comando #antilink al puente mediante un gateway autenticado cuando exista, sin recibir código de ejecución ni acceso a repositorios privados. El ejecutor protegido deberá comprobar decisiones actuales y verificadas por alcance; los datos públicos de contexto no autentican a nadie.
- Scope: grupo, socket (--mod) y valores generales de lectura (--global). No se permiten mutaciones generales. Especificación: docs/PRIVATE_CAPABILITY_BRIDGE.md.
- README raíz y del proveedor revisados; pruebas unitarias de contrato añadidas. CI y pruebas reales pendientes; gateway privado y autoridad remota todavía NO desplegados. Alpha solamente, beta/stable intactos; Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Corrección del menú estático por ámbito

- Base previa c8554033bb0594886443a90c14dd0811576021c8. La plantilla `templates/matterssmith-menu.txt` mostraba el socket como `status --global`, heredado de una convención retirada. Se sustituyó por `status --mod`, se añadió consulta de predeterminado general `status --global` y se renombró el encendido por defecto del socket como `bot --mod on/off`.
- El cambio es de presentación; no modifica estado, privilegios ni políticas. `tests/control_style.rs` comprueba el nuevo encabezado; el contrato `help` mantiene los ámbitos equivalentes. Estado sin argumentos de los comandos implementados continúa siendo consulta, no guía.
- README revisados: 2/2. CI de este SHA pendiente; no desplegar en WhatsApp hasta completar Linux, Windows y proveedor. Beta/stable no se modifican. Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Ajuste de Rustfmt en pruebas públicas Anti

- Base previa 01822a6d32e43839c809ba0580b2b9564d931087. Se aplicaron las dos diferencias exactas de Rustfmt en `tests/whatsapp_public_readonly.rs` para las llamadas de propietario y comprobaciones de rechazo; sin cambios de lógica, autoría ni autorización.
- El controlador mantiene el estado sin argumentos para comandos realmente implementados, con ámbitos `--mod` de socket y `--global` de predeterminado general en los módulos que los soportan. Consultas públicas Anti no ejecutan acciones.
- README revisados: 2/2. CI del SHA nuevo pendiente y validación WhatsApp real pendiente. Beta/stable intactos. Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Verificación de consultas públicas Anti y estado sin argumentos

- SHA anterior f3c308ab3683693b80b05cdae1d569bcc04a2112. Las suites Linux/Windows habían superado el transporte y la mayor parte de los comandos, pero la nueva prueba de estado sin argumentos esperaba un encabezado anterior. Se ajustó la aserción a «Antis del grupo» conservando una respuesta real y distinguible del chat privado.
- Se incorporó una prueba de WhatsApp que habilita un grupo firmado y verifica que un usuario normal puede consultar `anti`, `anti link`, `anti --mod link` y `anti --global link` sin concesión de permisos, y que `anti link on` / `set anti link on` son denegados.
- Persistencia de políticas: todos OFF de fábrica, `--mod` socket, `--global` solo general sin Vault configurado. Ninguna consulta crea escrituras o acciones de moderación.
- README revisados: 2/2; CI del nuevo SHA pendiente, beta/stable sin cambios, Bot-Matter bloqueado. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Migración de pruebas HMAC/IPC al ámbito del socket

- Base previa baa59b01be636593b6742f78e66ddfcb53804db6. Se ajustaron las pruebas de `tests/control_ipc.rs` y `tests/whatsapp_bridge.rs` que enviaban el alias heredado `bot --global on` y consultaban `status --global` como si pertenecieran al socket. Las nuevas comprobaciones usan `bot --mod on` y `status --mod`, sin alterar el transporte HMAC, el antirreplay ni las firmas.
- La opción `--global` queda reservada al valor predeterminado general; en modo vivo no concede escritura global y requiere backend autenticado para guardar cambios. `--mod` es el socket propio, nunca otro.
- README revisados: 2/2. La integración de un resolvedor seguro de usuarios por respuesta/mención sigue pendiente. CI del nuevo SHA pendiente; beta/stable intactos, Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Corrección compatibilidad de estado Anti y proveedor

- Base previa: 83e0a17b47c80c595a43b0d0168d0df74cdf6675. CI Linux y Windows encontraron una sola aserción heredada en `tests/anti.rs`: la etiqueta del grupo dejó de decir «Antis del grupo». Se restaura esa etiqueta; «chat privado» distingue DM, y «socket actual» sustituye al antiguo término ambiguo «socket global».
- Se aplicó la diferencia exacta de Rustfmt del proveedor `whatsapp-rust/src/lib.rs`, sin cambiar autorización ni contenidos.
- Consultas sin argumentos devuelven estado cuando existe: bot, anti, status, set y ajustes implementados. No crear respuestas ficticias de módulos pendientes o de usuarios sin metadatos verificados.
- README revisados: 2/2. CI del nuevo SHA pendiente; no integrar en beta/stable. Bot-Matter sigue bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — CI de consultas sin argumentos, formato y controles globales

- SHA anterior: 5b229870eea827864f48961fefb2278beca8bf33. Se aplicaron 21 ajustes exigidos por Rustfmt (despachador, consultas de estado y pruebas); `bot --mod on` ahora usa el tipo Rust correcto del token `action`.
- En modo WhatsApp unificado quedan bloqueadas **ambas** formas de escritura global: `bot --global on` y `bot on --global`; `bot --mod on` es la escritura local del socket autenticado. La prueba cubre el bypass de orden de argumentos.
- Se mantiene el comportamiento: comandos con estado, sin argumentos, muestran su valor actual; `help` explica la sintaxis sin ejecutar comandos. Anti permite consulta read-only en chat activo y registra solo configuración, no moderación.
- README revisados: 2/2. CI del nuevo SHA aún pendiente; no promover a beta/stable ni confirmar WhatsApp real. Bot-Matter sigue bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Consultas de estado sin argumentos y ayuda por ámbito

- La consulta de estado es el comportamiento predeterminado de las familias **con estado**: `=/>bot`, `=/>anti`, `=/>status`, `=/>set`, `=/>set style`, `=/>set lang`, `=/>set anti` y `=/>set anti link`. En un chat privado verificado, Anti consulta la configuración de ese chat. En un chat de grupo, la de ese grupo. Comandos meramente informativos o pendientes sin estado real mantienen sintaxis de ayuda o indisponibilidad.
- `--mod` identifica **este socket**; `--global`, **el valor predeterminado general**. No mezclar ambos: el socket no es global. Para Anti, `--global` muestra el fallback inmutable OFF/warn y no un registro Vault. Para status/bot en modo vivo, `--global` solo informa la disponibilidad del fallback general y jamás el socket. `bot --global on/off` se rechaza en WhatsApp vivo; `bot --mod on/off` configura el socket.
- `=/>set` y sus variantes sin valor consultan preferencias reales de menú; `=/>set --global` reporta los valores de fábrica, no una preferencia remota configurable. Los comandos `set anti` con argumentos son escrituras verificadas; sin argumentos solo consultan. `set anti link` sin operación muestra estado y lista de dominios actual.
- En chats habilitados, usuarios normales pueden leer estado Anti mediante un canal dedicado **sin escritura**, para `anti`, `anti <tipo>`, `anti --mod`, `anti --global`; el proveedor solo firma consultas limitadas, nunca `anti link on/off`. Los detalles privados del socket y los rangos globales no se divulgan.
- Los nombres/respuestas/menciones/números para consultar usuarios siguen **pendientes de resolvedor autenticado**; este cambio no simula estados de terceros.
- Los cambios de catálogo describen `--mod` cuando la familia lo soporta y eliminan la documentación errónea «--global = socket». No anunciar soporte universal de ámbitos en comandos pendientes.
- Pruebas añadidas/ajustadas: estado sin argumentos, ámbitos, no escalada, consultas públicas, contrato de help y compatibilidad. CI Linux/Windows y proveedor siguen siendo requisito antes de activar en servidor.
- README revisados: 2/2; sin promocionar a beta/stable, Bot-Matter bloqueado. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Control de ámbito estandarizado y seguridad de consulta por usuario

- Base anterior: `c584b3d55569`. Se aplicaron ajustes de Rustfmt a `src/control_scope.rs` y orden de `src/lib.rs` sin cambiar la autorización.
- El contrato `docs/CONTROL_SCOPE_CONTRACT.md` define: chat actual, `--mod` socket actual, `--global` predeterminado general; dos banderas son inválidas. Un ámbito no otorga permisos.
- El resolver de usuario por respuesta, mención, número o `user` es un pendiente de integración con metadatos verificados, no una función pública activa. No conceder por texto un rol ni divulgar identidad privada.
- Continuidad actualizada y READMEs revisados: 2/2. Verificación CI de este SHA pendiente. `beta`/`stable` intactos. Bot-Matter bloqueado.

## 2026-11-19 — Ámbitos y estado de usuario (contrato de migración)

`--mod` identifica el socket autenticado; `--global` los valores predeterminados generales (no un alias del socket). Consulta sin bandera = chat actual. Ver `docs/CONTROL_SCOPE_CONTRACT.md`. Cualquier usuario podrá consultar solo campos públicos de otro participante por respuesta, mención, número o `user <id>` cuando exista un resolvedor de identidad verificado; esa ruta **aún no está conectada**. Se introdujo el parser común y la protección Anti lo usa; otras familias requieren migración explícita, pruebas y permisos por comando. Las escrituras globales no están implementadas y siguen bloqueadas.

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

## 2026-11-19 — Formato de resolver de comandos en ayuda

- Base alpha anterior 97472dbdca9a5a8a7f6f535fa91d73bccb9abbe4. La verificación Rustfmt solicitó reducir el filtro de nombre exacto o prefijo en `src/menu.rs` a una línea.
- Corrección puramente de estilo; ya está escrita la prueba de la ayuda por cita y de los subcomandos, y el transporte Whatsapp sigue sin ejecutar mensajes citados.
- README 2/2 revisados. CI del nuevo commit pendiente.

## 2026-11-19 — Resolver nombres de subcomandos en help

- SHA previo 42c95c8452528d70dcce19ae82b0dad187545610. El test de ayuda citado reveló que `help set role` intentaba interpretar `role` como un número de página de categoría.
- Solo una segunda palabra numérica selecciona página. Las consultas `help anti audit`, `help set role` y las citas con subcomandos ahora buscan el descriptor específico más largo; no ejecutan nada.
- Test ampliado. READMEs 2/2 revisados; CI de nuevo SHA pendiente.

## 2026-11-19 — Corrección final de formato y Clippy en ayuda citada

- Base alpha anterior 61d7af72039aa9b810a29396e263bc553ed71162. Se aplicaron diferencias exactas de Rustfmt en `src/menu.rs` y `clippy::unnecessary_lazy_evaluations` en el proveedor WhatsApp Rust (respuesta citada).
- `=/>help`, `=/>help anti`, `=/>help set anti link` y el uso respondiendo a un comando conservan seguridad y requisitos de habilitación.
- Archivos READMEs revisados 2/2; CI del nuevo SHA pendiente, aún sin verificar teléfono físico.

## 2026-11-19 — Prioridad de comandos sobre categoría de igual nombre

- SHA anterior: c41fdeb6f2fdefbaf1329376dc0f6418cc9c9a3e. Prueba real de Linux identificó un conflicto: `help anti` tomaba la categoría Anti y no la ficha de comando; también la antigua aserción `Ayuda: antilink` ya no corresponde a la nueva ficha `Ayuda completa`.
- Exactitud: el nombre exacto de comando tiene prioridad en `help`; para categorías siguen `help antis`, `help antis 1`, etc. El menú mantiene categorías sin cambios. Un comando citado se resuelve a metadata, no a ejecución.
- READMEs 2/2 actualizados y pruebas del cambio pendientes. El acceso sin registro y Anti OFF se mantienen.

## 2026-11-19 — Compatibilidad estricta del test de citas

- Base alpha anterior: 425bb2df3ac7a4541ce5a06f6eb4867e32259918. Rust provider 0.7 requiere el trait `MessageBuilderExt` para `Message::text`; en prueba `no_quote` se usa directamente el constructor explícito de `wa::Message`, sin dependencia adicional.
- READMEs 2/2 revisados. Comportamiento de consulta de ayuda citado sin cambios, y sin conceder ejecución por texto citado.
- Pruebas Linux/Windows/proveedor pendientes para el SHA de esta corrección.

## 2026-11-19 — Finalización de Rustfmt en prueba de ayuda por cita

- SHA anterior: 79b3335c521eaab335ad8f8167ae91aef34c847d. El comprobador solicitó solo el orden de imports en `tests/menu_catalog.rs`; se aplicó sin alterar lógica.
- `help <comando>` y `help` respondiendo al mensaje consultan metadatos, nunca ejecutan el comando citado. Anti sigue con ediciones/camuflaje en modo pasivo.
- README revisados 2/2; CI nuevo pendiente. No actualizar Debian hasta Linux, Windows y provider verdes.

## 2026-11-19 — Corrección CI de ayuda en mensaje citado

- SHA anterior 6e0a2956a035e7fb1f05d4f3217cb322b7aac7e7. Se corrigió la importación local de `matter_base::menu` en la prueba de ayuda por cita, sin cambiar el contrato.
- Se aplicó Rustfmt del proveedor: módulo help_reply, match del firmante read-only y orden de imports en main.rs.
- `help` citado devuelve **solo metadatos** y no ejecuta órdenes; se mantiene control por chat, tipo de remitente, HMAC, antirreplay y filtrado de manipulación.
- READMEs 2/2; esperar CI Linux/Windows/proveedor del SHA nuevo antes de probar en WhatsApp.

## 2026-11-19 — Formato y cobertura de ayuda contextual

- Base `alpha` anterior: 80eb18be86e5d35c87101164508e618906575176. Rustfmt exacto aplicado en `src/menu.rs` y `tests/menu_catalog.rs` según CI.
- `help` en respuesta a un mensaje usa citas de WhatsApp Rust sin ejecutarlas; quitada lógica duplicada en el proveedor y dependencia de test aislada bajo `#[cfg(test)]`.
- El acceso público a ayuda específica acepta `help set anti link`, `help anti audit`, `help set role`; las respuestas son solo metadatos. Reglas de bot OFF y Vault sin cambios.
- READMEs 2/2 revisados; esperar CI antes de actualizar servidor.

## 2026-11-19 — Ayuda completa y respuesta al mensaje citado

- Base alpha anterior: 9aa97bdda1113181785768da2d8ae7869f8dec52. `=/>help` solo ofrece el uso y explica la opción de responder a un comando; no muestra categorías.
- `=/>help <comando>` ahora expone categoría, descripción, estado real de catálogo, todas las formas de uso y restricciones documentadas; `help set anti link`, `help anti`, `help register` y `help set role` destacan su estado operativo.
- Proveedor WhatsApp Rust 0.7: al RESPONDER a un mensaje que contenga un comando con `=/>help`, extrae `extended_text_message.context_info.quoted_message` del mensaje recibido, rechaza otra conversación, limita longitud y envía solo `=/>help --quoted <base64url>` firmado por HMAC. El núcleo busca el nombre más específico en el catálogo, nunca ejecuta texto citado.
- Casos de prueba: quote de `set anti link`, `anti audit`, bot, registro, rol, cita cross-chat, cita ausente, contenido inválido y lectura directa. No hay delegación de permisos ni activación de comandos desde citas.
- READMEs revisados 2/2; CI SHA nuevo pendiente. Guardrails Anti conservados y Bot-Matter sigue bloqueado.

## 2026-11-19 — Ajuste Rustfmt de política Anti por grupos activos

- SHA previo d0afffdff09a5f73965f7f640b31dd7141cdf6f8. Apliqué formato de CI para `bot_admission(...).decision.enabled` de `inspect_anti_observation`; no cambia lógica.
- Monitoreo pasivo solo en grupos activados por propietario, edición cifrada sin clave permanece no inspeccionada, comandos camuflados nunca se ejecutan.
- README revisados 2/2. Pendiente CI del nuevo SHA y confirmación real en WhatsApp.

## 2026-11-19 — Privacidad y semántica de ediciones Anti

- Base previo ffb8a7a344d010f870e195571e8d0d14285e6478. La pasarela de inspección Anti registra metadatos de mensajes de grupos **solo si el propietario habilitó ese grupo** según política actual del socket. Grupos apagados se omiten; la pasarela no introduce una excepción de activación.
- Una edición clásica sin `edited_message.text_content()` ya no se marca como descifrada/legible: se considera **opaca sin inspeccionar**, igual que un sobre E2EE sin clave original.
- Sin dar permisos, ejecutar comandos normalizados ni realizar moderación. `anti audit` conserva solo conteos para grupo activo.
- Ambos README revisados; CI del SHA siguiente pendiente.

## 2026-11-19 — Corrección CI de auditoría y documentaciones del proveedor

- Base previo fdbad8252aa080c0988be136236a6a5620bc547e. Se aplicaron las diferencias de Rustfmt de la función `anti audit` y tests.
- Solucionado `clippy::empty_line_after_doc_comments` en `providers/whatsapp-rust/src/lib.rs`: documentos del firmante y del nuevo evento Anti quedan correctamente vinculados a sus métodos.
- `=/>anti audit` sólo muestra conteos, sin cuerpos/identificadores. El puente no ejecuta mensajes editados ni normalizados y todos los antis continúan apagados inicialmente.
- READMEs 2/2 revisados; CI del SHA siguiente pendiente. No habilitar el despliegue hasta tres suites verdes.

## 2026-11-19 — Auditoría de mensajes editados y camuflados

- Base alpha previo: 1aaf76e48930959f45f72bede16e4e33228d6ae4. `=/>anti audit` permite consultar contadores de ediciones legibles, cifradas sin inspeccionar, comandos disimulados y enlaces disimulados del grupo. `=/>anti --global audit` en privado del dueño agrega el socket.
- Se audita simultáneamente edición y camuflaje, sin conservar cuerpo, remitente ni identificador de mensaje. En un mismo evento pueden aumentar varios contadores; son indicadores, no personas culpables.
- Filtros OFF y acciones automáticas desconectadas, no hay borrado ni expulsión. Comandos alterados no se ejecutan.
- Clippy del test con carácter invisible: se usa escape explícito `\\u{200B}`.
- READMEs 2/2 revisados, CI del nuevo commit pendiente. Precaución: ediciones cifradas del cliente sin secreto original se clasifican como no inspeccionadas.

## 2026-11-19 — Corrección CI menú Anti y proveedor

- El SHA anterior `876d0b7420c8214937cdeb76916ab51ac3378622` ya pasó el formato del núcleo, pero Windows indicó una expectativa antigua: `set anti link` está en **página 4**, no en página 1 de Anti; test ajustado con página real. Las páginas 1–4 siguen cubiertas, sin redes sociales.
- El proveedor requirió un ajuste exacto de Rustfmt para la prueba de firma de Anti; no cambia HMAC ni código operativo.
- Menú sigue sin comandos sociales; lista chat/socket y observación Anti firmada con ediciones y camuflaje en estado pasivo. No hay moderación automática.
- README revisados 2/2; CI nuevo pendiente.

## 2026-11-19 — CI Rustfmt para puente de inspección WhatsApp

- SHA previo: 8f8d8981980aef7655651c15d9d7ea79ca978d35. Se aplicaron exactamente las diferencias de rustfmt detectadas en `src/control.rs`, `tests/anti.rs` y `tests/whatsapp_public_readonly.rs`.
- Transporte HMAC de observación Anti (solo grupo; sin acciones), estados original/editado legible/editado cifrado ilegible, antirreplay y normalización sin ejecutar texto.
- READMEs revisados 2/2; CI del nuevo SHA pendiente, todavía no autorizar actualización del servidor.

## 2026-11-19 — Observación Anti real desde WhatsApp, sin conceder permisos

- Base alpha inicial: `c4428b4e4fb9e781aea8a98a85d776d67edf2cbd`. El proveedor Rust ahora observa mensajes de grupos, incluidos textos originales, ediciones clásicas `ProtocolMessage.MESSAGE_EDIT` y sobres cifrados `MESSAGE_EDIT` identificables. Las ediciones cifradas sin clave del mensaje padre se registran como **ilegibles** y nunca se despachan como comandos.
- Un SignedFrame HMAC ya existente, con prefijo interno no disponible como orden pública, enlaza mensaje/grupo/remitente reales a un canal **exclusivamente de inspección**. El puente verifica HMAC, JID, timestamp, antirreplay y ámbito de grupo; no llama al dispatcher administrativo ni manda respuestas.
- El inspector `AntiStore::inspect_message` detecta dominios y camuflaje con metadatos de edición; solo guarda tipo/ámbito en SQLite privado, **no cuerpos ni JIDs de remitentes**. Todos los antis permanecen OFF y moderación aún no está conectada.
- Pruebas nuevas cubren firma del proveedor, edición recibida de otro participante, duplicados, firma alterada y rechazo de monitorización en privado. Se mantienen pruebas de desactivación por defecto y coincidencias de dominio.
- Edición cifrada E2EE moderna requiere recuperar `messageContextInfo.messageSecret` del mensaje padre; no se descifra automáticamente en whatsapp-rust 0.7. Una edición detectada pero ilegible debe tratarse como no inspeccionada.
- README revisados 2/2. CI de esta revisión pendiente; prueba real WhatsApp posterior, Bot-Matter bloqueado.

## 2026-11-19 — Contrato central de inspección editada y camuflaje

- Base SHA previo: 6941fa23706dc41a9005e9c37f820a7e692d18d2. Nuevo módulo `src/anti_guard.rs` con `EditState`: original, editado legible y editado cifrado NO legible. Normalización de análisis detecta separadores Unicode, caracteres invisibles, control bidi, cambios de puntuación, rutas malformadas y prefijos confusos.
- **Regla fundamental:** no despachar nunca un comando normalizado; únicamente inspeccionar y auditar. Un texto alterado o cifrado no representa permiso para ejecutarlo ni para moderar.
- `AntiStore::inspect_message` acepta el estado de edición verificado por el proveedor y aplica el detector `antilink` usando el contenido inspeccionable. Registra únicamente tipo de evento y ámbito en SQLite: ni cuerpo, ni identificador de persona, ni mensaje original.
- Los filtros sin detector específico aún NO reciben adaptadores multimedia (antiimage, antiabuse, etc.); compartir metadatos de edición será la fase de integración del proveedor. Ediciones cifradas modernas requieren clave de secreto del mensaje original en whatsapp-rust 0.7, por lo que se registran como NO inspeccionadas.
- Corrige fallo CI menú: la primera página de Anti contiene `antilink`, no la segunda tras retirar reglas de redes.
- README revisados 2/2. Código nuevo CI pendiente. Sin activar moderación, roles ni Bot-Matter.

## 2026-11-19 — Primer cierre de CI AntiLink

- SHA previo: b5b2615e6989a1fc1c29227bd921c8bab2c4bb07.
- CI detectó diferencias de formato Rust en `src/anti.rs`, `src/control.rs`, `tests/anti.rs` y `tests/menu_catalog.rs`. Se aplicaron las diferencias exactas, sin cambios de lógica.
- Los cambios funcionales anteriores: retiro de Anti por redes sociales, lista de dominios chat/socket, OFF predeterminado, SQLite privado y auditado.
- Próxima fase inmediata: análisis de mensajes editados, camuflaje y detección de protocolo. Historial y pruebas del proveedor; no ejecutar comandos alterados ni prometer lectura de ediciones cifradas sin su clave.
- README revisados 2/2; GitHub Actions nuevo SHA pendientes.

## 2026-11-19 — AntiLink por lista de dominios, sin filtros de redes sociales

- Base alpha anterior: `8b0fc5516e6d8ab7e313469bfbd05bd1103d7f30`.
- Se eliminaron del registro y del menú `antilinkwa`, `antilinktelegram`, `antilinkdiscord`, `antilinkinstagram`, `antilinkfacebook`, `antilinktiktok`, `antilinkyoutube`, `antilinkx`, `antilinkshort` y `antilinkphishing`. Las tablas antiguas conservan sus filas sin ejecutarlas ni eliminarlas; no se reactivan tras reiniciar.
- Queda solo el detector genérico `antilink`. Su lista permite dominios como `t.me`, `instagram.com` o `youtube.com` bajo un **único** filtro. Compara host exacto y subdominios (no substrings); lista vacía => no detecciones.
- Nuevo comando `=/>set anti link add|del|list <dominios>` para el **chat actual** firmado, en grupos o privado, y `=/>set anti link --global add|del|list ...` para todo el socket; alias `=/>set antilink`. El global hereda hacia cada chat. Cambios persistentes SQLite privados, auditados y autorizados al propietario; hasta 12 entradas por comando y 64 por ámbito. No altera encendido. Un global `del` no borra override local.
- En grupos `=/>anti link on|off|inherit` sigue separando configuración de habilitación. Todos los antis OFF inicialmente; aun si está ON el detector es solo lectura y NO ejecuta moderación, expulsión ni borrado automático.
- Pruebas: persistencia, herencia, aislamiento, URL con dominio exacto, rechazo de sintaxis, no ejecución de nombres retirados y menú sin entradas de redes; CI de este SHA pendiente.
- README revisados 2/2. Bot-Matter sigue bloqueado. Siguiente: verificar CI y pedir prueba WhatsApp de `set anti link` en el chat propio y un grupo autorizado.


## 2026-11-19 — Integración futura Matterssmith Commerce (solo contrato)

- SHA Base `alpha` anterior: `24349b0ae29c88ebecda571de01821fe310b8800`.
- Matriz MAT-COM-001..018 en `https://github.com/Matterssmith/Marketplace-Matter/blob/matter/docs/REQUISITOS_ECOSISTEMA_COMERCIO.md`; función de Base: interfaces neutrales, eventos y seguridad, no negocio de tienda.
- Documento `docs/COMMERCE_ECOSYSTEM_BOUNDARY.md`. Sin cambios en código, proveedores, gating, sesiones ni bloqueo de Bot-Matter.
- ZIP del propietario revisado; distingue marketplace de software, generador experimental, relevadores y Tri-Red. No se copian datos privados ni archivos completos.
- README revisado: 1/1 en este repo. Sin pruebas funcionales nuevas; solo verificación de documentación/commit.
- Próximo: mantener prioridades originales de Base-Matter, sin despliegue comercial hasta que se apruebe y se valide.

## 2026-11-19 — Corrección de formato en acceso invitado

- Base alpha anterior: 3c1fa6f325df63718eb67eefb9ce1a66ff4424e4.
- Proveedor WhatsApp verificó PASS para el primer cambio. CI núcleo detuvo por rustfmt: orden alfabético de módulos y dos bloques de tests con espaciado incorrecto. Diferencias exactas aplicadas.
- Política intacta: lectura para invitados sin registro cuando el chat esté habilitado; `=/>register` informativo; Vault no recibe datos y no se asignan roles.
- Vault-Matter `matter` actualizó contrato de invitados en `850071e1bda6f62d68eea802666e45474d379825`.
- README revisados: 2/2. Nuevo CI pendiente; no afirmar teléfono probado.

## 2026-11-19 — Modo invitado estilo demo, registro voluntario

- HEAD remoto alpha antes del cambio: ea7c0da6c579f24406093940e7677fe6a349dfbd. El usuario confirmó que se debe poder usar el bot sin registro, como una versión de prueba: **no imponer CURP, biometría, género, edad ni inscripción Vault para comandos básicos**.
- Nuevo contrato Rust `src/guest.rs`: status e información de registro disponibles sin bot activo; menú, ayuda, diagnósticos de lectura, créditos y ayuda de bot disponibles solo donde el socket/chat fue habilitado; deniega escrituras, perfil Vault, verificación civil y ejecución de código a invitados.
- `=/>register` publica estado de prueba sin crear cuenta; `=/>register start` indica que el gateway de Vault aún no está listo y NO almacena datos. Ruta pública firmada en el proveedor y HMAC/antirreplay normales, incluyendo estado OFF. Descriptor en menú Principal.
- No se lanzó Bot-Matter: el contrato es reutilizable cuando se desbloquee. Ninguna concesión automática ni limitación arbitraria por tiempo. Los bot comandos `#` aún dependen de su runtime preparado y autorizado.
- Dos pruebas nuevas: acceso invitado/read-only con bot desactivado y matriz de capacidades. README revisados: 2/2. CI del nuevo SHA pendiente. Siguiente: confirmar UI de WhatsApp y contratos Vault del invitado; continuar corrección Anti pendiente.

## 2026-11-19 — Clippy Rust del receptor ListResponse

- CI del SHA 031ebb79ad1edc7c81d68a8997ba9c11918681b5: proveedor compiló y todos sus tests pasaron; Clippy estricta falló en tests/ui.rs por un `..Default::default()` innecesario en `SingleSelectReply`. Se quitó sin modificar protocolo ni permisos.
- README revisados 2/2. CI del nuevo SHA pendiente. Siguiente: lista de dominios personalizable en vez de reglas anti por red social, después prueba real con =/>test ui.

## 2026-11-19 — Respuestas de listas WhatsApp y verificación auténtica del botón

- Base alpha: 37698e3967af5b2857c252473d5ad792a289a3e1. Una selección del menú produce burbuja, pero no dispara respuesta como el texto escrito manualmente.
- Añadida lectura de ListResponseMessage.single_select_reply.selected_row_id y TemplateButtonReplyMessage.selected_id a NativeFlow/ButtonsResponse existentes; IDs bajo lista blanca estricta. Firma HMAC, identidad y antirrepetición se mantienen.
- El ACK manual es una prueba de comando, no de botón. Solo el evento interactivo decodificado y autorizado muestra «✅ Botón verificado»; un log sin teléfonos confirma el evento.
- Test de respuestas ListResponse y de rechazo de IDs no autorizados. README revisados 2/2; CI y prueba en móvil pendientes.

## 2026-11-19 — Corrección Rustfmt + tests TS18/TS19 nativos

- Base alpha: 351d4d4a416cf8aa6b15fc33a6607bf1150763b7; Linux y Windows del cambio de menú sin fallo conocido al iniciar esta corrección, proveedor falló por `cargo fmt --check` antes de compilar.
- Aplicadas las cuatro diferencias exactas de formato que reportó CI en `providers/whatsapp-rust/src/main.rs` y `src/ui.rs`.
- Prueba nueva `native_ts18_ts19_buttons_use_quick_reply_and_readonly_ack_ids`: verifica 2/3 quick replies, los IDs cerrados Audio/MP4/Enlace, el texto mostrado y el enrutamiento de ACK. No activa permisos ni descargas.
- Las filas del selector permanecen como títulos exactos `=/>menu <categoría>` sin `Abrir menú`. En grupos y chats sin activación se mantienen las restricciones existentes.
- CI del siguiente SHA pendiente. HTML RichResponse sigue sin representación móvil confirmada. README revisados: 2/2.

## 2026-11-19 — Burbuja de selección y tarjetas compatibles

- Reporte visual Android: al pulsar «Categorías», el cliente publicaba «☾︎ Anti / Abrir menú» en vez del comando elegido. El selector ahora usa `title: "=/>menu anti"` y elimina `description`. El ID opaco y su enrutamiento HMAC siguen intactos; la burbuja depende de cómo WhatsApp represente esa fila.
- `MATTER_WA_MENU_LAB=1` añade tarjetas NativeFlow de prueba TS18/TS19 en menús raíz de **chats privados habilitados**. Sus botones únicamente emiten ACK de lectura y pasan por las comprobaciones existentes. Si el envío nativo falla, se intenta la tarjeta legacy.
- El chat privado del propietario compara adicionalmente el transporte legacy y sigue siendo el único lugar que puede ensayar Rich HTML y audio. No se envía HTML, multimedia ni comparaciones de legacy a lectores externos o grupos.
- RichResponse HTML no equivale a HTML web y el cliente puede descartarlo sin dar error: estado **experimental sin visualización confirmada**. El HTML offline continúa disponible.
- No se modifican roles, antis, activación, secretos ni derechos de ejecución. Bot-Matter permanece bloqueado. CI Linux/Windows/proveedor y prueba de teléfono para este SHA pendientes. README revisados: 2/2.

## 2026-11-19 — Instrucciones compactas y citas WhatsApp (propuesta de Leito)

- Base GitHub alpha: `8a1176ff7cdfe1d86c5c5d689c4ec2e9cd1620c0`. Nueva sugerencia de Dev MS Leito Delfin 3 Siss respaldada por capturas del menú en Android.
- `=/>menu` en estilo `classic`: sustituir seis explicaciones extensas con marcos `│` por cuatro líneas breves con prefijo literal `> ` de cita WhatsApp. Conservar encabezado, cierre, categorías visibles, botón único y estado autenticado. `status` con bot OFF, identidad para admin, externos OFF y aislamiento/auditoría siguen expresados.
- `style none`: texto igualmente compacto pero sin tipografía ni cita decorativa.
- `TextFormat::Quote` permite reutilizar citas de una o varias líneas sin introducir HTML o alterar controles. Registrar `whatsapp.quote` en `templates/message-alphabets.json`; sumar pruebas de cuatro líneas compactas y renderizado reutilizable.
- Cambios sólo de presentación: ninguna operación, permiso, categoría, sesión, Vault ni laboratorio TS18/TS19 se altera. Seguimiento en `AGENTS.md`, `docs/DECISION_LOG.md`, `docs/WORK_CONTINUITY.md`, `docs/DEVELOPMENT_STATUS.md`.
- README revisados: 2/2. CI Linux, Windows y proveedor sobre el nuevo SHA: pendiente. Captura visual del formato cita en WhatsApp: pendiente.

## 2026-11-19 — Corrección de plantilla CRLF en Windows

- Base alpha: 96bd61fc8a61ca745f91e4b89a62d4bf3611832f.
- CI Windows del SHA anterior: el test de tipografía falló porque la plantilla de texto conservó CRLF en Windows, y el cierre terminaba en retorno de carro.
- Solución: normalizar `\r\n` a `\n` antes de renderizar mensajes arbitrarios. No afecta texto, caracteres o permisos.
- Linux había completado sus verificaciones; proveedor en validación del SHA previo. Repetir los tres trabajos para el nuevo SHA.
- README revisados: 2/2; WhatsApp real pendiente.

## 2026-11-19 — Rustfmt final de selector de categorías

- Base alpha: 605ae3bf3da68de49276f1ef9d68d6e1b77f8161; Linux Rust completó correctamente sus pruebas de formato, compilación, tests, Clippy y documentación.
- El proveedor Rust falló sólo en formato de la expresión que valida el manifiesto. Se aplicó el diff exacto del CI.
- Windows y proveedor del nuevo SHA requieren nueva confirmación. Sin cambios en permisos, sesiones, Vault o plantilla visual.
- README revisados: 2/2. Continuidad: validar CI para este SHA y probar =/>menu en WhatsApp real.

## 2026-11-19 — Rustfmt y tipado del cambio de menú

- Base alpha previa: 83cc0f993e24b1ea70659c6e11d5bf8277015302.
- GitHub Actions de la revisión inicial: formato FAIL en proveedor y Rust; se aplicaron exactamente las diferencias de rustfmt de ambos trabajos.
- Se hace explícita la comparación `&str` frente a `String` del manifiesto de categorías activas, y el índice de los alfabetos JSON.
- No cambian categorías, alcance, comandos, botones, permisos ni datos de Vault.
- Siguiente: comprobar CI Linux/Windows/proveedor y después pantalla real en WhatsApp.
- README revisados: 2/2. CI del siguiente SHA: pendiente.

## 2026-11-19 — Cambio visual solicitado: tipografías y categorías activas

- Base remota alpha: 8639b9e4beab5217f344766d74c1b85578b3daba.
- Idea de Leito: negritas para títulos, `código inline` para comandos, soporte de bloques ```código```; separar información y ornamento.
- Ocultar todas las categorías sin comandos implementados en menú principal, agregación, submenús y botón de WhatsApp. Catálogo contractual de 21 entradas preservado.
- Archivos funcionales: src/menu.rs, providers/whatsapp-rust/src/ui.rs, src/message_typography.rs, src/lib.rs, templates/menu-visible-categories.json, templates/message-alphabets.json y templates/message-frame.txt.
- Pruebas de catálogo, interfaz y puente ajustadas; menús heredados por compatibilidad conservan su autenticación HMAC y permisos.
- Continuidad: revisar CI de este commit (Linux/Windows/proveedor). Si falla, corregir en alpha y actualizar estos cuatro registros. Después realizar prueba de pantalla en WhatsApp de =/>menu y selector Categorías, junto a TS18/TS19, sonido y Rich HTML opcional. Beta/stable sin cambios.
- README revisados: 2/2. CI y pantalla real pendientes.

## 2026-11-18 — Se corrigió la posición de la prueba de Grupo
El commit anterior sustituyó por error el caso POSITIVO "menu group"
y dejó Grupo en la lista NEGATIVA. Ahora la lista positiva contiene
Grupo (una de las 21 entradas) y la negativa prueba "menu unknown".
No se modificó el signer ni se ampliaron permisos. README
revisados: 2/2. Revalidación del nuevo SHA pendiente.

## 2026-11-18 — Corrección de test y checkpoint tras CI

CI del SHA 97de8ac mostró:
- La suite del proveedor marcó fallida una aserción heredada que
  rechazaba `=/>menu group`, aunque «Grupo» es una de las 21
  categorías autorizadas. El caso negativo ahora es
  `=/>menu unknown`; todas las categorías siguen probándose.
- La verificación documental detectó que el commit previo omitió
  modificar docs/WORK_CONTINUITY.md. Este commit lo corrige
  y refuerza la regla de registrar el checkpoint SIEMPRE.
- Los cambios son tests/documentación, no permisos de escritura.
- README revisados: 2/2. CI nuevo pendiente.

## 2026-11-18 — Test real de 21 categorías y documentos de WhatsApp

La plantilla antigua de pruebas sólo renderiza "menu" raíz y
rechaza submenús. Se movió la prueba de las 21 categorías y páginas
a un fixture dinámico con MenuPreferences en memoria, sin alterar
la política de producción. Corrección Rustfmt del provider lib.rs.
Se actualizaron las guías del menú TS18/TS19, RichResponse y sonido.
README revisados: 2/2. CI y prueba real del teléfono pendientes.

## 2026-11-18 — Corrección Rustfmt laboratorio y listas canónicas

El CI detectó formato no estándar en los módulos del proveedor, en
el catálogo de 21 rutas y en sus pruebas. Se aplicó exactamente el
estilo solicitado a imports, match, tarjetas y arrays de test.
No se modificó la autorización de escritura ni se habilitó IA/Vault.
README revisados: 2/2. CI del nuevo SHA por confirmar.

## 2026-11-18 — Selector: 21 categorías y páginas admitidas (pruebas)
El proveedor de entrada de lectores acepta exclusivamente las 21
categorías autorizadas y páginas 1–3, además de aliases antiguos,
siempre como lectura y sujetos a identidad PN/LID, HMAC, antirreplay
y activación. Se añadieron pruebas para cada opción y para botones
TS18/TS19 Audio/MP4/Enlace y HTML con audio no automático.
La prueba de `=/>menu group` ahora debe pasar (es una categoría
autorizada); comandos no registrados siguen rechazados.
README revisados: 2/2. CI de este SHA pendiente.

## 2026-11-18 — Laboratorio =/>menu (en desarrollo)
Proveedor Rust: se añadió tono Ogg/Opus y ensayo opt-in en chat propio
con HTML experimental, tarjetas TS18/TS19 y selector normal primero.
Sin autoplay, JS, descargas ni permisos añadidos.
CI del nuevo SHA pendiente; faltan allowlist de categorías,
pruebas e interacción real móvil. README revisados: 2/2.

## Checkpoint de implementación — 2026-11-18 (México)

**Idea del propietario:** cada nueva idea y cambio debe quedar en las
instrucciones y en el punto exacto de progreso. Se crean
docs/DECISION_LOG.md y docs/WORK_CONTINUITY.md; AGENTS.md exige
actualizarlos con docs/DEVELOPMENT_STATUS.md en cada commit. CI añade
un control automático para evitar omisiones y mantener referencia
a la revisión de ambos README.

**Permiso actualizado:** el propietario informa que Jeriel le
autorizó utilizar su clave de Aura Reed para la IA planeada y
no especificó límites. Documentado en Vault-Matter privado commit
8872f9c; no equivale a licencia activa ni habilitación de API.
No hay keys ni worker, ejecución de código u operaciones añadidas.

**Archivos modificados:** AGENTS.md, ambos README, docs/MATTER_AI.md,
docs/DEVELOPMENT_STATUS.md, docs/DECISION_LOG.md,
docs/WORK_CONTINUITY.md, scripts/check_continuity.py,
.github/workflows/rust-ci.yml, tests/menu_catalog.rs,
tests/command_audit.rs.

**Diagnóstico del SHA anterior d6ff4cb:** Linux tests/menu_catalog
falló por mezclar la cabecera de ayuda con el menú; Windows
tests/command_audit falló intermitentemente por ruta temporal
AlreadyExists. Ambas causas corregidas en esta revisión.
**Pruebas del nuevo SHA:** pendientes. Las pruebas de integración en
WhatsApp físico no se han hecho. Beta/stable permanecen intactas.
**Siguiente:** verificar CI exacto, reparar si es necesario con
actualización de todos los registros, después ensayar menú/help
en WhatsApp. README revisados: 2/2.

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

### UI de botones por categorías (alpha)

Selector NativeFlow con nueve categorías, Ayuda contextual y Ver todo. Se aceptan respuestas de botones nativos y de botones tradicionales con IDs exactos; todo pasa por el HMAC y replay. Pruebas offline del serializador y parser agregadas. Sin confirmar funcionamiento en cliente físico. README 2/2 revisados.

### CI: normalización Rustfmt de la interfaz

Las dos comprobaciones Linux se detuvieron en formato; se aplicaron sus hunks con comparación textual exacta, sin alterar comportamiento. Pendiente resultado CI del nuevo SHA.

### Corrección del primer corte de interfaz

Se corrigieron coincidencias duplicadas y el bloqueo de help privado/menu completo público antes de CI. Dos README revisados; pruebas WhatsApp y botones pendientes.

### Primer corte UX de WhatsApp, 2026-11-18

Código de menú compacto, `menu all`, ayuda contextual y control HMAC ampliado. Pruebas existentes adaptadas a la nueva cabecera; CI del SHA pendiente. Los botones/HTML y pruebas reales aún no están verificados. Dos README revisados.

### 2026-11-18 — Referencia guardada: tarjeta Go con botones `SendButtonsRow`

- Código original `ts18`/`ts19` almacenado como texto inerte en
  `docs/references/ts18_ts19_buttons.go.txt` para impedir que se compile
  como parte de Base-Matter.
- Características reales del ejemplo: miniatura cuadrada JPEG de 300×300,
  descripción con metadatos, botones Audio/MP4 (y Enlace en ts19).
- La referencia no demuestra el selector de múltiples países de otra captura:
  esa UI necesita un diseño/contrato distinto.
- Confirmación del operador: botones actuales no funcionan según lo esperado
  y el menú `antis` es difícil de leer. Rediseño e integración Rust pendientes.
- Mantener Go únicamente como referencia; no trasladar códigos de descarga,
  APIs ni dependencias sin permisos/licencia revisados.
- Auditar README raíz y README de proveedor por este commit; sin cambios
  de ejecución, ramas beta/stable, sesionamiento o protecciones.

### 2026-11-18 — CI núcleo Linux verde / proveedor formato

En `dcf39b2`, núcleo Linux: Rustfmt, cargo check,
cargo test y Clippy `-D warnings` exitosos.
El proveedor Rust independiente se detuvo
en una única diferencia Rustfmt de la lista
de categorías; se normalizó sin alterar
la autorización. Windows seguía en ejecución
al consultar. Necesaria nueva ronda CI para
SHA corregido y prueba de WhatsApp antes de beta.
README raíz y proveedor revisados (2/2).

### 2026-11-18 — Integridad de submenús y tests del proveedor

- `578c680`: núcleo Linux pasó compilación y
  tests en la ronda previa; Clippy había señalado
  nombres internos idénticos al módulo, ya resueltos.
- Proveedor Rust: 15/16 pruebas de contrato pasaron.
  Falló la prueba de rechazo de `menu group` por
  permitir una categoría arbitraria en el firmante.
  Corregido con allowlist exacta de categorías y
  páginas 1..8; añadidas pruebas de menús válidos.
- Orden Rustfmt ajustado en dos módulos de catálogo.
- Pendiente ejecutar CI completo del nuevo SHA
  Linux/Windows/proveedor, luego prueba WhatsApp.
- `beta` y `stable` siguen intactas; no mover
  sin validación. Ambos README revisados (2/2).

### 2026-11-18 — Iteración de Clippy/standalone

En `2943cedd` pasaron Rustfmt, check y
tests Linux; Clippy señaló `module_inception`
en las carpetas de comandos. Corregido con
alias internos sin suprimir el linter ni
reubicar archivos. El CI de formato del
proveedor marcó dos diferencias en el
decorador independiente, corregidas.
Reejecutar Linux, Windows y proveedor;
no considerar listo ni pasar a beta
hasta comprobar resultados. Ambos
README revisados en este commit.

### 2026-11-18 — Cierre de Rustfmt y compatibilidad proveedor

- Se corrigieron los diffs completos emitidos por
  `cargo fmt --check` del núcleo y del proveedor en
  `44ab5b35`, sin cambiar la lógica autorizativa.
- El standalone Rust WhatsApp ya no importa el
  `ControlThemes` del núcleo que depende de
  `crate::menu`. Su decorador de respaldo es
  autónomo, sin dependencias del registro.
- Los módulos por categoría siguen independientes;
  el estilo `none`/idioma y los botones requieren
  pruebas de compilación y teléfono real.
- No promover a beta hasta compilar, pasar tests
  y validar WhatsApp. README raíz y de proveedor
  revisados en el commit.

## 2026-11-18 — Menú por comando, entrega alpha

- 55 módulos de metadatos en nueve categorías;
  `antis` incluye exactamente los 32 filtros reales.
- Renderizador de menú y páginas por categoría,
  estilo classic/none e idioma español/inglés;
  sin créditos automáticos ni marca de bot personal.
- `set style`/`set lang` requieren propietario
  en privado y escriben SQLite independiente.
- Botones rápidos Estado/Menú/Antis enlazados
  a comandos firmados solo de lectura.
- `src/whatsapp_bridge.rs`: `matter` admitido
  en la lista privada, sin permisos nuevos.
- Incluye ajustes de Rustfmt y compatibilidad
  Windows del cliente Vault AI. CI nuevo pendiente.
- Gateway Vault ausente, permiso Alya pendiente,
  servicios/system/sandbox/test/eval/console y
  archivos citados pendientes. Sin promoción beta.
- README raíz y proveedor revisados 2/2.

## 2026-11-18 — Contrato de uso externo de IA y consentimiento (alpha)

- Vault-Matter `matter` en commit `427b169`: concesiones
  revocables del titular, texto obligatorio que leerá cada modelo,
  política de datos/cuotas y tipos de credenciales cifradas.
- Nuevo `src/vault_ai.rs` con broker Unix, ruta/UID esperados,
  nonce, límites y recibo de política; sin claves en Base-Matter.
- `src/matter_ai.rs` pasa a consulta del broker; no ejecuta
  comandos, cambia roles ni escribe repositorios. Extensión
  WhatsApp `matter` permanece privada para el dueño del socket.
- En ausencia de broker y autorización comprobada se deniega.
  El servidor Vault, descifrado de claves, verificación real del
  dueño de la API y comprobaciones de modelos aún **no existen**.
- La revisión `d4ead81` falló CI de núcleo por rustfmt;
  el nuevo código sustituye ese módulo y requerirá validación.
- README principal y proveedor revisados íntegramente (2/2).
  No modificar beta ni stable sin pruebas verdes.

## 2026-11-18 — Integración Matter AI (alpha)

- Reconocido el último cambio de propósito: `matter` ya no
  es atajo de `sandbox` ni consola; será asistente de
  programación. `eval` y `console` directos, `test`
  directo, `sandbox test` aislado son comandos distintos.
- Revisado Aura Reed/Aura-Bot-2.0 (master/test): no
  se encontró proveedor Venice/Claude configurado en los
  archivos de IA examinados. No se migran claves.
- Nuevo `src/matter_ai.rs`: tres protocolos HTTPS
  oficiales, elección automática Claude > Venice > OpenAI,
  credenciales/modelos externos, sin edición ni ejecución,
  respuestas acotadas, tests offline.
- Extensión `matter` instalada únicamente en el
  controlador privado; proveedor Rust permite esa palabra
  solo en ámbito propietario PN/LID/HMAC, no en público
  ni grupos. IPC dispone de tiempo especial.
- Pendiente: confirmar CI y prueba WhatsApp con clave
  válida, completar pruebas seguras para adjuntos, menú
  modular y todas las restantes operaciones. Vault no
  cuenta con lector autenticado, por tanto los roles
  permanecen bloqueados. Sin promociones a beta.
- README raíz y README de proveedor revisados 2/2.

## 2026-11-18 — Corrección de formato RichResponse (alpha)

`d8c3857` inició dos workflows. Linux compiló la base,
pero la suite independiente del proveedor se detuvo en
Rustfmt de `src/main.rs` y `tests/rich_transport.rs`
antes de comprobar el nuevo mensaje. Se normalizó todo
según el log. Se conservan tests de límites, firma y no
suplantación, con ensayo apagado por defecto y self-chat.
READMEs auditados 2/2; nuevo CI por verificar.

## 2026-11-18 — Integración compatible con trabajo paralelo Rich (alpha)

Al comparar el HEAD se detectaron dos commits concurrentes
`2dfc214` y `a6cbb19`, no presentes al inicio de esta tarea.
Se verificó por comparación que el último incluía
`rich_preview.rs`, tests, CLI offline y documentación; sus
workflows Linux/Windows y proveedor estaban en verde.
Se **preservó** ese trabajo y se descartó la implementación
duplicada aún no publicada.

Nueva capa independiente `rich_transport.rs` y tests,
consumiendo el JSON offline existente: empaqueta un
`wa::AIRichResponseMessage` sin suplantación de Meta AI.
`MATTER_WA_RICH_HTML=1` opt-in, self-chat propietario
verificado por HMAC, solo `=/>menu`. Fallo en transporte
cae a NativeFlow/texto. No se afirma que un envío aceptado
sea visible. No habilita grupos ni privilegios nuevos.
CI y prueba en teléfono pendientes para el nuevo SHA.
README raíz y proveedor auditados 2/2; sesiones, `beta`
y `stable` intactas. No se adjuntó ni ejecutó `xd.zip`.

## 2026-11-18 — Alineación Rustfmt del prototipo offline (alpha)

CI `44383479213` informó tres diferencias de Rustfmt en
`providers/whatsapp-rust/tests/rich_preview.rs` (dos accesos JSON
y una aserción); corregidas. No se alteró el diseño del menú, la
semántica de las pruebas, el puente HMAC ni sesiones.
READMEs 2/2 revisados y actualizados; se necesita CI nuevo.

## 2026-11-18 — Inspección de xd.zip y avance Rich/HTML (alpha)

- Revisado ZIP aportado por usuario **sin ejecutar código**: 10 entradas
  (4 Go, 3 HTML, 1 JS, 1 MD y un ZIP de audio). Las pruebas Go
  referencian assets de juegos ausentes. `help_rich.go` advierte
  falta de soporte de menú animado en iPhone/WhatsApp Web.
- `testeos.js` y el código Go serializan `unifiedResponse` y
  `embedded_screens`, pero dependen de un envoltorio que simula
  un bot Meta AI. No se importó ni habilitó esa suplantación.
- Rust: añadido `rich_preview.rs` para HTML local escapado y JSON
  de esquema experimental sin sesión, envíos, scripts ni permisos.
  Expuestas opciones CLI `--rich-preview-html/json` antes de
  leer credenciales; tests de estructura, inocuidad y rechazo de
  HTML inyectado; no sustituye NativeFlow ni el menú original.
- Documentación creada `docs/WHATSAPP_RICH_HTML_COMPAT.md`.
  Auditados y actualizados los dos README (2/2). Solo rama `alpha`.
  Falta CI exacto y pruebas reales Android/iPhone/Web.

## 2026-11-18 — Rustfmt de alias NativeFlow (alpha)

La verificación `44380170502` detectó exclusivamente orden
de imports en `ui.rs` antes de los demás pasos. Corrección
puntual realizada, sin tocar permisos ni sesiones. READMEs
auditados 2/2, actualizados 2/2. Repetir CI de SHA exacto.

## 2026-11-18 — Corrección estricta de Clippy en NativeFlow (alpha)

`44379206555` pasó compilación, tests offline y tres pruebas de
`tests/ui.rs`: NativeFlow, clics de solo lectura y rechazo de HTML
ejecutable. La fase Clippy detectó dos rechazos de estilo:
`needless_update` en Body y `infallible_destructuring_match` en
respuesta interactiva. Ajustados sin relajar lint.
READMEs raíz y proveedor auditados 2/2; nueva revisión CI exigida,
sin cambios en sesiones, default ni autorización administrativa.

## 2026-11-18 — CI de NativeFlow: formato pendiente y reparación (alpha)

Primera ejecución `44378651037`: el proveedor rechazó formato de
`src/ui.rs`, `src/main.rs` y `tests/ui.rs`, antes de compilación.
Se normalizaron los ocho bloques de Rustfmt y el match de respuestas.
El núcleo Linux pasó su workflow independiente; falta proveedor con
tests/clippy y Windows del SHA final. Revisión de README: 2/2.

## 2026-11-18 — Fase UI: botones Rust y HTML de estilo (alpha)

- Identificada API real de `whatsapp-rust` 0.7.0:
  `ctx.send_message(wa::Message)`, mensajes NativeFlow y respuesta
  nativa tipada. No usar Node/Baileys ni HTML web ejecutable.
- `src/ui.rs` prepara un mensaje con tres botones `quick_reply`
  (Estado, Menú y Créditos) en una sola carga útil; preserva menú
  original y convierte un pequeño fragmento HTML permitido a
  marcado WhatsApp. Rechaza etiquetas y atributos no permitidos.
- Integrada lectura de IDs con asignación estática solamente a
  acciones de consulta; ingreso firmado HMAC y antirreplay se
  conservan. Banderín `MATTER_WA_NATIVE_BUTTONS=1` habilita el
  ensayo experimental; default desactivado; fallback a texto ante
  error de envío.
- Se agregan tests offline de estructura NativeFlow, HTML y rechazo
  de IDs administrativos; queda validar Cargo, CI Linux/Windows,
  entrega real Android/iPhone y compatibilidad visual. Sin afirmación
  de que el cliente interprete HTML web ni de detección de SO.
- README revisados íntegramente: 2/2; ambos actualizados.
  Ramas beta/stable y sesiones privadas intactas.

## 2026-11-18 — Ajuste de suite independiente del proveedor (alpha)

El CI `ce57169` completó satisfactoriamente todo el núcleo Linux.
La suite de whatsapp-rust falló en una expectativa vieja:
`incoming_readonly_commands_bind_real_sender_and_never_gain_owner_authority`
asumía que el proveedor descartaba `status --global` de terceros.
El contrato actual permite firmarlo y devuelve rechazo informativo
sin datos privados del socket. El test fue alineado; se mantienen
los límites de firma, ámbito, replay y permisos. Pendiente nuevo CI.
READMEs raíz y proveedor auditados y actualizados (2/2).

## 2026-11-18 — Original Matterssmith / status de lectura (alpha)

- Restaurada `templates/matterssmith-menu.txt` con la estética exacta
  suministrada: ornamentos, Matterssmith, Base-Matter, 12 renglones y crédito
  final. El renderer no duplica el marco del menú, ni añade créditos
  automáticos a `status` y otras respuestas.
- `status` conserva la ruta HMAC pública de solo lectura para cualquier
  remitente verificado en grupos/privados, aun con bot apagado.
- `status --global` usa la rama de propiedad del socket para quien es
  dueño autenticado tanto en grupo como en chat propio. Los externos reciben
  denegación informativa y no descubren estado interno.
- Se amplían regresiones de Rust (tema, puente, ingreso público y despachador).
  CI del commit y pruebas manuales WhatsApp aún por comprobar; no ascender
  a beta/stable ni proclamar producción.
- Auditado README 2/2: README raíz y proveedor; ambos actualizados.
  Sesiones y credenciales fuera de Git; Bot-Matter bloqueado.

## 2026-11-18 — Clippy estricta: tres aserciones anti (alpha)

- Tras `7de9a0d`, GitHub Actions Linux aprobó `cargo fmt` y alcanzó
  `cargo clippy --all-targets -- -D warnings`; Clippy informó exactamente
  tres comparaciones booleanas con `false` en `tests/anti.rs`.
- Corregidas mediante aserciones negativas equivalentes. No se relajó
  el pipeline ni se activaron las reglas de moderación.
- El workflow separado `44367046939` del proveedor `whatsapp-rust`
  finalizó con éxito en `7de9a0d`. La verificación Linux/Windows
  completa debe repetirse para el nuevo SHA.
- README raíz y proveedor revisados (2/2) y actualizados (2/2).
  Sin acceso ni alteración de sesiones del operador. Ramas beta/stable intactas.

## 2026-11-18 — Reparación de CI de antis/créditos en alpha (pendiente verificación)

- Diagnóstico GitHub Actions sobre `6142c6c`: las comprobaciones de formato fallaron
  en `src/bin/whatsapp_control.rs`, `src/anti.rs`, `src/control.rs`,
  `src/control_style.rs`, `src/lib.rs` y `tests/anti.rs`.
- Windows terminó la compilación y detectó fallo de expectativa obsoleta
  en `tests/live_unified_controls.rs::credits_are_explicit_and_do_not_turn_on_any_chat`.
- Aplicados los cambios de formato emitidos por Rustfmt, unificado el texto
  de `=/>credits` con crédito exacto y fundadora, y ajustada la prueba.
- READMEs revisados: 2 de 2 (raíz y proveedor); ambos actualizados para
  distinguir correcciones de CI de validación final. `AGENTS.md` revisado
  y conservado. Ninguna sesión WhatsApp ni rama beta/stable modificada.
- **Pendientes:** confirmar CI Linux/Windows + proveedor para nuevo SHA;
  revisar errores adicionales, probar comandos de protección sin moderación
  destructiva y efectuar prueba autorizada de WhatsApp real. No liberar.

## 2026-11-18 — Ajuste final de respuestas y pruebas IPC (alpha)

- `src/control_ipc.rs` elimina de los cuerpos de respuesta el prefijo
  `Base-Matter:` y `Base-Matter |`, porque el único marco ya indica
  `Control`. Acorta los acuses de política y deja intactos HMAC,
  autorización, replay y sesiones.
- `tests/control_ipc.rs` exige el único marco reducido y el texto
  conciso; se revisaron las expectativas de estilos comunitarios,
  menú y proveedor contra los nuevos archivos de plantilla.
- La compilación, Clippy y pruebas sobre este lote de GitHub
  **siguen pendientes** de ejecución Debian. No dar por
  instalado el cambio en el WhatsApp del operador hasta actualizar
  y reiniciar el proceso.

## 2026-11-18 — Limpieza de menú y separación de créditos (alpha)

1. **Prueba previa:** el operador confirmó todo el conjunto Base,
   Clippy estricto y 21 pruebas del proveedor; publicó el commit
   `61b45ee` (formato Rust). No se deben confundir esas
   comprobaciones con los nuevos commits visuales.
2. **Cambios en GitHub alpha:** reemplazadas las dos plantillas
   de presentación. Un marco corto, sin título Matterssmith/
   Base-Matter repetido ni footer de créditos; prefijo global
   `=/>` solo una vez en menú; variantes `status [--global]`
   y `bot [ámbito] on|off|inherit` agrupadas. Sin categorías.
3. **Nuevo control de lectura:** `=/>credits` y aliases
   `creditos/créditos`, autorizado por el núcleo y por los
   filtros del puente HMAC/proveedor Rust. Fuera del propietario,
   solo es visible con política de bot activada. No cambia
   roles, sesiones, conexiones ni políticas de activación.
4. **Validaciones de código:** validador de plantillas conserva
   límite, rechazo de controles y marco `{{BODY}}`; `TITLE`
   y `ACCENT` son opcionales para conservar temas de grupo.
   Nuevas expectativas en las pruebas del menú, IPC, estilos
   comunitarios, proveedor y lectura pública.
5. **Referencia externa:** revisión de Aura y del ZIP YoSoyYo.
   Aura incorpora una vista HTML experimental y botones nativos
   mediante su implementación TypeScript/Baileys; el ZIP usa
   `quick_reply`/`cta_url`. La heurística Android/iPhone del
   ZIP se basa en prefijos/longitud de IDs y no constituye
   identificación fiable. No copiar código Node ni inferir SO.
6. **Pendientes reales:** ejecutar Rustfmt, Clippy y tests en
   Debian sobre el SHA de esta revisión, comprobar créditos
   y menú en WhatsApp real, investigar envío nativo de botones
   con un indicador de compatibilidad verificable. No hay
   garantía de incrustar HTML de Android en iOS o de identificar
   automáticamente su plataforma. `beta/stable` intactas.

## 2026-11-18 — Segunda referencia posicional al snapshot de políticas (alpha)

1. **Resultado Debian:** `cargo clippy --all-targets --locked
   -- -D warnings` detuvo la compilación en
   `tests/whatsapp_native.rs:82` con `E0609`, porque la prueba
   aún utilizaba `.0` sobre `BotDefaultPolicies`.
2. **Cambio en GitHub alpha:** la prueba consulta el campo
   `socket_powered`. El método, modelo de datos, política
   de autorización y flujo de WhatsApp no se modificaron.
3. **Auditoría estática:** comprobados los consumidores en
   `src/control.rs`, `tests/live_unified_controls.rs`,
   `tests/whatsapp_bridge.rs`, `tests/whatsapp_native.rs`
   y otros módulos de control / pruebas de seguridad. Sin
   otras referencias posicionales encontradas en los inspeccionados.
4. **Pendiente:** repetir Clippy estricto en Base y proveedor,
   pruebas completas, verificador de README y `rustfmt`.
   El SHA de esta corrección no tiene ejecución CI confirmada.
   Preservar stash, parches y `providers/whatsapp-rust/Cargo.lock`.
5. **Promoción:** `beta` y `stable` sin cambios. No promover
   revisiones cuyo Clippy o pruebas no estén verificados.

## 2026-11-18 — Referencia residual en prueba WhatsApp Bridge (alpha)

1. El operador ejecutó Clippy después de migrar
   `inspect_bot_defaults()` de tupla a `BotDefaultPolicies`.
   Falló en `tests/whatsapp_bridge.rs:111` con `E0609`:
   el campo `.0` dejó de existir.
2. Se cambió solo la prueba para leer `.socket_powered`
   y se aplicó el estilo esperado por rustfmt. El núcleo
   y la integración WhatsApp no cambian de comportamiento.
3. Las suites funcionales anteriores estaban aprobadas,
   incluidas las pruebas del proveedor; **el nuevo commit**
   requiere compilación, tests y Clippy estrictos en Debian.
4. Ramas `beta/stable` sin modificación. Preservar los
   cambios locales de `cargo fmt`, stashes y el archivo
   `providers/whatsapp-rust/Cargo.lock` sin seguimiento;
   no usar comandos destructivos.
5. Siguiente: sincronizar `alpha` sin sobrescribir los
   cambios locales, verificar primero `cargo clippy
   --all-targets --locked -- -D warnings`, luego las
   suites completas, Clippy del proveedor y auditoría.

## 2026-11-18 — Refactorización contra clippy::type_complexity (alpha; validación pendiente)

1. **Evidencia en Debian:** las pruebas visibles de `cargo test
   --all-targets --locked` pasaron, incluido
   `tests/whatsapp_native.rs`, `tests/whatsapp_theme_scopes.rs`,
   el puente y los accesos públicos de solo lectura.
   `cargo test --manifest-path providers/whatsapp-rust/Cargo.toml
   --all-targets` pasó **21 pruebas**.
2. **Bloqueo concreto:** `cargo clippy --all-targets --locked
   -- -D warnings` falló en `src/activation.rs` por
   `clippy::type_complexity` en una tupla con seis componentes;
   `cargo check` y `cargo test` sí la compilaron.
3. **Cambio publicado:** `BotDefaultPolicies` es una estructura
   con campos explícitos que mantiene la semántica anterior.
   Ajustados el consumidor de `=/>status --global` en
   `src/control.rs` y las comprobaciones de activación
   en `tests/live_unified_controls.rs`. No se inhibió el lint.
4. **Pendientes reales:** sincronizar con seguridad los cambios
   locales de formato, ejecutar `cargo fmt`,
   `cargo clippy --all-targets --locked -- -D warnings`,
   suite completa, Clippy del proveedor, auditoría de README.
   Comprobar si surge otro lint nuevo. Sin CI verificado del SHA
   ni prueba manual WhatsApp real del nuevo commit.
5. **Ramas:** `alpha` exclusivamente; no promover a
   `beta/stable` hasta pasar los controles.

## 2026-11-18 — Corrección de orden de validación en WhatsApp Native (alpha)

1. **Hallazgo real del operador:** `cargo test --all-targets --locked`
   avanzó con pruebas aprobadas hasta `tests/whatsapp_native.rs`,
   que terminó con 4 aprobadas y 1 fallida:
   `duplicated_expired_and_malformed_events_fail_closed`.
2. **Causa:** `ingest` clasificaba el alias retirado `=/>on`
   como `Ignored` antes de validar que su texto excediera
   los 4096 bytes. La prueba esperaba un rechazo explícito.
3. **Corrección publicada:** validación del tamaño e ID del
   mensaje antes del filtro de alias retirados. Se añadieron
   pruebas de desbordamiento tanto para `=/>on` antiguo como
   para `=/>bot on` vigente y de no consumo del replay ID.
   La activación real **sigue siendo solamente**
   `=/>bot on/off`.
4. **Pendiente:** ejecutar `cargo test --test whatsapp_native
   --locked`, después `cargo test --all-targets --locked`,
   suite del proveedor y Clippy. Las comprobaciones de la
   nueva corrección no tienen resultados Debian todavía.
   Mantener `alpha` como única rama de desarrollo.

## 2026-11-18 — Regresión de estilo detectada en Debian (alpha)

- **Evidencia:** operador sincronizó hasta `19b5fc6`;
  `cargo check --all-targets --locked` pasó. La suite
  `cargo test --all-targets --locked` pasó hasta
  `tests/control_style.rs`, donde 1/2 casos falló en
  `assert!(...contains("community-one"))`.
- **Diagnóstico confirmado en código:** `community-one` era el
  `bot_id` de `ControlTheme::new` y el esquema de plantilla
  predeterminado contiene `{{TITLE}}`, `{{ACCENT}}`,
  `{{BODY}}`, pero no el identificador de bot. El nuevo
  menú global no presenta ese metadato a usuarios.
- **Corrección publicada:** `tests/control_style.rs` ya
  verifica el título visual particular de un grupo, el
  tema predeterminado de otro y la aplicación del diseño
  particular en mensajes de estado. El núcleo no cambió
  para satisfacer una expectativa obsoleta.
- **Pendiente real:** el operador tiene cambios locales de
  `cargo fmt` en archivos rastreados y el archivo
  `providers/whatsapp-rust/Cargo.lock` sin seguimiento.
  Guardar el formato rastreado en un stash **sin -u**,
  avanzar alpha con fast-forward y reformatear nuevamente;
  mantener el stash como respaldo y el `Cargo.lock`
  intacto. Ejecutar primero `cargo test --test control_style
  --locked`, después suite Base, proveedor y Clippy.
  No hay resultados CI del nuevo SHA ni verificación live.
- **Promoción:** `beta/stable` sin cambios; no subir
  cambios incompletos desde el servidor.
  
## 2026-11-18 — Endurecimiento de temas y avisos (alpha; recompilación pendiente)

- `ControlTheme::decorate` valida tamaño y caracteres de
  control **antes** de aceptar un cuerpo aparentemente ya
  formateado, evitando que el camino idempotente evite
  la barrera de transporte.
- Actualizadas las pruebas de estilos: una plantilla
  personalizada queda idempotente y rechaza controles
  incrustados y respuestas excesivas.
- `bot_admission` incluye la clase `community` en los
  alcances externos que **deniegan por defecto**. El
  ajuste `=/>bot community on/off` solo se persiste en
  el ámbito de tipo; no debe deducirse que cualquier
  JID `@g.us` sea grupo de avisos.
- **Sin compilación en esta sesión:** el ambiente de
  revisión no dispone de `cargo/rustfmt`. Se requiere
  `cargo fmt --all -- --check`, `cargo check
  --all-targets --locked`, suite completa de Base y
  prueba del proveedor WhatsApp en el Debian del operador.
  No publicar en `beta/stable` sin resultados verdes.

## 2026-11-18 — Plantillas seleccionadas en todos los controles (alpha; validación pendiente)

1. **Implementado:** el registro `ControlThemes` expone una selección
   común y aplica `decorate` por chat. `WhatsAppControlIpc`
   recibe el `Arc<ControlThemes>` compartido con el menú.
   El identificador de grupo se obtiene del marco firmado
   **después** de una validación `Applied`; nunca es tomado
   de un mensaje sin autenticar para decidir su presentación.
2. **Proveedor Rust:** las respuestas ya compuestas por IPC se
   entregan sin aplicar nuevamente la plantilla general.
   Los informes REGISTER/recuperación que crea el proveedor
   sí usan el renderizador compartido. Esto evita marcos
   duplicados y conserva el tema personalizado del grupo.
3. **Prueba incorporada:** `tests/whatsapp_theme_scopes.rs`
   comprueba diseño de grupo en `status`, `bot on` y
   `bot off`. El tema predeterminado preserva el estilo
   del propietario y no copia categorías del bot de ejemplo.
4. **Pruebas y resultados:** cambios publicados en `alpha`.
   Los últimos `cargo check/test/fmt` informados en verde
   correspondían a `dfd9dab`, **antes** de este lote.
   La compilación del lote actual y la prueba en WhatsApp
   real **no están verificadas**. Sin promoción a `beta/stable`.
5. **Pendientes:** compilar y corregir las pruebas en Debian;
   comprobar otro grupo y un tercero, revisar el tamaño
   de respuestas especiales, fijar `Cargo.lock` del proveedor,
   conectar lector autenticado de Vault-Matter y verificar
   metadatos de canales/comunidades antes de habilitarlos.

Revisados README raíz y README del proveedor, documentación
de política y guía de prueba. Créditos conservados.

## 2026-11-18 — Contrato final del propietario: bot-only y temas externos (alpha; tests pendientes)

1. **Implementado en alpha:** se retiran `=/>on/off` de la
   firma del proveedor, del puente HMAC y de la ruta nativa
   WhatsApp. La interfaz pública de activación es
   `=/>bot on/off/inherit` con `--global`, `group`,
   `private`, `channel` y `community`/`avisos`.
   La función `set_live_bot_policy` actualiza el valor elegido
   y despierta el socket en la **misma transacción** al activar.
   Apagar un chat no desconecta el resto de grupos.
2. **Plantillas desacopladas:** añadidos
   `templates/matterssmith-control.txt` y
   `templates/matterssmith-menu.txt`. `ControlTheme::decorate`
   y `with_template` validan tamaño, marcador de contenido,
   caracteres de control y créditos. El controlador IPC aplica
   la misma plantilla a todas sus respuestas; el proveedor
   Rust importa exactamente el mismo renderizador para REGISTER
   y vistas previas de recuperación. Menú sin categorías.
3. **Alcances:** los grupos y privados externos siguen
   apagados hasta habilitación explícita. `kind:community`
   guarda el predeterminado para grupos de avisos, pero no
   se aplica en WhatsApp hasta tener metadatos autenticados.
   Los canales también precisan identificación específica.
4. **Pruebas actualizadas:** `live_unified_controls`,
   `control_style`, `control_ipc`, `whatsapp_bridge`,
   `whatsapp_native`, `whatsapp_process`,
   `whatsapp_public_readonly` y provider. Falta ejecutar
   las suites completas en Debian y corregir regresiones
   que aparezcan; no declarar 100% sobre este commit.
5. **Ramas:** `alpha`, `beta` y `stable` existen, pero
   sus historiales divergen. Se requiere promoción selectiva
   por PR al pasar pruebas y una prueba real en WhatsApp.
   No se modificó Vault-Matter, la sesión ni las ramas
   beta/stable. Los roles siguen bloqueados sin su autoridad.
6. **README revisados** (raíz y proveedor), instrucciones
   y guía del ensayo WhatsApp. Créditos mantenidos:
   Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-18 — Corrección de aserción grupal (alpha, test Debian pendiente)

- **Evidencia nueva:** el operador probó el commit
  `b57f6c0`. `cargo test --test control_ipc --locked`
  pasó 6/6; en la suite principal las suites mostradas
  pasaron hasta `tests/group_control.rs`, que terminó
  1/2 debido a una expectativa obsoleta.
- **Fallo exacto:** la prueba
  `group_control_rejects_nonowners_and_wrong_chat_scope`
  exigía `is_err()` a `=/>status` del dueño dentro de
  `120363111@g.us` con socket apagado. El nuevo contrato
  permite consultar estado grupal aun cuando todo está
  apagado; el resultado esperado es
  `GroupStatus { enabled: false, source: PoweredOff }`.
- **Acción:** corregido `tests/group_control.rs` para
  validar la respuesta esperada y añadir una comprobación
  separada de rechazo de `status` administrativo para
  un remitente ajeno; no se debilita ninguna política.
- **Verificación pendiente:** ejecutar
  `cargo test --test group_control --locked`,
  `cargo test --all-targets --locked` y
  `cargo fmt --all -- --check` en el Debian real.
  La suite del proveedor WhatsApp Rust ya había pasado
  20 pruebas en la ejecución anterior.
- Persisten las condiciones de publicación:
  Vault-Matter como única autoridad de roles, ninguna
  escritura local de roles en runtime, sesiones
  protegidas y `beta/stable` intactas. Revisados ambos
  README de Base-Matter.

## 2026-11-18 — Verificación Debian y corrección de expectativa IPC (alpha, segunda ejecución pendiente)

1. **Evidencia real aportada por el operador:** `git pull`
   avanzó desde `21bbfcf` hasta `46ffe7d`; `cargo check
   --all-targets --locked` terminó correctamente. La suite
   independiente de WhatsApp Rust terminó con **20 pruebas
   aprobadas** (1 + 15 + 3 + 1). El servidor tiene
   aproximadamente **5.1 GB** de espacio disponible.
2. **Regresión localizada:** `cargo test --all-targets
   --locked` se detuvo en
   `control_ipc::disabled_chat_menu_is_silent_but_status_and_reactivation_work`.
   En `tests/control_ipc.rs:170` el test esperaba `Ignored`
   tras `=/>bot private off`, pero el resultado fue
   `Applied`. El controlador actuó conforme al contrato
   vigente: privado externo y chat propio tienen ámbitos
   distintos (`kind:private` frente a `kind:self`).
3. **Corrección publicada en alpha:** renombrada la prueba
   y actualizado el caso para comprobar que `private off`
   no silencie el menú propio; añadido `bot off` en el
   chat propio, que **sí** debe silenciarlo, seguido de
   `status` y `bot inherit` para comprobar recuperación.
   No se alteró la implementación IPC ni se ampliaron
   permisos; no se tocaron Vault-Matter, beta o stable.
4. **Pendiente real:** ejecutar en Debian el caso focalizado,
   `cargo test --all-targets --locked` completo y
   `cargo fmt --all -- --check`. Todas las pruebas
   posteriores a `control_ipc` siguen sin verificarse
   en esta última ejecución. Mantener intacto el archivo
   local `providers/whatsapp-rust/Cargo.lock` hasta
   decidir su seguimiento reproducible.

## 2026-11-18 — Revisión final de contratos IPC; esperar pruebas reales de Debian

- Añadida `tests/control_ipc.rs::disabled_chat_menu_is_silent_but_status_and_reactivation_work`:
  firma HMAC, menú ignorado sin respuesta, status permitido, encendido de socket y activación
  de política `bot --global`, nuevo silencio al desactivar el tipo privado.
- Se mantuvo el catálogo de estilos Matterssmith sin copiar el enlace, nombre ni
  comandos de otro bot. La gestión persistente del tema, importación de
  `#menu`, IA `--auto` y capacidades Vault todavía están pendientes.
- CI exacto aún no disponible mediante estado GitHub; pruebas pendientes de
  ejecutar con `cargo fmt`, `cargo check` y `cargo test` en Debian.
  `alpha` no está calificada para promoción.
- Revisión documental: README raíz, README del proveedor, instrucciones
  y guías de trial revisados; se conservaron apartados históricos etiquetados.

## 2026-11-18 — Pruebas de regresión ampliadas y migración de menú (alpha, revisión final pendiente)

- Se corrigió el transporte IPC para diferenciar un resultado
  `ignored` legítimo, que el proveedor no debe responder,
  de una operación no autorizada o error de comunicación.
- Se amplió el acuse del proveedor de 1024 a 8192 bytes,
  con tamaño del mensaje validado hasta 3800 bytes y
  rechazo de caracteres de control. Añadida regresión de
  menús largos y límites inválidos. Sin aumentar el tamaño
  máximo de frames de entrada ni desactivar HMAC/replay.
- El nuevo `ControlTheme` presenta categorías y créditos
  Matterssmith. `help/h` dejan de existir. `menu/menú`
  es válido para chats autorizados con política habilitada;
  `status/estado` se admite aun apagada.
- `tests/bot_policy.rs` prueba estado apagado, configuración
  predeterminada independiente del encendido, herencia,
  aislamiento grupal, silencio y `--debug`. La prueba
  `tests/whatsapp_bridge.rs` autentica y comprueba el
  cambio `bot` y el estado grupal a través del puente HMAC.
- El sufijo `--debug` se interpreta solo para `=/>`,
  con comprobación de propietario y diagnóstico depurado.
  Las operaciones REGISTER/recuperación mantienen su
  manejo especial cuando se añade `--debug`.
- Se actualizaron encabezados de código y se marcaron
  apartados obsoletos de los README como históricos,
  conservando el historial sin declarar esos controles
  como actuales.
- **Pruebas a ejecutar:** cargo fmt/check/test del núcleo
  y del proveedor independiente en Debian. No se observó
  CI sobre el SHA exacto. Estado live nuevo no validado;
  no se ha modificado beta ni stable.
- **Pendientes:** acceso read-only de terceros y canales,
  Vault como servidor real de autorización, `set style`
  persistente, importación visual `#menu` autorizada,
  `--auto` supervisado, `self/selfmod/mod`,
  descubrimiento automático del catálogo de comandos,
  y el resto de comandos históricos. No afirmar completitud.

## 2026-11-18 — Default-deny para grupos/privados externos/canales (alpha)

- Endurecida `bot_admission`: aunque la política
  `--global` esté encendida, un grupo, privado externo
  o canal sin política explícita de su tipo se considera
  apagado. La excepción precisa de grupo sigue teniendo
  prioridad. `inherit` retorna al tipo y ausencia implica
  denegar. La auto-conversación conserva `kind:self`.
- Ajustadas las pruebas de grupos HMAC, privados externos
  y políticas por tipo para verificar el bloqueo por defecto.
  `status --global` indica explícitamente los valores
  externos deshabilitados que no han sido configurados.
- El usuario debe ejecutar Cargo y una prueba real de
  terceros; no afirmar que las nuevas lecturas públicas
  funcionan en WhatsApp hasta recibir ese resultado.

## 2026-11-18 — Ingreso de consultas públicas PN/LID firmadas (alpha; compilación y dispositivo pendientes)

- Añadida la ruta separada para `status/menu/bot` de otros
  participantes en grupos y privados externos, limitada
  a consultas. Firma HMAC vincula el JID real de remitente
  de WhatsApp y el chat exacto; conserva la comprobación
  de sesión PN/LID y ventana temporal. No permite
  administración del socket, `--global`, `--debug`, rol
  ni comandos del bot normal.
- Se añadió `ControlDispatcher::dispatch_public_readonly`
  con `ControlThemes` explícitamente render-only; la ruta
  ordinaria owner sigue aislada. El chat propio usa
  `kind:self`; el estado `kind:private` no lo afecta.
- Pruebas nuevas: `tests/whatsapp_public_readonly.rs`,
  regresiones de `tests/bot_policy.rs` y firma/denegación
  pública en `providers/whatsapp-rust/tests/provider.rs`.
  Es necesario ejecutarlas en Debian para confirmar
  compilación y después comprobar la recepción real desde
  otra cuenta autorizada del grupo.
- Conserva la prohibición de copiar rangos locales como
  fuente de verdad. Vault-Matter solo puede autorizar
  cuando exista un lector autenticado; sigue sin conectar.
- Canales y adaptación automática de temas/IA no están
  implementados. No asumir que todos los usuarios de
  privados PN/LID mixtos recibirán respuestas sin pruebas.
  No se promueve a beta/stable.

## 2026-11-18 — Nuevo modelo de activación del bot y menú contextual (alpha; compilación pendiente)

- Evidencia real de WhatsApp proporcionada por el operador:
  el dueño hizo `=/>help` y `=/>group on/status` en un grupo.
  La respuesta `PoweredOff` es consistente con el `=/>off`
  privado anterior, sin ser una prueba de fallo de grupo.
- La nueva prioridad solicitada sustituye `help/h` por menú
  categorizado, introduce `status` local/global, `bot`
  por chat/tipo/socket, herencia, `set` de ayuda, alias
  españoles y diagnóstico `--debug` limitado al propietario.
- El controlador Rust implementa políticas por clase mediante
  claves internas persistentes `kind:group/private/channel` en
  el almacén de activación existente; el grupo exacto tiene
  preferencia. `inspect_bot_defaults` lee los valores sin
  ocultarlos por el encendido operativo. La propiedad del
  socket se mantiene al escribir o inspeccionar.
- La capa WhatsApp ahora admite `status/menu/bot` en el
  propio chat y grupos del dueño, y retira `help/h`. La
  opción `with_silent_bot_commands` impide respuestas de
  funciones normales cuando el bot está apagado, sin romper
  extensiones de otros anfitriones del núcleo.
- El menú usa un tema Matterssmith separado de los comandos
  y los enlaces externos del ejemplo. Los límites de IPC
  se ajustaron de forma acotada; el proveedor reconoce un
  acuse silencioso auténtico. Pruebas nuevas en
  `tests/bot_policy.rs` y prueba de estilo/alias.
- Pendientes reales: ejecutar Cargo fmt/check/test en Debian,
  corregir cualquier regresión, pruebas en grupo/chat propio,
  identidad autorizada de usuarios externos y canales,
  tema persistente editable, importación visual de `#menu`
  verificado, autorización `self/selfmod/mod` por Vault,
  decisiones IA voluntarias y demás comandos históricos.
  El proyecto Bot-Matter sigue bloqueado. Sin promoción
  a beta/stable.
- README raíz, README del proveedor, instrucciones y
  contrato `docs/BOT_POLICY_V2.md` revisados/actualizados.
  Compilación y funcionamiento live de este nuevo commit
  **NO están verificados todavía**.

## 2026-11-18 — Corrección adicional de pruebas del proveedor WhatsApp Rust (alpha; Debian pendiente)

- El operador ejecutó la suite general de Base-Matter: las pruebas mostradas,
  incluidas `group_control`, `socket_control_roles`,
  `socket_vault_outbox`, `vault_role_fail_closed`,
  `whatsapp_bridge` y `whatsapp_process`, pasaron.
  Una prueba opcional del contrato remoto de Vault quedó omitida por falta
  de `MATTER_VAULT_CHECKOUT`.
- La suite independiente del proveedor WhatsApp Rust falló al compilar
  `tests/recovery.rs:97`: `env!("CARGO_BIN_EXE_whatsapp-matter-rust-provider")`
  no estaba disponible en tiempo de compilación.
- Se corrigió `tests/recovery.rs` con resolución opcional en compilación
  y alternativa de variable en tiempo de ejecución. Los comandos del
  test reciben el ejecutable por referencia para evitar mover el `PathBuf`.
- Se ajustó la prueba del alias LID de `tests/provider.rs`: `=/>status`
  sí está permitido en el chat privado propio; `=/>menu` solo pertenece
  al ámbito grupal y debe ignorarse en privado.
- Revisados el README raíz y el README del proveedor. Pendientes:
  `cargo fmt --manifest-path ... --all -- --check`, `cargo check` y
  `cargo test --all-targets` del proveedor en Debian; no se declara
  prueba real de grupos ni transporte Vault funcional. Sin promoción
  a `beta` o `stable`.

## 2026-11-18 — Cambio de arquitectura: Vault-Matter autoridad única de roles

- Solicitud expresa: Base-Matter solo lectura de rangos; Vault-Matter
  conserva toda la escritura y es la fuente única. Un rol que no esté
  registrado/autorizado en Vault no debe funcionar.
- WhatsApp/Telegram/Discord serán adaptadores autenticados para
  solicitudes de administración a Vault, no réplicas ni bases paralelas.
  Vault valida identidad, guarda una revisión cifrada y confirma la
  persistencia antes de responder éxito; Base consume un contrato de
  autorización autenticado, con denegación por ausencia, fallo,
  caducidad o revocación.
- Mitigación inmediata en alpha: retirado del ejecutable vivo
  `whatsapp-matter-control` el `RoleStore` local y su cableado.
  El código experimental de RoleStore continúa para pruebas offline,
  pero **no es fuente de permisos** y no se carga al procesar comandos
  WhatsApp. Los cambios de rango se rechazan hasta tener Vault real.
- Nuevo documento `docs/VAULT_ROLE_AUTHORITY.md` define separación
  de funciones, contrato de solo lectura, escrituras por gateways,
  revisión, auditoría, cifrado, recuperación y fail-closed.
- La política operativa local de encendido de socket y grupo continúa
  separada de los rangos. Esto NO significa que Base pueda asignar
  rangos o disponer de claves de escritura de Vault.
- Pendientes: servicio Rust de Vault, seguridad de los adaptadores,
  firma de decisiones, origen confiable de claves, escritura cifrada
  autenticada del repositorio, consulta verificada, revocaciones,
  pruebas offline y de WhatsApp real. Aún no promover a beta/stable.

## 2026-11-18 — Vault-Matter metadata alignment and encrypted role regression (alpha; no live sync)

- The authenticated private Vault-Matter repository remains separate from
  Base-Matter. Renamed manifest display metadata to `Vault-Matter` and
  corrected the Base-Matter GitHub link, preserving vault ID/schema and
  ciphertext format. Updated the Vault README with strict encrypted
  explicit-role persistence, CAS and externally supplied key requirements.
- Added `tests/socket_vault_outbox.rs`: local-only fake sink + ephemeral
  test encryption verifies a non-default PN role produces an encrypted
  envelope and reverting to implicit `user` produces an empty-grant
  encrypted snapshot. No production data or keys were shared.
- Runtime still does not enqueue live changes or publish to GitHub; the
  authenticated Vault transport and remote recovery must be implemented
  and tested before any real role persistence is claimed.
- CI, lint, root/provider tests and real WhatsApp group messages are
  pending external verification on Debian. No promotion to beta/stable.

## 2026-11-18 — Real owner group control and scoped rank recovery (alpha; CI/live groups pending)

- Operator confirmed **real** WhatsApp self-chat responses to
  `=/>status`, `=/>on`, `=/>help` and `=/>menu`.
  Prior provider PN/LID authorization fix is live-proven for private commands.
- Source audit found an incorrect private/group split: the provider
  silently ignored all group JIDs and the controller had no group context.
- Added verified owner group signing and routing over the existing HMAC
  protocol; bridge derives group scope exclusively from the signed
  `chat_jid@...g.us`. Source remains authenticated PN/LID and `from_me`.
- Private owner chat now permits only socket-wide settings; group
  `menu/help/start/add/add++` and owner `group on/off/inherit/status`
  are group-scoped. The group policy retains its independence from
  socket power and normal bot handlers are still disabled.
- Added socket/group role control over a dedicated RoleStore SQLite,
  independent of BotRegistry and project staff. `owner` is nonassignable;
  `user` is an implicit empty grant. Exclusive changes and immutable
  audit are transactional, and PN/LID subjects must be numeric.
  A role record does **not** confer control execution by itself.
- New tests: group policy scopes, HMAC group frames and replays,
  private/group allowlists, PN/LID owner signatures and local exclusive
  role changes. CI and actual Debian `cargo test` for these SHAs not yet
  observed; no promotion to beta/stable.
- Private `Matterssmith/Vault-Matter` was inspected. Its manifest,
  envelope schema and README exist, but no authenticated runtime
  GitHub sync or recovery and no external key/binding provider have
  been installed. Existing encrypted outbox cannot be treated as
  complete remote role persistence. No role data were pushed in plaintext.
- README root, provider README and instructions updated; other
  requested legacy commands requiring bot binding and a real Vault sink
  remain pending. Preserve WhatsApp sessions and user data.

## 2026-11-18 — WhatsApp Rust conectado; corrección PN/LID (alpha; prueba de respuesta pendiente)

- Evidencia del operador: `cargo run` en una única terminal con `CARGO_TARGET_DIR` compartido completó compilación y obtuvo `WhatsApp session connected`.
- Bloqueo real observado: `outgoing control ignored (verified PN/LID owner mapping required)`.
- Contrastada la API pública whatsapp-rust 0.7.0: el cliente ofrece PN y LID autenticados, y Jid permite normalización sin sufijos de dispositivo.
- Modificado el proveedor para validar PN de sesión contra el teléfono configurado, aceptar PN/LID propios en chat y remitente, y firmar un frame canónico HMAC. Mantiene rechazo de terceros, grupos, mensajes entrantes y sesiones ajenas.
- Añadidas pruebas offline para combinaciones PN/LID verificadas, denegación de identidad extraña y LID configurado. No se requirió rotación de llaves ni cambio de base SQLite.
- Revisados README raíz y README del proveedor. Quedan pendientes compilación y `cargo test` en Debian, respuesta efectiva de `=/>status` y `=/>menu`, reinicio y recuperación de sesión. Ninguna promoción a beta/stable.

## 2026-11-18 — Instalación única y compilaciones compartidas (alpha; prueba Debian pendiente)

- El operador informó agotamiento de disco al clonar/compilar alpha, beta y stable por separado y retiró el entorno local.
- Corregida en el lanzador alpha la ruta del controlador para que respete `CARGO_TARGET_DIR` absoluto; antes asumía `$root/target` y fallaba al compartir compilaciones.
- Preparación recomendada: un único clon Git con cambios de rama secuenciales, CARGO_HOME común y un único directorio target, sin perder las sesiones separadas por rama.
- Compilación liviana recomendada: sin debug e incremental, limitada por trabajos en paralelo. Cambiar de rama con provider parado; el lanzador está disponible solo en alpha.
- Revisados ambos README y actualizada esta bitácora. CI, ejecución Debian y sesión WhatsApp real siguen pendientes. Ningún código experimental promovido a beta ni stable.

## 2026-11-18 — Recuperación del arranque WhatsApp Rust (alpha; ejecución Debian y CI pendientes)

- Comparados los tres ZIP aportados: `Matter-Base-matter-1.zip` incluye el arranque Rust directo y el `#ping` de prueba; los otros dos no constituyen ese punto de recuperación.
- Corregidos los accesos `env!(CARGO_BIN_EXE...)` del test del proveedor, siguiendo el patrón del controlador ya probado.
- Incorporado `cargo run` como lanzador Rust para una única terminal; el ejecutable administra el controlador local y el proveedor Rust separado, preservando la separación de SQLite y el control global `=/>`.
- El script de configuración admite una ruta y puerto por rama sin sobrescribir sesiones anteriores; `alpha`, `beta` y `stable` continúan aisladas y no se han fusionado.
- Añadido diagnóstico sin JID, teléfono ni secretos cuando un comando saliente autorizado por origen no coincide con el JID provisional PN/LID configurado.
- Revisados los dos README y actualizado el procedimiento de arranque. No se ha probado conexión WhatsApp ni respuesta real con este nuevo SHA. No promocionar hasta que Debian y CI confirmen compilación, tests y recepción real.

# Matter-Base development status

## 2026-11-17 — Rustfmt del parser add global (alpha; CI pendiente)

- Aplicados 18 cambios de formato exactos extraídos de GitHub Actions al parser y sus pruebas.
- Revisados ambos README completos (2/2), conservados sin cambios; esta corrección no toca sesión, permisos ni ramas estables.
- Validar test del parser, Windows, Linux y proveedor sobre el SHA definitivo.


## 2026-11-17 — Parser de add conectado al control global (alpha; CI pendiente)

- Parser tipado `src/add_commands.rs` con `add user/group/community/backup`, destino `official --register`, nombre `--Nombre`, y `--hide`, sin mutaciones y con pruebas offline.
- Controlador Rust registra `=/>add` con el mismo control de dueño de socket y contesta con un acuse genérico, sin números de teléfono ni acciones administrativas. Menú universal incorpora la orden sin decoración.
- README raíz y proveedor auditados íntegramente (2/2), ambos actualizados. AGENTS, documentación ADD_COMMANDS y bitácora sincronizados.
- Falta autorizar acción con metadatos WhatsApp reales; CI Rust Linux/Windows y proveedor pendiente para este commit. La beta existente no se modificó.


## 2026-11-17 — HMAC test tamper determinista, sin flakiness (alpha, CI pendiente)

- El CI reprodujo un fallo intermitente del test `rejects_forged_signatures_cross_socket_and_sender`: reemplazaba los primeros dos bytes hex por `ab`, que podían coincidir con la firma original. La prueba ahora alterna `ab/ac` garantizando siempre una mutación.
- No se alteró la verificación HMAC de producción, se corrigió únicamente la prueba. Ambos README leídos íntegros (2/2) sin cambios adicionales. Requerimos CI verde antes de fusionar a beta.


## 2026-11-17 — Prueba real de path traversal en CLI y rustfmt (alpha, CI pendiente)

- Se corrigió test `--recovery-verify` para probar realmente el nombre `../unsafe` en lugar de un argumento extra, evitando una falsa cobertura de seguridad. Se aplicó el formato requerido por rustfmt.
- Revisados ambos README de forma íntegra (2/2), sin cambios. Mantener las pruebas del SHA exacto antes de fusionar a beta.


## 2026-11-17 — E2E offline del CLI recovery (alpha; CI pendiente)

- Test Rust ejecuta el binario real con `--recovery-verify`, clave y snapshot propios en directorio 0700, verifica lectura sin crear SQLite, sin exponer JIDs, y rechaza clave errónea y nombre de archivo inválido.
- Se revisaron íntegros ambos README (2/2) y se actualizaron. La conexión real WhatsApp y migración por aceptación siguen pendientes.


## 2026-11-17 — Rustfmt del CLI de verificación (alpha, CI pendiente)

- Se aplicaron 2 ajustes de formato en el CLI offline `--recovery-verify` y los imports del proveedor según GitHub Actions.
- Revisados los dos README completos (2/2); ninguna modificación necesaria, credenciales y sesiones intactas. Volver a comprobar provider CI.


## 2026-11-17 — Verificación local del respaldo tras pérdida de sesión (alpha; CI pendiente)

- Agregada opción offline `--recovery-verify <filename>.enc.json` al proveedor, con comprobación del filename, clave, AAD de socket/dueño e integridad; no abre WhatsApp ni expone JIDs o llaves.
- README raíz, proveedor y guía operativa revisados completos (2/2 README) y actualizados para `=/>add++`, snapshot y prueba de archivo cifrado. Contrato de recuperación sincronizado. CI pendiente, sin cambios en `beta` ni en `stable`.


## 2026-11-17 — Rustfmt de callback de snapshot (alpha; CI pendiente)

- Corregida línea extra antes de `snapshot_community_recovery` según reporte rustfmt del proveedor. Ambos README revisados en su totalidad (2/2), sin cambios necesarios; claves y sesiones intactas.
- Continuamos CI antes de cualquier prueba de WhatsApp real.


## 2026-11-17 — Rustfmt de snapshot cifrada (alpha, CI pendiente)

- Aplicados ajustes exactos de rustfmt de GitHub Actions al nuevo módulo criptográfico y sus tests. README raíz y proveedor revisados completos (2/2) sin cambios de contenido.
- Aún faltan pruebas de compilación y la integración real por pairing code; no se modifican sesiones ni grupos.


## 2026-11-17 — Callback =/>add++ snapshot con backup cifrado (alpha; CI pendiente)

- Conectado el comando `=/>add++ snapshot <community-jid>` al provider: verifica admin PN/LID de la comunidad activa, obtiene miembros vinculados, deduplica alias conocidos y cifra datos localmente con clave separada.
- La respuesta al chat propio confirma el nombre del archivo y el conteo, sin JIDs; errores no afirman éxito. `=/>add++` continúa como vista previa.
- Actualizado el setup `--enable-recovery` para comprobar OpenSSL. Leídos íntegros ambos README (2/2) y actualizados. Contrato de recuperación actualizado. No se agregan miembros ni se cambia la rama beta.
- CI y prueba real de WhatsApp son requisitos para calificar el respaldo. En caso de suspensión no se hace bypass de restricciones de cuenta.


## 2026-11-17 — Almacén de respaldo cifrado en proveedor (alpha; CI pendiente)

- Nuevo módulo Rust `providers/whatsapp-rust/src/recovery.rs` con XChaCha20-Poly1305, key dedicada, nonce aleatorio, AAD de socket/dueño, archivos privados 0600 sin overwrites, lectura autenticada y tests offline de integridad y separación de claves.
- Actualizado setup para nuevas configuraciones y `--enable-recovery` explícito para existentes, sin sobrescribir sesiones ni regenerar claves previas. Snapshot todavía NO está conectado al callback WhatsApp: falta certificar esa etapa.
- Revisados ambos README completos (2/2), ambos actualizados. Contrato de recuperación actualizado; dependencias proveedor añadidas, CI pendiente. No se suben claves ni listas en texto claro.


## 2026-11-17 — Formato de escaneo add++ (alpha, CI pendiente)

- Dos ajustes rustfmt exactos al callback del proveedor detectados por GitHub Actions. Sin cambios de lógica, sesiones ni miembros.
- Auditoría de los 2 README completos (2/2), sin cambios de contenido. Volver a verificar CI antes de promover.


## 2026-11-17 — Corrección de tipo del subcomando add++ (alpha; CI pendiente)

- Se resolvió E0308 en Windows: el matcher de `&[String]` no permite comparar `"snapshot"` como patrón `&str`. Ahora se valida la longitud y el texto directamente antes de autorizar la instantánea.
- Corregidos 2 detalles de rustfmt en `tests/control_extensions.rs`. README raíz y proveedor revisados íntegros (2/2) sin cambios de contenido. No se han incorporado usuarios ni alterado comunidades.


## 2026-11-17 — =/>add++ lee miembros de comunidades autorizadas (alpha; CI pendiente)

- El proveedor Rust ahora consulta `get_linked_groups_participants` después del acuse HMAC de Base para comunidades con admin PN/LID verificado. Excluye las suspendidas, deduplica identificadores disponibles y envía solo conteos aproximados al self-chat.
- El nombre sugerido para el grupo de respaldo conserva el nombre de la comunidad con el sufijo `- Respaldo`. Sin creación de grupos ni incorporación de participantes.
- `snapshot` declara claramente que todavía no se guarda un respaldo. Límite 12 comunidades, 15.000 participantes por comunidad y 20 s de consulta.
- Dos README revisados completos (2/2), ambos actualizados; contrato de recuperación ampliado. Pendiente CI y prueba real; `beta` permanece sin cambios.


## 2026-11-17 — Control =/>add++ para recuperación (alpha; CI pendiente)

- Se añadió la sintaxis exacta `add++` al registro global y al controlador Rust con permiso de dueño de socket, independientemente del encendido del bot. El modo inicial ofrece un inventario y reserva el comando explícito `snapshot <community-jid>`.
- Cualquier mutación `apply` y otras variantes no aprobadas son rechazadas; no se intenta incorporar a usuarios automáticamente, dado que WhatsApp impone privacidad, permisos y límites.
- Se añadieron pruebas de aislamiento entre sockets, spoofing del prefijo y rechazo de variantes, y el contrato `docs/COMMUNITY_RECOVERY.md`.
- Auditoría íntegra de los dos README (2/2); ambos actualizados. AGENTS complementado. Pendiente CI y lectura real del proveedor. No se tocó `beta` ni `stable`.


## 2026-11-17 — Reparación de regresión de formato neutro (alpha; CI pendiente)

- GitHub Actions detectó una aserción obsoleta en `tests/control_style.rs`: el texto plano `Bot ID` ya no coincide con la cadena decorativa anterior `BOT ID`. Ajustada la aserción sin cambiar la funcionalidad.
- Los dos README (raíz y proveedor) fueron revisados en su totalidad, sin cambios adicionales; seguridad y sesiones intactas.
- Repetir CI completa antes de promover `beta`.


## 2026-11-17 — Corrección de formato e instrucciones para beta WhatsApp (CI pendiente)

- Corregido rustfmt del menú plano. Actualizadas las instrucciones: `stable` ya es rama predeterminada; solo deben permanecer `stable`, `beta`, `alpha`, retirando refs heredadas sin perder commits únicos.
- Revisados enteros ambos README (2/2) y actualizados. Guía de prueba con `beta` y comandos `status/menu/help/h/start/on/off` corregida.
- Documentada la intención exacta de importar SOLO decoración de un menú real de otro bot; el observador externo y la importación automática NO están implementados.
- No se han eliminado ramas con herramientas que carecen de esa capacidad. No se copiaron sesiones ni se modifica `stable`.
- Se requiere CI verde antes de promover alpha a beta y pairing code real para promover a stable.


## 2026-11-17 — Menú universal plano para WhatsApp (alpha; CI pendiente)

- Corregida la interpretación: el propietario no pidió que Base inventase un menú decorativo; solo pide tomar estética de un menú real de otro bot. El menú universal ahora se presenta sin marcos, con comandos que realmente puede ejecutar el controlador.
- Los dos README se revisaron íntegros (2/2) y actualizaron. La futura importación de estética requiere observar una respuesta real de un bot autorizado, sin copiar comandos ni credenciales.
- No cambian seguridad, sesiones o permisos. Verificar CI antes de promover a beta.


## 2026-11-17 — Prueba del mensaje IPC autorizado (alpha; CI pendiente)

- El test TCP de lazo local verifica que `applied` para control `on/off` incluye una respuesta informativa, mientras que el replay `duplicate` no contiene el campo `message`.
- El test visual comprueba que el título de tema se muestra junto al bot ID.
- READMEs raíz/proveedor inspeccionados completos (2/2), ambos preservados. Sin mutación del proveedor ni sesiones.


## 2026-11-17 — Formato de menú comunitario (alpha; CI pendiente)

- Aplicadas cinco modificaciones exactas de rustfmt reportadas por GitHub Actions en `src/lib.rs`, `src/control_style.rs` y controlador WhatsApp.
- Ambos README verificados íntegros (2/2) sin cambios. No se alteró la lógica, sesiones ni permisos.


## 2026-11-17 — Render de menu/help/h hasta self-chat WhatsApp (alpha; CI pendiente)

- Conectado el renderer `ControlTheme` con tres órdenes globales del ControlDispatcher: `=/>menu`, `=/>help`, `=/>h` (alias); todas requieren dueño del socket. El título de la plantilla ahora se muestra en la cabecera.
- El proveedor Rust maneja `ControlAck::AppliedWithMessage` para entregar el texto validado al chat propio; el inventario `=/>start` conserva su informe especializado. Se ignoran duplicados/rechazos.
- README raíz y del proveedor auditados íntegramente (2/2) y actualizados. Sin acceso a sesiones externas, sin cambios administrativos y sin pruebas de teléfono aún.
- CI del SHA exacto pendiente.


## 2026-11-17 — Acuse opcional con texto validado en proveedor Rust (alpha; CI pendiente)

- El proveedor interpreta `applied` con un campo opcional `message` acotado a 760 bytes y sin controles, compatible con respuestas antiguas sin ese campo. No admite mensajes en `duplicate` o `denied`; tests ampliados.
- Los 2 README fueron leídos íntegros (2/2), actualizado el README del proveedor. No se enviará el campo hasta enlazar el callback.


## 2026-11-17 — Respuesta de texto validada del puente IPC (alpha; CI pendiente)

- El receptor Rust puede incluir `message` únicamente al acuse `applied` para status, encendido/apagado y extensiones; límite 760 bytes y sanitización. La API `dispatch()` conserva el retorno anterior.
- El transporte conserva el HMAC de entrada y el replay; las denegaciones/duplicados no filtran contenido. Falta que el proveedor reconozca el nuevo campo.
- 2 README revisados íntegros (2/2), actualizado solo raíz. Documentación IPC ampliada. CI pendiente.


## 2026-11-17 — Rechazo temprano de caracteres de control en #matter (alpha; CI pendiente)

- Se corrigió el fallo reproducido en Linux/Windows: un NUL tras `#matter` se ignoraba como un prefijo inválido antes de validarse. Ahora se rechaza toda entrada con caracteres de control antes de analizar el separador.
- 2 README inspeccionados íntegros (2/2), sin cambios necesarios. Sin cambios en sesiones, autorizaciones ni ramas estables.


## 2026-11-17 — Render de menu/help/h por comunidad (alpha, CI pendiente)

- Nuevo `src/control_style.rs`: renderer reutilizable para `menu`, `help`, `h`, con título, bot ID, créditos y temas por comunidad; 2 tests comprueban aislamiento de temas y rechazo de entradas malformadas.
- Es una API visual sin autoridad sobre roles ni sesiones. No se afirmó que se haya duplicado el diseño de un bot externo sin tener su código.
- Ambos README auditados íntegros 2/2; actualizado solo README raíz. La entrega real de menús en WhatsApp sigue pendiente.


## 2026-11-17 — Ajustes de rustfmt para REGISTER (alpha; CI pendiente)

- Aplicados los tres ajustes de formato exactos solicitados por las dos lanes de CI (controlador, test de alcance y escaneo de proveedor).
- Revisados íntegros los 2 README (2/2), sin cambios necesarios; sesiones y permisos intactos.
- Pendiente el resultado de las pruebas de compilación reales del adaptador y del controlador.


## 2026-11-17 — Descubrimiento REGISTER mediante API WhatsApp Rust (alpha, CI pendiente)

- El proveedor Rust consulta comunidades reales con `get_participating`, `get_subgroups` y `query_linked_group` después de que Base autoriza `=/>start`. Verifica admin del JID propio PN/LID, subgrupo vinculado e identificador exacto en primera línea.
- Devuelve un resumen de solo lectura en self-chat, con límites de alcance/tiempo. Errores dan aviso de inspección incompleta y ninguna mutación; no crea ni configura grupos.
- READMEs íntegros auditados 2/2; ambos actualizados. Contrato de comunidad actualizado; CI y la sesión real siguen pendientes para este commit.


## 2026-11-17 — Identificador REGISTER exacto en WhatsApp Rust (alpha; CI pendiente)

- Añadida función pura `official_register_description` en el proveedor Rust, con validación del JID y primera línea exacta del ID comunitario; incorpora pruebas para coincidencias, inyección y prefijos parciales.
- README raíz y proveedor revisados íntegros (2/2); solo README del proveedor actualizado. La descripción no concede autorización; falta lectura real de comunidades.


## 2026-11-17 — Extensión =/>start protegida en Base (alpha, CI pendiente)

- Registrada `=/>start` en el proceso controlador Rust mediante la autorización del ControlDispatcher; funciona con bot apagado sin conceder autorización a otros sockets.
- `=/>start apply` y argumentos no admitidos fallan cerrados hasta completar planes de mutación; agregado test de alcance/rol por socket.
- README raíz y proveedor revisados íntegros (2/2); actualizado solo raíz. Sin sesión WhatsApp real en esta fase; queda por conectar el descubrimiento del proveedor.


## 2026-11-17 — Parser externo #matter en alpha (CI pendiente)

- Agregado `src/external_compat.rs` como normalizador sintáctico no autenticador de `#matter` a `=/>`, con 2 tests de regresión para prefijos y caracteres de control.
- Ambos README leídos íntegros (2/2) y conservados sin cambios; no se copiaron sesiones ni se integraron bots externos reales.
- Falta aprobación CI del commit. El adaptador externo aún debe autenticar actor/socket y pasar por InboundGateway.


## 2026-11-17 — Ramas stable/beta/alpha y contrato REGISTER (alpha)

- Se crearon nuevas ramas GitHub `stable` desde `matter`, `beta` y `alpha` desde `matter-beta`; las anteriores se preservan. GitHub continúa con default `matter` hasta una actualización de la configuración.
- Documentados `=/>start` (descubrimiento/planificación y fases de aplicación) y `#matter` para bots externos con IPC autenticado; nada de compartir o copiar sesiones activas.
- Se habilita CI en `alpha`, `beta`, `stable` y ramas legadas sin modificar la estabilidad de WhatsApp.
- Auditoría README: 2/2 revisados íntegramente; ambos actualizados. Documentos nuevos: BRANCH_STRATEGY, COMMUNITY_REGISTER y EXTERNAL_BOT_COMPAT. Esta fase es especificación, no prueba de ejecución de mutaciones WhatsApp.
- Bot-Matter sigue bloqueado. Fase siguiente: servicio de detección seguro y tests, más adelante adaptación de WhatsApp Rust con cuenta de prueba.


## 2026-11-17 — Auditoría de afirmaciones de integración (pendiente CI)

- Root README and provider README reviewed in full (2/2). Corrected root statements that were true before, but became false after the Rust provider and loopback listener landed. Provider README reviewed unchanged.
- Updated architecture, WhatsApp bridge and release-gate documents to separate offline two-process success from an unverified live WhatsApp phone link and self-chat reply.
- No runtime code, Cargo.lock, credentials or SQLite sessions changed. No stable `matter` promotion; release requires real pairing, revoked-session and reconnection testing.


## 2026-11-17 — Prevención de eco en respuesta privada (CI pendiente)

- Añadido caso de regresión al proveedor Rust para demostrar que el mensaje de acuse «Base-Matter: solicitud procesada.» no se interpreta como un nuevo comando `=/>`; evita ciclos de autorespuesta.
- Auditoría íntegra de los 2 README (2/2), sin cambios necesarios. No se modificaron sesiones, claves o la rama estable.


## 2026-11-17 — Coherencia de documentación del acuse en WhatsApp (CI pendiente)

- Corregidas descripciones históricas en los dos README que afirmaban que el proveedor nunca respondía por WhatsApp. Ahora distinguen el acuse genérico de `applied` de una respuesta completa de `status`.
- Revisados íntegramente 2 README (2/2), ambos actualizados, sin cambios funcionales. Continúa pendiente la verificación real de recepción/envío en teléfono y CI del SHA.


## 2026-11-17 — Confirmación mínima en WhatsApp del control del dueño (CI pendiente)

- El callback de `whatsapp-rust` intenta responder en el self-chat privado del propietario con un texto fijo sin prefijo, exclusivamente después del acuse `applied`; no responde a `duplicate`, `denied`, `ignored`, grupos ni otros chats.
- Se conserva el control HMAC y replay antes del efecto, sin respuestas de estado falsas; un fallo en el envío del acuse se registra sin intentar repetir el comando.
- 2 README revisados íntegramente (2/2), ambos actualizados; guía WhatsApp actualizada. No se inicia Bot-Matter, no se cambia SQLite ni las claves.
- CI en Rust y prueba real de recepción/respuesta en dispositivo son obligatorios para estabilidad.


## 2026-11-17 — Formato del sondeo local del controlador (CI pendiente)

- Se eliminó una línea de separación adicional entre funciones en `providers/whatsapp-rust/src/main.rs`, según rustfmt CI; sin cambios lógicos.
- Ambos README revisados completos (2/2), sin cambios. Verificar en CI antes de promover.


## 2026-11-17 — Receptor activo antes del pairing (CI pendiente)

- El proceso `whatsapp-rust` verifica TCP loopback en su receptor Base-Matter antes de abrir el SQLite del proveedor o iniciar la solicitud de pairing code. Si no escucha, falla sin tocar sesiones ni credenciales.
- Modo `--check` permanece estrictamente offline. Test Tokio con listener local disponible/no disponible añadido; ningún secreto ni mensaje de control es transmitido durante el sondeo.
- README raíz y README proveedor leídos completos (2/2) y actualizados, guía operador actualizada. La disponibilidad TCP no sustituye autenticación HMAC ni prueba real de teléfono.
- CI del proveedor pendiente para el nuevo commit; estable `matter` sin cambios.


## 2026-11-17 — Formato del test E2E de procesos (CI pendiente)

- Corregidos tres detalles de rustfmt en el test nuevo de procesos del controlador, sin alterar su comportamiento.
- Se revisaron los 2 README íntegros (2/2), sin cambios adicionales; la prueba interproceso debe aprobar Linux y Windows antes de aceptar el checkpoint.


## 2026-11-17 — Regresión E2E real entre procesos Rust (CI pendiente)

- Añadida `tests/whatsapp_process.rs`: ejecuta el binario real del controlador Rust con configuración temporal privada, realiza `--init`, arranca TCP loopback, envía comandos HMAC, comprueba replay, interrumpe bruscamente el proceso y reinicia sobre el lock v2.
- Comprueba que el replay de mensajes emitidos antes de reiniciar sigue bloqueado y que el SQLite de activación/replay persiste. Solo usa material de prueba ficticio; no conecta un teléfono ni abre una sesión real.
- Auditados íntegramente los 2 README (raíz y proveedor): 2/2; actualizado únicamente raíz. Sin cambios en proveedor, llaves de operación, runtime o rama estable.
- Requiere CI Linux/Windows en el SHA exacto y la posterior vinculación autorizada por pairing code.


## 2026-11-17 — Formato de locks v2 (CI pendiente)

- Se aplicaron los dos ajustes exactos de rustfmt de GitHub Actions al proveedor y receptor Rust. Sin modificaciones de semántica, secretos ni sesiones.
- READMEs verificados íntegros 2/2, sin cambios de contenido; el diseño de recuperación v2 sigue pendiente de CI en el SHA corregido.


## 2026-11-17 — Bloqueos nativos recuperables v2 (CI pendiente)

- Migrados los locks exclusivos de controlador y proveedor Rust a marcadores v2 persistentes + `File::try_lock` de Rust 1.89+, sin dependencias nuevas ni cambio de formato SQLite.
- Un crash libera automáticamente el bloqueo a nivel de OS y permite la siguiente ejecución sobre el mismo lock v2; intentos concurrentes se rechazan. Los locks v1 con PID y los archivos inválidos fallan cerrados hasta revisión manual, evitando coexistencia con procesos legados.
- Los modos `--check` verifican el estado v2 sin crear ni borrar archivos de sesión. Tests de exclusión, reanudación y rechazo legacy ampliados.
- README raíz y proveedor inspeccionados íntegros (2/2), ambos actualizados. Guía de prueba actualizada, crédito intacto. No se promueve a estable sin CI y prueba real de WhatsApp.


## 2026-11-17 — Corrección de tipo del acuse del proveedor (CI pendiente)

- Se corrigió el error E0308 reportado por GitHub Actions: `deliver` devuelve el `ControlAck` validado (Applied/Duplicate), no `()`.
- Los dos README fueron revisados completos (2/2), sin cambios necesarios. Sin modificación de HMAC, sesiones ni claves.
- Pendiente CI del SHA corregido y posterior verificación real con WhatsApp.


## 2026-11-17 — Normalización de rustfmt del resultado IPC (CI pendiente)

- Ajustados en el proveedor dos detalles exactos de rustfmt identificados en GitHub Actions: separación de tipos y anidamiento de callback `match`.
- READMEs revisados completos 2/2 (raíz y proveedor), ambos sin cambio en este commit. Ninguna sesión ni llave fue modificada.
- Reejecutar el CI del proveedor en el SHA exacto antes de afirmar que el contrato de acuses quedó aprobado.


## 2026-11-17 — Resultados IPC tipados, sin falsos positivos (pendiente CI)

- Corregida la interpretación de acuses de `whatsapp-rust`: `applied` y `duplicate` se reportan por separado; acuses ignorados, denegados, inválidos, extralimitados o con campos desconocidos fallan cerrados.
- Añadido un test Rust de regresión del contrato de respuesta; sin cambios en el formato HMAC, en la sesión SQLite ni en el control global.
- Revisados íntegros los 2 README del repositorio (2/2), ambos actualizados; guía de prueba ajustada; créditos y licencias conservados.
- Pendiente CI de proveedor en el SHA exacto, y validación de pairing code en dispositivo real antes de promocionar a `matter`.


## 2026-11-17 — Corrección de formato IPC (CI pendiente)

- Rustfmt en el primer CI IPC identificó dos líneas del nuevo test; se aplicó el diff exacto requerido antes de ejecutar el resto de las verificaciones.
- Se volvieron a revisar íntegros ambos README (2/2), sin cambios; no se tocó la lógica de seguridad, sesiones ni la rama estable.
- Pendiente resultado del CI en Linux, Windows y proveedor para el SHA corregido.


## 2026-11-17 — Plazo total para la recepción IPC (CI pendiente)

- El receptor local de WhatsApp aplica 16 KiB y un límite global de 4 segundos por frame, en lugar de reiniciar un plazo por cada lectura. Las conexiones parciales o excesivas devuelven un rechazo sin ejecutar efectos.
- Se añadió un test de TCP real para frames parciales y excesivos. No se cambian HMAC, mensajes del usuario ni sesiones reales.
- Revisados los dos README íntegros (2/2); README raíz actualizado y README del proveedor revisado sin cambios. Documentación IPC ampliada y créditos existentes intactos.
- Quedan pendientes CI, una prueba real con teléfono, reconexión y evaluación de saturación del mismo host.


## 2026-11-17 — Bloqueos privados desde creación (CI pendiente)

- Hardening de los dos procesos Rust de WhatsApp: los locks exclusivos de controlador y proveedor se crean en modo Unix 0600 sin ventana intermedia; se conserva create_new, no se borran sesiones ni se activan reintentos inseguros.
- Pruebas añadidas al binario Base y al proveedor para exclusividad/permisos. Su validación requiere CI y pruebas reales posteriores.
- READMEs auditados íntegros 2/2 (raíz y proveedor); ambos actualizados con el contrato de lock. Se conserva pairing code, los créditos y la independencia de Bot-Matter.
- No se modifican la rama estable, las llaves, el formato cifrado ni las sesiones del operador.


## 2026-11-17 — Formato del registro de plataformas (CI pendiente)

- La primera ejecución de CI detectó tres diferencias rustfmt en `src/lib.rs` y `tests/platforms.rs`, sin llegar a comprobar el comportamiento de los nuevos registros. Se aplicó el formato requerido por el propio diff de CI.
- README auditados íntegros 2/2 (raíz y proveedor Rust); ambos conservados sin cambios. No se modificaron sesiones, secretos, Cargo.lock ni la rama estable.
- Verificar CI Linux, Windows y proveedor para el nuevo commit antes de aceptar el registro.


## 2026-11-17 — Contrato universal de proveedores (CI pendiente)

- Se añadió el registro Rust de conexiones y mensajería por plataforma: WhatsApp, Telegram y Discord utilizan los contratos de Base-Matter existentes, sin reemplazar whatsapp-rust ni iniciar Bot-Matter.
- Las plataformas desconocidas, los controladores ausentes y los registros duplicados fallan cerrados; no hay fallback entre sockets ni entre plataformas. Cuatro pruebas de regresión offline cubren enrutamiento, nombres, aislamiento y ciclo de conexión.
- Revisados íntegros 2 README (raíz y proveedor Rust): 2/2; actualizado el raíz, sin cambios necesarios al proveedor. Créditos y guía de pairing code conservados.
- La CI de Rust y las pruebas con cuenta/telefono reales aún deben confirmar este nuevo checkpoint. Telegram y Discord no están conectados.


## 2026-11-17 — Pairing guide Markdown rendering corrected

- Corrected literal Markdown backtick escaping in the new WhatsApp operator guide and root README so that terminal commands render as fenced code instead of raw escape text.
- Audited 2 tracked READMEs; root updated, Rust provider README reviewed unchanged. No source code, credentials, sessions or runtime behavior changed in this checkpoint.


## 2026-11-17 — Format verification on operator preflight

- Applied rustfmt formatting to controller binary and CLI tests; removed an extra blank line in provider tests. First preflight CI reported only these format issues at the initial step.
- Root and provider READMEs both reviewed (2/2), unchanged; no runtime secrets or live sessions touched.
- Re-run Linux, Windows and provider CI before accepting the preflight checkpoint. No stable WhatsApp claim or Bot-Matter work.


## 2026-11-17 — Reproducible operator pairing-code test setup (pending CI)

- Added Bash-only operator setup helper for two Rust processes; generates two random 256-bit secrets in a private local file with directory mode 0700 and file mode 0600; refuses to overwrite settings or sessions.
- Added docs/WHATSAPP_TRIAL.md with exact source/checkout, build, preflight, init, serve and provider pairing instructions for the real phone operator. No secrets, phone numbers or credentials written to the repository.
- Reviewed both project READMEs (2/2), updated both with the operator guide. Production promotion still requires live pairing and reconnect/revocation tests, plus Vault and broader readiness gates.


## 2026-11-17 — WhatsApp provider preflight, without touching sessions (pending CI)

- Added `--check` to both existing Rust executables, checking all required environment settings, loopback and private paths. Neither command opens a live connection nor SQLite in preflight.
- Added isolated offline process tests for the controller and provider, including rejection of bad configuration and preservation of session files.
- Root and provider READMEs reviewed/updated (2/2); credits preserved. No Node, QR or Bot-Matter.
- A phone pairing and session recovery still require the authorized operator before a stable merge.


## 2026-11-17 — Rust loopback TCP end-to-end regression (pending CI)

- Added a socket-level integration test that opens 127.0.0.1, exchanges real signed JSON frames, and verifies applied/duplicate/applied acknowledgment sequence while keeping WhatsApp offline.
- Reviewed both root and provider README files (2/2), updated root README and WhatsApp IPC docs. Full pairing with a real phone and external security assessment remain release gates.
- Stable matter contains separately verified socket owner-power correction via PR #4; no unverified WhatsApp provider promoted.


## 2026-11-17 — Blocker fixes, linked Rust processes (CI pending)

- Fixed control_ipc.rs E0034 (ambiguous Read/Write by_ref), explicitly using the bounded Read::take on the TCP stream.
- Whatsapp-rust provider 0.7.0 transitive wacore-binary failed E0554 on stable from default SIMD feature; disabled default whatsapp-rust features and explicitly selected stable Tokio/transport/HTTP/SQLite features. CI must confirm transitive SIMD is also disabled.
- Read both root README and provider README, unchanged this commit. Rust only, no Bot-Matter or QR UI; live phone test still outstanding.


## 2026-11-17 — Rust local HMAC control IPC server (pending CI)

- Added WhatsAppControlIpc with strict 127.0.0.1 listener and <=16KiB signed frame admission; response contains only sanitized outcome. Reuses existing Rust WhatsAppControlBridge and persistent replay store.
- Added standalone Base Rust binary whatsapp-matter-control with explicit --init/--serve, owner verification, private SQLite directories, independent 256-bit transport and replay keys, and single-process control-plane lease. No Node process or QR.
- Added three integration tests for HMAC/replay/power, malformed and ping rejection, and public bind refusal. Provider remains a separate Rust executable to resolve incompatible sqlite3 native links.
- README audited/updated (1/2 affected README files; provider README reviewed unchanged), docs/WHATSAPP_IPC.md added and release gates updated. CI pending; no live phone test.


## 2026-11-17 — Separate Rust processes to resolve SQLite native linking conflict (pending CI)

- Provider compilation revealed libsqlite3-sys native links collision: current Base rusqlite 0.32 and whatsapp-rust 0.7 SQLite use incompatible versions. Rather than upgrading Base blindly or replacing Rust with Node, decoupled the whatsapp-rust process from Base's direct Cargo dependency.
- Standalone Rust provider now signs owner-only =/> events with the exact HMAC frame accepted by Base's Rust WhatsAppControlBridge and sends over 127.0.0.1. Five offline tests cover config, JID isolation, protocol vector, timestamp and exclusive SQLite lease.
- Main crate remains independent; no Bot-Matter code or QR handler. Root README and provider README both reviewed/updated (2/2).
- The Base TCP listener and actual phone-linked connection remain to be implemented/verified. Upstream SQLite security review is mandatory before production/stable qualification. CI pending.


## 2026-11-17 — Executable whatsapp-rust provider adapter (pending CI)

- Introduced an independent Rust provider crate at providers/whatsapp-rust, pinned to upstream whatsapp-rust 0.7.0. No Node/Baileys implementation or Bot-Matter runtime.
- Adapted historical PairCodeOptions + SQLite + connected/message callbacks from commit 3c36d97bb to trusted WhatsAppNativeIngress and reserved =/> control.
- Explicit trusted provisioning, owner-only absolute private paths, anti-replay secret, exclusive per-session lock, no QR callback and no permanent ping handler.
- Offline configuration/security/lock tests and dedicated CI compile job added. Upstream itself may generate internal QR during code pairing; none is displayed.
- README audit: 2 README files reviewed and updated (root plus provider). Real phone linking, session-schema upgrade, restart/revocation testing, and upstream compatibility remain unverified until CI and real operator test. Stable matter untouched.


## 2026-11-17 — Rust native WhatsApp owner event guard (CI pending)

- Added WhatsAppNativeIngress with owner private-JID exact matching, from_me and non-group checks, 120-second freshness, bounded input, persistent replay, and canonical =/> routing.
- Added five Rust unit/integration tests for owner power/policy, group/other chat, no ping, replay expiry and invalid identity.
- README reviewed and updated (1/1), new docs/WHATSAPP_NATIVE.md. No Node provider and no live phone pairing is claimed; stable branch unchanged.


## 2026-11-17 — CI verde y README alineado con Rust (documentación)

- La revisión `184a0819865ffa3548cc8eb1e51f7bd5dc521cc9` pasó la CI de
  Linux y Windows (run 44149787360). Los seis nuevos tests de comandos
  globales, Cargo fmt/check/test, documentación y Clippy fueron aprobados.
- Auditoría README: 1 README raíz leído íntegro; 1 actualizado. Se corrigió
  la descripción antigua que podía sugerir Baileys como proveedor seleccionado.
  El proveedor seleccionado es whatsapp-rust histórico, con pairing code.
- Se mantuvo intacta la referencia al experimento Node en su repositorio, pero
  queda congelado; NO sustituye el proveedor Rust ni se inicia Bot-Matter.
- Siguen pendientes las pruebas de vinculación real y la reintegración del
  proveedor histórico con el control global actual. Solo se cambia beta;
  rama estable matter sin cambios en este checkpoint.


## 2026-11-17 — Rust control extension registry (pending CI)

- Prior Rust-only WhatsApp checkpoint passed GitHub Actions 44148678617 on Linux and Windows.
- Added ControlExtensionRegistry: canonical =/> commands and aliases with owner checks, reserved administrative commands, bounded responses and sanitized panic/error handling. No Bot-Matter, Node or untrusted code execution added.
- Added six regression tests including off/unbound socket, cross-owner isolation, prefix/alias separation and policy invariants.
- README reviewed and updated (1/1). Architecture and docs/CONTROL_EXTENSIONS.md document contracts and required author credit.
- CI pending for this checkpoint. Historic whatsapp-rust provider remains the selected implementation, without claiming a new live pairing test.



## 2026-11-17 — Rust-only WhatsApp direction and socket-policy regression (pending CI)

- Owner corrected course: WhatsApp provider remains **whatsapp-rust** with pairing code only; Python/C++/Rust are principal languages. The Node/Baileys experiment is not the selected provider and is frozen. No work on Bot-Matter.
- Historical Rust provider `src/providers/whatsapp/mod.rs` from commit `3c36d97bb` remains retrievable. The current beta does not yet restore its live runtime; do not claim that the pairing test was repeated.
- GitHub Actions run 44147212748 failed two new Rust tests on Linux and Windows because test fixtures assumed `=/>on` automatically enabled the socket command policy. Corrected tests to check deny-by-default after on, explicitly enable the socket policy for the positive case, and keep replay/expiration assertions.
- Renamed an unnecessarily Node-specific HMAC test function to a transport-independent Rust protocol-vector name. Authenticated boundary remains Rust; no Node requirement was introduced in Base.
- README audit: 1 root README reviewed, updated and cross-referenced. AGENTS and bridge docs updated; exact credits retained in modified Rust tests. This checkpoint awaits CI.
- Experimental work remains on `matter-beta`; stable `matter` untouched. Actual WhatsApp linkage/credentials are not available to this CI environment.


## 2026-11-17 — Signed WhatsApp-to-Base global control bridge (pending CI)

- Historical Rust WhatsApp pairing provider source recovered from commit
  3c36d97bb. The live provider was subsequently removed; current Base
  exports pairing contracts, not the historical live connection.
- Added WhatsAppControlBridge verifying dedicated HMAC-SHA256 proof over
  socket/owner/private chat/event/text/timestamp and requiring from_me=true.
  Uses persistent InboundReplayStore and existing ControlDispatcher, while
  refusing Bot-Matter routing and #ping as global commands.
- Added six regression tests: owner-controlled off/on/status, no ping or
  alias bypass, signature/identity changes, replay and clock skew,
  malformed/oversized frames, and a cross-language HMAC test vector.
- README reviewed and updated; protocol documented in
  docs/WHATSAPP_BRIDGE.md. No provider keys, phone numbers, session files or
  credentials committed.
- The bridge has no live socket or full provider transport yet. Neither
  a real WhatsApp pairing nor delivery can be declared verified.
  Run CI before merging; stable matter remains untouched.


## 2026-11-17 — Reserved socket status control (pending CI)

- Built `=/>status [SOCKET_ID]` as a socket-owner-only introspection
  operation independent of Bot-Matter and available when bot is off/unbound.
- Reads policy, source and optional bot binding in a single stored admission
  snapshot. Never treats activated power as proof of network connectivity.
- Added three regressions covering power on/off, unauthorized actors,
  cross-socket targets, bot bindings and rejection of `=/>ping`.
- README audit: 1 root README reviewed and updated; activation, pairing
  contracts and stage status reviewed/updated. No actual WhatsApp
  credentials, session files or QR flow introduced.
- Stage I remains in progress; external pairing/connection and Vault
  integration are not complete. This checkpoint awaits CI validation.
  `matter` stable and Bot-Matter untouched.


## 2026-11-17 — Owner correction: pairing-code-only and global control boundary (pending CI)

- Owner clarifies the 22+ \`Matter\` repos form one ecosystem, but Bot-Matter
  must NOT receive development until Base-Matter is complete.
- Historical WhatsApp connection and ping verification are acknowledged.
  Current beta tree has no executable provider implementation. Do not confuse
  previously tested server work with a current live integrated GitHub build.
- WhatsApp pairing is strictly through a code derived from the operator's
  authorized phone number; QR is excluded. The previous ping is a test-only
  behavior and will not replace Base's global prefix \`=/>\`.
- Added PairingCodeService contract in Base with per-socket owner/host policy,
  phone and transient-code validation and zeroizing/redacted return.
  It has no QR API, no network adapter, no credential persistence, no
  automatic reconnect and no ping handler. Provider-specific implementation
  and authenticated end-to-end tests remain open.
- Added five tests covering access control, provider/code failure, session
  eligibility and global prefix isolation without bot binding.
- Existing legacy alias prefixes remain supported. All changes only in
  matter-beta; matter stable and Bot-Matter are untouched.
- README audit: 1 README reviewed/updated; AGENTS, architecture, session docs
  and release-readiness updated. CI for this checkpoint pending.


## 2026-11-17 — Verified stable promotion, naming and WhatsApp beta

- Promoted only standalone Rust lint/security/test hardening, MIT LICENSE,
  Cargo.lock and Linux/Windows CI from stable candidate through PR #3 into
  \`matter\`, merge SHA \`e509cc4f8c53eafbb1233359bc2ae63e0dde7f48\`.
  The post-merge stable run 44139981479 passed on Ubuntu and Windows.
- \`matter-beta\` retains incomplete integrations and diverged feature work;
  it was not mass-merged into stable. Last named beta CI run 44138531259
  passed; no older local server files were touched.
- Matter-Baileys has a separate matter-beta branch for the future
  WhatsApp-Matter adapter. Node dependency resolution, 8 offline test cases
  and upstream import check passed run 44139262152. This cannot establish a
  live WhatsApp connection or authorize production credentials/groups.
- Repo naming: 22 remotes require GitHub administrative rename; no repository
  rename was executed through this connector. Safe gh CLI rename plan and
  docs/ECOSYSTEM_RENAME.md are committed in beta.
- README audit: one root README inspected and updated. This checkpoint is
  documentation-only, with CI validation to follow.
- Stage I integration/resilience remains active. H audit closure and J–L
  release gates remain open. Do not start Matter-Bot until Base is ready.


## 2026-11-17 — Reversed ecosystem names and WhatsApp provider smoke

- User requested '<Component>-Matter' names across the Matterssmith
  ecosystem. Inventoried 22 prefixed repositories. Added
  docs/ECOSYSTEM_RENAME.md and safe Python gh CLI migration planner;
  default mode only previews, and --apply requires --only or --all.
- GitHub connector lacks repository rename administration, so no actual
  remote repository name has changed. The existing Beta and stable URLs
  remain authoritative until a separately authorized GitHub CLI rename.
- Provider repository Matterssmith/Matter-Baileys now has its own
  matter-beta branch with offline-tested Baileys QR/pairing-code smoke,
  fail-closed allowlisted #ping and seven tests (GitHub run 44138049692
  successful). No live QR pairing or authenticated Base bridge proved.
- Root README audited, reviewed and updated (1/1). AGENTS updated to the
  latest naming convention and strict verified-subset release policy;
  Rust package/crate names retained until compatibility validation.
- Added unit tests for all 22 proposed names. Stage I integration continues;
  Vault authentication, real provider binding, sandbox isolation,
  cross-store transitions and Termux remain unverified.


## 2026-11-17 — Eliminate Windows test-directory collision (pending CI)

- Windows run 44126198795 exposed a flaky duplicate temporary directory
  name in tests/control.rs. Clock-based nanosecond identifiers are not
  reliably unique on Windows under parallel test execution.
- Replaced process/time-derived directory IDs with 128-bit randomized IDs
  in tests/control.rs, tests/activation.rs, tests/platform_flow.rs,
  tests/bot_registry.rs and tests/core.rs. This changes test fixture
  allocation only, not production state or behavior.
- README audit: one root README fully reviewed, unchanged, no new broken
  links or design discrepancy. CI on Linux and Windows must pass again.
- Real Vault integration, signed platform adapters, isolated sandbox,
  crash-recoverable owner coordination and Termux verification remain open.
  Stable matter and Matter-Bot remain unchanged.


## 2026-11-17 — Verified beta checkpoint and actual remaining gates

- Verified GitHub Actions run **44125657053** on exact beta SHA
  `d4dcc7791fdf1641de74e755a2edef81f87531b8`:
  **success** on Ubuntu and Windows. Jobs included locked compilation,
  unit/integration tests, strict Clippy, formatting, docs, README link/credit
  checks, tracked-secret-name audit and whitespace.
- New since earlier stage H: scoped role authorization, owner handover,
  reserved set/unset role controls, durable session metadata, outgoing
  activation/role admission, connection/credential contracts, encrypted outbox
  and lost-ack reconciliation, plugin trust/sandbox interface and trusted
  inbound gateway/replay claims. These are verified local contracts, not
  evidence of live provider or Vault integration.
- `matter` stable remains at
  `c1ad3498381ba9f1f18d4aef3cf702686dcd285c`.
  Matter-Bot remains blocked. No changes to real user-local worktrees,
  ignored sessions, production keys, Vault or stable branch were made.
- README audit for this documentation-only commit: one README found, fully
  reviewed, **0 updated, 1 unchanged**. Its current project status and
  boundaries remain accurate; release-readiness document updated.
- **Critical unverified work:** authenticated private Vault key/remote
  publisher and restart/restore integration; real signed/authenticated
  WhatsApp, Telegram and Discord adapters and test accounts; an independently
  tested OS/WASM sandbox; coordinated atomic or crash-recoverable bot/socket/
  role ownership handover; Termux/Android tests and final security threat
  model/migrations. External secrets, provider test accounts and isolation
  infrastructure are unavailable in this GitHub-only execution context.
- Do not mark H–L or Matter-Base as complete. Next session must inspect actual
  beta head/CI status and continue from these exact missing integrations,
  without repeating passing modules or touching Matter-Bot.


## 2026-11-17 — Authenticated ingress and replay contract (pending CI)

- Previous Windows SQLite-handle fix passed GitHub Actions run 44124628703
  on both Ubuntu and Windows (compile, tests and strict Clippy).
- Added InboundGateway requiring external authenticated provider identity and
  mandatory authorization/consent policy before ControlDispatcher.
- Added bounded SQLite HMAC-keyed replay claims per verified platform/socket
  and provider event ID, without storing message text, raw IDs or replay key.
- Five regressions cover power gating, duplicate handling, denial, provider
  mismatch, durable replay isolation and invalid/oversized input.
- README audit: one root README reviewed and updated; ingress and architecture
  docs added. CI for this phase is pending.
- Production webhook signature verification, platform identity mapping and
  replay-key rotation still require real adapter validation.
  Stable matter/Matter-Bot untouched; H–L remain open.


## 2026-11-17 — Windows SQLite file-handle regression (pending CI)

- Run 44124073091: Linux checks passed, Windows compilation passed, but two
  tests in tests/users.rs failed with Windows OS code 32 because test-owned
  UserRegistry SQLite connections remained open during remove_file.
- Corrected both tests by explicitly dropping registries before deleting
  temporary database files, preserving production behavior and assertions.
- README audit: 1 root README found/reviewed/updated, with cross-platform
  verification limitations retained. Re-run both CI jobs before accepting.
- Real Vault/provider/sandbox/Termux tests remain open; matter and Matter-Bot
  unchanged.


## 2026-11-17 — Windows Rust CI lane (pending CI)

- Prior release and tracked-file hygiene passed GitHub Actions run 44123810438.
- Expanded matter-beta workflow with a Windows stable Rust lane running
  locked all-target compilation, unit/integration tests and strict Clippy.
- README audit: one root README reviewed and updated; release-readiness
  notes updated to distinguish Windows CI from missing Termux/platform tests.
- A passing result on Windows is pending. No production readiness claim,
  stable branch merge, Matter-Bot work or local session modifications.


## 2026-11-17 — Release hygiene and evidence gates (pending CI)

- Previous encrypted lost-ack reconciliation passed GitHub Actions run
  44123443517.
- Added MIT LICENSE matching Cargo.toml's declared license, without altering
  upstream provider licensing or incorporating external provider code.
- Added scripts/verify_repository.py to check all tracked README credits,
  relative links, tracked LICENSE presence and sensitive tracked filenames.
- CI now runs the tracked-file/document audit alongside Rust checks; new
  docs/RELEASE_READINESS.md lists independently verifiable release blockers
  instead of treating interfaces/mocks as completed integrations.
- README audit: one root README reviewed and updated; its relative links
  are intended to be checked in CI. Full release readiness is NOT declared.
- Test results for this commit await GitHub Actions. Stable matter and
  Matter-Bot unchanged.



## 2026-11-17 — Remote encrypted acknowledgment reconciliation (pending CI)

- Prior connection lifecycle checkpoint passed GitHub Actions run 44122884865.
- Added EncryptedEnvelopeReader and fail-closed outbox reconcile operation,
  verifying full envelope identity, metadata and ciphertext before updating
  the local revision/sync flag in the same transaction as immutable audit.
- Three regressions cover lost network acknowledgments, remote divergence,
  missing entries and ciphertext tampering. No retry or remote overwrite on
  conflict; no plaintext, token or master key stored in GitHub.
- README audit: one root README reviewed and updated; Vault docs updated.
  Real authenticated GitHub sink/source and full remote import remain open.
- This checkpoint awaits CI. No work on stable matter or Matter-Bot.


## 2026-11-17 — Connection lifecycle and credentials contract (pending CI)

- Previous scope-owner handover is verified by GitHub Actions run 44122169726.
- Added ConnectionService orchestrating connection state via SessionRegistry,
  mandatory authenticated ConnectionAdmission, zeroizing credential source,
  and external ConnectionDriver for connect/disconnect/liveness.
- Five deterministic regression tests cover owner checks, transient credentials,
  connection failures, health faulting, recovery and versioned transitions.
- No provider credentials, login sessions, master keys or network adapters
  are added to Matter-Base. Driver implementations require real external
  generation fencing and credential storage.
- README audit: one root README reviewed and updated; session/architecture
  contracts updated. CI for this checkpoint is pending.
- H–L remain in progress; independent provider integration and real isolated
  plugin execution are unverified. Stable matter untouched, Matter-Bot blocked.



## 2026-11-17 — Role scope owner handover (pending CI)

- Previous plugin contract checkpoint passed GitHub Actions run 44102970673.
- Added explicit RoleStore::transfer_scope_owner; caller must be the previous
  role scope owner and target must already be current owner in BotRegistry.
  Scope update and audit share one SQLite transaction, with fail-closed
  target checks and idempotent same-owner operation.
- Two regressions cover incorrect targets, revoked old administration, new
  owner-only history and audit-trigger rollback/recovery.
- README audit: 1 total, 1 reviewed and updated; corrected stale roadmap
  and documented cross-store non-atomic owner handover.
- This phase's CI is pending. Lost-owner recovery still requires a distinct
  authenticated authority; no production-atomic bot/role transfer is claimed.
  H–L remain open. Matter-Bot and stable matter remain unchanged.



## 2026-11-17 — Plugin trust and sandbox contracts (pending CI)

- Previous encrypted control-outbox checkpoint passed GitHub Actions run
  44101946067.
- Added plugin manifests with bounded parsing, traversal rejection,
  deterministic directory loading and duplicate-name protection.
- Added mandatory PluginTrust hooks for publisher and invocation checks,
  plus PluginSandbox contract. Default DenyPluginSandbox never executes
  untrusted code. Four deterministic tests cover manifest/path checks,
  policy rejection, canonical commands and invalid batches.
- README audit: one root README reviewed and updated; new docs/plugins.md
  and architecture map the actual boundary. No platform executable imported.
- CI for this checkpoint is pending. No production process isolation,
  external signature verification or runtime sandbox has been verified.
  H–L remain open; stable matter and Matter-Bot unchanged.


## 2026-11-17 — Encrypted control outbox (pending CI)

- Previous reserved role-control checkpoint passed GitHub Actions run
  44100755985.
- Added EncryptedControlOutbox with authenticated XChaCha20-Poly1305 envelopes,
  key-bound HMAC change fingerprints, SQLite pending ciphertext, immutable
  audit, compare-and-swap sink revision and restart-safe pending status.
- Added RoleStore::export_snapshot with one SQL snapshot, stable ordering
  and owner authorization; transient plaintext is zeroized after encryption.
- Regression tests cover encrypted-only persistence, retry on sink failure,
  role scope authorization and key-rotation resubmission.
- README audit: one README fully reviewed and updated; Vault docs clearly
  distinguish the contract from absent real GitHub and authenticated key
  service adapters. No key material, secret tokens or platform sessions added.
- CI validation for this checkpoint is pending. Cross-process conflicts,
  acknowledgement loss and remote recovery still need explicit reconciliation.
  H–L are not complete, stable matter unchanged, Matter-Bot blocked.


## 2026-11-17 — Reserved role control (pending CI)

- Previous outbound message-control integration passed GitHub Actions run
  44099969055.
- Added optional set/unset role operations on all reserved control prefixes,
  requiring current socket, bot and authorization-scope ownership.
- Added tests for powered-off operation, unauthorized actors, unknown roles,
  idempotence, revocation, missing role configuration and bot transfer.
- README audit: one root README reviewed and updated, plus activation docs.
  Cross-store owner-transfer serialization remains a necessary host contract.
- This checkpoint awaits CI verification. H–L are not fully complete.
  Stable matter and Matter-Bot remain untouched.



## 2026-11-17 — Outbound control-plane integration (pending CI)

- Previous session contracts passed GitHub Actions run 44099305219.
- Added ControlPlaneMessageAdmission bridging existing socket activation,
  group precedence, bot binding, current bot owner, matching session owner,
  persistent role rules and mandatory app-supplied legal/provider policy.
  A disconnected/off/unbound/revoked socket fails before transport.
- OutboundMessage now declares optional group_id explicitly; this avoids
  treating private destinations as group-policy resources. This is a pre-1.0
  API addition without a compatibility guarantee.
- Three integration tests exercise powered-off, group denial, role revocation,
  external policy, session owner mismatch and binding recovery.
- README audit: one README reviewed and updated; documented cross-store
  admission limitations. No secret data or platform sessions are persisted.
- New integration CI is pending; external providers and exactly-once semantics
  remain unavailable. H–L remain open; no Matter-Bot work.


## 2026-11-17 — Session state and transport contracts (pending CI)

- Previous authorization code passed GitHub Actions run 44098196776:
  formatting, all-target checks/tests, documentation tests and strict Clippy.
- Added SessionRegistry with bounded metadata, owner-only lifecycle management,
  optimistic revision checks, immutable audit and transactional rollback.
  Added MessageService, mandatory MessageAdmission policy and TransportDriver
  abstraction; it holds no provider secrets or private session data.
- Added six regression tests (restart, independent SQLite connections,
  audit rollback, admission/connection, transport errors and payload bounds).
  They exercise local contracts only, not external platform integration.
- README audit: one root README reviewed and updated. Architecture and
  docs/sessions.md describe the new boundary and actual missing providers.
- CI results for this new session checkpoint are pending.
  Keep H/I/J/K/L open; do not start Matter-Bot or modify the stable branch.


## 2026-11-17 — Scoped persistent role authorization (pending CI validation)

- Added opt-in SQLite role scopes, immutable parent inheritance, owner-only
  role/grant/rule administration, explicit deny precedence, default deny, a
  bounded immutable audit and atomic policy-plus-audit transactions.
- Added RoleCommandAuthorizer for the canonical CommandRouter permission
  check; authenticated ingress and independent socket gating remain required.
- Six integration regressions cover inheritance/deny, scope isolation and owner
  authorization, persistence/revoke, audit rollback, alias dispatch and corrupt
  role-chain refusal. They are not claims of live provider integration.
- README audit before this commit: one README fully reviewed and updated.
  New docs/authorization.md documents operational and trust limitations.
- CI validation for this new code is pending. All earlier code is preserved,
  matter remains unchanged and Matter-Bot is not started.
- Follow-up: inspect GitHub Actions, fix any build/test/lint faults, then
  extend integrations and resilience under stages H/I/J/K/L.


## 2026-11-17 — Strict CI lint remediation

- Retrieved GitHub Actions run `44095876210` and job logs: formatting,
  `cargo check --all-targets --locked`, all default tests and doc tests
  succeeded. Strict Clippy failed with exactly three diagnostics.
- Corrected `src/legal/mod.rs` by naming the existing SQLite tuple type,
  `src/security/encryption.rs` by using `div_ceil(3)`, and
  `src/users/mod.rs` by using inclusive range membership. These are
  targeted, behavior-preserving lint corrections, not a completed phase.
- Reviewed the sole root README and updated it to explain this CI gate.
  Source changes were published on `matter-beta`; no local worktree,
  stable branch, Matter-Vault or Matter-Bot files were changed.
- A successful CI run for the final commit is **not yet confirmed**.
  The previous run's passing tests do not verify these new commits.
  Check the final run and address any new diagnostics before proceeding
  with persistent resource authorization and H–L completion.
- README audit: 1 total, 1 reviewed, 1 updated, 0 unchanged.
  Full repository-wide release/security verification remains pending.


## 2026-11-17 — Remote CI groundwork (ChatGPT checkpoint)

- Continued from the actual remote `matter-beta` commit
  `665ae8c1aa2e1858993ac8e2ba29de3ced0bce7f`, without modifying
  `matter`, Matter-Bot, local worktrees, ignored sessions or Vault.
- Added `.github/workflows/rust-ci.yml` on `matter-beta`. It runs
  formatting, `cargo check --all-targets --locked`,
  `cargo test --all-targets --locked`, `cargo test --doc --locked`,
  `cargo clippy --all-targets --locked -- -D warnings`, and a parent-commit
  whitespace diff on push/PR or manual dispatch. Checkout uses depth two.
- The GitHub connector confirmed the file commits were published. **No
  successful CI run has yet been verified.** Combined commit statuses returned
  no status entries. GitHub API tools in this session cannot retrieve the
  Actions run list; do not interpret that as a passing build. These changes
  are CI groundwork, not a completed H/I/J/K/L phase.
- README audit: 1 README found and reviewed, 1 updated, 0 unchanged.
  Documented the new workflow and the unverified external Vault test.
  No source or crypto contracts were modified. Tests and Clippy were not run
  in this environment, so no test result is claimed.
- Next: confirm a successful GitHub Actions run, correct any workflow/build
  issues, then implement persistent general role/resource authorization with
  scoped owner administration, explicit deny precedence and command integration.
  Preserve prior work and audit the complete README before each commit.
  H–L remain open; Matter-Bot remains blocked.


This is a development ledger. GitHub remains the source of truth. Its
synchronization entry records the published base of each working checkpoint;
verify the branch log for the current published commit.

## 2026-11-17 — Socket handover and binding recovery

- Base: `dc9ae57d0dd6bfa982312c58a70c98f0ac9313fb`, pushed and fetched on
  `matter-beta` with identical origin SHA before this checkpoint.
- Added socket-owner-only transfer and binding release. Both atomically power
  off, remove local overrides/binding, and persist their audit. Transfer also
  changes ownership; the previous owner loses administrative/audit access.
  No global administrator bypass, bot-ownership takeover, platform credential
  transfer or implicit reactivation is introduced. Same-owner transfer and
  release of an absent binding are authorized no-ops.
- Unified dispatcher admission into one SQLite snapshot containing power,
  policy, socket owner and bot binding. This prevents old activation state from
  being combined with a new binding during concurrent handover. Previously
  admitted handlers may finish; subsequent admissions see the reset state.
- Schema 4 transactionally adds optional audit target IDs while preserving
  historical rows as NULL targets. New binding/release events name the bot;
  transfer names the recipient. Public audit records now expose actor/target.
  Strict decoding rejects corrupt persisted audit boolean values.
- Added five tests: complete handover and revoked-bot binding recovery through
  the dispatcher, independent-connection transfer race with one winner,
  complete rollback on audit failure, legacy v3 audit migration, and an
  in-flight handler overlapping transfer without admitting the new binding
  while off. Other sockets and bot ownership remain independent.
- Verification passed: `cargo fmt --all`, `cargo check --all-targets`, focused
  activation/control tests (19 passed), full `cargo test` (239 passed, 0 failed,
  1 opt-in Vault test ignored; 0 doc tests), and `git diff --check`.
- README audit: 1 total, 1 reviewed in full, 1 updated, 0 unchanged. Updated
  transfer/recovery, explicit reactivation and snapshot claims; documented
  schema/API compatibility in activation contracts. Examples match exercised
  APIs, relative links and credits are correct. Full diff/file inventory and
  secret-pattern scan reviewed; no new/deleted files or secret matches.
  Lost-owner recovery and actual platform/network operations remain pending.
- Next independent prerequisite: persistent bounded role/resource authorization
  with owner-scoped administration, role inheritance, deny precedence and command
  integration. Continue H/I/J implementation and tests; external Vault verification
  remains unavailable, but does not stop this local work. H–L remain open and
  Matter-Bot is not started.

## 2026-11-17 — Persistent global activation policy

- Resumed from verified `7ef49de36cdeae03446c8230bffe7a6aee76e18f` on
  `matter-beta` after fetch/switch/reset with a clean tree. The existing stash,
  ignored build output and stable branch were preserved.
- Added trusted one-time global-policy administrator provisioning, owner-only
  policy mutation and bounded owner-only audit pages. Provisioning and policy
  changes commit with their audit rows; audit failure rolls everything back.
- Schema `socket_activation=3` upgrades versions 1/2 transactionally while
  preserving existing socket state and bindings. No administrator is inferred.
- `resolve_persisted` and `ControlDispatcher::new_with_persistent_policy` read
  power plus Group > Socket > Global > deny in one SQLite snapshot per normal
  admission. Independent-connection changes become visible without rebuilding
  the dispatcher. Existing static-global APIs retain their previous behavior.
  Global administration grants no authority over another owner's socket power
  or overrides. Missing global policy denies fallback; corrupt values fail closed.
- Added five regressions for restart recovery, precedence and isolation,
  authorization/pagination, provisioning and mutation rollback, corrupt global
  values, concurrent writes/audit order, migration failure/retry, and actual
  dispatcher behavior with live policy changes. No external service is mocked
  or represented as verified by these tests.
- Checks passed: `cargo fmt --all`, `cargo check --all-targets`, focused
  activation/control tests (14 passed), full `cargo test` (234 passed, 0 failed,
  1 opt-in Vault test ignored; 0 doc tests), and `git diff --check`.
- README audit: 1 found, 1 fully reviewed, 1 updated, 0 unchanged. Added actual
  persisted-global API use and corrected roadmap scope; examples match compiled
  tests, relative links resolve and credits remain intact. Full diff and file
  inventory reviewed; no new/deleted files or secret-pattern matches. Remaining
  documentation gaps correspond to unimplemented systems, not this policy flow.
- Next: socket ownership transfer and owner-authorized binding recovery with
  atomic power-off, isolation and audit, then general resource authorization.
  Continue independent local prerequisites while the external Vault key/binding
  implementation and authenticated access contract remain unavailable. H–L
  are still open; no stable promotion or Matter-Bot work occurred.

## 2026-11-17 — Durable command audit integration

- Continued independent Base work after publishing and fetching control fix
  `402f1bec2432e6646e89242f57ee944d4b5eed0b`; local HEAD and origin matched.
  Worked only on `matter-beta`, preserving the legacy stash and ignored files.
- Added `CommandAuditStore`, owner-scoped bounded history, immutable SQLite
  attempts/outcomes and transactional schema version `command_audit=1`.
  `CommandRouter::new_with_audit` persists recognized canonical commands before
  callbacks and records authorization, middleware and handler results, including
  unwinding panics. Default `new` preserves its existing unaudited behavior.
- Audit start failure prevents callbacks; outcome-write failure returns a
  sanitized error and leaves a durable Pending attempt. Handler effects cannot
  be undone by this store. No automatic retry or exactly-once claim is made.
  Arguments, message bodies, responses and callback diagnostics are omitted.
- Added six integration tests covering callback failures/panics, SQLite start
  and completion failures, recovery, foreign/current-owner read authorization,
  ownership revocation, bounded pagination, canonical alias logging, payload
  omission, concurrent independent connections, immutable records (including
  replacement inserts), migration rollback and future-version rejection.
  One flow integrates actual persistent bot/activation/audit stores with the
  router and ControlDispatcher: off blocks normal commands before audit;
  power changes retain their separate atomic activation audit.
- Files created: `src/command_audit.rs`, `tests/command_audit.rs`,
  `docs/command-audit.md`. Modified router/public exports, README, architecture
  and this ledger. No dependencies, cryptographic formats or external
  repositories changed; no files deleted.
- Verification: `cargo fmt --all`, `cargo check --all-targets`, focused audit
  tests (6 passed), full `cargo test` (229 passed, 0 failed, 1 opt-in Vault test
  ignored; 0 doc tests), and `git diff --check` passed. The explicit Vault
  metadata test passed in the preceding checkpoint; no live-source verification
  has become available.
- README audit before commit: 1 total, 1 fully reviewed, 1 updated, 0 unchanged.
  Corrected previously absent audit claims, described the opt-in constructor,
  failure/recovery semantics and bounded owner-scoped reads, and linked the
  new contract. Existing examples and new API use match the compiled tests;
  internal links resolve, project identity/credits are preserved, full diff
  and all new files were reviewed, and secret-pattern scans found no matches.
  The scan is not an independent security audit. Broader ingress auditing,
  archival/retention operations and release documentation remain pending.
- H/I progress is verified at this scope; no phase is declared complete.
  The Vault implementation/access-contract question remains pending. A real
  source is needed to implement and verify authenticated key/binding reads,
  failure handling, rotation and recovery. Its metadata repository alone
  cannot fulfill that dependency.
- Exact next local work: persist authorized global activation policy and its
  audit without granting global owners power over sockets; add socket ownership
  recovery/transfer and general resource authorization. Then complete
  session/credential and connection/messaging contracts and implementations,
  effective plugin isolation, cross-module resilience, security review and
  public compatibility/release criteria. Keep Matter-Bot blocked and do not
  substitute isolated tests for an external integration. Read this latest
  entry and synchronize safely on resume; historical entries below are retained.

## 2026-11-17 — Resumed control audit and Vault prerequisite

- Synchronized `matter-beta` with `origin/matter-beta` at
  `cec9df539f5031af9381564545c0abce7880ccc8`. The working tree was clean;
  no new stash was necessary. The existing legacy bot-registry stash and
  ignored `target/` were preserved. Root `AGENTS.md` applies; no nested
  instructions were found. No local stable-branch changes were made.
- Recovered previously unrecorded progress from that backup commit: durable
  bot/socket bindings, ControlDispatcher, schema-v1 upgrade coverage, ecosystem
  naming instructions and the opt-in real Vault repository metadata test.
  These were inspected and retained, not reimplemented.
- Confirmed and corrected an admission-order defect: bot binding/ownership
  checks ran before reserved power commands, preventing a socket owner from
  powering off after bot ownership transfer or before binding. Controls now
  check the target socket owner directly through the transactional activation
  store. Normal messages check power/policy, binding and current bot authority
  before reaching the router. A new bot owner/global owner gains no authority
  over an independently owned socket. No network connection is needed to
  change operational power. Admission remains snapshot-based.
- Added regressions for all three reserved prefixes, unbound control, revoked
  bot authority, foreign/global owner denial and absence of handler effects.
  Added independent-connection concurrent power/audit recovery coverage and
  binding-audit failure rollback, idempotent binding and conflicting rebind
  rejection. Existing Group > Socket > Global > deny tests remain passing.
- Verified: `cargo fmt --all`, `cargo check --all-targets`, focused control and
  activation tests (9 passed), full `cargo test` (223 passed, 0 failed,
  1 opt-in Vault test ignored; 0 doc tests), and `git diff --check`.
  Explicit `MATTER_VAULT_CHECKOUT=/home/Sakura/Matter-Vault cargo test --test
  vault_repository -- --ignored` also passed (1 test). Formatting normalized
  the preexisting Vault test; no encryption format changed.
- README audit: 1 total, 1 reviewed in full, 1 updated, 0 unchanged.
  Findings/fixes: clarified power independence, linked real Vault metadata
  evidence, and corrected roadmap wording that implied all socket policy was
  still absent. README examples were reviewed against the actual APIs;
  repository-relative Markdown links resolve. Identity/credits are preserved.
  Full diff/new-file and added-line secret-pattern review found no accidental
  secrets; this is not a comprehensive secret audit. No new/deleted files.
  Release documentation for absent systems remains pending.
- Real external blocker: read-only remote inspection confirms Matter-Vault
  `matter` is still `072c60a288d0773020f3460a639e77f84bd76c4c`; no
  `matter-beta` exists there. Its clean checkout has only README/ignore rules,
  manifest, version metadata and envelope schema. There is no executable
  key/binding provider or authenticated service contract to exercise. The
  metadata test cannot substitute for real key retrieval, binding, rotation,
  recovery or authenticated administrative integration. Requested the location
  of that external implementation/contract without requesting secret values.
- H and I have advanced but remain incomplete; J–L are not complete. Missing
  local subsystems remain implementation work, not external blockers: durable
  command audit, general authorization, persistent global policy, socket
  recovery/transfer, session/credential interfaces, connections/messaging,
  plugin isolation and public contract/release readiness work. Matter-Bot and
  platform implementation remain unopened.
- Exact next work: obtain and inspect the real external key/binding source and
  authenticated access contract, implement the Base-side integration against
  it while preserving Vault independence/envelope compatibility, and verify
  failure/recovery with a real source. Continue local audit and the missing
  prerequisites above; do not mark H–L done from metadata or isolated tests.
  At resume, use the repository's safe synchronization procedure and inspect
  this checkpoint's actual published SHA before editing.

## Historical recovery snapshot

- Repository: `/home/Sakura/Matter-Base`
- Working branch: `matter-beta`
- Previous synchronized base commit: `ea36faf68944b0d9be1a8563ebb0bfbc8782bfb6`
- Latest confirmed published checkpoint: `916fdbbf61c7b6c1668f7d58f0c43df3eb323d09`
- Remote: `origin` (`https://github.com/Matterssmith/Matter-Base.git`)
- Synchronization: push and subsequent fetch confirmed local HEAD and
  origin/matter-beta at `916fdbbf61c7b6c1668f7d58f0c43df3eb323d09` for the
  persistent socket activation checkpoint. This ledger-only follow-up does not
  change the tested implementation.
- Initial tracked/untracked state: clean. `target/` was present as ignored
  build output and was preserved.
- Applicable repository `AGENTS.md`: none found in the repository or its
  ancestor directories.

## Current stage

**H — integral audit, in progress.** Initial repository-wide source and test
inventory has been completed. A Vault identity recheck flaw in authenticated
key loading was corrected and has a regression test. Legal writes were also
made transactional and tested over independent connections. Confirmed gaps
are recorded below. Timestamp overflow in provider assertions was also
rejected safely. Legal access evaluation now uses one SQL statement for a
consistent database snapshot. Event and translator inputs are converted
before acquiring write locks so caller-supplied conversion code cannot poison
those locks by panicking. The H–L stages are not complete, and Matter-Base is
not ready for Matter-Bot.

## Capability assessment from the current working tree

| Capability | Status | Evidence / limitation |
| --- | --- | --- |
| Runtime and lifecycle | PARTIAL | `MatterRuntime` owns a shared `ModuleManager` and propagates initialize/start/shutdown errors; no built-in modules are registered and services have no lifecycle. |
| Bot identity, family/release/instance IDs, registration | PARTIAL | `src/bots` generates IDs, persists validated identities, and allocates unique names; owner-only transfer/removal, owner/co-owner-scoped identity reads and lists, co-owner operations bounded to 256, legacy recovery authorization, and audit history work. General role/resource authorization remains absent. |
| Shared application users and bot data isolation | PARTIAL | `src/users` provides in-memory/SQLite global IDs, explicit unique external links, and bot/user/key-scoped text data with authorization hooks. A consuming application's `UserAuthorizer` must authenticate principals and verify bot existence/ownership; user storage does not consult the bot registry or implement general roles. |
| Sessions and platform credential contracts | ABSENT | No session/credential subsystem; platform credentials must remain outside this crate. |
| Persistence and recovery | PARTIAL | SQLite persists bot identities, shared users/identity links/bot-scoped data, and legal documents/decisions. No config/network-connection/service persistence or ecosystem-wide migration coordinator; each persistent component has its own versioned schema. |
| Events and messaging | PARTIAL | Named synchronous in-memory event callbacks exist; no durable events or messaging contract. |
| Router and command registry | PARTIAL | `CommandRouter` supports bounded prefixes/names/aliases/arguments, mandatory injected authorization, middleware, and synchronous handlers; ControlDispatcher provides socket gating and reserved control-prefix interception; an injected CommandAuditStore persists recognized attempts/outcomes; default routing remains unaudited and platform-specific parsing remains pending. |
| Middleware, roles, permissions, authorization | PARTIAL | Bot owner/co-owner checks and injectable user/legal action/resource authorization contracts exist; no general role inheritance or socket/resource policy engine. |
| Control Plane, sockets, activation | PARTIAL | SQLite socket activation store provides owner checks, power, group/socket overrides, recovery and atomic audit. Durable bot binding and control-prefix interception/router gating exist. Global fallback persistence and single-snapshot dispatcher integration are implemented; owner-authorized transfer/binding recovery are implemented; network lifecycle and lost-owner recovery remain pending. |
| Services and connections | PARTIAL | Typed in-memory service registry only; no connection manager or lifecycle. |
| Plugins, extensions, sandbox | ABSENT | No manifest validation, isolation, or resource limits. |
| Application audit log | PARTIAL | Legal decisions, bot/socket administrative changes and optional recognized-command attempts/outcomes persist. A unified audit service and broader ingress coverage remain pending. |
| Encryption/key/provider contracts | PARTIAL | Primitives and injection contracts have tests; no production key service integration or independent security audit. |
| Provider assertion replay protection | PARTIAL | Default is in memory; an injectable SQLite store persists IDs and rejects duplicates across restarts/connections, but no distributed store exists. |
| Provider input resource limits | PARTIAL | Assertions are capped at 16 KiB and provider key loads at 4,096 IDs; no general service/plugin resource limits exist. |
| Matter-Vault | BLOCKED | Interfaces exist, but there is no live service integration or external verification. |
| Public API stability / multi-language contracts | PARTIAL | Rust public exports exist; no compatibility policy, versioned external protocol, or SDK contracts. |
| Documentation | PARTIAL | Core README and architecture claims were stale and have been corrected locally; remaining release/documentation audit is pending. |

## Work completed in this local session

- Confirmed the remote branch and commit before building or editing.
- Ran `cargo fmt --all`; it applied formatting changes throughout existing
  Rust source and tests. Behavior changes are listed separately.
- Corrected the root README and architecture, service, i18n, and logger docs
  to describe actual Rust APIs and mark absent capabilities as pending.
- Corrected `INFO.md` to use the canonical repository and describe current
  scope.
- Fixed a Vault identity time-of-check/time-of-use gap: authenticated key
  loading now validates the provider's Vault ID again at staging time, before
  any staged key set replaces the existing `KeyManager` state.
- Added an integration-style SQLite recovery test that closes and reopens the
  runtime and verifies bot identity, user, consent history, and access survive.
- Made legal document registration and consent decision validation/writes
  transactional across independent SQLite connections.
- Changed legal access evaluation to one SQL statement so user and consent
  requirements are read from one SQLite snapshot.
- Added a concurrency test proving two independent connections cannot both
  withdraw the same consent grant.
- Added a regression test for a provider that changes Vault IDs between the
  binding proof and key loading; the change is rejected and preexisting keys
  remain intact.
- Replaced overflow-prone arithmetic on assertion timestamps with bounded
  timestamp-window checks and checked subtraction. Added regression cases for
  extreme signed timestamp values.
- Moved caller-provided `Into<String>` conversions outside EventBus and
  Translator write-lock scopes. Added a regression test proving a panicking
  conversion does not poison those internal locks.
- Confirmed EventBus invokes callbacks after releasing its registry lock and
  added a reentrant-subscription test. Documented that callback panics
  propagate and stop the current publish call; callback error isolation is
  not implemented.
- Added fail-closed limits of 16 KiB for authenticated assertion ciphertexts
  and 4,096 key IDs per provider load. Oversized inputs are rejected before
  decryption or key material loading; regression tests pass.
- Added an atomic `ProviderAssertionReplayStore` contract, bounded in-memory
  implementation, and SQLite implementation. `ProviderAuthenticator` can
  receive a shared store; SQLite replay rejection survives verifier and
  process restarts. Added persistence, expiry, capacity, and cross-connection
  tests.
- Added this continuity record.
- Preserved the exact project credit in the root README.
- Extended `tests/platform_flow.rs` to exercise mandatory legal access denial
  before consent, grant after acceptance, renewed denial after withdrawal, and
  recovery of both bot identity and legal history after closing and reopening
  the SQLite stores. The test uses a unique temporary directory and removes
  only that test-owned directory.
- Fixed unauthorized access to bot ownership audit history. Reads now require
  the current owner and check ownership inside the same store lock/read
  transaction as retrieving records. Co-owners and unrelated actors are
  denied. Records remain stored after bot deletion but are inaccessible through
  the current registry API after the identity is removed.
- Current legal-store schema work modifies `src/legal/mod.rs` and adds three
  isolated migration tests in `tests/legal.rs`; `README.md` and
  `docs/architecture.md` now describe the transactional versioned initialization.
- Legal schema hardening is now implemented locally in `src/legal/mod.rs`:
  SQLite foreign keys are enabled before the migration transaction, and schema
  objects plus the `legal_store` component version are committed atomically.
  Existing unversioned records survive upgrade; failed DDL rolls back; negative
  or unsupported future versions are rejected.
- Added public `LegalAction`, `LegalAuthorizationRequest`, `LegalAuthorizer`,
  and `LegalPrincipal` contracts. Legal user registration/lookups, document
  registration/activation, consent writes/history, required-document queries,
  and access checks now require the supplied principal policy. Runtime/context
  access checks require it too. Authorization errors are sanitized. The crate
  delegates real principal authentication and policy to consuming applications;
  no shared user directory or production authorization provider is implemented.
- Added `tests/common/mod.rs` with a test-only policy and regression coverage
  rejecting cross-user consent forgery, unauthorized history reads, and
  unauthorized document activation. This fixture verifies contract use, not a
  real external identity integration.
- Updated README and architecture docs to define the delegated legal identity
  boundary and the actual implemented/pending scope.
- Files changed for the legal authorization contract: `src/legal/mod.rs`,
  `src/core/context.rs`, `src/core/runtime.rs`, `src/lib.rs`,
  `tests/common/mod.rs` (created), `tests/core.rs`, `tests/legal.rs`, and
  `tests/platform_flow.rs`; docs: `README.md`, `docs/architecture.md`, and
  this ledger. No files were deleted.
- Wired `ModuleManager` into `MatterRuntime` through shared synchronized state.
  Consumers register modules before initialization and invoke runtime
  initialize/start/shutdown. Errors propagate; lifecycle callback panics are
  converted to errors and failed initialization attempts cleanup. The
  cross-module flow exercises success, and a second integration test verifies
  start-error propagation plus cleanup.
- Files changed for runtime lifecycle integration: `src/core/runtime.rs`,
  `src/modules/mod.rs`, `tests/platform_flow.rs`, `README.md`,
  `docs/architecture.md`, and this ledger. No files were deleted.
- Modified public API files `src/legal/mod.rs`, `src/core/context.rs`,
  `src/core/runtime.rs`, and `src/lib.rs` to require a principal policy for
  sensitive legal operations. Updated `tests/core.rs`, `tests/legal.rs`, and
  `tests/platform_flow.rs`; added the shared test policy at
  `tests/common/mod.rs`.
- Added a migration-failure regression for the SQLite bot registry. An injected
  conflicting schema object forces migration to fail; the test verifies the
  schema version and identity columns remain at version 1, legacy rows remain
  present, and a clean retry completes to version 3.
- Added component-scoped schema version 1 for the legal SQLite store. Schema
  creation, indexes, immutability triggers, and the version row are now one
  immediate transaction. Existing unversioned legal tables/data are preserved;
  failed DDL rolls back, negative/future versions fail closed, and retry after
  resolving a test-injected schema conflict succeeds.

### Files created, modified, and deleted

- Created: `Cargo.lock` (generated by Cargo during dependency resolution),
  `docs/DEVELOPMENT_STATUS.md`.
- Modified for documentation: `INFO.md`, `README.md`, `docs/architecture.md`,
  `docs/i18n.md`, `docs/logger.md`, `docs/services.md`.
- Modified for behavior and tests: `src/security/key_manager.rs`,
  `src/security/authenticated_provider.rs`, `tests/core.rs`, and
  `tests/authenticated_key_provider.rs`; `src/legal/mod.rs` also uses
  immediate transactions for registration and decision writes; provider
  assertion timestamp validation changed in
  `src/security/provider_authenticator.rs` and
  `tests/provider_authenticator.rs`; provider list limits changed in
  `src/security/key_manager.rs` and `tests/key_provider.rs`; legal access checks were consolidated in
  one SQL statement in `src/legal/mod.rs`; caller conversion handling changed
  in `src/events/mod.rs` and `src/i18n/mod.rs`; callback behavior is documented
  in `README.md` and `docs/architecture.md`.
- Added for persistent replay protection: `src/security/provider_replay_store.rs`
  and `tests/provider_replay_store.rs`; exported the contract and both stores
  from `src/security/mod.rs`.
- Modified by `cargo fmt --all` (formatting only):
  `src/core/config.rs`, `src/core/context.rs`, `src/core/logger.rs`,
  `src/core/mod.rs`, `src/core/runtime.rs`, `src/lib.rs`, `src/modules/mod.rs`,
  `src/security/encryption.rs`, `src/security/key_provider.rs`,
  `src/security/mod.rs`, `src/security/provider_binding_store.rs`,
  `src/security/provider_identity.rs`,
  `src/services/mod.rs`, and test Rust files other than
  `tests/core.rs`, `tests/authenticated_key_provider.rs`, and
  `tests/provider_authenticator.rs`, `src/security/provider_replay_store.rs`,
  and `tests/provider_replay_store.rs`.
- Deleted: none.

## Verification results

After formatting, the following commands completed successfully:

- `cargo fmt --all`
- `cargo check --all-targets`
- Initial baseline `cargo test` — 143 tests passed, 0 failed; doc tests: 0.
- Targeted persistence recovery test — passed.
- Targeted independent-connection withdrawal concurrency test — passed.
- Targeted provider Vault-switch regression test — passed.
- Provider assertion extreme-timestamp regression test — passed.
- EventBus/Translator panicking-conversion regression test — passed.
- EventBus callback reentrant-subscription test — passed.
- Oversized assertion and provider key-list tests — passed.
- Provider replay-store integration tests — 4 passed, 0 failed.
- Full `cargo test` after resource-limit changes — **151 passed, 0 failed**;
  doc tests: 0 passed, 0 failed.
- Full `cargo test` with SQLite replay-store implementation — **155 passed, 0 failed**;
  doc tests: 0 passed, 0 failed. `cargo check --all-targets` passed;
  `git diff --check` passed.
- Full `cargo test` after lock-poison recovery — **158 passed, 0 failed**;
  doc tests: 0 passed, 0 failed. `cargo check --all-targets` passed.
- Full `cargo test` after checked event dispatch — **159 passed, 0 failed**;
  doc tests: 0 passed, 0 failed. `cargo check --all-targets` passed.
- Full `cargo test` after legal-store concurrency and rollback tests — **161 passed, 0 failed**;
  doc tests: 0 passed, 0 failed. `cargo check --all-targets` passed.
- `git diff --check`
- Root README link audit — 1 README reviewed; all relative links resolve.
- Full `cargo test` after envelope-size limits — **163 passed, 0 failed**;
  doc tests: 0 passed, 0 failed. `cargo check --all-targets` passed.
- Full `cargo test` after adding `ModuleManager` — **166 passed, 0 failed**;
  doc tests: 0 passed, 0 failed. `cargo check --all-targets` passed.
- Full `cargo test` after global replay-store expiry cleanup — **167 passed, 0 failed**;
  doc tests: 0 passed, 0 failed. `cargo check --all-targets` passed.
- Full `cargo test` after provider error sanitization — **167 passed, 0 failed**;
  doc tests: 0 passed, 0 failed. `cargo check --all-targets` passed.
- Full `cargo test` after owner/co-owner enforcement — **179 passed, 0 failed**;
  doc tests: 0 passed, 0 failed. `cargo check --all-targets` passed.
- Targeted independent-connection owner-transfer race — one winner; persisted
  owner and one transfer audit event verified.
- Local Markdown-link scan — all relative links resolved.
- Secret-pattern scan of the tracked diff — no matches. This is a pattern
  scan, not a comprehensive secret detector.

- Full `cargo test` with persistent identity and cross-module flow — **175 passed, 0 failed**;
  doc tests: 0 passed, 0 failed. `cargo check --all-targets` passed.
- Current local verification after legal grant/withdrawal and dual-store
  recovery flow — focused `cargo test --test platform_flow`: **1 passed**;
  `cargo fmt --all`, `cargo check --all-targets`, and full `cargo test`:
  **182 passed, 0 failed**; doc tests: 0 passed, 0 failed. `git diff --check`
  passed.
- Current audit-fix verification: focused `cargo test --test bot_registry` —
  **12 passed**; `cargo fmt --all`, `cargo check --all-targets`, full
  `cargo test` — **182 passed, 0 failed**; doc tests: 0 passed, 0 failed;
  `git diff --check` passed.
- Current bot migration rollback verification: focused test passed;
  `cargo fmt --all`, `cargo check --all-targets`, and full `cargo test` —
  **183 passed, 0 failed**; doc tests: 0 passed, 0 failed; `git diff --check`
  passed.
- Current legal schema migration verification: the three focused migration
  tests passed; `cargo fmt --all`, `cargo check --all-targets`, and full
  `cargo test` — **186 passed, 0 failed**; doc tests: 0 passed, 0 failed;
  `git diff --check` passed.
- Current legal schema migration verification: 3 focused migration tests passed;
  `cargo fmt --all`, `cargo check --all-targets`, full `cargo test` —
  **186 passed, 0 failed**; doc tests: 0 passed, 0 failed;
  `git diff --check` passed.
- Current legal authorization verification after denied cross-user consent,
  denied user-data lookup, denied document registration/activation, and generic
  error checks: focused legal/core/platform flow tests passed;
  `cargo fmt --all`, `cargo check --all-targets`, full `cargo test` —
  **187 passed, 0 failed**; doc tests: 0 passed, 0 failed;
  `git diff --check` passed.
- Current runtime-owned lifecycle verification: focused callback-panic cleanup
  and cross-module runtime success/start-failure tests passed; `cargo fmt --all`,
  `cargo check --all-targets`, full `cargo test` — **189 passed, 0 failed**;
  doc tests: 0 passed, 0 failed; `git diff --check` passed.

These results verify only the code that exists. They do not verify absent bot
platform capabilities or a live external integration.

## README audit

- README files found: **1** (`README.md` at repository root).
- README files reviewed: **1**.
- README files updated for the prior replay-store checkpoint: **1**.
- README files reviewed for the lock-recovery checkpoint: **1**.
- README files with no changes for the lock-recovery checkpoint: **1**.
- README files reviewed for checked event dispatch: **1**.
- README files updated for checked event dispatch: **1**.
- README files with no changes for checked event dispatch: **0**.
- README files reviewed for legal-store resilience tests: **1**.
- README files updated for legal-store resilience tests: **0**.
- README files with no changes for legal-store resilience tests: **1**.
- README files reviewed for envelope-size hardening: **1**.
- README files updated for envelope-size hardening: **1**.
- README files with no changes for envelope-size hardening: **0**.
- README files reviewed for lifecycle coordination: **1**.
- README files updated for lifecycle coordination: **1**.
- README files with no changes for lifecycle coordination: **0**.
- README files reviewed for lifecycle cleanup refinement: **1**.
- README files updated for lifecycle cleanup refinement: **0**.
- README files with no changes for lifecycle cleanup refinement: **1**.
- README files reviewed for global replay-store expiry cleanup: **1**.
- README files updated for global replay-store expiry cleanup: **1**.
- README files with no changes for global replay-store expiry cleanup: **0**.
- README files reviewed for provider error sanitization: **1**.
- README files updated for provider error sanitization: **1**.
- README files with no changes for provider error sanitization: **0**.
- README files reviewed for bot identity registry: **1**.
- README files updated for bot identity registry: **1**.
- README files with no changes for bot identity registry: **0**.
- README files reviewed for transactional identity schema migrations: **1**.
- README files updated for transactional identity schema migrations: **1**.
- README files with no changes for transactional identity schema migrations: **0**.
- README files reviewed for cross-module flow test: **1**.
- README files updated for cross-module flow test: **0**.
- README files with no changes for cross-module flow test: **1**.
- README files reviewed for architecture wording correction: **1**.
- README files updated for architecture wording correction: **0**.
- README files with no changes for architecture wording correction: **1**.
- README files reviewed for persisted identity validation: **1**.
- README files updated for persisted identity validation: **0**.
- README files with no changes for persisted identity validation: **1**.
- README files reviewed for runtime registry injection: **1**.
- README files updated for runtime registry injection: **1**.
- README files reviewed for bot ownership and co-owner policy: **1**.
- README files updated for bot ownership and co-owner policy: **1**.
- README files with no changes for bot ownership and co-owner policy: **0**.
- README files reviewed for ownership example/roadmap correction: **1**.
- README files updated for ownership example/roadmap correction: **1**.
- README files with no changes for ownership example/roadmap correction: **0**.
- README files reviewed for co-owner code example and auth boundary: **1**.
- README files updated for co-owner code example and auth boundary: **1**.
- README files with no changes for co-owner code example and auth boundary: **0**.
- README files reviewed for owner-transfer concurrency test: **1**.
- README files updated for owner-transfer concurrency test: **0**.
- README files with no changes for owner-transfer concurrency test: **1**.
- README files reviewed for legal-consent and dual-store recovery flow: **1**.
- README files updated for legal-consent and dual-store recovery flow: **0**.
- README files with no changes for legal-consent and dual-store recovery flow: **1**.
- README files reviewed for owner-only audit-history reads: **1**.
- README files updated for owner-only audit-history reads and API example: **1**.
- README files with no changes for owner-only audit-history reads: **0**.
- README files reviewed for bot-registry migration rollback coverage: **1**.
- README files updated for bot-registry migration rollback coverage: **0**.
- README files with no changes for bot-registry migration rollback coverage: **1**.
- README files reviewed for legal schema versioning and DDL rollback: **1**.
- README files updated for legal schema versioning and DDL rollback: **1**.
- README files with no changes for legal schema versioning and DDL rollback: **0**.
- README files reviewed for the legal principal/action/resource contract: **1**.
- README files updated for the legal principal/action/resource contract: **1**.
- README files with no changes for the legal principal/action/resource contract: **0**.
- README files reviewed for legal principal enforcement across runtime/store APIs: **1**.
- README files updated for legal principal enforcement across runtime/store APIs: **1**.
- README files with no changes for that API enforcement checkpoint: **0**.
- README files reviewed for runtime-owned module lifecycle: **1**.
- README files updated for runtime initialize/start/shutdown: **1**.
- README files with no changes for runtime lifecycle integration: **0**.
- README files reviewed for transactional/versioned legal-store initialization: **1**.
- README files updated for transactional/versioned legal-store initialization: **1**.
- README files with no changes for transactional/versioned legal-store initialization: **0**.
- README files reviewed for the added Vault and WhatsApp/Telegram/Discord
  integration scope: **1**.
- README files updated for that scope: **0**.
- README files with no changes for that scope: **1**.
- README files with no changes for runtime registry injection: **0**.
- README files reviewed for lifecycle cleanup refinement: **1**; unchanged.
- README files reviewed for replay expiry cleanup: **1**, updated **1**, unchanged **0**.
- Problems found: documented JavaScript/Node project structure and `npm`
  commands did not match the Rust Cargo crate; future bot/provider and other
  systems were presented as available; repository links and component claims
  were stale; the required credit text was missing.
- Problems corrected: documented current Rust crate, actual commands and APIs,
  implemented/pending boundaries, security persistence limitations, exact
  required credits, authenticated Vault revalidation, timestamp validation,
  replay-store configuration and its SQLite persistence boundary, synchronous
  event callback behavior, explicit panic-isolated dispatch API, and provider
  input size limits. The Matterssmith
  banner/heading identity has been retained.
- Problems pending: component-by-component README examples cannot cover the
  absent subsystems; comprehensive release documentation waits for their
  implementation. No README was skipped.

## Stages and remaining work

- H: **In progress.** The initial inventory and gap classification are done;
  a confirmed provider Vault identity check gap was corrected with a
  regression test; legal duplicate-check/write sequences now use immediate
  transactions, access checks use one snapshot, and the independent-connection
  withdrawal race is covered.
  Provider assertion time arithmetic now rejects malformed extremes without
  overflow; replay tracking now supports SQLite persistence when configured.
  The default constructor remains process-local. Caller string
  conversions now run outside EventBus/Translator write locks, and reentrant
  callback behavior is tested. Provider inputs have bounded size/count.
  EventBus, ServiceRegistry and Translator now recover from poisoned internal
  locks; dedicated lock-poison tests pass. These subsystems still need broader
  concurrency review. Legacy `publish` propagates callback panics;
  `publish_checked` isolates unwinding panics and reports a dispatch error.
  The source pass found no `todo!`, `unimplemented!`, or production
  `panic!`; lock `.expect` calls were removed from production event, service,
  and translation registries. The rest of the module audit remains. A caller-owned `ModuleManager` was added with duplicate/name/version
  validation, initialization order, reverse shutdown, start-failure cleanup,
  and stop-before-start cleanup. Replay storage removes expired entries across
  scopes and has per-scope plus global capacity limits. `stop_all` also releases modules
  that initialized but never started, and retains failed-cleanup state for retry.
  `EventBus::publish_checked` continues notifying listeners after an individual
  unwinding panic and returns an error summary; default `publish` behavior
  remains backward compatible. SQLite replay storage rejects duplicate
  assertions across independent connections and verifier restarts; four focused
  tests pass. Legal store tests cover concurrent idempotent document
  registration and rollback after a simulated SQLite write failure. Bot identity
  registration persists family/release/instance identity and atomically resolves
  display-name collisions; owner/co-owner controls are scoped and audited.
  Audit-history reads now require the current owner. Envelope payloads
  are capped at 16 MiB before encryption or Base64 decoding. Errors
  from key, binding, and proof providers are sanitized at module boundaries; a
  regression test confirms an upstream secret-like diagnostic is not returned.
- I: **Not complete.** Added runtime legal-store close/reopen recovery,
  changing-provider-identity tests, and a cross-module identity/runtime/service/
  event/module lifecycle flow including legal grant/withdrawal and recovery of
  bot identity plus legal consent after reopening SQLite. Broad
  authorization/socket/service fault-injection tests remain blocked by absent
  subsystems.
- J: **Not complete.** Existing security-focused tests pass; general
  authorization, socket/plugin isolation, operational auditing, and an
  independent security review remain unavailable or absent.
- K: **Not complete.** Rust contracts and envelope schema exist; public API
  compatibility/versioning and language-neutral contracts are not defined.
- L: **Not complete.** The listed local checks passed, but H–K acceptance,
  dependency/license/secret review, integration coverage, and release criteria
  remain outstanding.
- Matter-Bot: **Blocked by unmet Matter-Base criteria. Do not start.**

Checkpoint `b788929235653a829f007f1ef38407b1d39ff4a9` bounded replay storage across scopes. Checkpoint `5c2b1db6e1c082212892852e20924b44aedac763` refined module cleanup.
Checkpoint `cd351b5c2267fb5ace70fa51b163a2dcb64b07ab` added the module lifecycle
coordinator. Earlier checkpoint `ba100c45433f145d9863e8aeffbb5bce29775b18`
added 16 MiB envelope payload bounds; `e48ce509add1e08eaed63dfda3917b3f5d14accd`
added checked event dispatch; `9c77cff43889028606eb384afc708eab0e597ed2`
added legal-store concurrency/rollback tests. Earlier checkpoints added lock
recovery and persistent replay storage. These commits were pushed to
`origin/matter-beta` over authenticated SSH and confirmed by fetch. No promotion to `matter` was made. Matter-Vault live
verification is the only specifically identified external dependency; most
remaining blockers are missing code and contracts in this repository, not
external services.

## Previous published checkpoint

- Branch: `matter-beta`; after safe stash, fetch, reset, and stash reapply,
  both `HEAD` and `origin/matter-beta` were
  `b9ca1d0e46c3f63428d9545da7e9923f6b86b129`. No conflicts; ignored
  `target/` was preserved.
- Added a module shutdown retry test and a cloned-runtime concurrent-start
  integration test. They verify retained cleanup state and serialized lifecycle
  callbacks.
- Files changed: `src/modules/mod.rs`, `tests/platform_flow.rs`, and this
  ledger. No files created or deleted. README audit: 1 found, 1 reviewed,
  0 updated, 1 unchanged. No README issue applies to these test-only changes.
- Verification passed: `cargo fmt --all`, `cargo check --all-targets`,
  `cargo test` (191 tests, 0 failed; 0 doc tests), and `git diff --check`.
- Commit `348431451771822581505484b4d042c098e92cba` was pushed to
  `origin/matter-beta`; fetch confirmed the published SHA. No stable branch
  promotion was made.

## Latest checkpoint — shared user registry

- Implemented `UserRegistry`, `UserPrincipal`, `UserAuthorizer`, and a
  replaceable `UserStore`; provided in-memory and SQLite stores.
- Identity links are explicit and unique by provider plus external subject.
  Repeated links to the same user are idempotent; linking the same external
  identity to another user fails. Matching identifiers across providers remain
  distinct.
- Bot data is keyed by bot instance, shared user ID, and data key. Every
  create/link/unlink/resolve/read/write/delete operation requires the supplied
  authorization policy. The application must authenticate principal IDs and
  verify bot existence/ownership; that integration is not yet provided here.
- Added optional user-registry injection to `MatterContext` and
  `MatterRuntime::open_with_registries` / `with_stores_and_users`.
- Added tests for SQLite reopen recovery, bot isolation, denial sanitization,
  provider namespace separation, link ownership, and concurrent link claims.
- Files changed: `src/users/mod.rs` (new), `src/lib.rs`,
  `src/core/context.rs`, `src/core/runtime.rs`, `tests/users.rs` (new),
  `README.md`, `docs/architecture.md`, and this ledger. No files deleted.
- Verification before the recovery interruption: `cargo fmt --all`,
  `cargo check --all-targets`, full `cargo test` (195 passed, 0 failed; 0 doc
  tests), `git diff --check`, and focused user tests (5 passed after the runtime
  injection/recovery case). After restoring the working tree, the full sequence
  passed again: `cargo fmt --all`, `cargo check --all-targets`, `cargo test`
  (196 passed, 0 failed; 0 doc tests), and `git diff --check`. Platform flow
  passed (3); no live services were used.
- README audit for this work: 1 found, 1 reviewed, 1 updated, 0 unchanged.
- Commit `6b501997f377e8dd424ba78669b04796f6a2a989` was pushed to
  `origin/matter-beta`; a subsequent fetch confirmed the same SHA. The README
  audit found 1 README, reviewed 1, updated 1, unchanged 0; its relative links
  resolve. No stable-branch promotion was made.
- Recovery note: an earlier stash attempt failed while intent-to-add entries
  were present; a separate preserved bot-registry stash was partially applied
  and conflicted. It remains in `git stash list` and was not discarded or
  merged. Its files and conflict snapshots were copied to
  `/tmp/Matter-Base-recovery-20261116`. The user-registry work was restored from
  Git objects `d1f504f…` (tracked files) and `e213e3b…` (new source/tests).
  Review the preserved bot stash later; its alternative in-memory bot registry
  conflicts with the existing persistent bot registry and should not be merged
  without a deliberate architecture review.

## Current checkpoint — user-store resilience tests

- Added SQLite migration rollback/retry coverage, future-schema rejection
  without rewriting stored data, and rejection of writes to unregistered users
  or over-limit values.
- Changed files: `tests/users.rs` and this status ledger. No files created or
  deleted. No production behavior changed in this checkpoint.
- Verified after synchronizing with `origin/matter-beta` at
  `81b48f5a1a42c927650ea4e9b5e3b9230c615bea`: `cargo fmt --all`,
  `cargo check --all-targets`, focused user tests (8 passed), full `cargo test`
  (199 passed, 0 failed; 0 doc tests), and `git diff --check` all passed.
- README audit: 1 found, 1 reviewed, 0 updated, 1 unchanged; test-only changes
  do not affect README claims.
- Commit `7f385de9579094221ab55e52d3ac672969b846ed` was pushed to
  `origin/matter-beta`; a subsequent fetch confirmed the exact SHA. No stable
  branch promotion was made. The original separate bot-registry stash remains
  preserved.

## Current local work — bot co-owner resource bound

- Added a 256 co-owner cap to both in-memory and SQLite assignment paths.
  Persisted identities with larger lists fail validation; ownership transfer
  and removal also fail closed when they encounter an oversized list.
- Added coverage for both stores and a corrupted SQLite row.
- Updated README and architecture documentation to state the cap.
- Audited public legal-store entry points. Sensitive reads, writes, document
  administration, and consent paths invoke `LegalPrincipal` before storage
  access. Authorization diagnostics are sanitized; no omitted auth path was
  confirmed in this pass.
- Verified after syncing with `origin/matter-beta` at
  `34f31718da728822cde3d12aca69437fb96ec553`: `cargo fmt --all`,
  `cargo check --all-targets`, focused bot-registry tests (15 passed), full
  `cargo test` (201 passed, 0 failed; 0 doc tests), and `git diff --check`.
- README audit for this code checkpoint: 1 found, 1 reviewed, 1 updated,
  0 unchanged; relative links resolve.
- Commit `82e4562ca6b022c2d45a53744f92da87d0b7ad92` was pushed to
  `origin/matter-beta`; a subsequent fetch confirmed the same SHA. No stable
  branch promotion was made.

## Current local work — reject empty provider key sets

- Confirmed that a provider returning a successful but empty key list caused
  `KeyManager` to replace and erase its current keys. Empty results now fail
  closed, preserving the current key set and active key.
- Added a regression test. Updated README and architecture documentation.
- Verified after syncing with `origin/matter-beta` at
  `b1f286c982e580608d2f37be6610fc8b5ea479a4`: `cargo fmt --all`,
  `cargo check --all-targets`, focused key-provider tests (7 passed), full
  `cargo test` (202 passed, 0 failed; 0 doc tests), and `git diff --check`.
- README audit: 1 found, 1 reviewed, 1 updated, 0 unchanged; relative links
  resolve.
- Commit `b1afcc84490de64436a909072bf4f15ae699d912` was pushed to
  `origin/matter-beta`; a subsequent fetch confirmed the exact SHA. No stable
  branch promotion was made.

## Exact next task to resume

1. Continue H by auditing remaining provider/key and cross-module trust
   boundaries; add tests for confirmed defects. Authenticated key reads are
   Vault-scoped, and bot identity reads/lists now enforce owner/co-owner scope.
2. Continue I with cross-module resilience around bot ownership, provider
   failures, legal-store errors, and shared users. One bot-owner/shared-user
   isolation flow is now covered; router, sockets, plugins, general resource
   authorization, and live services remain uncovered.
3. Complete J, K, and L in order. Reassess capabilities from actual code and
   keep Matter-Bot blocked until all readiness criteria are satisfied.
4. At each checkpoint, review every README, update this ledger, commit and push
   to `matter-beta`, then fetch and verify the exact published SHA. Never claim
   Vault or platform integrations work without real verification.

## Documentation audit — service registry

- Corrected `docs/services.md`: its lock-poisoning note no longer matches the
  implementation. The registry recovers poisoned locks and has a regression
  test; lifecycle, health checks, persistence, and authorization remain
  unimplemented.
- README audit: 1 found, 1 reviewed, 0 updated, 1 unchanged. No README claims
  the service registry provides lifecycle or connection management.
- Documentation-only correction; no tests were run for this edit.
- Base before publication: `5ee4a71e562decc3e72b38fbe54e765b2736ed02`.

## Current local work — total key-manager capacity

- Added the 4,096-key limit to manual `insert` and `rotate`, matching the
  provider-load limit. At capacity, both operations fail without mutating
  key count or active-key state.
- Added a boundary regression test and updated README/architecture docs.
- Verified after syncing with `origin/matter-beta` at
  `6b7a7f8e75c5d5409b9e5b81cfdd1e97b4e06b1e`: `cargo fmt --all`,
  `cargo check --all-targets`, focused key-provider tests (8 passed), full
  `cargo test` (203 passed, 0 failed; 0 doc tests), and `git diff --check`.
- README audit: 1 found, 1 reviewed, 1 updated, 0 unchanged; relative links
  resolve.
- Commit `7ffe9c570033b0e50cd0ce4a00c902e3772d786a` was pushed to
  `origin/matter-beta`; a subsequent fetch confirmed the exact SHA. No stable
  branch promotion was made.

## Current local work — key-manager Vault isolation

- Confirmed that an `AuthenticatedProvider` could read from a `KeyManager`
  loaded from a different Vault when the manager handle was reused.
- Authenticated key ID, key material, and active-key reads now compare the
  manager's recorded Vault ID with the authenticated identity and reject a
  mismatch. In-memory manually inserted keys without a provider Vault remain
  usable by existing direct tests.
- Added a two-Vault regression and documented the scope check.
- Verified after syncing with `origin/matter-beta` at
  `40c3bcac140aaab8b91be8541a849293b6f0ec71`: `cargo fmt --all -- --check`,
  `cargo check --all-targets`, full `cargo test` (204 passed, 0 failed; 0 doc
  tests), and `git diff --check`.
- README audit: 1 found, 1 reviewed, 1 updated, 0 unchanged. The architecture
  note was updated too; the documented internal links still resolve.
- Commit `66b327616c05344305f3351195541869161c4e6b` was pushed to
  `origin/matter-beta`; a subsequent fetch confirmed the exact SHA. No stable
  branch promotion was made.

## Current local work — bot and shared-user integration

- Added a cross-module test that creates two bot instances and a shared user,
  persists them in separate SQLite stores, closes and reopens both stores,
  authorizes access through recovered bot-owner permissions, verifies user data
  remains isolated by instance, and rejects cross-owner reads and writes.
- After syncing with `origin/matter-beta` at
  `7b455090f1cf25ed2b6b1dc6a1cd25da57a1a5b3`, `cargo fmt --all -- --check`,
  `cargo check --all-targets`, the full suite (205 passed, 0 failed; 0 doc
  tests), and `git diff --check` passed.
- README audit: 1 found, 1 reviewed, 0 updated, 1 unchanged. This test adds no
  public behavior; README and architecture claims remain accurate.
- Commit `d1e9af11bad9785b137cb9018b4024e7ebd864ed` was pushed to
  `origin/matter-beta`; a subsequent fetch confirmed the exact SHA. H and I
  remain in progress; no stable branch promotion was made.

## Current local work — bot identity read authorization

- Confirmed that `BotRegistry::get` and `list` returned owner and co-owner
  identity data without an actor scope.
- Both methods and the `BotIdentityStore` contract now require an actor ID;
  reads return only records owned by or shared with that actor. Inaccessible
  single records appear absent, and list results are filtered. The registry
  repeats filtering for replaceable store implementations.
- Updated SQLite and in-memory stores, legacy migration tests, and call sites.
  Legacy unowned records stay hidden until successfully claimed through the
  recovery authorizer.
- README and architecture now show the actor-scoped signatures and the
  required authenticated-request boundary.
- After syncing with `origin/matter-beta` at
  `007d4e0e52cc42a9c629111cc0c0b33984078cf4`, format, all-target check, focused
  bot/platform integration tests, full suite (205 passed, 0 failed; 0 doc
  tests), and `git diff --check` passed.
- README audit: 1 found, 1 reviewed, 1 updated, 0 unchanged.
- Commit `ff2301abbb01bd4d7b0beca05f7bd26cce708cf0` was pushed to
  `origin/matter-beta`; a subsequent fetch confirmed the exact SHA. H remains
  in progress; no stable branch promotion was made.

## Current local work — universal command router

- Added and exported `CommandRouter`, `CommandInvocation`, and the mandatory
  `CommandAuthorizer` contract. It handles configured prefixes, case-insensitive
  canonical names and aliases, whitespace arguments, middleware order, and
  synchronous handler dispatch.
- Enforced limits for input bytes, prefixes, command names/count, aliases,
  middleware count, and argument count. Aliases resolve to canonical names
  before authorization. Authorization, middleware, and handler callbacks run
  outside the registry lock; callback errors/panics are returned as sanitized
  errors.
- Added unit coverage for alias authorization, middleware order/rejection,
  unauthorized dispatch, duplicate names, size limits, and callback panics, as
  well as an integration test exercising the documented root API.
- Added a cross-module flow test wiring `CommandRouter` authorization directly
  to the real bot registry: one owner executes against its instance, another
  owner's request is rejected before handler effects, and that owner can use
  its own separate instance.
- Command routing remains PARTIAL: durable audit records, platform-specific
  argument parsing, and Control Plane/socket activation are not integrated.
  The authorizer must verify authenticated actor, bot ownership, permissions,
  and operational activation.
- After syncing with `origin/matter-beta` at
  `9eb8943b281f7e8a35a981068d08406fcf7859ce`, format, all-target check, full
  suite (214 passed, 0 failed; 0 doc tests), and `git diff --check` passed.
- README audit: 1 found, 1 reviewed, 0 updated, 1 unchanged. The public example
  is compiled and exercised by `tests/commands.rs`.
- Router implementation commit `1c3225ba3e8ac93bd669203c914421c571dc3705` and
  integration commit `7212f10bf531f69de417b4a95970a868d92fa490` were pushed to
  `origin/matter-beta`; a subsequent fetch confirmed the exact SHA. H and I
  remain in progress; no stable branch promotion was made.

## Additional requested ecosystem scope

The user added the following required scope; it is a goal, not an implemented
or verified capability:

1. Complete Matter-Base stages H–L and meet the existing readiness criteria.
2. Integrate Matter-Vault through its independent repository/service contract
   and verify a real integration when that service is available. Matter-Base
   must not store platform credentials or WhatsApp sessions.
3. Build and verify a functional WhatsApp adapter in Matter-Baileys, including
   account linking (QR/code where supported), receive/send, filters, activation
   controls, instance isolation, reconnect, and persistence. Live verification
   must use a secondary test account; do not overwrite existing sessions.
4. Then add Telegram (Matter-Telegraf) and Discord (Matter-Discord) adapters
   with equivalent capabilities where each platform supports them, against
   Matter-Base's stable platform-neutral contracts.
5. Keep provider-specific logic in the adapter repositories. Do not start
   adapter development before Matter-Base is ready, and do not claim a live
   integration based only on mocks.

These repositories and services must each be inspected for their real branch,
state, instructions, and API before work. At this checkpoint the only verified
repository is Matter-Base; the availability and state of Matter-Vault and the
three adapter repositories have not yet been inspected.

For later work, re-fetch the development branch before verification or code
changes, using the safe stash/fetch/reset/reapply procedure from the project
instructions. Never infer publication from local commit state alone.

Bot identity/ownership implementation in checkpoint `1a4750ca90572701192451d9cd635d6c3e9eb495`: `src/bots/mod.rs`,
`src/lib.rs`, and `tests/bot_registry.rs`, plus cross-module behavior in
`tests/platform_flow.rs`. The registry provides memory and SQLite stores,
validation, generated instance IDs, atomic unique names, persistence recovery,
transactional component-scoped schema migrations, future-version rejection,
cross-connection concurrency, owner-only changes/deletion, bounded co-owner
authority, legacy owner recovery hooks, persistent ownership audit records, and a
  separate-connection owner-transfer race with one winner.
Owner/co-owner operations use point-of-write authorization and transactional
audit records; legacy unowned records require an injected recovery authorizer.
Owner IDs must come from an authenticated application principal; this crate does
not authenticate platform users or verify owner IDs against a shared user store.
Component-scoped schema versioning is transactional and rejects unsupported
future versions. The full suite passed
with 180 tests; `cargo check --all-targets` and `git diff --check` passed. The
README audit found 1 README, reviewed 1, updated 1, unchanged 0; all relative
links resolve.
Owner/co-owner checkpoint `1a4750ca90572701192451d9cd635d6c3e9eb495` was pushed to
`origin/matter-beta`; a subsequent fetch confirmed the same SHA. No promotion
to `matter` occurred.

## 2026-11-16 — Persistent activation prerequisite for WhatsApp

- Branch: `matter-beta`; synchronized base: `77648da3658fed79cd6a18e7de10f4cadae70a13`.
- Stage H corrections and prerequisite implementation remain in progress; H–L
  and Matter-Base readiness are NOT complete.
- Added `src/activation.rs`, `tests/activation.rs`, and `docs/activation.md`;
  updated `src/lib.rs`, README, architecture and this ledger; no files deleted.
- Implemented SQLite activation ownership, default power-off, group/socket
  overrides, trusted global fallback, versioned transactional schema and atomic
  policy/audit writes. Audit failure rolls back the state mutation. Corrupt
  persisted policy values fail closed rather than interpreting any nonzero
  integer as enabled.
- Trusted application provisioning and actor authentication remain required.
  There is no network connection, session storage, bot/socket binding, ownership
  transfer, interchangeable backend or integrated command gate yet.
- Executed `cargo fmt --all`, `cargo check --all-targets`, focused activation
  tests (4 passed), full `cargo test` (218 passed, zero failed) and
  `git diff --check` successfully after synchronization.
- README audit: 1 found, 1 reviewed, 1 updated, 0 unchanged. Corrected stale
  claims that command routing and shared users were absent; added actual module
  paths and activation limitations. Internal links were checked. Credits preserved.
- Read-only GitHub inspection: Matterssmith/Matter-Baileys has only branch
  `matter`, at `57226de377c1dfeecd9de49415fb593c8ac01936`, with only README.md
  in its root. No runnable adapter is available there. No session was opened.
- Exact next task: add authenticated bot/socket binding and a dispatch boundary
  that intercepts control prefixes before normal routing and rejects normal
  commands while powered off. Test owner denial, concurrent changes, persistence
  and handler side effects before declaring that flow verified.
- Continue with session/credential contracts, connections/messaging, persistent
  command audit and remaining readiness criteria; real WhatsApp testing also
  requires implementing the external adapter and linking a secondary test account.
- To resume: read this ledger, inspect actual Git and instructions, preserve local
  changes and ignored data, fetch/synchronize matter-beta, then continue the exact
  next task. Existing legacy bot-registry stash is retained.

- Publication confirmed: activation checkpoint
  `916fdbbf61c7b6c1668f7d58f0c43df3eb323d09` is on origin/matter-beta.
  README re-review for this ledger-only commit: 1 found/reviewed, 0 updated,
  1 unchanged; no new documentation discrepancies. `git diff --check` passed.

## 2026-11-17 — Compatibilidad de tests CLI y ruta Base-Matter

- Situación observada por el operador: `cargo check --all-targets --locked`
  y `cargo test --all-targets --locked` fallan localmente porque `env!`
  exige `CARGO_BIN_EXE_whatsapp-matter-control` durante la compilación
  de los tests `whatsapp_cli` y `whatsapp_process`.
- Corrección propuesta: resolver la ruta con `option_env!` y una alternativa
  de entorno de ejecución; mantener las pruebas contra el binario real.
- Se actualizó la guía de dos terminales para `/home/Sakura/Base-Matter`.
  La ruta de sesión esperada bajo `~/.local/share/base-matter/whatsapp-test`
  no existe en el entorno comunicado; recuperar cualquier sesión anterior
  solo tras localización y verificación, sin borrados ni regeneración.
- Se revisaron el README raíz y el README de `providers/whatsapp-rust`;
  el README raíz se actualizó para la verificación y el nombre real del repo.
  El README del proveedor no requiere edición para esta corrección.
- Verificación remota, pruebas CI y sesión viva: pendientes de comprobar
  en el SHA exacto una vez publicados los cambios.
