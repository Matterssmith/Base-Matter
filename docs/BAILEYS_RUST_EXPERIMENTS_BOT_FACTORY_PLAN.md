# Base-Matter — Plan y auditoría: Baileys-Rust experimental + creador universal de bots

Fecha: 2026-11-21. Estado: **plan previo a implementación**. Rama `alpha` únicamente.

## Solicitud y decisiones permanentes

1. Examinar y preservar una copia íntegra, atribuida y sin activar de `LuferOS/Baileys-Rust/src/Utils/experiments.rs`. Verificar licencia, dependencias y compatibilidad con `whatsapp-rust` instalado. **No** pegarlo en el runtime de producción sin verificar tipos y pruebas reales.
2. Para creación de bots, el lenguaje **por defecto será MSEL** (integración `Matter-MSEL`); alternativas explícitas: `Rust` (`Matter-Rust`), Python, C++, C, Go, TypeScript, JavaScript y nuevas lenguas mediante contrato versionado de host. **No se obliga a instalar el compilador MSEL privado desde el Base público**.
3. Biblioteca o adaptador seleccionable, incluido uno propio identificado por un ID seguro, por ejemplo `xyzbaileys` (nombre mostrado `XYZBailyes`); opciones conocidas `baileys-rust`, `baileys`, `whatsmeow`, `whatsapp-rust`, etc. No otorgar acceso a una biblioteca solo por declararla.
4. Plataformas independientes de lenguaje y adaptador. Por defecto `social-matter`. Permitir seleccionar `whatsapp`, `telegram`, `discord` y **varias plataformas juntas**. Cada plataforma resuelve su adaptador con su propio registro de capacidades y permisos.
5. Todo lo anterior será selección **offline y declarativa**, con aprobación exclusiva de un catálogo autenticado del Host. No abrir sesiones, emparejar dispositivos, tocar Vault, instalar binarios ni migrar conexiones actuales por seleccionar una opción.
6. Preservar `stable`/`beta` y el proveedor WhatsApp existente: este trabajo no debe afectar el socket vivo ni el menú, roles o antis.

## Examen de la referencia (lectura efectuada antes del plan)

Repositorio externo: https://github.com/LuferOS/Baileys-Rust
Ruta: `src/Utils/experiments.rs`, blob Git `d9ce040e2c7f0d5b06950860bca9eae594f49651`.
Licencia: **Apache-2.0**. `NOTICE` mantiene atribución a **Luis Fernando Guzmán Niño (LuferOS)**.

Se observan: `NativeButtonInput` (quick_reply, cta_url, cta_copy, cta_call, open_webview, single_select, galaxy_message, custom), `InteractiveInput`, `RichResponseInput`, `RichBlock` (texto, código, tabla, LaTeX), carruseles y creación de nodos auxiliares `biz`/`bot`. Construye estructuras protobuf del crate **baileys-rs**, no un renderer HTML arbitrario. `open_webview` abre una **URL**, no ejecuta HTML contenido en el mensaje. La visualización en clientes reales debe comprobarse; puede fallar o ser ignorada.

**Desajuste confirmado:** Base-Matter usa `whatsapp-rust =0.7.0` con tipos protobuf `wa`/`buffa`; LuferOS usa `baileys-rs`, `crate::proto`/`prost` y nodos binarios propios. Copiar la implementación directamente al provider activo **no es compilable tal cual** ni demuestra que soporte HTML dentro de WhatsApp. El experimento existente de Base es una vista HTML offline y transporte RichResponse opt-in del dueño, **sin resultado visual confirmado**.

## Rutas previstas y secuencia

**Primero documentación** (este archivo, antes de código) y anexo MSEL `docs/CHANGE_PLAN.md`.

**Copia aislada legal y revisable**:
- `third_party/luferos-baileys-rust/experiments.rs`: copia idéntica de la referencia, sin compilación.
- `third_party/luferos-baileys-rust/LICENSE`, `NOTICE`, `PROVENANCE.md`: licencia/atribución y limitación técnica.
- Ninguna referencia a `0@bot`, suplantación Meta AI o forwards falsos en pruebas reales.

**Contrato creador**:
- `src/bot_setup.rs`: lenguaje por defecto `msel` en esquema v2, aceptación explícita `rust`; mantener default `rust` únicamente en selector legacy v1 para no romper integraciones previas; esquema v2 de creador de proyectos, `platforms: Vec`, selección de adaptadores por plataforma, rechazo de duplicados/IDs inseguros y ningún grant implícito. Mantener el esquema legacy v1 con su forma de JSON original.
- `src/bin/bot_init.rs`: selector CLI con `--platform` repetible, `--provider-for plataforma=ID`, `--language`, `--system` y `--provider` genérico. Valor por defecto `social-matter`.
- `docs/BOT_SETUP.md` y `docs/PLATFORM_CONTRACT.md`: rutas de creación y estados de proveedores, sin prometer conexiones. `docs/WHATSAPP_RICH_HTML_COMPAT.md`: resultado de auditoría y fallback.
- `README.md`, `docs/DEVELOPMENT_STATUS.md` y `docs/DECISION_LOG.md`: estado, pruebas y diferencias con stable.

**Pruebas requeridas**: construir/test Rust cuando exista Cargo; mientras tanto inspección de código y tests estáticos no sustituyen compilación. Incluir tests del selector MSEL/social-matter, Rust explícito, WhatsApp/Telegram/Discord simultáneos, adaptadores propios con ID seguro, rechazo de URLs/path traversal, duplicados, plataformas no reconocidas, y preservación del esquema antiguo. El provider WhatsApp con `whatsapp-rust` no se tocará en esta fase.

## Pendientes tras esta fase

Adaptar realmente estructuras de mensajes de LuferOS al provider actual con mapping protobuf/ABI, pruebas de envío y **renderizado verificado** en Android/iOS/Web; decidir si se requiere biblioteca externa bajo licencia y condiciones oficiales. Los plugins Social-Matter, Matter-MSEL, Matter-Rust y custom libraries requerirán Host autorizado. Priorizar después el lenguaje **MSEL P0**, su motor Matter-MSEL y su contrato de aplicaciones sin duplicar el código privado del lenguaje en Base público.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
