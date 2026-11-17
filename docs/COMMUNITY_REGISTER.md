# Base-Matter — Contrato comunitario REGISTER

## Propósito y alcance

`=/>start` inicia la preparación segura de comunidades oficiales a través
del adaptador de WhatsApp Rust. El prefijo `=/>` pertenece a Base-Matter
independientemente del encendido del bot. `start` no debe habilitar mensajes
normales ni crear recursos por sorpresa.

## Identidad del grupo oficial

Primera línea EXACTA de la descripción de cada grupo REGISTER:

`MATTER:OFFICIAL:register:<community-id>`

`<community-id>` es el identificador estable y autenticado del padre
reportado por WhatsApp (JID). El parser exige cadena completa, sin
espacios invisibles, coincidencias parciales o búsquedas dentro del texto.
El encabezado aporta descubrimiento, NO prueba autoridad; además comprobar
que WhatsApp confirma que el subgrupo pertenece a ese padre.

Un padre no puede ser elegido por coincidencia de título únicamente.
El dueño de la sesión debe ser administrador verificado de la comunidad.
Un grupo con el encabezado correcto pero no vinculado al padre se ignora.

## Algoritmo de =/>start

1. Validar mensaje propio en chat privado, contexto `fromMe`, JID/PN/LID,
   socket y autorización del dueño en Base-Matter.
2. Leer todas las comunidades del proveedor, limitar resultados y comprobar
   comunidad real + privilegio admin de la identidad autenticada.
3. Para cada comunidad autorizada, enumerar grupos vinculados y consultar
   sus descripciones y admins. Marcar REGISTER por encabezado exacto.
4. Si se encuentra UNO: preparar diferencias del título, descripción,
   foto y políticas frente a la plantilla; no mutar aún.
5. Si se encuentran varios: informar colisión y no tocar grupos.
6. Si no se encuentra ninguno: preparar propuesta de crear subgrupo oficial
   y añadirlo a la comunidad, solo si hay permisos de creación.
7. Preparar solicitudes de enlaces a admins de cada grupo, sin promover
   a ninguno por ello. Nunca publicar enlaces privados automáticamente.
8. Crear un PLAN inmutable de alcance limitado, con nonce, expiración,
   revisión del estado de WhatsApp y permisos necesarios.
9. Una operación separada `=/>start apply <plan-id>` solo podrá ejecutar
   acciones autorizadas tras implementar auditoría, idempotencia y
   confirmación del dueño; se rechazará hasta estar verificada.

## Datos por comunidad

- Plantilla de título, descripción, imagen, quién puede editar, enviar,
  invitar y aprobar entradas; configuración por comunidad y roles.
- Dos cuentas propias confiables configurables (no codificadas). Añadir
  miembros solo con autorización y aceptación de políticas; promover a
  admin solo si ya pertenecen al grupo y si la cuenta ejecutora es admin.
- Directorio de asistencia Matter: conservar enlace, función
  `register`, JID, permisos de publicación, fecha de obtención,
  verificación, fecha de vencimiento y revocación. Cifrar y restringir
  acceso; un link de invitación es sensible.
- No suponer capacidad de mencionar admins, cambiar imagen o crear
  subgrupos sin confirmar API, permisos y resultado remoto.

## Visuales y compatibilidad

Diseño independiente de la lógica: esquema renderizable para encabezados
menu/help/h, ícono, identidad del bot, bloques, estado, créditos, versión,
variaciones por comunidad. Herencia por ámbito Group > Community > Socket >
Global > defaults, sin otorgar permisos por estilos. La misma plantilla
se puede usar para `=/>start` y `#matter` de terceros.

## Estado

La biblioteca `whatsapp-rust 0.7.0` expone consulta de comunidades,
subgrupos, grupos y operaciones de gestión, pero requieren una sesión y
permisos reales. Fase alfa: construir descubrimiento seguro, pruebas
offline, simulación de plan; mutaciones y creación no se consideran
implementadas sin confirmación remota. No iniciar Bot-Matter.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
