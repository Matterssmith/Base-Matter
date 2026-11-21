# Base-Matter — Plan y auditoría: Baileys-Rust experimental + creador universal de bots

Fecha: 2026-11-21. Estado: **plan previo publicado y primera implementación descrita abajo**. Rama `alpha` únicamente.

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

## Resultado tras implementación — revisión 2026-11-21

**Código de referencia copiado y VERIFICADO:** `third_party/luferos-baileys-rust/experiments.rs`, `LICENSE` y `NOTICE`. Los blobs coinciden exactamente con los de LuferOS en el commit fijado; sin editar y fuera de Cargo. Referencia guardada en commits `5675d6a3c612b4b55a049ed798e35b9d86b8b889`, `4de3982ccb2c11cd32d077575c6f389a1c8e9d66`, `e2c3c31f04a5ffdc3c5c38ffc61476643b98ce2f`. Procedencia en `PROVENANCE.md`.

**Selector v2 publicado en alpha:** `src/bot_factory.rs` (commit `f141750ea34127395515afc44436740cef42c774`), exportación `src/lib.rs` (`aa80a907a24d8e9a1b269a3831c66872f019ddd4`), CLI `src/bin/bot_init.rs` (`6face1d569e6f4c4b54f343938f3956ceab7b4db`). MSEL/Social-Matter por defecto, Matter-Rust/Rust explícito, proveedores opacos y lista de múltiples plataformas con adaptadores individuales. El selector v1 se preserva sin cambio de default (Rust) y se invoca con `--legacy-selector`.

**Pruebas actuales:** 8 pruebas unitarias Rust nuevas añadidas en `src/bot_factory.rs`, **no ejecutadas con Cargo todavía** en esta sesión: Cargo/Rustc no están disponibles en el runtime de herramientas. No confundir inspección estática ni SHA exacto de los archivos externos con compilación Rust, envío real WhatsApp, UI renderizada ni conexión Social-Matter.

**Próximo requisito verificable:** ejecutar `cargo fmt --check`, `cargo check --all-targets --locked`, `cargo test --all-targets --locked`, CI de Linux y Windows; probar CLI `matter-bot-init` con combinaciones; documentar fallos y no promover `stable`. Para experimentos HTML, primero adapter protobuf desacoplado y test real en self-chat del propietario con fallback legible; no alterar el proveedor activo automáticamente.

**Decisión final:** el código de LuferOS se conserva como referencia externa licenciada, no como capa del lenguaje MSEL. La siguiente gran prioridad vuelve a MSEL (lenguaje P0) y Matter-MSEL (motor P1).


Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
