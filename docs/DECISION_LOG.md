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

## 2026-11-19 — Política obligatoria de cifrado para capacidades protegidas

- Toda información operativa que atraviese una capacidad protegida se clasifica como credencial: cifrado autenticado en tránsito/almacenamiento, claves externas y segregación por bot/socket/recurso. Sin cifrado o autoridad real se **deniega**, nunca se permite fallback en texto claro.
- Base-Matter público expone solo contratos, identificadores y solicitudes; **no conoce rutas ni nombres de repositorios privados**, no descarga implementación interna y no debe poseer claves para descifrar material privilegiado. El host confiable autoriza y transporta ciphertext opaco.
- Regla vinculante: `docs/PROTECTED_CAPABILITY_ENCRYPTION.md`. Contrato existente aún no implementa canal operativo cifrado. HMAC aislado no proporciona confidencialidad. Los datos operativos y auditorías deben seguir cifrados; el descifrado temporal solo ocurre en el proceso autorizado.
- Corrección documental, no modifica runtime/WhatsApp ni demuestra cumplimiento criptográfico. No promover `alpha` hasta pruebas Linux/Windows, seguridad y proveedor. Bot-Matter sigue bloqueado. Ambos README y registros de continuidad revisados.
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

## BASE-D018 — AntiLink único y dominios por ámbito (2026-11-19)

- Petición del propietario: eliminar filtros Anti por red social y usar `set anti link` para añadir varios dominios por chat o `--global` por socket.
- Decisión: nombres de reglas sociales retirados del catálogo operativo; sin modificación destructiva de datos viejos. Lista persistente `socket_id,scope_id,domain` por SQLite, con auditoría y autorización del dueño.
- Filtro general OFF por defecto, detección pasiva, sin acciones automáticas. Dominio global heredado, local independiente.
- Base 8b0fc5516e6d8ab7e313469bfbd05bd1103d7f30. CI pendiente. READMEs 2/2. Bot-Matter bloqueado.

## BASE-D017 — Frontera de negocio Matterssmith Commerce (2026-11-19)

- Idea tienda/hardware/generador del propietario convertida a contrato transversal; Base-Matter no implementa lógicas de mercado ni datos Vault.
- Contratos neutrales y seguridad siguen siendo su función; Bot-Matter permanece bloqueado. SHA previo 24349b0ae29c88ebecda571de01821fe310b8800. No hay pruebas comerciales activas.

## BASE-D016 — Rustfmt de contrato guest (2026-11-19)

- CI del SHA 3c1fa6f325df63718eb67eefb9ce1a66ff4424e4: proveedor PASS, Rustfmt FAIL. Aplicar solo cambios de estilo requeridos por CI en lib.rs y tests/whatsapp_public_readonly.rs.
- Sin cambios de seguridad/alcance. README 2/2. Nuevo SHA pendiente de CI.

## BASE-D015 — Sin registro obligatorio: invitado demo (2026-11-19)

- Invitados pueden consumir funciones básicas autorizadas, sin CURP, biometría ni registro Vault; heredan las restricciones normales del socket/chat.
- `=/>register` no da roles ni confirma alta. El futuro gateway de Vault es voluntario y verificará consentimiento para funciones personales.
- Base alpha previa: ea7c0da6c579f24406093940e7677fe6a349dfbd. Nuevos tests guest + puente. README 2/2. Bot-Matter todavía bloqueado. CI y prueba de teléfono pendientes.

## 2026-11-19 — Clippy Rust del receptor ListResponse

- CI del SHA 031ebb79ad1edc7c81d68a8997ba9c11918681b5: proveedor compiló y todos sus tests pasaron; Clippy estricta falló en tests/ui.rs por un `..Default::default()` innecesario en `SingleSelectReply`. Se quitó sin modificar protocolo ni permisos.
- README revisados 2/2. CI del nuevo SHA pendiente. Siguiente: lista de dominios personalizable en vez de reglas anti por red social, después prueba real con =/>test ui.

## BASE-D013 — Decodificar ListResponse + ACK interactivo verificado (2026-11-19)

- Una burbuja «=/>menu anti» no equivale a que el proveedor haya recibido el clic.
- Interpretar ListResponse y TemplateButtonReply además de NativeFlow y botones clásicos, con IDs cerrados y HMAC.
- Distinguir ACK tecleado del callback real para test UI; sin permisos adicionales. SHA base 37698e3967af5b2857c252473d5ad792a289a3e1. CI pendiente. README 2/2.

## BASE-D012 — Rustfmt y regresión de botones NativeFlow (2026-11-19)

- SHA base: 351d4d4a416cf8aa6b15fc33a6607bf1150763b7; el CI de proveedor marcó fallas de formato solamente.
- Decisión: aplicar rustfmt exacto y probar tamaños 2/3, labels y correspondencia al ACK; mantener todas las rutas bajo control HMAC/antirreplay.
- Evidencia UI Android del selector disponible; render visual TS18/TS19 y HTML siguen pendientes.
- README revisados: 2/2; CI nuevo pendiente.

## BASE-D011 — El título del selector debe ser el comando, y laboratorio NativeFlow opt-in

- Reporte: WhatsApp muestra al pulsar una fila «☾︎ Anti / Abrir menú». Decisión: `title` exactamente `=/>menu anti`; quitar `description`, preservar IDs opacos, HMAC y read-only.
- La prueba TS18/TS19 anteriormente solo usaba ButtonsMessage legacy y self-chat. Se agrega NativeFlow quick_reply (compatible a nivel de transporte con el selector probado). En `MATTER_WA_MENU_LAB=1` se muestran tarjetas readonly en privados habilitados, con fallback legacy por fallo de transporte; el dueño en self-chat recibe también comparación legacy.
- HTML/audio siguen limitados al self-chat del dueño; compatibilidad HTML del cliente aún no demostrada.
- Base alpha: c2d8c2c84fe1bbd8ed11f667004654848c9bc9ae. CI y prueba real pendientes. README revisados: 2/2.

## BASE-D010 — Instrucciones compactas en citas de WhatsApp (2026-11-19)

- Solicitud de Dev MS Leito Delfin 3 Siss: convertir el bloque largo de instrucciones en líneas compactas, usando `> texto`.
- Diseño: cuatro citas breves; sin párrafos o líneas vacías entre instrucciones. Se mantiene la decoración ajena al contenido, las categorías no vacías y el botón único.
- Compatibilidad: los estilos `classic` y `none` comparten semántica; `none` no recibe citas decorativas.
- Reutilización: `TextFormat::Quote` con saltos de línea seguros y catálogo de formato extendido.
- Base alpha: `8a1176ff7cdfe1d86c5c5d689c4ec2e9cd1620c0`. Revisión 2/2 README. Pruebas automáticas y prueba visual nuevas: pendientes.

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

## BASE-D009 — UX Leito y alfabetos reutilizables (2026-11-19)

- Propuesta recibida: hacer perceptible la diferencia entre adorno y contenido mediante formatos de WhatsApp.
- Decisión: ocultar categorías sin comandos implementados en menú y botones; mantener 21 identificadores solo en contrato interno y routing histórico de lectura.
- Usar manifiesto compartido en Rust y catálogo JSON de cinco alfabetos, marco reutilizable y módulo de formato. No conceder permisos ni añadir comandos.
- Base: 8639b9e4beab5217f344766d74c1b85578b3daba. CI pendiente; READMEs 2/2.

# Historial de decisiones de implementación — Base-Matter

## BASE-D008 — corrección de casos positivos/negativos de grupo
- Incidencia en  ce9465a8: reemplazo inicial mal ubicado.
- Ahora `=/>menu group` aparece solo en el conjunto autorizado
  y `=/>menu unknown` solo en el conjunto rechazado.
- El código de seguridad no cambió; solo la prueba.
- README revisados: 2/2; CI pendiente.


## BASE-D007 — recuperación del checkpoint y test de grupo
- SHA anterior: 97de8ac040148925baa194710a0d5dab82d90aff.
- Incidencia: el commit previo omitió WORK_CONTINUITY.
- Prueba: `menu group` es una categoría permitida, no debe
  figurar en la lista de denegaciones del signer.
- Corrección: sustituida por `menu unknown`.
- Resultado: revalidación nueva pendiente; README 2/2.


## BASE-D006 — Fixture dinámico del menú y guías WhatsApp
- Falla CI: tests/whatsapp_public_readonly.rs probaba la nueva
  navegación con un renderer de plantilla estática antigua que sólo
  soporta "menu". No es fallo de permisos de la ruta pública.
- Solución: mantener fixture legado para sus tests y probar
  categorías/páginas en fixture con MenuPreferences real.
- Formato: corrección de rustfmt del match del proveedor.
- Documentos: instrucciones precisas de prueba audio/HTML/TS18/TS19.
- Estado: CI nuevo por verificar; ningún dato secreto modificado.
- README revisados: 2/2.


## BASE-D005 — Rustfmt laboratorio, 18-11-2026
- Problema: CI de provider y Rustcore rechazaron formato de arrays,
  match e imports del laboratorio y los tests de catálogo.
- Decisión: aplicar rustfmt sin variar comportamiento ni permisos.
- Evidencia: workflows de SHA 05ec05c5bba5bb33b63dcb9aec457e2f8fcedda4.
- Resultado: archivos modificados y publicados, CI nueva pendiente.
- README revisados: 2/2.


## BASE-D004 — Ingreso de 21 categorías y páginas, 18-11-2026
- Se evita que el click de un selector ya autorizado sea descartado
  por el signer público, manteniendo lectura sin permisos de modificación.
- Las 21 rutas visibles y las páginas 1–3 son entradas válidas;
  aliases históricos sólo de compatibilidad.
- Pruebas: signer read-only y despachador con las 21 entradas,
  seguridad de botones, vista HTML con audio original.
- Estado: CI pendiente, teléfono todavía no probado. README 2/2.


## BASE-D003 — Menú HTML y sonido TS18/TS19, 2026-11-18
- Origen: prueba visual =/>menu pedida por el propietario.
- Decisión: un selector y 21 categorías primero; tarjetas TS18
  (Audio/MP4), TS19 (Audio/MP4/Enlace), RichResponse y un tono
  de 0.45s como extras sólo en self-chat verificado y opt-in.
- Implementación: proveedor Rust, audio Opus original, sin JavaScript,
  suplantación Meta AI, descargas ni roles; fallback texto.
- Pendiente: CI Linux/Windows/proveedor, móvil real, compatibilidad
  RichResponse y botones. No afirmar éxito visual sin pantalla.


## 2026-11-18 (MX) — BASE-D002: permiso informado de Jeriel
- **Origen:** titular del ecosistema comunica autorización de Jeriel,
  creador de Aura Reed, para usar su clave de IA según lo planeado.
- **Límites:** no se informaron restricciones numéricas. NO confundir
  eso con cuota infinita, ausencia de términos del proveedor ni
  autorización para otras capacidades.
- **Estado:** consentimiento comunicado; evidencia/titularidad
  operativa pendiente de verificar por Vault, sin clave ni gateway.
- **Referencia autoritativa:** Vault-Matter privado, rama matter,
  docs/AI_PERMISSION_AURA_REED.md, commit
  8872f9c3dc3d6aafcf4268c531e7c07f571cbcf4.
- **Arquitectura:** Base-Matter continúa solicitando sólo resultados
  al broker autenticado de Vault. No extrae claves ni ejecuta el
  código sugerido por la IA. Capacidades previstas: guía de menús,
  revisión/propuestas de código con control independiente.
- **Archivos:** AGENTS, docs/MATTER_AI.md, documentos de continuidad
  y ambos README. No cambia transporte ni claves.

## 2026-11-18 (MX) — BASE-D001: registro de cada idea/cambio
- **Origen:** petición expresa de guardar instrucciones y punto
  exacto de avance en caso de perder la comunicación.
- **Regla:** AGENTS, este historial, docs/WORK_CONTINUITY.md
  y docs/DEVELOPMENT_STATUS.md se actualizan en cada commit.
  Auditar README rastreados (2/2).
- **Control:** scripts/check_continuity.py integrado a CI para
  impedir commits futuros que omitan estos registros.
- **Problema corregido:** test de Linux esperaba encabezado de help
  dentro de menu antis 2. Se separa menu de help. En Windows,
  tests/command_audit.rs chocaba en nombres temporales por PID/reloj.
  Se reemplaza la carpeta de prueba por nonce criptográfico.
- **Estado:** cambios incorporados en alpha, revalidación CI pendiente.
- **Siguiente:** validar Linux, Windows y proveedor sobre SHA exacto,
  revisar manualmente menú/help y selector en WhatsApp.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
