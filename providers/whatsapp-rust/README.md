# WhatsApp-Matter: proveedor Rust independiente

El proveedor original es whatsapp-rust, ejecutado en un **proceso Rust
separado** del núcleo Base-Matter. No usa Node ni Baileys ni Bot-Matter.

La separación responde a versiones incompatibles de libsqlite3-sys entre
la biblioteca whatsapp-rust 0.7.0 y las dependencias actuales de Base.
Ambos procesos siguen escritos en Rust y se comunican por un canal TCP
en 127.0.0.1 protegido con HMAC-SHA256 de 256 bits.

## Parámetros locales

- MATTER_WA_PHONE: número para pairing code, 8–15 dígitos internacionales
- MATTER_WA_OWNER_JID: JID propio privado (dominio s.whatsapp.net o lid)
- MATTER_WA_SOCKET_ID: socket registrado en Base-Matter
- MATTER_WA_OWNER_ID: dueño registrado en Base-Matter
- MATTER_WA_SESSION_DB: ruta absoluta al SQLite privado de whatsapp-rust
- MATTER_WA_BRIDGE_ADDR: 127.0.0.1:puerto del controlador de Base
- MATTER_WA_BRIDGE_KEY_HEX: clave privada HMAC de 32 bytes, 64 hex

La carpeta del SQLite debe estar protegida para el usuario del servidor.
El propietario de la sesión se bloquea mediante archivo owner.lock para
impedir procesos duplicados. No se elimina la sesión de WhatsApp.
Realizar un respaldo SQLite consistente antes de actualizar una sesión
existente. Probar primero un número secundario.

Ejecución desde el repositorio, con las variables de entorno cargadas:

    cargo run --manifest-path providers/whatsapp-rust/Cargo.toml

El enlace se realiza exclusivamente mediante código de vinculación mostrado
en la terminal del operador. El proveedor no registra callback QR ni muestra
códigos QR; whatsapp-rust puede crear un desafío QR interno en paralelo,
sin que este proceso lo exponga.

El proceso valida el mensaje de la cuenta propia, JID de chat y remitente
exactos, fromMe y chat no grupal. Firma únicamente mensajes canónicos
=/> con timestamp reciente y los entrega al controlador Rust de Base.
No envía mensajes de WhatsApp, no responde ping y no ejecuta Bot-Matter.

## Límite de estabilidad

El proveedor requiere un proceso receptor Base-Matter configurado con el
mismo secreto y socket, una sesión real vinculada mediante pairing code,
revisión de compatibilidad SQLite y pruebas de revocación y reconexión.
No se puede afirmar estabilidad por aprobar únicamente pruebas offline.

Biblioteca upstream: oxidezap/whatsapp-rust, versión 0.7.0 (tercero, no
oficial de WhatsApp). Conservar licencias y atribuciones upstream.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## Preflight Rust sin conexión

Con el entorno configurado ejecutar `cargo run --manifest-path providers/whatsapp-rust/Cargo.toml -- --check`. Comprueba el número internacional, JID propio, claves y ruta privada sin abrir sesiones ni conectarse a WhatsApp. El controlador independiente de Base ofrece `cargo run --locked --bin whatsapp-matter-control -- --check`. No publicar variables de entorno ni archivos SQLite.
