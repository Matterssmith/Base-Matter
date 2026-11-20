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

## 2026-11-19 — Política obligatoria Vault-D019: protección equivalente a credenciales

- **Regla permanente del ecosistema:** TODOS los activos administrados por Vault — roles, permisos, decisiones, credenciales, políticas, metadatos, evidencia, auditoría, caché, respaldos y estados — deben cifrarse/autenticarse en reposo y transmitirse por canales cifrados y autenticados. Descifrado temporal sólo en memoria confiable; claves fuera de Git, nunca textos sensibles en logs, respuestas públicas ni archivos temporales.
- Base pública: sin rutas, URLs ni dependencias a repositorios privados. El selector provider/language/system y los puentes emiten solicitudes sin credenciales; todo dato o decisión privada del servicio Vault deberá viajar por un canal autenticado y cifrado, y jamás se persistirá en claro. No cambiar el código de runtime hasta existir gateway verificado. Al recibir un resultado mínimo autorizado, no volcarlo a logs, tests ni mensajes. `alpha` solamente; beta/stable intactos.
- La norma principal se mantiene en la política privada VAULT-D019. Esta actualización es documental y no acredita gateway Vault, ejecución privada o migración de sesiones. No conceder permisos sin verificación y no afirmar pruebas funcionales.
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


## Límite de comercio Matter

Revisar `docs/COMMERCE_ECOSYSTEM_BOUNDARY.md` antes de añadir código de producto, propiedad, garantía, flete o API de tienda a Base-Matter. Base solo contratos de core y seguridad, Vault autoridad de propiedad y roles, Marketplace dominio comercial. Mantener la prioridad actual y Bot-Matter bloqueado; registrar documentación/README en cada commit.

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

# Base-Matter — repository instructions

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


## 2026-11-18 — Laboratorio del menú en el chat propio
La prueba =/>menu mantiene primero las 21 categorías originales y UN
selector «Categorías». Con MATTER_WA_MENU_LAB=1, solo en self-chat
autenticado del dueño, se ensayan HTML RichResponse opcional,
tarjetas TS18/TS19 Audio/MP4/Enlace y audio Ogg/Opus original.
No hay autoplay, JS, descarga real ni permisos extra por pulsaciones;
nunca enviar estas pruebas a terceros. Requerir fallback texto.
Registrar cada cambio también en DECISION_LOG, WORK_CONTINUITY,
DEVELOPMENT_STATUS y revisar ambos README.


## REGLA PRIORITARIA — CONTINUIDAD Y DECISIONES POR COMMIT (2026-11-18)

Consultar siempre, **antes de modificar**, docs/WORK_CONTINUITY.md,
docs/DECISION_LOG.md, docs/CATEGORY_CONTRACT.md, docs/MATTER_AI.md,
docs/DEVELOPMENT_STATUS.md y todos los README. Recuperar el SHA real
de GitHub y el CI de ese SHA. El registro más reciente prevalece sobre
notas históricas contradictorias de este documento.

Por CADA idea, decisión, cambio, corrección de Rustfmt, prueba o
actualización que produzca un nuevo commit, editar EN ESE MISMO COMMIT:
- AGENTS.md: instrucción vigente, o referencia con estado y corrección;
- docs/DECISION_LOG.md: origen de la idea, decisión, archivos y límites;
- docs/WORK_CONTINUITY.md: SHA anterior, alcance, pruebas, pendientes,
  bloqueos, pasos exactos de reanudación y próximo trabajo;
- docs/DEVELOPMENT_STATUS.md: resumen técnico por etapa.
Revisar TODOS los README rastreados antes de cada commit, actualizar
cualquier texto desactualizado e indicar «README revisados: 2/2».
El CI valida la presencia de estos cambios conjuntos. El nuevo
commit debe publicarse y compararse con la rama remota antes de
informar que está seguro. Nunca inventar SHA ni resultados CI:
anotar «pendiente de CI» en el commit inicial y comprobar acciones
reales al terminar. Un nuevo cambio después de revisar CI actualiza
los cuatro registros otra vez.

No publicar secretos, PN/LID, sesiones, prompts privados ni credenciales.
No realizar reset --hard de trabajo local sin respaldarlo, ni activar
Matter-Bot antes de cumplir los criterios de Base-Matter. Trabajar
sobre alpha sin alterar beta o stable hasta aprobación y pruebas.

### Autorización de Jeriel: estado actualizado
El propietario de Matterssmith informó el 18-11-2026 que Jeriel,
creador de Aura Reed (GitHub: jerielweb), le concedió usar su clave
para la IA prevista SIN comunicar límites numéricos. Ya no está
«pendiente de respuesta». Registro privado en Vault-Matter:
docs/AI_PERMISSION_AURA_REED.md (rama matter, commit 8872f9c).
Esto es consentimiento COMUNICADO POR EL SOLICITANTE, no una
concesión técnica ya activada. Sin cuota informada no significa
sin cuotas del proveedor. Ninguna clave se ha provisionado, no hay
worker Vault de IA desplegado y no se autorizan consola, escritura,
dispositivos ni roles por este permiso. Sólo registrar usos
previstos (orientación de menús, asesoría/programación) y activar
tras verificación/cifrado/recibo de política. Nunca copiar clave.


## Regla de salida de menús y ayuda (2026-11-18)
El menú sólo lista categorías y sintaxis disponible; no ofrecer narraciones, explicación de implementación ni opciones no implementadas. `help` sin argumentos enseña sintaxis de ayuda, no el menú; `help <comando|categoria>` es el lugar de detalle. Si un comando no tiene función al invocarse sin argumentos, responde con `Uso` y sus parámetros reales. `style none` tiene idéntica semántica sin adornos. Véase `docs/CATEGORY_CONTRACT.md`.

## Contrato visual exacto de menú (noviembre 2026)
Respetar la cenefa inicial/final, encabezado global, 21 categorías cerradas, autoridad descriptiva, bloque dinámico de identidad/socket/rango/estado e instrucciones conforme a [docs/CATEGORY_CONTRACT.md](docs/CATEGORY_CONTRACT.md). Sólo un botón «Categorías»; no listar ni etiquetar categorías vacías. `style none` debe operar sin símbolos. Nunca confundir propietario verificado del socket con Owner Global de Vault, ni exponer PN/LID a terceros. No inventar rangos, número total ni estado.

## Aplicación del catálogo (alpha, noviembre 2026)
La presentación `src/menu.rs` agrupa ahora comandos únicamente en las 20 categorías permitidas; `Todo` es la entrada 21 y agrega comandos. Categorías autorizadas sin comandos visibles quedan vacías. `providers/whatsapp-rust/src/ui.rs` mantiene un solo botón «Categorías» y páginas para llegar a las 21 entradas; ninguna fila de paginación constituye una nueva categoría. Los antiguos nombres de módulos solo son rutas internas y aliases. Los directorios aún necesitan migración física; no confundir ese trabajo con añadir categorías. La aceptación final requiere CI completo y prueba real de WhatsApp.

## Regla vinculante de categorías cerradas — 2026-11-18

**Consultar antes de cambiar cualquier menú:** [docs/CATEGORY_CONTRACT.md](docs/CATEGORY_CONTRACT.md). El propietario fijó **21 entradas exactas**, en el orden de su muestra: IA, Anti, Todo, Configuración, Descargas, Economía, Diversión, Gacha, Juegos, Grupo, Información, Enlaces, Principal, Modo, Propietario, Perfil, Reacciones, Búsqueda, Conexiones, Sistema, Utilidades. **No inventar, renombrar ni añadir categorías**. Todo es una vista agregada, no un destino para comandos. Un comando existente se reubica en alguna entrada permitida; nombres heredados de módulos sólo pueden ser aliases internos. Ningún comando pendiente se declara operativo. WhatsApp conserva un único botón «Categorías». La migración del código debe completar el contrato y sus pruebas antes de promover a beta.


## UI NativeFlow de categorías (2026-11-18)

El proveedor `ui.rs` ofrece UN SOLO botón selector NativeFlow «Categorías» (single_select), con nueve categorías reales y «Todo» =/>menu all como fila. La presentación del dueño emplea `╭─💠`, `│·♪·╠[`, `│·♪·╚[` y `╰───────────⟨ 🌀`. No mostrar botones separados «Ayuda»/«Ver todo», ni inventar categorías sin ejecutor. Se admite respuesta legacy ButtonsResponseMessage de los botones fila Go, solo con IDs cerrados. Ninguna selección concede capacidades. El cliente WhatsApp real no está probado. Efectos HTML y stickers de roles quedan separados y desactivados sin autorización/datos reales.


## Prioridad UX confirmada 2026-11-18

Menú compacto, ayuda por categorías y botones al estilo SendButtonsRow; el propietario confirmó que HTML/JS y sonido se ven funcionando en su referencia externa, pero falta demostrar equivalencia en whatsapp-rust. Pantalla completa solo al mantener pulsado un botón HTML, sonidos solo por interacción. Soporte futuro de stickers de verificación de rangos cuando llegue el código; visual no concede autoridad Vault. No activar moderación por defecto.


## Referencia prioritaria del propietario — botones de fila del ejemplo Go (2026-11-18)

El propietario proporcionó código fuente `package dl` con comandos `ts18` y
`ts19`. Conservar el original en `docs/references/ts18_ts19_buttons.go.txt`.
Señal visual deseada: tarjeta con miniatura JPEG cuadrada, título y texto
descriptivo, con botones INFERIORES horizontales (Audio, MP4 y, opcional,
Enlace). La función Go que muestra esa tarjeta es `c.SendButtonsRow`.
El ejemplo no acredita soporte de HTML nativo ni del selector de países de
otra imagen. Solo sirve como referencia de interfaz; Rust sigue siendo el
lenguaje del proveedor y núcleo. No incorporar código Go ni habilitar
descargas o acciones desde botones sin verificar contratos, autorización
y cliente WhatsApp real. La observación del propietario de que los botones
actuales no funcionan y el menú antis es ilegible es un bug de UX abierto.
Ver `docs/WHATSAPP_BUTTON_CARD_REFERENCE.md`.


## Contrato de menú de Base-Matter, 2026-11-18

`src/menu.rs` contiene contratos de presentación:
nueve carpetas y un módulo por comando, sin
autoridad de ejecución. Los menús se generan
automáticamente y muestran `[pendiente]` cuando
no existe ejecutor. Estilo classic con
`╞═╦═`, `│♪║`, `│♪╠`, `│♪╚`;
estilo none sin símbolos. Idiomas es/en.
El prefijo global aparece una sola vez y no
se agrega marca personal ni créditos al menú.
Solo el dueño autenticado cambia el estilo
desde el privado; persistir en SQLite separado.

Las respuestas rápidas solo ejecutan consultas
en el puente HMAC/PN/LID, nunca modifican
activación ni roles. El HTML completo es
experimental, sin pruebas de renderizado móvil.
No borrar la plantilla antigua hasta auditar
sus consumidores, pero no usarla en el
controlador de producción. Jeriel ya comunicó autorización para la IA planeada,
sin límites específicos reportados; no hay una concesión
Vault activa ni clave importada. Ningún secreto se
transmite por WhatsApp. Ver `docs/MENU_CATALOG.md`.

## Contrato de permisos de IA concedidos por titulares — 2026-11-18

Cada clave externa pertenece al titular de la API. Su disponibilidad
en un repositorio compartido o una sesión GitHub **no autoriza su uso**.
Obtener consentimiento específico, alcance, modelos, cuotas, datos
permitidos y vencimiento. Vault-Matter conservará credenciales
cifradas únicamente y aplicará esas condiciones por código antes
de invocar modelos; además pondrá las restricciones del titular en
las instrucciones prioritarias de la IA para **cada llamada**.
No publicar credenciales ni en código, README, mensajes o logs.

Base-Matter no debe manejar claves directas ni invocar APIs con
fallos de Vault: usar solo `src/vault_ai.rs` con Unix socket y
`MATTER_VAULT_AI_UID` del broker de confianza. Rechazar mensajes
sin `policy_applied=true`, concesión válida, revisión positiva
y capacidad exacta. El broker y el verificador de licencias
no están desplegados, por tanto se falla cerrado. Los
permisos del modelo no autorizan ejecución directa, sandbox,
roles, acceso a dispositivos ni publicación de código.
No añadir créditos ni marca personal al menú de Base-Matter.

## Contrato 2026-11-18 — `matter` es programación asistida

El usuario reasignó `matter` desde su reserva anterior:
ahora es un asistente tipo Codex que pregunta y propone
planes, **no ejecuta comandos ni cambia archivos**.
Nunca volver a registrarlo como alias del sandbox o consola.
Se verificó `jbeckforddev/Aura-Bot-2.0` en master/test;
no se encontró una integración Venice/Claude en los
archivos consultados. No copiar ni publicar secretos de
ese bot. Credenciales solo del servicio privado.

Jerarquía de proveedores: Claude > Venice > OpenAI,
según clave y modelo explícitos. El protocolo y los
endpoints están en `src/matter_ai.rs`. API no equivale a
permiso Vault, rol global o escritura GitHub. Solo PN/LID
propio, contexto privado, HMAC y antirreplay autorizan
el comando; las peticiones externas fallan cerrado.
Sin prueba en cuenta real no declarar el agente listo.
Los comandos directos `console`/`eval`/`test` y las
alternativas `sandbox`/`sandbox test` son independientes.
No incluir marcas personales ni créditos automáticos
en respuestas o menús de Base-Matter.

## Invariante más reciente — ensayo RichResponse del propietario, solo self-chat

El archivo `rich_preview.rs` y el CLI `--rich-preview-*` conservan
su naturaleza offline. Como continuación separada, `rich_transport.rs`
puede empaquetar el JSON validado como `wa::AIRichResponseMessage`,
**sin** `botForwardedMessage`, `0@bot`, `forwardOrigin` ni
`isForwarded` inventados. Solo se intenta enviar si el operador fijó
`MATTER_WA_RICH_HTML=1`, el control HMAC ya fue autorizado y el
mensaje es del propietario en su chat privado propio. No habilitar
grupos, privados externos ni funciones administrativas por este modo.
Ante error de envío, fallback NativeFlow/texto. Un envío aceptado no
demuestra que el cliente renderice la hoja. No se declara soporte
Android/iOS hasta la prueba manual. Mantener todo desactivado por defecto.

## Invariante adicional — compatibilidad de Rich HTML (alpha, 2026-11-18)

Se revisaron ejemplos no confiables de Go/JS/HTML recibidos en
`xd.zip`, usados exclusivamente como referencia de datos.
Nunca copiar el origen falso Meta AI `0@bot`, marcar como
reenviado un mensaje que no lo fue ni activar scripts HTML
mediante el protocolo de WhatsApp. No incorporar juegos/medios
externos a Base-Matter sin contratos, licencias y pruebas.

El módulo `providers/whatsapp-rust/src/rich_preview.rs` construye
HTML estático y estructura JSON experimental **solo offline**.
El CLI `--rich-preview-html/json` no debe tocar sesión,
credenciales ni red. HTML en WhatsApp **no está demostrado**
y no se debe confundir con botones NativeFlow (opt-in).
Bot-Matter permanece bloqueado. READMEs 2/2 antes de commits.

## Contrato nuevo — botones NativeFlow y HTML breve (alpha, 2026-11-18)

Solicitud expresa: preparar controles interactivos reales al estilo de botones
de Telegram, conservando íntegro el menú original en **un solo mensaje**.
El proveedor Rust `whatsapp-rust` 0.7.0 ofrece envío de protobuf
`InteractiveMessage.NativeFlowMessage` y respuestas `quick_reply`;
integrar sin Node y con un único proceso dueño de la sesión.

El envío de NativeFlow es **opt-in** mediante
`MATTER_WA_NATIVE_BUTTONS=1`, desactivado por defecto hasta probar
recepción y visualización reales, con fallback texto si el envío falla.
Los tres botones iniciales son exclusivamente
`status`, `menu`, `credits`: toda respuesta de botón vuelve a
pasar por la comprobación de identidad y firma HMAC de Base-Matter.
No activar `bot on/off` ni roles mediante IDs de botón.

HTML breve de estilo debe convertirse a texto WhatsApp con etiquetas
permitidas y validación cerrada. No enviar `FOAHtmlPrimitiveDemoDONOTUSE`,
HTML/CSS/JS ejecutable ni afirmar WebView nativo. **No se puede
garantizar la distinción Android/iPhone por el evento actual.**
No usar heurísticas de JID; botones específicos por sistema deberán
esperar metadatos autenticados y pruebas en ambos clientes.

## Invariante de usuario actualizada — 2026-11-18: restaurar diseño original y alcance de status

Esta instrucción expresa del propietario prevalece sobre la sección antigua de
menú abreviado y prohibición de créditos en el **menú**. El menú completo de
`templates/matterssmith-menu.txt` conserva exactamente el encabezado ornamental,
`Matterssmith`, `Base-Matter`, las 12 líneas de comandos, el cierre con 🌀 y
`Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].` una sola vez.
No repetir ese pie en respuestas ordinarias de `status`, ajustes ni errores.
Conservar las decoraciones por chat para las otras respuestas.

`=/>status` es consulta de estado del chat actual, permitida a cualquier
usuario con identidad WhatsApp verificada, también con el bot apagado, en
grupos y privados externos. No concede poderes, roles ni acceso a otro socket.
`=/>status --global` expone políticas de socket solamente al propietario
autenticado, incluso si lo envía desde un grupo; las lecturas externas reciben
una explicación sin revelar el estado global. Mutaciones siempre reservadas al
dueño. Es obligatorio probar ambas rutas firmadas y el renderizador real,
manteniendo Bot-Matter bloqueado y antis apagados por defecto.

## Invariante vigente de interfaz — 2026-11-18

- Los créditos `Hecho por Lournyx Matterssmith [Lou-Nyx].`
  siguen en todos los códigos y documentos correspondientes,
  **nunca** como pie automático de cada respuesta WhatsApp.
  El único comando visible de atribución es `=/>credits`
  (alias `creditos` y `créditos`), sin añadir autoridad
  a lectores externos.
- El menú único presenta el prefijo global `=/>` una sola vez;
  las variantes comparten una entrada abreviada
  (`status [--global]`, `bot [ámbito] on|off|inherit`).
  El marco común no repite los títulos Matterssmith/Base-Matter.
- El tema por chat puede usar `{{TITLE}}` y `{{ACCENT}}`
  de forma opcional, pero el marco base debe continuar sin ambos
  títulos. `{{BODY}}` es obligatorio, único y acotado.
- HTML incrustado no es equivalente a HTML web normal:
  Aura usa tipos experimentales no validados para Rust/iOS.
  No enviar scripts no confiables en mensajes ni convertir
  a Node. No inferir Android/iPhone por IDs de mensajes;
  usar una señal autenticada y compatibilidad probada
  antes de seleccionar botones por dispositivo.
- Esta revisión es solo `alpha`. Repetir Rustfmt/Clippy/tests
  en Debian y probar el menú en WhatsApp antes de promover.


Read `docs/DEVELOPMENT_STATUS.md`, the root README and architecture before work.
Inspect actual code, Git status, branch, ignored files and applicable nested instructions.
Do not assume a past report is current. Preserve useful existing implementations.

### Invariante de plantilla por chat (alpha; 2026-11-18)

El registro `ControlThemes` debe compartirse entre el menú y el
IPC. El identificador del chat solo se usa para seleccionar estilo
**después** de que el puente valide HMAC, ámbito, actor y replay.
Cada resultado con texto —incluidos estado, políticas, auditoría,
ayuda y previsualizaciones— se representa mediante el renderizador
de `src/control_style.rs`. No insertar marcos Unicode directamente
en cada comando. El proveedor `whatsapp-rust` **no** aplica otra
plantilla a un mensaje ya formateado por IPC, aunque tenga tema
de grupo distinto. La ruta de salida comprueba longitud y controles
antes del reconocimiento idempotente del tema.

La plantilla de estilo por grupo puede configurarse mediante
`ControlThemes::set_community` con `ControlTheme::with_template`
validado; todavía no hay comando persistente `=/>set style`,
y no debe afirmarse lo contrario. `community` en las políticas
de activación es un tipo de chat distinto del ámbito de
personalización de temas; su encendido sigue sin aplicarse
sin metadatos atestiguados de WhatsApp.

## Contrato prioritario del propietario — 2026-11-18 (sustituye comandos anteriores)

- En WhatsApp, **solo `=/>bot on/off/inherit`** gestiona la activación. Retirar
  `=/>on/off` de proveedor, puente, ayuda y menús; el gate de power queda
  interno. `bot on` activa en una transacción la política autorizada y el
  gate operativo, sin necesitar una segunda acción; `bot off` nunca apaga
  chats ajenos. Los viejos APIs de power del núcleo son compatibilidad,
  **nunca comandos de WhatsApp**.
- `=/>bot on/off` es el chat exacto; `=/>bot --global on/off` es la política
  por defecto del socket; `=/>bot group/private/channel/community on/off`
  son políticas por tipo. Comunidad significa *grupo de avisos* y **solo**
  debe aplicarse cuando el proveedor atestigüe el tipo mediante metadatos;
  nunca asumirlo a partir de la terminación `@g.us`. Canales también
  requieren identidad de proveedor verificada. Ámbitos externos apagados
  por defecto, estado `=/>status` siempre disponible.
- Menú global **sin categorías**. Tomar el marco, símbolos y espaciado
  del ejemplo del propietario, NO importar sus categorías ni comandos `#`.
  Los diseños viven en `templates/matterssmith-control.txt` y
  `templates/matterssmith-menu.txt`, fuera de manejadores de comandos.
  Cada respuesta, incluidas las operaciones REGISTER y vistas previas
  de WhatsApp, debe pasar por el renderizador compartido de
  `src/control_style.rs` con `\n` reales. Permitir sustitución controlada
  por alcance sin modificar seguridad, rutas ni handlers.
- Roles: Vault-Matter sigue siendo autoridad exclusiva; sin lector remoto
  autenticado, no existe ninguna concesión operativa de rango local.
- Publicar cambios primero en alpha. Promover a beta únicamente revisiones
  concretas con compilación, pruebas y uso real verificados; beta→stable
  después de estabilidad repetible. Las ramas ya divergen: no hacer merges
  ciegos ni sobrescribir sus commits propios.

## Identity

Preserve the project's visual identity and existing credits. Where credits apply:

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

Do not add a separate author line or attribute project authorship to automation.
Preserve upstream copyright/license notices when importing provider code.

## Git and verification

Default development: `alpha` (experimental), `beta` (integration), `stable` (releases). GitHub is the source of truth. `stable` is the repository default. Only these THREE branches should remain after migration. Legacy references `matter`, `matter-beta`, `stable-hardening-20261117` and `stable-socket-owner-control-20261117` have no exclusive commits relative to their replacements; an authorized repository administrator may delete their refs after updating consumers and preserving tags or release references as appropriate. Never rewrite unique commits or delete session data.
Inspect all local differences first. When dirty, protect tracked and new
nonignored files with `git stash push -u`. Fetch origin, switch to the explicitly selected `alpha` or `beta` branch,
reset ONLY to the verified remote head of the explicitly selected working branch, restore the stash and inspect conflicts. Do not mix `alpha`, `beta` and legacy `matter-beta` during recovery. If fetch
or stash restoration fails, stop dependent modifications/verification, retain
recovery data and report the failure. Never use git clean or remove ignored
sessions, databases or credentials. Preserve unrelated stashes and user changes.

Run cargo fmt --all, cargo check --all-targets, cargo test and git diff --check
for implementation checkpoints. Report actual results. Do not change global
linker/toolchain configuration without demonstrated need.

The user has authorized committing and pushing verified development checkpoints
to alpha for experimental checkpoints. Promote reviewed, independently verified subsets by PR to beta, then stable. Review EVERY repository README before EACH commit, report total,
reviewed, updated, unchanged, findings, fixes and pending issues. Update affected
README/code/contracts together; preserve correct content and design. Check new
files, full diff, examples, internal links, credits and accidental secrets.
Confirm the pushed SHA against origin. The owner has authorized promoting independently verified stable subsets to
`stable`, not a bulk merge of incomplete alpha/beta. Require green CI on the exact
candidate SHA and preserve divergent stable commits (including Cargo.lock
history). Leave all unverified work in alpha.

## Scope and continuity

Finish Matter-Base before starting Matter-Bot. Continue H audit/corrections,
I integration/resilience, J security, K contracts and L release readiness.
Missing earlier prerequisites must be implemented and tested; do not call
interfaces, mocks or isolated tests a completed external integration.
Record exact next work, changes, tests and blockers in docs/DEVELOPMENT_STATUS.md
at checkpoints and before ending a session. Preserve its historical entries.

Priority is Matter-Base with the real Matter-Vault contract and global control
commands. Vault is independent and must not store platform credentials/sessions.
Do not change cryptographic formats without compatibility/migration review.
Intercept `=` and `/>` control prefixes before the normal router. Only the socket
owner can power it on/off. A global owner label is not authority over another
socket; an explicit socket ID still needs authorization. Operational power is
independent of network connection. Powered-off sockets execute no normal commands.
Policy precedence when powered on: Group > Socket > Global > None (deny).
Preserve socket isolation and persist/audit administrative changes atomically.

## Owner correction — real group operations and encrypted socket roles (2026-11-17)

- A real WhatsApp `=/>status`, `=/>on`, `=/>help` and `=/>menu`
  round-trip was observed in the self-chat after PN/LID correction.
  This proves the private owner bridge works, not group functionality.
- WhatsApp private self-chat accepts **only socket-wide configuration**:
  power, status, socket-role changes, read-only role catalog and audit.
  Menus, commands targeting groups/communities, recovery previews and
  normal bot functionality must not be handled in private.
- WhatsApp group events need a **verified actual group JID and sender**
  from the authenticated Rust connection and authenticated HMAC binding.
  Owner-only `=/>group status/on/off/inherit`, `=/>group role`,
  menus and appropriately scoped preview commands are being integrated
  in alpha; never accept a text-supplied group ID as authority.
- Only the real socket owner can change power and group policy. A global
  project owner, `co-owner` or `staff` label is **not** a bypass.
  Project-level ranks, per-bot ranks and per-socket/per-group ranks are
  separate security namespaces; do not infer one from another.
- Socket/group ranks: `owner` is the immutable validated socket owner;
  other available ranks are `co-owner`, `admin`, `moderator`,
  `operator`, `trusted`, `member`, `restricted`, `banned`.
  `user` remains implicit and never creates a role-grant entry.
  All changes must be owner-authorized, audited and persisted.
- A local SQLite role record is **not** GitHub durability. Before
  claiming Vault-Matter integration, implement verified encrypted
  role snapshot publication and recovery through an authenticated
  GitHub/Vault provider with CAS, key isolation and audit. No plaintext
  roles or secrets in Git, and no WhatsApp session files in Vault.
  Fail closed on inaccessible Vault; do not fabricate successful sync.
- Alpha is for regressions and trials; promote isolated, verified
  changes by review to beta then stable, never bulk-merge divergent
  histories. Operate from a single Git clone and shared Cargo build
  cache in the owner's one-terminal environment. Preserve sessions.

## Safety invariant: external chat surfaces default OFF

The `group`, external `private`, and `channel` kinds must remain
disabled unless the verified socket owner explicitly enables that kind
or an exact supported chat scope. A socket-wide `--global on`
**must not** make all joined groups or unknown private DMs respond
automatically. `inherit` removes an override, falling back to the
kind default; missing kind default means deny. The owner's own
private self-chat uses a separate `kind:self` scope and can
inherit the socket fallback. Never conflate external-private
with self-chat settings. New regression tests must prove default
denial even when the socket-global policy is enabled.

## Verified-public-read exception (alpha; identity boundary)

The earlier owner-only HMAC path remains exclusive for mutations.
Incoming `from_me=false` events may enter a **different**
HMAC-signed path only for `status/estado`, `menu/menú`
(with one category), and argument-free `bot`.
The signer must obtain actor and destination JIDs from the
authenticated WhatsApp event and verify the linked client
own PN/LID; never elevate an external sender to socket owner.
The controller must verify signature, exact context, nonce/
timestamp and replay, then use a **read-only** policy/menu
dispatcher. Status is available when bot-off, while menu
is silent. Any control/mutation/debug/global exposure by
this path is forbidden. No arbitrary extension callbacks.

The owner's self-chat has an independent `kind:self`
activation setting. `kind:private` applies to external
private chats only. Do not infer newsletter/channel
identity from group/private and do not claim it works until
its provider source is verified. Tests must include incoming
group and external-private traffic plus attempted privilege
escalation. `beta/stable` remain untouched.

## Owner update — activation commands and menu design (current alpha contract)

This section supersedes the older 2026-11-17 statement that menus
must be group-only and that help/h remain valid. The user now requires:
- `=/>status` as the **effective current-chat bot policy** even
  while disabled; `=/>status --global` reports actual default
  policy and socket power independently. The socket is not the bot.
- `=/>bot on/off/inherit` for the chat, bot defaults for
  `group/private/channel` in owner-private configuration,
  `=/>bot --global on/off` for the entire socket's fallback.
  Specific chat > kind > socket global > deny, with socket power
  always a separate outer gate.
- `=/>menu` in any authenticated, bot-enabled context, with
  a distinct Matterssmith Unicode theme and categorized entries.
  `=/>help` and `=/>h` are retired. When the bot is disabled,
  menu and normal commands are silent; status and the authenticated
  owner's necessary recovery/configuration actions remain available.
- `=/>set` without arguments describes configuration;
  `=/>set role` never performs local grants. Translation aliases
  must be explicitly mapped, not arbitrary text matches. The
  optional final `--debug` may expose safe technical context to
  the verified owner but never secrets, raw identities or role data.
- `--global` specifies a **policy scope**, not a role.
  `mod` is a group-moderation authorization requirement,
  `self` is socket-owner plus explicitly Vault-authorized delegates,
  and `selfmod` requires both capabilities. None are implicit
  authority; delegated execution requires a verified Vault reader.
- `=/>set style/theme`, explicitly approved `#menu`
  visual adaptation and AI-backed `--auto` are future
  capabilities. Do not copy commands, users, source
  identities, third-party links, tokens or permissions from
  another bot's message; preview before applying.
- `menu/status` for external actors and WhatsApp channels requires
  a new signed ingress model. Never simply remove the
  `from_me` check to make unauthenticated messages work.
- Develop/test on alpha only; do not promote incomplete
  features to beta or stable. Full command semantics and
  limitations: `docs/BOT_POLICY_V2.md`.

## New authoritative policy: Vault-Matter owns ALL socket/group role state

- Vault-Matter is the **sole authoritative role manager** for socket
  and group roles. Base-Matter has **read-only** decision/query access
  to Vault and no GitHub Vault write credentials or role mutation API.
- A role only takes effect after fresh authenticated Vault confirmation;
  no record, unreadable Vault, bad scope/actor, expired proof or
  revoked revision => deny. No local default grants or offline copy.
- Vault-Matter alone may approve commands from its own authenticated
  WhatsApp/Telegram/Discord gateways, write encrypted role state and
  audit records, and recover from its private repository. A forwarded
  message or Base-generated relay is not itself verified user consent.
- Base's original authenticated socket owner can power down the socket
  without delegated role checks; project staff/owners never inherit
  socket rank permissions implicitly.
- `user` is an implicit absence of a grant; every explicit non-default
  rank is Vault-owned. Local `RoleStore` and encrypted outbox remain
  test utilities, never installed as a live second role authority.
- Alpha WhatsApp controller MUST NOT install local socket role stores
  or allow `set role` to mutate SQLite pending Vault service.
  Keep `=/>roles` honest about missing authority.
- Build a separate Vault Rust service and read-only verified Base
  interface, not a plaintext roles file or GitHub token in the Base
  process. See `docs/VAULT_ROLE_AUTHORITY.md`.
- Continue alpha -> verified beta -> independently validated stable,
  without declaring a working integration from schema-only metadata.

## Current hard constraints — owner clarification 2026-11-17

- Every \`*-Matter\` repository is part of the Matter ecosystem; Bot-Matter
  will consume its components in a later, separate stage.
- DO NOT implement, refactor or launch Bot-Matter until Base-Matter is complete.
  Freeze separate platform prototypes until core readiness, unless the owner
  explicitly reauthorizes them.
- Historical WhatsApp pairing-code connection/ping tests were undertaken
  before this beta version. Do not erase that history. However, do not claim
  the current remote Base repository contains a verified live WhatsApp
  connection without actual source and an end-to-end test.
- WhatsApp linking MUST use pairing code only. QR must not be exposed,
  advertised or used as fallback. Never overwrite existing local sessions.
- The former ping is a test only; it is NOT a permanent global command.
  Global control is implemented by Base-Matter using canonical \`=/>\`,
  independent from the bot's own command prefix and even when the bot is off.
  Legacy \`=\` and \`/>\` aliases are retained until an intentional migration.
- Preserve every ecosystem repository's history and respect the fact that
  provider modules are ultimately consumed by Bot-Matter, not developed as
  an excuse to start Bot-Matter early.

## Lenguajes y WhatsApp — corrección expresa del propietario

- Lenguajes principales: Python, C++ y Rust. Mantener Rust como núcleo y proveedor de WhatsApp ya trabajado. C y Go son secundarios; JavaScript/Node.js es terciario y NO sustituye ni dirige la implementación `whatsapp-rust`.
- Recuperar el código histórico `src/providers/whatsapp/mod.rs` del commit `3c36d97bb` para continuidad; conservar licencias, datos de sesiones y progreso existente.
- Prioridad actual: completar comandos globales, seguridad, verificación y componentes pendientes de Base-Matter sobre Rust. No seguir desarrollando la sonda Baileys/Node ni Bot-Matter.
- La vinculación WhatsApp es exclusivamente por pairing code; el `ping` anterior era una prueba y se retira de comandos durables.
- Atribuir comandos/archivos con el crédito exacto: `Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].` (en comentarios o notas discretas), sin línea duplicada de Autor.
- El estado `powered` de un socket NO implica que esté habilitada su política de ejecución. Las pruebas deben mantener esa separación.

## Updated ecosystem direction (user instruction)

- Target name WhatsApp-Matter replaces Matter-Baileys.
- Target name Telegram-Matter replaces Matter-Telegraf.
- All other repositories follow <Component>-Matter, with Base-Matter and
  Vault-Matter first. Preserve Matterssmith credits and stable branches.
- Inspect and rename the actual GitHub repositories when ecosystem work begins;
  documenting these targets does not mean remote repositories were renamed.
- Each platform repository may contain multiple provider methods in separate
  directories, e.g. providers/baileys and providers/telegraf. Verify the official
  upstream source, pin its revision, preserve history/licenses/notices and keep
  local modifications traceable. Do not clone unverified lookalike repositories.
- Adapters implement universal Matter-Base contracts. Platform libraries and
  provider-specific changes remain in their platform repository, outside Base.
- Full ecosystem management does not imply bypassing a platform's permissions.
- Real WhatsApp verification uses a secondary test account and a single process
  owning its connection; never overwrite an existing session. Then verify
  equivalent Telegram and Discord functionality where supported.
- This scope does not relax Matter-Base readiness criteria. External service
  availability/account linking must be reported honestly when blocked.

## Proveedor Rust verificable

El proveedor de WhatsApp seleccionado es whatsapp-rust, ejecutado desde providers/whatsapp-rust con biblioteca fijada, sesión SQLite privada y pairing code. No utilizar Node/Baileys en sustitución. No afirmar estabilidad hasta comprobar compilación y una sesión real autorizada. Revisar su README y el README raíz antes de cada commit.

## Estado de ramas (instrucción del propietario 2026-11-17)

- Nuevas ramas canónicas: `stable` (release; creada desde `matter`), `beta` (integración; creada desde `matter-beta`) y `alpha` (experimental; creada desde `matter-beta`).
- `matter` y `matter-beta` se conservan temporalmente sin borrar ni reescribir su historial; el predeterminado sigue siendo `matter` hasta cambio separado en GitHub.
- Flujo: cambios en `alpha` -> CI -> revisión por PR a `beta` -> comprobación real externa -> PR a `stable`. No hacer merge global de ramas divergentes. Ver `docs/BRANCH_STRATEGY.md`.
- En cada commit leer los DOS README (raíz y `providers/whatsapp-rust/README.md`), actualizar los afectados y reportar revisión, resultados y problemas.

## Comunidad oficial REGISTER y =/>start

- La orden canónica `=/>start` pertenece a Base-Matter. El controlador autentica el socket y su propietario; el adaptador `whatsapp-rust` verifica el contexto privado del dueño y consulta las comunidades de la sesión autorizada.
- Fase primera (segura): inventario de comunidades, prueba de rol admin del dueño y búsqueda de subgrupos oficiales mediante el encabezado EXACTO `MATTER:OFFICIAL:register:<community-id>` situado en la primera línea de la descripción. El título por sí solo no identifica un grupo oficial.
- La relación comunidad/subgrupo debe venir del proveedor autenticado; un texto o ID de WhatsApp enviado por terceros jamás confiere autoridad. Resolver PN/LID sin adivinar identidades.
- Fase de plan: comparar identidad oficial, nombre, descripción, foto, permisos y enlaces; preparar acciones idempotentes. No crear grupos ni cambiar administradores por la mera ejecución de `start`.
- Fase de aplicación separada: verificar admin de comunidad Y admin del grupo, consentimiento/autorización para cada mutación, aprobación específica del plan vigente, revisión de conflictos, auditoría transaccional, límites de tasa y recuperación. Permitir crear el grupo REGISTER si no existe; vincularlo a la comunidad con autorización explícita y sin duplicados.
- Las DOS cuentas de confianza se deben configurar mediante identidad validada y participación aprobada; nunca inventar números, agregar ni promover usuarios sin verificar consentimientos y permisos de WhatsApp.
- Los administradores existentes solo recibirán una solicitud moderada de enlaces de invitación, evitando menciones masivas, y solamente si la política del grupo lo autoriza. Esos enlaces se guardan cifrados, con caducidad, revocación y uso explícito para búsqueda de asistencia Matter. No publicar enlaces privados sin consentimiento.
- Menú único y diseño comunitario: separar componentes visuales reutilizables (encabezado, cuerpo y pie corto sin atribución automática) de la lógica de autorización. Configurar tema por comunidad; mantener el crédito Matterssmith en código y en `=/>credits` exclusivamente.
- NO anunciar que hay un escaneo o mutación real si el proveedor no confirma su resultado. Conservar pruebas y diferencias respecto a la biblioteca `whatsapp-rust`.

## Interoperabilidad con otros bots: #matter

- `#matter` es comando voluntario de ENTRADA de bots ajenos; NO sustituye `=/>`, que sigue como control canónico en Base-Matter. Los bots externos deben integrarse por contrato versionado de adaptador/IPC autenticado y permisos de mínimo privilegio.
- Identidad de actor, socket, bot externo, canal, alcance, expiración y nonce deben venir de enlace autentificado. No confiar en nombres, números o texto como prueba de identidad, ni permitir acceso a sockets ajenos.
- Sin compartir tokens, claves maestras ni archivos de sesión por defecto. Si un proveedor incompatible requiere migración, crear un EXPORT LOCAL ENCRIPTADO, voluntario y autorizado por dueño, con bloqueo de proceso y controles de origen/destino; jamás duplicar sesiones activas o publicar credenciales en Git/Vault.
- Solo mapear capacidades compatibles; Telegram, Discord y plataformas externas pueden carecer de algunas operaciones y deben reportarlo.

## Menú universal y copia de estética (instrucción correctiva 2026-11-17)

- `=/>menu`, `=/>help` y `=/>h` muestran exclusivamente los comandos
  globales reales en texto plano, sin estética inventada por Base-Matter.
- La intención es leer una respuesta real del comando `menu` de OTRO
  bot, con fuente identificada y permiso del propietario, y extraer
  SOLAMENTE su diseño decorativo. No copiar su lista de comandos,
  identidad, credenciales, enlaces, base de datos, sesiones ni lógica.
- No afirmar que Base observa otros bots sin una sesión autorizada,
  fuente real y control de mensajes. Los estilos importados deben
  validarse y aislarse por comunidad antes de activarse.
- `#matter` queda como interfaz opcional para integraciones externas
  autenticadas; no sustituye el prefijo `=/>` ni copia sesiones activas.

## Recuperación comunitaria =/>add++ (solicitud 2026-11-17)

- Objetivo: anticipar pérdida de una comunidad, preservando un inventario privado de sus integrantes y la posibilidad de ofrecerles entrar voluntariamente en una comunidad reemplazo.
- La sintaxis exacta es `=/>add++` (dos signos más). Requerir autorización global del propietario del socket y verificación admin en el origen y destino. El bot normal apagado no modifica ese control.
- Separar la lectura de miembros, instantánea cifrada y plan de invitación de las mutaciones. Sin consentimiento, límites de WhatsApp o permisos no añadir de golpe a terceros. Evitar reintentos intensivos y suspender ante señales de restricciones.
- Poner en cuarentena `apply` hasta tener verificación de consentimiento, plan revisado, idempotencia y pruebas externas. Las instantáneas cifradas no son sesiones WhatsApp y no se publican en Git.
- El grupo de emergencia puede llevar el nombre de la comunidad, acompañado de `Respaldo`, para evitar suplantar el original. Si la comunidad fue suspendida antes del respaldo y no hay acceso, no afirmar recuperación posible.

## Gramática de =/>add (petición 2026-11-17)

- Las órdenes universales nuevas son `=/>add user <número|@usuario|reply> [group|community <id>]`, `=/>add group [--hide] community <id>`, `=/>add community <id>`, `=/>add community new --Nombre`, `=/>add community backup <origen> to <destino> [--hide]`.
- `<id>` admite `official` y, para el subgrupo oficial configurado, `official --register`. La resolución debe emplear IDs realmente autenticados, nunca coincidencias de título.
- El bot solo debe planificar y reportar, NO añadir usuarios sin su consentimiento ni eludir ajustes de privacidad, cuotas o suspensiones. La promoción de un grupo a otra comunidad exige autorización admin, verificación del chat origen, destino y capacidad API. `--hide` es una preferencia, no una técnica para ocultar usuarios.
- Un mensaje citado se resuelve desde metadata confiable. Un alias @usuario solo se acepta si el proveedor da una resolución de identidad comprobable. Los comandos de grupo aún no cuentan con admisión de mensajes grupales: no declarar mutaciones completas.
- `=/>add++` sigue siendo diferente: inventario y respaldo cifrado preventivo de participantes; no es autorización para incorporaciones masivas.
