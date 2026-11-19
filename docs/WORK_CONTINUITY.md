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
