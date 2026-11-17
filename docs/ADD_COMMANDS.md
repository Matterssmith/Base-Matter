# Base-Matter — Gramática universal de add

## Operaciones definidas

| Comando | Propósito |
|---|---|
| `=/>add user +521... ` | Preparar ingreso de persona al grupo del mensaje, solo con contexto verificado |
| `=/>add user @usuario group <id>` | Seleccionar grupo de destino explícito |
| `=/>add user reply community <id>` | Seleccionar al autor del mensaje citado y comunidad de destino |
| `=/>add group [--hide] community <id>` | Proponer mover el grupo actual a una comunidad |
| `=/>add community <id>` | Seleccionar una comunidad existente |
| `=/>add community new --Mi Comunidad` | Proponer crear comunidad con nombre propio |
| `=/>add community backup <origen> to <destino> [--hide]` | Preparar grupo o comunidad de respaldo para miembros, sujeto a aceptación |

El identificador `<id>` admite una referencia exacta o el alias `official`; `official --register` denota un subgrupo oficial previamente registrado. Los nombres no autentican comunidades.

## Contratos de seguridad

El parser `src/add_commands.rs` no hace llamadas de red ni copia sesiones. El controlador global verifica al dueño y devuelve **solo vista previa**; no expone números de teléfono en la respuesta. No usa un chat grupal como fuente del grupo actual hasta disponer de ingreso autenticado desde grupos.

Cualquier incorporación o traslado futuro requiere verificar privilegios en origen y destino, límites del proveedor, permisos, grupos vinculados y aceptación del usuario; si la privacidad impide añadirlo, ofrecer invitación voluntaria. No eludir sanciones ni utilizar cuentas alternativas para recuperar acceso bloqueado.

Un `reply` solo debe resolverse usando metadatos WhatsApp del mensaje citado y no texto arbitrario. `@usuario` no implica que la API pueda resolverlo. `--hide` solicita ocultamiento únicamente si existe soporte documentado: nunca garantiza visibilidad especial ni eludir notificaciones.

Para `community backup`, el tipo de destino puede ser ambiguo; resolverlo desde metadatos reales antes de ejecutar. Exigir plan de cambios, nonce, caducidad, idempotencia, registro de acciones, control de tasa y confirmación específica del administrador. Si no hay capacidad, rechazar el plan sin cambios.

## Estado actual

Implementados en alpha el parser tipado y validación de permisos del dueño del socket para mostrar vista previa. **No implementados:** agregar miembros, cambiar comunidades, crear grupos, ocultar grupos, leer replies en chats grupales, mapeo real de alias official ni mutaciones en WhatsApp. CI debe aprobar antes de promover a beta.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
