# Matter-MSEL
## Lenguaje Matter — Matter Script Extension Language

> **Prioridad P0 del Matter Ecosystem.** Matter-MSEL es el lenguaje nativo y motor de desarrollo principal para bots y sistemas. Su definición, frontend, MSEL IR, runtime y contratos básicos se planifican **antes de nuevas ampliaciones arquitectónicas de Base-Matter**. Los servicios existentes de Base-Matter y Vault-Matter continúan bajo sus reglas de seguridad; esta prioridad no deshabilita controles existentes ni afirma que MSEL ya se ejecute.

**Estado:** especificación y hoja de ruta; **no** se declara implementado el compilador, el intérprete, las 32 integraciones ni las APIs sin pruebas.  
**Origen:** definiciones de Matter-MSEL y matriz de 32 entradas tecnológicas del propietario, 2026-11-21.  
**Propiedad intelectual / créditos:** Hecho por Lournyx Matterssmith [Lou-Nyx].

## 1. Principio y objetivo

**MSEL es el lenguaje Matter.** Matter-MSEL designa su implementación en el ecosistema. Un programa en MSEL puede expresarse en distintos idiomas humanos; el núcleo semántico, los tipos, el enlace de módulos y la representación intermedia son comunes. La prioridad inicial de producto es poder crear **bots y sistemas** con MSEL; posteriormente CLI, TUI, GUI, Web, escritorio, móvil, servicios, APIs, distribuidos y embedded.

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
Matter Runtime + SDK / Host independiente
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

## 5. Bots Matter: primer objetivo funcional

**El primer caso de uso real debe ser un bot escrito en MSEL** y ejecutado por un host Matter. MSEL deberá expresar:

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

## 7. Orden de ejecución de desarrollo

| Etapa | Prioridad | Entregable verificable |
| --- | --- | --- |
| M0 | P0 | Especificación de gramática, tipos, IR y permisos; decisiones de lenguaje versionadas |
| M1 | P0 | Workspace Rust `msel-lexer`, `msel-parser`, `msel-ir`; tests unitarios |
| M2 | P0 | Frontends es/en; idéntica IR para dos programas equivalentes y archivos multilingües |
| M3 | P0 | Intérprete mínimo/VM, biblioteca estándar de I/O restringida, CLI `run/check/test` |
| M4 | P0 | SDK/host de bots: eventos, comandos, sandbox, permisos, proveedor WhatsApp |
| M5 | P1 | Integración estable Base-Matter con MSEL sin ruptura de identidades ni Vault |
| M6 | P1 | Backends e interop prioritarios Rust/C/C++/Python/Go/JS, según compatibilidad |
| M7 | P2 | Adaptadores SQL/configuración/multimedia/GUI/móvil/GPU y otras plataformas |
| M8 | P2 | Ecosistema oficial de paquetes lingüísticos y SDKs; conformidad y documentación |

**Puerta de salida mínima para M4:** dos versiones lingüísticas de un bot compilan a IR equivalente; un bot responde en WhatsApp mediante eventos reales; se prueban permisos denegados, errores de idioma, aislamiento de sockets, CI en Linux y Windows y persistencia del estado si aplica. Ninguna etapa se da por terminada sin evidencias.

**Dependencias ya desplegadas:** las correcciones operativas y de seguridad actuales de Base-Matter y Vault-Matter se conservan. “MSEL antes de Base-Matter” significa **prioridad de arquitectura y nueva implementación**, no eliminar o dejar sin mantenimiento servicios activos.

## 8. Reglas permanentes de continuidad

- Todos los cambios nuevos de arquitectura Matter deben comprobar primero si corresponden al lenguaje MSEL, al runtime, al SDK, a Base-Matter o a un proveedor.
- Primero lenguaje/contrato MSEL; después API del host; finalmente implementación por plataformas.
- Registrar por cambio: (1) resumen, (2) archivos, (3) pruebas, (4) pendientes, (5) siguiente etapa, con SHA remoto cuando exista.
- Revisar los README afectados antes de cada commit. No afirmar código funcional sin pruebas y CI.
- Preservar la autoría y créditos de forma discreta, sin línea duplicada de “Autor”.

---

Créditos:  
Hecho por Lournyx Matterssmith [Lou-Nyx].
