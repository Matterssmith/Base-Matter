# Base-Matter — Contrato público con el servicio Vault

Estado: límite de integración documentado. El motor privado de cifrado local
ya cuenta con pruebas offline, pero todavía NO existe gateway de red
autenticado ni concesiones operativas para Base.

## Dependencia correcta

Base-Matter NO depende de Bot-Matter. Los dos son consumidores hermanos
de una interfaz de servicio protegida y autenticada. Base solo publica
contratos, selectores y puentes de capacidades, como el puente anti_link.rs.

La clave maestra de cifrado nunca se compila ni se almacena en Base,
Bot, paquetes, Git, variables de los bots o plantillas. Un runtime
protegido de Vault la obtiene de una fuente externa (KMS/HSM o archivo
protegido de cuenta de servicio durante desarrollo).

La solicitud pública únicamente indica la capacidad, operación y
argumentos validados; NO garantiza permisos ni identidad. Un adaptador
instalado por el host aportará un canal cifrado y mutuamente autenticado,
que comprobará una afirmación de origen firmada y el destino verificado.

## Flujo futuro

1. Proveedor autentica mensaje, cuenta, socket y chat.
2. Base clasifica y valida intención, sin otorgar permiso por prefijo.
3. Host establece canal protegido a la autoridad Vault.
4. Vault verifica identidad de servicio específica, actor, bot,
   socket, recurso, acción, caducidad y revocación.
5. Vault consulta o modifica solo el registro cifrado autorizado.
6. El ejecutor privado realiza efectos sobre la plataforma únicamente
   tras revalidar permisos y devolver un resultado mínimo y cifrado.
7. No se entregan claves, repositorios, tokens de GitHub, ni acceso
   a archivos de otro bot a un consumidor.

La identidad que ejecuta Base no puede representar la de Bot,
ni intercambiar permisos entre tenants/sockets. Los parámetros de
provider/language/system del instalador offline NO autorizan Vault.

## Estado comprobable y pendiente

Implementado: contrato de puente protegido con UnavailableGateway
(fail closed); núcleo privado Rust de almacenamiento XChaCha20-Poly1305,
AAD por recurso y bot/socket, indexación opaca y pruebas offline
en el servicio custodio.

Pendiente: conexión mTLS/IPC cifrada, autenticación y autorización remota,
auditoría cifrada, revocación/replay, locks transaccionales y replicación
cifrada. No existe un enlace de control Base -> Vault en producción.

No conectar el núcleo criptográfico privado como dependencia Cargo
ni como fuente descargada de la biblioteca pública. No crear
fallback en SQLite o JSON plaintext. Hasta completar la autoridad,
los comandos privilegiados siguen rechazados. Bot-Matter bloqueado.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
