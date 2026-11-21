## Selección de múltiples plataformas para crear bots (contrato v2)

`src/bot_factory.rs` añade una **declaración de proyecto** con `platforms: Vec<String>` y `adapters` por plataforma. Su plataforma predeterminada es **`social-matter`**, sistema `host` y lenguaje `msel`. Permite combinar Social-Matter, WhatsApp, Telegram y Discord en un bot, cada uno con su proveedor y sesión independientes.

Esto **no modifica** el enum `MatterPlatform` ni el enrutamiento existente: el enum de runtime actual sólo reconoce WhatsApp/Telegram/Discord, y registrar IDs adicionales requiere drivers reales. `social-matter` es por ahora un destino de creación **declarativo**, sin driver instalado ni privilegios. Los IDs custom también son opacos hasta aprobación del catálogo protegido.

**Separación**: plataforma = red/servicio del usuario; adaptador = implementación elegida (por ejemplo `baileys-rust`, `whatsmeow`, `xyzbaileys`); lenguaje = MSEL/Rust/otros; motor = Matter-MSEL/Matter-Rust/host externo. Ningún cambio de proveedor implícito ni copias de sesiones. Los adaptadores multi-plataforma se asignan por destino, no se heredan de uno a otro.

Ver [BOT_SETUP](BOT_SETUP.md) y [plan de prueba del proveedor](BAILEYS_RUST_EXPERIMENTS_BOT_FACTORY_PLAN.md).

# Base-Matter — contrato común de plataformas

El núcleo Rust controla las decisiones de autenticación, permisos, roles,
consentimiento, activación de sockets, auditoría, recepción y mensajería.
Cada plataforma tiene su propio adaptador y una credencial/sesión aislada.

## Registro de controladores

`MatterPlatform` reconoce los IDs canónicos `whatsapp`, `telegram`
y `discord`. `PlatformConnectionDrivers` implementa el contrato
`ConnectionDriver` y `PlatformTransports` implementa `TransportDriver`
de Base. Por tanto, `ConnectionService` y `MessageService` pueden
utilizar el mismo núcleo y el mismo mecanismo de autorización con diferentes
transportes instalados por el host. Los tres IDs están reservados: **registrar
un controlador no significa que ya exista su implementación externa**.

El registro rechaza plataformas no reconocidas, drivers no instalados y
registros duplicados. El transporte rechaza mensajes cuyo socket no coincide
con el socket de la sesión. No hay controlador de respaldo a otra plataforma.

## Límites obligatorios para cada adaptador

- Derivar el actor, socket y grupo de una conexión autenticada o un webhook
  verificado; nunca de texto proporcionado por el usuario.
- Usar `InboundGateway` para admisión, protección contra replay, política
  legal, activación y control global `=/>` antes del router normal.
- Aplicar `ConnectionService` y `MessageService` con políticas reales;
  el registro por sí solo no autoriza acciones ni verifica destinatarios.
- Revocar generaciones de conexión y operaciones pendientes cuando cambie
  el propietario. Evitar reenvíos automáticos después de errores inciertos.
- Mantener credenciales y sesiones fuera de Git y fuera de Base-Matter.
- Reportar funciones no admitidas por cada plataforma; no prometer
  uniformidad para capacidades que las APIs externas prohíban.

WhatsApp-Matter: Rust/`whatsapp-rust`, solo pairing code, sin QR,
con validación de propietario, reconexión y sesión privada.
Telegram-Matter: adaptador separado pendiente de implementación/pruebas.
Discord-Matter: adaptador separado pendiente de implementación/pruebas.

**Estado:** registro y pruebas de enrutamiento offline. Ninguna conexión
real adicional se declara implementada. No iniciar Bot-Matter.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
