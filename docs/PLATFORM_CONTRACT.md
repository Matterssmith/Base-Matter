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
