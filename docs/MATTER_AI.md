## Autorización de Aura Reed comunicada — 2026-11-18

El propietario de Matterssmith informó que Jeriel (GitHub:
https://github.com/jerielweb), creador de Aura Reed, le autorizó
usar su clave de IA para las tareas planeadas, **sin establecer
límites específicos**. Este informe ya está registrado en
Vault-Matter privado (rama matter, commit 8872f9c):
docs/AI_PERMISSION_AURA_REED.md.

**No equivale a proveedor operativo:** no hay clave provisionada,
gateway Vault activo ni licencia cifrada y verificable. La
ausencia de límites numéricos informados no es cuota infinita;
se respetan límites del servicio y controles técnicos/costes
del ecosistema. Las futuras capacidades de orientación de
menús y asistencia de programación deben registrarse por
concesión y verificarse por operación. El permiso no concede
ejecución de código, escritura GitHub, roles ni dispositivos.

# Matter AI — acceso indirecto mediante Vault-Matter

**Estado:** contrato del cliente Rust en `alpha`, sin gateway Vault
desplegado ni credenciales provisionadas. No significa que el modelo
responda actualmente en WhatsApp. Esta versión sustituye el diseño
anterior que consideraba variables de claves directas en Base-Matter.

## Distribución de responsabilidades

- `Base-Matter` identifica al dueño del socket y autoriza las
  peticiones en el puente WhatsApp firmado. No almacena, descifra ni
  transmite claves de APIs de terceros.
- `Vault-Matter` será la única entidad autorizada para guardar
  **credenciales cifradas** y llamar a los modelos después de
  comprobar el permiso concedido por el titular, la identidad del
  solicitante y las capacidades efectivas.
- El modelo externo solo recibe los datos permitidos junto al texto
  de restricciones del titular de la API.
- El futuro trabajador Vault devolverá exclusivamente una respuesta
  y un comprobante verificable de política aplicada. No entrega
  secretos, permisos transferibles ni tokens.

El contrato del permiso se encuentra en el repositorio privado
`Matterssmith/Vault-Matter`:

- `schemas/ai-provider-registration.schema.json`: proveedor y
  referencia a credencial.
- `schemas/ai-usage-grant.schema.json`: concesión expresa,
  límites, datos, cuotas, caducidad y revocación.
- `vault/ai/AI_READ_FIRST.md`: plantilla de texto que la IA debe
  recibir en sus instrucciones prioritarias *en cada llamada*.
- `docs/AI_CREDENTIALS.md`: separación de claves, privacidad
  y autorización.

**Acceso a un repositorio ajeno no autoriza usar sus claves.**
La incorporación de IA de Aura Reed/Alya, Claude, Venice, OpenAI y
futuros proveedores requiere autorización del titular, una clave
dedicada con alcance limitado y provisión fuera de GitHub.

## Contrato del cliente

`MATTER_VAULT_AI_SOCKET` debe contener la ruta absoluta de un
socket Unix privado y protegido propiedad del servicio Vault.
`MATTER_VAULT_AI_UID` indica el UID numérico esperado del servicio
Vault. Ruta y directorio deben estar protegidos con modo privado;
si falta el UID o no corresponde al propietario del socket, se deniega.
Este control inicial de propietario Unix **no sustituye** la
validación de identidad y capacidad en el servidor Vault.
El cliente envía `protocol=1`, nonce aleatorio, `capability`,
`policy_required=true`, identificador de socket/actor verificado
y texto de la consulta. Nunca envía una clave de proveedor.

**Respuesta obligatoria:** `protocol=1`, el mismo nonce,
`approved=true`, `policy_applied=true`, `grant_id`
válido, `grant_revision>0`, `capability` igual a la solicitada,
`provider` válido y `text` con tamaño limitado.
Se rechazan las respuestas de un trabajador antiguo o incompleto.

El comprobante de política **no sustituye** la autenticación del
socket local ni prueba por sí solo que la IA recibió el texto.
Vault debe implementar verificación del cliente, permisos
firmados, control de revocación, cuota, consentimiento de datos,
control del prompt, auditoría y transporte seguro antes de
habilitar producción.

## Uso en WhatsApp (cuando el gateway esté disponible)

En el chat privado autenticado del propietario:

```text
=/>matter
=/>matter status
=/>matter ask <pregunta>
=/>matter plan <tarea>
```

`matter` es asesoría de programación y no ejecuta comandos.
`console`, `eval`, `test` y `sandbox` requieren vías
independientes y permisos específicos. El acceso a una IA no
concede permisos de consola, escritura de repositorios,
moderación, gestión de roles ni control de dispositivos.

**Pendiente:** servicio real de Vault, provisión de claves
por su titular, pruebas de licencia/revocación, interacción
con el proveedor y ensayo físico de WhatsApp. No publicar
claves de Aura ni reutilizar una clave encontrada en un
repositorio, aunque sea privado.
