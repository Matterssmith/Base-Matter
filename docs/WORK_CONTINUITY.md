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

# Checkpoint 2026-11-18 — corrección exacta de test Grupo

- SHA base: ce9465a8cffaa1c7b17dc7596f8798a7fbfb83a8.
- Commit: "fix(alpha): correct positive and negative group menu tests".
- Próxima tarea: comprobar GitHub Actions Linux/Windows y
  provider offline para nuevo SHA; obtener logs en caso de fallo.
- Incidencia: test de provider rechazaba incorrectamente
  "menu unknown" en el conjunto autorizado (y viceversa).
- Estado de funcionalidad: menu HTML/audio/TS18/TS19 ya en
  alpha; falta confirmación de representación en WhatsApp real.
- Flags de ensayo: MATTER_WA_NATIVE_BUTTONS=1,
  MATTER_WA_MENU_LAB=1, MATTER_WA_RICH_HTML=1.
- Ejecución: cargo run --locked desde /home/Sakura/Base-Matter;
  antes git fetch y pull --ff-only en alpha.
- Seguridad: sólo self-chat de dueño para lab. Ninguna clave.
- README revisados: 2/2. beta/stable intactas.

# Punto vigente de recuperación — Base-Matter alpha (18-11-2026)

- SHA base: 97de8ac040148925baa194710a0d5dab82d90aff.
- Commit esperado: "fix(alpha): restore continuity checkpoint and valid readonly menu tests".
- Archivos: providers/whatsapp-rust/tests/provider.rs y los
  cuatro archivos obligatorios de seguimiento (AGENTS, DECISION_LOG,
  WORK_CONTINUITY, DEVELOPMENT_STATUS), ambos README.
- Corrección: prueba de rechazo del signer reemplaza "menu group"
  por "menu unknown". Grupo es categoría legítima.
- Incidencia de proceso: el commit anterior no actualizó este
  checkpoint; el CI lo detectó. Retomar la actualización estricta
  de los cuatro archivos en cada commit.
- CI SHA 97de8ac: proveedor FAIL en test de categoría Grupo,
  Rust Linux FAIL por checker de continuidad; Windows en ejecución.
- CI nuevo: pendiente. No afirmar que la interfaz se vio en móvil.
- Funcionalidad actual para probar: =/>menu en self-chat,
  21 entradas y un selector nativo; con flags MENU_LAB=1 y
  RICH_HTML=1 se prueban tarjetas TS18, TS19, RichResponse y
  audio Ogg/Opus 0.45s. El sonido requiere pulsar reproducir.
- Siguiente: verificar GitHub Actions Linux/Windows y proveedor,
  corregir fallas de pruebas o compilación, registrar un nuevo
  checkpoint en cada commit. Tras verde, iniciar WhatsApp Rust
  con pairing code o sesión existente y probar =/>menu.
- README revisados: 2/2. Rama beta/stable intacta.

# Checkpoint 2026-11-18 — 21 categorías y pruebas de transporte

- SHA base: 954aea37c2c92bc944ceb309780eaa9b1581a87d.
- Commit siguiente: "test(alpha): unlock 21 readonly menu categories and TS18 TS19 regression".
- Archivos: provider src/lib.rs, tests/provider.rs, tests/ui.rs,
  tests/rich_preview.rs, tests/rich_transport.rs, tests/whatsapp_public_readonly.rs.
- Seguridad: sin modificación de HMAC ni roles; sólo permite
  menu páginas 1–3 y las 21 categorías en lectura autenticada.
- Estado CI: pendiente de Linux, Windows y proveedor en el nuevo SHA.
- Siguiente: actualizar documentación de prueba de menú,
  comprobar CI y corregir fallos; luego probar WhatsApp real.
- README revisados: 2/2. beta/stable sin cambios.

# Checkpoint 2026-11-18 — Rustfmt de laboratorio

- SHA base: 05ec05c5bba5bb33b63dcb9aec457e2f8fcedda4.
- Commit: "fix(alpha): rustfmt TS18 TS19 HTML audio menu lab".
- Modificado: lib/main/ui/rich_preview del proveedor, pruebas de
  proveedor/UI/lector público, instrucciones y los README.
- Resultado de CI anterior: fallo de formato (sin ejecutar tests).
- Resultado nuevo: pendiente de GitHub Actions Rust Linux/Windows
  y proveedor independiente.
- Próximo: leer logs CI del nuevo commit, corregir errores de
  compilación o pruebas, actualizar registros por commit, probar
  menú con sonido y botones en WhatsApp real.
- README revisados: 2/2. beta/stable sin cambios.


## Checkpoint 2026-11-18 — experimento TS18/TS19 + HTML + audio
- SHA anterior: 206d50707ec556880fe4e65b912e40245bef722a.
- Rama: alpha. Commit: buscar mensaje "feat(alpha): start menu html audio TS18 TS19 lab".
- Cambio: fuente Ogg/Opus original, envío opt-in en self-chat
  de HTML experimental, tarjetas ts18/ts19, audio pulsable y
  menú normal primero. Flags MATTER_WA_NATIVE_BUTTONS=1,
  MATTER_WA_MENU_LAB=1, MATTER_WA_RICH_HTML=1.
- Estado CI de este cambio: pendiente. Bot-Matter bloqueado.
- Siguiente: corregir allowlist de las 21 categorías y navegación
  en lectores, añadir pruebas UI, revisar Rustfmt, CI y teléfono.
- README revisados: 2/2. beta/stable intactos.


## Punto de recuperación (18 nov 2026 México / 19 nov UTC)

- **Repositorio**: Matterssmith/Base-Matter.
- **Rama de trabajo autorizada**: alpha. No tocar beta/stable.
- **HEAD anterior a este commit**: d6ff4cb2ac9493f0a94a3354ce9a25b65c331787.
- **Nuevo commit**: identificar HEAD de alpha cuyo mensaje sea
  "fix(alpha): preserve decisions and recoverable checkpoints, record Aura consent".
  Consultar SHA real a través de GitHub después de publicarlo.
- **README revisados: 2/2** (README.md y providers/whatsapp-rust/README.md).
- **Cambios de esta revisión**: instrucciones permanentes, historial,
  checkpoint y desarrollo, expediente IA, ambos README,
  test menu/help, test de directorios temporales Windows,
  script y workflow de continuidad.
- **Estado de pruebas de la versión anterior d6ff4cb**:
  * Proveedor whatsapp-rust offline: SUCCESS.
  * Linux: FAILED en tests/menu_catalog.rs: la prueba esperaba
    "📖 Ayuda: Anti" en menu antis 2. El nuevo contrato exige
    que tal encabezado sólo aparezca en help antis 2.
  * Windows: FAILED en tests/command_audit.rs, error Os 183
    AlreadyExists por ruta temporal PID + timestamp.
  * Las demás suites no deben declararse completas por esos fallos.
- **Estado de pruebas del nuevo commit**: PENDIENTE de CI; verificar
  GitHub Actions Linux/Windows, proveedor, SHA y logs antes de
  informar éxito. La validación en teléfono WhatsApp sigue PENDIENTE.
- **Contrato de menú**: las 21 entradas exactas en
  docs/CATEGORY_CONTRACT.md, un botón «Categorías».
  menu: atributos/opciones disponibles, sin detalles.
  help (sin args): sólo sintaxis de ayuda; help con args:
  descripción/uso. No anunciar categorías vacías.
  style classic / none deben conservarse.
- **Permiso IA**: usuario comunica autorización de Jeriel para usar
  su clave de Aura Reed; sin límites específicos comunicados.
  Vault-Matter commit 8872f9c registra el permiso como declaración,
  no como licencia operativamente activa. No hay API key ni
  broker Vault de IA; no habilitar ejecución ni llamadas.
- **Bloqueos reales**: Vault runtime, grant autenticado,
  credencial cifrada/dedicada, pruebas reales de WhatsApp,
  clasificación física de módulos históricos, resto de Etapas H–L.
  Matter-Bot BLOQUEADO.

### Instrucciones concretas para la próxima sesión
1. Consultar AGENTS.md, este checkpoint y docs/DECISION_LOG.md.
2. Leer docs/CATEGORY_CONTRACT.md, docs/MATTER_AI.md y
   docs/DEVELOPMENT_STATUS.md, más ambos README.
3. Consultar rama alpha y SHA exacto de GitHub; inspeccionar
   los workflows Rust Linux/Windows y whatsapp-rust.
4. Si falla CI, abrir logs, arreglar la causa y hacer un nuevo
   commit **actualizando de nuevo los cuatro registros**.
5. Si CI está verde, solicitar prueba real móvil en chat propio:
   =/>menu, =/>menu anti, =/>menu all, =/>help,
   =/>help antilink, =/>menu config, set style none.
   El selector requiere MATTER_WA_NATIVE_BUTTONS=1.
6. Revisar Vault-Matter (rama matter) antes de añadir funcionalidad
   IA; no traer claves desde repositorios de terceros ni chats.
7. Mantener créditos, contrato HMAC/antirreplay/PN/LID, bot OFF
   externo por defecto, Vault autoridad única y ramas separadas.
8. Registrar resultado exacto en el siguiente commit; nunca
   proclamar compatibilidad del teléfono por CI offline.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## Checkpoint 2026-11-18 — experimento TS18/TS19 + HTML + audio
- SHA anterior: 206d50707ec556880fe4e65b912e40245bef722a.
- Rama: alpha. Commit: buscar mensaje "feat(alpha): start menu html audio TS18 TS19 lab".
- Cambio: fuente Ogg/Opus original, envío opt-in en self-chat
  de HTML experimental, tarjetas ts18/ts19, audio pulsable y
  menú normal primero. Flags MATTER_WA_NATIVE_BUTTONS=1,
  MATTER_WA_MENU_LAB=1, MATTER_WA_RICH_HTML=1.
- Estado CI de este cambio: pendiente. Bot-Matter bloqueado.
- Siguiente: corregir allowlist de las 21 categorías y navegación
  en lectores, añadir pruebas UI, revisar Rustfmt, CI y teléfono.
- README revisados: 2/2. beta/stable intactos.


## Punto de recuperación (18 nov 2026 México / 19 nov UTC)

- **Repositorio**: Matterssmith/Base-Matter.
- **Rama de trabajo autorizada**: alpha. No tocar beta/stable.
- **HEAD anterior a este commit**: d6ff4cb2ac9493f0a94a3354ce9a25b65c331787.
- **Nuevo commit**: identificar HEAD de alpha cuyo mensaje sea
  "fix(alpha): preserve decisions and recoverable checkpoints, record Aura consent".
  Consultar SHA real a través de GitHub después de publicarlo.
- **README revisados: 2/2** (README.md y providers/whatsapp-rust/README.md).
- **Cambios de esta revisión**: instrucciones permanentes, historial,
  checkpoint y desarrollo, expediente IA, ambos README,
  test menu/help, test de directorios temporales Windows,
  script y workflow de continuidad.
- **Estado de pruebas de la versión anterior d6ff4cb**:
  * Proveedor whatsapp-rust offline: SUCCESS.
  * Linux: FAILED en tests/menu_catalog.rs: la prueba esperaba
    "📖 Ayuda: Anti" en menu antis 2. El nuevo contrato exige
    que tal encabezado sólo aparezca en help antis 2.
  * Windows: FAILED en tests/command_audit.rs, error Os 183
    AlreadyExists por ruta temporal PID + timestamp.
  * Las demás suites no deben declararse completas por esos fallos.
- **Estado de pruebas del nuevo commit**: PENDIENTE de CI; verificar
  GitHub Actions Linux/Windows, proveedor, SHA y logs antes de
  informar éxito. La validación en teléfono WhatsApp sigue PENDIENTE.
- **Contrato de menú**: las 21 entradas exactas en
  docs/CATEGORY_CONTRACT.md, un botón «Categorías».
  menu: atributos/opciones disponibles, sin detalles.
  help (sin args): sólo sintaxis de ayuda; help con args:
  descripción/uso. No anunciar categorías vacías.
  style classic / none deben conservarse.
- **Permiso IA**: usuario comunica autorización de Jeriel para usar
  su clave de Aura Reed; sin límites específicos comunicados.
  Vault-Matter commit 8872f9c registra el permiso como declaración,
  no como licencia operativamente activa. No hay API key ni
  broker Vault de IA; no habilitar ejecución ni llamadas.
- **Bloqueos reales**: Vault runtime, grant autenticado,
  credencial cifrada/dedicada, pruebas reales de WhatsApp,
  clasificación física de módulos históricos, resto de Etapas H–L.
  Matter-Bot BLOQUEADO.

### Instrucciones concretas para la próxima sesión
1. Consultar AGENTS.md, este checkpoint y docs/DECISION_LOG.md.
2. Leer docs/CATEGORY_CONTRACT.md, docs/MATTER_AI.md y
   docs/DEVELOPMENT_STATUS.md, más ambos README.
3. Consultar rama alpha y SHA exacto de GitHub; inspeccionar
   los workflows Rust Linux/Windows y whatsapp-rust.
4. Si falla CI, abrir logs, arreglar la causa y hacer un nuevo
   commit **actualizando de nuevo los cuatro registros**.
5. Si CI está verde, solicitar prueba real móvil en chat propio:
   =/>menu, =/>menu anti, =/>menu all, =/>help,
   =/>help antilink, =/>menu config, set style none.
   El selector requiere MATTER_WA_NATIVE_BUTTONS=1.
6. Revisar Vault-Matter (rama matter) antes de añadir funcionalidad
   IA; no traer claves desde repositorios de terceros ni chats.
7. Mantener créditos, contrato HMAC/antirreplay/PN/LID, bot OFF
   externo por defecto, Vault autoridad única y ramas separadas.
8. Registrar resultado exacto en el siguiente commit; nunca
   proclamar compatibilidad del teléfono por CI offline.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
