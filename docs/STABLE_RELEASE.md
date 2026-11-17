# Base-Matter — stable checkpoint

Esta rama estable continúa con el código de Matter-Base y conserva
el historial propio de \`matter\`. El nombre Rust \`matter_base\` sigue
siendo la API compatible. \`Base-Matter\` es la convención de marca futura,
no una afirmación de renombrado remoto.

Promociones aceptadas para la base:
- corrección de límites de base64 con \`div_ceil\`;
- validación de rango del esquema de usuarios;
- tipo explícito para la fila de consulta legal;
- cierre de SQLite antes de borrar archivos temporales en Windows;
- rutas temporales de prueba con identificadores aleatorios;
- licencia MIT y Cargo.lock para builds reproducibles;
- comprobación Rust de Ubuntu y Windows en PR y branch.

Estas correcciones ya fueron verificadas en la rama beta. Su promoción
a \`matter\` exige CI satisfactoria también sobre la rama candidata estable,
sin copiar módulos de integración pendientes de beta.

No incluye el Runtime de Matter-Bot, adapters WhatsApp/Telegram/Discord,
integración real de Matter-Vault ni ejecutor sandbox. Los tests no autorizan
producción de conectores externos.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
