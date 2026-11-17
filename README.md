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
registries, translation lookup, a persistent bot identity registry, an
optionally persistent shared-user registry with explicitly authorized external
identity links and bot-scoped user data, a persistent legal-consent store, and
cryptographic key/envelope and provider-authentication components, a command
router, and persistent socket activation policies. The bot
platform capabilities listed in the long-term architecture are not yet
implemented. See [the development status](docs/DEVELOPMENT_STATUS.md) for the
verified stage assessment.

## Responsibilities and boundaries

Matter-Base is intended to host reusable Rust capabilities consumed through
public APIs. It must remain independent of Matter-Bot and platform adapters.
This repository currently does **not** implement WhatsApp, Telegram, Discord,
network socket control, plugin execution, or a sandbox. General scoped role
authorization is opt-in and is not wired into platform adapters. Command routing and persistent activation policies are partial.

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
```

There is one README in this repository: this root `README.md`.

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
\`command:<name>\` with action \`execute\`. A successful authorization is
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
private key, master key or external session is stored here. Remote conflicts,
multiple simultaneous publishers, recovery/import and actual authenticated
GitHub commits remain integration requirements. See
[Vault integration](docs/vault-integration.md).

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

## Roadmap boundary

General role inheritance, full Control Plane lifecycle, connection and
messaging contracts, plugin validation, and
effective sandbox isolation remain pending. Matter-Bot must not be built on
this crate until those core contracts are implemented, integrated, documented,
and tested.

## Credits

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## Ecosystem names and provider layout

The requested target repositories are **Matter-WhatsApp** (formerly
Matter-Baileys) and **Matter-Telegram** (formerly Matter-Telegraf). Remote renames
and provider imports are pending ecosystem work. Provider methods belong in
separate directories such as `providers/baileys` and `providers/telegraf`, using
verified official upstream revisions and preserved licenses/history. Base and
Vault readiness remains the immediate priority. See [repository instructions](AGENTS.md).
