## 2026-11-20 — Reparación de alta Ubuntu Sakura

- Se reportó `failed to look up user Sakura: No such process`, `sudo: unknown user Sakura` y `sudoet_audit` al usar `loginctl` y `sudo -iu Sakura`. Diagnóstico probable: `Sakura` todavía no existe, no falla la conexión OpenAI.
- Crear usuario **antes** de linger o cambio de sesión. Método manual: `sudo useradd -m -U -s /bin/bash Sakura` si no existe, `sudo passwd Sakura`, `sudo usermod -aG sudo Sakura`, `id Sakura`, `sudo loginctl enable-linger Sakura`, `sudo -iu Sakura`. No pasar contraseñas en la línea de comandos ni borrar administrador original.
- Reparación con script: `sudo bash tools/setup-sakura.sh --admin --password-prompt --linger` en rama `workstreams/02-base`. Crear/actualizar cuenta idempotentemente sin sudo sin contraseña.
- No afirmar que el usuario está creado ni que autostart 24/7 funciona hasta que el servidor muestre UID/grupos y servicio activo.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## Matter-AI local sin consumo de API comercial — 2026-11-20

- `matter-ai backend local` selecciona Ollama en loopback (`http://127.0.0.1:11434`) y `matter-ai local-model qwen3:0.6b` selecciona el modelo de bajo consumo; `matter-ai backend openai` revierte a API OpenAI con facturación aparte.
- `matter-autopilot doctor` valida disponibilidad local y claves. El ciclo «continúa el proyecto matter» utiliza el backend configurado y respeta los mismos permisos de ramas, Vault y Bot-Matter. Sin llave API en modo local, sin garantía de GPT-6 ni acceso al historial ChatGPT de teléfono.
- Modelo de 0.6B o 1.7B no alcanza razonamiento/seguridad de modelos grandes; tareas de Rust/CI requieren revisión. No activar servicios ni publicar ramas protegidas automáticamente.
- Documento: `docs/MATTER_AI_OLLAMA_LOCAL.md`. Ubuntu Sakura y Ollama todavía sin instalar o verificar.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## Operación autónoma local — Matter-Autopilot (2026-11-20)

- Comando literal repetido: **«continúa el proyecto matter»**. Supervisor de workstreams: `matter-autopilot`, instalado desde Base-Matter `workstreams/02-base`.
- Autopilot sólo edita archivos fuente/docs seleccionados y crea commits locales después de cada ronda exitosa; no ejecuta tests ni hace push por defecto. Aprobación `--push-workstreams` permite publicar únicamente en esa misma rama de trabajo, nunca en `alpha`/`beta`/`stable`. Bot-Matter bloqueado.
- Estado local persistente y diario `~/.local/state/matter-ai/autopilot-state.json`. El monitor salta checkouts sucios y muestra pendientes. Guía `docs/MATTER_AUTOPILOT_UBUNTU.md`.
- No hay ejecución de este servicio en la laptop sin instalación, key API, clones y un `systemd --user` activo. No se promete trabajo ilimitado por consumo, red, batería ni API.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## Cuenta Sakura y persistencia entre sesiones — 2026-11-20

- Usuario Linux `Sakura` creado por `tools/setup-sakura.sh` al ejecutarlo como administrador en Ubuntu; NO se ha ejecutado aún. Por defecto contraseña bloqueada, no vacía; se permite opción `--password-prompt` o clave SSH pública.
- Cada trabajo en `/home/Sakura/Matter/workspaces/<ID>`, rama exacta `workstreams/NN-id`; checkpoints de metadatos privados en `/home/Sakura/Matter/checkpoints/<ID>`.
- Guardado masivo: `matter-ai checkpoint all` (sólo metadatos) o `matter-ai save all` (commit/push individual por cada checkout detectado, con confirmaciones por proyecto). Workstreams no clonados y Bot-Matter se omiten; un error no se oculta.
- Comandos CLI `matter-ai sync ID`,  `branches ID`, `workstatus ID`, `checkpoint ID`, `save ID`, `promote ID alpha|beta|stable|feat/rama`. Save requiere confirmación de commit/push; promoción prepara PR draft con `gh` y no merge. No escribir directamente sobre alpha/beta/stable.
- El avance no está respaldado en GitHub hasta que `git push` se complete. Checkpoint local NO sustituye copia real de código. CI/verificación y revisiones siguen siendo gates independientes.
- Vault/Matter permisos protegidos; no mover sesiones ni datos sensibles. Bot-Matter bloqueado.
- Leer `docs/MATTER_SAKURA_BRANCH_RECOVERY.md` antes de configurar Ubuntu.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

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
Matriz `config/ecosystem-workstreams.json` y ramas `workstreams/01-orchestration` ... `workstreams/27-bot`. 27 líneas en 22 repos, ramas ya creadas. `docs/ORDERED_WORKSTREAM_BRANCHES.md` y `scripts/verify_workstream_branches.py` documentan orden. Trabajar en ramas correspondientes, CI y PR draft, sin push automático. El agente puede publicar exclusivamente mediante `publish_workstream` si el operador confirma la lista exacta de archivos y rama. Auditar primero SHA real de GitHub.

## Protocolo de trabajo del agente

1. Identifica el workstream solicitado y verifica que el clon existe. Si no, indica `matter-ai sync ID`.
2. Lee README/AGENTS/WORK_CONTINUITY y archivos de implementación existentes; evita reescribir lo ya hecho.
3. Crea cambios en la rama `workstreams/NN-id` correspondiente; no toques `main`, `alpha`, `matter`, `stable`, `beta`.
4. Ejecuta pruebas y describe resultados reales. Si una herramienta no autoriza cambios, no afirmes que se ejecutaron.
5. Mantén estado y siguiente paso en la documentación; no incluyas secretos.
6. Responde siempre: resumen, archivos, pruebas, pendientes reales, siguiente etapa.
7. Sin aprobación del operador, **no reiniciar servicios, modificar sesiones, hacer push, alterar Vault ni ejecutar comandos arbitrarios**. Aunque haya aprobación, no usar merge automático ni push a ramas protegidas.

**Nota:** Los repositorios privados requieren autenticación GitHub ya establecida en Ubuntu. Esta aplicación solo posee herramientas locales; los pasos que requieran operaciones no disponibles deberán explicarse y no simularse.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
