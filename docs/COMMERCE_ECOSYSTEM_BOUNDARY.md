# Base-Matter — límites de integración con Marketplace-Matter

**Estado:** planificación únicamente. **Fecha:** 2026-11-19. **Requisitos relacionados:** MAT-COM-003,007,008,013,014,017,018.
Documento central: https://github.com/Matterssmith/Marketplace-Matter/blob/matter/docs/REQUISITOS_ECOSISTEMA_COMERCIO.md

## Responsabilidad de Base (no del comercio)

Base-Matter **deberá proporcionar únicamente contratos neutrales y extensibles** para eventos, permisos y sesión, si el comercio necesita integrarlos. El dominio comercial pertenece a Marketplace-Matter; las decisiones de propiedad civil y titularidad de aparatos pertenecen a Vault-Matter.

- Evitar introducir `Order`, impuestos, precios, rutas gratuitas o estados de pago en el core universal.
- Los eventos de comercios deben validarse por identidad, scope, versión, marca temporal, idempotencia y política de permisos del consumidor; no utilizar texto libre, menciones de WhatsApp o IDs escritos para transferir equipos.
- No replicar claves, roles, CURP, biometría, direcciones ni expedientes de titulares de Vault en SQLite del núcleo. Un recibo firmado de Vault concede un permiso acotado, no privilegios administrativos generales.
- Los comandos de consulta para invitados no requieren registro civil. La activación de chat/grupo, límites y autorización de acciones sensibles siguen vigentes; el proyecto Bot-Matter no se desbloquea por iniciar una tienda.
- Los adaptadores deben permitir integración posterior Web/App/API/Server sin dependencia rígida de Bot-Matter.
- El marketplace de módulos del archivo fuente es distinto de la tienda de dispositivos/electrodomésticos. El núcleo debe continuar soportando ambos contratos sin mezclar acceso a código ejecutable y compra de productos físicos.
- Tri-Red y el generador mecánico-magnético son líneas experimentales separadas; Base no debe presumir su validez científica, seguridad eléctrica o que controlan dispositivos reales.

## Requisitos para desbloquear integración comercial en una fase futura

1. Documentos Marketplace, Vault y API con owners, políticas y pruebas pactadas.
2. Pruebas de principal/actor/scope, firma de recibo, no replay, error de Vault, negación por revocación.
3. Eventos versionados y validación de esquemas y límites de recursos.
4. Confirmación de que no se alteran las pruebas existentes de WhatsApp Rust ni la separación de comandos `#` y controles `=/> `.
5. Revisión de todos los README y registro de pruebas por commit. Trabajo de diseño no equivale a disponibilidad en WhatsApp.

## Prioridad

Completar la preparación de Base-Matter y su WhatsApp Rust estable antes de convertir la iniciativa comercial en trabajo de Bot-Matter. No se cambia código funcional ni se abren funciones administrativas por esta nota.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
