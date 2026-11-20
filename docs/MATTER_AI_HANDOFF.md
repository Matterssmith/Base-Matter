# Handoff — Matter-AI para Ubuntu Server

**Actualizado:** 2026-11-20. **Origen:** instrucciones de Matterssmith verificadas durante el desarrollo, ramas y documentación GitHub. **Uso:** contexto de arranque; cada proyecto define sus reglas reales en sus README/AGENTS. No es historial privado de ChatGPT ni autoriza permisos de producción.

## Identidad y diseño

Ecosistema Matterssmith creado por Lournyx Matterssmith [Lou-Nyx]. Todos los archivos nuevos y la documentación deben llevar:
**Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].**

Prioridad de lenguajes: Python, C++, Rust; secundarios C y Go; TS/JavaScript/Bash/PowerShell/Lua para contratos y compatibilidad. Núcleo del proveedor WhatsApp y motor general **Rust**, no sustituir por Node. En la marca Matter se utilizan nombres Base-Matter, Vault-Matter, WhatsApp-Matter, Telegram-Matter, Discord-Matter, Social-Matter, Matter-Rust y MSEL.

La base es independiente de Bot-Matter. **Bot-Matter queda BLOQUEADO** hasta que Base/Vault/transportes pasen los criterios de preparación verificables. Prefijo global de control `=/>`, comandos específicos de bot `#`; autorización de owner global sólo con identidad verificada por Vault, sin fiarse de números enviados, nombres visibles ni nombres de usuario.

## Siguientes prioridades por repositorio

### Base-Matter — workstreams/02-base
Verificar núcleo y pruebas; mantener sesión WhatsApp existente intacta. Permisos globales iguales entre owners verificados, pero jamás saltarse autenticación y controles por socket. Mantener README, AGENTS, docs/WORK_CONTINUITY.md en cada cambio.

### Matter-Rust — workstreams/07-matter-rust (Base-Matter)
Núcleo multiplataforma Rust para WhatsApp/Telegram/Discord y Social-Matter futura. Revisar `crates/matter-rust`; sus adaptadores son contratos offline, **no conectores plenamente integrados**.
Consola experimental en `crates/matter-rust/web`, con inventario de 126 capacidades procedentes de Baileys/WhatsMeow y extensiones Matter; comprobar CI, Rustfmt, tests, Linux/Windows, y evitar falsear soporte real. `--serve-preview` sólo loopback. No invadir sesión de producción.

### WhatsApp — workstreams/10-whatsapp / workstreams/11-whatsmeow-compat
Revisar funciones oficiales tulir/whatsmeow, WhiskeySockets/Baileys, oxidezap/whatsapp-rust; comparar licencias y versiones. Nativo Rust y emparejamiento por código, no QR. Integrar grupos, canales, multimedia, mensajes editados, anti camuflaje, botones, encuestas, NativeFlow, HTML/JS previo y compatibilidad multicliente **solo con pruebas E2E reales**. No confundir HTML servido en navegador con mensaje HTML renderizable en WhatsApp. Los proveedores de plataforma pertenecen a sus repos.

### Vault y Register — workstreams/03-vault, 08-register-ui, 09-register-gateway
Vault-Matter privado, control cifrado por IPC, XChaCha20-Poly1305, anti-replay, autorización tipada. Ningún secreto, clave maestra, sesión, teléfono de usuarios o token en Git. Identidades globales enlazadas por plataforma tras verificación real. Matter Register: frontend alojado por servicio del bot, HTTPS privado autorizado, consentimiento y protocolo de registro; el frontend loopback actual no constituye servicio público desplegado. No registrar por sólo pulsar un botón.

### MSEL — workstreams/06-msel (System-Matter)
Matter Script Extension Language, extensión `.msel`; objetivo de interfaz unificada para 32 lenguajes/tecnologías distintos, agrupados por compatibilidad y tipo: lenguajes, runtimes, serialización, multimedia, gráficos, UI y sistemas. Investigar el documento de clasificación real en System/Docs/Server antes de fijar gramática, ABI o compilador. Seguir cadena `.msel → .ts → .js` para compatibilidad de scripts legacy sin convertir JS en núcleo. Empezar por sintaxis, lexer, AST, validación, contratos y tests offline, sin ejecutar código arbitrario.

### Social-Matter — workstreams/22-social
Futura plataforma social independiente y privada; API y modelo de usuario, permisos, módulos, mensajes, contenidos, tiendas, privacidad y seguridad. No declarar servicio lanzado. Marketplace-Matter depende del contrato Social, pero Social puede avanzar independientemente.

### Telegram y Discord — workstreams/12-telegram / workstreams/13-discord
Conectores separados en Rust (Bot API, Gateway/HTTP), límites propios de APIs oficiales, eventos normalizados y vinculación de cuentas vía Vault. No almacenar tokens en repos ni prometer conexión real sin pruebas.

### Todo el ecosistema — workstreams/01-orchestration (Server-Matter)
Matriz `config/ecosystem-workstreams.json` y ramas `workstreams/01-orchestration` ... `workstreams/27-bot`. 27 líneas en 22 repos, ramas ya creadas. `docs/ORDERED_WORKSTREAM_BRANCHES.md` y `scripts/verify_workstream_branches.py` documentan orden. Trabajar en ramas correspondientes, CI y PR draft, sin push automático. Auditar primero SHA real de GitHub.

## Protocolo de trabajo del agente

1. Identifica el workstream solicitado y verifica que el clon existe. Si no, indica `matter-ai sync ID`.
2. Lee README/AGENTS/WORK_CONTINUITY y archivos de implementación existentes; evita reescribir lo ya hecho.
3. Crea cambios en la rama `workstreams/NN-id` correspondiente; no toques `main`, `alpha`, `matter`, `stable`, `beta`.
4. Ejecuta pruebas y describe resultados reales. Si una herramienta no autoriza cambios, no afirmes que se ejecutaron.
5. Mantén estado y siguiente paso en la documentación; no incluyas secretos.
6. Responde siempre: resumen, archivos, pruebas, pendientes reales, siguiente etapa.
7. Sin aprobación del operador, **no reiniciar servicios, modificar sesiones, hacer push/merge, alterar Vault ni ejecutar comandos arbitrarios**.

**Nota:** Los repositorios privados requieren autenticación GitHub ya establecida en Ubuntu. Esta aplicación solo posee herramientas locales; los pasos que requieran operaciones no disponibles deberán explicarse y no simularse.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
