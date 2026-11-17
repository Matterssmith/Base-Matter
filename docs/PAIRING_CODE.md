# Base-Matter — Contrato exclusivo de código de vinculación

## Alcance

La conexión WhatsApp se probó históricamente mediante código y \`ping\`.
El árbol \`matter-beta\` actual no incluye ese proveedor funcional, por
lo que ni las pruebas del contrato ni el estado de una sesión SQLite prueban
que un dispositivo esté vinculado ahora.

\`PairingCodeService\` es la única interfaz de emparejamiento prevista
para WhatsApp en Base-Matter. No expone QR ni caminos alternativos.
Recibe el número autorizado como dígitos internacionales sin símbolo \`+\`.
\`SessionRegistry\` identifica el socket y su propietario; la aplicación
suministra una implementación de \`ConnectionAdmission\` que verifica la
identidad y consentimiento real. Solo acepta sesiones WhatsApp que aún no
estén conectadas. \`PairingCodeProvider\` solicita el código efímero al
proveedor, a través de una conexión previamente autorizada por el host.

El código devuelto se guarda temporalmente en memoria y se borra al soltar
el valor. \`Debug\` lo redacta; nunca se guarda en SQLite ni en logs. El
operador autenticado lo introduce manualmente en la interfaz de dispositivo
vinculado de WhatsApp. El secreto no debe devolverse a un actor no autorizado.

No implementa transportes ni sesiones reales. La autenticación,
revocación, rate limits entre procesos, generación del socket, reconexión y
almacén privado de credenciales son responsabilidad de la integración
posterior. Nunca eliminar ni sustituir sesiones existentes de otro socket.

## Control global

\`=/>on\`, \`=/>off\` y las operaciones de rango reservadas pasan por
\`ControlDispatcher\` antes de cualquier bot. \`=/>ping\` no es un comando
global implementado. Un \`#ping\` histórico fue únicamente una prueba de
conectividad y no forma parte de la API requerida.

\`Bot-Matter\` permanece bloqueado. Los repositorios del ecosistema se
integrarán como consumidores cuando termine Base-Matter.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

La consulta de estado del socket se hace mediante `=/>status`, con
identidad del dueño. No solicita ni muestra el código temporal de
vinculación, y no expone un método QR.

## Global control after verified pairing

`WhatsAppControlBridge` verifies a provider-signed owner-only event before
calling ControlDispatcher for `=/>` commands. It cannot pair or authenticate
a live phone by itself; the external provider must use pairing code only.
See [bridge protocol](WHATSAPP_BRIDGE.md).
