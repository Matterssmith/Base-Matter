# Base-Matter — Transporte protegido para capacidades privadas

**Regla obligatoria de integración. Estado:** arquitectura y requisito, NO capacidad de red desplegada.
Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## Alcance

Base-Matter es una biblioteca pública. Sus contratos no deben conocer direcciones
de repositorios privados, rutas de código interno, claves, tokens ni credenciales.
Una llamada de capacidad comunica **únicamente** un identificador y argumentos
validados mediante una interfaz de confianza suministrada por el host.

**Todo dato operativo recibido de una autoridad protegida o enviado hacia ella
debe tratarse como una credencial.** Ninguna solicitud sensible, decisión,
registro, auditoría, resultado o sesión puede atravesar la frontera Base/host/
ejecutor en texto plano. La conexión y las cargas deben contar con cifrado
autenticado, integridad, autenticación recíproca y protección contra replay.

El host autorizado gestiona las claves y resuelve los servicios; nunca la
biblioteca pública ni los bots que la consumen. Cuando Base actúe como
intermediario debe transportar sobres cifrados opacos y no descifrar
material privilegiado. Una respuesta visible al usuario se entrega únicamente
al destinatario autenticado, tras una decisión de liberación mínima, y el
transporte hasta él sigue cifrado.

## Comportamiento obligatorio

- Sin canal cifrado y autenticado: denegar. HMAC por sí solo no cifra.
- Sin identidad de bot, socket, grupo y actor autenticada: denegar.
- Sin concesión actual, revocable y limitada a operación/ámbito: denegar.
- No incluir claves, payloads, teléfonos, códigos o material descifrado en
  logs, mensajes de error, base de datos, telemetría o respuestas de ayuda.
- No compartir claves ni permisos de una instancia entre bots o sockets.
- No descargar ni compilar código privado desde el cliente. El ejecutor
  permanece en un proceso aislado; Base expone únicamente contratos públicos.
- Los resultados de capacidad deben ser limitados, aptos para mostrar y
  carentes de datos internos. Fallo del gateway = acceso denegado, no fallback.
- En la frontera de confianza el descifrado se permite únicamente en memoria
  de un proceso autorizado, durante el tiempo mínimo necesario. No declarar
  que la información está cifrada mientras se procesa descifrada en memoria.

## Estado de implementación

La interfaz `ProtectedCapabilityGateway` y
`UnavailableGateway` existentes constituyen contratos, **no un canal
de transporte seguro operativo**. Ninguna aplicación debe interpretar
`CommandContext` como una prueba de origen. Hasta que el host aporte una
implementación autenticada y probada, se rechazan las operaciones privadas.

La creación de bots puede elegir plataforma, proveedor, lenguaje y sistema
mediante descriptores públicos, pero esa selección nunca concede permisos.
Un proveedor funcional se verifica independientemente de su nombre.

## Validación requerida

Comprobar cifrado sobre transporte, modificación del sobre/AAD, replay,
expiración, acceso cruzado por socket/bot/grupo, errores sin filtración,
rotación y revocación; CI Linux/Windows y pruebas de integración.
Esta actualización solo documenta requisitos y no cambia Rust.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
