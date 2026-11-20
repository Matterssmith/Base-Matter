# Matter-Rust — núcleo universal de transporte

**Ubicación provisional:** `Base-Matter/crates/matter-rust`. El repositorio independiente `Matterssmith/Matter-Rust` aún no existe. La aplicación de GitHub conectada no ofrece creación de repositorios. Por eso el código está aislado en un crate Rust independiente, sin afectar los binarios existentes. Se podrá trasladar posteriormente conservando el contrato.

## Objetivo

**Un núcleo Rust, conectores separados.** Matter-Rust NO convierte la API de Telegram, Discord y WhatsApp en un único protocolo. Uniforma eventos, sockets, registro de conectores, controles previos y disponibilidad.

| Plataforma | Repositorio de transporte | SDK Rust propuesto | Estado de conexión mediante Matter-Rust |
| --- | --- | --- | --- |
| WhatsApp | Matterssmith/WhatsApp-Matter | `whatsapp-rust` nativo existente | Pendiente de integración; no reemplaza el proveedor en Base-Matter |
| Telegram | Matterssmith/Telegram-Matter | `teloxide` (Bot API) | Contrato definido; aún sin token ni polling/webhook |
| Discord | Matterssmith/Discord-Matter | `serenity` (Discord Gateway/HTTP) | Contrato definido; aún sin token ni gateway |
| Social-Matter | Matterssmith/Social-Matter (privado) | API propia futura | Reservado, **desactivado por defecto** |

Los SDK son propuestas por plataforma, no dependencias instaladas ni clientes en ejecución. Las funciones avanzadas de WhatsMeow/Baileys (botones, webviews, mensajes editados, comunidades, media, roles, llamadas) deben traducirse mediante adaptadores sin falsificar capacidades en Telegram o Discord.

## Capacidades implementadas en el crate

- Tipos de plataforma y nombres estrictos, incluyendo `SocialMatter` desactivado.
- Registro por identidad **plataforma + socket**, sin reemplazos automáticos.
- Comprobación de preparación de un conector inyectado por el sistema host.
- Ingreso de texto de observación **sin autoridad**: campos limitados, IDs validados, rechazo de datos binarios y duplicados entre mensajes de una sesión.
- Ventana de deduplicación limitada a memoria; no sustituye antireplay persistente de Vault.
- Aislamiento de eventos entre WhatsApp, Telegram y Discord.
- Mutaciones denegadas por defecto, hasta que Base-Matter confirme identidad y Vault-Matter autorice alcance.

## Aún NO implementado

- Conexión de cuentas WhatsApp/Telegram/Discord desde el nuevo crate.
- Recepción real de eventos, manejo seguro de secretos, rotación y persistencia de sesiones.
- Firma del contexto verificado y su transferencia al gateway de Base.
- Motor universal para mensajes multimedia, comandos, botones, polls, interacciones, moderación y permisos.
- Cualquier integración ejecutable con Social-Matter.
- Despliegue del crate en el servidor del bot.

No se debe afirmar que un `MockAdapter` de prueba representa un socket en producción. El ejecutable offline solo imprime los transportes planificados.

## Límites de seguridad

Un teléfono, JID, Telegram ID, Discord user ID o `@username` **no acredita la misma identidad en otras plataformas**. Las vinculaciones deberán hacerse por desafíos autenticados y decisiones de Vault, separadas de los roles. `owner --global` no se obtiene desde un mensaje ni desde Matter Register.

El transporte saliente real debe validar actor, destino, scope, versión de sesión, capacidad, limitación de tasa y antireplay. `Social-Matter` no puede activarse sin un adaptador firmado y políticas revisadas. No almacenar tokens o llaves en Git, archivos públicos, sesiones compartidas ni en el núcleo.

## Próximas fases

1. CI Linux/Windows de este crate y revisión de todas las pruebas.
2. Adaptador WhatsApp con sesión aislada, manteniendo `whatsapp-rust` actual operativo.
3. Adaptadores Telegram (`teloxide`) y Discord (`serenity`) con sus tokens entregados desde gestor de secretos, y eventos autenticados mediante el gateway correspondiente.
4. Integración con Base y Vault, aliases y verificación de roles entre plataformas.
5. SDK extensible para Social-Matter, sin conectarlo antes de tener servidor, API y autorización definidos.
6. Pruebas E2E por plataforma y despliegue controlado/rollback.

## Uso offline

```bash
cargo fmt --manifest-path crates/matter-rust/Cargo.toml --all -- --check
cargo test --manifest-path crates/matter-rust/Cargo.toml --all-targets
cargo run --manifest-path crates/matter-rust/Cargo.toml -- --status
```

El proyecto actual **no modifica el socket de WhatsApp en funcionamiento**.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
