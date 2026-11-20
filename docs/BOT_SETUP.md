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
