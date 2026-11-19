# Contrato de ámbito y consulta de usuario — Base-Matter

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## Semántica uniforme

Todos los comandos de control que tengan una operación con ámbito usarán el mismo significado; una bandera no convierte una operación sin soporte en una operación disponible:

- Sin bandera: chat firmado actual, siempre que el comando tenga configuración de chat.
- `--mod`: **socket actual autenticado**, no otra cuenta ni otro servidor.
- `--global`: valores predeterminados **generales del ecosistema**; exige repositorio/servicio global autenticado para escrituras, no se confunde con el socket actual.
- Una sola bandera; ambas juntas, repeticiones u otros ámbitos no autorizados se rechazan.
- Orden de bandera: `=/>anti --mod link` y `=/>anti link --mod` tienen igual ámbito.
- Ejemplo de herencia futura: chat > socket > global. El estado del chat puede sobrescribir el predeterminado de socket, y este el general.

## Consultas de usuario (lectura pública)

Sintaxis solicitada:
- `=/>status` como respuesta a un mensaje: el objetivo procede del remitente citado verificado.
- `=/>status @mención` o `=/>status user <id>`: resolver contra un JID/identidad auténtica del proveedor.
- `=/>status numero <número>`: solo una **pista de búsqueda**; no equivale a identidad probada.
- `=/>status --mod @mención`: atributos públicos verificables del objetivo en el socket.
- `=/>status --global @mención`: exclusivamente atributos declarados públicos del directorio global cuando Vault responda autenticado.

Cualquier persona de un chat habilitado podrá consultar estado **público** del usuario, donde la implementación esté disponible. No se revelan JID completos, números internos, sesiones, consentimientos, roles ocultos ni estado de otros chats. Una respuesta por cita necesita identidad de participante firmada y vínculo con el mismo chat; no bastan texto copiado, nombre visible ni un JID escrito. Menciones requieren metadatos de mención, no inspección por coincidencia del texto.

**Estado real (2026-11-19):** `src/control_scope.rs` normaliza las banderas y valida **solo la gramática** de la consulta de usuario. No existe aún un resolvedor conectado y no se ha autorizado `=/>status @usuario` ni consultas por respuesta/numero para terceros. `anti` ya consume el analizador común; las demás familias migrarán cuando cada módulo pueda validar permisos y ámbito. `anti --global` muestra únicamente el fallback inmutable OFF/warn del módulo, explícitamente no una configuración Vault compartida. Los cambios globales siguen bloqueados.

## Autorización y precedencia

La bandera `--mod` jamás otorga autoridad de propietario; el socket surge del transporte autenticado. `--global` no está autorizado para mutación sin owner global autenticado por Vault y auditoría. Los resultados por usuario se filtran mediante política de datos públicos incluso si los solicita una cuenta administradora. Las opciones se rechazan antes de ejecutar handlers que no admitan ese ámbito.

En implementaciones futuras, cada comando debe declarar los ámbitos que soporta, si es lectura o escritura, y la autorización necesaria. Probar el árbol de herencia, modificaciones concurrentes, transporte de menciones/citas, aislamiento de sockets y ausencia de filtraciones. No crear un bypass dentro de texto citado ni ejecutar comandos encontrados en una cita. La cuenta conectada nunca es identificada por texto plano.

## Prueba y liberación

Requeridos: Cargo fmt/check/test/Clippy Windows y Linux, proveedor WhatsApp Rust y prueba real con pairing code. El contrato no certifica soporte de WhatsApp. `beta` y `stable` siguen intactos. Bot-Matter permanece bloqueado.
