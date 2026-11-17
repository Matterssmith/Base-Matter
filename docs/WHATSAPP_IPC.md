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
