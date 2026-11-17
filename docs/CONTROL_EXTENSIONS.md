# Extensiones de control global — Base-Matter

Registro opcional para comandos escritos en Rust por la aplicación autorizada.

- Únicamente se aceptan prefijos canónicos =/> para extensiones.
- ControlDispatcher verifica al propietario actual del socket antes de llamar.
- La sesión puede estar apagada y no necesita vinculación a Bot-Matter.
- Los nombres on/off/status/set/unset/ping están reservados y no admiten alias que los reemplacen.
- Los manejadores se instalan por código Rust confiable; no se evalúa JavaScript, shell ni código no confiable.
- Pánicos, errores y salidas sobredimensionadas se bloquean sin revelar detalles.
- Cada comando con efectos debe aportar sus propias reglas de permisos y auditoría.
- La respuesta Extension solo puede mostrarse a su solicitante autenticado.
- La conexión elegida de WhatsApp permanece whatsapp-rust, con código de vinculación únicamente.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
