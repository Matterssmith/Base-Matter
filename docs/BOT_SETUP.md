## Contrato v2 — Bot Factory universal (2026-11-21)

El **nuevo** creador `matter-bot-init` devuelve un descriptor **v2**, no un bot ejecutable. Sus valores por defecto son: **lenguaje `msel`**, **motor `matter-msel`**, **plataforma `social-matter`**, **biblioteca `default`** y **sistema `host`**. La aplicación Social-Matter, la lógica `.msel` y el compilador privado no se incorporan al repositorio público.

**Elecciones independientes y combinables**: 
- Lenguaje de código: `msel` (Matter-MSEL), `rust` (Matter-Rust), o `python`, `cpp`, `c`, `go`, `typescript`, `javascript`, `lua`, `kotlin`, `swift` como destinos declarativos que requieren motor instalado.
- Plataforma: `social-matter` (predeterminada), `whatsapp`, `telegram`, `discord`; otras plataformas pueden declararse mediante ID seguro, pero el host puede rechazarlas si no están instaladas.
- Biblioteca/proveedor: `--provider` (un destino) o `--provider-for plataforma=biblioteca` para asociaciones distintas. Ejemplo de nombre propio `XYZBailyes`: ID normalizado a `xyzbaileys`; **no** se ejecuta directamente el texto recibido ni una URL arbitraria.
- Sistema objetivo: `host`, `linux`, `windows`, `macos`, `android`, `ios`, `web` o `embedded`, **sin presumir builds existentes**.

```bash
# Bot nuevo: MSEL + Social-Matter (solo descriptor)
cargo run --locked --bin matter-bot-init

# Bot en Matter-Rust usando biblioteca WhatsApp elegida
cargo run --locked --bin matter-bot-init -- --language rust --platform whatsapp --provider baileys-rust

# Bot MSEL para cuatro plataformas, con bibliotecas independientes
cargo run --locked --bin matter-bot-init -- --language msel --platform social-matter --platform whatsapp --platform telegram --platform discord --provider-for whatsapp=xyzbaileys --provider-for telegram=telegram-api

# Contrato antiguo, desactivado en el nuevo CLI salvo elección explícita
cargo run --locked --bin matter-bot-init -- --legacy-selector
```

**Invariante de seguridad**: el selector `BotProjectRequest` valida identificadores opacos, destinos únicos, adaptadores dentro de las plataformas solicitadas y prohibición de asignar una biblioteca global no predeterminada a varios transportes. `UnavailableBotFactoryCatalog` impide resolver nada sin Host autorizado. El catálogo instalado valida proveedor, versión, permisos y capacidades; **ningún selector concede credenciales, enlaces de repositorio privado, sockets ni roles**.

**Compatibilidad:** `BotSetupRequest` v1 y su predeterminado histórico `rust` siguen intactos para consumidores antiguos. El nuevo flujo v2 y el CLI por defecto sí usan MSEL. El argumento `--legacy-selector` reproduce la selección v1 sin campos de plataformas. Es deliberado para no romper servicios previamente desplegados.

**Estado:** módulo Rust `src/bot_factory.rs` y CLI v2 publicados en `alpha`; están escritos **8 tests unitarios de Rust**, pero **compilación y ejecución Cargo no confirmadas** en esta iteración. No hay launcher real, pairing, conexión WhatsApp/Telegram/Discord ni integración MSEL alojada confirmada. Ver [plan/auditoría](BAILEYS_RUST_EXPERIMENTS_BOT_FACTORY_PLAN.md).

## 2026-11-19 — Política obligatoria Vault-D019: protección equivalente a credenciales

- **Regla permanente del ecosistema:** TODOS los activos administrados por Vault — roles, permisos, decisiones, credenciales, políticas, metadatos, evidencia, auditoría, caché, respaldos y estados — deben cifrarse/autenticarse en reposo y transmitirse por canales cifrados y autenticados. Descifrado temporal sólo en memoria confiable; claves fuera de Git, nunca textos sensibles en logs, respuestas públicas ni archivos temporales.
- Base pública: sin rutas, URLs ni dependencias a repositorios privados. El selector provider/language/system y los puentes emiten solicitudes sin credenciales; todo dato o decisión privada del servicio Vault deberá viajar por un canal autenticado y cifrado, y jamás se persistirá en claro. No cambiar el código de runtime hasta existir gateway verificado. Al recibir un resultado mínimo autorizado, no volcarlo a logs, tests ni mensajes. `alpha` solamente; beta/stable intactos.
- La norma principal se mantiene en la política privada VAULT-D019. Esta actualización es documental y no acredita gateway Vault, ejecución privada o migración de sesiones. No conceder permisos sin verificación y no afirmar pruebas funcionales.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

# Base-Matter — Creación neutral de bots (contrato público)

Estado: selector de configuración offline, NO autorización ni ejecución de proveedores externos.

## Elección de proveedor, lenguaje y sistema

El núcleo permite seleccionar tres dimensiones independientes: `provider`, `language` y `system`.

Valores por defecto:
- **provider:** `default` (identificador virtual; un catálogo protegido debe resolver a un proveedor instalado y autorizado).
- **language:** `rust`.
- **system:** `host` (sistema del host real, no una promesa de soporte universal).

Ejemplo sin opciones: `cargo run --locked --bin matter-bot-init`
Ejemplo explícito: `cargo run --locked --bin matter-bot-init -- --provider family-example --language go --system windows`

La salida JSON es **solo una declaración de elección**: no genera un bot desplegable, no instala binarios, no concede permisos, no abre sesiones ni conecta WhatsApp. Las lenguas listadas son destinos de integración a planificar, NO afirmaciones de que cada SDK ya funciona. La ausencia de registro autorizado devuelve `Unavailable`.

## API Rust pública

`matter_base::bot_setup::{BotSetupRequest, BotSetupSelection, ProtectedBotSetupCatalog, resolve_setup}`.

Un creador puede establecer sus propios nombres de comandos y parámetros. El host autorizado ofrece una implementación de `ProtectedBotSetupCatalog`; Base valida la selección y el catálogo realiza la resolución bajo la autoridad protegida. **El código público no contiene nombres ni URLs de repositorios privados, rutas de código fuente, credenciales o llaves.** El ID `default` no significa permisos predeterminados.

Una implementación privada nunca se importa mediante una dependencia Cargo visible para el creador. Un ejecutor controlado por el host, con autorización revocable, aislamiento por bot y socket, límites y auditoría, realiza únicamente las operaciones permitidas. Un módulo `.rs` público solo describe cómo solicitar una operación; no ofrece la implementación privada.

## Migración y seguridad

El lanzador anterior y el proveedor activo se mantienen hasta que el ejecutor autorizado y un proveedor versionado superen pruebas y migración de estado. No arrancar dos instancias de un mismo almacén de sesión. La consulta sin argumentos no debe mutar el bot.

Este contrato no modifica el proveedor activo, el menú, la política Anti, los roles ni las ramas beta/stable. Antes del corte real se requieren CI Linux/Windows, selección de proveedor instalada, capacidad de conexión real, pairing por código, regresión HMAC/replay y plan de reversión.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
