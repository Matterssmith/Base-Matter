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

## Persistence and trust boundaries

The bot registry is shared through `MatterContext` and persists family/release/instance identity in SQLite when injected; initialization and schema version registration are transactional. Owner and co-owner checks, controlled removal, and registry ownership audit records are implemented. Reading bot audit history requires the current owner. General role inheritance and authorization across other resource types are not. Shared users have global IDs and explicit provider/subject links; every link, resolution, and bot-scoped data operation requires a caller-supplied `UserPrincipal` policy. Matching external identifiers are never merged automatically. Private user data is keyed by bot instance, user, and key. The caller's `UserAuthorizer` must verify principal authentication, bot existence, ownership, and scope; this registry does not query the bot registry itself. The legal subsystem persists legal documents and consent decisions in SQLite; initialization and component schema-version updates are transactional, preserve unversioned existing data, roll back failed DDL, and reject unsupported future schema versions. Sensitive legal reads/writes require a `LegalPrincipal` evaluated by an application-supplied `LegalAuthorizer`; this crate delegates external identity authentication.
Document registration and the read/validate/write path for consent decisions
use immediate transactions, so concurrent independent database connections
cannot both withdraw the same grant or observe a document-registration race
between its duplicate check and insert. An access check uses one SQL statement
to read the user and applicable document/decision state from one snapshot.
The event bus, service registry, translation catalogs, and runtime config are
in memory or caller supplied. General roles, socket control, plugin isolation,
and a general authorization/audit service remain unimplemented.

The security provider abstractions allow a caller to supply trusted key
material and providers. Envelope plaintext and ciphertext are limited to
16 MiB; encoded ciphertext length is checked before Base64 decoding. They do not establish that an external Matter-Vault
service is connected. There is no storage of WhatsApp sessions or external
platform credentials in Matter-Base. Authenticated key loading checks that the
Vault identity returned while staging keys still matches the authenticated
identity before replacing the current key set.

Assertion replay tracking defaults to an in-memory store. An injectable SQLite
store persists replay IDs, globally removes expired records, and enforces
uniqueness and capacity transactionally across connections and process restarts. Deployments that need restart safety must
configure that store; this does not provide a distributed replay service.

## Planned architecture (not implemented)

The following systems are future work and must be designed behind stable
contracts before applications consume them:

- Ownership, authorization, and audited registration control for bots.
- Ownership, roles, permissions, and audited authorization.
- Platform-neutral router, command registry, middleware, and filters.
- Control Plane, socket ownership, activation policy, and isolation.
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
