# Base-Matter — Recuperación de miembros de una comunidad
## Necesidad
Tras una suspensión de una comunidad de WhatsApp, el administrador desea conservar el contacto con los usuarios que ya participaban para invitarlos a la nueva comunidad.

## Comandos
- `=/>add++`: solo el dueño del socket puede previsualizar comunidades administradas, conteos de participantes únicos y si existe un grupo de respaldo. No añade participantes.
- `=/>add++ snapshot <community-jid>`: solicita una instantánea cifrada y privada del inventario accesible. Debe usarse antes de la suspensión. La fase de exportación requiere llave propia independiente y disco privado.
- `=/>add++ apply`: **denegado** mientras no haya aprobación de plan, permisos, aceptación de participantes, limitación de lotes y auditoría real.

## Principios obligatorios
1. Solo usar la API autenticada de WhatsApp para los miembros de comunidades donde la cuenta vinculada es admin verificado mediante PN/LID. No confiar en nombres de grupos ni títulos elegidos por terceros.
2. Deduplicar usuarios de todos los subgrupos mediante identidad PN/LID sin inventar equivalencias. Los identificadores son información personal; cualquier respaldo debe cifrarse localmente y quedar fuera de GitHub y Vault.
3. Evitar cualquier acción de grupo hasta que el propietario apruebe un destino específico. Si WhatsApp marca la comunidad suspendida, no saltarse ese bloqueo y no intentar recuperar acceso restringido mediante otras cuentas.
4. Respetar las preferencias de privacidad de los participantes. Una persona puede no admitir incorporaciones directas: usar invitación/solicitud de ingreso en su lugar. La copia de seguridad no significa consentimiento para incorporarla a otro grupo.
5. Nombre sugerido del respaldo: nombre de la comunidad con indicación explícita de *Respaldo*, sin suplantar una comunidad ni crear una duplicada silenciosamente.
6. Registrar un plan con expiración, identidad de origen/destino, cantidad y autorización, detener ante errores, cuotas, rate limits o suspensión y no reintentar indefinidamente.
7. Si el grupo original se suspendió antes de realizar el respaldo, no garantizar que WhatsApp permita consultar miembros. Una instantánea previa no copia sesiones ni garantiza conservar usuarios.

## Estado del desarrollo
En `alpha` se registra el comando con vista previa protegida. La lectura live, el respaldo cifrado, la verificación del destino y el flujo de invitaciones requieren implementación y prueba propias. El objetivo no autoriza un agregado masivo no solicitado ni acciones para evadir sanciones.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## Descubrimiento alfa implementado

El proveedor Rust ahora responde `=/>add++` con una vista previa conectada a `Community::get_linked_groups_participants`, verificación admin por PN/LID, límites de tamaño y un grupo de respaldo sugerido. Se considera un número aproximado porque las identidades PN/LID de algunas sesiones pueden no estar totalmente resueltas. No se escriben archivos ni se ejecutan mutaciones. El comando `snapshot` responde por ahora que el respaldo cifrado no está disponible.

## Almacenamiento cifrado local (alfa)

`providers/whatsapp-rust/src/recovery.rs` define snapshot de comunidad y miembros con identidades JID/PN/LID, cifrado autenticado XChaCha20-Poly1305, clave dedicada de 256 bits, nonce aleatorio y AAD ligado al socket y dueño. Los archivos se crean con nombres aleatorios y permisos 0600 dentro de la carpeta privada del SQLite, sin modificar credenciales ni incorporar usuarios. Se añade API de lectura autenticada para una recuperación posterior y tests de integridad/key mismatch. La llave vive exclusivamente en `runtime.env` del operador, fuera de Git; copias de seguridad cifradas y llave deben resguardarse separadamente para no perderlas con un servidor temporal. La función aún debe conectarse a `=/>add++ snapshot` y validar la API real.

## Integración del callback (alpha)

El comando de snapshot está conectado al proveedor Rust. Antes de escribir se comprueba que el padre `@g.us` exista, no esté suspendido, que la propia identidad verificada de la sesión sea administradora (PN/LID) y que las identidades se obtengan por `get_linked_groups_participants`. La carpeta de sesión y la clave son privadas; se conserva el resultado ciphertext sin mostrar los JIDs al chat. Las pruebas CI aún son offline: no confirman que la API de la cuenta pueda consultar y almacenar sus miembros reales. El fichero debe respaldarse fuera del servidor temporal, junto con una copia segura e independiente de la clave. La incorporación masiva permanece sin implementar, requiere consentimiento y aprobación.

## Verificación offline tras pérdida de acceso

Se incorporó `--recovery-verify <nombre>.enc.json` al binario del proveedor. Este modo no requiere sesión conectada y descifra solamente snapshots autenticados con la clave/identidad de socket correctas. Muestra conteo sin exportar miembros. No permite una incorporación automática a grupos nuevos: primero debe existir destino, acceso autorizado y consentimiento verificable. La recuperación del roster del archivo sirve para planear invitaciones futuras sin recopilar miembros después de una suspensión.
