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
