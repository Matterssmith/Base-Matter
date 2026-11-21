# MSEL — lenguaje oficial de Matter Ecosystem
## Matter-MSEL — integración del lenguaje MSEL con Matter

> **Prioridad P0: hacer MSEL funcional.** El nombre del lenguaje es **MSEL**, no «Matter-MSEL». Su sintaxis, tipos, lexer, parser, semántica, IR, intérprete/compilador, CLI y pruebas se desarrollan primero, sin depender de Base-Matter. **Matter-MSEL** es la capa posterior de integración con el ecosistema, análoga al soporte Matter-Rust: contratos, host, SDK y adaptadores para que los bots y sistemas Matter ejecuten programas escritos en MSEL. La estabilidad y seguridad de Base-Matter y Vault-Matter se siguen manteniendo.

**Estado:** especificación y hoja de ruta de MSEL; **no** se declara implementado el compilador, el intérprete, las 32 integraciones ni las APIs sin pruebas.  
**Origen:** definiciones de Matter-MSEL y matriz de 32 entradas tecnológicas del propietario, 2026-11-21.  
**Propiedad intelectual / créditos:** Hecho por Lournyx Matterssmith [Lou-Nyx].

## 1. Principio y objetivo

**MSEL es el lenguaje oficial Matter.** **Matter-MSEL** designa únicamente la integración de MSEL en Matter Ecosystem, comparable a Matter-Rust; no es un idioma alternativo, otro compilador ni el nombre del núcleo del lenguaje. Un programa en MSEL puede expresarse en distintos idiomas humanos; el núcleo semántico, los tipos, el enlace de módulos y la representación intermedia son comunes. La prioridad inicial de producto es poder crear **bots y sistemas** con MSEL; posteriormente CLI, TUI, GUI, Web, escritorio, móvil, servicios, APIs, distribuidos y embedded.

No se crearán lenguajes diferentes para español, inglés o japonés: todos son MSEL con paquetes léxicos/lingüísticos. MSEL tampoco pretende reimplementar literalmente todos los lenguajes externos: integra sus capacidades mediante backends, FFI, bindings, bibliotecas, serialización y adaptadores específicos.

### Arquitectura canónica

```text
Código .msel / .<idioma>.msel
          ↓
Resolver de idioma / paquetes lingüísticos
          ↓
Lexer + parser + AST tipado + análisis semántico
          ↓
MSEL IR versionado (con source maps)
          ↓
Optimizaciones + verificador + gestor de capacidades
          ↓
 ┌────────┴───────────────┐
 │                       │
Intérprete/VM MSEL    Backends / FFI / adaptadores
 │                       │
 └────────┬───────────────┘
          ↓
Runtime MSEL autónomo / SDK propio
          ↓
Matter-MSEL (adaptador opcional al ecosistema)
          ↓
Base-Matter (host de bots) · System-Matter · APIs de plataforma
          ↓
Sistema operativo / hardware, según permisos reales
```

**Separación de responsabilidades:** MSEL puede funcionar de forma autónoma como lenguaje y runtime con contrato estable. Base-Matter debe usarlo como motor de scripts y plugins de bot, no ser una dependencia obligatoria para ejecutar cualquier programa. Matter System coordina servicios, no es una etapa secuencial necesaria para cada ejecución.

## 2. Extensiones e idioma

- `programa.msel`: código neutral respecto de la extensión; resolver idioma por contexto.
- `programa.es.msel`, `programa.en.msel`, `programa.ja.msel`, `programa.de.msel`, `programa.fr.msel`, `programa.pt-BR.msel`: extensión explícita.
- Los alias solicitados `jp`, `kr`, `br` pueden aceptarse por compatibilidad, normalizados a `ja`, `ko`, `pt-BR`; preferir etiquetas BCP 47.
- Resolver: (1) sufijo explícito, (2) idioma del proyecto, (3) metadatos del archivo, (4) entorno, (5) preferencia de usuario, (6) detección con umbral de confianza, (7) predeterminado del proyecto; ante ambigüedad solicitar selección.
- En caso de dos declaraciones explícitas incompatibles (p. ej., sufijo es y metadatos en), reportar diagnóstico; no reinterpretar silenciosamente el código.
- El idioma modifica la expresión superficial, **no** la semántica de tipos, operadores, permisos ni runtime.
- Proyectos multilingües admiten diferentes sufijos; todos los módulos comparten MSEL IR y símbolos canónicos.
- Traducción entre idiomas: reemitir AST/IR semánticamente equivalente; conservar nombres de variables, cadenas y comentarios salvo instrucción del autor para traducirlos.

### Ejemplo conceptual (sintaxis provisional, aún no garantizada por parser)

```msel
si usuario existe entonces
    mostrar "Hola"
fin
```

La gramática formal, los escapes, precedencia de operadores y delimitadores deben especificarse antes de aceptar esta sintaxis como estable.

## 3. Matriz de 32 entradas — categorías y capacidades

La lista original contiene **32 posiciones**, con **Rust repetido** en principales y secundarios. No todas las entradas son lenguajes de programación: algunas son runtimes, formatos, frameworks, motores o APIs. Se preservan las categorías solicitadas y se asigna un mecanismo técnico verificable.

| Grupo | Entrada | Incorporación prevista a MSEL |
| --- | --- | --- |
| Principales | Python | API, módulos, automatización y bindings Python; puente seguro a CPython |
| Principales | C++ | Compilación nativa, FFI C ABI, bibliotecas, objetos vía bindings |
| Principales | Rust | Implementación del núcleo MSEL, backend/interoperabilidad, memoria segura |
| Secundarios | C | ABI, syscalls y bibliotecas nativas sujetas a capacidades |
| Secundarios | Rust (reiterado) | Prioridad transversal para compilador y runtime, no tecnología duplicada |
| Secundarios | Go | Interoperabilidad por ABI/procesos/protocolos; servicios concurrentes |
| Terciarios | Node.js / JavaScript | Runtime JS, bindings, módulos, backend web/bots |
| Terciarios | Bash | Adaptador de shell controlado, subprocessos con argumentos estructurados |
| Terciarios | PowerShell | Integración administrada Windows/.NET y shell restringido |
| Terciarios | Lua | Scripting integrado mediante VM o binding controlado |
| Terciarios | Embedded Scripts | Perfil embebido con recursos/capacidades limitadas |
| Cuartarios | SQL | Consultas tipadas, parámetros y adaptadores de bases de datos |
| Cuartarios | XML | Parser/serializer, validación de esquemas |
| Cuartarios | JSON | Parser/serializer, esquemas y mensajes |
| Cuartarios | YAML | Configuración, parser seguro y validación |
| Cuartarios | FFmpeg Scripts | Construcción/ejecución de grafos de audio y video vía FFmpeg |
| Cuartarios | GStreamer Pipelines | Grafos multimedia tipados y adaptador GStreamer |
| Quintarios | C# (Unity / .NET) | Interoperabilidad CLR/.NET; integración Unity mediante SDK |
| Quintarios | Java | Interoperabilidad JVM y clases/paquetes Java |
| Quintarios | Kotlin | Interoperabilidad JVM y Android Kotlin APIs |
| Quintarios | Swift | Bindings nativos Apple; toolchain por plataforma |
| Quintarios | Objective-C | Interoperabilidad Objective-C runtime/Cocoa |
| Quintarios | Vulkan Shaders | Generación/consumo de shaders SPIR-V con validación |
| Quintarios | Metal Shading Language (MSL) | Backend shader Metal/macOS/iOS donde esté disponible |
| Sexarios | HTML | Construcción segura del DOM/plantillas o serialización HTML |
| Sexarios | CSS | Estilos, reglas y generación CSS |
| Sexarios | SVG | Gráficos vectoriales y serialización segura |
| Sexarios | OpenGL | Adaptador de renderizado OpenGL/GLSL |
| Sexarios | DirectX | Adaptador DirectX/HLSL de Windows |
| Sexarios | Cairo | Biblioteca de gráficos 2D |
| Sexarios | Qt Quick (QML) | Bindings Qt/QML para UI declarativa |
| Sexarios | ImGui | UI inmediata vía bindings nativos |

**Alcance de la palabra “integrar”:** ofrecer contratos, herramientas, ejemplos y tests de interoperabilidad; no prometer compatibilidad idéntica con cada característica o plataforma. Los backends y adaptadores informarán explícitamente qué subconjunto soportan.

## 4. MSEL IR y compatibilidad

- IR versionado y documentado: módulos, funciones, tipos primitivos/compuestos, control de flujo, errores, efectos, llamadas, símbolos y origen de posiciones.
- Paquetes idiomáticos se normalizan antes de la IR; ninguna IR incorpora tokens traducidos como semántica.
- Formato IR legible para diagnóstico y formato binario opcional, con verificador, límites y compatibilidad de versión declarada.
- Sistema de capacidades/efectos: filesystem, network, subprocess, FFI, db, camera, microphone, GPU, device, admin y secretos, por identidad y contexto.
- Interoperabilidad ABI/IPC sin asumir que compilar a C/JS/Python da automáticamente todas sus funciones.
- Portabilidad condicionada por toolchain y APIs disponibles: Windows, Linux, Android, macOS, iOS y embedded.
- Cada backend tendrá matriz: soportado, parcial, experimental o no soportado, con pruebas.
- CLI propuesta: `msel init`, `msel check`, `msel run`, `msel build`, `msel test`, `msel fmt`, `msel translate`, `msel doctor`. Son comandos **planeados**, no existentes.

## 5. Bots Matter: primera integración después de MSEL funcional

**La primera meta P0 es ejecutar programas MSEL autónomos y pasar su suite de conformidad, sin host Matter.** El primer consumidor posterior será un bot escrito en MSEL, ejecutado mediante Matter-MSEL por un host Matter. Para esa fase MSEL deberá expresar:

1. Eventos de mensaje, edición, eliminación, reacción, grupo, usuario y sistema.
2. Registro, descubrimiento y ejecución de comandos con argumentos y ayuda.
3. Contexto por plataforma, bot, socket, chat y usuario; identidades verificadas por proveedores.
4. Roles, permisos y políticas consultados al host; ningún script eleva permisos por sí mismo.
5. Sesiones y estado persistente mediante interfaces declaradas y drivers autorizados.
6. Plugins, SDK, importación de módulos y sandbox opcional con cuotas.
7. WhatsApp primero como caso de prueba; Telegram y Discord mediante sus propios proveedores.
8. Compatibilidad progresiva con bots externos TS/JS, Go, Rust, Python y otros mediante contratos.

**Convenciones Matter existentes:** el prefijo normal de bots es `#`; el control global `=/>` pertenece a la autoridad del host y a identidades verificadas, no a cualquier script MSEL. Los controles antispam/antis, edición de mensajes, niveles de permiso, roles globales y autenticación de socket siguen supervisados por Base/Vault. Los scripts no leen claves, tokens ni sesiones en claro por defecto.

## 6. Seguridad y Vault

- Denegación por defecto de red, procesos, sistema de archivos ajeno, FFI arbitrario, acceso administrativo y dispositivos.
- Las capacidades se otorgan explícitamente por contexto, registran auditoría y pueden revocarse.
- Bot-Matter y Base-Matter deben consumir Vault mediante el servicio autenticado: **nunca** incluir llave maestra en código, scripts o repositorios, públicos o privados.
- No usar nombres de usuario recibidos como texto ni números declarados en chat para autenticación sin validación de proveedor y Vault.
- Paquetes lingüísticos externos descargables sólo con manifiesto, integridad, firma/confianza configurada, versión compatible y ejecución aislada; no adquieren estatus oficial por descargar.
- Ningún backend puede eludir controles de permisos de la IR/runtime o las políticas del sistema operativo.

## 7. Prioridad real y orden de ejecución

**No confundir prioridad de lenguaje con prioridad de integración.** Los entregables P0 deben funcionar localmente sin WhatsApp, sin sockets, sin Vault y sin Base-Matter. Matter-MSEL empieza **después** de que el lenguaje esté operativo y sus tests principales estén en verde.

| Fase | Prioridad | Resultado verificable |
| --- | --- | --- |
| L0 — Diseño formal | **P0 · MSEL** | Gramática versionada, tokens, reglas de idioma, tipos, semántica, errores y especificación de IR v0 |
| L1 — Frontend | **P0 · MSEL** | Lexer y parser en Rust; AST, localización es/en, tests válidos e inválidos |
| L2 — Núcleo | **P0 · MSEL** | Resolución de módulos, símbolos, verificación de tipos y MSEL IR con posiciones de origen |
| L3 — Ejecución | **P0 · MSEL** | Intérprete/VM o compilador mínimo, biblioteca estándar segura, CLI msel check/run/test |
| L4 — Lenguaje funcional | **P0 · MSEL** | Programa real en .msel, .es.msel y .en.msel; funciones, control de flujo, módulos, errores, E/S permitida y CI verde |
| I0 — Integración Matter | P1 · Matter-MSEL | SDK/adaptador con contratos versionados para consumir el lenguaje MSEL desde hosts Matter |
| I1 — Bots Matter | P1 · Matter-MSEL + Base-Matter | Bot MSEL con eventos y comandos probados en proveedor real, preservando autorización del host/Vault |
| I2 — Interoperabilidad principal | P1 · Adaptadores | FFI y contratos con Python, C++, Rust, C, Go y Node.js/JavaScript con tests por tecnología |
| I3 — Matriz extendida | P2 · Adaptadores | Shell, multimedia, datos, JVM/.NET, Apple, gráficos, UI y demás tecnologías de la matriz de 32 entradas |
| I4 — Ecosistema | P2 · SDK | Paquetes de idioma adicionales, herramientas y conformidad multiplataforma |

**Criterios obligatorios para declarar MSEL funcional (cierre L4):**

1. Ejecutar un programa MSEL **sin** depender de Base-Matter ni de Matter-MSEL.
2. El comando msel run produce resultados observables; msel check informa errores con archivo, línea y columna.
3. Programas equivalentes en español e inglés generan el mismo comportamiento y una IR normalizada semánticamente equivalente.
4. Control de flujo, funciones, variables, tipos, módulos e importaciones tienen pruebas unitarias e integración.
5. Resolver idiomas .msel y .<idioma>.msel de forma determinista y diagnosticar ambigüedades/conflictos.
6. Respetar permisos y capacidades del proceso anfitrión; no ofrecer acceso arbitrario a secretos, red o procesos por defecto.
7. Ejecutar tests reproducibles en Linux y Windows cuando exista soporte CI, registrando cobertura, limitaciones y SHA.

**Dependencias existentes:** mantener correcciones de seguridad y operación de Base-Matter/Vault-Matter es compatible con priorizar MSEL. Ni la integración de bots ni las 32 tecnologías se declaran terminadas hasta probarlas.

## 8. Reglas permanentes de continuidad

- Todos los cambios nuevos de arquitectura Matter deben comprobar primero si corresponden al lenguaje MSEL, al runtime, al SDK, a Base-Matter o a un proveedor.
- Primero **MSEL como lenguaje autónomo funcional** (P0); después **Matter-MSEL como integración** (P1); luego bots, plataformas y adaptadores. No utilizar «Matter-MSEL» como sinónimo de MSEL.
- Registrar por cambio: (1) resumen, (2) archivos, (3) pruebas, (4) pendientes, (5) siguiente etapa, con SHA remoto cuando exista.
- Revisar los README afectados antes de cada commit. No afirmar código funcional sin pruebas y CI.
- Preservar la autoría y créditos de forma discreta, sin línea duplicada de “Autor”.

---

Créditos:  
Hecho por Lournyx Matterssmith [Lou-Nyx].
