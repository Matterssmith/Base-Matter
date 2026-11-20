## 2026-11-19 — Política obligatoria de cifrado para capacidades protegidas

- Toda información operativa que atraviese una capacidad protegida se clasifica como credencial: cifrado autenticado en tránsito/almacenamiento, claves externas y segregación por bot/socket/recurso. Sin cifrado o autoridad real se **deniega**, nunca se permite fallback en texto claro.
- Base-Matter público expone solo contratos, identificadores y solicitudes; **no conoce rutas ni nombres de repositorios privados**, no descarga implementación interna y no debe poseer claves para descifrar material privilegiado. El host confiable autoriza y transporta ciphertext opaco.
- Regla vinculante: `docs/PROTECTED_CAPABILITY_ENCRYPTION.md`. Contrato existente aún no implementa canal operativo cifrado. HMAC aislado no proporciona confidencialidad. Los datos operativos y auditorías deben seguir cifrados; el descifrado temporal solo ocurre en el proceso autorizado.
- Corrección documental, no modifica runtime/WhatsApp ni demuestra cumplimiento criptográfico. No promover `alpha` hasta pruebas Linux/Windows, seguridad y proveedor. Bot-Matter sigue bloqueado. Ambos README y registros de continuidad revisados.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-19 — Política obligatoria Vault-D019: protección equivalente a credenciales

- **Regla permanente del ecosistema:** TODOS los activos administrados por Vault — roles, permisos, decisiones, credenciales, políticas, metadatos, evidencia, auditoría, caché, respaldos y estados — deben cifrarse/autenticarse en reposo y transmitirse por canales cifrados y autenticados. Descifrado temporal sólo en memoria confiable; claves fuera de Git, nunca textos sensibles en logs, respuestas públicas ni archivos temporales.
- Base pública: sin rutas, URLs ni dependencias a repositorios privados. El selector provider/language/system y los puentes emiten solicitudes sin credenciales; todo dato o decisión privada del servicio Vault deberá viajar por un canal autenticado y cifrado, y jamás se persistirá en claro. No cambiar el código de runtime hasta existir gateway verificado. Al recibir un resultado mínimo autorizado, no volcarlo a logs, tests ni mensajes. `alpha` solamente; beta/stable intactos.
- La norma principal se mantiene en la política privada VAULT-D019. Esta actualización es documental y no acredita gateway Vault, ejecución privada o migración de sesiones. No conceder permisos sin verificación y no afirmar pruebas funcionales.
- Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

> **Sintaxis definitiva prevista:** `on|off|inherit`. Nunca utilizar `-` como activación o herencia. Contrato documental; el puente Rust sigue sin exponer `inherit`.

## Ajuste de integración propuesto — 2026-11-19 (sin cambio Rust)

Los comandos de bot `#antilink` y los controles `=/>anti link` deben usar en el futuro semántica compartida de activación `on|off|inherit` y de consultas, pero con permisos distintos. `set` se reserva a configuración. El puente público existente continúa sin implementación privada; `inherit` requiere operación nueva de retiro de override, aún NO incorporada al tipo Rust `AntiLinkCommand`. No afirmar soporte actual. Ver `docs/ANTI_COMMAND_CONVENTIONS.md`.

# Base-Matter — Public protected capability bridge

Estado: Rust interface in alpha; authenticated private executor is NOT deployed.
This is a public developer contract, not the private implementation.

## Use from a custom bot

Import the Rust module matter_base::capabilities::anti_link. The bridge file
src/capabilities/anti_link.rs exposes Status, SetEnabled, SetAction,
AddDomain, RemoveDomain and ListDomains. It contains no anti-link rules,
private source, credentials or executable moderation. The caller provides a
ProtectedCapabilityGateway installed by a trusted host. With
UnavailableGateway the call fails closed.

An application may map its own #antilink command to anti_link::invoke.
The native =/>anti link command remains unchanged and retains the socket
owner and group rules it already has. Neither spelling confers permission.
The private service MUST verify the originating account, message and source
independently of public CommandContext fields before applying any policy.

Supported scopes: Group, Socket, GeneralDefaults. GeneralDefaults is
read-only. Group operations require an authenticated group. The executor
verifies membership and administrative capability before actual moderation.
Never grant access to another bot's sessions or resources via a shared
capability. Guest or read-only queries must not turn into mutation.

## Production boundary

The gateway must use an authenticated transport and a short-lived, scoped,
revocable authorization decision, with replay protection, expiry and audit.
The protected executor must reverify actor, bot, socket, resource and action,
enforce quotas and fail closed on missing or unavailable authority. Do not
send keys, repository credentials or private implementation to bot creators.
A private repository is not an execution service.

The current public src/anti.rs has already been published. Moving
implementations later will not erase their former public history; stage
extraction with compatibility tests and a review of what can safely remain
open source.

## Required validation

Run cargo fmt --all -- --check, cargo check --all-targets --locked,
cargo test --all-targets --locked and Linux/Windows/provider CI for the
exact commit before promotion. Remote authorization, group moderation and
live WhatsApp tests remain PENDING. Do not claim an operational protected
gateway until those integrations pass.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
