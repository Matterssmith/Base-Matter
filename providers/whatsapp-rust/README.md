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

### 2026-11-18 — Prueba opcional HTML, sonido y botones en menú
En alpha, el primer =/>menu sigue mostrando 21 categorías y el
selector Categorías. MATTER_WA_MENU_LAB=1 añade sólo en self-chat
autenticado tarjetas TS18 y TS19, tono original en audio
cifrado Ogg/Opus y HTML RichResponse cuando
MATTER_WA_RICH_HTML=1. El audio requiere pulsar reproducir;
HTML no ejecuta JavaScript y puede no aparecer en WhatsApp.
README revisados: 2/2. CI y teléfono pendientes.

### 2026-11-18 — Continuidad obligatoria y permiso Aura Reed comunicado

El usuario exige registrar cada idea y cambio en instrucciones, un
historial de decisiones y un checkpoint exacto para reiniciar tras
desconexiones. [AGENTS.md](../../AGENTS.md) es la norma vinculante;
[continuidad](../../docs/WORK_CONTINUITY.md) y
[decisiones](../../docs/DECISION_LOG.md) se actualizan en cada commit.
El workflow Rust lo exige. La documentación de IA
[docs/MATTER_AI.md](../../docs/MATTER_AI.md) remite al permiso comunicado
por Jeriel, registrado en Vault-Matter (sin límites específicos
comunicados, pero sin clave o broker activo). Se corrigen una
aserción de menu/help y una colisión temporal Windows.
**README revisados: 2/2**. CI de este nuevo SHA pendiente.

### 2026-11-18 — Rustfmt y ayuda sin categorías
Se aplicó formato Rustfmt en la ayuda por página, la disponibilidad de comandos, las variantes del menú y las pruebas. `=/>help` sin argumentos muestra exclusivamente formas de ayuda, sin copiar el menú. Ambos README revisados 2/2; se requiere nueva verificación.

### 2026-11-18 — Menú de atributos separado de ayuda contextual
Las páginas de `=/>menu` sólo enumeran variantes reales, sin descripciones ni pendientes; `=/>menu all` es un índice compacto. `=/>help` sin argumentos indica la sintaxis de ayuda y no copia categorías. `=/>help <categoria|comando>` mantiene los detalles. Invocaciones sin una operación de cero argumentos asignada, como `=/>set` y `=/>bot`, muestran uso. Ambos estilos (`classic` y `none`) siguen disponibles. READMEs revisados 2/2. Validar Linux, Windows y proveedor antes de usar en WhatsApp.

### 2026-11-18 — Compilación Windows: nombre local del rótulo de categoría
El compilador de Windows detectó que la variable local label ocultaba la función label del renderizador. Se renombró el local a decorated_label sin variar presentación ni permisos. Ambos README revisados; se requieren verificaciones completas del nuevo commit.

### 2026-11-18 — Formato final del menú sin símbolos
El CI detectó una única diferencia de Rustfmt en el nombre de Todo del estilo none; se aplicó literalmente sin cambiar comportamiento. Se mantienen 21 rutas, sección de autoridad no otorgante y botón de categorías. README principal y proveedor revisados 2/2; revalidación por SHA.

### 2026-11-18 — Rustfmt de menú autenticado y texto sin diseño
CI detectó formato en control_ipc.rs, control_style.rs, src/menu.rs y tests/menu_catalog.rs. Se aplicó el formato exacto. `style none` ahora enumera nombres simples de las 21 entradas, sin glifos de la lista clásica. El formato visual clásico, la autoridad por socket y los controles de seguridad no cambiaron. READMEs revisados 2/2; verificar nuevo SHA.

### 2026-11-18 — Corrección de saltos y delimitadores del menú completo
Se corrigió la serialización UTF-8 para que el marco use saltos de línea reales, no secuencias escapadas, y se agregó la cenefa completa al detector de respuestas ya decoradas. El test de límite NativeFlow se ajustó a 3900 bytes; nueva prueba valida formato clásico, identidad verificada, variante sin diseño, ausencia de rótulos vacíos y separación de la sección de autoridad para lectores externos. Revisados ambos README; CI del SHA anterior detectará/confirmará diferencias adicionales.

### 2026-11-18 — Formato completo de menú global y variante sin diseño
Se adoptó la cenefa, encabezado, 21 filas inmutables, sección descriptiva de autoridad, campos dinámicos de identidad verificada, comandos, rango, estado y socket, instrucciones y cierre. La ruta de IPC sólo rellena datos después de HMAC/antirreplay/admisión, no deriva Owner Global de un número. Se suprimió «Sin comandos registrados» y el menú all omite encabezados sin comandos. `style none` sigue mostrando categorías y datos. El selector conserva un único botón; se aumentó su límite de texto a 3800 bytes sin alterar límites de transporte ni permisos. Revisados ambos README y AGENTS; la comprobación CI y teléfono real siguen siendo necesarias.

### 2026-11-18 — Formato de prueba de catálogo cerrado
CI del SHA 653b6ec marcó únicamente formato de la prueba Rust del catálogo; se aplicaron exactamente los saltos de línea solicitados por Rustfmt. La aserción de IDs compara explícitamente slices en orden. README principal y proveedor revisados (2/2); se vuelve a validar. No hay cambios de transporte, sesión, Vault ni permisos.

### 2026-11-18 — Invariante de 21 entradas y comandos únicos
Se añadió una prueba que compara el orden exacto de las 21 filas con el contrato cerrado, exige 20 destinos de comandos distintos de Todo y evita duplicados. El comando general help se ubica en Principal, no IA; matter permanece en IA. Los módulos internos siguen conservando rutas históricas sin exponerse como categorías. READMEs revisados 2/2; validar CI del nuevo SHA.

### 2026-11-18 — Catálogo de 21 entradas aplicado a la presentación Rust
Se reorganizaron los descriptores existentes dentro de IA, Anti, Configuración, Información, Principal, Modo, Propietario, Sistema y Utilidades. Otras entradas autorizadas no contienen comandos inventados. Todo sigue siendo una vista agregada. Aliases anteriores permanecen sólo como compatibilidad. Un único selector «Categorías», paginado en tres pantallas por límites del cliente, mantiene 21 entradas accesibles. Rustfmt corrigió las diferencias que reportó CI del SHA 457de96; se vuelve a validar Linux, Windows y el proveedor. Ver docs/CATEGORY_CONTRACT.md.

### 2026-11-18 — Catálogo de categorías cerrado por instrucción del propietario
El contrato [docs/CATEGORY_CONTRACT.md](../../docs/CATEGORY_CONTRACT.md) registra las 21 entradas exactas del menú original y prohíbe categorías inventadas. Todo = vista agregada. Establece destino de cada comando histórico, aliases de compatibilidad y un único botón «Categorías». La documentación **no equivale a migración completa del código**; pendiente adecuar el renderizador y las pruebas.

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

### Botón selector + Ayuda + Ver todo, y lectura de respuestas Go-style

Se reemplazaron los tres quick replies fijos por un `single_select` con nueve filas para categorías y dos botones de consulta. El parser reconoce tanto `NativeFlowResponseMessage` como `ButtonsResponseMessage` cuando los IDs pertenecen a la lista cerrada. Sin comandos administrativos, envíos espontáneos ni descarga de medios. El cliente real Android/iOS no ha sido validado. README revisado.

### Formato del puente para help

Se aplicó el bloque Rustfmt solicitado por CI a las ramas de ayuda pública. Sin cambios de comportamiento, permisos, claves ni sesión. README revisado.

### Corrección HMAC para navegación

La allowlist del proveedor incorpora `help/h/ayuda` en contextos verificados, y permite `menu all` públicamente si el chat está activo. Mantiene antirreplay y restricciones de propietario. README revisado.

### Actualización UX en alpha

El puente reconoce `help` además del menú, siempre con HMAC y permisos previos. Los botones actuales aún no replican el ejemplo Go. Compatibilidad HTML, audio y pantalla completa pendiente de prueba real; no se han alterado las sesiones.

### 2026-11-18 — Ejemplo original de tarjeta con botones de fila (sin integración)

Se conserva en el repositorio el código Go suministrado por el propietario
para `ts18`/`ts19`, incluida la llamada `SendButtonsRow` que presenta
miniatura, texto y botones inferiores Audio/MP4/Enlace. Es una referencia
visual; **no** se añadió un motor Go, descarga multimedia ni endpoints externos.
Los botones de menú en Rust actuales son un prototipo distinto y la prueba
del teléfono no demostró aún la interacción deseada.
Ver [`docs/WHATSAPP_BUTTON_CARD_REFERENCE.md`](../../docs/WHATSAPP_BUTTON_CARD_REFERENCE.md).

### CI final del menú por categorías

El commit `dcf39b2` pasó las verificaciones del
núcleo en Linux. El CI del proveedor señaló una
única diferencia Rustfmt para el match de
categorías en `src/lib.rs`; corregida exactamente.
La lista sigue cerrada a nueve categorías
registradas; no permite comandos administrativos
a terceros ni páginas arbitrarias. Sin cambios
de sesión o credenciales. Este README fue revisado
antes de publicar la corrección, pendiente de
nueva ejecución del CI y prueba en teléfono.

### CI de submenús públicos (alpha 2026-11-18)

La prueba offline de `578c680` detectó que el proveedor
firmaba submenús genéricos como `menu group`, antes
explícitamente prohibidos. Se sustituyó el comodín por
una lista exacta de categorías: status, bot, set, antis,
roles, system, sandbox, ai y navigation. Solo se
aceptan páginas 1..8; no se autoriza ninguna
mutación ni categoría inventada. Los casos positivos
de menú incluyen set, antis, antis 2 y system.
La lectura pública depende de la activación del chat;
las operaciones de propietario usan otra firma.
README del proveedor actualizado, sin credenciales.

### Verificación de código modular / independiente

La compilación de `2943cedd` llegó a Clippy
en el núcleo y detectó únicamente la regla
`module_inception` de nombres internos; se
mantiene un archivo por comando sin suprimir
advertencias. El proveedor standalone marcó
dos diferencias Rustfmt en
`provider_control_style.rs`; ambas corregidas.
Se conserva el decorador de respaldo y la
separación del menú (no reimportar
`crate::menu` a la biblioteca autónoma).
README actualizado; nueva compilación pendiente.

### Decoración independiente del proveedor (CI 2026-11-18)

La importación anterior de `../../../src/control_style.rs`
se reemplaza por el módulo aislado
`src/provider_control_style.rs` para los mensajes de
REGISTER/recuperación y fallback. Evita importar el
registro de comandos y SQLite del núcleo. El menú
dinámico recibido por IPC ya viene formateado; no
se envuelve otra vez. Se aplicaron los diffs
Rustfmt del verificador. Probar los mensajes
NativeFlow reales en Android/iOS tras confirmar
CI, conservando solo tres botones de lectura.
No se copió ningún secreto ni se modificaron sesiones.

## Menú modular y botones nativos, 2026-11-18 (alpha)

El controlador usa un catálogo Rust por carpeta
y por comando. Permite `menu`, `menu set`,
`menu antis`, `menu antis 2`, `menu system`,
`menu ai` y demás categorías. Estilos
`classic`/`none`, idiomas `es`/`en`,
preferencias persistidas fuera de la sesión.

`MATTER_WA_NATIVE_BUTTONS=1` intenta enviar
en un solo mensaje Estado, Menú y Antis.
Su respuesta se convierte exclusivamente en
`=/>status`, `=/>menu` o `=/>menu antis`,
y se verifica por HMAC, origen, antirreplay
y política de chat. Las llamadas a submenús
de otros usuarios siguen siendo solo lectura.
El pie usa marcas nativas generadas desde
HTML breve validado; no HTML/CSS arbitrario.
`MATTER_WA_RICH_HTML=1` es experimental,
self-chat y no demuestra visualización Android/iOS.

No se importan claves, no se ejecuta sandbox,
no se activan servicios ni se conceden rangos.
Ver `../../docs/MENU_CATALOG.md`.
README del proveedor revisado.

## 2026-11-18 — Matter AI: solo broker Vault y permiso del titular

El controlador `matter` sigue limitado al propietario autenticado
en su chat privado PN/LID, con HMAC y antirreplay. No se reutilizan
claves de IA obtenidas de Aura ni otros repositorios; Base-Matter
no acepta claves API directas. Solo un gateway Unix con ruta y UID
esperado configurados puede responder, tras verificar permisos y aplicar
la política del titular en cada solicitud. El cliente rechaza respuestas
sin `policy_applied=true`, `grant_id` y revisión positiva.
El gateway productivo **aún no existe**; no anunciar capacidad
de IA en vivo como verificada. Ver `../../docs/MATTER_AI.md`.

## Matter AI por WhatsApp: comando privado de solo análisis (alpha)

El propietario puede usar `matter status`, `matter ask` y
`matter plan` a través del puente firmado existente. El
proveedor solo acepta `matter` en su lista privada
**cuando el mensaje procede de la cuenta autenticada**;
no existe ruta pública ni de grupos. El controlador
aplica además permisos de propietario y aislamiento de
la ruta de extensiones. La consulta IA no es ejecución
de shell ni modificación de GitHub.

El controlador consume claves de entorno autorizadas
(Claude preferente, Venice después, OpenAI alternativo),
no claves de Aura Reed ni archivos de sesiones.
El plazo de entrega IPC se amplía solo para consultas
`matter`. Otras operaciones conservan 4 segundos.
No hay garantía de respuesta externa hasta probar
cuentas válidas. Ver `../../docs/MATTER_AI.md`.

## Normalización Rustfmt del transporte RichResponse (alpha)

La comprobación inicial del commit `d8c3857` se detuvo
en el formato de la bandera `MATTER_WA_RICH_HTML` y
tres aserciones del test `rich_transport`. Ajustado
conforme al diff de CI. Sin modificación funcional,
sin habilitar envío por defecto. Revalidar el nuevo SHA.

## Ensayo RichResponse opt-in únicamente en el chat propio (alpha)

Se conserva `rich_preview.rs` y sus comandos offline
`--rich-preview-html` / `--rich-preview-json`. El módulo
independiente `rich_transport.rs` empaqueta ese JSON como
`AIRichResponseMessage` nativo, sin alterar los datos ni
afirmar ser Meta AI. Sólo se intenta entregar tras recibir
`=/>menu` del propietario autenticado en su self-chat y con
`MATTER_WA_RICH_HTML=1`. En grupos o mensajes de terceros
esta opción no se usa, aun si está activada. Default apagado.

El proveedor hace fallback a NativeFlow o al menú original si
`send_message` rechaza la operación; no detecta si el cliente
ignora un envío aceptado. Es un **ensayo**, no HTML WhatsApp
garantizado. El modo de prueba no ejecuta scripts de `xd.zip`,
no atribuye origen Meta AI, no permite botones administrativos
ni abre una segunda sesión. La vista previa local conserva sus
botones deshabilitados; los únicos controles con interacción
real continúan siendo los tres NativeFlow de solo lectura.
Detalles: `../../docs/WHATSAPP_RICH_HTML_COMPAT.md`.

## Ajuste de formato del prototipo Rich (alpha)

El verificador de `2dfc214` rechazó el formato de dos índices
JSON y una aserción en `tests/rich_preview.rs`. Corregido conforme
al log de Rustfmt, sin alterar lógica ni habilitar envío Rich.
Nueva ejecución de verificación obligatoria.

## Compatibilidad experimental del material Rich HTML (2026-11-18)

Analizados ejemplos externos en Go/JavaScript/HTML: el contenedor
`embedded_screens`/`FOAIDNixelButtonSheets` puede estudiarse, pero
el envío proviene de un `botForwardedMessage` que aparenta tener
identidad `0@bot`. Ese método **no** se trasplanta a Base-Matter ni
se afirma que WhatsApp acepte HTML de terceros.

Ahora `rich_preview.rs` genera HTML estático y un JSON de compatibilidad
sin abrir el proveedor, con el menú original y sin JavaScript:
```bash
cargo run --manifest-path providers/whatsapp-rust/Cargo.toml -- --rich-preview-html > matter-preview.html
cargo run --manifest-path providers/whatsapp-rust/Cargo.toml -- --rich-preview-json > matter-preview.json
```
El proceso debe iniciarse desde la raíz del repositorio. Las salidas
son solamente **archivos locales de prueba**, no mensajes WhatsApp.
Los botones del HTML se muestran deshabilitados deliberadamente; los
únicos botones nativos reales siguen siendo NativeFlow opt-in. No se
usa Node, Baileys, spoof de Meta AI ni una segunda sesión.
Véase [análisis de compatibilidad](../../docs/WHATSAPP_RICH_HTML_COMPAT.md).

## Ajuste de orden de imports NativeFlow (alpha)

El paso rustfmt en `3178e32` exigió ordenar el alias
`NativeResponse` y `wa`. Corregido siguiendo el diff de CI.
Sin modificaciones de lógica y sujeto a CI completo en el nuevo SHA.

## Ajuste de Clippy para el experimento NativeFlow (alpha)

En la revisión `d6dd6f1`, las pruebas nativas y el convertidor de
HTML breve terminaron en verde. Clippy detectó exclusivamente el
`..Default::default()` redundante del cuerpo protobuf y un patrón
de coincidencia irrefutable. Corregidos sin modificar botones,
lógica de firma HMAC o interfaz de sesiones. Nuevas pruebas CI
pendientes antes de ensayar Android/iOS.

## Corrección de formato NativeFlow (alpha, 2026-11-18)

El primer workflow del experimento `d532ef2` indicó solo diferencias
de Rustfmt en los archivos nuevos y el `main.rs` modificado; aún no
alcanzaba las pruebas. Se aplicaron las diferencias del log y se evitó
un patrón potencialmente irrefutable en el lector de respuestas.
Nuevo CI es obligatorio para verificar firma, botones y parser.

## Experimento NativeFlow y HTML de estilo (alpha, 2026-11-18)

Para una prueba controlada de botones, ejecutar en la terminal del
operador `export MATTER_WA_NATIVE_BUTTONS=1` y luego
`cargo run --locked` desde la raíz. Por defecto `=/>menu`
continúa enviándose en texto, con la plantilla original.
Con la opción activada, el proveedor intenta enviar un único
`InteractiveMessage.NativeFlowMessage` con el cuerpo exacto del menú
más tres `quick_reply`: Estado, Menú y Créditos. Los botones tienen
IDs específicos solo de lectura; una selección se procesa por el mismo
puente firmado HMAC, autorización de chat y protección antirreplay.
Un error de envío provoca fallback al texto. No se garantiza que
Android/iOS rendericen el proto sin un ensayo real.

`src/ui.rs` permite HTML **solo como lenguaje abreviado de estilo**:
`b/strong`, `i/em`, `s`, `code`, `br`, algunas entidades,
sin atributos ni JavaScript, convertido a marcado de WhatsApp.
No se crea un WebView, no se usa el tipo experimental
`FOAHtmlPrimitiveDemoDONOTUSE`. La discriminación Android/iPhone
no está implementada: no existe una identidad de SO autenticada en
los eventos que recibe esta versión del proveedor.
Ninguna función nueva envía mensajes espontáneos, cambia roles o
activa el bot por defecto. Mantener respaldo de sesión y bloqueo.

## 2026-11-18 — Corrección de la prueba de `status --global` público

El CI del proveedor sobre `ce57169` compiló y pasó 15/16 pruebas
`tests/provider.rs`; la restante aún esperaba ignorar `status --global`
de un lector externo. Ahora esa consulta se firma pero el controlador
siempre devuelve una negativa de privacidad; nunca expone políticas
ni acepta cambios del socket. Actualizada la prueba y solicitada
verificación nueva sobre el SHA exacto.

## 2026-11-18 — Menú original y lectura contextual / global (alpha)

El proveedor `whatsapp-rust` conserva el mensaje original de Matterssmith en
`=/>menu`, incluido el crédito final, sin una segunda caja de control.
`=/>status` de un participante externo se firma para la lectura del chat
efectivo, aun con el bot apagado. `=/>status --global` del propietario funciona
desde el chat propio y grupos autenticados, con las políticas reales.
Cuando un tercero pide `--global`, se firma únicamente una consulta inocua
y el controlador responde con una negativa informativa; no entrega datos de
socket ni admite mutaciones. Los cambios de estado siguen siendo owner-only.
La vinculación sigue por pairing code, con procesos Rust independientes y sin
modificar bases de sesión. Son correcciones sujetas a CI y prueba WhatsApp real.

# WhatsApp-Matter: proveedor Rust independiente

El proveedor original es whatsapp-rust, ejecutado en un **proceso Rust
separado** del núcleo Base-Matter. No usa Node ni Baileys ni Bot-Matter.

La separación responde a versiones incompatibles de libsqlite3-sys entre
la biblioteca whatsapp-rust 0.7.0 y las dependencias actuales de Base.
Ambos procesos siguen escritos en Rust y se comunican por un canal TCP
en 127.0.0.1 protegido con HMAC-SHA256 de 256 bits.

## Verificación del proveedor Rust (alpha; 2026-11-18)

GitHub Actions `44367046939` aprobó la verificación independiente del
proveedor para `7de9a0d`, tras la corrección de formato del marco compartido.
Queda una advertencia de Clippy del núcleo en las pruebas de antis, corregida
en el siguiente checkpoint. Este resultado no prueba un reinicio real de
sesión WhatsApp ni una moderación automática activa.

## Corrección de verificación Rust (alpha; 2026-11-18)

El último CI (`6142c6c`) falló por formato de la plantilla compartida
`src/control_style.rs` en la comprobación del proveedor, no por errores
verificados de emparejamiento. El mismo cambio también corrige el formato
del núcleo y la prueba desactualizada de créditos. Falta nuevo CI sobre el
SHA exacto y prueba real de WhatsApp antes de declarar esta revisión estable.

## Clippy: segundo consumidor del snapshot (alpha)

En la nueva verificación de `alpha`, Clippy del núcleo se detuvo
en `tests/whatsapp_native.rs:82`, que aún accedía al campo
posicional `.0` de `inspect_bot_defaults`. Se reemplazó por
`.socket_powered`, sin alterar la autenticación, las respuestas
tematizadas o la vinculación por pairing code. La suite del proveedor
no falló; simplemente no llegó a ejecutarse en ese intento porque
`set -e` detuvo el bloque en Clippy. Falta repetir ambas suites.

## Menú compacto y comando de créditos (alpha; pruebas reales pendientes)

El proveedor Rust permite `=/>credits`, `=/>creditos` y
`=/>créditos` con la misma autenticación y verificación HMAC que
los demás controles. Para remitentes ajenos solo se permite lectura.
Los mensajes normales heredan el marco común corto, sin un footer
automático de autor ni nombres duplicados. La atribución completa
permanece en el código y se muestra exclusivamente mediante
el comando de créditos.

Referencias estudiadas: Aura-Bot-2.0 usa mensajes enriquecidos,
miniaturas y, en comandos concretos, una vista HTML experimental
(`FOAHtmlPrimitiveDemoDONOTUSE`). El ZIP YoSoyYo incluye
`nativeFlowMessage` y botones `quick_reply` / `cta_url`.
Ninguno ofrece una detección fiable del sistema operativo del destinatario.
**No se envían aún botones HTML ni versiones forzadas Android/iOS**
desde este proveedor; implementar eso sin detección fiable rompería
el requisito de no mostrar una variante equivocada. El transporte
Rust, el pairing code, los archivos SQLite y el control de sesión
permanecen sin modificaciones.

## Ajuste posterior a Clippy del núcleo (alpha)

La nueva compilación de Clippy encontró una referencia posicional
antigua en la prueba del puente `tests/whatsapp_bridge.rs`.
La referencia fue actualizada a `BotDefaultPolicies::socket_powered`;
el proveedor WhatsApp Rust no requiere cambios de protocolo ni
de sesión para este ajuste. La suite independiente de 21 pruebas
estaba aprobada en la ejecución anterior, no en el nuevo SHA.
Las verificaciones estrictas restantes deben repetirse.

## Resultado comunicado del proveedor Rust — 2026-11-18

El operador ejecutó la suite independiente del proveedor después
de corregir el ingreso nativo. Pasaron **21 pruebas**:
una del ejecutable principal, dieciséis de `tests/provider.rs`,
tres de recuperación y una de REGISTER. El bloqueo restante
reportado fue `clippy::type_complexity` del **núcleo Base**,
no una prueba fallida del proveedor.

La corrección sustituye la tupla de seis políticas por una
estructura Rust nombrada, sin alterar el protocolo HMAC,
las sesiones SQLite, pairing code ni las plantillas WhatsApp.
Falta la nueva ejecución de Clippy de ambos paquetes.

## Revisión del ingreso nativo (alpha; suite completa pendiente)

La suite Base identificó un caso donde `=/>on` retirado y
demasiado largo se ignoraba antes de validar el límite de entrada.
Se reordenó la validación en `src/whatsapp_native.rs`. El
proveedor independiente continúa firmando solo los comandos
autorizados y no recupera los alias antiguos. La prueba nueva
comprueba también un `=/>bot on` excesivo y que un rechazo
no consuma el identificador de replay. No modificar sesiones
WhatsApp, archivos SQLite ni el `Cargo.lock` local.
La nueva ejecución del proveedor aún no está confirmada.

## Validación por fases en Debian (alpha)

La última suite reportada del **núcleo** se detuvo por
una aserción antigua de `tests/control_style.rs`, que
buscaba un `bot_id` interno en el menú compacto.
La expectativa ya fue corregida en GitHub, sin añadir
dicho identificador a las respuestas. El proveedor Rust
no llegó a ejecutarse en ese lote: **no asumir su
aprobación para el nuevo commit**.

El operador tiene modificaciones locales generadas por
`cargo fmt` y un `providers/whatsapp-rust/Cargo.lock`
sin seguimiento. Preservar ambos al sincronizar; el lock
no contiene una sesión WhatsApp y no se debe borrar como
forma de corregir esta regresión. Repetir las pruebas del
proveedor después de confirmar la suite central.

## Respuestas propias y plantilla seleccionada de cada chat (alpha)

La respuesta de `WhatsAppControlIpc` llega ya renderizada con la
plantilla del chat verificado. El proceso `whatsapp-rust` conserva
esa respuesta y **no impone encima el diseño predeterminado**.
Para informes generados directamente en el proveedor (REGISTER,
recuperación comunitaria y respuestas de reserva), utiliza
`render_provider_control`, que carga exactamente el mismo
archivo de plantilla y nunca otra variante escrita dentro de
los handlers.

La prueba nueva `tests/whatsapp_theme_scopes.rs` verifica en
el núcleo la salida por grupo, con créditos y saltos de línea
reales. Falta probarlo con el binario WhatsApp conectado.
No se cambian SQLite, pairing code, claves ni sesiones.

## Contrato WhatsApp actual: un solo comando y una sola plantilla (alpha)

Los comandos `=/>on/off` se han retirado del proveedor y del puente
firmado. Usar solo `=/>bot on/off` en el chat autorizado; habilitar
despierta el socket sin un encendido adicional. Para predeterminados
usar `--global` o los alcances `group/private/channel/community`
en el chat propio. La variante `community` corresponde a avisos
comunitarios: no se aplica a grupos ordinarios sin metadatos de tipo.

El proveedor **reutiliza `src/control_style.rs`** para decorar también
respuestas directas REGISTER, recuperación y vistas previas. Ambos
procesos comparten las plantillas editables de la raíz:
`templates/matterssmith-control.txt` y
`templates/matterssmith-menu.txt`. No copiar diseños dentro de los
manejadores. El menú único no tiene subcategorías. El salto de línea
es un carácter real, jamás texto literal `\n`.

Esta revisión está publicada solo en alpha, pendiente de pruebas
reales en Debian; los 20 tests del proveedor informados previamente
corresponden al commit anterior `dfd9dab`, no certifican este cambio.

## Distinción en la suite Base: consulta grupal con bot apagado

La regresión siguiente, tras aprobar `control_ipc`, estaba
en `tests/group_control.rs`: la prueba rechazaba
`=/>status` desde un grupo del propietario. Se ajustó
la expectativa para exigir estado grupal `PoweredOff`
cuando el socket está apagado, sin permitir operaciones
administrativas de un participante externo. El proveedor
WhatsApp no necesita cambios para este caso; su suite
independiente ya había aprobado 20 pruebas.
La suite Base debe volver a ejecutarse para validar la
corrección antes de continuar con WhatsApp real.

## Resultado de compilación del proveedor (Debian, 2026-11-18)

El operador informó `cargo test --manifest-path
providers/whatsapp-rust/Cargo.toml --all-targets` con
**20 pruebas aprobadas y ninguna fallida**: una prueba del
binario principal, quince del protocolo/proveedor, tres de
recuperación y una de REGISTER. El defecto que persiste es una
expectativa en la suite **Base**, no en este proveedor:
`=/>bot private off` debe deshabilitar privados externos,
sin afectar al chat propio del dueño. La prueba IPC ya fue
alineada con esa separación, pero necesita ejecución Debian.
No eliminar `Cargo.lock`, sesiones, llaves ni caché compilada.

## Lectura pública autenticada sin rangos ni control de socket (alpha; pruebas pendientes)

En un grupo, el proveedor puede firmar `status/menu/bot` procedentes
de otros participantes usando el JID PN/LID real del remitente. En
un chat privado externo exige que el JID del chat coincida exactamente
con el del remitente. La sesión conectada debe coincidir además
con la identidad emparejada del socket; el HMAC vincula al actor,
chat, comando, mensaje y tiempo. En la capa Base una ruta especial
devuelve solo estado o menú, sin invocar extensiones generales.

No se acepta `=/>status --global`, `--debug`, `=/>on`,
`=/>bot on`, cambios de configuración ni `=/>set role`
desde terceros. Cuando está deshabilitado el bot, el estado
sigue respondiendo y el menú no responde. No asumir
funcionamiento de canales o privado PN/LID cruzado aún:
esas identidades necesitan pruebas verificadas del proveedor.
El chat propio usa política `self`, no el predeterminado
`private` de chats externos. No se modificaron sesiones,
llaves, ramas beta/stable ni repositorios de roles.

## Contratos recientes de chat, estado y silencio (alpha; build Debian pendiente)

La sesión real ya respondió comandos de propietario dentro de un
grupo, pero dejó claro que `PoweredOff` significa socket apagado,
no fallo de la política grupal. En alpha se incorporaron
`=/>status` (contexto actual), `=/>status --global`
(predeterminados), `=/>bot on/off/inherit` (chat),
`=/>bot group/private/channel on/off/inherit` (tipos) y
`=/>bot --global on/off/inherit` (socket).

`=/>menu` / `=/>menú` muestra un diseño Matterssmith propio por
categorías al habilitar el bot. Los comandos normales, el menú y
la ayuda de `bot` no responden cuando la política está apagada;
`status` sigue disponible. `=/>help` y `=/>h` se retiraron.
El proveedor acepta un acuse `ignored` auténtico y no lo responde
en WhatsApp. El límite acotado de respuestas IPC se amplió para
incluir el menú decorativo y `--debug` sin cargar datos secretos.

**Límite de seguridad:** `from_me` y PN/LID solo autorizan al
propietario vinculado; no abrir mensajes de otros usuarios
suplantando su identidad. El soporte de canales y privados
externos exige un nuevo ingreso autenticado. La conexión con
Vault-Matter y `selfmod` aún no están instalados.
Ver `../../docs/BOT_POLICY_V2.md`.

## Pruebas offline del binario y recuperación (alpha)

La suite del proveedor se ejecuta de forma independiente al núcleo:

```bash
cargo test --manifest-path providers/whatsapp-rust/Cargo.toml --all-targets
```

`tests/recovery.rs` resuelve el ejecutable
`whatsapp-matter-rust-provider` mediante
`option_env!("CARGO_BIN_EXE_whatsapp-matter-rust-provider")` y,
en su ausencia, `std::env::var_os` durante la prueba. La prueba
invoca realmente `--recovery-verify`, rechaza la clave equivocada
y rutas inseguras, y verifica que la sesión SQLite no se abre.
No crea una nueva vinculación de WhatsApp ni borra las sesiones.

`tests/provider.rs` utiliza el mismo mecanismo para `--check`.
Su prueba del JID `@lid` usa `=/>status` en privado, y confirma
que `=/>menu` se ignora en ese ámbito. Pruebas de compilación
y ejecución en el servidor: pendientes tras esta corrección.

## Fuente única de permisos (Vault-Matter)

Los rangos de socket y grupo no se pueden escribir localmente en
Base-Matter. El usuario dirige altas, cambios y revocaciones de
rango a un canal **propio y autenticado de Vault-Matter** en WhatsApp,
Telegram o Discord. No se aceptan mensajes reenviados o un nombre
visible como prueba de autorización. El servicio Vault es la única
autoridad, autentica cada actor, cifra las asignaciones y mantiene la
persistencia del repositorio privado. Base consulta permisos como
lector de solo lectura y deniega todo permiso no confirmado.

**Hasta que exista el servicio y el verificador:**
`=/>set role`, `=/>unset role`, `=/>role` y
`=/>group role` se rechazan sin modificar SQLite. El
`=/>roles` privado indica que falta la conexión con Vault.
La identidad propietaria original puede seguir apagando el socket
sin depender de un rango delegable.

## Archivo histórico — División privado/grupo anterior (sustituida por BOT_POLICY_V2)

El control remoto ya respondió en WhatsApp al propietario con `=/>status`
y `=/>on`. El chat privado propio se reserva desde ahora a los
ajustes globales del socket; `menu/help/start/add/add++` son
comandos de ámbito grupal. El proveedor verifica `from_me`, el
PN/LID real del cliente, el chat `@g.us` y el remitente propio.
El destino grupal queda firmado en el HMAC; el controlador determina
el grupo por ese dato autenticado, sin aceptar IDs en texto como
prueba de permisos. El controlador del socket no arranca Bot-Matter.
`=/>group on/off/inherit/status` afecta únicamente a ese grupo;
el encendido general `=/>on/off` sigue siendo privado.

`=/>set role <PN/LID> <rango>` es control privado del socket.
`=/>group role <PN/LID> <rango>` es una asignación específica
del grupo, todavía no una delegación de ejecución. Los rangos no
pertenecen al escalafón de administración del proyecto. `user`
es implícito; el almacén local persiste solamente asignaciones
no predeterminadas. Falta la publicación cifrada autenticada en
Vault-Matter y su recuperación; no asumir persistencia remota.
Las pruebas de grupos reales siguen pendientes en esta revisión.

## Corrección de identidad PN/LID autenticada (alpha)

La sesión vinculada se identifica ahora mediante `ctx.client.pn()` y
`ctx.client.lid()` de whatsapp-rust 0.7.0. Solo se aceptan mensajes
`=/>` propios en chat privado cuya identidad de chat y remitente
coincidan con PN o LID verificados del cliente, con JIDs de dispositivo
normalizados. El PN autenticado debe coincidir con el número configurado
y el JID del propietario debe ser uno de esos identificadores. El puente
HMAC conserva el JID canónico del propietario. Siguen denegados chats
ajenos, grupos, mensajes entrantes y prefijos antiguos `#ping`.
No es necesario regenerar la sesión ni el archivo `runtime.env`.
La prueba de recepción y respuesta real sigue pendiente.

## Ejecución en una sola terminal (alpha)

Para ahorrar espacio en Debian, utilizar **un solo clon Git** y exportar
`CARGO_TARGET_DIR` como ruta absoluta compartida antes de ejecutar
`cargo run`. El lanzador de `alpha` respeta la ruta tanto para
`whatsapp-matter-control` como para este proveedor Rust independiente.
Los artefactos compartidos no contienen las sesiones SQLite. Las sesiones,
claves y configuración son locales y particulares de cada rama. Cambiar de
rama requiere detener antes cualquier proceso WhatsApp activo. `stable` y
`beta` conservan sus propias verificaciones; este lanzador no ha sido
promovido y aún requiere prueba en un dispositivo real.


La rama `alpha` incorpora `cargo run` desde la raíz de Base-Matter.
El ejecutable `base-matter` inicia el controlador y este proveedor
Rust como procesos independientes bajo una sola sesión de terminal,
para evitar conflictos `libsqlite3-sys`. No usa Node ni QR.

Cada ejecución utiliza una ruta privada y un puerto reservados según
la rama Git: `alpha` usa 39821, `beta` 39822 y `stable` 39823.
El lanzador solo existe por ahora en `alpha`; las ramas `beta` y
`stable` requieren promoción y verificación por separado.
Sin configuración previa pregunta únicamente el teléfono y conserva
su SQLite y secretos para el siguiente arranque. Un JID derivado de
un número no garantiza coincidencia con el LID autenticado y los
comandos propios permanecen denegados ante cualquier discrepancia.
Una conexión en pantalla no demuestra que se haya aplicado `=/>status`.

## Parámetros locales

- MATTER_WA_PHONE: número para pairing code, 8–15 dígitos internacionales
- MATTER_WA_OWNER_JID: JID propio privado (dominio s.whatsapp.net o lid)
- MATTER_WA_SOCKET_ID: socket registrado en Base-Matter
- MATTER_WA_OWNER_ID: dueño registrado en Base-Matter
- MATTER_WA_SESSION_DB: ruta absoluta al SQLite privado de whatsapp-rust
- MATTER_WA_BRIDGE_ADDR: 127.0.0.1:puerto del controlador de Base
- MATTER_WA_BRIDGE_KEY_HEX: clave privada HMAC de 32 bytes, 64 hex

La carpeta del SQLite debe estar protegida para el usuario del servidor.
El propietario de la sesión se bloquea mediante archivo owner.lock para
impedir procesos duplicados. No se elimina la sesión de WhatsApp.
Realizar un respaldo SQLite consistente antes de actualizar una sesión
existente. Probar primero un número secundario.

Ejecución desde el repositorio, con las variables de entorno cargadas:

    cargo run --manifest-path providers/whatsapp-rust/Cargo.toml

El enlace se realiza exclusivamente mediante código de vinculación mostrado
en la terminal del operador. El proveedor no registra callback QR ni muestra
códigos QR; whatsapp-rust puede crear un desafío QR interno en paralelo,
sin que este proceso lo exponga.

El proceso valida el mensaje de la cuenta propia, JID de chat y remitente
exactos, fromMe y chat no grupal. Firma únicamente mensajes canónicos
=/> con timestamp reciente y los entrega al controlador Rust de Base.
Solo responde con el acuse genérico `applied` al chat propio autenticado;
no responde ping ni ejecuta Bot-Matter.

## Límite de estabilidad

El proveedor requiere un proceso receptor Base-Matter configurado con el
mismo secreto y socket, una sesión real vinculada mediante pairing code,
revisión de compatibilidad SQLite y pruebas de revocación y reconexión.
No se puede afirmar estabilidad por aprobar únicamente pruebas offline.

Biblioteca upstream: oxidezap/whatsapp-rust, versión 0.7.0 (tercero, no
oficial de WhatsApp). Conservar licencias y atribuciones upstream.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## Preflight Rust sin conexión

Con el entorno configurado ejecutar `cargo run --manifest-path providers/whatsapp-rust/Cargo.toml -- --check`. Comprueba el número internacional, JID propio, claves y ruta privada sin abrir sesiones ni conectarse a WhatsApp. El controlador independiente de Base ofrece `cargo run --locked --bin whatsapp-matter-control -- --check`. No publicar variables de entorno ni archivos SQLite.

## Guía de prueba real

La [guía de dos terminales y emparejamiento](../../docs/WHATSAPP_TRIAL.md) usa únicamente Rust. El script `scripts/whatsapp-test-setup.sh` prepara claves privadas diferentes sin sobrescribir sesiones. El modo `--check` valida el entorno antes de abrir una sesión. Nunca compartas el código de vinculación ni los secretos.

## Propiedad de sesión segura

En sistemas Unix, el archivo `owner.lock` se crea con permisos `0600` en la llamada de apertura, sin una ventana inicial con permisos de grupo/otros. La carpeta de sesión debe seguir siendo privada (`0700`). Un bloqueo residual exige recuperación manual verificada; no eliminarlo automáticamente ni borrar `session.db`.

## Resultados del control global

Al enviar `=/>`, el proveedor diferencia el efecto nuevo `applied` del
replay `duplicate` y los rechazos. Un resultado `duplicate` nunca se anuncia
como ejecución nueva; los acuses malformados o desconocidos se rechazan.
El control solo intenta una confirmación breve cuando se ejecuta una
operación; no devuelve todavía los detalles del estado. Requiere prueba
real autorizada antes de promoverlo a estable.

## Reinicio seguro tras una caída

En Rust 1.89 o posterior, la exclusión del archivo `owner.lock` utiliza un
bloqueo de sistema operativo y un marcador v2 permanente. Tras un cierre o
caída del proceso, el bloqueo se libera y el mismo archivo puede utilizarse
de nuevo sin tocar `session.db`. Si hay otro proceso activo, se deniega
el acceso. Los antiguos locks con PID (v1) se rechazan por seguridad hasta
la revisión manual del operador: la migración nunca borra locks ni sesiones.
`--check` verifica la disponibilidad del archivo de modo de solo inspección.

## Orden de arranque comprobado

Al ejecutar el proveedor para vincular WhatsApp, primero comprueba que el
receptor Base-Matter escucha en la dirección 127.0.0.1 configurada; si no,
aborta antes de abrir la sesión SQLite o solicitar un código. La comprobación
solo establece disponibilidad TCP, **no autentica el controlador**: el HMAC
protege los comandos. El modo `--check` sigue sin red ni escrituras.

## Confirmación en el chat privado

Tras un acuse `applied` autenticado, el proveedor intenta enviar al
chat propio «Base-Matter: solicitud procesada.». Los duplicados y errores no
generan respuestas. No se envía a grupos, terceros ni mensajes que no tengan
el prefijo `=/>` desde el JID autenticado. Es un aviso genérico: no implica
que `status` ya tenga salida detallada, ni sustituye las pruebas de entrega
con un teléfono autorizado.

## Comunidad REGISTER (alpha)

`=/>start` se reserva para inventario y verificación de comunidades
gestionadas por la sesión de WhatsApp. El diseño en
[`docs/COMMUNITY_REGISTER.md`](../../docs/COMMUNITY_REGISTER.md)
requiere verificar el rol admin y los identificadores exactos de subgrupo,
y distingue vista previa de operaciones con cambios. Ninguna actualización
administrativa se debe asumir completa por aprobar CI offline.

## Encabezado oficial REGISTER

La función de detección solo reconoce la primera línea exacta
`MATTER:OFFICIAL:register:<community-jid>`, validando formato de JID
comunitario. Las coincidencias parciales, colocadas en otra línea o con
ID de otra comunidad se rechazan. La descripción **no es autenticación**;
además se deben verificar metadatos de pertenencia y rol de administrador.

## Escaneo de comunidades reales con =/>start (alpha)

Tras pasar HMAC y permisos en Base, el callback de `whatsapp-rust` utiliza
`Community::get_participating`, `get_subgroups` y
`query_linked_group`. El escaneo verifica el admin del propio JID PN/LID,
pertenencia del subgrupo a la comunidad y encabezado REGISTER exacto.
Responde en el chat propio y no modifica grupos ni roles. Limita a 12
comunidades, 64 subgrupos por comunidad y 20 segundos por solicitud.
No se puede declarar probado en red sin un teléfono vinculado.

## Respuesta de comandos globales desde Base

El proveedor admite el acuse opcional `message` de una operación
`applied` para mostrar menús o estados en el self-chat. Exige menos
de 761 bytes, texto UTF-8 válido, sin NUL ni controles salvo saltos de
línea. Los `duplicate`, `denied` y campos inesperados no contienen
respuestas. La verificación no equivale a privilegio de administración.

## Ayuda y menú del controlador Base (alpha)

Los comandos `=/>menu`, `=/>help` y `=/>h` devuelven en el chat
privado propio el formato autorizado por Base-Matter. El texto se valida
y se limita a 760 bytes; los mensajes duplicados o no autorizados no
generan respuesta. El proveedor no interpreta comandos externos
`#matter` ni importa sesiones de bots ajenos.

## Menú simple para pruebas beta

El menú de control de Base-Matter es intencionalmente plano. `=/>menu`, `=/>help` y `=/>h` no incluyen una estética atribuida a otro bot sin una muestra observada y aprobada. Todas las respuestas siguen exigiendo validación del dueño del socket.

## Prueba desde la nueva rama beta

La guía `docs/WHATSAPP_TRIAL.md` corresponde ahora a `beta`. `=/>menu`, `=/>help` y `=/>h` muestran comandos del núcleo sin diseño preestablecido, y `=/>start` solo consulta comunidades. No copiar menús de otras cuentas sin una fuente verificable.

## Respaldo de comunidades `=/>add++` (fase alfa)

El controlador Base-Matter autoriza un inventario de recuperación, pero todavía no incorpora participantes ni crea un grupo espejo. El respaldo cifrado local y la lectura real del proveedor se integran de forma separada, conservando permisos, privacidad, exclusión de duplicados y protección contra nuevas suspensiones. La prueba actual de WhatsApp desde `beta` no debe declararse capaz de migrar usuarios automáticamente.

## Inventario de recuperación WhatsApp

Después de que Base-Matter autorice `=/>add++`, se consultan las comunidades donde la sesión actual figure como administradora. Se muestran conteos aproximados de integrantes de subgrupos (sin publicar identidades), el ID de comunidad y el nombre sugerido del grupo de respaldo. No se añaden participantes; no se extraen miembros de comunidades suspendidas.

## Almacén local de instantáneas cifradas

El módulo `recovery` cifra registros de participantes y la identidad de la comunidad con XChaCha20-Poly1305, usando una clave independiente `MATTER_WA_RECOVERY_KEY_HEX`. Se crean archivos nuevos `community-recovery-*.enc.json` con permisos 0600 en la carpeta privada del SQLite; no se sobrescriben snapshots anteriores. Se exige revalidar el socket y el dueño al descifrar. Para un entorno ya configurado, ejecutar `bash scripts/whatsapp-test-setup.sh --enable-recovery` sin borrar la sesión. No confundir snapshot de miembros con exportación de credenciales de WhatsApp.

## Comando real de instantánea (alpha)

`=/>add++ snapshot <ID_COMUNIDAD@g.us>` solicita un respaldo de identidades PN/LID desde los grupos enlazados solo cuando la cuenta es admin de esa comunidad y WhatsApp permite consultarla. Se deduplican las identidades conocidas, se cifran usando la clave independiente y se escribe un archivo privado nuevo. La respuesta enviada al chat muestra el conteo y el nombre del archivo, no la lista. No se agrega nadie ni se eluden suspensiones o privacidad. Para habilitar la clave en una configuración existente: `bash scripts/whatsapp-test-setup.sh --enable-recovery`.

## Verificar el respaldo sin sesión activa

Con el mismo `runtime.env` y una copia del archivo cifrado, `cargo run --manifest-path providers/whatsapp-rust/Cargo.toml -- --recovery-verify NOMBRE.enc.json` valida clave, integridad, socket y dueño sin abrir la sesión de WhatsApp. El nombre debe corresponder a un archivo del directorio privado y no puede contener rutas. El resultado muestra solo el conteo, nunca PN/LID.

## Verificación del binario real

Se añadió prueba offline ejecutando el binario `--recovery-verify`, comprobando integridad, ausencia de sesión SQLite nueva, rechazo con clave errónea y no filtración de JIDs por stdout. No se ejecuta ninguna operación de grupo ni conexión de WhatsApp en CI.

## Nuevo =/>add (alpha)

El controlador Base-Matter valida la gramática de `=/>add` y responde con un acuse de vista previa, sin realizar altas, traslados ni cambios de permisos. `reply` exigirá metadatos de un mensaje real; `official` deberá resolverse desde un registro verificado de grupos/comunidades. El proveedor actual admite controles globales desde el chat propio y no se declara listo para operar `add user` dentro de grupos.
