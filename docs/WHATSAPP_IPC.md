# Base-Matter — receptor Rust para whatsapp-rust independiente

Dos procesos Rust se conectan por TCP local a 127.0.0.1.

- Base-Matter: binario whatsapp-matter-control --init para provisionar el
  socket una vez; --serve para aceptar eventos firmados.
- WhatsApp-Matter: el proveedor whatsapp-rust 0.7 recibe eventos de la cuenta
  vinculada exclusivamente mediante pairing code, firma las solicitudes
  privadas propias y las envia a Base.

Ambos procesos comparten configuracion de socket, dueno, JID de propietario
y una clave HMAC 256 bits que NO debe publicarse ni imprimirse:

- MATTER_WA_SOCKET_ID
- MATTER_WA_OWNER_ID
- MATTER_WA_OWNER_JID
- MATTER_WA_BRIDGE_ADDR = 127.0.0.1:PORT
- MATTER_WA_BRIDGE_KEY_HEX = 32 bytes en hexadecimal

Base utiliza ademas:

- MATTER_WA_ACTIVATION_DB = ruta SQLite privada absoluta
- MATTER_WA_REPLAY_DB = ruta SQLite privada absoluta y diferente
- MATTER_WA_REPLAY_KEY_HEX = clave distinta de la del puente

Los datos SQLite permanecen locales y no deben subirse a Git. Se requiere
un directorio privado, propiedad exclusiva del operador. La firma HMAC
abarca version, socket, actor, JID, ID de mensaje, texto, from_me y tiempo.
El receptor aplica autenticacion, ventana temporal, anti-replay y las reglas
del dueno antes de ejecutar los verbos de control global =/>.

El receptor no tiene endpoints HTTP ni acepta direcciones publicas.
Las respuestas contienen solo un resultado sanitizado: applied, ignored,
duplicate o denied. No responde por WhatsApp ni ejecuta Bot-Matter.

Los tests de Rust pueden aprobar sin vincular un telefono. La prueba de
emparejamiento real, revocacion, reinicio y seguridad de SQLite upstream
siguen siendo requisitos antes de declarar el servicio de produccion.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## Prueba real de la conexión local

`tests/control_ipc.rs` abre un socket en 127.0.0.1 de puerto efímero, envía tramas HMAC válidas y comprueba el resultado, el bloqueo de replays y que no se devuelvan datos de identidad al cliente. No accede a WhatsApp ni a sesiones de producción.

## Tiempo total y tamaño de los frames (hardening)

La implementación TCP mantiene un límite estricto de 16 KiB por frame y un
plazo total de 4 segundos desde que acepta una conexión, incluso si el
cliente transmite bytes lentamente. Un frame parcial, excesivo o que no
cierra su mitad de escritura se deniega; no ejecuta comandos y no expone
identificadores en la respuesta. El protocolo HMAC no cambia. El receptor
usa una cola secuencial local, por lo que sigue pendiente una auditoría de
resistencia a saturación con procesos no confiables en el mismo host.

## Respuestas legibles a controles (alpha)

El IPC puede adjuntar un campo JSON opcional `message` a un
`applied` autorizado. Se devuelve texto para `status`, encendido/
apagado y extensiones visuales; nunca para denegaciones ni duplicados.
Máximo 760 bytes UTF-8 y sin caracteres de control salvo saltos de línea.
El campo no contiene secretos ni JIDs y el protocolo HMAC de entrada
permanece intacto. El cliente debe rechazar campos desconocidos
y no interpretar el texto como un nuevo comando.

Esta capacidad no autentica por sí sola el canal de respuesta TCP.
Solo debe utilizarse en el puente local protegido del host, con un
remitente/sesión ya autenticado y reglas de salida estrictas.
