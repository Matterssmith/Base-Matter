# ꧁✪⃢.✪ 𝐌𝖆𝖙𝖙𝖊𝖗𝖘𝖘𝖒𝖎𝖙𝖍۞₪»̶ ༄☾︎𝐌𝐒☽︎༄

<div align="center">

<img
src="https://capsule-render.vercel.app/api?type=waving&color=gradient&customColorList=12,19,24&height=240&section=header&text=𝐌𝖆𝖙𝖙𝖊𝖗%20𝐁𝖆𝖘𝖊&desc=The%20Core%20Architecture&fontSize=52&fontColor=FFFFFF&animation=fadeIn&fontAlignY=38&descAlignY=58&descSize=20"
width="100%"
/>

# Matter Base

### The Core Architecture.

**Powered by Matterssmith.**  
**Engineered for the Future.**

*A reusable Rust foundation for the Matterssmith ecosystem.*

</div>

---

## Project status

Matter-Base is an **incomplete development library**. The current repository
contains a runtime/context foundation, in-memory event and service
registries, translation lookup, a persistent bot identity registry, a persistent
legal-consent store, and cryptographic key/envelope and provider-authentication
components. The bot
platform capabilities listed in the long-term architecture are not yet
implemented. See [the development status](docs/DEVELOPMENT_STATUS.md) for the
verified stage assessment.

## Responsibilities and boundaries

Matter-Base is intended to host reusable Rust capabilities consumed through
public APIs. It must remain independent of Matter-Bot and platform adapters.
This repository currently does **not** implement WhatsApp, Telegram, Discord,
shared application users, general authorization roles,
command routing, socket control, plugin execution, or a sandbox.

Bot registration requires an owner ID. Consumers must derive actor IDs from
authenticated requests before calling owner-management APIs. Owners can manage
co-owners and transfer/delete bots; co-owners can manage bot-scoped operations
but cannot change ownership or delete. Legacy unowned records require an
injected `OwnerRecoveryAuthorizer` before ownership can be claimed. Ownership
changes are written to the registry audit history.
Reading a bot's audit history requires the current owner ID and is checked by
the identity store when records are read. Access to the retained audit history
of a deleted bot is not exposed through the current registry API.

The security module defines provider and key-provider contracts. It does not
ship a production Matter-Vault integration or store platform sessions or
credentials. The tests use local test providers and do not verify a live Vault
service.

## Implemented components

| Component | Current behavior | State |
| --- | --- | --- |
| `core::config` | Default application name, environment, debug flag, and locale | Partial; no validation/loading |
| `core::runtime` | Opens legal storage and builds a context; can inject a bot registry | Partial; default bot registry is in memory and `initialize` logs only |
| `core::context` | Shares event bus, typed service registry, legal store, and bot registry | Implemented at this scope |
| `bots` | Family/release/instance identity, unique display names, in-memory or SQLite-backed registration and recovery with atomic schema versioning | Implemented owner/co-owner enforcement and audited ownership changes; general roles and shared user validation remain pending |
| `events` | Synchronous named callbacks; listeners are copied before invocation so callbacks run outside the registry lock | Basic; in-memory and not durable; a callback panic propagates from `publish`; `publish_checked` catches unwinding panics per listener, continues dispatch, and returns an error |
| `services` | In-memory type-indexed service registration and retrieval | Basic; no lifecycle management |
| `i18n` | In-memory translation catalogs and key fallback | Basic; no bundled catalogs or locale validation |
| `legal` | SQLite documents, user IDs, immutable decision history, and access checks | Implemented at this scope |
| `security` | Authenticated encryption envelopes, key management, provider identity/policy, replay-aware assertions, binding contracts | Implemented contracts/primitives; external integration unverified |
| `modules` | `CoreModule` plus caller-owned `ModuleManager` for ordered initialize/start and reverse stop with failure cleanup | Implemented lifecycle coordinator; not wired into `MatterRuntime` |

These states describe this repository's present behavior, not the full planned
Matter-Base platform.

## Repository layout

```text
src/
  bots/       Bot identity contracts and in-memory/SQLite registry
  core/       Configuration, shared context, logging, runtime
  events/     In-memory event bus
  i18n/       Translation catalog lookup
  legal/      SQLite legal-document and consent store
  modules/    CoreModule contract
  security/   Encryption, key management, provider authentication/contracts
  services/   Typed in-memory service registry
tests/        Integration-style tests for the implemented contracts
docs/         Architecture and continuity documentation
```

There is one README in this repository: this root `README.md`.

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
    runtime.initialize();
    Ok(())
}
```

The default `MatterRuntime::new` path is `data/legal/registry.db`. Tests and
applications that manage storage separately can provide a `LegalStore` with
`MatterRuntime::with_legal_store`. Use `open_with_bot_registry` with
`BotRegistry::open_sqlite` when bot identities must survive restarts.

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
  authenticate platform identities or provide the shared user directory.
  Authorization failures are returned as a generic denial without forwarding
  policy-provider diagnostics.
- The runtime persists only the legal store. Bot registrations, users, socket
  state, configuration, events, and service metadata do not have persistence
  implementations here.
- Envelope encryption validates metadata, uses the existing
  XChaCha20-Poly1305 format, and caps plaintext/ciphertext at 16 MiB before
  encryption or Base64 decoding. Keep compatibility when changing that contract.
- Key-provider and binding-store interfaces are injection points. A live,
  production Vault connection has not been verified by this repository.
- Authenticated key loading rechecks the Vault identity when the key set is
  staged; a mismatch is rejected before replacing the current key manager
  contents.
- `ProviderAuthenticator::new` uses a bounded in-memory replay store.
  Applications can inject `SqliteProviderAssertionReplayStore` through
  `new_with_replay_store` to reject assertion replays across verifier restarts
  and independent database connections. Expired rows are cleaned across scopes;
  both per-scope and global capacity are bounded. It is not a distributed store.
- Authenticated provider assertions are limited to 16 KiB, and a key provider
  may return at most 4,096 key IDs per load. Oversized inputs fail closed.
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
    let _audit = bots.audit_history(bot.instance_id(), authenticated_owner_id)?;
    Ok(())
}
```

Pass an owner ID derived from the application's authenticated request. Do not
accept it directly from an untrusted request field.

## Roadmap boundary

Shared users and their explicit identity links, general role inheritance,
universal command routing, Control Plane/socket policy,
connection and messaging contracts, plugin validation, and effective sandbox
isolation remain pending. Matter-Bot must not be built on this crate until
those core contracts are implemented, integrated, documented, and tested.

## Credits

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
