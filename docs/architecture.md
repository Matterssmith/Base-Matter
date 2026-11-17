# Matter-Base architecture

## Purpose and dependency direction

Matter-Base is intended to provide reusable, platform-independent Rust
infrastructure to applications in the Matterssmith ecosystem. Applications may
depend on Matter-Base; the core must not depend on Matter-Bot or a concrete
messaging platform.

The intended full platform architecture is not yet present. This document
separates the implemented crate from the planned capabilities so interfaces
and diagrams do not imply that missing systems already work.

## Current crate structure

```text
Public crate API (`src/lib.rs`)
├── Bot identity registry (memory or SQLite)
├── Shared users and bot-scoped data (memory or SQLite)
├── Core
│   ├── CoreConfig
│   ├── MatterRuntime
│   └── MatterContext ── EventBus (in memory)
│                     ├─ ServiceRegistry (in memory)
│                     ├─ UserRegistry (injected; memory or SQLite)
│                     └─ LegalStore (SQLite)
├── SessionRegistry (SQLite connection metadata/revision/audit)
├── MessageService (admission and external driver contract)
├── RoleStore (SQLite inherited role policy, bounded audit)
├── SocketActivationStore (SQLite policy/ownership/bot binding/audit)
├── ControlDispatcher (control interception and socket activation gate)
├── CommandRouter (prefixes, aliases, injected authorization and middleware)
│   └── Optional CommandAuditStore (SQLite attempts/outcomes, owner-scoped history)
├── PluginHost (trusted manifest + injected sandbox contract)
├── Translator (in memory)
├── CoreModule + runtime-owned ModuleManager
└── Security
    ├── EncryptedEnvelope / KeyManager
    ├── ProviderAuthenticator / ProviderAccessPolicy
    └── KeyProvider / ProviderBindingStore contracts
```

`MatterRuntime::open` opens the SQLite legal store and constructs a context with in-memory bot and user registries. `open_with_bot_registry` configures bot persistence. `open_with_registries` accepts explicit bot and user registries, including SQLite persistence. The runtime owns a shared `ModuleManager`; callers register modules before initialization, then call `initialize`, `start`, and `shutdown`. Lifecycle errors propagate to the application. Callback unwinding panics become errors; failed initialization attempts cleanup. No modules are registered automatically. See the crate modules in [`src/`](../src/).

The current event bus clones a listener list and releases its registry lock
before invoking callbacks, so callbacks may subscribe or clear listeners
reentrantly. Callbacks are synchronous. `publish` propagates a callback panic and stops
dispatch; `publish_checked` catches unwinding panics per handler, continues
with remaining listeners, and returns an error summary.

`CommandRouter` resolves configured prefixes, canonical command names and
aliases, and whitespace-separated arguments. It requires an injected
`CommandAuthorizer`, runs bounded middleware after authorization, and invokes
synchronous handlers outside its registry lock. Authorization sees the
canonical command name, so aliases cannot change permission matching. Callback
errors and unwinding panics fail closed with sanitized errors. An injected
`CommandAuditStore` persists recognized attempts before callbacks and terminal
outcomes afterward; absent outcomes mean unknown effects, not safe retry.
The default router is unaudited. Platform parsing and network connection
lifecycle are not integrated;
the caller's authorizer must enforce bot scope, permissions, and activation.

## Persistence and trust boundaries

The bot registry is shared through `MatterContext` and persists family/release/instance identity in SQLite when injected; initialization and schema version registration are transactional. Owner and co-owner checks, controlled removal, and registry ownership audit records are implemented. Each bot accepts at most 256 co-owners, and corrupted persisted lists over that limit fail validation. Reading bot audit history requires the current owner. Identity reads require an actor ID and reveal a record only to its owner or a co-owner; list results are limited to records shared with that actor. Consumers must pass an ID from authenticated request context. General role inheritance and authorization across other resource types are not. Shared users have global IDs and explicit provider/subject links; every link, resolution, and bot-scoped data operation requires a caller-supplied `UserPrincipal` policy. Matching external identifiers are never merged automatically. Private user data is keyed by bot instance, user, and key. The caller's `UserAuthorizer` must verify principal authentication, bot existence, ownership, and scope; this registry does not query the bot registry itself. The legal subsystem persists legal documents and consent decisions in SQLite; initialization and component schema-version updates are transactional, preserve unversioned existing data, roll back failed DDL, and reject unsupported future schema versions. Sensitive legal reads/writes require a `LegalPrincipal` evaluated by an application-supplied `LegalAuthorizer`; this crate delegates external identity authentication.
Document registration and the read/validate/write path for consent decisions
use immediate transactions, so concurrent independent database connections
cannot both withdraw the same grant or observe a document-registration race
between its duplicate check and insert. An access check uses one SQL statement
to read the user and applicable document/decision state from one snapshot.
The event bus, service registry, translation catalogs, and runtime config are
in memory or caller supplied. Scoped roles, session metadata and a message driver contract are opt-in.
Real network socket lifecycle, external platform connections, and plugin
isolation remain unimplemented.

The security provider abstractions allow a caller to supply trusted key
material and providers. Envelope plaintext and ciphertext are limited to
16 MiB; encoded ciphertext length is checked before Base64 decoding. They do not establish that an external Matter-Vault
service is connected. There is no storage of WhatsApp sessions or external
platform credentials in Matter-Base. Authenticated key loading checks that the
Vault identity returned while staging keys still matches the authenticated
identity before replacing the current key set. Provider key lists must contain
between 1 and 4,096 entries; an empty or oversized list is rejected without
replacing the current keys. The key manager also caps its total number of keys
at 4,096, including manually inserted or rotated keys. Authenticated key reads
reject a key manager associated with a different Vault.

Assertion replay tracking defaults to an in-memory store. An injectable SQLite
store persists replay IDs, globally removes expired records, and enforces
uniqueness and capacity transactionally across connections and process restarts. Deployments that need restart safety must
configure that store; this does not provide a distributed replay service.

## Planned architecture (not implemented)

The following systems are future work and must be designed behind stable
contracts before applications consume them:

- General role and resource authorization beyond the existing bot owner/co-owner checks.
- Ownership, roles, permissions, and audited authorization.
- Broader ingress audit (including invalid/unregistered messages); recognized
  command auditing and socket gating are implemented when configured.
- Full Control Plane/network lifecycle; owner-authorized socket handover and
  binding recovery, activation storage and control dispatch are implemented in
  `activation`/`control`. Lost-owner recovery needs a separate trusted policy.
- Session/credential interfaces, services, connections, and messaging.
- Plugin manifest validation, resource limits, and effective sandboxing.
- Persistent recovery, migrations, and end-to-end integration tests.

Matter-Bot and platform adapters remain separate consumers. No concrete
WhatsApp, Telegram, or Discord implementation belongs in this crate.

## Verification status

The current automated suite exercises legal storage, encryption/key handling,
provider-authentication contracts, and basic in-memory core components. It is
not an end-to-end bot platform suite. Current stage classifications and exact
verification results are maintained in
[`DEVELOPMENT_STATUS.md`](DEVELOPMENT_STATUS.md).

Socket activation policy storage is described in [activation contracts](activation.md).
It provides durable ownership/power/overrides/global fallback and atomic audits, but does not
manage network connections. `ControlDispatcher` enforces activation before
normal routing and intercepts power commands with socket-owner checks before
bot lookup. A missing binding or revoked bot ownership blocks normal commands,
but cannot prevent the socket owner from switching operational power off.

Persistent command audit is described in [command audit contracts](command-audit.md).

The optional persistent global-policy dispatcher reads all activation scopes in
one SQLite snapshot together with socket ownership and bot binding. Owner
transfer/release atomically clears binding/local overrides and powers off, so
new admissions cannot combine state from before and after a handover. Global policy administrators have no implicit authority
over socket power or local overrides; static configuration remains supported.
