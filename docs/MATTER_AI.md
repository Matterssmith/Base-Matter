# Matter AI — asistente de programación opcional (alpha)

**Alcance:** el comando `matter` es un asistente de programación por
mensajes, NO es sinónimo de `console`, `eval` ni `sandbox`.
No ejecuta órdenes del servidor, edita archivos ni publica commits. Una
respuesta del modelo es una propuesta, no un cambio aplicado.

## Origen de los proveedores

Se inspeccionó `jbeckforddev/Aura-Bot-2.0`, ramas `master` y
`test`, incluidos `config.ts`, `cmd/AI/chatGPT.ts` y
`cmd/AI/DeepSeek.ts`. No se identificaron integraciones directas de
Venice/Claude ni una clave reutilizable autorizada. **No se importan
credenciales del bot externo.**

El controlador Rust consulta los proveedores en orden:

| Orden | Proveedor | Variables obligatorias | Endpoint |
|---|---|---|---|
| 1 | Claude | `ANTHROPIC_API_KEY` + `MATTER_CLAUDE_MODEL` | `https://api.anthropic.com/v1/messages` |
| 2 | Venice | `VENICE_API_KEY` + `MATTER_VENICE_MODEL` | `https://api.venice.ai/api/v1/chat/completions` |
| 3 | OpenAI | `OPENAI_API_KEY` + `MATTER_OPENAI_MODEL` | `https://api.openai.com/v1/responses` |

Ninguno se activa sin sus **dos** variables. Los IDs de modelo no se
inventan ni se fijan en el repositorio; deben corresponder a modelos
reales de la cuenta contratada. No copiar claves al chat, commits,
README, configuración de bot ni Vault Git.

La solicitud HTTP se realiza con `curl` instalado en
`/usr/bin/curl` (Debian), con URL HTTPS fija, sin redirecciones,
tiempo máximo de 25 segundos por proveedor y salida limitada.
La configuración con las credenciales pasa por entrada estándar,
no por argumentos de proceso. Se descartan errores de transporte
con detalles sensibles; no se escriben trazas con contenido de la clave.

## Comandos

En el **chat privado propio** del socket WhatsApp, después de activar
la política del bot para ese chat:

- `=/>matter` o `=/>matter help`: ayuda.
- `=/>matter status`: proveedores configurados; no consulta servicios
  remotos ni imprime claves.
- `=/>matter ask <pregunta>`: consulta de programación.
- `=/>matter plan <tarea>`: plan de implementación y pruebas.

Los mensajes de terceros no pasan el filtro de proveedor; los
comandos del propietario verifican PN/LID, firma HMAC, nonces
antirreplay y contexto de chat en el puente existente.
`matter` **no** se permite en grupos. La invocación completa
puede bloquear brevemente el controlador IPC mientras espera
el proveedor remoto; todavía no es un agente de ejecución
asíncrona ni una sesión interactiva tipo Codex. No se puede
prometer soporte real hasta probar una clave válida y el
mensaje recibido en WhatsApp.

## Separación con el resto del sistema

- `console` y `eval`, cuando se integren, serán explícitamente
  directos (no aislados) y tendrán su propio bloqueo por defecto.
- `sandbox` y `sandbox test` son operaciones distintas con
  aislamiento por proceso y validación de archivos.
- `test` se diseñó como prueba directa de código/archivos; los
  adjuntos respondidos requieren conexión de descarga verificada.
- `matter` NO actúa como alias para esos comandos.
- La autoridad de `Vault-Matter` no se infiere de una API de IA.
  La conexión de roles requiere un lector autenticado del Vault;
  ninguna variable de IA crea rangos, acceso Git o privilegios.

No se ha promovido a `beta` ni `stable` hasta que el CI del
commit y la validación de API estén disponibles.
