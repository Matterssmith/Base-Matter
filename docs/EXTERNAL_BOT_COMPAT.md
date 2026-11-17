# Base-Matter — protocolo de compatibilidad #matter

`#matter` se reserva para bots EXTERNOS al ecosistema, incluidos proyectos
que no usan Rust. Es una puerta de entrada, no un comando de superusuario.

## Flujo de confianza

`#matter start` -> adaptador externo autentica evento, actor, chat y
socket -> normaliza la intención -> firma una solicitud acotada ->
Base-Matter valida identidad del adaptador y que el actor es dueño
autorizado -> traduce a comando `=/>start` -> verifica permisos,
replay, consentimiento y política -> registra resultado auditable.

Los permisos del otro bot nunca se convierten automáticamente en
permisos de Base-Matter. No aceptar actor o socket desde texto arbitrario.
Requerir proveedor y bot externo registrados, versión del contrato,
identidad real, TTL corto, nonce y firma HMAC/doble autenticación.
Los bots externos no pueden controlar sockets ajenos.

## Sesiones y migración

Por defecto **NO se copian** sesiones de WhatsApp/Telegram/Discord. Se
prefiere reutilizar una conexión existente mediante un IPC local
autenticado, sin entregar archivos de credenciales.

Cuando una migración autorizada requiera exportar una sesión incompatible:
solo un operador autenticado podrá iniciar un respaldo cifrado local,
con permiso específico, bloqueo exclusivo, integridad y revisión de
versiones. El origen debe estar desconectado antes de entregar la
sesión al destino; nunca ejecutar ambos simultáneamente sobre el mismo
estado, jamás guardar sesiones en GitHub o Matter-Vault, y purgar la
copia temporal después de verificar la restauración. Si el proveedor
no permite importación segura, fallar sin copiar.

## Apariencia y diseño

El adaptador externo puede aplicar la plantilla de estilo Matter a su
comando `#matter` sin alterar permisos. Soportar temas y lenguaje por
comunidad/bot, encabezado/menú/help/h, identificador estético y créditos
`Hecho por Lournyx Matterssmith [Lou-Nyx].`.

## Alcance de implementación

Este archivo es un contrato, no un conector ya distribuido en bots ajenos.
No copiar tokens, no introducir secretos en logs y no declarar una
integración activa hasta que un adaptador externo complete su prueba.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
