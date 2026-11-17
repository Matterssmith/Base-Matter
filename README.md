# ꧁✪⃢.✪ 𝐌𝖆𝖙𝖙𝖊𝖗𝖘𝖘𝖒𝖎𝖙𝖍۞₪»̶ ༄☾︎𝐌𝐒☽︎༄

<div align="center">

<img
src="https://capsule-render.vercel.app/api?type=waving&color=gradient&customColorList=12,19,24&height=240&section=header&text=𝐌𝖆𝖙𝖙𝖊𝖗%20𝐁𝖆𝖘𝖊&desc=The%20Core%20Architecture&fontSize=52&fontColor=FFFFFF&animation=fadeIn&fontAlignY=38&descAlignY=58&descSize=20"
width="100%"
/>

# Base-Matter (antes Matter-Base)

### The Core Architecture.

**Powered by Matterssmith.**  
**Engineered for the Future.**

*A reusable Rust foundation for the Matterssmith ecosystem.*

</div>

---

## Project status

Base-Matter (repositorio actualmente llamado Matter-Base) es una **biblioteca de desarrollo incompleta**. The current repository
contains a runtime/context foundation, in-memory event and service
registries, translation lookup, a persistent bot identity registry, an
optionally persistent shared-user registry with explicitly authorized external
identity links and bot-scoped user data, a persistent legal-consent store, and
cryptographic key/envelope and provider-authentication components, a command
router, and persistent socket activation policies. The bot
capabilities are only partially implemented: a separate Rust WhatsApp
provider and a loopback HMAC control process exist in beta with offline tests,
but actual phone pairing, live revocation, production Vault, plugin isolation,
Telegram and Discord adapters are not release-qualified. See
[the development status](docs/DEVELOPMENT_STATUS.md) for the
verified stage assessment.

## Responsibilities and boundaries

Matter-Base is intended to host reusable Rust capabilities consumed through
public APIs. It must remain independent of Matter-Bot and platform adapters.
This repository includes a separate `whatsapp-rust` provider executable and
a loopback-only Rust control receiver (both development/beta, not live verified).
The core remains independent of provider-specific libraries. Telegram and
Discord connection drivers are not implemented. Plugin contracts are present,
but the real execution sandbox is not. Scoped role authorization is opt-in;
full provider admission and audited cross-store lifecycle are not yet complete.

Bot registration requires an owner ID. Consumers must derive actor IDs from
authenticated requests before calling owner-management APIs. Owners can manage
co-owners and transfer/delete bots; co-owners can manage bot-scoped operations
but cannot change ownership or delete. Each bot supports at most 256
co-owners; additions beyond the limit are rejected in memory and SQLite.
Legacy unowned records require an
injected `OwnerRecoveryAuthorizer` before ownership can be claimed. Ownership
changes are written to the registry audit history.
Reading a bot's audit history requires the current owner ID and is checked by
the identity store when records are read. Access to the retained audit history
of a deleted bot is not exposed through the current registry API.

The security module defines provider and key-provider contracts. It does not
ship a production Matter-Vault integration or store platform sessions or
credentials. The tests use local test providers and do not verify a live Vault
service. [Repository contract evidence](docs/vault-integration.md) is available
separately and does not establish a live integration.

## Implemented components

| Component | Current behavior | State |
| --- | --- | --- |
| `core::config` | Default application name, environment, debug flag, and locale | Partial; no validation/loading |
| `core::runtime` | Opens legal storage, builds a context, owns module initialize/start/shutdown lifecycle; can inject bot and user registries | Partial; default registries are in memory, no built-in modules are registered |
| `core::context` | Shares event bus, typed service registry, legal store, bot registry, and user registry | Implemented at this scope |
| `bots` | Family/release/instance identity, unique display names, in-memory or SQLite-backed registration and recovery with atomic schema versioning | Implemented owner/co-owner enforcement and audited ownership changes; general roles remain pending |
| `users` | Global application user IDs, explicit unique external identity links, and per-bot/user key-value data in memory or SQLite | Implemented with an injected `UserAuthorizer`; the application authenticates principals and enforces ownership and bot scope |
| `commands` | Prefixes, canonical aliases, authorization, middleware and handlers | Optional SQLite attempt/outcome audit; socket gating via ControlDispatcher; no platform parsing |
| `activation` | SQLite owner-scoped power, group/socket overrides, persisted global fallback and atomic audit | Trusted provisioning, durable bot binding, owner transfer/recovery; no network connections |
| `events` | Synchronous named callbacks; listeners are copied before invocation so callbacks run outside the registry lock | Basic; in-memory and not durable; a callback panic propagates from `publish`; `publish_checked` catches unwinding panics per listener, continues dispatch, and returns an error |
| `services` | In-memory type-indexed service registration and retrieval | Basic; no lifecycle management |
| `i18n` | In-memory translation catalogs and key fallback | Basic; no bundled catalogs or locale validation |
| `legal` | SQLite documents, user IDs, immutable decision history, and access checks | Implemented at this scope |
| `security` | Authenticated encryption envelopes, key management, provider identity/policy, replay-aware assertions, binding contracts | Implemented contracts/primitives; external integration unverified |
| `modules` | `CoreModule` plus ordered initialize/start and reverse stop with failure cleanup; unwinding callback panics become errors | Implemented and coordinated by `MatterRuntime`; consumers register their own modules |

These states describe this repository's present behavior, not the full planned
Matter-Base platform.

## Repository layout

```text
src/
  activation.rs  Persistent socket activation policy and owner checks
  command_audit.rs  Persistent command attempts/outcomes and owner-scoped history
  commands.rs    Universal command routing
  control.rs     Control-prefix interception and socket-gated dispatch
  control_ipc.rs Loopback-only authenticated control service
  whatsapp_bridge.rs Signed WhatsApp owner-control bridge
  platforms.rs   Shared platform driver/transport registries
  bots/       Bot identity contracts and in-memory/SQLite registry
  users/      Shared users, explicit external identity links, scoped user data
  core/       Configuration, shared context, logging, runtime
  events/     In-memory event bus
  i18n/       Translation catalog lookup
  legal/      SQLite legal-document and consent store
  modules/    CoreModule contract
  security/   Encryption, key management, provider authentication/contracts
  services/   Typed in-memory service registry
tests/        Integration-style tests for the implemented contracts
docs/         Architecture and continuity documentation
providers/whatsapp-rust/  Standalone Rust pairing-code provider
```

There are two tracked README files: this root `README.md` and
[`providers/whatsapp-rust/README.md`](providers/whatsapp-rust/README.md).
Both require review before every development commit.

Clippy's first strict CI run identified three pre-existing lint failures in legal-document row typing, ciphertext size arithmetic, and user schema-range validation. The follow-up changes preserve public APIs and require a passing CI run before they are considered verified.

## Persistent roles and resource authorization

The optional role store, in [src/authorization.rs](src/authorization.rs),
provides SQLite-backed bot-scope role definitions, single-parent inheritance,
owner-only administration, bounded memberships and rules, explicit deny
precedence, default denial, and immutable audit records. Provisioning an
owner/scope pair is a **trusted application-only operation**. Actor IDs must
come from authenticated ingress; passing a claimed ID is not authentication.
Scope administration grants no implicit command execution, socket power or
access to another scope.

[RoleCommandAuthorizer](src/authorization.rs) can be injected into
CommandRouter; it checks the canonical name as resource
`command:<name>` with action `execute`. A successful authorization is
only a decision at admission time. Callers must separately enforce the
socket/activation gate, consent, and any platform security policy. The store
does not persist sessions, platform credentials or encryption keys.

## Session metadata and transport boundary

The [session module](src/sessions.rs) provides a SQLite-backed lifecycle for
socket connection **metadata**: owner, platform, connection state, optimistic
revision and immutable audit. It does not store login tokens, session files,
private platform credentials, WhatsApp keys or a live network connection.
A trusted application provisions session metadata and drives valid lifecycle
transitions. Socket power and bot activation are independent.

MessageService accepts a required application MessageAdmission policy and
a TransportDriver supplied by a platform adapter. Its sender refuses
disconnected sessions, malformed/oversized messages and denied requests.
The application must enforce authenticated actor, consent, socket activation,
resource scope and group policy; this library does not infer those permissions.
A driver acknowledgement is not a delivery guarantee, and in-flight sends
can race with connection changes. No automatic retry or exactly-once
semantics are provided. A built-in ControlPlaneMessageAdmission can combine the persisted socket
activation/bot binding snapshot with role rules and an additional mandatory
application policy. Group destinations must be explicitly identified using
OutboundMessage.group_id; private destinations use None. This still does not
make separate stores or network effects atomic.
See [session contracts](docs/sessions.md).

## Reserved role-control commands

With ControlDispatcher::with_roles, =, /> and =/> support "set role
USER_ID ROLE_ID" and "unset role USER_ID ROLE_ID" for the bot bound to the
receiving socket. The socket owner must remain the bot and role-scope owner.
Role controls work even when the socket is off; the ordinary router cannot
intercept them. Missing role configuration and foreign owners are denied.
The host must serialize bot transfer with cross-store role administration.


## Encrypted control-plane synchronization

EncryptedControlOutbox holds only authenticated encrypted envelopes and a keyed
change fingerprint. It can export a consistent, authorized RoleStore snapshot,
encrypt it with the active KeyManager key and queue it durably. An application
must install a trusted EncryptedEnvelopeSink (for example a correctly scoped
GitHub App writing to the private Matter-Vault repository). It MUST use
compare-and-swap revisions, deduplicate envelope IDs and never write plaintext.
The same content under the same key is not queued again; key rotation queues
fresh ciphertext. Failed publication keeps the encrypted record pending.

This is **not** a live GitHub/Matter-Vault adapter. No GitHub credential,
private key, master key or external session is stored here. An optional EncryptedEnvelopeReader supports lost-acknowledgment recovery:
a pending record is marked published only if the returned remote envelope
matches every field of the locally queued ciphertext. Conflicting remote
records remain pending; they are never overwritten automatically. Multiple
concurrent publishers, full restore/import and actual authenticated GitHub
commits remain integration requirements. See
[Vault integration](docs/vault-integration.md).

## Trusted plugin manifests and sandbox contracts

The [plugin host](src/plugins.rs) accepts bounded JSON manifests for
Rust/C++/Go/Python/Node workers. It rejects path traversal, unsupported
runtimes, duplicate IDs/commands and oversized input. Installation requires
an explicit PluginTrust provider to authenticate publishers; execution checks
the policy again and goes through an injected PluginSandbox. Without an
external sandbox, DenyPluginSandbox refuses all plugin execution. The host
does **not** execute an entrypoint directly or expose system credentials.

Loading manifests from a directory prevalidates the batch, but registration
into the command router is not atomic against concurrent installations.
No OS-level sandbox backend, signed artifact verification service or live
cross-language process is supplied. No in-process mock should be treated as
real isolation. See [plugin security contract](docs/plugins.md).

## External connection lifecycle

ConnectionService orchestrates an authenticated session owner, an
application-provided ConnectionAdmission policy, a SessionCredentialSource
returning zeroizing secret bytes and a separate ConnectionDriver implemented
by a platform adapter. Failures fault the session metadata and are not retried
automatically. The service exposes guarded connect/disconnect, live health
checks and explicit recovery for uncertain states. **No real WhatsApp,
Telegram or Discord adapter is bundled.**

Credential bytes are transient and never persisted in Matter-Base; the
external source is responsible for authentication, storage, rotation and
retrieval. A connected database state cannot prove the external provider is
online. Network operations across processes need provider-side generation
fencing and revocation. See [connections and sessions](docs/sessions.md).


## Readiness gates, licensing and tracked-file hygiene

Matter-Base is not yet a production-ready release. The exact remaining
requirements are maintained in [release readiness](docs/RELEASE_READINESS.md).
The [MIT license](LICENSE) is included for this repository's own code;
third-party provider imports must retain their original notices and licenses.
The beta CI also verifies all tracked README links/credits, rejects
sensitive private-key/session filenames and runs a second Rust job on
Windows. Passing Linux and Windows CI is not a Termux or provider test. Windows
also checks that SQLite-backed test files are closed before cleanup.


## Authenticated platform ingress

InboundGateway receives untrusted provider text/events but does not accept
claimed actor IDs from those messages. A required external InboundAuthenticator
verifies the provider connection/webhook and derives the authenticated socket,
sender, source and group. A separate InboundAdmission enforces legal consent
and platform policy before routing to ControlDispatcher. A SQLite-backed,
HMAC-keyed InboundReplayStore refuses duplicate event IDs across connections
and keeps only encrypted-style keyed fingerprints, never raw provider IDs.

Replay claims occur before handler effects. A crash after claiming can leave
an uncertain result; blind retry is prohibited. Replay binding keys must be
provisioned externally and shared across processes; key rotation and pruning
require coordinated retention. **No live WhatsApp/Telegram/Discord identity
authenticator ships with Base.** See [inbound contracts](docs/ingress.md).

## Vinculación WhatsApp y control global — contrato corregido

Se reconoce la prueba anterior de conexión a WhatsApp y `ping` en el
desarrollo histórico. La rama publicada actualmente no contiene el antiguo
adaptador ejecutable; conservar su historial no equivale a comprobar una
conexión real hoy.

`PairingCodeService` añade un contrato en Base-Matter para **solicitar
únicamente un código de vinculación**, con autorización del propietario,
número internacional de 8–15 dígitos, código temporal redactado y sin
persistencia en SQLite. **No implementa QR, no registra un comando ping y
no abre por sí mismo una conexión WhatsApp.** El proveedor Rust independiente está implementado en
`providers/whatsapp-rust`, pero la conexión real y recuperación en teléfono
siguen pendientes de validación; no se deben borrar sesiones.
No se modifica ni se inicia Bot-Matter.

`ControlDispatcher` procesa el prefijo global canónico `=/>` antes del
router del bot. Los prefijos `=` y `/>` permanecen como alias de
compatibilidad. Ejemplos: `=/>on`, `=/>off`,
`=/>set role USER_ID ROLE_ID`. El dueño del socket sigue siendo el único
autorizado para manipularlo, y el control funciona con el bot apagado.
No se transfiere poder al propietario de otros sockets.

Todos los repositorios de la organización pertenecen al ecosistema Matter;
el futuro `Bot-Matter` consumirá contratos de sus componentes **solo
cuando Base-Matter termine**. Véase [pairing code](docs/PAIRING_CODE.md).

## WhatsApp owner control — authenticated bridge v1 (beta)

The opt-in `WhatsAppControlBridge` validates a short-lived HMAC-SHA256
provider frame bound to a **specific socket ID, owner ID, owner JID, private
chat, event ID and message body**. It requires the WhatsApp Web provider to
authenticate the original `fromMe` event. Only canonical `=\/>` control
verbs are admitted; the old ping and bot routing are unreachable.

The library accepts a signed JSON frame and dispatches through
ControlDispatcher. The separate `whatsapp-matter-control` Rust executable
listens exclusively on 127.0.0.1, while the independent
`providers/whatsapp-rust` process owns the platform session.
Neither process by itself proves a live WhatsApp connection.
The transport must keep its bridge key local, reject group/nonowner messages
and use the shared replay guard. The bridge rejects
signatures, timestamps beyond two minutes, duplicate event IDs and attempts
to control another owner's socket. The protocol is documented in
[WhatsApp bridge v1](docs/WHATSAPP_BRIDGE.md).


## Continuous verification

GitHub Actions workflow [Rust verification](.github/workflows/rust-ci.yml)
is configured for pushes and pull requests targeting `matter-beta`, and can
also be dispatched manually. It runs formatting, all-target compilation,
unit/integration and documentation tests, Clippy with warnings denied, and a
commit whitespace check. Workflow presence does **not** establish that a run
has passed; inspect the latest run and its logs before treating any commit
as verified. The opt-in external Matter-Vault repository test is not run by
default and still requires a separately available checkout.

## Requirements and build

Install a Rust toolchain with Cargo, then run from the repository root:

```sh
cargo fmt --all
cargo check --all-targets
cargo test
git diff --check
```

The SQLite dependency uses its bundled SQLite feature. No Node.js `npm start`
workflow or JavaScript runtime is required for the Rust library. The legacy
installer scripts remain in the repository but are not the build or test
workflow for the current Cargo crate.

## Minimal library use

```rust,no_run
use matter_base::{CoreConfig, MatterRuntime};

fn main() -> anyhow::Result<()> {
    // Use a path owned by the application. The legal database contains
    // consent records; do not put secrets or platform sessions in it.
    let runtime = MatterRuntime::open(
        CoreConfig::default(),
        "data/legal/registry.db",
    )?;
    runtime.initialize()?;
    runtime.start()?;
    runtime.shutdown()?;
    Ok(())
}
```

The default `MatterRuntime::new` path is `data/legal/registry.db`. Tests and
applications that manage storage separately can provide a `LegalStore` with
`MatterRuntime::with_legal_store`. Use `open_with_bot_registry` with
`BotRegistry::open_sqlite` when bot identities must survive restarts. Use
`open_with_registries` with `UserRegistry::open_sqlite` to persist shared users,
explicit external identity links, and bot-scoped user data. User operations need
a `UserPrincipal` backed by the consuming application's `UserAuthorizer`; never
link platform identities by matching names or IDs automatically. Keep secrets,
platform credentials, and sessions out of the user data store.

## Security and persistence notes

- Legal documents and consent decisions use SQLite; document registration and
  consent validation/writes run in transactions. Decision history is append-only
  through the store's database triggers. Schema initialization and component
  version registration are transactional; unversioned existing tables and their
  data are preserved, failed DDL rolls back, and unsupported future versions
  are rejected without rewriting the database.
- Legal access checks evaluate the user and required documents in one SQLite
  statement, so the result uses one database snapshot.
- Consent writes, legal-user lookups/history, document administration, and
  access checks require a `LegalPrincipal` backed by the consuming
  application's `LegalAuthorizer`. The application must authenticate the
  principal and authorize the requested action/resource; Matter-Base does not
  authenticate platform identities; the shared user directory is provided by `users`.
  Authorization failures are returned as a generic denial without forwarding
  policy-provider diagnostics.
- The standard runtime persists only the legal store. Bot registrations and
  shared users have independent SQLite stores that must be explicitly injected
  to survive restarts. Socket activation policies have a separate SQLite store in `activation`.
  Network connections, configuration, events, and service metadata do not have
  persistence implementations here.
- Shared user data is a generic, bounded text value store. The consuming
  application must reject unauthorized owners and unknown bot instance IDs in
  its `UserAuthorizer`; the user store does not itself query the bot registry.
  Do not put credentials, sessions, secrets, or sensitive personal data in it.
- Envelope encryption validates metadata, uses the existing
  XChaCha20-Poly1305 format, and caps plaintext/ciphertext at 16 MiB before
  encryption or Base64 decoding. Keep compatibility when changing that contract.
- Key-provider and binding-store interfaces are injection points. A live,
  production Vault connection has not been verified by this repository.
- Authenticated key loading rechecks the Vault identity when the key set is
  staged; a mismatch is rejected before replacing the current key manager
  contents. Authenticated key reads also reject a key manager scoped to a
  different Vault.
- `ProviderAuthenticator::new` uses a bounded in-memory replay store.
  Applications can inject `SqliteProviderAssertionReplayStore` through
  `new_with_replay_store` to reject assertion replays across verifier restarts
  and independent database connections. Expired rows are cleaned across scopes;
  both per-scope and global capacity are bounded. It is not a distributed store.
- Authenticated provider assertions are limited to 16 KiB, and a key provider
  must return 1–4,096 key IDs per load. Empty and oversized lists fail closed,
  preserving the current key set.
- The key manager caps its total key count at 4,096 for provider loads and
  manual insertion/rotation alike. Capacity errors leave existing keys intact.
- Errors returned by injected key and binding providers are replaced with
  operation-level messages before they cross the integration boundary; this
  prevents provider-supplied diagnostics from exposing secrets through logs.
- Never add platform credentials, WhatsApp sessions, master keys, or private
  provider material to source, documentation, or logs.

See [architecture](docs/architecture.md), [services](docs/services.md),
[internationalization](docs/i18n.md), and [security module source](src/security/)
for component-level details.

## Bot ownership

```rust,no_run
use matter_base::{BotAction, BotRegistry};

fn register_bot(bots: &BotRegistry, authenticated_owner_id: &str) -> anyhow::Result<()> {
    let bot = bots.register("family-key", "release-key", "matter", authenticated_owner_id)?;
    bots.authorize(bot.instance_id(), authenticated_owner_id, BotAction::Manage)?;
    let _owned_bot = bots.get(bot.instance_id(), authenticated_owner_id)?;
    let _owned_bots = bots.list(authenticated_owner_id)?;
    let _audit = bots.audit_history(bot.instance_id(), authenticated_owner_id)?;
    Ok(())
}
```

Pass an owner ID derived from the application's authenticated request. Do not
accept it directly from an untrusted request field. `get` and `list` also
require that authenticated actor ID and return only bots owned by or shared
with that actor; inaccessible instances are returned as absent. Legacy
unowned records must be claimed through the recovery authorization flow before
they become visible to a regular owner query.

## Universal command routing

`CommandRouter` matches configured prefixes, resolves case-insensitive command
names and aliases, and passes a platform-neutral invocation to an injected
authorization policy, middleware, and handler. Authorization runs before
middleware or handler code. Message size, command count, aliases, middleware,
and argument count are bounded. Arguments are split on whitespace; adapters
must handle platform-specific quoting and message parsing.

`CommandRouter::new_with_audit` injects a `CommandAuditStore` for durable
recognized-command attempts and outcomes. The default `new` constructor remains
unaudited. `ControlDispatcher` adds socket activation checks before normal
command routing; power changes use the separate activation audit.
The injected authorizer must enforce authenticated identity, bot scope,
permissions, and operational activation before dispatch.

```rust,no_run
use anyhow::Result;
use matter_base::{CommandAuthorizer, CommandInvocation, CommandRouter};
use std::sync::Arc;

struct Policy;

impl CommandAuthorizer for Policy {
    fn authorize(&self, invocation: &CommandInvocation) -> Result<()> {
        if invocation.bot_instance_id == "known-bot" {
            Ok(())
        } else {
            anyhow::bail!("command denied")
        }
    }
}

fn make_router() -> Result<CommandRouter> {
    let router = CommandRouter::new(vec!["#".to_owned()], Arc::new(Policy))?;
    router.register("help", &["h".to_owned()], |invocation| {
        Ok(format!("{} {}", invocation.command, invocation.args.join(" ")))
    })?;
    Ok(router)
}
```

For persistent command auditing, create an application-owned SQLite database
and pass `Arc::new(CommandAuditStore::open(path)?)` to
`CommandRouter::new_with_audit(prefixes, authorizer, audit)`. Attempts persist
before authorization/middleware/handlers run. Audit write failures return an
error; an outcome-write failure cannot undo handler effects and leaves a
`Pending` attempt. Never automatically retry based only on that status.
`audit.history(bot_id, authenticated_owner, &bots, after_id, limit)` returns
owner-scoped pages of at most 1,000 records. Arguments, responses, message bodies
and callback diagnostics are omitted. See [command audit contracts](docs/command-audit.md)
for coverage, recovery and storage boundaries.

## Socket activation policies

`activation::SocketActivationStore` persists socket ownership, operational power,
and group/socket overrides with atomic audit writes. New sockets start powered
off. When powered on, resolution uses Group > Socket > trusted global default >
deny. Only the stored owner can change policies or read its audit history.
The application must authenticate provisioning and actors; this store does not
authenticate platform sessions. `bind_bot` establishes a durable bot association;
`ControlDispatcher` intercepts `=on`, `=off` (also `/>` and `=/>`) and gates
normal commands. Explicit socket targets still require the target owner.
Socket power controls work before bot binding and remain available to the socket
owner after bot ownership changes; normal commands still require a valid binding
and current bot authority.

For durable global policy, trusted setup calls `provision_global(admin_id)` and
the authenticated administrator calls `set_global_policy(actor, Some(true))`,
`Some(false)` or `None` (remove fallback). Construct the dispatcher with
`ControlDispatcher::new_with_persistent_policy` to observe committed changes
on subsequent admissions. Global administration grants no socket power rights;
group/socket overrides still take precedence and power-off always denies.
The existing `new(..., global_default)` constructor retains a static fallback.
Socket owners can call `transfer_owner(socket, actor, new_owner)` or
`release_binding(socket, actor)` to hand over a socket or recover a stale bot
association. Both reset power/local overrides and release the binding in the
same transaction as their audit; the new binding and power-on must be explicit.
Normal admission reads owner, binding and policy together to preserve isolation.
See [activation contracts](docs/activation.md).

## Promoción estable y pruebas WhatsApp — 17 nov 2026

La rama `matter` ya recibió el PR #3 con **solo** endurecimiento verificado
de seguridad, compilación y pruebas SQLite/Windows. Su commit
`e509cc4f8c53eafbb1233359bc2ae63e0dde7f48` pasó la
[CI estable](https://github.com/Matterssmith/Matter-Base/actions/runs/44139981479)
en Ubuntu y Windows. No se trasladaron los módulos de integración incompletos.

El proveedor elegido es **whatsapp-rust**, recuperable desde el commit histórico
[3c36d97bb](https://github.com/Matterssmith/Matter-Base/blob/3c36d97bb/src/providers/whatsapp/mod.rs).
La vinculación por código y la prueba anterior de ping pertenecen al historial
del desarrollo Rust. No se han repetido sobre la revisión actual de Base-Matter.

El repositorio Matter-Baileys conserva un experimento separado, **congelado y
no seleccionado** para este proyecto. No se usará para sustituir la integración
Rust ni se desarrollará Bot-Matter antes de completar Base. En la beta actual
están verificados los contratos Rust de pairing y el control global, pero no
un nuevo emparejamiento real de WhatsApp.

## Global socket status without Bot-Matter

The reserved `=/>status [SOCKET_ID]` control now returns the
authenticated socket owner's current policy decision and bound bot identifier
from one consistent database snapshot, even while the bot is powered off.
The response is an application-level `DispatchResult::Status`:
integrations must render it only to the authenticated requesting owner.
It is not a network connection-health probe. It never starts Bot-Matter,
invokes the ordinary command router, or creates a WhatsApp ping handler.
Other users and owners of different sockets are denied.

## Roadmap boundary

Scoped roles, persistent session metadata, guarded messaging contracts and
plugin validation exist and have passed CI. Remaining readiness work includes
complete authenticated Vault synchronization/recovery, externally verified
provider connections, real sandbox isolation, cross-store owner handover
coordination, and full release and security audits. Matter-Bot must not be built on
this crate until those core contracts are implemented, integrated, documented,
and tested.

## Credits

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## Nombres del ecosistema — Matter al final

La convención solicitada es `Componente-Matter`: `Base-Matter`,
`Vault-Matter`, `Bot-Matter`, `WhatsApp-Matter`,
`Telegram-Matter`, `Discord-Matter` y equivalentes. El inventario
de los 22 repositorios y las comprobaciones previas a su renombrado
están en [ecosystem rename](docs/ECOSYSTEM_RENAME.md).

Los identificadores Rust `matter-base` / `matter_base` se mantienen
temporalmente por compatibilidad. La renominación de los repositorios de
GitHub requiere una operación administrativa aparte y no se ha ejecutado.

## Ecosystem names and provider layout

The requested target repositories are **WhatsApp-Matter** (currently
Matter-Baileys) and **Telegram-Matter** (currently Matter-Telegraf).
El protocolo para WhatsApp exige **solo código de vinculación**, sin QR.
El antiguo ping fue una prueba temporal, no el bot global. La rama beta del
repositorio de proveedor conserva un prototipo no apto para este flujo y queda
congelada mientras se termina Base-Matter; no constituye prueba de conexión.
El control global usa `=/>` desde Base, antes de cualquier comando del bot.
Administrative repository renames and full provider imports remain outstanding. Provider methods belong in
separate directories such as `providers/baileys` and `providers/telegraf`, using
verified official upstream revisions and preserved licenses/history. Base and
Vault readiness remains the immediate priority. See [repository instructions](AGENTS.md).

## Proveedor de WhatsApp: Rust exclusivamente

El proveedor previamente probado era **`whatsapp-rust`** con `PairCodeOptions` y SQLite, recuperable desde el commit histórico `3c36d97bb`. Esta es la implementación que se debe continuar, sin sustituirla por Node.js, Baileys o un bot nuevo. La vinculación es exclusivamente por código, sin QR.

El archivo `src/whatsapp_bridge.rs` es una frontera de verificación escrita en Rust; **no es un adaptador Node**, no establece conexión y no debe confundirse con una prueba real de WhatsApp. El antiguo `ping` no forma parte de los comandos permanentes. El control de Base usa `=/>` con las autorizaciones del propietario del socket.

Los códigos deben conservar los créditos exigidos en `AGENTS.md` y sus pruebas deben verificar la separación entre encendido operativo y política de ejecución.

## Extensiones del control global en Rust

Base-Matter contiene ControlExtensionRegistry, que admite comandos adicionales bajo el prefijo canónico =/> y alias declarados por un host Rust confiable. El núcleo comprueba al propietario del socket antes de ejecutar la extensión; los comandos continúan independientes de Bot-Matter y están disponibles incluso con el bot apagado. Los verbos administrativos on/off/status/set/unset y el ping histórico son reservados y no pueden sobrescribirse. Los prefijos antiguos = y /> solo mantienen los controles incorporados.

Los controladores de extensiones deben incluir comprobaciones adicionales de permisos y auditoría donde sus efectos lo requieran. Los fallos no exponen datos internos; los argumentos y resultados están acotados. Consulte docs/CONTROL_EXTENSIONS.md. Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## Ingreso nativo de WhatsApp en Rust

El modulo WhatsAppNativeIngress acepta eventos autenticados exclusivamente dentro del proceso de whatsapp-rust, comprueba el JID privado propio, from_me, ausencia de grupos, expiracion y replay persistente antes de ejecutar controles canónicos =/>. No activa Bot-Matter, no incluye QR ni registra un ping permanente. El proveedor externo real, el emparejamiento y las pruebas contra un telefono siguen pendientes. Ver docs/WHATSAPP_NATIVE.md.

## Proveedor Rust ejecutable (pruebas aisladas)

El directorio providers/whatsapp-rust contiene un binario Rust real que integra whatsapp-rust 0.7.0 con la autorización, replay y control de Base-Matter, sin iniciar Bot-Matter, Node ni mostrar QR. Se requiere provisionar el socket desde el host seguro y utilizar un archivo SQLite privado, con bloqueo de sesión. Ver providers/whatsapp-rust/README.md para variables y pruebas. La vinculación de un teléfono real todavía no ha sido validada sobre esta versión, por lo que la promoción live a matter continúa bloqueada.

## Separación de procesos Rust para WhatsApp

El proveedor whatsapp-rust 0.7.0 y el núcleo actual utilizan versiones de SQLite nativas incompatibles. Para evitar alterar la persistencia estable del núcleo, el proveedor corre como proceso Rust independiente y entrega controles firmados HMAC por TCP local (127.0.0.1). El verificador Rust ya implementado en src/whatsapp_bridge.rs autentica esos eventos. No interviene Node. Ver providers/whatsapp-rust/README.md. El receptor TCP aún debe probarse e integrarse antes de su uso estable.

## Receptor Rust de comandos de WhatsApp

El binario de Base-Matter whatsapp-matter-control abre exclusivamente un socket TCP de loopback para eventos WhatsApp firmados por el proceso Rust whatsapp-rust independiente. Primero se ejecuta --init una sola vez para provisionar la identidad del socket; --serve exige dueño existente y claves de transporte/replay diferentes, y rechaza datos no autenticados. No ejecuta Bot-Matter, no admite QR ni ping. Los comandos de prueba on/off/status pasan por el mismo ControlDispatcher con persistencia y auditoria. Ver docs/WHATSAPP_IPC.md. La comunicacion entre procesos necesita pruebas reales con telefono antes de denominarse estable.

La prueba local del puente TCP Rust comprueba una sesión de lazo local: on, repetición del mismo ID y off, sin enviar mensajes a WhatsApp. No sustituye las pruebas con un teléfono real.

## Prueba de configuración del puente Rust

Los dos ejecutables Rust incorporan `--check` para revisar variables de entorno, secretos diferentes, acceso a directorios privados y dirección TCP local antes de abrir una sesión. No crean bases SQLite ni inician WhatsApp en este modo. Las instrucciones de emparejamiento deben seguir siendo exclusivamente por pairing code. No utilizar el antiguo ping ni desarrollar Bot-Matter.

## Prueba real de WhatsApp — dos procesos Rust

Para probar el emparejamiento exclusivamente por código en `matter-beta`, sigue la [guía paso a paso](docs/WHATSAPP_TRIAL.md). Incluye creación de un entorno privado y comandos separados del receptor Base-Matter y del proveedor `whatsapp-rust`. Sin sesión real verificada, no declarar estabilidad ni promover el proveedor a `matter`.

## Contrato común para WhatsApp, Telegram y Discord

La infraestructura de conexiones y mensajes conserva los controles de Base-Matter para todas las plataformas. El [registro de controladores Rust](docs/PLATFORM_CONTRACT.md) dirige cada sesión exclusivamente a su proveedor canónico, sin fallback cruzado, sin compartir credenciales y sin suplantar identidad. Los adaptadores de Telegram-Matter y Discord-Matter permanecen pendientes; este contrato no afirma que estén conectados. WhatsApp continúa usando `whatsapp-rust` y pairing code; la prueba con teléfono y las puertas de estabilidad siguen pendientes.

## Protección de archivos de bloqueo en Rust

Los dos procesos de WhatsApp crean sus archivos de bloqueo de propiedad de sesión con permisos privados `0600` desde la apertura en Unix. Conservan la exclusión por `create_new`, no sustituyen bases SQLite ni recuperan automáticamente locks obsoletos. Requiere CI y evaluación de la sesión real; no cambia el control global `=/>` ni habilita Bot-Matter.

## Límite de recepción del puente WhatsApp

El receptor TCP de controles `=/>` corta los frames de más de 16 KiB y los envíos incompletos o demasiado lentos. El plazo máximo de lectura es 4 segundos por conexión, no un temporizador reiniciado por cada fragmento. Esta es una protección local; no sustituye la verificación de sesión WhatsApp, HMAC ni las pruebas con un teléfono.

## Verificación del resultado del control WhatsApp

El proveedor Rust ahora distingue el acuse de control `applied` del resultado
`duplicate`. Los frames repetidos no se muestran como ejecuciones nuevas;
`ignored`, `denied`, errores de JSON, claves extrañas y acuses incompletos
fallan cerrados. Esto mejora la observabilidad de las pruebas sin alterar
el protocolo HMAC. El proveedor añade posteriormente una confirmación
genérica solo para `applied`, documentada abajo. La conexión real y
la identidad del JID siguen pendientes de prueba.

## Recuperación de procesos WhatsApp

Los archivos de bloqueo de sesión y del controlador usan a partir de este
checkpoint un marcador de formato v2 y el bloqueo exclusivo nativo de archivos
de Rust 1.89 o posterior. El lock no se elimina al apagar; al terminar un
proceso se libera automáticamente el bloqueo del sistema operativo, permitiendo
reiniciar sin borrar la sesión SQLite. Los locks v1 con PID, sin formato v2,
se rechazan hasta que el operador confirme que el proceso anterior se detuvo:
**nunca se retiran ni se sobrescriben automáticamente**. La validación
`--check` verifica un lock v2 sin modificar sesiones ni archivos.

## Prueba real de procesos Rust sin dispositivo

La suite de integración incluye un test que ejecuta el binario controlador
`whatsapp-matter-control` en otro proceso, le envía comandos HMAC firmados
por TCP local, corta bruscamente el proceso y verifica el reinicio con el lock
v2. Un ID de mensaje usado antes de la caída sigue rechazado como replay
gracias a SQLite. La prueba no conecta ni vincula ningún teléfono; la
validación de WhatsApp real continúa pendiente.

## Receptor obligatorio antes de WhatsApp

El arranque del proveedor Rust exige que su controlador Base-Matter esté
escuchando en el loopback configurado antes de abrir SQLite o iniciar el
pairing code. Si no hay conexión TCP, el proveedor cancela el arranque con
un error genérico, sin modificar la sesión. Esta comprobación no autentica
el receptor ni sustituye HMAC: la firma se verifica en cada orden posterior.
`--check` continúa sin conexiones ni cambios de archivos.

## Confirmación privada del control global en WhatsApp

Después de verificar un comando `=/>` del dueño en su propio chat privado,
el proveedor Rust responde únicamente si Base-Matter devuelve `applied`,
con el texto fijo «Base-Matter: solicitud procesada.». Un replay
`duplicate`, un comando no autorizado o un fallo de IPC no generan respuesta.
La respuesta no empieza por ningún prefijo de comando y, por tanto, no se
auto-ejecuta. La función no devuelve todavía información detallada del estado,
y requiere prueba con teléfono y recepción real antes de declararse estable.

## Rama de desarrollo y la comunidad REGISTER

La estrategia de ramas canónicas es [`stable` / `beta` / `alpha`](docs/BRANCH_STRATEGY.md).
Las ramas anteriores `matter` y `matter-beta` se conservan sin forzar
fusiones mientras se migran las referencias, la rama por defecto y la CI.

La especificación de [`=/>start` y grupos oficiales REGISTER](docs/COMMUNITY_REGISTER.md)
separa **detección y auditoría** de los cambios reales (imagen, nombre,
descripción, ajustes, creación y administración). Los subgrupos requieren
coincidencia exacta del identificador al inicio de la descripción, pertenencia
verificada a la comunidad y autorización del dueño. No hay creación masiva
ni cambio de permisos sin revisión del plan.

El [contrato externo `#matter`](docs/EXTERNAL_BOT_COMPAT.md) permite que
bots ajenos actúen como clientes autorizados del control `=/>`, sin copiar
sesiones WhatsApp ni conceder privilegios por compartir un prefijo.

## Comando alpha =/>start

El proceso de control Base-Matter registra la orden `=/>start` y comprueba
autorización del dueño del socket antes de admitir el inventario de
comunidades. Funciona aunque el socket esté apagado. Las operaciones
`=/>start apply` se rechazan mientras no exista un plan de cambios
verificado; no se modifica ninguna comunidad por autorizar la lectura.

## =/>start — inventario comunitario de solo lectura

Desde `alpha`, la confirmación del controlador Base-Matter permite que el
proveedor Rust consulte las comunidades de la sesión WhatsApp, compruebe
qué comunidades tienen admin verificado para el propio JID PN/LID, enumere
subgrupos vinculados y compare la primera línea exacta de la descripción
con `MATTER:OFFICIAL:register:<community-id>`. Responde únicamente al
chat propio con el resultado o un fallo de lectura. Los cambios de nombre,
imagen, permisos, links y creación permanecen bloqueados.

## Estética reutilizable de menú y ayuda

El módulo Rust `control_style` renderiza `menu`, `help` y `h`
mediante una plantilla compartida con título, bot ID, encabezado,
comandos globales y créditos. `ControlThemes` aísla overrides
por identificador de comunidad y utiliza un diseño por defecto.
Es solo presentación y no atribuye roles ni permisos. La persistencia,
la entrega al chat y los estilos de bots externos se integrarán
por el contrato público; no hay una copia exacta de una estética
ajena sin su diseño fuente.

## Respuestas de estado y extensiones del control

El receptor `whatsapp-matter-control` puede adjuntar texto informativo
limitado a 760 bytes al resultado `applied`, para `status`,
encendido/apagado y extensiones autorizadas. Nunca devuelve estos datos
en un rechazo ni un replay. El proveedor verificará ese campo antes de
enviarlo al chat; el cambio no altera la firma HMAC de entrada.

## Menús globales visibles desde WhatsApp (alpha)

Las extensiones `=/>menu`, `=/>help` y `=/>h` usan el renderizador
Rust de `ControlTheme`, tras validar el propietario del socket por HMAC.
El receptor devuelve texto limitado por el IPC y el proveedor lo responde
únicamente al chat privado propio. La plantilla incorpora nombre,
bot ID y créditos. El diseño puede variar por comunidad a través de
`ControlThemes`; todavía no hay almacenamiento ni administración de temas
desde WhatsApp. En paralelo `=/>start` mantiene el informe de lectura.

## Menú universal sin diseño inventado

Los comandos `=/>menu`, `=/>help` y `=/>h` muestran exclusivamente los comandos globales existentes, con texto plano y sin un marco decorativo inventado. La estética de otro bot solo puede reutilizarse tras observar un menú real autorizado y extraer sus componentes visuales, nunca sus comandos o identidad. No se afirma que el proveedor pueda leer automáticamente menús de terceros.

## Ramas definitivas y estética del menú

La rama predeterminada GitHub ya es `stable`; el desarrollo continúa en `alpha` y la integración validada se prepara en `beta`. Se retirarán referencias heredadas sin descartar commits exclusivos. La salida de `=/>menu` y `=/>help` es deliberadamente un listado sencillo. [La estética real de otros bots](docs/MENU_STYLE_IMPORT.md) se extraerá únicamente de un mensaje fuente autenticado, sin copiar sus comandos ni contenido.

## Respaldo de recuperación comunitaria `=/>add++`

El controlador Rust reconoce la orden global exacta `=/>add++` del dueño del socket, con independencia de que el bot esté encendido. Por defecto solo admite una previsualización; `=/>add++ snapshot <community-jid>` reserva una operación explícita de inventario privado cifrado. No existe modo `apply` de incorporación masiva y ningún comando agrega personas sin autorización o consentimiento. El objetivo es preparar una copia protegida de la lista accesible **antes** de una eventual suspensión; no garantiza acceso posterior ni eludir restricciones de WhatsApp. [Plan de recuperación](docs/COMMUNITY_RECOVERY.md).

## Vista previa real de `=/>add++` en WhatsApp Rust

En `alpha` el proveedor consulta `get_linked_groups_participants` solo para comunidades con administrador verificado, deduplica los identificadores PN/LID conocidos y muestra el número aproximado y el nombre sugerido para un grupo de respaldo. No transmite listas de números por chat ni agrega a ninguna persona. La modalidad `snapshot` aún indica explícitamente que no se ha guardado un respaldo cifrado. No está aprobada para una comunidad suspendida.

## Cifrado del respaldo comunitario

El proveedor Rust incorpora instantáneas de recuperación cifradas con XChaCha20-Poly1305, un nonce aleatorio y una clave dedicada de 256 bits distinta del puente WhatsApp. Cada instantánea contiene los identificadores PN/LID accesibles para recuperación futura, pero se guarda únicamente como ciphertext local con permisos 0600 en el directorio privado de la sesión. No se copia ninguna sesión ni se publican números en GitHub. Para configuraciones anteriores se habilita la clave sin reemplazar los valores existentes con `bash scripts/whatsapp-test-setup.sh --enable-recovery`. La conexión de este almacén con `=/>add++ snapshot` queda sujeta al próximo checkpoint de CI.

## `=/>add++ snapshot` — Respaldo cifrado conectado

Cuando el dueño emite `=/>add++ snapshot <community-jid>` en su propio chat, Base-Matter primero valida la firma HMAC y el propietario del socket; el proveedor revisa después que la sesión sea administradora de una comunidad activa, solicita los participantes vinculados y crea un archivo cifrado XChaCha20-Poly1305, sin publicar números de teléfono ni modificar grupos. Se requiere una clave adicional generada con `scripts/whatsapp-test-setup.sh --enable-recovery` para configuraciones existentes. Si falla la consulta, el guardado o la autorización, no se declara una copia completada. La recuperación por invitaciones sigue pendiente; una instantánea no fuerza a miembros a unirse a otra comunidad.
