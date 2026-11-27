## 2026-11-26 — Rangos WA y Matter independientes en consola

Por solicitud del propietario, la terminal muestra Rango WA, Rango Matter e ID en filas independientes de la plantilla compartida. El rango WA procede de metadatos del grupo; Matter muestra Owner global únicamente con ACK positivo del controlador, Owner socket cuando corresponde o Sin verificar si no hay confirmación. No se confunde ausencia de confirmación con ausencia de autoridad global. No se cambian permisos, resolución de identidad, autenticación, sesiones ni Vault. Prueba de regresión añadida; compilación, CI y comprobación de la identidad real en idkbot pendientes. README revisados: 2/2.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-26 — Resolución de presentación por usuario autenticado

El IPC selecciona el actor real de la solicitud firmada y utiliza `effective-style` de Vault para decorar las respuestas y reconstruir menús y ayuda admitidos. Los adaptadores que no cuentan con actor verificado mantienen la API anterior. Ante indisponibilidad de Vault se usa el estilo de fábrica, sin reutilizar preferencias personales potencialmente revocadas. La consulta de anti acepta también `--chat` y `--socket`. Validación Rust y prueba integral en idkbot pendientes. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-26 — Ámbitos explícitos y preferencias de grupo

Cliente IPC y control WhatsApp conectados a la operación de presentación cifrada de Vault-Matter. `set style` y `set style --chat` gestionan el chat actual; `--user` selecciona el remitente o un único participante por mención/cita con comprobación de pertenencia; `set priority --chat` consulta o escribe el orden de resolución del grupo. Los ámbitos no implementados se rechazan sin redirigir mutaciones. Falta conectar/verificar el renderizador de todas las respuestas, probar la compilación y validar en ejecución las menciones y citas. No promover a producción. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-26 — Ámbito explícito --chat

El analizador común admite `--chat` como equivalente explícito al chat actual sin bandera, y `--socket` como alias de `--mod`. Los ámbitos son excluyentes. Se añadieron pruebas de equivalencia y rechazo de combinaciones. La integración de preferencias personales y prioridad de grupo continúa por separado; no se amplían permisos de comandos. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-26 — Formato Rustfmt de resolución PN/LID

Aplicadas las diferencias reportadas por idkbot en el filtro de PN y la prueba de conflicto de identidades del proveedor WhatsApp. Sin cambios funcionales. Vault ya autorizó las cuatro identidades globales y confirmó respaldo remoto cifrado (722 objetos); compilación y prueba real del proveedor siguen pendientes. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-26 — Consola con JID íntegros y resolución PN de grupos

La consola local muestra los JID completos (PN, LID, grupos y canales) sin abreviar; conserva el filtrado de IDs inválidos y la redacción de argumentos sensibles. Para comandos de un LID externo en un grupo, el proveedor resuelve el PN solo cuando el remitente exacto aparece en una única fila de metadatos del grupo y el PN es canónico e inequívoco; si sender_alt existe, debe ser consistente. Se vuelve a firmar el actor de control con HMAC antes de enviar a Base; el evento original y el chat real permanecen intactos. Si falta el mapeo o hay conflicto se conserva el LID y Vault decide con su registro exacto, sin otorgar rango por ser admin WA. No se cambia el enlace de terceros en chats privados ni se inscriben IDs reales. Añadidas pruebas sintéticas de resolución, conflictos y consola. CI, despliegue y comprobación de la identidad real pendientes. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-26 — Separar construcción verificada y arranque alpha

`cargo run` ya no invoca Cargo dentro del lanzador ni repite formato, check, test o build de los dos paquetes. Ejecutar `bash scripts/matter-verified-run.sh --build-only` una vez tras cambios: aplica gates de formato/check/test/Clippy estricto y `cargo build` para ambos manifiestos, sin conectar WhatsApp. El arranque exige SHA Git, árbol rastreado limpio y huellas SHA256 de ambos binarios y Cargo.lock que coincidan con la construcción validada. `cargo run -- --check-only` solo verifica la construcción y hace preflight sin conexión; Cargo externo conserva su comprobación incremental propia. `CARGO_TARGET_DIR` selecciona el target Base y `MATTER_PROVIDER_TARGET_DIR` permite separar el target del proveedor. Regresar con `tmux new-session -A -s matter`; detener únicamente la instancia anterior con Ctrl+C. Se conservan el guard de instancia única, sesión existente, Vault dedicado y pairing desactivado. CI y prueba real de este cambio pendientes. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-26 — CI: formato y orden de pruebas del rango global

Aplicadas las diferencias exactas de Rustfmt para `src/control_ipc.rs` y `src/vault_ipc.rs` del run 45926476278. Movido sin cambios el módulo `console_rank_ack_tests` al final de `providers/whatsapp-rust/src/lib.rs` para resolver `items_after_test_module` del run 45926476282; Clippy conserva `-D warnings`. Sin cambios en autoridad, criptografía, sesión ni comportamiento del ACK. CI y prueba real en idkbot pendientes. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-26 — Rango global verificado en consola

El proveedor ya no imprime el rango antes de recibir el resultado IPC. Con MATTER_WA_DIAG=1 el controlador comprueba nuevamente la firma HMAC del remitente de un comando admitido o denegado y pide a Vault una confirmación actual, con timeout de lectura/escritura diagnóstico de 250 ms. El ACK local opcional global_owner_verified=true solo cambia la etiqueta de consola a Owner global, con prioridad sobre Owner socket y Admin WA, sin alterar la aceptación o denegación del comando. Nunca se consulta para observaciones Anti, mensajes ignorados o duplicados, nunca se incluye el rango en la respuesta WhatsApp, no se cachean owners ni se confía en nombres. Los dos owners globales son pares. Si Vault no confirma el JID exacto, se conserva el rango local, no se afirma que el actor carezca de rango global. La resolución PN/LID de terceros y la mutación de roles no cambian en este checkpoint; verificar las identidades reales por separado. Pruebas de parsing de ACK y precedencia añadidas; CI/formato/compilación y prueba real pendientes. No se toca la sesión ni el servicio instalado. README revisados: 2/2.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-26 — Formato de la lectura offline de Vault

Se aplican los ocho diffs exactos de rustfmt del job 140260885962 (badf5fe), sin omitir las pruebas. Nuevas comprobaciones pendientes. README revisados: 2/2.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-26 — Respuestas cerradas y lectura cifrada fuera de línea

Por decisión del propietario, las respuestas individuales usan el cierre Matter completo; solo menús conectados y terminal conservan sus marcos independientes. El cliente de configuración Vault mantiene instantáneas XChaCha20-Poly1305 de las lecturas/ACK confirmados para estilo, idioma, modo y Anti, con clave derivada de la credencial local mediante HMAC, contexto vinculado al cliente/tenant/bot/socket/registro, ficheros privados y sustitución atómica. Solo se consultan ante socket ausente o conexión rechazada; no ante denegación, permisos, autenticación inválida ni corrupción. Las ausencias confirmadas también se registran. El comando de estilo identifica lecturas en caché y ausencia de copia; no se inventa un valor guardado. Las escrituras siempre usan get/put o global_style_set en vivo, sin cola offline. No se cachean owners, permisos, secretos, credenciales ni health. El arranque permite modo offline solo para servicio no alcanzable, conservando controles de identidad, sesión y archivos privados; un handshake denegado con socket alcanzable sigue bloqueando. Las lecturas offline pueden estar desactualizadas, y necesitan copia previa; CI y prueba de servidor pendientes. README revisados: 2/2.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-26 — Formato CI de configuración Vault

Aplicados exactamente los diffs de rustfmt de los jobs 140247506586 y 140247505975 del SHA fd534ff. No se desactivan validaciones ni se modifica la política de configuración. Nuevos CI pendientes; README revisados: 2/2.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-26 — Configuración en Vault y estilo Matter por ámbito

Decisión del propietario: todos los `set` y las configuraciones persistentes pertenecen a Vault-Matter; nunca confirmar como guardado un cambio local no sincronizado. `status` utiliza el cierre completo del tema Matter. El nombre público `classic` se retira de estilo y selector de menú; las preferencias SQLite históricas se traducen al nombre Matter cuando se abren, sin alias público. `set style [matter|none|inherit]` acepta ámbitos de chat, `--mod` (socket), `--global` (ecosistema); las escrituras de grupo exigen admin WA verificado por metadatos frescos o propietario autorizado, las del socket dueño/global y las globales un owner verificado en Vault. Precedencia chat > socket > global. El marcador interno está firmado por HMAC y no autoriza otros comandos. Vault alpha 435e982 añade lectura/escritura global. Las preferencias visuales (estilo, idioma, modo) y las reglas/listas Anti operativas de chat/socket usan registros Vault cifrados con revisión; sin Vault se rechazan escrituras, no hay persistencia local alternativa. La base de menú de producción se mantiene en memoria para la interfaz de render; archivos SQLite antiguos no se borran ni se importan automáticamente. Los permisos de roles globales siguen en Vault; las mutaciones de roles locales siguen bloqueadas en el controlador WA sin registro autorizado. Persisten pendientes: migrar preferencias antiguas de menú/Anti a Vault de forma controlada, configuración global Anti/idioma/modo, activación y políticas de bot/socket, auditoría y configuraciones de otros componentes. No se afirma migración integral del ecosistema. CI, revisión rustfmt y prueba real pendientes. README revisados: 2/2.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-26 — Identidad oficial: estilo Matter

El propietario corrige la nomenclatura: el estilo visual propio es Matter, no Classic. `templates/menu/theme.json` declara `theme: matter`, `matter_frame` y `matter_templates`, con consola independiente en el mismo archivo. `set style matter|none` es la interfaz oficial; `classic` y `default` siguen admitidos por compatibilidad y el valor histórico SQLite `classic` se conserva para no migrar preferencias innecesariamente. El modo de presentación `menu classic` permanece separado del nombre del estilo. Los veinte layouts unificados conservan contenido y símbolos. Se aplican exactamente los dos diffs de rustfmt del proveedor registrados por CI de fcf6f63; el CI de núcleo de ese SHA completó pruebas y Clippy pero falló por omisión de los cuatro documentos de continuidad, corregida aquí. Nuevos CI pendientes; no se omiten validaciones. HTML experimental, permisos, sesiones y Vault sin cambios. README revisados: 2/2.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-26 — Classic unificado en un archivo

`templates/menu/theme.json` es la única fuente de los diseños Classic: `classic_frame` conserva adornos de WhatsApp, `classic_templates` define control, menú heredado, marco de mensajes y Pong; `classic_templates.menu` contiene las doce secciones del menú dinámico; `console.templates` define los cuatro layouts independientes de la terminal, junto a su paleta y textos existentes. Se eliminan los veinte archivos TXT sustituidos. El núcleo, proveedor y pruebas usan los mismos accesos en `src/classic_template.rs`; las plantillas Plain y las configuraciones personalizadas no se modifican. La migración conserva saltos de línea, símbolos, campos y colores, y exige recompilar. Sin cambios en transporte HTML, permisos, sesiones, Vault o MSEL. CI y prueba visual pendientes. README revisados: 2/2.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-26 — Consola independiente y diagnóstico de menú HTML

Alpha separa encabezado, filas normal/final y cierre de terminal en templates/console. Se conserva la paleta y se alinea el cierre de terminal `╰──╮╭───────⟨ 🌀` con el encabezado; el marco WhatsApp conserva `╰─╮╭────────⟨ 🌀`. Tras ACK aplicado, los menús web de grupos pueden intentar RichResponse aun si el remitente no es la cuenta vinculada; no se amplían comandos mutantes, pruebas HTML privadas ni permisos. La consola distingue desactivación, autosend, contexto privado, construcción, rechazo y timeout; no reintenta HTML incierto. Si HTML no se intenta o falla de forma definitiva, el selector nativo conserva el texto como alternativa; no se afirma render HTML. Se expande el menú de vista previa y se registra el flag autosend. La capacidad experimental depende del cliente WhatsApp; compilación, CI y prueba visual pendientes. README revisados: 2/2.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-26 — Cierre conectado exacto solicitado por el propietario

En alpha, el encabezado compartido conserva `╭─ 💠` y el cierre conectado de `templates/menu/theme.json` pasa a `╰─╮╭────────⟨ 🌀`. Se conserva el conector previo, cierre independiente, Pong, temas personalizados, paleta, ramas, filas, fuentes y layouts. El cierre conectado se propaga a consola, menú Classic y marco Control; su unión queda una columna a la izquierda respecto del conector anterior por el diseño exacto solicitado. La prueba del proveedor autónomo expande la plantilla antes de comparar sus extremos; se mantienen las aserciones de contenido, saltos de línea e idempotencia. Se aplica además el formato de menu_catalog.rs reportado por el servidor. No se cambian permisos, sesiones, Vault ni MSEL. Recompilación y CI pendientes de verificar. README revisados: 2/2.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-26 — Consola como referencia oficial del estilo Classic

El propietario prefiere el diseño de la consola y exige que su presentación también proceda de la plantilla. templates/menu/theme.json define el emblema único, prefijo del encabezado, ramas, líneas, cierre conectado con ✦ y conector centrado. Console, menú Classic y marco de control usan el mismo cierre conectado. El prefijo de las filas de comandos se comparte con las ramas de consola; las secciones informativas conservan su estructura y Pong su cierre histórico 🌀. La paleta ANSI, etiquetas y distribución de los datos de consola ahora también están en el JSON; la lógica solo selecciona colores semánticos y aporta datos. No se emiten secuencias ANSI en WhatsApp. Se aplicaron las cinco diferencias únicas de rustfmt de los CI del núcleo y proveedor del SHA 50905c6; también se adaptaron las dos aserciones del catálogo que exigían el cierre y conector antiguos, verificando su composición mediante la plantilla compartida. No se desactivan pruebas. CI nuevo pendiente; verificar ambos workflows antes de declarar aprobado. Cambiar las plantillas exige recompilar; se mantiene la alineación de dos celdas del emoji. No se modifican permisos, sesiones, Vault ni MSEL. README revisados: 2/2.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-26 — Plantilla clásica compartida y centrado del emblema

El propietario exige que los cambios de plantilla se propaguen a los consumidores del diseño clásico. La fuente común de adornos es templates/menu/theme.json (classic_frame y decoration); src/classic_template.rs la expande antes de insertar datos y se comparte con el proveedor autónomo. Menú Classic dinámico, menú de control heredado, marco de control, marco de mensajes, Pong y consola consumen esos componentes. Los layouts propios declaran contenido y estructura; los datos no se reinterpretan como adornos. El cierre conectado y el conector se centran en columnas 4–5, contando desde 1, suponiendo un emoji de dos celdas. Se añade el mismo patrón de ramas del menú a la consola. El menú plain y temas personalizados no se modifican. Los binarios deben recompilarse al modificar la plantilla; no hay hot reload. No se cambian permisos, Vault, MSEL ni sesión. El CI del SHA 53687dc aprobó ambos workflows; el nuevo cambio requiere verificación de formato, compilación y render real en SSH/WhatsApp. README revisados: 2/2.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-26 — Pong oficial Matter-Bot en el control WhatsApp

Solicitud del propietario: utilizar la plantilla exacta de docs/MATTER_BOT_PING.md en =/>ping. El proveedor muestra un solo Pong con Socket (RTT IPC local autenticado), Process (verificación local más procesamiento posterior al ACK, sin consulta diagnóstica) y Total (suma), medidos con reloj monotónico y tres decimales; excluye el formateo y envío posteriores. No equivale a RTT remoto de WhatsApp. Se evita el marco Control adicional únicamente para este Pong; otros comandos conservan sus temas. Solo un ACK aplicado permite responder; en chat apagado ping permanece silenciado, sin modificar admisión, permisos, Vault ni sesión. El futuro #ping del bot sigue separado. Prueba exacta del marco y valores añadida. CI del SHA 9eb25de aprobado en núcleo y proveedor; CI nuevo y prueba real pendientes. README revisados: 2/2.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-26 — Identidad visual Matter y formato del proveedor

El propietario solicita conservar el símbolo 💠, la tipografía 𝗠𝗮𝘁𝘁𝗲𝗿 y el marco original de consola. Se restauran el encabezado, las líneas │·♪· y el cierre ╰─╮╭────⟨ ✦, manteniendo los límites, rangos verificados y colores de la consola compacta. Se separa el rango del nombre para reducir desbordamientos. Aplicadas las ocho diferencias exactas de rustfmt del CI del proveedor para a6c52e3, incluida la aserción reportada en idkbot. CI del núcleo de ese SHA aprobado; proveedor bloqueado en formato antes de compilar. Nuevo CI pendiente; no se modifica la admisión, las sesiones ni Vault. README revisados: 2/2.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-26 — Consola compacta de comandos para SSH

Por solicitud del propietario, MATTER_WA_DIAG=1 muestra encabezado con evento y hora UTC, origen, comando con argumentos de vocabulario permitido (máximo 24 caracteres), usuario hasta 18 caracteres, grupo hasta 20, identificadores abreviados, rango del socket verificado por el proveedor o administrador/miembro de WhatsApp según metadatos; el rango global Vault no se infiere y queda sin verificar cuando no hay dato. ANSI brillante por campo y resultado, sin ANSI con NO_COLOR, TERM=dumb o salida no TTY. IPC/TX separados y acotados; aceptación no confirma visualización en el teléfono. Tokens desconocidos y argumentos sensibles se ocultan; no se registran cuerpos ni secretos. No se cambian permisos, comandos, menús, sesiones ni Vault. El CI anterior 061908a pasó pruebas Linux/Windows y Clippy del núcleo y proveedor; falló exclusivamente el gate documental por no actualizar los cuatro registros obligatorios. Este commit los actualiza y conserva la política status-only. Nuevas pruebas y render real pendientes. README revisados: 2/2.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-26 — Regresiones de status único y recuperación local

Se actualizan las pruebas de control_extensions, group_control y whatsapp_process para la política de admisión: ping falla si el socket está encendido pero el chat no tiene permiso; las solicitudes de chat apagado se ignoran sin respuesta; Vault sigue denegando solicitudes sin credenciales aun con el chat activo. La prueba de proceso verifica bloqueo de recuperación local mientras el controlador está vivo, activación local tras detenerlo y persistencia de replay después de reinicio. Se aplican las diferencias de rustfmt del CI anterior. No se omiten pruebas, no se cambia la implementación de activación ni la sesión. CI nuevo pendiente.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-25 — Solo status en chats desactivados

Política del propietario: el estado efectivo de cada chat se comprueba antes de ejecutar cualquier comando de WhatsApp, incluso si el remitente es owner global. Solo `=/>status` (y su alias `=/>estado`, incluidas sus variantes de consulta sujetas a permisos) permanece disponible. Ping, registro, menús, pruebas, cambios de activación y operaciones Vault quedan silenciados al estar apagado; las solicitudes descartadas no pueden reproducirse tras activar el chat. La administración local autorizada conserva una vía de recuperación separada: detener el controlador y ejecutar `whatsapp-matter-control --set-chat-policy self on` con el entorno privado existente; admite también group/private/channel/community, --mod y un JID de grupo. No se modifica la sesión de WhatsApp ni las claves. Los canales y avisos sin metadatos de plataforma autenticados continúan sin ingreso verificable; no se concede acceso por un JID declarado. CI y pruebas reales pendientes.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-25 — Consola compacta y ping de control

`MATTER_WA_DIAG=1`: cuatro líneas estilo Matter, colores ANSI solo en TTY (NO_COLOR desactiva), nombre de perfil, username/LID si WhatsApp los facilita, nombre y JID del grupo, IPC y TX juntos con latencias y resultados. Consulta de metadatos de grupo limitada a 500 ms, caché de 256 entradas durante cinco minutos, sin efecto sobre permisos; sin dato se muestra `-`. `=/>ping` es control de lectura autenticado, disponible incluso con bot OFF, mide RTT local del puente HMAC en milisegundos; no mide latencia del teléfono. Añadido a Información, ayuda y botón. `#ping` sigue fuera del control. No se cambian sesiones ni Vault. CI y formato pendientes; probar en dispositivo antes de declarar estable.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-25 — Diagnóstico compacto de comandos WhatsApp

Con `MATTER_WA_DIAG=1` la consola local identifica cada comando por número correlativo, hora UTC, origen (texto/botón/cita), nombre de perfil no verificado, JID de usuario y chat, etapas IPC y TX, resultado y tiempo total. No registra mensajes normales, argumentos sensibles, respuestas, claves ni payloads de error. Los JID son datos personales: desactivar con `MATTER_WA_DIAG=0` antes de compartir registros. El nombre de grupo no se consulta todavía; se identifica por JID para no introducir tráfico ni bloqueos de metadatos. Un ACK IPC o TX no confirma renderizado en el teléfono; un timeout indica entrega incierta. También se acotan los avisos de error IPC a ocho segundos. Se conserva la separación web/classic/buttons y no se habilitan reintentos, pairing ni cambios en Vault. Prueba de redacción incluida; CI y ejecución real pendientes.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-25 — Rustfmt del catálogo compartido

CI del SHA 0a2b1e6 y servidor idkbot detuvieron el arranque en formato. Aplicadas todas las diferencias exactas de rustfmt de los dos workflows Linux: src/menu.rs, src/control_ipc.rs y proveedor src/lib.rs y src/ui.rs. Sin cambios de comportamiento, permisos, sesión ni Vault. README revisados: 2/2. Nuevo CI pendiente; no declarar aprobación hasta comprobar resultados.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-25 — Vistas independientes y categoría compartida

Solicitud del propietario: web sin texto de respaldo ni marca en el selector; Buttons no debe copiar el clásico; un ID canónico por categoría con presentación según el formato. El controlador deriva descriptores de botones del mismo CATEGORIES/CommandSpec y los adjunta únicamente al ACK autorizado. El proveedor muestra respuestas rápidas de hasta tres por tarjeta y selector final exclusivo «Otros menús». Los ID de navegación Buttons contienen el ID canónico y conservan el contexto al pulsarlos incluso desde una tarjeta anterior; el último formato se recuerda por chat/remitente en memoria acotada para los comandos escritos. Los botones de comandos sin argumentos seguros muestran ayuda explícita, no mutan configuración. Todo agrega navegación por categorías; las páginas de una categoría conservan su identificador. El catálogo web por categoría queda pendiente: las consultas desde web siguen usando clásico mientras no se verifique el HTML. Web no tiene cuerpo nativo textual; HTML y selector son transportes separados, sin garantía de renderizado del selector sin cuerpo. No se modifica Vault, autenticación ni sesiones. README revisados: 2/2. CI, formato y prueba real pendientes.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-25 — Clippy del proveedor y verificación del formato web

CI del SHA e076cfd: núcleo aprobado y proveedor pasó formato, compilación y pruebas antes de fallar en Clippy por single_match (main.rs). Se sustituye el match exterior por if let sin cambiar la admisión, los ACK ni el silencio ante rechazos. Se mantiene la corrección rustfmt de tests/ui.rs. README revisados: 2/2. Nuevo CI pendiente; no se modifica la sesión ni Vault.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-25 — Formato de la prueba de menú web

El propietario reportó bloqueo MATTER_GATE=FORMAT en idkbot. Se aplica exactamente la diferencia de rustfmt en providers/whatsapp-rust/tests/ui.rs; no cambia el comportamiento, la sesión ni Vault. README revisados: 2/2. CI y arranque pendientes de confirmación.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-25 — Separar HTML real del menú clásico

El propietario informa que web muestra el marco clásico precedido por «Menú web». Se retira ese encabezado y la copia del cuerpo clásico del NativeFlow web. El proveedor vuelve a intentar el RichResponse HTML real con el contenido autorizado del controlador, solo para el propietario de la sesión autenticada en chat propio o grupo habilitado; requiere MATTER_WA_RICH_HTML=1 y, para el menú raíz automático, MATTER_WA_HTML_AUTOSEND=1. El selector nativo «Otros menús» conserva exclusivamente los formatos alternativos y un cuerpo mínimo de marca. **Excepción de transporte:** web puede enviar dos mensajes (HTML experimental y tarjeta nativa de navegación), ya que no hay representación combinada verificada; classic/buttons siguen con una sola tarjeta. Si HTML no se representa, no se suplanta con el clásico. La aceptación del envío no confirma el renderizado del cliente. Prueba de regresión del cuerpo web añadido; CI y dispositivo pendientes. Sin cambios en Vault, permisos ni sesiones. README revisados: 2/2.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-25 — Consola sin spam de recepción

Por solicitud del propietario, el proveedor WhatsApp ya no imprime callbacks generales, respuestas interactivas desconocidas, ediciones, candidatos, rechazos previos de firma ni ACK ignorados o duplicados. El flujo de comandos retorna temprano para texto sin prefijo de control y registra una sola confirmación al recibir ACK aplicado del controlador; el diagnóstico distingue una selección interactiva aceptada sin registrar contenido ni identificadores. La observación pasiva necesaria para antis permanece intacta y silenciosa en operación normal; una caída de transporte se informa una vez hasta recuperación. Se conservan errores operativos de comandos y pruebas explícitas, identidad, firma, antirreplay, Vault y sesión. README revisados: 2/2. CI y despliegue pendientes.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-25 — Selector exclusivo «Otros menús»

Corrección solicitada por el propietario: la tarjeta conserva su contenido y un único selector final «Otros menús». No se muestran categorías, páginas ni accesos a categorías en ese selector. Cada formato explícito ofrece los otros dos; la vista sin formato específico ofrece los tres. Las categorías y los comandos manuales del catálogo se conservan. El proveedor conserva el contenido real del controlador en web/buttons como respaldo textual, no HTML real, y resuelve correctamente el modo clásico persistido. El receptor admite variantes de ID de selección y nombre NativeFlow ausente, siempre limitados a la lista de comandos de lectura existente; mantiene HMAC, identidad y antirreplay. Se agregaron pruebas de ida y vuelta de cada formato, variantes de respuesta y rechazo de mutación. El diagnóstico no registra contenido ni identificadores privados. README revisados: 2/2. CI, instalación y prueba de selección real en WhatsApp pendientes; no se cambian sesiones ni Vault.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Arquitectura aprobada: Engine-Matter y Extension-Matter

El propietario dispone que WhatsApp-Matter se renombre a **Engine-Matter** (núcleo de sistemas Matter sin soporte directo WhatsApp/Telegram/Discord) y Telegram-Matter se renombre a **Extension-Matter** (conectores externos WhatsApp/Telegram/Discord y futuras redes). Discord-Matter será retirado de forma controlada con respaldo. **Matter-MSEL/** tendrá su ubicación en Engine-Matter como integración del lenguaje oficial independiente MSEL. Social-Matter es sistema propio del motor; Bot-Matter sigue bloqueado. **Extensions-Matter** (plural) no es el nuevo Extension-Matter y continúa separado. Se conserva Base-Matter como biblioteca y contratos sin dependencias cíclicas.

La preparación actual modifica solo documentación de `alpha`: no mover el runtime/proveedor WhatsApp, no tocar sesiones, cifrado, comandos ni generar nuevos sockets. Plan completo en [docs/ENGINE_EXTENSION_REFACTOR_PLAN.md](../../docs/ENGINE_EXTENSION_REFACTOR_PLAN.md). Revisados README de raíz y de proveedor; ningún despliegue realizado.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Activación segura de Vault alpha documentada para idkbot

Nueva guía `docs/ACTIVAR_VAULT_ALPHA.md`: diagnosticar el servicio y `base-wa-alpha`, probar ping y self-test sintético, verificar el lanzador `--check --vault-env`, y reiniciar de manera controlada el único proveedor alpha existente con `--resume-only --vault-env`. Preserva `wa-alpha`, evita reenrolar, mezclar `wa-test-1`, generar pairing o tocar las claves. Hace explícita la validación CI de binarios, revisión de credenciales y limitación del grant de configuración. La activación no desbloquea `=/>register start`, roles ni registro civil. Documentación reflejada en Vault-Matter alpha; no hay despliegue realizado desde esta conversación. README raíz y README proveedor revisados (2/2); sólo documentación alpha.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Corrección del test de cierre visual del menú tras CI

El commit 4b1a307 cambió correctamente el espacio de «╰─╮╭────────⟨ 🌀», pero la prueba unitaria src/menu.rs continuaba buscando el texto anterior sin el espacio. Los tests predeterminados de Linux y Windows fallaron por esa aserción obsoleta; formato y compilación pasaron. Se actualiza únicamente la expectativa del test para igualarla al diseño aprobado, sin modificar renderizador, sesiones, Vault, proveedores ni procesos en ejecución. CI del SHA resultante pendiente. README revisados: 2/2; documentación de desarrollo y continuidad sincronizada.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Orden y espacios del menú clásico de control global

- Orden aprobado de la vista clásica pública: Categorías → Información Matter → ficha de Usuario → Instrucciones, conservando el marco ornamental y la identidad Matter.
- Espaciado literal verificado: encabezados con «╭─ 💠», cierre de categorías con «⟨ 🌀», sin línea vacía entre el conector «││» y la información; una línea vacía entre información/ficha de usuario y entre ficha/instrucciones. Sin triples saltos de línea.
- Los bloques exclusivos del propietario verificado («Control del socket» y «Sistema · Host») continúan disponibles después de la ficha; no se revelan al público. No se cambian permisos, sesiones ni lógica de comandos.
- Los campos dinámicos se conservan (fecha, versión, cantidades y datos del usuario). Se actualizan las plantillas, el formateo de los bloques opcionales y las aserciones de tests/menu_catalog.rs. Cambios solo en alpha; CI y prueba real en WhatsApp pendientes.
- Revisión documental de ambos README, docs/DEVELOPMENT_STATUS.md, docs/WORK_CONTINUITY.md y docs/MENU_TEMPLATE_ENGINE.md.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Control del ingreso WhatsApp: rechazo estricto de variantes de lectura

El CI del SHA cec5997 reportó 17/18 pruebas del proveedor correctas y fallo en incoming_readonly_commands_bind_real_sender_and_never_gain_owner_authority. Causa: el fallback genérico de signed_public_readonly admitía variantes rechazadas por el catálogo (por ejemplo =/>menu unset, =/>status --debug, =/>test run). Se limita ese fallback a verbos con potencial de mutación que el controlador y Vault autentican; todas las rutas de menú, estado, ayuda, prueba visual, Vault y crédito usan patrones explícitos de admisión. Se conserva verificación de remitente, HMAC, alcance de socket y ausencia de privilegios automáticos para terceros. Prueba real de vinculación no realizada, nuevo CI pendiente. No tocar sesiones, bases SQLite ni servicios idkbot. README revisados (2/2).

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Test de menú corregido para etiqueta de plantilla

CI Linux de 0fa6284 verificó Rustfmt y compiló; 6/7 pruebas de menu_catalog pasaron y una conservaba la etiqueta literal obsoleta «Comandos de catálogo» pese al contrato nuevo @LabelCommands: @Commands. La prueba ahora valida Comandos: [menu::command_count()] en vez de un texto anterior. No cambia fuente de datos ni estilos, no hay despliegue WhatsApp. README revisados: 2/2. Mantener registros continuos y MSEL P0. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Compilación del motor de menú: corrección de préstamo mutable

GitHub Actions para 8bd87f verificó cargo fmt, pero cargo check identificó E0499 en src/menu/theme.rs: el argumento de v.insert("Pagination", if...) intentaba modificar v dentro de otro préstamo mutable. Se calcula pagination primero y se inserta después, sin cambiar estilos, comandos ni permisos. CI del nuevo SHA pendiente; no se instaló en WhatsApp. README revisados: 2/2, continuidad sincronizada por commit. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Ajuste exacto de rustfmt al motor de plantillas

El CI Linux del SHA 7606e5a mostró únicamente diferencias de formato en src/menu.rs, src/menu/theme.rs y src/menu/telemetry.rs; este commit incorpora los 28 cambios exactos de rustfmt de GitHub Actions. Se conserva la separación de diseño en templates/menu/, datos dinámicos y seguridad; no hay cambios en sesiones WhatsApp, Vault o permisos. No declarar aprobación hasta verificar CI del SHA actualizado. README revisados: 2/2. Continúa MSEL P0. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Menú dinámico con plantillas independientes

Base-Matter/alpha sustituye la presentación incrustada en Rust por templates/menu/theme.json y fragmentos classic/plain. El proveedor continúa enviando los controles de menú autenticados al controlador; no se ha cambiado su sesión ni activado HTML externo. Valores @Commands y @CategoriesActive se derivan del catálogo registrado, mientras que @CategoryName utiliza datos de categoría, no directorios. No confundir @ControllerRAM ni @Uptime con RAM/tiempo del proceso WhatsApp-Rust: falta telemetría auténtica de proveedor. El fallback de menú autónomo anterior permanece separado para compatibilidad hasta migrar con pruebas. Pendiente CI Linux/Windows/proveedor y teléfono antes de instalar. Véase docs/MENU_TEMPLATE_ENGINE.md. No tocar sockets existentes.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Revisión de integración de información automática

Base-Matter/alpha añadió datos de solo lectura del controlador al menú, sin cambios a la sesión WhatsApp-Rust. No declarar memoria RSS/uptime del proveedor, sub-bots, premium o enlace de la instancia como métricas reales hasta incorporar un registro y un contrato de telemetría firmado. Los datos de host solo aparecen para el owner del socket autenticado. CI de integración y prueba en dispositivo pendientes. No reiniciar ni migrar proveedores activos del servidor compartido idkbot. Contrato: docs/MENU_TELEMETRY_AUTOMATION.md en Base-Matter.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Integración pendiente de métricas en el menú

`Base-Matter/alpha` integra información dinámica general en `=/>menu`. Las cifras de uso del servidor son del host y el RSS/uptime mostrados corresponden **al proceso controlador Base**, no al proveedor WhatsApp-Rust. Para acreditar el campo «RAM del bot», el proveedor deberá reportar telemetría verificable del proceso por un contrato autenticado, sin exponer sesiones, tokens, JID de terceros ni identificadores privados. También faltan contadores verificados de sub-bots y premium y URL por instancia. No hay cambios de conexión, sesiones, roles, ni botón en WhatsApp en esta revisión. Contrato: [docs/MENU_TELEMETRY_AUTOMATION.md](../../docs/MENU_TELEMETRY_AUTOMATION.md). Prueba real de dispositivo y CI pendientes.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-21 — Windows Clippy MSEL test imports

En CI de Base-Matter Windows, `cargo check` y todas las pruebas Rust terminaron correctamente para el SHA `1cb11f4`, pero Clippy `-D warnings` detectó dos imports sin uso (`MselCommandRunner` y `PathBuf`) en `tests/msel_bridge.rs`, ya que el test que los usa es solo Linux. Se limitan los imports a `#[cfg(target_os = "linux")]`, conservando las pruebas multiplataforma y el test original opcional. No cambia el motor MSEL, sesiones WhatsApp ni Vault. README revisados: 2/2. [CI de diagnóstico](https://github.com/Matterssmith/Base-Matter/actions/runs/44954149724). Nuevo CI pendiente.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-21 — Test real opt-in Base → lenguaje MSEL

El pipeline Base-Matter Rust en Linux finalizó **SUCCESS** para el commit `1cb11f4b2f36fe963e3d00a6e324c7f6af2c1e0c` ([ejecución 44954149724](https://github.com/Matterssmith/Base-Matter/actions/runs/44954149724), job Linux). Se añade test explícitamente ignorado por defecto `original_msel_checkout_e2e` para conectar **el host Rust de Base** con la **biblioteca nativa C++ real de MSEL privado** en el servidor del dueño, sin copiar ni exponer su código. Requiere `MATTER_MSEL_CHECKOUT` y compilación de `build/libmsel_embed.so`. El nuevo test no afirma ejecución verificada hasta que se ejecute. Ninguna sesión, llave ni transporte WhatsApp se abre o cambia. README revisados: 2/2. Bot-Matter bloqueado.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-21 — Matter-MSEL (separación de transportes)

El proveedor activo `whatsapp-rust` se mantiene sin cambios en esta etapa. `Base-Matter/alpha` contiene un bridge opcional de mensajes autenticados hacia scripts del **lenguaje original MSEL**; `WhatsApp-Matter/matter` aporta el contrato neutral. Aún no hay una ruta de envío live conectada por MSEL, ni activación automática de bots en grupos. Sesiones, SQLite, pairing, llaves y Vault **no se migran**. [Manual](../../docs/MSEL_WHATSAPP_INTEGRATION.md). README revisados: 2/2. CI pendiente.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-21 — LuferOS/Baileys-Rust: referencia externa sin cambio de provider

La implementación `experiments.rs` de LuferOS se revisó y copió íntegra en `Base-Matter/third_party/luferos-baileys-rust/` con Apache-2.0 y NOTICE de autoría. **No** se vinculó a esta biblioteca `whatsapp-rust` 0.7.0: usa `baileys-rs` y protobuf `prost` diferentes del provider actual (`wa`/`buffa`). La referencia expone botones NativeFlow, `open_webview` con URL, carruseles y RichResponse, no un motor que procese HTML/CSS/JS inline.

**Estado:** ensayo de HTML existente permanece desactivado por defecto y limitado a self-chat autorizado; ningún test de dispositivo confirmó renderizado. No se cambió emparejamiento, sesión, roles, antis ni control de socket. La selección futura de bibliotecas `baileys-rust`, `whatsmeow` o `xyzbaileys` es un descriptor offline de Bot Factory v2, **no una instalación de este provider**. [Plan y límites](../../docs/BAILEYS_RUST_EXPERIMENTS_BOT_FACTORY_PLAN.md).

## 2026-11-20 — CI: enlace de permisos globales con WhatsApp

- Se aplicaron las diferencias exactas `rustfmt` al filtro de comandos del proveedor. El controlador Linux ya superó formato, pruebas y Clippy.
- Los owners 1 y 2 poseen igual autoridad una vez autenticados mediante JID firmado y cliente Vault. Los remitentes no autorizados permanecen read-only.
- README revisados: 2/2. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Igualdad real de privilegios de ambos owners globales

- La autoridad de global owner 1 y 2 es idéntica. No existe rango subordinado: ambos pueden ejecutar los controles de Base en cualquier socket Matter cuyo proveedor valide el JID y cuyo cliente privado Vault verifique la membresía global del remitente.
- `WhatsAppControlBridge` añade un adaptador global verificado **antes** de la ruta de lectura pública. Sólo el sender externo autenticado con HMAC + `VaultClient::require_global_owner` obtiene permiso delegado para los controles del socket; el resto queda en `dispatch_public_readonly` sin mutación. Los controles de administración y grupo siguen las reglas funcionales existentes, como destino/argumentos y estado del bot.
- `whatsapp-rust` puede firmar comandos administrativos recibidos de otros usuarios en chats/grupos sin dar privilegios automáticamente: sólo Vault puede elevar a uno de los dos owners globales. Ningún comando de texto, alias o rol de grupo supera la comprobación.
- `=/>set role owner --global NUMERO` acepta owner global 1 o 2, con alcance de chat privado para esta modificación. El cliente deja de informar erróneamente que los roles/controles globales están siempre bloqueados. Revocación cruzada en el servicio Vault en rotación de owner 2.
- **Límite real:** sólo controles que existen en Base quedan ejecutables. Telegram/Discord no tienen todavía proveedores operativos ni callback firmado; no se puede proclamar que ya manejen comandos. No se ha desplegado este SHA en instabox-se.
- Pruebas de seguridad deniegan escalamiento por un sender firmado sin membresía Vault. README revisados: 2/2, los cuatro documentos de continuidad actualizados de forma atómica.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Rustfmt: concesión global owner 2

- Se aplicaron automáticamente los cambios exactos de rustfmt detectados en CI al cliente Vault, puente WhatsApp, pruebas de aislamiento y proveedor Rust. Sin nuevas capacidades ni cambios de secretos.
- El comando `=/>set role owner --global NUMERO` se autoriza sólo desde el owner 1 autenticado por Vault en chat privado. Tests y binarios nuevos deben terminar con CI success antes de instalarlos.
- README revisados: 2/2; documentación de continuidad actualizada. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Real Vault =/>set role owner --global

- Ruta global tipada: `=/>set role owner --global NUMERO` en chat privado, con PN auténtico firmado por WhatsApp y validado en el puente; la operación se envía exclusivamente al servicio privado Vault como `owner_grant_secondary`. El owner del socket no hereda autoridad global.
- El servicio Vault restringe la mutación al principal global slot 1 previamente enrolado, asignando sólo slot 2 / WhatsApp. Rechaza autoasignación, dueño de bot, otros remitentes, números no canónicos y grupos. El slot 1 no se puede modificar desde WhatsApp.
- Sin número, devuelve el uso al principal autorizado, sin cambio de roles. La respuesta satisfactoria no muestra el número destino; la prueba procesal sin Vault debe denegarlo. No se incluyen números reales en repositorios.
- **No se ha instalado la versión nueva en instabox-se**. Se requiere compilar Vault y Base desde los SHAs CI aprobados, reemplazar control/proveedor con sesión detenida y registrar primero el PN global 1 de forma privada. No se han implementado otros roles globales ni los adaptadores Telegram/Discord.
- README revisados: 2/2, cuatro documentos de continuidad actualizados; créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Política owner global principal en todas las redes

- Ancla privada: el teléfono público del propietario es el principal global 1 en Vault, sin colocarlo en el repositorio público. Cada bot y plataforma deben comprobar la identidad real del evento y consultar `owner_check` cifrado antes de responder `=/>vault`.
- WhatsApp prueba su JID; Telegram y Discord requieren vincular sus identificadores estables una vez, no conceder privilegios por un número o nombre que alguien escriba. El owner 2 mantiene identidades diferentes, seleccionables por plataforma.
- Se aplican dos ajustes rustfmt detectados por CI para el cliente v2 de plataforma. La compatibilidad de cliente v1 sigue limitada a WhatsApp y Vault se mantiene privado. El servidor no se ha actualizado.
- README revisados: 2/2. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Cliente global de Vault con identidad inmutable por red

- Base-Matter introduce `require_global_owner_for_platform(platform, subject_id)` para el contrato v2 de Vault, con `whatsapp` PN/LID real, `telegram` user.id y `discord` user.id. Username/nickname es solo un alias visual y no se convierte en principal por texto.
- `require_global_owner` existente sigue soportando WhatsApp. Toda autorización usa paquete cifrado autenticado al servicio privado y compara la respuesta de forma estricta. Identidades erróneas o plataformas no autorizadas fallan sin filtrar nombres.
- Nuevo diagnóstico CLI `matter-vault-client owner-check-platform SOCKET CLIENT KEY TENANT BOT SOCKET_ID PLATFORM` lee únicamente `subject_id` por stdin, no imprime números ni IDs. La cuenta pública del operador se introduce al registrar Vault local; no existe número fijo en código.
- Contratos disponibles para Telegram/Discord sin afirmar integraciones en ejecución. En WhatsApp el proveedor usa los JID obtenidos de eventos autenticados. Nunca confiar en un @username escrito en un mensaje.
- Requiere actualizar binarios de Vault y Base tras CI y enlazar cada bot adicional con su propio grant. README revisados: 2/2 y cuatro documentos de continuidad sincronizados. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Verificación host-local de owner global desde stdin

- El cliente público `matter-vault-client owner-check` lee un JID canónico de stdin, verifica por AEAD contra el registro cifrado de Vault-Matter y devuelve solo `global_owner_authorized` o error genérico. No acepta el JID en argv, no imprime números, no consulta registros de roles.
- Usado para E2E con dos identidades sintéticas. En producción las dos identidades reales se enrolan exclusivamente bajo el UID privado `matter-vault`; no introducirlas en Git ni en chat.
- README revisados: 2/2 y cuatro documentos de continuidad actualizados en el mismo commit; créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Protección de replay en comandos Vault globales

- El puente autentica al JID global con el servicio privado, reclama `message_id` durable contra replay, y **solo después** permite `=/>vault test` (escritura sintética). Repite autorización antes de ejecutar para negar si se revocó el principal entre ambos pasos. Replays y terceros no pueden provocar un nuevo write.
- Sin cambios de scopes, roles, secretos, permisos de grupos ni registro público. README revisados 2/2, documentos de continuidad sincronizados en este commit; créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Correcciones de formato Rust de autoridad global

- CI original: Rustfmt falló antes de compilar en las rutas del puente/cliente y una prueba; se aplicaron los diffs exactos del log. La comparación de la prueba de respuesta AEAD usa `is_some_and` para evitar ambigüedad del patrón de slice.
- Ningún cambio de privilegios, números, claves ni roles. Servicio Vault privado y controlador público continúan pendiente de despliegue coordinado y enrolamiento de dos identidades.
- README revisados: 2/2 y documentación de continuidad actualizada en este mismo commit. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Acceso Vault reservado al owner global de dos números

- El servicio privado Vault-Matter implementó `set-global-owners` y `owner_check` bajo autoridad cifrada (commit `e7134ce4d81193f0f34e3c434c81d77c860baa9e`); ningún número aparece en Base público ni en docs. Requiere enrolar **exactamente dos JID verificados**, en el host, antes de habilitar.
- Base incorpora `VaultClient::require_global_owner`: petición AEAD de exacta identidad WhatsApp al servicio privado, sin permitir que el cliente elija su socket; no concede rangos ni otras acciones.
- `WhatsAppControlBridge` dirige sólo `=/>vault [status|test]` a ese cliente en chats privados: permite al owner global desde el chat propio de cualquier bot Matter enrolado, incluso como remitente externo, siempre con HMAC firmado por el proveedor y autorización positiva del Vault. Dueño del socket ≠ owner global. Grupos, terceros, alias y permisos de roles continúan denegados.
- El proveedor whitelist permite mensajes privados entrantes `=/>vault` sin conceder control de socket. La respuesta negativa a un tercero es silencio; ninguna lista de números, secretos o configuraciones se expone. La ejecución de pruebas de escritura sigue limitada al registro sintético del grant del bot/socket.
- Se sustituyó el antiguo handler local de Vault por la ruta de autorización global. Las pruebas de controlador sin Vault ahora exigen `denied` en lugar de aceptar un status falso. Un sender PN y otro LID no se equiparan automáticamente: el operador debe verificar la correspondencia en WhatsApp y enrolar el identificador que realmente llega, sin asumirla.
- Sin implementación ni integración automática de nuevos bots: cada bot deberá enrolar su propio cliente Vault (sin copiar el secreto de alpha) antes de usar el comando. No hay despliegue en instabox-se de esta revisión ni números enrolados.
- README revisados: 2/2; documentación de continuidad AGENTS, DECISION_LOG, WORK_CONTINUITY y DEVELOPMENT_STATUS sincronizada en este commit. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Corrección real de =/>vault en WhatsApp: allowlist del puente

- Diagnóstico confirmado con usuario: `wa-alpha` está autenticada a WhatsApp, nuevo cliente Vault `base-wa-alpha` pasó `vault_roundtrip_verified revision=1`, y el preflight imprime `ALPHA_VAULT=AUTHENTICATED_CONFIGURATION_ONLY`. Sin embargo, `=/>vault` y `=/>vault test` devolvían rechazo genérico.
- **Causa raíz identificada:** `providers/whatsapp-rust/src/lib.rs` ya autorizaba `=/>vault` en el chat propio, y `src/bin/whatsapp_control.rs` lo registraba; sin embargo, **`src/whatsapp_bridge.rs` omitía `=/>vault` de su allowlist de controles privados**, rechazándolo antes de ejecutar el handler o cualquier IPC. El cambio de timeout en el proveedor no podía solucionarlo.
- Solución: admitir exactamente el verbo `=/>vault` en el allowlist de mensajes privados ya autenticados por HMAC y dueño del socket. No admitirlo en grupos, invitados, bots externos ni prefijos alternativos. `=/>vault` y `=/>vault test` siguen en estado de configuración únicamente; roles, registro y comandos privados bloqueados.
- Regresión: `tests/whatsapp_process.rs` verifica **proceso controlador Rust real** y respuesta `applied` + texto sin configurar en ambos comandos aun con bot OFF; elimina toda variable Vault heredada para no utilizar secretos del runner. `tests/whatsapp_bridge.rs` prueba rechazo a grupo y remitente no dueño.
- **Despliegue pendiente:** reconstruir/descargar desde GitHub Actions el controlador `whatsapp-matter-control` del nuevo SHA, validar hash y `BUILD_INFO`; detener único proveedor, sustituir únicamente el controlador y reanudar `wa-alpha` con `--vault-env`. No re-enrolar, no tocar SQLite ni claves. Confirmar respuesta en chat propio tras CI.
- README revisados: 2/2 (`README.md` y `providers/whatsapp-rust/README.md`); AGENTS/DECISION_LOG/WORK_CONTINUITY/DEVELOPMENT_STATUS actualizados conjuntamente en este commit.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Formato CI del diagnóstico Vault

- El CI del commit anterior rechazó únicamente el formato de una línea de la prueba de aislamiento en `providers/whatsapp-rust/src/main.rs`; se aplicó el diff exacto de rustfmt.
- No se alteran claves, políticas, estado de WhatsApp, autoridad Vault ni comportamiento del comando. Se revalida CI y solo se instala un proveedor si el workflow finaliza success.
- README revisados: 2/2; cuatro documentos de continuidad actualizados en el mismo commit. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Diagnóstico de respuesta =/>vault por WhatsApp

- La conexión real Vault del socket wa-alpha se confirmó por el operador: `ALPHA_VAULT_ENROLLED=YES`, `ALPHA_VAULT_IPC=AUTHENTICATED`, `vault_roundtrip_verified revision=1`. El preflight alpha verificó `ALPHA_VAULT=AUTHENTICATED_CONFIGURATION_ONLY`. WhatsApp reconectó y respondió `WhatsApp session connected. Base-Matter owner control available.`.
- Problema reportado: `=/>vault` no devuelve respuesta visible. El proveedor actual registraba solamente `Base-Matter control denied or unreachable` si el IPC fallaba, sin notificar al propietario. También fijaba 4 segundos para cualquier consulta no-IA.
- Corregido `providers/whatsapp-rust/src/main.rs`: sólo para `=/>vault` originado en el chat privado autenticado del propietario, el IPC dispone de 12 segundos; errores reciben un aviso genérico sin secretos ni roles; el envío aceptado se registra sin el contenido. El acceso a grupos y a terceros sigue denegado. Agregada prueba Rust de aislamiento contra comandos falsos, grupos y remitentes externos.
- Estas modificaciones son código en GitHub; **requieren descargar/verificar el binario nuevo del proveedor desde CI y reemplazarlo sólo con la sesión detenida**. No se ha ejecutado esta sustitución ni demostrado todavía la respuesta por WhatsApp. No tocar credenciales ni sesión SQLite.
- README revisados: 2/2 (`README.md` y `providers/whatsapp-rust/README.md`), cuatro documentos de continuidad actualizados atómicamente. Bot-Matter permanece bloqueado, roles y Github sync independientes.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Etapa Vault: continuidad y revisión documental del cliente wa-alpha

- WhatsApp real: el propietario confirmó que `=/>status`, `=/>menu` y `=/>help` funcionan en la sesión alpha conectada.
- Base-Matter alpha modifica únicamente el lanzador existente `scripts/whatsapp-alpha-existing.sh` con opt-in `--vault-env /etc/base-matter/vault-alpha.env`. Sin opt-in, Vault permanece aislado. Con opt-in: `base-wa-alpha`, llave `alpha-client.key`, `wa-alpha`, ruta IPC y ping autenticado. Nunca mezclar con `base-wa-test-1` o con la base de sesión original.
- Vault-Matter privado incorpora instalador para el nuevo grant y sincronización de tar de ciphertext a GitHub usando cuenta separada, token externo, validación y timer; el repositorio público no recibe código ni token Vault.
- EVIDENCIA LIMITADA: los commits están en GitHub. No consta todavía ejecución del nuevo enrolamiento en instabox-se, `=/>vault` real en alpha ni subida remota de una copia cifrada. La autoridad de roles, registros y ejecución privada continúa bloqueada.
- Pruebas requeridas antes de declarar conexión: CI Linux/Windows y proveedor, enrolamiento real sólo de cliente alpha, self-test y preflight (sin segundo proveedor), reconexión controlada y WhatsApp self-chat `=/>vault`.
- README revisados: 2/2 (`README.md` y `providers/whatsapp-rust/README.md`); cambios y decisiones sincronizados en AGENTS/DECISION_LOG/WORK_CONTINUITY/DEVELOPMENT_STATUS en el mismo commit.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Sesión WhatsApp alpha recuperada y autenticada en host real

- Evidencia enviada desde `instabox-se`: en `/home/Sakura/Base-Matter`, el script `scripts/whatsapp-alpha-existing.sh --check` pasó `Base-Matter control settings checked`, `whatsapp-rust pairing settings checked`, `ALPHA_PREFLIGHT=OK`, `ALPHA_VAULT=ISOLATED_NOT_ENROLLED`, `ALPHA_PAIRING=DISABLED`.
- Luego se ejecutó una única prueba `--resume-only` con los binarios previamente verificados `e073b1b5ebc3ad89cec5a9ce8a71647312aebb64`: `ALPHA_SNAPSHOT=VERIFIED_PRIVATE`, `ALPHA_CONTROLLER=STARTED`, `ALPHA_PROVIDER=RESUME_ONLY`, `PN=true`, `LID=true`, **`WhatsApp session connected. Base-Matter owner control available.`**. El proceso quedó en primer plano. No se solicitó código ni QR.
- Esta señal **acredita autenticación y conexión real de la sesión aislada alpha**, distinta del `wa-test-1` original que recibió `AUTH_REJECTED_LOGGED_OUT`. No acredita aún respuesta del bot a un comando ni integración Vault para `wa-alpha`. Conservar originales, respaldos privados, puertos y archivos separados. No copiar archivos entre sesiones.
- Próxima prueba SIN detener el proveedor en curso: en WhatsApp > chat propio, enviar `=/>status`, luego `=/>menu` o `=/>help`; comprobar respuesta real y evento `Base-Matter owner control applied`. Si no responde, inspeccionar sólo errores genéricos del terminal sin publicar JID, teléfono o tokens. No probar `=/>vault test` en alpha hasta enrolar su cliente IPC independiente y verificar concesiones.
- Vault-Matter en `wa-test-1` continúa con última evidencia `vault_roundtrip_verified revision=2`; **no atribuir** esa concesión al socket `wa-alpha`. Ramas beta/stable intactas y Bot-Matter bloqueado hasta criterios de preparación.
- README revisados: 2/2.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Recuperación segura y separada de dispositivo alpha

- Base alpha y socket original tienen PN/LID de la misma cuenta, pero las claves y JID de dispositivo difieren. La sesión original fue rechazada; alpha todavía no está probada en el servidor.
- Creado `scripts/whatsapp-alpha-existing.sh` exclusivamente para el estado `branches/alpha/whatsapp-test`, con `--check` sin cambios y `--resume-only` que exige bases/configuración existentes, puerto y ruta aislados, crea snapshot SQLite privado, desactiva entorno Vault heredado y jamás solicita códigos ni Cargo ni `--init`.
- CI prueba sintaxis y escenarios con binarios simulados: check seguro, snapshot, rutas equivocadas, socket original y configuración ausente. WhatsApp móvil no se ha reconectado.
- README revisados: 2/2. Actualizada continuidad y decisión. Bot-Matter sigue bloqueado; ramas beta/stable no modificadas.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Comparación de cuenta sin sufijos de dispositivo

- Las bases WhatsApp original `whatsapp-test` y aislada `branches/alpha/whatsapp-test` tienen `device.id=1`, pero PN, LID, claves y account difieren. No tratarlas como respaldos intercambiables. Un JID distinto puede deberse a `:n` y representar otro dispositivo de la MISMA cuenta.
- Comparador offline amplía solo el informe con `SAME_PN_ACCOUNT` y `SAME_LID_ACCOUNT` calculados en SQLite (no salen JIDs/secretos), con `unknown` en formato no reconocido. Cero emparejamientos, cero cambios a sesiones, cero cambios Vault; actualizar ambos README y continuidad.
- Próximo paso: ejecutar comparador tras `git pull`, verificar si ambas identidades pertenecen a una misma cuenta y a partir de allí diseñar recuperación aislada; Bot-Matter bloqueado.
- README revisados: 2/2.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Dos bases WhatsApp distintas: comparar sin mover credenciales

- Inventario operador: `whatsapp-test/session.db` original de 475136 bytes y `branches/alpha/whatsapp-test/session.db` de 630784 bytes. La segunda no es una copia verificada ni evidencia de conexión.
- Herramienta Python de lectura `scripts/whatsapp-stores-compare.py` con pruebas CI compara en SQLite los dos registros `device.id=1` sólo por flags de PN/LID/claves/cuenta. Nunca imprime secretos ni copia archivos.
- No mezclar rutas, sesiones, proveedores ni claves. La anterior rechazó autenticación; el teléfono todavía señalaba dispositivo vinculado. La siguiente acción depende de la comparación. Vault sigue OK; Bot-Matter bloqueado; beta/stable intactos.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Cierre de sesión WhatsApp observado con identidad y TLS válidos

- Evidencia directa: inspección de SQLite `device` con `DEVICE_HAS_PN=yes`, `DEVICE_HAS_LID=yes`, `DEVICE_HAS_ACCOUNT=yes`, `DEVICE_HAS_KEY_MATERIAL=yes`, integridad OK. `DNS_WHATSAPP=OK`, `TLS_WHATSAPP=OK` (`TLSv1.3`). El CI `0259d9f0c722dc01abfa3aa6529cdafde5eb196d` publicó binarios firmados por SHA-256 y operador los verificó.
- Lanzador existente sobre `wa-test-1`: `--resume-only` imprimió `PN=true, LID=true`, disparó **evento `LoggedOut` antes de `on_connected`** y terminó. El servicio Vault independiente permanece verificado con `vault_roundtrip_verified revision=2`.
- Revisado el código upstream `whatsapp-rust 0.7.0`: `LoggedOut` puede ser rechazo `<failure>` durante conexión (401/403/406) o stream error de remoción/conflicto (401/516). Se añade categoría anónima `WA_LOGOUT_CLASS` usando sólo `on_connect` y `ConnectFailureReason` (sin raw node, JIDs, claves, mensajes de servidor o número telefónico); pruebas Rust correspondientes.
- **No inferir** que se deba volver a emparejar sólo por `LoggedOut`; primero verificar en el teléfono la sesión de Dispositivos vinculados y si el estado de la cuenta está restringido. No repetir prueba de conexión hasta actualizar a nuevo binario de CI y validar hashes, sin tocar `session.db*`, llaves, Vault ni ramas beta/stable.
- README y continuidad auditados; Bot-Matter continúa bloqueado. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Señales de sesión local existentes; diagnóstico anónimo de transporte

- Inspector de la base real confirmó `device` singular con PN, LID, cuenta y material de claves presentes, SQLite OK; no implica autenticación WhatsApp activa. Eliminada impresión duplicada del estado offline.
- El proveedor Rust, únicamente en nuevo commit alpha después de CI, informa eventos de desconexión, rechazo de conexión, versión obsoleta, restricción temporal y error del stream sin datos personales ni claves. `--resume-only` también distingue identidad almacenada en memoria de conexión efectiva.
- La siguiente decisión depende de comprobar WhatsApp → Dispositivos vinculados y conectividad del host; no se destruye ni cambia ninguna sesión, ni se generan nuevos códigos. Vault continúa con roundtrip autenticado `revision=2`.
- Actualizados README y continuidad. Bot-Matter bloqueado; beta/stable intactos.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Diagnóstico de sesión WhatsApp (tabla singular) y salida del proveedor

- En `instabox-se`, `--resume-only` con binarios SHA verificados `6fc993b7f335f62dae8c84d91109e929a502a332` terminó inmediatamente sin `on_connected`; no se solicita más código ni se declara WA funcional.
- Inspector SQLite corregido tras revisar implementación upstream `whatsapp-rust 0.7.0`: la identidad vinculada es la tabla `device`, **no inferir autenticación de `devices` plural**. Sólo banderas de presencia de `pn/lid/account` y claves, sin extraer sus valores; prueba Python offline añadida.
- El proveedor informa sin secretos cuándo `--resume-only` termina antes o después de autenticación, y evento de desvinculación cuando es emitido. No se añaden reintentos, QR ni cambios de sesión. Manual `docs/WHATSAPP_PAIRING_DIAGNOSTIC.md`.
- Vault (proceso aislado) sigue verificado con `vault_roundtrip_verified revision=2`; no modificar repos privados/llaves. README revisados y continuidad actualizada. CI de esta revisión pendiente; beta/stable y Bot-Matter bloqueados.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Corrección rustfmt del modo resume-only

- CI del proveedor `4af6078bba1ea2dbd4885bcc4345790b54ecd78b` detectó exclusivamente el `println!` de entrada de `--resume-only` que requería múltiples líneas. Corregido de forma idéntica a rustfmt; no se modifica lógica, sesión, concesiones Vault, permisos o código de vinculación.
- Reiniciar CI Linux/Windows/proveedor para nuevo SHA y distribuir artefactos Linux emparejados sólo tras éxito completo.
- README inspeccionados y continuidad actualizada. Bot-Matter bloqueado. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Recuperación segura WhatsApp y self-test Vault en host

- Instalación en instabox-se ya acreditada con IPC autenticado y ahora `vault_roundtrip_verified revision=2` del cliente real Base; servicio `matter-vault.service` `active`.
- Inspector sin secretos confirmó sesión `session.db` SQLite `OK`, 20 tablas y fila de dispositivo existente, no autenticación vigente de WhatsApp.
- Agregado modo `--resume-only` al proveedor Rust; el lanzador seguro del socket legado `wa-test-1` lo usa por defecto, omitiendo peticiones de código y refresh. Rechazo si condiciones no se cumplen, nunca borrar sesión, jamás generar QR. Prueba Rust para presencia/ausencia de opción de emparejamiento.
- CI nuevo de alpha aún por confirmar; binarios anteriores del proveedor no conocen `--resume-only`. Sólo distribuir los artefactos del SHA que pase Linux/Windows/proveedor. A continuación comprobar `WhatsApp session connected` y mandar `=/>vault` en self-chat. No hay prueba WhatsApp positiva aún.
- Guía: `docs/WHATSAPP_PAIRING_DIAGNOSTIC.md`. README raíz y proveedor inspeccionados; beta/stable y Bot-Matter intactos. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Formato CI de prueba Vault y comando diagnóstico correcto

- `whatsapp-rust-ci` del commit `83668775362311ea3c87a6fcc933b910083556b3` falló únicamente en `cargo fmt --check` por una expresión `assert_eq!` larga en prueba nueva. Aplicado exactamente formato indicado por rustfmt. Los tests de lógica todavía deben pasar en nuevo CI.
- Guía de pairing corregida para ejecutar únicamente el inspector Python de sesión, sin proponer `--help` al lanzador productivo. No se ha relanzado ni regenerado WhatsApp en Debian, y el servicio Vault sigue operativo.
- Ambos README revisados, documentación de continuidad y seguridad preservada. CI nuevo pendiente, beta/stable y Bot-Matter intactos.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — WhatsApp pairing no completado; defecto del whitelist Vault corregido

- El operador ejecutó `scripts/whatsapp-vault-existing.sh` como Sakura con grupo IPC heredado. Ambas comprobaciones `--check` pasaron y el controlador inició, pero el proveedor informó solicitud de pairing fallida. No se logró reconexión WhatsApp ni prueba `=/>vault` en teléfono; NO confundir con fallo IPC Vault ya autenticado.
- Corregida lista de comandos firmables: `=/>vault` ahora sólo en self-chat propietario, nunca grupos/terceros. Prueba de firma/rechazo añadida al proveedor.
- Diagnóstico de rechazo de pairing categorizado sin imprimir secretos; inventario Python de SQLite/archivos de sesión **sólo lectura** y sin filas. No se borran ni regeneran credenciales, no se automatiza re-pairing.
- Archivo operativo `docs/WHATSAPP_PAIRING_DIAGNOSTIC.md`. CI de nuevo commit pendiente, no desplegar proveedor hasta pasar CI y recompilar artefacto en GitHub, no en Debian de 16 GB.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Corrección estricta de final de YAML CI

- El job Linux en el commit anterior llegó a `Verify whitespace` y falló al detectar un salto vacío al final del archivo workflow (verificación `git diff --check HEAD^ HEAD`). Se eliminó el salto excedente, sin cambiar código Rust ni sesiones.
- Se solicita un nuevo CI Linux, Windows y proveedor sobre el mismo SHA de empaquetado; ningún artefacto se declara aprobado antes de `success`.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Ajuste de workflow del empaquetado Base CI

- Se detectó que los pasos de empaquetado del controlador quedaron bajo el job Windows por ubicación YAML. Trasladados al job Linux `rust` **después de las pruebas** y añadida comprobación de sintaxis del lanzador sin Cargo. El proveedor revalida su mismo SHA con cambio mínimo en workflow.
- Sin cambio en runtime, sesión ni instalación Vault del servidor. No se consideran listos los nuevos binarios hasta que CI Linux/Windows y proveedor finalicen correctamente y ambos artefactos compartan SHA.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-20 — Binarios CI y prueba WhatsApp/Vault en servidor sin espacio

- Evidencia del usuario: `instabox-se` recuperó 4.7 GB, descargó y verificó SHA-256 de Vault/cliente del CI `44664373277` y ejecutó instalador root con resultado `CONEXIÓN REAL CONFIRMADA`, para `wa-test-1`. **La autenticación IPC en servidor está confirmada; `self-test` y WhatsApp reales pendientes.**
- Desacople identificado: lanzador Cargo por rama usa ruta `branches/alpha` y socket distinto; se prohíbe mezclar con el estado original `whatsapp-test` asociado a Vault.
- CI de Base alpha emite dos artefactos x86_64 por SHA del commit, después de pruebas: controlador y proveedor Rust sin claves/sesiones; lanzador opcional `scripts/whatsapp-vault-existing.sh` sólo reutiliza el entorno original, exige identidad Vault igual al socket y nunca compila, registra otro socket ni inicia segundo proveedor activo.
- Ver `docs/WHATSAPP_VAULT_PREBUILT.md`. CI de este commit aún por verificar; beta/stable intactos, Bot-Matter bloqueado. Autoridad Vault de roles y backup remoto siguen pendientes.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Conexión Base/Vault verificada en CI y paso al host pendiente

- Base SHA `98231581533bf641e46ce7f5d53a3978b95eb7ab`: Rust Windows/Linux CI `44659925103` verde, whatsapp-rust `44659925089` verde. Vault E2E `44660096430` verde para socket Unix real, cifrado, get/put CAS, revocación y arranque systemd con UIDs distintos; Base no pudo leer master.
- `=/>vault` informa estado; `=/>vault test` prueba get/put/get cifrados de un marcador reservado, propietario self-chat solamente. Necesita entorno y grupo IPC en el proceso real.
- NO está instalado en Debian del dueño; no hay copia segura de ciphertext runtime a GitHub; no habilitar roles, control global ni comandos privados. Guía pública docs/VAULT_CONNECTION_READY.md, referencia operativa en documentación privada Vault.
- Actualizados ambos README, AGENTS, DECISION_LOG, WORK_CONTINUITY y DEVELOPMENT_STATUS sin cambiar Rust; beta/stable intactos, Bot-Matter bloqueado. CI de este commit documental pendiente.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Vault test: prueba de escritura/lectura cifrada desde Base y WhatsApp

- `VaultClient::verify_roundtrip` verifica `get -> put(CAS) -> get` sobre un registro sintético reservado, con revisión comprobada y sin mostrar el payload. No permite roles ni otra categoría de registros; se conserva la concesión tenant/bot/socket vigente y validación del gateway.
- Nuevo operador CLI `matter-vault-client self-test` y comando nativo `=/>vault test` exclusivamente en chat privado del propietario autenticado. `=/>vault` sin argumentos continúa devolviendo estado. No modificar ni probar con secretos reales.
- Requiere servicio y secretos de cliente ya provisionados bajo UIDs diferentes. Pruebas de CI Rust y E2E privado del nuevo SHA pendientes. README revisados: 2/2; AGENTS/decisiones/continuidad actualizados, alpha únicamente, beta/stable intactos y Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Corrección rustfmt del selector del proveedor WhatsApp

- CI del proveedor para SHA 15baba4be23d4bb316eedbc70a3ae2c0d89c5ff3 detectó una sola diferencia rustfmt en tests/ui.rs; se aplicó exactamente, sin modificar ni categorías ni lógica de paginación, seguridad o transporte. El test offline ahora puede ejecutarse tras fmt.
- README revisados: raíz y proveedor. AGENTS, DECISION_LOG, WORK_CONTINUITY y DEVELOPMENT_STATUS actualizados. Nuevo CI pendiente. Solo alpha; beta/stable intactos y Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Selector WhatsApp de 10 categorías (alpha)

- CI anterior del proveedor fallaba en dos pruebas: el catálogo ahora tiene 10 categorías por activar Sistema (=/>vault), pero NativeFlow mantiene un límite de 9 categorías por página. No se cambia el límite ni se eliminan categorías; las pruebas ahora comprueban la primera página con navegación y la segunda con Utils + regreso.
- Seguridad: selector continúa ofreciendo exclusivamente IDs de consulta permitidos. No se habilitan comandos de escritura por botones. Cargo del núcleo Base ya había pasado; nuevo CI provider pendiente.
- README revisados: 2/2; AGENTS, DECISION_LOG, WORK_CONTINUITY y DEVELOPMENT_STATUS actualizados. alpha únicamente; beta/stable intactos; Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Corrección final rustfmt en catálogo y prueba Vault

- Se aplicaron las dos diferencias exactas del CI Linux (array de categorías y última coma en la llamada de prueba de status). No se modificó el contrato ni la autorización del comando =/>vault.
- README revisados: 2/2; estado, decisiones, continuidad y AGENTS actualizados. Este commit requiere nuevo CI Linux y Windows; beta/stable intactos, Bot-Matter bloqueado. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Test del comando Vault y categoría Sistema

- Corregidos rustfmt en la prueba owner-only de Vault y catálogo/menu raíz de 10 categorías, incluyendo Sistema ahora no vacía. El manifiesto templates/menu-visible-categories.json refleja la misma lista real. No se amplían permisos Vault.
- README revisados: 2/2; AGENTS, DECISION_LOG, WORK_CONTINUITY y DEVELOPMENT_STATUS cambiados en el mismo commit. CI nuevo pendiente; beta/stable intactos. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Comando de estado Vault protegido en WhatsApp-Rust

- El controlador =/> integra exclusivamente lectura de salud del canal Vault mediante =/>vault y =/>vault status, en el self-chat verificado y por actor dueño del socket. Opera incluso con bot apagado; NO concede roles, configura secretos ni ejecuta comandos privados.
- La configuración del cliente toma seis variables de entorno con rutas/identificadores, NUNCA la llave maestra ni contenido de la clave. Configuración incompleta bloquea startup. Errores y salida son mínimos, sin identidad ni rutas.
- Enlaces relativos README del proveedor corregidos. README revisados: 2/2. Cambios sólo alpha, con tests y CI por verificar; Bot-Matter sigue bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Ajuste exacto de formato rustfmt en CLI Vault

- El CI Rust de Base detectó un único orden de imports en src/bin/vault_client.rs; se aplicó corrección exacta sin cambios de funcionalidad. Windows ya compila el cliente limitado; CI nuevo por verificar.
- README revisados: 2/2. Decisiones y continuidad actualizadas, sin tocar beta/stable. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Base Vault IPC: corrección de Windows y formato

- Se corrigió orden de imports rustfmt y se separó secure_write por plataforma para no introducir codigo inaccesible ni warnings en Windows. Windows deniega el transporte IPC Unix y la escritura de salida protegida; Linux mantiene la implementación real.
- README revisados: 2/2. CI del SHA nuevo pendiente; actualizados decisiones y continuidad, sin tocar beta/stable ni Bot-Matter. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Corrección de formato y compilación Windows del cliente IPC

- Modulo vault_ipc y CLI ajustados a rustfmt; imports y estructuras Unix protegidas con cfg(unix), manteniendo denegación del transporte no disponible en Windows. No se alteró la identidad, cifrado o la llave maestra.
- README revisados: 2/2. Registros DECISION_LOG, WORK_CONTINUITY, DEVELOPMENT_STATUS y AGENTS actualizados. CI nuevo pendiente; beta/stable intactos.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Cliente IPC cifrado de Vault para Base alpha

- Nuevo modulo Rust vault_ipc y CLI matter-vault-client con canal Unix AEAD, identificador de cliente y secreto de transporte independiente de la llave maestra; ping/get/put restringidos a configuration, bot y socket vinculados al grant Vault.
- Las cargas se leen/escriben en archivos y nunca via argumentos de CLI. Base publico no conoce repos privados ni importa crypto store. Falta integrar autorizacion de actor WhatsApp y despliegue bajo UIDs separados.
- Cambios en codigo y docs/VAULT_IPC_CLIENT.md; README revisados: 2/2. Se actualizaron AGENTS, DECISION_LOG, WORK_CONTINUITY y DEVELOPMENT_STATUS. CI de esta revision pendiente; beta y stable intactas; Bot-Matter bloqueado.
- Creditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Corrección de rustfmt del selector y módulos (alpha)

- Error reproducido en CI Linux: únicamente diferencias de formato en src/bot_setup.rs y orden de pub mod en src/lib.rs. Se aplicaron las diferencias exactas del log; no cambia lógica de elección provider/language/system, permisos, Vault ni bot.
- Windows y el proveedor offline pasaban en el SHA anterior; Linux quedó bloqueado por rustfmt antes de ejecutar otras etapas. CI de nuevo SHA por comprobar; ambos README revisados, beta/stable intactos, Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Frontera de servicio Vault compartida sin claves en consumidores

- Base y Bot tendrán identidades independientes frente a la autoridad Vault. Base público no depende de Bot ni lleva URLs privadas, llaves maestras o source de servicios privados. Ningún bot accede al repo cifrado ni sustituye a Vault como proxy privilegiado.
- El almacenamiento local Rust cifrado del custodio pasó CI offline inicial. El gateway mTLS/IPC, roles remotos, revocación y auditoría cifrada aún no existen; por ello el puente continúa denegando. Documento: docs/VAULT_SERVICE_BOUNDARY.md.
- Revisados ambos README; cambios exclusivamente documentales. Alpha, beta/stable intactos; Bot-Matter bloqueado. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Política obligatoria de cifrado para capacidades protegidas

- Toda información operativa que atraviese una capacidad protegida se clasifica como credencial: cifrado autenticado en tránsito/almacenamiento, claves externas y segregación por bot/socket/recurso. Sin cifrado o autoridad real se **deniega**, nunca se permite fallback en texto claro.
- Base-Matter público expone solo contratos, identificadores y solicitudes; **no conoce rutas ni nombres de repositorios privados**, no descarga implementación interna y no debe poseer claves para descifrar material privilegiado. El host confiable autoriza y transporta ciphertext opaco.
- Regla vinculante: `docs/PROTECTED_CAPABILITY_ENCRYPTION.md`. Contrato existente aún no implementa canal operativo cifrado. HMAC aislado no proporciona confidencialidad. Los datos operativos y auditorías deben seguir cifrados; el descifrado temporal solo ocurre en el proceso autorizado.
- Corrección documental, no modifica runtime/WhatsApp ni demuestra cumplimiento criptográfico. No promover `alpha` hasta pruebas Linux/Windows, seguridad y proveedor. Bot-Matter sigue bloqueado. Ambos README y registros de continuidad revisados.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Selector público neutral para crear bots (alpha)

- Se añadió `src/bot_setup.rs`: contrato de selección `provider/language/system` con defaults virtuales `default/rust/host`. El núcleo NO lleva URLs, rutas ni dependencias hacia implementaciones privadas; proveedor real y permisos los resuelve un catálogo autenticado del host. Sin catálogo se deniega.
- Se añadió `matter-bot-init` como CLI **offline de solo descriptor JSON**; no genera runtime, no concede credenciales y no importa fuentes privadas. Guía: `docs/BOT_SETUP.md`.
- WhatsApp-Matter ya tiene extracción separada del proveedor Rust; no se realizó corte de sesiones ni borrado del proveedor que hoy utiliza el lanzador de Base. El gateway privado no existe aún. Bot-Matter continúa bloqueado.
- Revisar `cargo fmt --all -- --check`, `cargo check --all-targets --locked`, `cargo test --all-targets --locked`, CI Linux/Windows y proveedor tras este commit. README raíz y del proveedor actualizados, sin modificación beta/stable.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Corrección de rustfmt en puente privado (alpha)

- El CI Rust de SHA 5d0897cd8c71a895d5e7dc76777494e2ce7ffceb falló exclusivamente en etapa inicial rustfmt del puente `src/capabilities/mod.rs` y las pruebas `tests/private_capability_bridge.rs`; Windows y proveedor independiente estaban verdes. Se aplican únicamente las diferencias exactas del registro GitHub Actions, sin cambiar funcionalidad, reglas, autorización ni secretos.
- Continuidad: mantener `on|off|inherit`, `set` solo configuración; migración funcional de modos Anti pendiente. WhatsApp-Matter ya contiene copia separada de proveedor Rust y contratos Go, TS, JS; el corte de sesión no está autorizado ni probado.
- Revisados README raíz y proveedor. CI de este SHA nuevo pendiente, beta/stable intactos; Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Corrección de sintaxis Anti: on|off|inherit

- **Decisión actual:** los tres operadores de activación son exclusivamente `on|off|inherit`, tanto por filtro como por modo. El guion `-` queda DESCARTADO como operador; no implementarlo, mostrarlo ni usarlo como alias.
- `inherit` elimina únicamente el override del ámbito autorizado y hereda el valor superior. `set` permanece reservado a configuración; `anti` a consulta y activación. Se conserva `--include` / `--except` para selección dentro de modos.
- Especificación rectoría: [docs/ANTI_COMMAND_CONVENTIONS.md](../../docs/ANTI_COMMAND_CONVENTIONS.md). Corrección documental, **no** parser Rust, ni cambios de permisos, ni pruebas WhatsApp. Migrar runtime, help y menú antes de declarar el comando funcional. Beta/stable intactas. Bot-Matter sigue bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Reglas definitivas propuestas: Set configura; Anti activa y hereda

- **Diseño pendiente, sin cambio Rust:** `set` se reserva para configuración. `anti [regla] on|off|-` controla activación o retiro de override para heredar el valor superior; `set anti link on` queda descartado de la gramática objetivo.
- `anti --<mode> on|off|-` opera sobre un modo completo, opcionalmente limitado por `--include` y `--except` a filtros registrados (no archivos del servidor). Modos previstos: `--security` como subconjunto Anti y `--multimedia`, sin confundir Matter Security con filtros Anti.
- `--mod` conserva ámbito del socket; `--global` es predeterminado general y no puede editarse en modo vivo sin Vault. `-` nunca borra la política global. Actuaciones masivas requieren autorización y transacción/auditoría; en fallo se deniegan.
- Documento rector de continuidad: [docs/ANTI_COMMAND_CONVENTIONS.md](../../docs/ANTI_COMMAND_CONVENTIONS.md). Hay que actualizar router, ayuda, menú y pruebas en tarea posterior. No declarar implementado ni probado; `alpha` documentación solamente, sin modificar `beta`/`stable` y Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Puente público para capacidades protegidas (alpha, no desplegado)

- Se creó la API Rust de invocación anti.link en src/capabilities/anti_link.rs; sólo arma peticiones, no contiene implementación privada, claves ni permisos. Ausencia de gateway = rechazo. Los antis y =/>anti link existentes no cambian.
- Integradores podrán asociar su comando #antilink al puente mediante un gateway autenticado cuando exista, sin recibir código de ejecución ni acceso a repositorios privados. El ejecutor protegido deberá comprobar decisiones actuales y verificadas por alcance; los datos públicos de contexto no autentican a nadie.
- Scope: grupo, socket (--mod) y valores generales de lectura (--global). No se permiten mutaciones generales. Especificación: docs/PRIVATE_CAPABILITY_BRIDGE.md.
- README raíz y del proveedor revisados; pruebas unitarias de contrato añadidas. CI y pruebas reales pendientes; gateway privado y autoridad remota todavía NO desplegados. Alpha solamente, beta/stable intactos; Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Corrección del menú estático por ámbito

- Base previa c8554033bb0594886443a90c14dd0811576021c8. La plantilla `templates/matterssmith-menu.txt` mostraba el socket como `status --global`, heredado de una convención retirada. Se sustituyó por `status --mod`, se añadió consulta de predeterminado general `status --global` y se renombró el encendido por defecto del socket como `bot --mod on/off`.
- El cambio es de presentación; no modifica estado, privilegios ni políticas. `tests/control_style.rs` comprueba el nuevo encabezado; el contrato `help` mantiene los ámbitos equivalentes. Estado sin argumentos de los comandos implementados continúa siendo consulta, no guía.
- README revisados: 2/2. CI de este SHA pendiente; no desplegar en WhatsApp hasta completar Linux, Windows y proveedor. Beta/stable no se modifican. Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Ajuste de Rustfmt en pruebas públicas Anti

- Base previa 01822a6d32e43839c809ba0580b2b9564d931087. Se aplicaron las dos diferencias exactas de Rustfmt en `tests/whatsapp_public_readonly.rs` para las llamadas de propietario y comprobaciones de rechazo; sin cambios de lógica, autoría ni autorización.
- El controlador mantiene el estado sin argumentos para comandos realmente implementados, con ámbitos `--mod` de socket y `--global` de predeterminado general en los módulos que los soportan. Consultas públicas Anti no ejecutan acciones.
- README revisados: 2/2. CI del SHA nuevo pendiente y validación WhatsApp real pendiente. Beta/stable intactos. Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Verificación de consultas públicas Anti y estado sin argumentos

- SHA anterior f3c308ab3683693b80b05cdae1d569bcc04a2112. Las suites Linux/Windows habían superado el transporte y la mayor parte de los comandos, pero la nueva prueba de estado sin argumentos esperaba un encabezado anterior. Se ajustó la aserción a «Antis del grupo» conservando una respuesta real y distinguible del chat privado.
- Se incorporó una prueba de WhatsApp que habilita un grupo firmado y verifica que un usuario normal puede consultar `anti`, `anti link`, `anti --mod link` y `anti --global link` sin concesión de permisos, y que `anti link on` / `set anti link on` son denegados.
- Persistencia de políticas: todos OFF de fábrica, `--mod` socket, `--global` solo general sin Vault configurado. Ninguna consulta crea escrituras o acciones de moderación.
- README revisados: 2/2; CI del nuevo SHA pendiente, beta/stable sin cambios, Bot-Matter bloqueado. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Migración de pruebas HMAC/IPC al ámbito del socket

- Base previa baa59b01be636593b6742f78e66ddfcb53804db6. Se ajustaron las pruebas de `tests/control_ipc.rs` y `tests/whatsapp_bridge.rs` que enviaban el alias heredado `bot --global on` y consultaban `status --global` como si pertenecieran al socket. Las nuevas comprobaciones usan `bot --mod on` y `status --mod`, sin alterar el transporte HMAC, el antirreplay ni las firmas.
- La opción `--global` queda reservada al valor predeterminado general; en modo vivo no concede escritura global y requiere backend autenticado para guardar cambios. `--mod` es el socket propio, nunca otro.
- README revisados: 2/2. La integración de un resolvedor seguro de usuarios por respuesta/mención sigue pendiente. CI del nuevo SHA pendiente; beta/stable intactos, Bot-Matter bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Corrección compatibilidad de estado Anti y proveedor

- Base previa: 83e0a17b47c80c595a43b0d0168d0df74cdf6675. CI Linux y Windows encontraron una sola aserción heredada en `tests/anti.rs`: la etiqueta del grupo dejó de decir «Antis del grupo». Se restaura esa etiqueta; «chat privado» distingue DM, y «socket actual» sustituye al antiguo término ambiguo «socket global».
- Se aplicó la diferencia exacta de Rustfmt del proveedor `whatsapp-rust/src/lib.rs`, sin cambiar autorización ni contenidos.
- Consultas sin argumentos devuelven estado cuando existe: bot, anti, status, set y ajustes implementados. No crear respuestas ficticias de módulos pendientes o de usuarios sin metadatos verificados.
- README revisados: 2/2. CI del nuevo SHA pendiente; no integrar en beta/stable. Bot-Matter sigue bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — CI de consultas sin argumentos, formato y controles globales

- SHA anterior: 5b229870eea827864f48961fefb2278beca8bf33. Se aplicaron 21 ajustes exigidos por Rustfmt (despachador, consultas de estado y pruebas); `bot --mod on` ahora usa el tipo Rust correcto del token `action`.
- En modo WhatsApp unificado quedan bloqueadas **ambas** formas de escritura global: `bot --global on` y `bot on --global`; `bot --mod on` es la escritura local del socket autenticado. La prueba cubre el bypass de orden de argumentos.
- Se mantiene el comportamiento: comandos con estado, sin argumentos, muestran su valor actual; `help` explica la sintaxis sin ejecutar comandos. Anti permite consulta read-only en chat activo y registra solo configuración, no moderación.
- README revisados: 2/2. CI del nuevo SHA aún pendiente; no promover a beta/stable ni confirmar WhatsApp real. Bot-Matter sigue bloqueado.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Consultas de estado sin argumentos y ayuda por ámbito

- La consulta de estado es el comportamiento predeterminado de las familias **con estado**: `=/>bot`, `=/>anti`, `=/>status`, `=/>set`, `=/>set style`, `=/>set lang`, `=/>set anti` y `=/>set anti link`. En un chat privado verificado, Anti consulta la configuración de ese chat. En un chat de grupo, la de ese grupo. Comandos meramente informativos o pendientes sin estado real mantienen sintaxis de ayuda o indisponibilidad.
- `--mod` identifica **este socket**; `--global`, **el valor predeterminado general**. No mezclar ambos: el socket no es global. Para Anti, `--global` muestra el fallback inmutable OFF/warn y no un registro Vault. Para status/bot en modo vivo, `--global` solo informa la disponibilidad del fallback general y jamás el socket. `bot --global on/off` se rechaza en WhatsApp vivo; `bot --mod on/off` configura el socket.
- `=/>set` y sus variantes sin valor consultan preferencias reales de menú; `=/>set --global` reporta los valores de fábrica, no una preferencia remota configurable. Los comandos `set anti` con argumentos son escrituras verificadas; sin argumentos solo consultan. `set anti link` sin operación muestra estado y lista de dominios actual.
- En chats habilitados, usuarios normales pueden leer estado Anti mediante un canal dedicado **sin escritura**, para `anti`, `anti <tipo>`, `anti --mod`, `anti --global`; el proveedor solo firma consultas limitadas, nunca `anti link on/off`. Los detalles privados del socket y los rangos globales no se divulgan.
- Los nombres/respuestas/menciones/números para consultar usuarios siguen **pendientes de resolvedor autenticado**; este cambio no simula estados de terceros.
- Los cambios de catálogo describen `--mod` cuando la familia lo soporta y eliminan la documentación errónea «--global = socket». No anunciar soporte universal de ámbitos en comandos pendientes.
- Pruebas añadidas/ajustadas: estado sin argumentos, ámbitos, no escalada, consultas públicas, contrato de help y compatibilidad. CI Linux/Windows y proveedor siguen siendo requisito antes de activar en servidor.
- README revisados: 2/2; sin promocionar a beta/stable, Bot-Matter bloqueado. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Control de ámbito estandarizado y seguridad de consulta por usuario

- Base anterior: `c584b3d55569`. Se aplicaron ajustes de Rustfmt a `src/control_scope.rs` y orden de `src/lib.rs` sin cambiar la autorización.
- El contrato `docs/CONTROL_SCOPE_CONTRACT.md` define: chat actual, `--mod` socket actual, `--global` predeterminado general; dos banderas son inválidas. Un ámbito no otorga permisos.
- El resolver de usuario por respuesta, mención, número o `user` es un pendiente de integración con metadatos verificados, no una función pública activa. No conceder por texto un rol ni divulgar identidad privada.
- Continuidad actualizada y READMEs revisados: 2/2. Verificación CI de este SHA pendiente. `beta`/`stable` intactos. Bot-Matter bloqueado.

## 2026-11-19 — Ámbitos y estado de usuario (contrato de migración)

`--mod` identifica el socket autenticado; `--global` los valores predeterminados generales (no un alias del socket). Consulta sin bandera = chat actual. Ver `docs/CONTROL_SCOPE_CONTRACT.md`. Cualquier usuario podrá consultar solo campos públicos de otro participante por respuesta, mención, número o `user <id>` cuando exista un resolvedor de identidad verificado; esa ruta **aún no está conectada**. Se introdujo el parser común y la protección Anti lo usa; otras familias requieren migración explícita, pruebas y permisos por comando. Las escrituras globales no están implementadas y siguen bloqueadas.

## 2026-11-19 — Verificación de formato del test Anti

Se aplicó el ajuste exacto solicitado por Rustfmt en dos aserciones de herencia de `tests/anti.rs`. Semántica del comando sin cambios: `anti` solo lectura, `set anti` escritura autorizada, `--mod` socket, `--global` solo valores base generales no sincronizados. CI del nuevo SHA pendiente; no actualizar el servidor hasta pasar los verificadores.

## 2026-11-19 — Comandos Anti separados por consulta, edición y ámbito

- `=/>anti`: inventario de filtros del grupo actual, sin modificar valores.
- `=/>anti <tipo>`: estado efectivo de un filtro del grupo; `=/>anti audit` inspecciona contadores pasivos de ese grupo.
- `=/>anti --mod` y `=/>anti --mod <tipo>`: valores predeterminados del **socket actual**, incluidos sus overrides.
- `=/>anti --global` y `=/>anti --global <tipo>`: solo consulta del valor base general **OFF/warn**, no una lectura de configuración centralizada; todavía no existe una política global compartida de Vault. No muestra los cambios particulares de otros sockets.
- `=/>set anti <tipo> on|off|inherit`: habilitación individual por grupo; `=/>set anti --mod <tipo> on|off`: predeterminado del socket. Acciones: `set anti <tipo> action warn|delete|kick` (solo guardan preferencia, no moderan).
- Dominios: `=/>set anti link add|del|list` selecciona el chat firmado, y `=/>set anti --mod link add|del|list` el socket. Añadir un dominio no habilita el filtro.
- No se aceptan `=/>antis`, `=/>antilink on` ni `=/>anti link on` como operaciones. No se permite `set anti --global` hasta disponer de autoridad global autenticada y almacenamiento transaccional central.
- Estado de seguridad: escritura restringida al propietario autenticado del socket, solo grupos activados observados, sin ejecutar mensajes editados, sin borrados o expulsiones automáticas. Bot-Matter sigue bloqueado.
- Objetivo: revisar CI Linux/Windows/proveedor sobre el SHA final y repetir la prueba de menú por pairing-code. Esta anotación no es una certificación de WhatsApp real.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Formato de resolver de comandos en ayuda

- Base alpha anterior 97472dbdca9a5a8a7f6f535fa91d73bccb9abbe4. La verificación Rustfmt solicitó reducir el filtro de nombre exacto o prefijo en `src/menu.rs` a una línea.
- Corrección puramente de estilo; ya está escrita la prueba de la ayuda por cita y de los subcomandos, y el transporte Whatsapp sigue sin ejecutar mensajes citados.
- README 2/2 revisados. CI del nuevo commit pendiente.

## 2026-11-19 — Resolver nombres de subcomandos en help

- SHA previo 42c95c8452528d70dcce19ae82b0dad187545610. El test de ayuda citado reveló que `help set role` intentaba interpretar `role` como un número de página de categoría.
- Solo una segunda palabra numérica selecciona página. Las consultas `help anti audit`, `help set role` y las citas con subcomandos ahora buscan el descriptor específico más largo; no ejecutan nada.
- Test ampliado. READMEs 2/2 revisados; CI de nuevo SHA pendiente.

## 2026-11-19 — Corrección final de formato y Clippy en ayuda citada

- Base alpha anterior 61d7af72039aa9b810a29396e263bc553ed71162. Se aplicaron diferencias exactas de Rustfmt en `src/menu.rs` y `clippy::unnecessary_lazy_evaluations` en el proveedor WhatsApp Rust (respuesta citada).
- `=/>help`, `=/>help anti`, `=/>help set anti link` y el uso respondiendo a un comando conservan seguridad y requisitos de habilitación.
- Archivos READMEs revisados 2/2; CI del nuevo SHA pendiente, aún sin verificar teléfono físico.

## 2026-11-19 — Prioridad de comandos sobre categoría de igual nombre

- SHA anterior: c41fdeb6f2fdefbaf1329376dc0f6418cc9c9a3e. Prueba real de Linux identificó un conflicto: `help anti` tomaba la categoría Anti y no la ficha de comando; también la antigua aserción `Ayuda: antilink` ya no corresponde a la nueva ficha `Ayuda completa`.
- Exactitud: el nombre exacto de comando tiene prioridad en `help`; para categorías siguen `help antis`, `help antis 1`, etc. El menú mantiene categorías sin cambios. Un comando citado se resuelve a metadata, no a ejecución.
- READMEs 2/2 actualizados y pruebas del cambio pendientes. El acceso sin registro y Anti OFF se mantienen.

## 2026-11-19 — Compatibilidad estricta del test de citas

- Base alpha anterior: 425bb2df3ac7a4541ce5a06f6eb4867e32259918. Rust provider 0.7 requiere el trait `MessageBuilderExt` para `Message::text`; en prueba `no_quote` se usa directamente el constructor explícito de `wa::Message`, sin dependencia adicional.
- READMEs 2/2 revisados. Comportamiento de consulta de ayuda citado sin cambios, y sin conceder ejecución por texto citado.
- Pruebas Linux/Windows/proveedor pendientes para el SHA de esta corrección.

## 2026-11-19 — Finalización de Rustfmt en prueba de ayuda por cita

- SHA anterior: 79b3335c521eaab335ad8f8167ae91aef34c847d. El comprobador solicitó solo el orden de imports en `tests/menu_catalog.rs`; se aplicó sin alterar lógica.
- `help <comando>` y `help` respondiendo al mensaje consultan metadatos, nunca ejecutan el comando citado. Anti sigue con ediciones/camuflaje en modo pasivo.
- README revisados 2/2; CI nuevo pendiente. No actualizar Debian hasta Linux, Windows y provider verdes.

## 2026-11-19 — Corrección CI de ayuda en mensaje citado

- SHA anterior 6e0a2956a035e7fb1f05d4f3217cb322b7aac7e7. Se corrigió la importación local de `matter_base::menu` en la prueba de ayuda por cita, sin cambiar el contrato.
- Se aplicó Rustfmt del proveedor: módulo help_reply, match del firmante read-only y orden de imports en main.rs.
- `help` citado devuelve **solo metadatos** y no ejecuta órdenes; se mantiene control por chat, tipo de remitente, HMAC, antirreplay y filtrado de manipulación.
- READMEs 2/2; esperar CI Linux/Windows/proveedor del SHA nuevo antes de probar en WhatsApp.

## 2026-11-19 — Formato y cobertura de ayuda contextual

- Base `alpha` anterior: 80eb18be86e5d35c87101164508e618906575176. Rustfmt exacto aplicado en `src/menu.rs` y `tests/menu_catalog.rs` según CI.
- `help` en respuesta a un mensaje usa citas de WhatsApp Rust sin ejecutarlas; quitada lógica duplicada en el proveedor y dependencia de test aislada bajo `#[cfg(test)]`.
- El acceso público a ayuda específica acepta `help set anti link`, `help anti audit`, `help set role`; las respuestas son solo metadatos. Reglas de bot OFF y Vault sin cambios.
- READMEs 2/2 revisados; esperar CI antes de actualizar servidor.

## 2026-11-19 — Ayuda completa y respuesta al mensaje citado

- Base alpha anterior: 9aa97bdda1113181785768da2d8ae7869f8dec52. `=/>help` solo ofrece el uso y explica la opción de responder a un comando; no muestra categorías.
- `=/>help <comando>` ahora expone categoría, descripción, estado real de catálogo, todas las formas de uso y restricciones documentadas; `help set anti link`, `help anti`, `help register` y `help set role` destacan su estado operativo.
- Proveedor WhatsApp Rust 0.7: al RESPONDER a un mensaje que contenga un comando con `=/>help`, extrae `extended_text_message.context_info.quoted_message` del mensaje recibido, rechaza otra conversación, limita longitud y envía solo `=/>help --quoted <base64url>` firmado por HMAC. El núcleo busca el nombre más específico en el catálogo, nunca ejecuta texto citado.
- Casos de prueba: quote de `set anti link`, `anti audit`, bot, registro, rol, cita cross-chat, cita ausente, contenido inválido y lectura directa. No hay delegación de permisos ni activación de comandos desde citas.
- READMEs revisados 2/2; CI SHA nuevo pendiente. Guardrails Anti conservados y Bot-Matter sigue bloqueado.

## 2026-11-19 — Ajuste Rustfmt de política Anti por grupos activos

- SHA previo d0afffdff09a5f73965f7f640b31dd7141cdf6f8. Apliqué formato de CI para `bot_admission(...).decision.enabled` de `inspect_anti_observation`; no cambia lógica.
- Monitoreo pasivo solo en grupos activados por propietario, edición cifrada sin clave permanece no inspeccionada, comandos camuflados nunca se ejecutan.
- README revisados 2/2. Pendiente CI del nuevo SHA y confirmación real en WhatsApp.

## 2026-11-19 — Privacidad y semántica de ediciones Anti

- Base previo ffb8a7a344d010f870e195571e8d0d14285e6478. La pasarela de inspección Anti registra metadatos de mensajes de grupos **solo si el propietario habilitó ese grupo** según política actual del socket. Grupos apagados se omiten; la pasarela no introduce una excepción de activación.
- Una edición clásica sin `edited_message.text_content()` ya no se marca como descifrada/legible: se considera **opaca sin inspeccionar**, igual que un sobre E2EE sin clave original.
- Sin dar permisos, ejecutar comandos normalizados ni realizar moderación. `anti audit` conserva solo conteos para grupo activo.
- Ambos README revisados; CI del SHA siguiente pendiente.

## 2026-11-19 — Corrección CI de auditoría y documentaciones del proveedor

- Base previo fdbad8252aa080c0988be136236a6a5620bc547e. Se aplicaron las diferencias de Rustfmt de la función `anti audit` y tests.
- Solucionado `clippy::empty_line_after_doc_comments` en `providers/whatsapp-rust/src/lib.rs`: documentos del firmante y del nuevo evento Anti quedan correctamente vinculados a sus métodos.
- `=/>anti audit` sólo muestra conteos, sin cuerpos/identificadores. El puente no ejecuta mensajes editados ni normalizados y todos los antis continúan apagados inicialmente.
- READMEs 2/2 revisados; CI del SHA siguiente pendiente. No habilitar el despliegue hasta tres suites verdes.

## 2026-11-19 — Auditoría de mensajes editados y camuflados

- Base alpha previo: 1aaf76e48930959f45f72bede16e4e33228d6ae4. `=/>anti audit` permite consultar contadores de ediciones legibles, cifradas sin inspeccionar, comandos disimulados y enlaces disimulados del grupo. `=/>anti --global audit` en privado del dueño agrega el socket.
- Se audita simultáneamente edición y camuflaje, sin conservar cuerpo, remitente ni identificador de mensaje. En un mismo evento pueden aumentar varios contadores; son indicadores, no personas culpables.
- Filtros OFF y acciones automáticas desconectadas, no hay borrado ni expulsión. Comandos alterados no se ejecutan.
- Clippy del test con carácter invisible: se usa escape explícito `\\u{200B}`.
- READMEs 2/2 revisados, CI del nuevo commit pendiente. Precaución: ediciones cifradas del cliente sin secreto original se clasifican como no inspeccionadas.

## 2026-11-19 — Corrección CI menú Anti y proveedor

- El SHA anterior `876d0b7420c8214937cdeb76916ab51ac3378622` ya pasó el formato del núcleo, pero Windows indicó una expectativa antigua: `set anti link` está en **página 4**, no en página 1 de Anti; test ajustado con página real. Las páginas 1–4 siguen cubiertas, sin redes sociales.
- El proveedor requirió un ajuste exacto de Rustfmt para la prueba de firma de Anti; no cambia HMAC ni código operativo.
- Menú sigue sin comandos sociales; lista chat/socket y observación Anti firmada con ediciones y camuflaje en estado pasivo. No hay moderación automática.
- README revisados 2/2; CI nuevo pendiente.

## 2026-11-19 — CI Rustfmt para puente de inspección WhatsApp

- SHA previo: 8f8d8981980aef7655651c15d9d7ea79ca978d35. Se aplicaron exactamente las diferencias de rustfmt detectadas en `src/control.rs`, `tests/anti.rs` y `tests/whatsapp_public_readonly.rs`.
- Transporte HMAC de observación Anti (solo grupo; sin acciones), estados original/editado legible/editado cifrado ilegible, antirreplay y normalización sin ejecutar texto.
- READMEs revisados 2/2; CI del nuevo SHA pendiente, todavía no autorizar actualización del servidor.

## 2026-11-19 — Observación Anti real desde WhatsApp, sin conceder permisos

- Base alpha inicial: `c4428b4e4fb9e781aea8a98a85d776d67edf2cbd`. El proveedor Rust ahora observa mensajes de grupos, incluidos textos originales, ediciones clásicas `ProtocolMessage.MESSAGE_EDIT` y sobres cifrados `MESSAGE_EDIT` identificables. Las ediciones cifradas sin clave del mensaje padre se registran como **ilegibles** y nunca se despachan como comandos.
- Un SignedFrame HMAC ya existente, con prefijo interno no disponible como orden pública, enlaza mensaje/grupo/remitente reales a un canal **exclusivamente de inspección**. El puente verifica HMAC, JID, timestamp, antirreplay y ámbito de grupo; no llama al dispatcher administrativo ni manda respuestas.
- El inspector `AntiStore::inspect_message` detecta dominios y camuflaje con metadatos de edición; solo guarda tipo/ámbito en SQLite privado, **no cuerpos ni JIDs de remitentes**. Todos los antis permanecen OFF y moderación aún no está conectada.
- Pruebas nuevas cubren firma del proveedor, edición recibida de otro participante, duplicados, firma alterada y rechazo de monitorización en privado. Se mantienen pruebas de desactivación por defecto y coincidencias de dominio.
- Edición cifrada E2EE moderna requiere recuperar `messageContextInfo.messageSecret` del mensaje padre; no se descifra automáticamente en whatsapp-rust 0.7. Una edición detectada pero ilegible debe tratarse como no inspeccionada.
- README revisados 2/2. CI de esta revisión pendiente; prueba real WhatsApp posterior, Bot-Matter bloqueado.

## 2026-11-19 — Contrato central de inspección editada y camuflaje

- Base SHA previo: 6941fa23706dc41a9005e9c37f820a7e692d18d2. Nuevo módulo `src/anti_guard.rs` con `EditState`: original, editado legible y editado cifrado NO legible. Normalización de análisis detecta separadores Unicode, caracteres invisibles, control bidi, cambios de puntuación, rutas malformadas y prefijos confusos.
- **Regla fundamental:** no despachar nunca un comando normalizado; únicamente inspeccionar y auditar. Un texto alterado o cifrado no representa permiso para ejecutarlo ni para moderar.
- `AntiStore::inspect_message` acepta el estado de edición verificado por el proveedor y aplica el detector `antilink` usando el contenido inspeccionable. Registra únicamente tipo de evento y ámbito en SQLite: ni cuerpo, ni identificador de persona, ni mensaje original.
- Los filtros sin detector específico aún NO reciben adaptadores multimedia (antiimage, antiabuse, etc.); compartir metadatos de edición será la fase de integración del proveedor. Ediciones cifradas modernas requieren clave de secreto del mensaje original en whatsapp-rust 0.7, por lo que se registran como NO inspeccionadas.
- Corrige fallo CI menú: la primera página de Anti contiene `antilink`, no la segunda tras retirar reglas de redes.
- README revisados 2/2. Código nuevo CI pendiente. Sin activar moderación, roles ni Bot-Matter.

## 2026-11-19 — Primer cierre de CI AntiLink

- SHA previo: b5b2615e6989a1fc1c29227bd921c8bab2c4bb07.
- CI detectó diferencias de formato Rust en `src/anti.rs`, `src/control.rs`, `tests/anti.rs` y `tests/menu_catalog.rs`. Se aplicaron las diferencias exactas, sin cambios de lógica.
- Los cambios funcionales anteriores: retiro de Anti por redes sociales, lista de dominios chat/socket, OFF predeterminado, SQLite privado y auditado.
- Próxima fase inmediata: análisis de mensajes editados, camuflaje y detección de protocolo. Historial y pruebas del proveedor; no ejecutar comandos alterados ni prometer lectura de ediciones cifradas sin su clave.
- README revisados 2/2; GitHub Actions nuevo SHA pendientes.

## 2026-11-19 — AntiLink por lista de dominios, sin filtros de redes sociales

- Base alpha anterior: `8b0fc5516e6d8ab7e313469bfbd05bd1103d7f30`.
- Se eliminaron del registro y del menú `antilinkwa`, `antilinktelegram`, `antilinkdiscord`, `antilinkinstagram`, `antilinkfacebook`, `antilinktiktok`, `antilinkyoutube`, `antilinkx`, `antilinkshort` y `antilinkphishing`. Las tablas antiguas conservan sus filas sin ejecutarlas ni eliminarlas; no se reactivan tras reiniciar.
- Queda solo el detector genérico `antilink`. Su lista permite dominios como `t.me`, `instagram.com` o `youtube.com` bajo un **único** filtro. Compara host exacto y subdominios (no substrings); lista vacía => no detecciones.
- Nuevo comando `=/>set anti link add|del|list <dominios>` para el **chat actual** firmado, en grupos o privado, y `=/>set anti link --global add|del|list ...` para todo el socket; alias `=/>set antilink`. El global hereda hacia cada chat. Cambios persistentes SQLite privados, auditados y autorizados al propietario; hasta 12 entradas por comando y 64 por ámbito. No altera encendido. Un global `del` no borra override local.
- En grupos `=/>anti link on|off|inherit` sigue separando configuración de habilitación. Todos los antis OFF inicialmente; aun si está ON el detector es solo lectura y NO ejecuta moderación, expulsión ni borrado automático.
- Pruebas: persistencia, herencia, aislamiento, URL con dominio exacto, rechazo de sintaxis, no ejecución de nombres retirados y menú sin entradas de redes; CI de este SHA pendiente.
- README revisados 2/2. Bot-Matter sigue bloqueado. Siguiente: verificar CI y pedir prueba WhatsApp de `set anti link` en el chat propio y un grupo autorizado.


## 2026-11-19 — Corrección de formato en acceso invitado

- Base alpha anterior: 3c1fa6f325df63718eb67eefb9ce1a66ff4424e4.
- Proveedor WhatsApp verificó PASS para el primer cambio. CI núcleo detuvo por rustfmt: orden alfabético de módulos y dos bloques de tests con espaciado incorrecto. Diferencias exactas aplicadas.
- Política intacta: lectura para invitados sin registro cuando el chat esté habilitado; `=/>register` informativo; Vault no recibe datos y no se asignan roles.
- Vault-Matter `matter` actualizó contrato de invitados en `850071e1bda6f62d68eea802666e45474d379825`.
- README revisados: 2/2. Nuevo CI pendiente; no afirmar teléfono probado.

## 2026-11-19 — Modo invitado estilo demo, registro voluntario

- HEAD remoto alpha antes del cambio: ea7c0da6c579f24406093940e7677fe6a349dfbd. El usuario confirmó que se debe poder usar el bot sin registro, como una versión de prueba: **no imponer CURP, biometría, género, edad ni inscripción Vault para comandos básicos**.
- Nuevo contrato Rust `src/guest.rs`: status e información de registro disponibles sin bot activo; menú, ayuda, diagnósticos de lectura, créditos y ayuda de bot disponibles solo donde el socket/chat fue habilitado; deniega escrituras, perfil Vault, verificación civil y ejecución de código a invitados.
- `=/>register` publica estado de prueba sin crear cuenta; `=/>register start` indica que el gateway de Vault aún no está listo y NO almacena datos. Ruta pública firmada en el proveedor y HMAC/antirreplay normales, incluyendo estado OFF. Descriptor en menú Principal.
- No se lanzó Bot-Matter: el contrato es reutilizable cuando se desbloquee. Ninguna concesión automática ni limitación arbitraria por tiempo. Los bot comandos `#` aún dependen de su runtime preparado y autorizado.
- Dos pruebas nuevas: acceso invitado/read-only con bot desactivado y matriz de capacidades. README revisados: 2/2. CI del nuevo SHA pendiente. Siguiente: confirmar UI de WhatsApp y contratos Vault del invitado; continuar corrección Anti pendiente.

## 2026-11-19 — Clippy Rust del receptor ListResponse

- CI del SHA 031ebb79ad1edc7c81d68a8997ba9c11918681b5: proveedor compiló y todos sus tests pasaron; Clippy estricta falló en tests/ui.rs por un `..Default::default()` innecesario en `SingleSelectReply`. Se quitó sin modificar protocolo ni permisos.
- README revisados 2/2. CI del nuevo SHA pendiente. Siguiente: lista de dominios personalizable en vez de reglas anti por red social, después prueba real con =/>test ui.

## 2026-11-19 — Respuestas de listas WhatsApp y verificación auténtica del botón

- Base alpha: 37698e3967af5b2857c252473d5ad792a289a3e1. Una selección del menú produce burbuja, pero no dispara respuesta como el texto escrito manualmente.
- Añadida lectura de ListResponseMessage.single_select_reply.selected_row_id y TemplateButtonReplyMessage.selected_id a NativeFlow/ButtonsResponse existentes; IDs bajo lista blanca estricta. Firma HMAC, identidad y antirrepetición se mantienen.
- El ACK manual es una prueba de comando, no de botón. Solo el evento interactivo decodificado y autorizado muestra «✅ Botón verificado»; un log sin teléfonos confirma el evento.
- Test de respuestas ListResponse y de rechazo de IDs no autorizados. README revisados 2/2; CI y prueba en móvil pendientes.

## 2026-11-19 — Corrección Rustfmt + tests TS18/TS19 nativos

- Base alpha: 351d4d4a416cf8aa6b15fc33a6607bf1150763b7; Linux y Windows del cambio de menú sin fallo conocido al iniciar esta corrección, proveedor falló por `cargo fmt --check` antes de compilar.
- Aplicadas las cuatro diferencias exactas de formato que reportó CI en `providers/whatsapp-rust/src/main.rs` y `src/ui.rs`.
- Prueba nueva `native_ts18_ts19_buttons_use_quick_reply_and_readonly_ack_ids`: verifica 2/3 quick replies, los IDs cerrados Audio/MP4/Enlace, el texto mostrado y el enrutamiento de ACK. No activa permisos ni descargas.
- Las filas del selector permanecen como títulos exactos `=/>menu <categoría>` sin `Abrir menú`. En grupos y chats sin activación se mantienen las restricciones existentes.
- CI del siguiente SHA pendiente. HTML RichResponse sigue sin representación móvil confirmada. README revisados: 2/2.

## 2026-11-19 — Burbuja de selección y tarjetas compatibles

- Reporte visual Android: al pulsar «Categorías», el cliente publicaba «☾︎ Anti / Abrir menú» en vez del comando elegido. El selector ahora usa `title: "=/>menu anti"` y elimina `description`. El ID opaco y su enrutamiento HMAC siguen intactos; la burbuja depende de cómo WhatsApp represente esa fila.
- `MATTER_WA_MENU_LAB=1` añade tarjetas NativeFlow de prueba TS18/TS19 en menús raíz de **chats privados habilitados**. Sus botones únicamente emiten ACK de lectura y pasan por las comprobaciones existentes. Si el envío nativo falla, se intenta la tarjeta legacy.
- El chat privado del propietario compara adicionalmente el transporte legacy y sigue siendo el único lugar que puede ensayar Rich HTML y audio. No se envía HTML, multimedia ni comparaciones de legacy a lectores externos o grupos.
- RichResponse HTML no equivale a HTML web y el cliente puede descartarlo sin dar error: estado **experimental sin visualización confirmada**. El HTML offline continúa disponible.
- No se modifican roles, antis, activación, secretos ni derechos de ejecución. Bot-Matter permanece bloqueado. CI Linux/Windows/proveedor y prueba de teléfono para este SHA pendientes. README revisados: 2/2.

## 2026-11-19 — Instrucciones compactas y citas WhatsApp (propuesta de Leito)

- Base GitHub alpha: `8a1176ff7cdfe1d86c5c5d689c4ec2e9cd1620c0`. Nueva sugerencia de Dev MS Leito Delfin 3 Siss respaldada por capturas del menú en Android.
- `=/>menu` en estilo `classic`: sustituir seis explicaciones extensas con marcos `│` por cuatro líneas breves con prefijo literal `> ` de cita WhatsApp. Conservar encabezado, cierre, categorías visibles, botón único y estado autenticado. `status` con bot OFF, identidad para admin, externos OFF y aislamiento/auditoría siguen expresados.
- `style none`: texto igualmente compacto pero sin tipografía ni cita decorativa.
- `TextFormat::Quote` permite reutilizar citas de una o varias líneas sin introducir HTML o alterar controles. Registrar `whatsapp.quote` en `templates/message-alphabets.json`; sumar pruebas de cuatro líneas compactas y renderizado reutilizable.
- Cambios sólo de presentación: ninguna operación, permiso, categoría, sesión, Vault ni laboratorio TS18/TS19 se altera. Seguimiento en `AGENTS.md`, `docs/DECISION_LOG.md`, `docs/WORK_CONTINUITY.md`, `docs/DEVELOPMENT_STATUS.md`.
- README revisados: 2/2. CI Linux, Windows y proveedor sobre el nuevo SHA: pendiente. Captura visual del formato cita en WhatsApp: pendiente.

## 2026-11-19 — Corrección de plantilla CRLF en Windows

- Base alpha: 96bd61fc8a61ca745f91e4b89a62d4bf3611832f.
- CI Windows del SHA anterior: el test de tipografía falló porque la plantilla de texto conservó CRLF en Windows, y el cierre terminaba en retorno de carro.
- Solución: normalizar `\r\n` a `\n` antes de renderizar mensajes arbitrarios. No afecta texto, caracteres o permisos.
- Linux había completado sus verificaciones; proveedor en validación del SHA previo. Repetir los tres trabajos para el nuevo SHA.
- README revisados: 2/2; WhatsApp real pendiente.

## 2026-11-19 — Rustfmt final de selector de categorías

- Base alpha: 605ae3bf3da68de49276f1ef9d68d6e1b77f8161; Linux Rust completó correctamente sus pruebas de formato, compilación, tests, Clippy y documentación.
- El proveedor Rust falló sólo en formato de la expresión que valida el manifiesto. Se aplicó el diff exacto del CI.
- Windows y proveedor del nuevo SHA requieren nueva confirmación. Sin cambios en permisos, sesiones, Vault o plantilla visual.
- README revisados: 2/2. Continuidad: validar CI para este SHA y probar =/>menu en WhatsApp real.

## 2026-11-19 — Rustfmt y tipado del cambio de menú

- Base alpha previa: 83cc0f993e24b1ea70659c6e11d5bf8277015302.
- GitHub Actions de la revisión inicial: formato FAIL en proveedor y Rust; se aplicaron exactamente las diferencias de rustfmt de ambos trabajos.
- Se hace explícita la comparación `&str` frente a `String` del manifiesto de categorías activas, y el índice de los alfabetos JSON.
- No cambian categorías, alcance, comandos, botones, permisos ni datos de Vault.
- Siguiente: comprobar CI Linux/Windows/proveedor y después pantalla real en WhatsApp.
- README revisados: 2/2. CI del siguiente SHA: pendiente.

## 2026-11-19 — Cambio visual solicitado: tipografías y categorías activas

- Base remota alpha: 8639b9e4beab5217f344766d74c1b85578b3daba.
- Idea de Leito: negritas para títulos, `código inline` para comandos, soporte de bloques ```código```; separar información y ornamento.
- Ocultar todas las categorías sin comandos implementados en menú principal, agregación, submenús y botón de WhatsApp. Catálogo contractual de 21 entradas preservado.
- Archivos funcionales: src/menu.rs, providers/whatsapp-rust/src/ui.rs, src/message_typography.rs, src/lib.rs, templates/menu-visible-categories.json, templates/message-alphabets.json y templates/message-frame.txt.
- Pruebas de catálogo, interfaz y puente ajustadas; menús heredados por compatibilidad conservan su autenticación HMAC y permisos.
- Continuidad: revisar CI de este commit (Linux/Windows/proveedor). Si falla, corregir en alpha y actualizar estos cuatro registros. Después realizar prueba de pantalla en WhatsApp de =/>menu y selector Categorías, junto a TS18/TS19, sonido y Rich HTML opcional. Beta/stable sin cambios.
- README revisados: 2/2. CI y pantalla real pendientes.

## 2026-11-18 — Se corrigió la posición de la prueba de Grupo
El commit anterior sustituyó por error el caso POSITIVO "menu group"
y dejó Grupo en la lista NEGATIVA. Ahora la lista positiva contiene
Grupo (una de las 21 entradas) y la negativa prueba "menu unknown".
No se modificó el signer ni se ampliaron permisos. README
revisados: 2/2. Revalidación del nuevo SHA pendiente.

## 2026-11-18 — Corrección de test y checkpoint tras CI

CI del SHA 97de8ac mostró:
- La suite del proveedor marcó fallida una aserción heredada que
  rechazaba `=/>menu group`, aunque «Grupo» es una de las 21
  categorías autorizadas. El caso negativo ahora es
  `=/>menu unknown`; todas las categorías siguen probándose.
- La verificación documental detectó que el commit previo omitió
  modificar docs/WORK_CONTINUITY.md. Este commit lo corrige
  y refuerza la regla de registrar el checkpoint SIEMPRE.
- Los cambios son tests/documentación, no permisos de escritura.
- README revisados: 2/2. CI nuevo pendiente.

## 2026-11-18 — Test real de 21 categorías y documentos de WhatsApp

La plantilla antigua de pruebas sólo renderiza "menu" raíz y
rechaza submenús. Se movió la prueba de las 21 categorías y páginas
a un fixture dinámico con MenuPreferences en memoria, sin alterar
la política de producción. Corrección Rustfmt del provider lib.rs.
Se actualizaron las guías del menú TS18/TS19, RichResponse y sonido.
README revisados: 2/2. CI y prueba real del teléfono pendientes.

## 2026-11-18 — Corrección Rustfmt laboratorio y listas canónicas

El CI detectó formato no estándar en los módulos del proveedor, en
el catálogo de 21 rutas y en sus pruebas. Se aplicó exactamente el
estilo solicitado a imports, match, tarjetas y arrays de test.
No se modificó la autorización de escritura ni se habilitó IA/Vault.
README revisados: 2/2. CI del nuevo SHA por confirmar.

## 2026-11-18 — Selector: 21 categorías y páginas admitidas (pruebas)
El proveedor de entrada de lectores acepta exclusivamente las 21
categorías autorizadas y páginas 1–3, además de aliases antiguos,
siempre como lectura y sujetos a identidad PN/LID, HMAC, antirreplay
y activación. Se añadieron pruebas para cada opción y para botones
TS18/TS19 Audio/MP4/Enlace y HTML con audio no automático.
La prueba de `=/>menu group` ahora debe pasar (es una categoría
autorizada); comandos no registrados siguen rechazados.
README revisados: 2/2. CI de este SHA pendiente.

### 2026-11-18 — Prueba opcional HTML, sonido y botones en menú
En alpha, el primer =/>menu sigue mostrando 21 categorías y el
selector Categorías. MATTER_WA_MENU_LAB=1 añade sólo en self-chat
autenticado tarjetas TS18 y TS19, tono original en audio
cifrado Ogg/Opus y HTML RichResponse cuando
MATTER_WA_RICH_HTML=1. El audio requiere pulsar reproducir;
HTML no ejecuta JavaScript y puede no aparecer en WhatsApp.
README revisados: 2/2. CI y teléfono pendientes.

### 2026-11-18 — Continuidad obligatoria y permiso Aura Reed comunicado

El usuario exige registrar cada idea y cambio en instrucciones, un
historial de decisiones y un checkpoint exacto para reiniciar tras
desconexiones. [AGENTS.md](../../AGENTS.md) es la norma vinculante;
[continuidad](../../docs/WORK_CONTINUITY.md) y
[decisiones](../../docs/DECISION_LOG.md) se actualizan en cada commit.
El workflow Rust lo exige. La documentación de IA
[docs/MATTER_AI.md](../../docs/MATTER_AI.md) remite al permiso comunicado
por Jeriel, registrado en Vault-Matter (sin límites específicos
comunicados, pero sin clave o broker activo). Se corrigen una
aserción de menu/help y una colisión temporal Windows.
**README revisados: 2/2**. CI de este nuevo SHA pendiente.

### 2026-11-18 — Rustfmt y ayuda sin categorías
Se aplicó formato Rustfmt en la ayuda por página, la disponibilidad de comandos, las variantes del menú y las pruebas. `=/>help` sin argumentos muestra exclusivamente formas de ayuda, sin copiar el menú. Ambos README revisados 2/2; se requiere nueva verificación.

### 2026-11-18 — Menú de atributos separado de ayuda contextual
Las páginas de `=/>menu` sólo enumeran variantes reales, sin descripciones ni pendientes; `=/>menu all` es un índice compacto. `=/>help` sin argumentos indica la sintaxis de ayuda y no copia categorías. `=/>help <categoria|comando>` mantiene los detalles. Invocaciones sin una operación de cero argumentos asignada, como `=/>set` y `=/>bot`, muestran uso. Ambos estilos (`classic` y `none`) siguen disponibles. READMEs revisados 2/2. Validar Linux, Windows y proveedor antes de usar en WhatsApp.

### 2026-11-18 — Compilación Windows: nombre local del rótulo de categoría
El compilador de Windows detectó que la variable local label ocultaba la función label del renderizador. Se renombró el local a decorated_label sin variar presentación ni permisos. Ambos README revisados; se requieren verificaciones completas del nuevo commit.

### 2026-11-18 — Formato final del menú sin símbolos
El CI detectó una única diferencia de Rustfmt en el nombre de Todo del estilo none; se aplicó literalmente sin cambiar comportamiento. Se mantienen 21 rutas, sección de autoridad no otorgante y botón de categorías. README principal y proveedor revisados 2/2; revalidación por SHA.

### 2026-11-18 — Rustfmt de menú autenticado y texto sin diseño
CI detectó formato en control_ipc.rs, control_style.rs, src/menu.rs y tests/menu_catalog.rs. Se aplicó el formato exacto. `style none` ahora enumera nombres simples de las 21 entradas, sin glifos de la lista clásica. El formato visual clásico, la autoridad por socket y los controles de seguridad no cambiaron. READMEs revisados 2/2; verificar nuevo SHA.

### 2026-11-18 — Corrección de saltos y delimitadores del menú completo
Se corrigió la serialización UTF-8 para que el marco use saltos de línea reales, no secuencias escapadas, y se agregó la cenefa completa al detector de respuestas ya decoradas. El test de límite NativeFlow se ajustó a 3900 bytes; nueva prueba valida formato clásico, identidad verificada, variante sin diseño, ausencia de rótulos vacíos y separación de la sección de autoridad para lectores externos. Revisados ambos README; CI del SHA anterior detectará/confirmará diferencias adicionales.

### 2026-11-18 — Formato completo de menú global y variante sin diseño
Se adoptó la cenefa, encabezado, 21 filas inmutables, sección descriptiva de autoridad, campos dinámicos de identidad verificada, comandos, rango, estado y socket, instrucciones y cierre. La ruta de IPC sólo rellena datos después de HMAC/antirreplay/admisión, no deriva Owner Global de un número. Se suprimió «Sin comandos registrados» y el menú all omite encabezados sin comandos. `style none` sigue mostrando categorías y datos. El selector conserva un único botón; se aumentó su límite de texto a 3800 bytes sin alterar límites de transporte ni permisos. Revisados ambos README y AGENTS; la comprobación CI y teléfono real siguen siendo necesarias.

### 2026-11-18 — Formato de prueba de catálogo cerrado
CI del SHA 653b6ec marcó únicamente formato de la prueba Rust del catálogo; se aplicaron exactamente los saltos de línea solicitados por Rustfmt. La aserción de IDs compara explícitamente slices en orden. README principal y proveedor revisados (2/2); se vuelve a validar. No hay cambios de transporte, sesión, Vault ni permisos.

### 2026-11-18 — Invariante de 21 entradas y comandos únicos
Se añadió una prueba que compara el orden exacto de las 21 filas con el contrato cerrado, exige 20 destinos de comandos distintos de Todo y evita duplicados. El comando general help se ubica en Principal, no IA; matter permanece en IA. Los módulos internos siguen conservando rutas históricas sin exponerse como categorías. READMEs revisados 2/2; validar CI del nuevo SHA.

### 2026-11-18 — Catálogo de 21 entradas aplicado a la presentación Rust
Se reorganizaron los descriptores existentes dentro de IA, Anti, Configuración, Información, Principal, Modo, Propietario, Sistema y Utilidades. Otras entradas autorizadas no contienen comandos inventados. Todo sigue siendo una vista agregada. Aliases anteriores permanecen sólo como compatibilidad. Un único selector «Categorías», paginado en tres pantallas por límites del cliente, mantiene 21 entradas accesibles. Rustfmt corrigió las diferencias que reportó CI del SHA 457de96; se vuelve a validar Linux, Windows y el proveedor. Ver docs/CATEGORY_CONTRACT.md.

### 2026-11-18 — Catálogo de categorías cerrado por instrucción del propietario
El contrato [docs/CATEGORY_CONTRACT.md](../../docs/CATEGORY_CONTRACT.md) registra las 21 entradas exactas del menú original y prohíbe categorías inventadas. Todo = vista agregada. Establece destino de cada comando histórico, aliases de compatibilidad y un único botón «Categorías». La documentación **no equivale a migración completa del código**; pendiente adecuar el renderizador y las pruebas.

### 2026-11-18 — Diagnóstico sin argumentos incluso con bot OFF
La ruta pública de =/>test ahora devuelve un informe mínimo cuando
el bot está apagado, en lugar de descartarlo sin respuesta.
Las pruebas con botones siguen inactivas hasta que el bot esté
habilitado; no ejecutan código. El proveedor añade logs redaccionados
para distinguir falta de firma, rechazo y bot desactivado, sin
imprimir números, JID ni cuerpos de mensajes. Se incorporaron tests
de bot OFF. Los dos README se revisaron (2/2). CI requerido.

### 2026-11-18 — Rustfmt del menú de categorías y regresión de test
El CI del SHA bf6fe38 encontró dos diferencias de Rustfmt:
el iterador de categorías y un salto de línea en tests/whatsapp_bridge.rs.
Se aplicó exactamente el formato requerido sin alterar comportamiento.
READMEs raíz y proveedor revisados (2/2), nuevo SHA necesita CI.

### 2026-11-18 — Restauración del menú visual de categorías
Se adopta exclusivamente el DISEÑO aportado por el propietario:
«╭─💠 𝗖𝗮𝘁𝗲𝗴𝗼𝗿í𝗮𝘀», renglones «│·♪·╠[»,
último «│·♪·╚[» y cierre «╰───────────⟨ 🌀».
Presenta nueve categorías registradas y la opción «Todo»
para =/>menu all. No se inventaron comandos de Economía,
Gacha, Juegos u otras secciones del ejemplo. NativeFlow ofrece
UN SOLO botón «Categorías» (single_select), sin botones
separados Ayuda/Ver todo. Todo está dentro del selector.
El ensayo =/>test ui y el clásico =/>test row conservan
sus botones de diagnóstico independientes.
Vault, secretos, configuración de antis y ramas beta/stable intactos.
Dos README revisados antes del commit (2/2).

### 2026-11-18 — Corrección: =/>test silencioso desde WhatsApp

El proveedor y puente sólo aceptaban =/>test desde el propio chat
privado, por lo que los mensajes en grupos y de otras cuentas no
alcanzaban el handler. Ahora las pruebas visuales de solo lectura
(=/>test, ui, row y ack cerrados) funcionan:
- propietario autenticado PN/LID: propio chat y grupos;
- otros miembros: únicamente chats habilitados, con identidad
  verificada por la misma ruta pública que =/>menu.
HTML sigue restringido al propio privado del propietario y ninguna
prueba concede roles ni ejecuta archivos.
Envío NativeFlow/ButtonsRow permitido tras un ACK firmado para esas
pruebas; el emisor recibe fallback de texto si el transporte falla.
Pruebas de regresión cubren owner grupo/privado, replay, público off/on,
y rechazo de HTML externo. No verificar renderizado físico desde CI.
README (2/2) revisados; beta/stable no se modificaron.


### 2026-11-18 — Formato confirmado para tarjeta de prueba ButtonsRow

El CI del SHA 62399fa se detuvo antes de compilar por Rustfmt:
tres diferencias en tests/ui.rs y una en el match de src/control.rs.
Se aplicó la salida literal de Rustfmt, sin modificar el comportamiento
del ensayo clásico ni el protocolo. Dos README revisados (2/2).
Revalidar compilación/test/Clippy sobre el nuevo SHA.

### 2026-11-18 — Comparación WhatsApp: NativeFlow y ButtonsRow clásico

Se agregó =/>test row, una tarjeta de texto basada en la familia
protobuf ButtonsMessage con tres botones (Audio, MP4, Enlace),
equivalentes al patrón Go ts19 recibido. Las respuestas de
ButtonsResponseMessage se traducen a acuses de solo lectura bajo
el puente autenticado, sin descargas ni ejecución de código.
No se agrega imagen de cabecera: necesita un upload cifrado real,
que será una etapa posterior, ni se da por probado el renderizado
en el cliente. README 2/2 revisados, CI del SHA requerido.

### 2026-11-18 — CI de pruebas visuales: Rustfmt y Clippy

En el SHA 1291d43, la compilación y los tests por defecto de Linux
pasaron; el proveedor se detuvo en un formato de importación y
un salto de línea adicional en tests/ui.rs. Clippy detectó una
comparación booleana innecesaria en tests/menu_catalog.rs.
Se aplicó el formato exacto de Rustfmt y se sustituyó la aserción
por la comprobación real del uso =/>antilink on|off|inherit.
READMEs raíz y proveedor revisados (2/2), sin cambios de runtime.

### 2026-11-18 — Pruebas de menú y botones ejecutables desde WhatsApp (alpha)

El comando privado del propietario =/>test ui construye una prueba NativeFlow
con dos botones y dos filas; al seleccionarlos, el puente de control
normal verifica identidad de la sesión, HMAC y antirreplay, y emite un
acuse de solo lectura. =/>test html usa únicamente el transporte
experimental ya existente y requiere MATTER_WA_RICH_HTML=1.
Se documentan instrucciones de verificación visual manual en
docs/WHATSAPP_UI_TESTS.md.

Se corrigió también el filtro de ayuda pública para permitir nombres
de comando seguros como =/>help antilink y se admite help antis 2
en el renderizador. No se autoriza ninguna mutación pública.
El campo "test" del menú corresponde solo a pruebas visuales;
test <código> y ejecución remota permanecen pendientes.
Dos README revisados (2/2). No se alteran sesiones, Vault ni ramas beta/stable.

### 2026-11-18 — Clippy final del prototipo de sticker

CI confirmó compilación y pruebas del proveedor; Clippy -D warnings
encontró únicamente una importación no usada en el test
sticker_badge.rs. Se eliminó. El transporte productivo, roles,
llaves y bases no cambiaron. Dos README revisados (2/2).
Revalidar CI para este SHA antes de elevar a beta.

### 2026-11-18 — Formato requerido para prototipo de insignias

El CI del prototipo se detuvo únicamente en cuatro diferencias de
Rustfmt: orden de módulos, agrupación de dominios, imports del test
y array de comandos legado. Se aplicaron los cambios exactos, sin
alterar autorización, protocolo ni flujo de ejecución.
READMEs raíz y proveedor revisados 2/2; nuevo SHA requiere CI.

### 2026-11-18 — Prototipo Rust offline: sticker asociado al mensaje

Se añadió providers/whatsapp-rust/src/sticker_badge.rs y sus pruebas
offline. La función construye un mensaje protobuf con
MessageAssociation/STICKER_ANNOTATION (associationType 11), valida
la clave real de destino y metadatos cifrados del sticker y no envía
mensajes. Está aislada del receptor productivo y no autoriza rangos.
Vault-Matter contiene únicamente el contrato documental/JSON de las
insignias, sin gateway de roles en producción. Boom/boom2 NO se habilitan.
CI del nuevo SHA obligatoria, seguida de compatibilidad en teléfono.
README raíz y proveedor revisados antes del commit (2/2).

### 2026-11-18 — Proveedor: expectativas corregidas para help privado

El último CI del proveedor compiló, pero la prueba provider.rs todavía
prohibía =/>help y =/>h desde el chat propio, aunque la nueva ruta
firmada y autorizada de ayuda ya los permite. Se trasladan esos
comandos a los casos positivos, incluidas help all y help antis.
Las órdenes on/off retiradas siguen prohibidas. No se cambió
la función de autorización ni se amplió a terceros; es actualización
de prueba acorde al contrato de ayuda previamente incorporado.
READMEs revisados 2/2, nuevas pruebas CI por SHA necesarias.

### 2026-11-18 — Corrección de tipado en selector público

El CI del SHA 8c4c79d superó el formato del proveedor pero reveló
dos comparaciones inválidas de referencias dobles en src/lib.rs:
&&str frente a str. Se aplicó la corrección de tipos *category,
sin ampliar la lista de categorías ni los permisos.
README raíz y proveedor revisados (2/2); revalidar el nuevo SHA.

### 2026-11-18 — CI: corrección de formato UI y colisiones Windows

El SHA fb90999e falló en formato del proveedor UI, espacios en blanco
del archivo de tests y una prueba de SQLite en Windows porque el nombre
temporal usaba reloj/PID y podía colisionar tras reiniciar el test.
Se aplicó el formato exacto indicado por Rustfmt a los dos archivos
de NativeFlow; el directorio de prueba de replay ahora usa 128 bits
aleatorios, reintento de colisión y limpieza por RAII.
El menú/código de ejecución, Vault, roles y sesiones permanecen igual.
Se revisaron los dos README (2/2). CI debe confirmar este nuevo SHA;
no inferir compatibilidad del teléfono por pasar pruebas offline.

### Botón selector + Ayuda + Ver todo, y lectura de respuestas Go-style

Se reemplazaron los tres quick replies fijos por un `single_select` con nueve filas para categorías y dos botones de consulta. El parser reconoce tanto `NativeFlowResponseMessage` como `ButtonsResponseMessage` cuando los IDs pertenecen a la lista cerrada. Sin comandos administrativos, envíos espontáneos ni descarga de medios. El cliente real Android/iOS no ha sido validado. README revisado.

### Formato del puente para help

Se aplicó el bloque Rustfmt solicitado por CI a las ramas de ayuda pública. Sin cambios de comportamiento, permisos, claves ni sesión. README revisado.

### Corrección HMAC para navegación

La allowlist del proveedor incorpora `help/h/ayuda` en contextos verificados, y permite `menu all` públicamente si el chat está activo. Mantiene antirreplay y restricciones de propietario. README revisado.

### Actualización UX en alpha

El puente reconoce `help` además del menú, siempre con HMAC y permisos previos. Los botones actuales aún no replican el ejemplo Go. Compatibilidad HTML, audio y pantalla completa pendiente de prueba real; no se han alterado las sesiones.

### 2026-11-18 — Ejemplo original de tarjeta con botones de fila (sin integración)

Se conserva en el repositorio el código Go suministrado por el propietario
para `ts18`/`ts19`, incluida la llamada `SendButtonsRow` que presenta
miniatura, texto y botones inferiores Audio/MP4/Enlace. Es una referencia
visual; **no** se añadió un motor Go, descarga multimedia ni endpoints externos.
Los botones de menú en Rust actuales son un prototipo distinto y la prueba
del teléfono no demostró aún la interacción deseada.
Ver [`docs/WHATSAPP_BUTTON_CARD_REFERENCE.md`](../../docs/WHATSAPP_BUTTON_CARD_REFERENCE.md).

### CI final del menú por categorías

El commit `dcf39b2` pasó las verificaciones del
núcleo en Linux. El CI del proveedor señaló una
única diferencia Rustfmt para el match de
categorías en `src/lib.rs`; corregida exactamente.
La lista sigue cerrada a nueve categorías
registradas; no permite comandos administrativos
a terceros ni páginas arbitrarias. Sin cambios
de sesión o credenciales. Este README fue revisado
antes de publicar la corrección, pendiente de
nueva ejecución del CI y prueba en teléfono.

### CI de submenús públicos (alpha 2026-11-18)

La prueba offline de `578c680` detectó que el proveedor
firmaba submenús genéricos como `menu group`, antes
explícitamente prohibidos. Se sustituyó el comodín por
una lista exacta de categorías: status, bot, set, antis,
roles, system, sandbox, ai y navigation. Solo se
aceptan páginas 1..8; no se autoriza ninguna
mutación ni categoría inventada. Los casos positivos
de menú incluyen set, antis, antis 2 y system.
La lectura pública depende de la activación del chat;
las operaciones de propietario usan otra firma.
README del proveedor actualizado, sin credenciales.

### Verificación de código modular / independiente

La compilación de `2943cedd` llegó a Clippy
en el núcleo y detectó únicamente la regla
`module_inception` de nombres internos; se
mantiene un archivo por comando sin suprimir
advertencias. El proveedor standalone marcó
dos diferencias Rustfmt en
`provider_control_style.rs`; ambas corregidas.
Se conserva el decorador de respaldo y la
separación del menú (no reimportar
`crate::menu` a la biblioteca autónoma).
README actualizado; nueva compilación pendiente.

### Decoración independiente del proveedor (CI 2026-11-18)

La importación anterior de `../../../src/control_style.rs`
se reemplaza por el módulo aislado
`src/provider_control_style.rs` para los mensajes de
REGISTER/recuperación y fallback. Evita importar el
registro de comandos y SQLite del núcleo. El menú
dinámico recibido por IPC ya viene formateado; no
se envuelve otra vez. Se aplicaron los diffs
Rustfmt del verificador. Probar los mensajes
NativeFlow reales en Android/iOS tras confirmar
CI, conservando solo tres botones de lectura.
No se copió ningún secreto ni se modificaron sesiones.

## Menú modular y botones nativos, 2026-11-18 (alpha)

El controlador usa un catálogo Rust por carpeta
y por comando. Permite `menu`, `menu set`,
`menu antis`, `menu antis 2`, `menu system`,
`menu ai` y demás categorías. Estilos
`classic`/`none`, idiomas `es`/`en`,
preferencias persistidas fuera de la sesión.

`MATTER_WA_NATIVE_BUTTONS=1` intenta enviar
en un solo mensaje Estado, Menú y Antis.
Su respuesta se convierte exclusivamente en
`=/>status`, `=/>menu` o `=/>menu antis`,
y se verifica por HMAC, origen, antirreplay
y política de chat. Las llamadas a submenús
de otros usuarios siguen siendo solo lectura.
El pie usa marcas nativas generadas desde
HTML breve validado; no HTML/CSS arbitrario.
`MATTER_WA_RICH_HTML=1` es experimental,
self-chat y no demuestra visualización Android/iOS.

No se importan claves, no se ejecuta sandbox,
no se activan servicios ni se conceden rangos.
Ver `../../docs/MENU_CATALOG.md`.
README del proveedor revisado.

## 2026-11-18 — Matter AI: solo broker Vault y permiso del titular

El controlador `matter` sigue limitado al propietario autenticado
en su chat privado PN/LID, con HMAC y antirreplay. No se reutilizan
claves de IA obtenidas de Aura ni otros repositorios; Base-Matter
no acepta claves API directas. Solo un gateway Unix con ruta y UID
esperado configurados puede responder, tras verificar permisos y aplicar
la política del titular en cada solicitud. El cliente rechaza respuestas
sin `policy_applied=true`, `grant_id` y revisión positiva.
El gateway productivo **aún no existe**; no anunciar capacidad
de IA en vivo como verificada. Ver `../../docs/MATTER_AI.md`.

## Matter AI por WhatsApp: comando privado de solo análisis (alpha)

El propietario puede usar `matter status`, `matter ask` y
`matter plan` a través del puente firmado existente. El
proveedor solo acepta `matter` en su lista privada
**cuando el mensaje procede de la cuenta autenticada**;
no existe ruta pública ni de grupos. El controlador
aplica además permisos de propietario y aislamiento de
la ruta de extensiones. La consulta IA no es ejecución
de shell ni modificación de GitHub.

El controlador consume claves de entorno autorizadas
(Claude preferente, Venice después, OpenAI alternativo),
no claves de Aura Reed ni archivos de sesiones.
El plazo de entrega IPC se amplía solo para consultas
`matter`. Otras operaciones conservan 4 segundos.
No hay garantía de respuesta externa hasta probar
cuentas válidas. Ver `../../docs/MATTER_AI.md`.

## Normalización Rustfmt del transporte RichResponse (alpha)

La comprobación inicial del commit `d8c3857` se detuvo
en el formato de la bandera `MATTER_WA_RICH_HTML` y
tres aserciones del test `rich_transport`. Ajustado
conforme al diff de CI. Sin modificación funcional,
sin habilitar envío por defecto. Revalidar el nuevo SHA.

## Ensayo RichResponse opt-in únicamente en el chat propio (alpha)

Se conserva `rich_preview.rs` y sus comandos offline
`--rich-preview-html` / `--rich-preview-json`. El módulo
independiente `rich_transport.rs` empaqueta ese JSON como
`AIRichResponseMessage` nativo, sin alterar los datos ni
afirmar ser Meta AI. Sólo se intenta entregar tras recibir
`=/>menu` del propietario autenticado en su self-chat y con
`MATTER_WA_RICH_HTML=1`. En grupos o mensajes de terceros
esta opción no se usa, aun si está activada. Default apagado.

El proveedor hace fallback a NativeFlow o al menú original si
`send_message` rechaza la operación; no detecta si el cliente
ignora un envío aceptado. Es un **ensayo**, no HTML WhatsApp
garantizado. El modo de prueba no ejecuta scripts de `xd.zip`,
no atribuye origen Meta AI, no permite botones administrativos
ni abre una segunda sesión. La vista previa local conserva sus
botones deshabilitados; los únicos controles con interacción
real continúan siendo los tres NativeFlow de solo lectura.
Detalles: `../../docs/WHATSAPP_RICH_HTML_COMPAT.md`.

## Ajuste de formato del prototipo Rich (alpha)

El verificador de `2dfc214` rechazó el formato de dos índices
JSON y una aserción en `tests/rich_preview.rs`. Corregido conforme
al log de Rustfmt, sin alterar lógica ni habilitar envío Rich.
Nueva ejecución de verificación obligatoria.

## Compatibilidad experimental del material Rich HTML (2026-11-18)

Analizados ejemplos externos en Go/JavaScript/HTML: el contenedor
`embedded_screens`/`FOAIDNixelButtonSheets` puede estudiarse, pero
el envío proviene de un `botForwardedMessage` que aparenta tener
identidad `0@bot`. Ese método **no** se trasplanta a Base-Matter ni
se afirma que WhatsApp acepte HTML de terceros.

Ahora `rich_preview.rs` genera HTML estático y un JSON de compatibilidad
sin abrir el proveedor, con el menú original y sin JavaScript:
```bash
cargo run --manifest-path providers/whatsapp-rust/Cargo.toml -- --rich-preview-html > matter-preview.html
cargo run --manifest-path providers/whatsapp-rust/Cargo.toml -- --rich-preview-json > matter-preview.json
```
El proceso debe iniciarse desde la raíz del repositorio. Las salidas
son solamente **archivos locales de prueba**, no mensajes WhatsApp.
Los botones del HTML se muestran deshabilitados deliberadamente; los
únicos botones nativos reales siguen siendo NativeFlow opt-in. No se
usa Node, Baileys, spoof de Meta AI ni una segunda sesión.
Véase [análisis de compatibilidad](../../docs/WHATSAPP_RICH_HTML_COMPAT.md).

## Ajuste de orden de imports NativeFlow (alpha)

El paso rustfmt en `3178e32` exigió ordenar el alias
`NativeResponse` y `wa`. Corregido siguiendo el diff de CI.
Sin modificaciones de lógica y sujeto a CI completo en el nuevo SHA.

## Ajuste de Clippy para el experimento NativeFlow (alpha)

En la revisión `d6dd6f1`, las pruebas nativas y el convertidor de
HTML breve terminaron en verde. Clippy detectó exclusivamente el
`..Default::default()` redundante del cuerpo protobuf y un patrón
de coincidencia irrefutable. Corregidos sin modificar botones,
lógica de firma HMAC o interfaz de sesiones. Nuevas pruebas CI
pendientes antes de ensayar Android/iOS.

## Corrección de formato NativeFlow (alpha, 2026-11-18)

El primer workflow del experimento `d532ef2` indicó solo diferencias
de Rustfmt en los archivos nuevos y el `main.rs` modificado; aún no
alcanzaba las pruebas. Se aplicaron las diferencias del log y se evitó
un patrón potencialmente irrefutable en el lector de respuestas.
Nuevo CI es obligatorio para verificar firma, botones y parser.

## Experimento NativeFlow y HTML de estilo (alpha, 2026-11-18)

Para una prueba controlada de botones, ejecutar en la terminal del
operador `export MATTER_WA_NATIVE_BUTTONS=1` y luego
`cargo run --locked` desde la raíz. Por defecto `=/>menu`
continúa enviándose en texto, con la plantilla original.
Con la opción activada, el proveedor intenta enviar un único
`InteractiveMessage.NativeFlowMessage` con el cuerpo exacto del menú
más tres `quick_reply`: Estado, Menú y Créditos. Los botones tienen
IDs específicos solo de lectura; una selección se procesa por el mismo
puente firmado HMAC, autorización de chat y protección antirreplay.
Un error de envío provoca fallback al texto. No se garantiza que
Android/iOS rendericen el proto sin un ensayo real.

`src/ui.rs` permite HTML **solo como lenguaje abreviado de estilo**:
`b/strong`, `i/em`, `s`, `code`, `br`, algunas entidades,
sin atributos ni JavaScript, convertido a marcado de WhatsApp.
No se crea un WebView, no se usa el tipo experimental
`FOAHtmlPrimitiveDemoDONOTUSE`. La discriminación Android/iPhone
no está implementada: no existe una identidad de SO autenticada en
los eventos que recibe esta versión del proveedor.
Ninguna función nueva envía mensajes espontáneos, cambia roles o
activa el bot por defecto. Mantener respaldo de sesión y bloqueo.

## 2026-11-18 — Corrección de la prueba de `status --global` público

El CI del proveedor sobre `ce57169` compiló y pasó 15/16 pruebas
`tests/provider.rs`; la restante aún esperaba ignorar `status --global`
de un lector externo. Ahora esa consulta se firma pero el controlador
siempre devuelve una negativa de privacidad; nunca expone políticas
ni acepta cambios del socket. Actualizada la prueba y solicitada
verificación nueva sobre el SHA exacto.

## 2026-11-18 — Menú original y lectura contextual / global (alpha)

El proveedor `whatsapp-rust` conserva el mensaje original de Matterssmith en
`=/>menu`, incluido el crédito final, sin una segunda caja de control.
`=/>status` de un participante externo se firma para la lectura del chat
efectivo, aun con el bot apagado. `=/>status --global` del propietario funciona
desde el chat propio y grupos autenticados, con las políticas reales.
Cuando un tercero pide `--global`, se firma únicamente una consulta inocua
y el controlador responde con una negativa informativa; no entrega datos de
socket ni admite mutaciones. Los cambios de estado siguen siendo owner-only.
La vinculación sigue por pairing code, con procesos Rust independientes y sin
modificar bases de sesión. Son correcciones sujetas a CI y prueba WhatsApp real.

# WhatsApp-Matter: proveedor Rust independiente

El proveedor original es whatsapp-rust, ejecutado en un **proceso Rust
separado** del núcleo Base-Matter. No usa Node ni Baileys ni Bot-Matter.

La separación responde a versiones incompatibles de libsqlite3-sys entre
la biblioteca whatsapp-rust 0.7.0 y las dependencias actuales de Base.
Ambos procesos siguen escritos en Rust y se comunican por un canal TCP
en 127.0.0.1 protegido con HMAC-SHA256 de 256 bits.

## Verificación del proveedor Rust (alpha; 2026-11-18)

GitHub Actions `44367046939` aprobó la verificación independiente del
proveedor para `7de9a0d`, tras la corrección de formato del marco compartido.
Queda una advertencia de Clippy del núcleo en las pruebas de antis, corregida
en el siguiente checkpoint. Este resultado no prueba un reinicio real de
sesión WhatsApp ni una moderación automática activa.

## Corrección de verificación Rust (alpha; 2026-11-18)

El último CI (`6142c6c`) falló por formato de la plantilla compartida
`src/control_style.rs` en la comprobación del proveedor, no por errores
verificados de emparejamiento. El mismo cambio también corrige el formato
del núcleo y la prueba desactualizada de créditos. Falta nuevo CI sobre el
SHA exacto y prueba real de WhatsApp antes de declarar esta revisión estable.

## Clippy: segundo consumidor del snapshot (alpha)

En la nueva verificación de `alpha`, Clippy del núcleo se detuvo
en `tests/whatsapp_native.rs:82`, que aún accedía al campo
posicional `.0` de `inspect_bot_defaults`. Se reemplazó por
`.socket_powered`, sin alterar la autenticación, las respuestas
tematizadas o la vinculación por pairing code. La suite del proveedor
no falló; simplemente no llegó a ejecutarse en ese intento porque
`set -e` detuvo el bloque en Clippy. Falta repetir ambas suites.

## Menú compacto y comando de créditos (alpha; pruebas reales pendientes)

El proveedor Rust permite `=/>credits`, `=/>creditos` y
`=/>créditos` con la misma autenticación y verificación HMAC que
los demás controles. Para remitentes ajenos solo se permite lectura.
Los mensajes normales heredan el marco común corto, sin un footer
automático de autor ni nombres duplicados. La atribución completa
permanece en el código y se muestra exclusivamente mediante
el comando de créditos.

Referencias estudiadas: Aura-Bot-2.0 usa mensajes enriquecidos,
miniaturas y, en comandos concretos, una vista HTML experimental
(`FOAHtmlPrimitiveDemoDONOTUSE`). El ZIP YoSoyYo incluye
`nativeFlowMessage` y botones `quick_reply` / `cta_url`.
Ninguno ofrece una detección fiable del sistema operativo del destinatario.
**No se envían aún botones HTML ni versiones forzadas Android/iOS**
desde este proveedor; implementar eso sin detección fiable rompería
el requisito de no mostrar una variante equivocada. El transporte
Rust, el pairing code, los archivos SQLite y el control de sesión
permanecen sin modificaciones.

## Ajuste posterior a Clippy del núcleo (alpha)

La nueva compilación de Clippy encontró una referencia posicional
antigua en la prueba del puente `tests/whatsapp_bridge.rs`.
La referencia fue actualizada a `BotDefaultPolicies::socket_powered`;
el proveedor WhatsApp Rust no requiere cambios de protocolo ni
de sesión para este ajuste. La suite independiente de 21 pruebas
estaba aprobada en la ejecución anterior, no en el nuevo SHA.
Las verificaciones estrictas restantes deben repetirse.

## Resultado comunicado del proveedor Rust — 2026-11-18

El operador ejecutó la suite independiente del proveedor después
de corregir el ingreso nativo. Pasaron **21 pruebas**:
una del ejecutable principal, dieciséis de `tests/provider.rs`,
tres de recuperación y una de REGISTER. El bloqueo restante
reportado fue `clippy::type_complexity` del **núcleo Base**,
no una prueba fallida del proveedor.

La corrección sustituye la tupla de seis políticas por una
estructura Rust nombrada, sin alterar el protocolo HMAC,
las sesiones SQLite, pairing code ni las plantillas WhatsApp.
Falta la nueva ejecución de Clippy de ambos paquetes.

## Revisión del ingreso nativo (alpha; suite completa pendiente)

La suite Base identificó un caso donde `=/>on` retirado y
demasiado largo se ignoraba antes de validar el límite de entrada.
Se reordenó la validación en `src/whatsapp_native.rs`. El
proveedor independiente continúa firmando solo los comandos
autorizados y no recupera los alias antiguos. La prueba nueva
comprueba también un `=/>bot on` excesivo y que un rechazo
no consuma el identificador de replay. No modificar sesiones
WhatsApp, archivos SQLite ni el `Cargo.lock` local.
La nueva ejecución del proveedor aún no está confirmada.

## Validación por fases en Debian (alpha)

La última suite reportada del **núcleo** se detuvo por
una aserción antigua de `tests/control_style.rs`, que
buscaba un `bot_id` interno en el menú compacto.
La expectativa ya fue corregida en GitHub, sin añadir
dicho identificador a las respuestas. El proveedor Rust
no llegó a ejecutarse en ese lote: **no asumir su
aprobación para el nuevo commit**.

El operador tiene modificaciones locales generadas por
`cargo fmt` y un `providers/whatsapp-rust/Cargo.lock`
sin seguimiento. Preservar ambos al sincronizar; el lock
no contiene una sesión WhatsApp y no se debe borrar como
forma de corregir esta regresión. Repetir las pruebas del
proveedor después de confirmar la suite central.

## Respuestas propias y plantilla seleccionada de cada chat (alpha)

La respuesta de `WhatsAppControlIpc` llega ya renderizada con la
plantilla del chat verificado. El proceso `whatsapp-rust` conserva
esa respuesta y **no impone encima el diseño predeterminado**.
Para informes generados directamente en el proveedor (REGISTER,
recuperación comunitaria y respuestas de reserva), utiliza
`render_provider_control`, que carga exactamente el mismo
archivo de plantilla y nunca otra variante escrita dentro de
los handlers.

La prueba nueva `tests/whatsapp_theme_scopes.rs` verifica en
el núcleo la salida por grupo, con créditos y saltos de línea
reales. Falta probarlo con el binario WhatsApp conectado.
No se cambian SQLite, pairing code, claves ni sesiones.

## Contrato WhatsApp actual: un solo comando y una sola plantilla (alpha)

Los comandos `=/>on/off` se han retirado del proveedor y del puente
firmado. Usar solo `=/>bot on/off` en el chat autorizado; habilitar
despierta el socket sin un encendido adicional. Para predeterminados
usar `--global` o los alcances `group/private/channel/community`
en el chat propio. La variante `community` corresponde a avisos
comunitarios: no se aplica a grupos ordinarios sin metadatos de tipo.

El proveedor **reutiliza `src/control_style.rs`** para decorar también
respuestas directas REGISTER, recuperación y vistas previas. Ambos
procesos comparten las plantillas editables de la raíz:
`templates/matterssmith-control.txt` y
`templates/matterssmith-menu.txt`. No copiar diseños dentro de los
manejadores. El menú único no tiene subcategorías. El salto de línea
es un carácter real, jamás texto literal `\n`.

Esta revisión está publicada solo en alpha, pendiente de pruebas
reales en Debian; los 20 tests del proveedor informados previamente
corresponden al commit anterior `dfd9dab`, no certifican este cambio.

## Distinción en la suite Base: consulta grupal con bot apagado

La regresión siguiente, tras aprobar `control_ipc`, estaba
en `tests/group_control.rs`: la prueba rechazaba
`=/>status` desde un grupo del propietario. Se ajustó
la expectativa para exigir estado grupal `PoweredOff`
cuando el socket está apagado, sin permitir operaciones
administrativas de un participante externo. El proveedor
WhatsApp no necesita cambios para este caso; su suite
independiente ya había aprobado 20 pruebas.
La suite Base debe volver a ejecutarse para validar la
corrección antes de continuar con WhatsApp real.

## Resultado de compilación del proveedor (Debian, 2026-11-18)

El operador informó `cargo test --manifest-path
providers/whatsapp-rust/Cargo.toml --all-targets` con
**20 pruebas aprobadas y ninguna fallida**: una prueba del
binario principal, quince del protocolo/proveedor, tres de
recuperación y una de REGISTER. El defecto que persiste es una
expectativa en la suite **Base**, no en este proveedor:
`=/>bot private off` debe deshabilitar privados externos,
sin afectar al chat propio del dueño. La prueba IPC ya fue
alineada con esa separación, pero necesita ejecución Debian.
No eliminar `Cargo.lock`, sesiones, llaves ni caché compilada.

## Lectura pública autenticada sin rangos ni control de socket (alpha; pruebas pendientes)

En un grupo, el proveedor puede firmar `status/menu/bot` procedentes
de otros participantes usando el JID PN/LID real del remitente. En
un chat privado externo exige que el JID del chat coincida exactamente
con el del remitente. La sesión conectada debe coincidir además
con la identidad emparejada del socket; el HMAC vincula al actor,
chat, comando, mensaje y tiempo. En la capa Base una ruta especial
devuelve solo estado o menú, sin invocar extensiones generales.

No se acepta `=/>status --global`, `--debug`, `=/>on`,
`=/>bot on`, cambios de configuración ni `=/>set role`
desde terceros. Cuando está deshabilitado el bot, el estado
sigue respondiendo y el menú no responde. No asumir
funcionamiento de canales o privado PN/LID cruzado aún:
esas identidades necesitan pruebas verificadas del proveedor.
El chat propio usa política `self`, no el predeterminado
`private` de chats externos. No se modificaron sesiones,
llaves, ramas beta/stable ni repositorios de roles.

## Contratos recientes de chat, estado y silencio (alpha; build Debian pendiente)

La sesión real ya respondió comandos de propietario dentro de un
grupo, pero dejó claro que `PoweredOff` significa socket apagado,
no fallo de la política grupal. En alpha se incorporaron
`=/>status` (contexto actual), `=/>status --global`
(predeterminados), `=/>bot on/off/inherit` (chat),
`=/>bot group/private/channel on/off/inherit` (tipos) y
`=/>bot --global on/off/inherit` (socket).

`=/>menu` / `=/>menú` muestra un diseño Matterssmith propio por
categorías al habilitar el bot. Los comandos normales, el menú y
la ayuda de `bot` no responden cuando la política está apagada;
`status` sigue disponible. `=/>help` y `=/>h` se retiraron.
El proveedor acepta un acuse `ignored` auténtico y no lo responde
en WhatsApp. El límite acotado de respuestas IPC se amplió para
incluir el menú decorativo y `--debug` sin cargar datos secretos.

**Límite de seguridad:** `from_me` y PN/LID solo autorizan al
propietario vinculado; no abrir mensajes de otros usuarios
suplantando su identidad. El soporte de canales y privados
externos exige un nuevo ingreso autenticado. La conexión con
Vault-Matter y `selfmod` aún no están instalados.
Ver `../../docs/BOT_POLICY_V2.md`.

## Pruebas offline del binario y recuperación (alpha)

La suite del proveedor se ejecuta de forma independiente al núcleo:

```bash
cargo test --manifest-path providers/whatsapp-rust/Cargo.toml --all-targets
```

`tests/recovery.rs` resuelve el ejecutable
`whatsapp-matter-rust-provider` mediante
`option_env!("CARGO_BIN_EXE_whatsapp-matter-rust-provider")` y,
en su ausencia, `std::env::var_os` durante la prueba. La prueba
invoca realmente `--recovery-verify`, rechaza la clave equivocada
y rutas inseguras, y verifica que la sesión SQLite no se abre.
No crea una nueva vinculación de WhatsApp ni borra las sesiones.

`tests/provider.rs` utiliza el mismo mecanismo para `--check`.
Su prueba del JID `@lid` usa `=/>status` en privado, y confirma
que `=/>menu` se ignora en ese ámbito. Pruebas de compilación
y ejecución en el servidor: pendientes tras esta corrección.

## Fuente única de permisos (Vault-Matter)

Los rangos de socket y grupo no se pueden escribir localmente en
Base-Matter. El usuario dirige altas, cambios y revocaciones de
rango a un canal **propio y autenticado de Vault-Matter** en WhatsApp,
Telegram o Discord. No se aceptan mensajes reenviados o un nombre
visible como prueba de autorización. El servicio Vault es la única
autoridad, autentica cada actor, cifra las asignaciones y mantiene la
persistencia del repositorio privado. Base consulta permisos como
lector de solo lectura y deniega todo permiso no confirmado.

**Hasta que exista el servicio y el verificador:**
`=/>set role`, `=/>unset role`, `=/>role` y
`=/>group role` se rechazan sin modificar SQLite. El
`=/>roles` privado indica que falta la conexión con Vault.
La identidad propietaria original puede seguir apagando el socket
sin depender de un rango delegable.

## Archivo histórico — División privado/grupo anterior (sustituida por BOT_POLICY_V2)

El control remoto ya respondió en WhatsApp al propietario con `=/>status`
y `=/>on`. El chat privado propio se reserva desde ahora a los
ajustes globales del socket; `menu/help/start/add/add++` son
comandos de ámbito grupal. El proveedor verifica `from_me`, el
PN/LID real del cliente, el chat `@g.us` y el remitente propio.
El destino grupal queda firmado en el HMAC; el controlador determina
el grupo por ese dato autenticado, sin aceptar IDs en texto como
prueba de permisos. El controlador del socket no arranca Bot-Matter.
`=/>group on/off/inherit/status` afecta únicamente a ese grupo;
el encendido general `=/>on/off` sigue siendo privado.

`=/>set role <PN/LID> <rango>` es control privado del socket.
`=/>group role <PN/LID> <rango>` es una asignación específica
del grupo, todavía no una delegación de ejecución. Los rangos no
pertenecen al escalafón de administración del proyecto. `user`
es implícito; el almacén local persiste solamente asignaciones
no predeterminadas. Falta la publicación cifrada autenticada en
Vault-Matter y su recuperación; no asumir persistencia remota.
Las pruebas de grupos reales siguen pendientes en esta revisión.

## Corrección de identidad PN/LID autenticada (alpha)

La sesión vinculada se identifica ahora mediante `ctx.client.pn()` y
`ctx.client.lid()` de whatsapp-rust 0.7.0. Solo se aceptan mensajes
`=/>` propios en chat privado cuya identidad de chat y remitente
coincidan con PN o LID verificados del cliente, con JIDs de dispositivo
normalizados. El PN autenticado debe coincidir con el número configurado
y el JID del propietario debe ser uno de esos identificadores. El puente
HMAC conserva el JID canónico del propietario. Siguen denegados chats
ajenos, grupos, mensajes entrantes y prefijos antiguos `#ping`.
No es necesario regenerar la sesión ni el archivo `runtime.env`.
La prueba de recepción y respuesta real sigue pendiente.

## Ejecución en una sola terminal (alpha)

Para ahorrar espacio en Debian, utilizar **un solo clon Git** y exportar
`CARGO_TARGET_DIR` como ruta absoluta compartida antes de ejecutar
`cargo run`. El lanzador de `alpha` respeta la ruta tanto para
`whatsapp-matter-control` como para este proveedor Rust independiente.
Los artefactos compartidos no contienen las sesiones SQLite. Las sesiones,
claves y configuración son locales y particulares de cada rama. Cambiar de
rama requiere detener antes cualquier proceso WhatsApp activo. `stable` y
`beta` conservan sus propias verificaciones; este lanzador no ha sido
promovido y aún requiere prueba en un dispositivo real.


La rama `alpha` incorpora `cargo run` desde la raíz de Base-Matter.
El ejecutable `base-matter` inicia el controlador y este proveedor
Rust como procesos independientes bajo una sola sesión de terminal,
para evitar conflictos `libsqlite3-sys`. No usa Node ni QR.

Cada ejecución utiliza una ruta privada y un puerto reservados según
la rama Git: `alpha` usa 39821, `beta` 39822 y `stable` 39823.
El lanzador solo existe por ahora en `alpha`; las ramas `beta` y
`stable` requieren promoción y verificación por separado.
Sin configuración previa pregunta únicamente el teléfono y conserva
su SQLite y secretos para el siguiente arranque. Un JID derivado de
un número no garantiza coincidencia con el LID autenticado y los
comandos propios permanecen denegados ante cualquier discrepancia.
Una conexión en pantalla no demuestra que se haya aplicado `=/>status`.

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
Solo responde con el acuse genérico `applied` al chat propio autenticado;
no responde ping ni ejecuta Bot-Matter.

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

## Guía de prueba real

La [guía de dos terminales y emparejamiento](../../docs/WHATSAPP_TRIAL.md) usa únicamente Rust. El script `scripts/whatsapp-test-setup.sh` prepara claves privadas diferentes sin sobrescribir sesiones. El modo `--check` valida el entorno antes de abrir una sesión. Nunca compartas el código de vinculación ni los secretos.

## Propiedad de sesión segura

En sistemas Unix, el archivo `owner.lock` se crea con permisos `0600` en la llamada de apertura, sin una ventana inicial con permisos de grupo/otros. La carpeta de sesión debe seguir siendo privada (`0700`). Un bloqueo residual exige recuperación manual verificada; no eliminarlo automáticamente ni borrar `session.db`.

## Resultados del control global

Al enviar `=/>`, el proveedor diferencia el efecto nuevo `applied` del
replay `duplicate` y los rechazos. Un resultado `duplicate` nunca se anuncia
como ejecución nueva; los acuses malformados o desconocidos se rechazan.
El control solo intenta una confirmación breve cuando se ejecuta una
operación; no devuelve todavía los detalles del estado. Requiere prueba
real autorizada antes de promoverlo a estable.

## Reinicio seguro tras una caída

En Rust 1.89 o posterior, la exclusión del archivo `owner.lock` utiliza un
bloqueo de sistema operativo y un marcador v2 permanente. Tras un cierre o
caída del proceso, el bloqueo se libera y el mismo archivo puede utilizarse
de nuevo sin tocar `session.db`. Si hay otro proceso activo, se deniega
el acceso. Los antiguos locks con PID (v1) se rechazan por seguridad hasta
la revisión manual del operador: la migración nunca borra locks ni sesiones.
`--check` verifica la disponibilidad del archivo de modo de solo inspección.

## Orden de arranque comprobado

Al ejecutar el proveedor para vincular WhatsApp, primero comprueba que el
receptor Base-Matter escucha en la dirección 127.0.0.1 configurada; si no,
aborta antes de abrir la sesión SQLite o solicitar un código. La comprobación
solo establece disponibilidad TCP, **no autentica el controlador**: el HMAC
protege los comandos. El modo `--check` sigue sin red ni escrituras.

## Confirmación en el chat privado

Tras un acuse `applied` autenticado, el proveedor intenta enviar al
chat propio «Base-Matter: solicitud procesada.». Los duplicados y errores no
generan respuestas. No se envía a grupos, terceros ni mensajes que no tengan
el prefijo `=/>` desde el JID autenticado. Es un aviso genérico: no implica
que `status` ya tenga salida detallada, ni sustituye las pruebas de entrega
con un teléfono autorizado.

## Comunidad REGISTER (alpha)

`=/>start` se reserva para inventario y verificación de comunidades
gestionadas por la sesión de WhatsApp. El diseño en
[`docs/COMMUNITY_REGISTER.md`](../../docs/COMMUNITY_REGISTER.md)
requiere verificar el rol admin y los identificadores exactos de subgrupo,
y distingue vista previa de operaciones con cambios. Ninguna actualización
administrativa se debe asumir completa por aprobar CI offline.

## Encabezado oficial REGISTER

La función de detección solo reconoce la primera línea exacta
`MATTER:OFFICIAL:register:<community-jid>`, validando formato de JID
comunitario. Las coincidencias parciales, colocadas en otra línea o con
ID de otra comunidad se rechazan. La descripción **no es autenticación**;
además se deben verificar metadatos de pertenencia y rol de administrador.

## Escaneo de comunidades reales con =/>start (alpha)

Tras pasar HMAC y permisos en Base, el callback de `whatsapp-rust` utiliza
`Community::get_participating`, `get_subgroups` y
`query_linked_group`. El escaneo verifica el admin del propio JID PN/LID,
pertenencia del subgrupo a la comunidad y encabezado REGISTER exacto.
Responde en el chat propio y no modifica grupos ni roles. Limita a 12
comunidades, 64 subgrupos por comunidad y 20 segundos por solicitud.
No se puede declarar probado en red sin un teléfono vinculado.

## Respuesta de comandos globales desde Base

El proveedor admite el acuse opcional `message` de una operación
`applied` para mostrar menús o estados en el self-chat. Exige menos
de 761 bytes, texto UTF-8 válido, sin NUL ni controles salvo saltos de
línea. Los `duplicate`, `denied` y campos inesperados no contienen
respuestas. La verificación no equivale a privilegio de administración.

## Ayuda y menú del controlador Base (alpha)

Los comandos `=/>menu`, `=/>help` y `=/>h` devuelven en el chat
privado propio el formato autorizado por Base-Matter. El texto se valida
y se limita a 760 bytes; los mensajes duplicados o no autorizados no
generan respuesta. El proveedor no interpreta comandos externos
`#matter` ni importa sesiones de bots ajenos.

## Menú simple para pruebas beta

El menú de control de Base-Matter es intencionalmente plano. `=/>menu`, `=/>help` y `=/>h` no incluyen una estética atribuida a otro bot sin una muestra observada y aprobada. Todas las respuestas siguen exigiendo validación del dueño del socket.

## Prueba desde la nueva rama beta

La guía `docs/WHATSAPP_TRIAL.md` corresponde ahora a `beta`. `=/>menu`, `=/>help` y `=/>h` muestran comandos del núcleo sin diseño preestablecido, y `=/>start` solo consulta comunidades. No copiar menús de otras cuentas sin una fuente verificable.

## Respaldo de comunidades `=/>add++` (fase alfa)

El controlador Base-Matter autoriza un inventario de recuperación, pero todavía no incorpora participantes ni crea un grupo espejo. El respaldo cifrado local y la lectura real del proveedor se integran de forma separada, conservando permisos, privacidad, exclusión de duplicados y protección contra nuevas suspensiones. La prueba actual de WhatsApp desde `beta` no debe declararse capaz de migrar usuarios automáticamente.

## Inventario de recuperación WhatsApp

Después de que Base-Matter autorice `=/>add++`, se consultan las comunidades donde la sesión actual figure como administradora. Se muestran conteos aproximados de integrantes de subgrupos (sin publicar identidades), el ID de comunidad y el nombre sugerido del grupo de respaldo. No se añaden participantes; no se extraen miembros de comunidades suspendidas.

## Almacén local de instantáneas cifradas

El módulo `recovery` cifra registros de participantes y la identidad de la comunidad con XChaCha20-Poly1305, usando una clave independiente `MATTER_WA_RECOVERY_KEY_HEX`. Se crean archivos nuevos `community-recovery-*.enc.json` con permisos 0600 en la carpeta privada del SQLite; no se sobrescriben snapshots anteriores. Se exige revalidar el socket y el dueño al descifrar. Para un entorno ya configurado, ejecutar `bash scripts/whatsapp-test-setup.sh --enable-recovery` sin borrar la sesión. No confundir snapshot de miembros con exportación de credenciales de WhatsApp.

## Comando real de instantánea (alpha)

`=/>add++ snapshot <ID_COMUNIDAD@g.us>` solicita un respaldo de identidades PN/LID desde los grupos enlazados solo cuando la cuenta es admin de esa comunidad y WhatsApp permite consultarla. Se deduplican las identidades conocidas, se cifran usando la clave independiente y se escribe un archivo privado nuevo. La respuesta enviada al chat muestra el conteo y el nombre del archivo, no la lista. No se agrega nadie ni se eluden suspensiones o privacidad. Para habilitar la clave en una configuración existente: `bash scripts/whatsapp-test-setup.sh --enable-recovery`.

## Verificar el respaldo sin sesión activa

Con el mismo `runtime.env` y una copia del archivo cifrado, `cargo run --manifest-path providers/whatsapp-rust/Cargo.toml -- --recovery-verify NOMBRE.enc.json` valida clave, integridad, socket y dueño sin abrir la sesión de WhatsApp. El nombre debe corresponder a un archivo del directorio privado y no puede contener rutas. El resultado muestra solo el conteo, nunca PN/LID.

## Verificación del binario real

Se añadió prueba offline ejecutando el binario `--recovery-verify`, comprobando integridad, ausencia de sesión SQLite nueva, rechazo con clave errónea y no filtración de JIDs por stdout. No se ejecuta ninguna operación de grupo ni conexión de WhatsApp en CI.

## Nuevo =/>add (alpha)

El controlador Base-Matter valida la gramática de `=/>add` y responde con un acuse de vista previa, sin realizar altas, traslados ni cambios de permisos. `reply` exigirá metadatos de un mensaje real; `official` deberá resolverse desde un registro verificado de grupos/comunidades. El proveedor actual admite controles globales desde el chat propio y no se declara listo para operar `add user` dentro de grupos.

## 2026-11-25 — Ping de Matter-Bot y corrección CI alpha

El propietario define la plantilla original de Matter-Bot en `docs/MATTER_BOT_PING.md` (Socket, Process, Total). Separada del control autenticado `=/>ping`; `#ping` no se habilita en el proveedor de control. Se aplican diferencias rustfmt exactas de CI y se actualiza prueba IPC: ping autenticado aceptado, replay duplicado y argumentos extra denegados. CI anterior falló formato y expectativa histórica de ping; nueva ejecución pendiente. Error local instabox-se en whatsapp_theme_scopes sin causa inicial en la salida. No tocar sesiones/Vault. README revisados: 2/2.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
