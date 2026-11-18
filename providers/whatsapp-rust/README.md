# WhatsApp-Matter: proveedor Rust independiente

El proveedor original es whatsapp-rust, ejecutado en un **proceso Rust
separado** del núcleo Base-Matter. No usa Node ni Baileys ni Bot-Matter.

La separación responde a versiones incompatibles de libsqlite3-sys entre
la biblioteca whatsapp-rust 0.7.0 y las dependencias actuales de Base.
Ambos procesos siguen escritos en Rust y se comunican por un canal TCP
en 127.0.0.1 protegido con HMAC-SHA256 de 256 bits.

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
