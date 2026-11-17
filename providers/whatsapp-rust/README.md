# Proveedor WhatsApp-Matter — Rust

**Implementación:** whatsapp-rust 0.7.0 con SQLite y PairCodeOptions.
No se utiliza Node, Baileys, código QR visible ni Bot-Matter. Este binario
continúa el proveedor Rust de Base-Matter probado históricamente, commit
3c36d97bb, adaptándolo a los contratos de control actuales.

## Entorno obligatorio

Todas las variables se suministran fuera de Git, desde el operador local.
Nunca escribas teléfonos, claves o sesiones en este repositorio.

- MATTER_WA_PHONE: número internacional de 8 a 15 dígitos, sin +
- MATTER_WA_OWNER_JID: JID privado propio, con dominio s.whatsapp.net o lid
- MATTER_WA_SOCKET_ID: ID estable del socket
- MATTER_WA_OWNER_ID: identidad local del dueño autenticado
- MATTER_WA_SESSION_DB: archivo SQLite absoluto de whatsapp-rust
- MATTER_WA_ACTIVATION_DB: archivo SQLite absoluto del control de Base
- MATTER_WA_REPLAY_DB: archivo SQLite absoluto del anti-replay
- MATTER_WA_REPLAY_KEY_HEX: secreto local no nulo de 32 bytes en hexadecimal

La carpeta que contenga cada archivo debe existir, no ser un enlace simbólico
y, en Linux, tener permisos únicamente para su dueño (0700).

Una cuenta con acceso al servidor configura esas variables y ejecuta:

    cargo run --manifest-path providers/whatsapp-rust/Cargo.toml -- --init
    cargo run --manifest-path providers/whatsapp-rust/Cargo.toml

La primera instrucción registra el socket una sola vez. La segunda abre
whatsapp-rust con el almacenamiento de sesión indicado. Se muestra
**solo el código de emparejamiento**, que se introduce desde WhatsApp,
Dispositivos vinculados, Vincular con número de teléfono.

La biblioteca upstream 0.7 puede generar internamente un QR en paralelo al
pairing code; este binario no registra ni presenta ningún callback QR. No se
afirma que el flujo QR del protocolo esté desactivado.

El proveedor solo admite controles globales =/> desde el chat privado propio.
El verificador de Base necesita from_me, emisor y chat iguales al JID propio,
ausencia de grupo, tiempo reciente, ID no repetido y propiedad del socket.
No reenvía respuestas al chat ni acepta #ping como comando permanente.

## Seguridad / pruebas

- El archivo SQLite nunca se elimina de forma automática.
- Un archivo de bloqueo impide dos instancias del mismo adaptador.
  Ante un bloqueo dejado por un proceso caído, verifica manualmente la
  finalización del proceso y el estado de la sesión antes de retirarlo.
- Realiza una copia de seguridad consistente de la base SQLite existente
  antes de intentar actualizar versiones. Este código no migra ni modifica
  sesiones antiguas sin la biblioteca upstream.
- El almacén de repetición es independiente y debe ser persistente.
- Las pruebas offline no verifican el emparejamiento ni la red WhatsApp.
  Se requiere un número de pruebas autorizado y supervisión del socket
  para aprobar el paso a estable.
- La biblioteca whatsapp-rust es no oficial y no pertenece a Meta.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
