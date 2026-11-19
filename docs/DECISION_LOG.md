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

## BASE-D018 — AntiLink único y dominios por ámbito (2026-11-19)

- Petición del propietario: eliminar filtros Anti por red social y usar `set anti link` para añadir varios dominios por chat o `--global` por socket.
- Decisión: nombres de reglas sociales retirados del catálogo operativo; sin modificación destructiva de datos viejos. Lista persistente `socket_id,scope_id,domain` por SQLite, con auditoría y autorización del dueño.
- Filtro general OFF por defecto, detección pasiva, sin acciones automáticas. Dominio global heredado, local independiente.
- Base 8b0fc5516e6d8ab7e313469bfbd05bd1103d7f30. CI pendiente. READMEs 2/2. Bot-Matter bloqueado.

## BASE-D017 — Frontera de negocio Matterssmith Commerce (2026-11-19)

- Idea tienda/hardware/generador del propietario convertida a contrato transversal; Base-Matter no implementa lógicas de mercado ni datos Vault.
- Contratos neutrales y seguridad siguen siendo su función; Bot-Matter permanece bloqueado. SHA previo 24349b0ae29c88ebecda571de01821fe310b8800. No hay pruebas comerciales activas.

## BASE-D016 — Rustfmt de contrato guest (2026-11-19)

- CI del SHA 3c1fa6f325df63718eb67eefb9ce1a66ff4424e4: proveedor PASS, Rustfmt FAIL. Aplicar solo cambios de estilo requeridos por CI en lib.rs y tests/whatsapp_public_readonly.rs.
- Sin cambios de seguridad/alcance. README 2/2. Nuevo SHA pendiente de CI.

## BASE-D015 — Sin registro obligatorio: invitado demo (2026-11-19)

- Invitados pueden consumir funciones básicas autorizadas, sin CURP, biometría ni registro Vault; heredan las restricciones normales del socket/chat.
- `=/>register` no da roles ni confirma alta. El futuro gateway de Vault es voluntario y verificará consentimiento para funciones personales.
- Base alpha previa: ea7c0da6c579f24406093940e7677fe6a349dfbd. Nuevos tests guest + puente. README 2/2. Bot-Matter todavía bloqueado. CI y prueba de teléfono pendientes.

## 2026-11-19 — Clippy Rust del receptor ListResponse

- CI del SHA 031ebb79ad1edc7c81d68a8997ba9c11918681b5: proveedor compiló y todos sus tests pasaron; Clippy estricta falló en tests/ui.rs por un `..Default::default()` innecesario en `SingleSelectReply`. Se quitó sin modificar protocolo ni permisos.
- README revisados 2/2. CI del nuevo SHA pendiente. Siguiente: lista de dominios personalizable en vez de reglas anti por red social, después prueba real con =/>test ui.

## BASE-D013 — Decodificar ListResponse + ACK interactivo verificado (2026-11-19)

- Una burbuja «=/>menu anti» no equivale a que el proveedor haya recibido el clic.
- Interpretar ListResponse y TemplateButtonReply además de NativeFlow y botones clásicos, con IDs cerrados y HMAC.
- Distinguir ACK tecleado del callback real para test UI; sin permisos adicionales. SHA base 37698e3967af5b2857c252473d5ad792a289a3e1. CI pendiente. README 2/2.

## BASE-D012 — Rustfmt y regresión de botones NativeFlow (2026-11-19)

- SHA base: 351d4d4a416cf8aa6b15fc33a6607bf1150763b7; el CI de proveedor marcó fallas de formato solamente.
- Decisión: aplicar rustfmt exacto y probar tamaños 2/3, labels y correspondencia al ACK; mantener todas las rutas bajo control HMAC/antirreplay.
- Evidencia UI Android del selector disponible; render visual TS18/TS19 y HTML siguen pendientes.
- README revisados: 2/2; CI nuevo pendiente.

## BASE-D011 — El título del selector debe ser el comando, y laboratorio NativeFlow opt-in

- Reporte: WhatsApp muestra al pulsar una fila «☾︎ Anti / Abrir menú». Decisión: `title` exactamente `=/>menu anti`; quitar `description`, preservar IDs opacos, HMAC y read-only.
- La prueba TS18/TS19 anteriormente solo usaba ButtonsMessage legacy y self-chat. Se agrega NativeFlow quick_reply (compatible a nivel de transporte con el selector probado). En `MATTER_WA_MENU_LAB=1` se muestran tarjetas readonly en privados habilitados, con fallback legacy por fallo de transporte; el dueño en self-chat recibe también comparación legacy.
- HTML/audio siguen limitados al self-chat del dueño; compatibilidad HTML del cliente aún no demostrada.
- Base alpha: c2d8c2c84fe1bbd8ed11f667004654848c9bc9ae. CI y prueba real pendientes. README revisados: 2/2.

## BASE-D010 — Instrucciones compactas en citas de WhatsApp (2026-11-19)

- Solicitud de Dev MS Leito Delfin 3 Siss: convertir el bloque largo de instrucciones en líneas compactas, usando `> texto`.
- Diseño: cuatro citas breves; sin párrafos o líneas vacías entre instrucciones. Se mantiene la decoración ajena al contenido, las categorías no vacías y el botón único.
- Compatibilidad: los estilos `classic` y `none` comparten semántica; `none` no recibe citas decorativas.
- Reutilización: `TextFormat::Quote` con saltos de línea seguros y catálogo de formato extendido.
- Base alpha: `8a1176ff7cdfe1d86c5c5d689c4ec2e9cd1620c0`. Revisión 2/2 README. Pruebas automáticas y prueba visual nuevas: pendientes.

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

## BASE-D009 — UX Leito y alfabetos reutilizables (2026-11-19)

- Propuesta recibida: hacer perceptible la diferencia entre adorno y contenido mediante formatos de WhatsApp.
- Decisión: ocultar categorías sin comandos implementados en menú y botones; mantener 21 identificadores solo en contrato interno y routing histórico de lectura.
- Usar manifiesto compartido en Rust y catálogo JSON de cinco alfabetos, marco reutilizable y módulo de formato. No conceder permisos ni añadir comandos.
- Base: 8639b9e4beab5217f344766d74c1b85578b3daba. CI pendiente; READMEs 2/2.

# Historial de decisiones de implementación — Base-Matter

## BASE-D008 — corrección de casos positivos/negativos de grupo
- Incidencia en  ce9465a8: reemplazo inicial mal ubicado.
- Ahora `=/>menu group` aparece solo en el conjunto autorizado
  y `=/>menu unknown` solo en el conjunto rechazado.
- El código de seguridad no cambió; solo la prueba.
- README revisados: 2/2; CI pendiente.


## BASE-D007 — recuperación del checkpoint y test de grupo
- SHA anterior: 97de8ac040148925baa194710a0d5dab82d90aff.
- Incidencia: el commit previo omitió WORK_CONTINUITY.
- Prueba: `menu group` es una categoría permitida, no debe
  figurar en la lista de denegaciones del signer.
- Corrección: sustituida por `menu unknown`.
- Resultado: revalidación nueva pendiente; README 2/2.


## BASE-D006 — Fixture dinámico del menú y guías WhatsApp
- Falla CI: tests/whatsapp_public_readonly.rs probaba la nueva
  navegación con un renderer de plantilla estática antigua que sólo
  soporta "menu". No es fallo de permisos de la ruta pública.
- Solución: mantener fixture legado para sus tests y probar
  categorías/páginas en fixture con MenuPreferences real.
- Formato: corrección de rustfmt del match del proveedor.
- Documentos: instrucciones precisas de prueba audio/HTML/TS18/TS19.
- Estado: CI nuevo por verificar; ningún dato secreto modificado.
- README revisados: 2/2.


## BASE-D005 — Rustfmt laboratorio, 18-11-2026
- Problema: CI de provider y Rustcore rechazaron formato de arrays,
  match e imports del laboratorio y los tests de catálogo.
- Decisión: aplicar rustfmt sin variar comportamiento ni permisos.
- Evidencia: workflows de SHA 05ec05c5bba5bb33b63dcb9aec457e2f8fcedda4.
- Resultado: archivos modificados y publicados, CI nueva pendiente.
- README revisados: 2/2.


## BASE-D004 — Ingreso de 21 categorías y páginas, 18-11-2026
- Se evita que el click de un selector ya autorizado sea descartado
  por el signer público, manteniendo lectura sin permisos de modificación.
- Las 21 rutas visibles y las páginas 1–3 son entradas válidas;
  aliases históricos sólo de compatibilidad.
- Pruebas: signer read-only y despachador con las 21 entradas,
  seguridad de botones, vista HTML con audio original.
- Estado: CI pendiente, teléfono todavía no probado. README 2/2.


## BASE-D003 — Menú HTML y sonido TS18/TS19, 2026-11-18
- Origen: prueba visual =/>menu pedida por el propietario.
- Decisión: un selector y 21 categorías primero; tarjetas TS18
  (Audio/MP4), TS19 (Audio/MP4/Enlace), RichResponse y un tono
  de 0.45s como extras sólo en self-chat verificado y opt-in.
- Implementación: proveedor Rust, audio Opus original, sin JavaScript,
  suplantación Meta AI, descargas ni roles; fallback texto.
- Pendiente: CI Linux/Windows/proveedor, móvil real, compatibilidad
  RichResponse y botones. No afirmar éxito visual sin pantalla.


## 2026-11-18 (MX) — BASE-D002: permiso informado de Jeriel
- **Origen:** titular del ecosistema comunica autorización de Jeriel,
  creador de Aura Reed, para usar su clave de IA según lo planeado.
- **Límites:** no se informaron restricciones numéricas. NO confundir
  eso con cuota infinita, ausencia de términos del proveedor ni
  autorización para otras capacidades.
- **Estado:** consentimiento comunicado; evidencia/titularidad
  operativa pendiente de verificar por Vault, sin clave ni gateway.
- **Referencia autoritativa:** Vault-Matter privado, rama matter,
  docs/AI_PERMISSION_AURA_REED.md, commit
  8872f9c3dc3d6aafcf4268c531e7c07f571cbcf4.
- **Arquitectura:** Base-Matter continúa solicitando sólo resultados
  al broker autenticado de Vault. No extrae claves ni ejecuta el
  código sugerido por la IA. Capacidades previstas: guía de menús,
  revisión/propuestas de código con control independiente.
- **Archivos:** AGENTS, docs/MATTER_AI.md, documentos de continuidad
  y ambos README. No cambia transporte ni claves.

## 2026-11-18 (MX) — BASE-D001: registro de cada idea/cambio
- **Origen:** petición expresa de guardar instrucciones y punto
  exacto de avance en caso de perder la comunicación.
- **Regla:** AGENTS, este historial, docs/WORK_CONTINUITY.md
  y docs/DEVELOPMENT_STATUS.md se actualizan en cada commit.
  Auditar README rastreados (2/2).
- **Control:** scripts/check_continuity.py integrado a CI para
  impedir commits futuros que omitan estos registros.
- **Problema corregido:** test de Linux esperaba encabezado de help
  dentro de menu antis 2. Se separa menu de help. En Windows,
  tests/command_audit.rs chocaba en nombres temporales por PID/reloj.
  Se reemplaza la carpeta de prueba por nonce criptográfico.
- **Estado:** cambios incorporados en alpha, revalidación CI pendiente.
- **Siguiente:** validar Linux, Windows y proveedor sobre SHA exacto,
  revisar manualmente menú/help y selector en WhatsApp.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
