# Base-Matter — Convención oficial propuesta para Anti, Set y activación

**Estado:** decisión de diseño documental (2026-11-19). **No implementada en Rust**. Ningún comando real debe declararse operativo por esta nota. Este documento prevalece como requisito de la próxima migración sobre la gramática anterior de \`set anti ... on/off/inherit\`. Debe conservarse lo anterior como historial, pero no como instrucción de implementación nueva.

## Propósito

Separar sin excepciones tres responsabilidades:
1. \`set\`: **configura parámetros** de un módulo (acciones, listas, umbrales, excepciones persistentes, etc.). Nunca activa/desactiva por sí solo un anti.
2. \`on\`, \`off\`, \`-\`: **operan sobre la activación** mediante la familia \`anti\`, no mediante \`set\`.
3. \`anti --<mode>\`: **operación colectiva** de los antis pertenecientes a un modo, con selección explícita de integrantes mediante \`--except\` y \`--include\`.

El control nativo mantiene el prefijo \`=/>\`. El futuro bot puede usar alias \`#\` mediante el puente de capacidades protegido; nunca compartir permisos automáticamente entre ambos prefijos.

## Gramática objetivo (propuesta, NO comandos probados)

| Forma | Significado |
| --- | --- |
| \`=/>anti\` | Consultar estados reales de los antis del chat, sin cambios. |
| \`=/>anti link\` | Consultar el estado efectivo de AntiLink. |
| \`=/>anti link on\` | Activar AntiLink en el ámbito autenticado actual. |
| \`=/>anti link off\` | Desactivar AntiLink en el ámbito autenticado actual. |
| \`=/>anti link -\` | Quitar el override local para que herede el predeterminado aplicable; no elimina ni modifica el valor global. |
| \`=/>anti --multimedia\` | Consultar el estado del modo multimedia. |
| \`=/>anti --multimedia on\` | Activar todos los antis de ese modo, sin excepciones. |
| \`=/>anti --multimedia off --except video,sticker\` | Desactivar los antis del modo menos los dos expresamente excluidos; éstos no se modifican. |
| \`=/>anti --multimedia on --include image,video\` | Activar solo image y video dentro del modo. |
| \`=/>anti --security on --except link\` | Activar los filtros Anti definidos para ese modo, excepto link, solo si link realmente pertenece al modo. |
| \`=/>anti --multimedia - --include image,video\` | Retirar los overrides locales de image y video para heredar su valor predeterminado. |
| \`=/>anti --mod link on\` | Modificar el valor predeterminado del socket propio, si está autenticado y autorizado. |
| \`=/>anti --global link\` | Consultar valor general, sin modificarlo. |
| \`=/>set anti link action warn\` | Configurar la acción, **sin encender el filtro**. |
| \`=/>set anti link add example.org\` | Configurar la lista de dominios, **sin encender el filtro**. |

\`=/>set anti link on\`, \`=/>set anti link off\` y \`=/>set anti link inherit\` **no forman parte de la sintaxis objetivo**: no deben ofrecerse como configuración. Deben desaparecer del menú y help una vez migrados los handlers y pruebas. Antes de retirar compatibilidad, evaluar configuraciones existentes y una transición segura; no cambiar la base de datos sin migración.

## Semántica de los operadores

- \`on\`: override explícito activado.
- \`off\`: override explícito desactivado.
- \`-\`: **heredar** el valor superior eliminando exclusivamente el override del ámbito solicitado. No es un apagado global, un borrado del expediente o una revocación de la política general.
- Se mantiene el orden de precedencia ya documentado donde exista: grupo explícito > socket \`--mod\` > valor general \`--global\`. No asumir que un fallo de Vault habilita una política privilegiada; fallar cerrado.
- La consulta sin operador nunca crea escritura ni moderación.

## Modos

- \`--multimedia\`: conjunto de filtros Anti sobre imágenes, videos, audio, documentos, GIF, stickers y otros tipos de medios existentes. La pertenencia canónica debe salir de un inventario registrado, nunca del nombre del archivo.
- \`--security\`: **subconjunto de filtros Anti** de naturaleza defensiva (ej. enlaces, spam o flood si están registrados así). **No sustituye** la categoría independiente Matter Security, que gestionará incidentes, evidencia, alertas, riesgos y recuperaciones.
- Otros modos deberán definirse tras auditar el catálogo real. No inventar funciones ni anunciar modos vacíos en help.
- Un filtro puede estar en más de un modo solo si su registro lo declara explícitamente. Evitar ejecutar dos veces la misma transición.

## Selección de miembros de un modo

- Sin selectores: todos los filtros registrados y habilitables del modo.
- \`--include a,b\`: solo el conjunto nombrado **dentro del modo**.
- \`--except a,b\`: todos menos el conjunto nombrado.
- Ambos: \`(include si existe; en otro caso todos) - except\`. Si un identificador aparece en ambos, el resultado lo excluye; el intérprete deberá explicarlo en la vista previa.
- En \`--include\` y \`--except\` se admiten los nombres y alias canónicos del **registro de antis**, no rutas \`.rs\`, paths arbitrarios ni archivos cargados desde usuarios. La palabra "archivo" en el requisito significa filtro/módulo individual, no abrir archivos del servidor.
- Validación estricta: modos desconocidos, reglas inexistentes o no pertenecientes al modo, listas vacías, argumentos duplicados o selecciones finales vacías deberán informarse sin ejecutar cambios.
- El sistema debe mostrar qué filtros cambiarían, qué filtros quedan intactos, el ámbito y la herencia efectiva. Las operaciones masivas deben ser transaccionales y dejar auditoría única con detalle individual; no persistir activaciones parciales silenciosas.

## Separación entre configurar y activar

- \`set\` modifica **atributos**: acción \`warn|delete|kick\`, listas, umbrales y otros parámetros válidos de un filtro.
- \`anti\` modifica **únicamente estado de activación** o consulta estado.
- Añadir una excepción o dominio no habilita nada automáticamente.
- La selección dinámica \`--except\` o \`--include\` de una operación masiva es un filtro de ejecución de esa solicitud, no una modificación persistente de configuración; para guardar excepciones permanentes se usará \`set\` con una subopción dedicada y especificada.

## Permisos, ámbitos y seguridad

- \`--mod\` = predeterminado del **socket propio**, no todos los sockets. \`--global\` = general. \`-\` retira override del ámbito local o de socket según corresponda, no borra el global.
- Escritura del valor verdaderamente global queda BLOQUEADA hasta disponer de autoridad Vault comprobada; no convertir \`-\` en un atajo para omitir esta política.
- Antes de aplicar un lote: autenticar origen del mensaje, actor, socket, grupo y capacidad exacta. No confiar en texto de reply, menciones, alias o prefijos como prueba de autoridad.
- No ejecutar comandos editados o camuflados. Distinguir política guardada de operaciones reales: marcar una acción \`delete\` o \`kick\` no certifica que exista moderación efectiva en WhatsApp.
- Por defecto todos los antis siguen OFF hasta cambio autorizado. Solo informar estados realmente disponibles.

## Integración pública y privada

- Base publica la gramática, tipos y puentes de invocación. La implementación privada y sus llaves permanecen fuera de Base. Vault autoriza; un ejecutor privado verifica y ejecuta.
- Una aplicación podrá vincular \`#antilink\` al puente público \`anti_link.rs\` y ofrecer comandos propios sin recibir permiso sobre recursos de Matterssmith. Un alias no añade capacidades.
- No cambiar Rust ni activar gateway privado a partir de esta especificación. El puente actual solo representa solicitudes; sin gateway operativo las llamadas privilegiadas deben fallar cerradas.

## Trabajo pendiente para la siguiente sesión

1. Inventariar comandos y catálogos Anti reales, identificar las reglas y sus modos válidos y auditar interfaces \`help\` y \`menu\`.
2. Diseñar parser y validación Rust para \`anti [--<mode>] [regla] [on|off|-] [--include ...] [--except ...]\` sin romper la lectura sin argumentos.
3. Separar cualquier manejo actual de \`set anti ... on/off/inherit\` para que \`set\` trate exclusivamente configuración. Definir migración de datos y compatibilidad temporal sin anunciar sintaxis obsoleta.
4. Diseñar operaciones masivas transaccionales e idempotentes, log de auditoría, validación de autoridad y tests de selección/herencia/errores.
5. Integrar futuros puentes privados sólo después de la autenticación de Vault, manteniendo \`--mod\` y \`--global\` separados.
6. Probar Rust Linux/Windows, proveedor Rust por separado y WhatsApp real antes de promover \`alpha\` a \`beta\`; no modificar \`stable\` ni desbloquear Bot-Matter.

**Continuidad:** exclusivamente documentación. La gramática anterior permanece implementada hasta completar migración; esta nota NO equivale a una corrección del runtime.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
