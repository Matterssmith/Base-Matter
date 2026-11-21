# Revisión técnica: Aura-Reed-Rust → arquitectura Matter (handler y módulos)

**Estado:** propuesta documental. **Fecha:** 20 de noviembre de 2026. **Implementación:** ninguna en este cambio.  
**Objetivo:** identificar patrones reales del repositorio `LuferOS/Aura-Reed-Rust` que pueden adaptarse a Base-Matter, al proveedor WhatsApp-Matter y, cuando sea desbloqueado, a Bot-Matter.

## 1. Fuente primaria examinada

Repositorio público original: https://github.com/LuferOS/Aura-Reed-Rust  
Rama inspeccionada: `main`; árbol de Git observado: `f72e6b6de83dec3badf59517891f7a73b19ac02d`.

Archivos revisados directamente:

- [`src/handler.rs`](https://github.com/LuferOS/Aura-Reed-Rust/blob/main/src/handler.rs): extracción de texto, análisis de prefijos, moderación, permisos de grupo, construcción de `CommandContext` y despacho.
- [`src/context.rs`](https://github.com/LuferOS/Aura-Reed-Rust/blob/main/src/context.rs): `BotState`, `CommandContext`, respuestas, cachés, semáforos, envío multimedia.
- [`src/main.rs`](https://github.com/LuferOS/Aura-Reed-Rust/blob/main/src/main.rs): inicio, conexión, eventos del proveedor y sub-bots.
- [`src/cmd/mod.rs`](https://github.com/LuferOS/Aura-Reed-Rust/blob/main/src/cmd/mod.rs): categorías de comandos.
- [`src/cmd/socket.rs`](https://github.com/LuferOS/Aura-Reed-Rust/blob/main/src/cmd/socket.rs): menús, alias y comandos de sub-bots.
- [`src/db.rs`](https://github.com/LuferOS/Aura-Reed-Rust/blob/main/src/db.rs): registros de usuario/grupo y cachés acotadas.
- [`src/config.rs`](https://github.com/LuferOS/Aura-Reed-Rust/blob/main/src/config.rs): configuración, prefijos, parámetros y definiciones de propietarios.
- [`src/lang.rs`](https://github.com/LuferOS/Aura-Reed-Rust/blob/main/src/lang.rs): idiomas y traducciones.
- [`Cargo.toml`](https://github.com/LuferOS/Aura-Reed-Rust/blob/main/Cargo.toml) y README: dependencia relativa de `Baileys-Rust`, categorías y requisitos de ejecución.

**Precisión:** la arquitectura descrita debajo proviene de estos archivos. La arquitectura destino de Matter es una *recomendación*, no una función ya integrada. No se ejecutó el bot Aura, no se compiló el código externo ni se verificaron los benchmarks de su README. Sus cifras de consumo y latencia son afirmaciones del README y no pruebas independientes. Los fragmentos de configuración externa incluyen valores de API y propietarios definidos en código; **no copiar tales identificadores ni credenciales** a Matter.

## 2. Flujo observado de Aura

```text
Baileys-Rust / WhatsApp
      ↓
main.rs: sesión, conexión, eventos y SubBotManager
      ↓
handler::handle_message (msg, offline)
      ├─ Ignora mensajes offline
      ├─ Normaliza contenido; detecta revocaciones
      ├─ Registra información para snipe
      ├─ Resuelve chat, emisor, usuario y grupo
      ├─ Moderación: mute, antilink, antitoxic, antispam
      ├─ Estado del bot en el grupo y prefijo aplicable
      ├─ parse_command → prefijo, nombre, args, texto
      ├─ Permisos: ban, chatBanned, self, restrictedCmds, onlyAdmin
      ├─ Construye CommandContext
      ├─ Dispatch secuencial: system → socket → group → categorías
      └─ Si se atiende: contador de comandos + diagnóstico
```

**Detalles concretos:**

- `extract_message_text` usa contenido normalizado y extrae conversación, texto extendido o caption de imagen. No demuestra cubrir todos los tipos de mensaje o ediciones.
- `parse_command` recorre una lista de prefijos, devuelve nombre normalizado a minúsculas, argumentos y texto completo.
- `handler.rs` llama a `cmd::<categoria>::run(&ctx)` de forma secuencial hasta que uno retorna `true`. Es una cadena explícita, **no** un registro dinámico de comandos.
- Las categorías de `src/cmd/mod.rs` son `ai`, `download`, `economy`, `funy`, `gacha`, `games`, `group`, `interaccion`, `profile`, `search`, `socket`, `stalk`, `sticker`, `system`, `utils`.
- `BotState` agrupa estado de la aplicación, base de datos, cliente HTTP, sub-bots, semáforos y caches. `CommandContext` expone mensaje, emisor, rol/propietario, comando, prefijo, args y métodos de respuesta; **está acoplado** al socket WhatsApp.
- `main.rs` inicializa `SubBotManager::new(128)`, semáforo de comandos de 8, semáforo FFmpeg de 2, y proceso periódico de mantenimiento de DB de 180 segundos. Son valores **de Aura**, no parámetros obligatorios para Matter.
- Metadatos de grupos: caché con TTL de 300 segundos y máximo de 24 entradas; al llenarse, vacía todo. El límite es útil, pero la estrategia de reemplazo puede mejorarse.
- La moderación usa detecciones específicas de WhatsApp, palabras estáticas y ventana antispam de 5 segundos con umbral de 6 mensajes; son políticas simples, no un motor universal.
- Los nombres de comandos, menús y el estado grupal se encuentran en módulos separados, pero el acceso a SQLite y los permisos del socket aparecen mezclados en los ejecutores.

## 3. Diagnóstico de adaptación a Matter

**Sí adaptar:**

| Patrón real de Aura | Destino Matter propuesto | Tratamiento |
| --- | --- | --- |
| Handler que organiza ingreso y despacho | Base-Matter `InboundGateway` + `CommandRouter` | Reutilizar la secuencia conceptual; no copiar el router lineal |
| `CommandContext` compartido por comandos | Contrato de contexto de comandos de Base | Independizarlo de `MessagesSocket` y mensajes `WaMessage` |
| Categorías bajo `src/cmd/` | Catálogo de comandos y módulos de Bot-Matter cuando sea autorizado | Registro canónico con alias, permisos, ayuda y ciclo de vida |
| `BotState` con servicios compartidos | `MatterContext` / `ServiceRegistry` | Dividir servicios de núcleo, transporte y estado de plugins |
| Sub-bots y distintos sockets | `SessionRegistry` / `ConnectionService` / plataforma correspondiente | Identidades, permisos y credenciales aislados por socket |
| Moderación previa al despacho | Middleware/eventos de políticas en Base | Motor Anti auditable, modular y con herencia de ámbitos |
| `GroupRecord` y `UserRecord` | Registro de usuarios, política por chat/socket/ámbito | IDs canónicos autenticados, esquemas versionados |
| Cachés y semáforos | Servicios y límites por proveedor/instancia | Bounded caches con LRU/TTL e invalidación controlada |
| Idioma y respuestas | Traductor/i18n y transporte genérico | Mensajes neutrales al proveedor con adaptadores |
| Telemetría de comandos | `CommandAuditStore` y observabilidad | Distinguir intentos, denegaciones y efectos inciertos |
| Multimedia FFmpeg | Servicio multimedia independiente | Control de recursos, temporales privados, límites y compatibilidad |

**No copiar literalmente:**

1. **Despacho lineal `if/else if` por categoría.** Matter ya tiene `CommandRouter` y autorización de alias. Un registro por descriptor permite cargar plugins, consultar estado/help, verificar permisos y añadir lenguajes externos sin reescribir `handler.rs`.
2. **Propietarios o claves API definidos en `config.rs`.** Identidades globales deben venir de Vault autenticado y vincular plataformas; las credenciales deben permanecer fuera de código y Git. El hecho de que un JID o username aparezca en código no demuestra propiedad.
3. **Permisos basados en banderas simples `is_owner`, `bot_online` o `GroupRecord.antilink`.** Matter exige `=/> `independiente del bot, ámbito chat/sesión/global, `on|off|inherit`, permiso del socket, consentimiento y decisiones auditables.
4. **Acoplamiento de `CommandContext` a tipos WhatsApp.** Matter requiere mismo núcleo para Telegram, Discord y otros proveedores; `WaMessage` pertenece al adaptador WhatsApp.
5. **Borrado por defecto de sesiones, recuperación no verificada o reconexiones de otro proveedor.** No trasladar mecanismos que puedan desconectar el socket activo. La sesión WhatsApp de Matter se preserva y sólo se migra mediante operación autenticada y autorizada.
6. **`snipe` o captura de contenidos sin política de datos.** Definir consentimiento, retención, privacidad, administración por chat y borrado; evitar registros permanentes de contenido personal por defecto.
7. **Antilink por substrings como única detección** o listas estáticas de insultos como modelo central. Incorporar normalización, ediciones/eventos, excepciones y pruebas contra evasión. No interpretar revocación como prueba de que el escaneo de ediciones ya está implementado.
8. **Benchmark y soporte anunciado en README** como garantía de rendimiento real de Matter. Se requieren pruebas equivalentes, memoria RSS en cargas representativas y CI independiente.

## 4. Flujo objetivo Matter

```text
Proveedor autenticado (WhatsApp/Telegram/Discord/...)
         ↓
Adaptador normaliza mensaje/evento/autor/chat/socket
         ↓
InboundGateway: autenticación, antirreplay, consentimiento, estado de socket
         ↓
Control global =/> (independiente de los bots)
         ↓
Políticas Anti + permisos + auditoría por ámbito
         ↓
CommandRouter: comando canónico + alias + argumentos + descriptor
         ↓
CommandContext neutral a plataforma + servicios autorizados
         ↓
Handler Rust / contrato de plugin Rust, Python, C++, Go, JS...
         ↓
MessageService / TransportDriver de esa plataforma
         ↓
Resultado auditable + métricas + control de recursos
```

Base-Matter **ya documenta** `InboundGateway`, `CommandRouter`, `CommandAuditStore`, `MessageService`, `PluginHost` y el contrato de plataforma en:
[`docs/architecture.md`](architecture.md), [`docs/command-audit.md`](command-audit.md), [`docs/PLATFORM_CONTRACT.md`](PLATFORM_CONTRACT.md), [`docs/plugins.md`](plugins.md). El sandbox real y ciertos conectores de red **todavía no deben declararse implementados** solo porque exista el contrato.

**Separación de repositorios:**

- **Base-Matter:** normalización de comandos, registro de handlers, autorización, middleware, eventos, auditoría, contexto neutral y contratos.
- **WhatsApp-Matter:** normalización del tipo WhatsApp y su transporte Rust existente; pairing code; identidad del socket y servicios de grupo reales. No sustituir el proveedor actual por `Baileys-Rust` automáticamente.
- **Vault-Matter:** servicio autenticado para identidades, propietarios, autorizaciones, claves y políticas cifradas; ningún secreto en el repo público.
- **System-Matter/MSEL:** contratos de lenguaje, plugins y ABI/IPC cuando exista un entorno de aislamiento efectivo.
- **Bot-Matter:** catálogo y handlers de usuario final solamente **después de que se desbloquee** oficialmente. Se permite diseñar/documentar, no construirlo ni desplegarlo todavía.

## 5. Propuesta de estructura de módulos (NO existente todavía)

```text
Base-Matter/src/
  ingress/           # normalización, validación y recepción autenticada
  commands/
    descriptor.rs     # nombre canónico, alias, ámbito, permiso, argumentos, ayuda
    registry.rs       # carga controlada de comandos
    router.rs         # resolución, middleware y resultado
    context.rs        # contexto neutral a plataforma
  moderation/
    policy.rs         # contrato Anti y on/off/inherit
    events.rs         # mensajes, ediciones, revocaciones, medios
  observability/
    command_audit.rs  # intentos/resultados y telemetría saneada

WhatsApp-Matter/
  provider/           # adaptador hacia WhatsApp Rust actual
  handlers/           # grupo, medios y eventos nativos
```

Estas rutas son **ideas organizativas**, no instrucciones para duplicar módulos existentes. Antes de cualquier implementación, localizar los módulos reales del árbol Rust y extenderlos donde corresponda, sin renombrados disruptivos.

## 6. Fases futuras, sin intervenir el runtime ahora

1. **Inventario:** mapear los handlers reales de Matter y la tabla de comandos/alias existentes contra las 15 categorías Aura. Detectar colisiones; no copiar comandos ni categorías vacías.
2. **Especificación:** agregar a cada comando un descriptor `id`, `alias`, `scope`, `permission`, `platform_capabilities`, `args_schema`, `help`, `version` y `handler`. Diferenciar comando informativo y comando con estado.
3. **Contexto neutral:** definir acceso a `actor`, `session`, `chat`, `bot`, `message`, `reply`, `services` y `capabilities` mediante referencias validadas; no permitir que un plugin elija arbitrariamente identidades.
4. **Moderación:** `on|off|inherit`; capas chat → socket `--mod` → global `--global`; reportar estado al consultar sin argumentos. Probar captura de mensajes editados, camuflaje de comandos, enlaces, adjuntos y revocaciones según capacidades verificadas del proveedor.
5. **Pruebas:** casos positivos/negativos para prefijos, alias, menús, grupos desactivados, rol owner global autenticado, usuario ordinario, administración de chat, varios sockets y plataformas, eventos duplicados, race conditions y fallos de Vault. No modificar grupos activos ni sesiones reales.
6. **Integración por ramas:** implementar cuando se autorice en `workstreams/*` o rama de desarrollo aprobada, CI Linux/Windows, PR revisado hacia `alpha`, después `beta` y `stable` solo donde existan. Bot-Matter sigue bloqueado.

### Criterios de aceptación antes de considerar el diseño listo

- Un handler nuevo se registra sin tocar una cadena larga de `if/else`.
- Alias comparten exactamente las mismas políticas que su comando canónico.
- `=/> `se procesa independientemente de `#` y del estado del bot.
- El permiso global procede de identidad autenticada, no de `is_owner` calculado desde un identificador de texto.
- El estado Anti sin argumentos es lectura; operaciones `on|off|inherit` se auditan con su ámbito correcto.
- WhatsApp, Telegram y Discord pueden implementar el contrato sin arrastrar tipos específicos de WhatsApp al núcleo.
- Los errores de transporte no producen reintentos ciegos ni duplican acciones administrativas.
- No hay claves/sesiones en Git; no se promociona a una rama protegida sin revisión y pruebas.

## 7. Ollama y equipos modestos

**Ollama NO es dependencia de este refactor.** La separación handler/contexto/comandos/moderación se ejecuta en Rust sin descarga de modelos. Compilar Rust puede consumir RAM/CPU, pero instalar y ejecutar un LLM local es una carga distinta.

Si la laptop tiene poca RAM, **posponer Ollama**. Se puede seguir con diseño `.md`, comandos de Git, revisión de archivos y pruebas de Rust en etapas. La IA en servidor para programar de modo autónomo requiere un backend de inferencia (modelo local o API) cuando se active; este documento por sí solo no activa automatización.

## 8. Créditos y alcance

**Referencia técnica externa:** Aura-Reed-Rust, desarrollado por [LuferOS](https://github.com/LuferOS); repositorio y código de origen sujetos a su licencia declarada `Apache-2.0`. Esta nota describe patrones conceptuales, **no importa archivos fuente ajenos ni redistribuye credenciales**. Si en el futuro se reutiliza código, comprobar LICENSE y conservar atribuciones/avisos aplicables.

**Matter:**
Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

**Resultado de este cambio:** solo documentación; ninguna conexión, runtime, socket, comando, bot, credencial, clave o rama estable ha sido modificado.
