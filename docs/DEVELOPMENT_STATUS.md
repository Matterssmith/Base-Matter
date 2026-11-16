# Matter-Base development status

This is a development ledger. GitHub remains the source of truth. Its
synchronization entry records the published base of each working checkpoint;
verify the branch log for the current published commit.

## Recovery snapshot

- Repository: `/home/Sakura/Matter-Base`
- Working branch: `matter-beta`
- Previous synchronized base commit: `b9ca1d0e46c3f63428d9545da7e9923f6b86b129`
- Latest confirmed checkpoint before the current local work:
  `b9ca1d0e46c3f63428d9545da7e9923f6b86b129`
- Remote: `origin` (`https://github.com/Matterssmith/Matter-Base.git`)
- Synchronization: `git fetch origin` succeeded before and after the checkpoint
  push; local `HEAD` matched `origin/matter-beta` at
  `b9ca1d0e46c3f63428d9545da7e9923f6b86b129` before the lifecycle resilience
  tests. A fresh fetch and equality check are still required after publishing.
- Initial tracked/untracked state: clean. `target/` was present as ignored
  build output and was preserved.
- Applicable repository `AGENTS.md`: none found in the repository or its
  ancestor directories.

## Current stage

**H — integral audit, in progress.** Initial repository-wide source and test
inventory has been completed. A Vault identity recheck flaw in authenticated
key loading was corrected and has a regression test. Legal writes were also
made transactional and tested over independent connections. Confirmed gaps
are recorded below. Timestamp overflow in provider assertions was also
rejected safely. Legal access evaluation now uses one SQL statement for a
consistent database snapshot. Event and translator inputs are converted
before acquiring write locks so caller-supplied conversion code cannot poison
those locks by panicking. The H–L stages are not complete, and Matter-Base is
not ready for Matter-Bot.

## Capability assessment from the current working tree

| Capability | Status | Evidence / limitation |
| --- | --- | --- |
| Runtime and lifecycle | PARTIAL | `MatterRuntime` owns a shared `ModuleManager` and propagates initialize/start/shutdown errors; no built-in modules are registered and services have no lifecycle. |
| Bot identity, family/release/instance IDs, registration | PARTIAL | `src/bots` generates IDs, persists validated identities, and allocates unique names; owner-only transfer/removal, limited co-owner scope, legacy recovery authorization, and audit history work. Shared-user validation and general role/resource authorization remain absent. |
| Shared application users and bot data isolation | ABSENT | `legal_users` are legal-consent subjects, not a shared application user system. |
| Sessions and platform credential contracts | ABSENT | No session/credential subsystem; platform credentials must remain outside this crate. |
| Persistence and recovery | PARTIAL | SQLite persists bot identities and legal documents/decisions. No config/socket/service persistence or ecosystem-wide migration coordinator; the bot registry has its own transactional schema versions. |
| Events and messaging | PARTIAL | Named synchronous in-memory event callbacks exist; no durable events or messaging contract. |
| Router and command registry | ABSENT | No implementation. |
| Middleware, roles, permissions, authorization | PARTIAL | Bot owner/co-owner checks and an injectable legal action/resource authorization contract exist; no shared user identity, general role inheritance, or socket/resource policy engine. |
| Control Plane, sockets, activation | ABSENT | No implementation; socket ownership/policies cannot be verified. |
| Services and connections | PARTIAL | Typed in-memory service registry only; no connection manager or lifecycle. |
| Plugins, extensions, sandbox | ABSENT | No manifest validation, isolation, or resource limits. |
| Application audit log | ABSENT | Legal decisions are append-only; no general administrative audit system. |
| Encryption/key/provider contracts | PARTIAL | Primitives and injection contracts have tests; no production key service integration or independent security audit. |
| Provider assertion replay protection | PARTIAL | Default is in memory; an injectable SQLite store persists IDs and rejects duplicates across restarts/connections, but no distributed store exists. |
| Provider input resource limits | PARTIAL | Assertions are capped at 16 KiB and provider key loads at 4,096 IDs; no general service/plugin resource limits exist. |
| Matter-Vault | BLOCKED | Interfaces exist, but there is no live service integration or external verification. |
| Public API stability / multi-language contracts | PARTIAL | Rust public exports exist; no compatibility policy, versioned external protocol, or SDK contracts. |
| Documentation | PARTIAL | Core README and architecture claims were stale and have been corrected locally; remaining release/documentation audit is pending. |

## Work completed in this local session

- Confirmed the remote branch and commit before building or editing.
- Ran `cargo fmt --all`; it applied formatting changes throughout existing
  Rust source and tests. Behavior changes are listed separately.
- Corrected the root README and architecture, service, i18n, and logger docs
  to describe actual Rust APIs and mark absent capabilities as pending.
- Corrected `INFO.md` to use the canonical repository and describe current
  scope.
- Fixed a Vault identity time-of-check/time-of-use gap: authenticated key
  loading now validates the provider's Vault ID again at staging time, before
  any staged key set replaces the existing `KeyManager` state.
- Added an integration-style SQLite recovery test that closes and reopens the
  runtime and verifies bot identity, user, consent history, and access survive.
- Made legal document registration and consent decision validation/writes
  transactional across independent SQLite connections.
- Changed legal access evaluation to one SQL statement so user and consent
  requirements are read from one SQLite snapshot.
- Added a concurrency test proving two independent connections cannot both
  withdraw the same consent grant.
- Added a regression test for a provider that changes Vault IDs between the
  binding proof and key loading; the change is rejected and preexisting keys
  remain intact.
- Replaced overflow-prone arithmetic on assertion timestamps with bounded
  timestamp-window checks and checked subtraction. Added regression cases for
  extreme signed timestamp values.
- Moved caller-provided `Into<String>` conversions outside EventBus and
  Translator write-lock scopes. Added a regression test proving a panicking
  conversion does not poison those internal locks.
- Confirmed EventBus invokes callbacks after releasing its registry lock and
  added a reentrant-subscription test. Documented that callback panics
  propagate and stop the current publish call; callback error isolation is
  not implemented.
- Added fail-closed limits of 16 KiB for authenticated assertion ciphertexts
  and 4,096 key IDs per provider load. Oversized inputs are rejected before
  decryption or key material loading; regression tests pass.
- Added an atomic `ProviderAssertionReplayStore` contract, bounded in-memory
  implementation, and SQLite implementation. `ProviderAuthenticator` can
  receive a shared store; SQLite replay rejection survives verifier and
  process restarts. Added persistence, expiry, capacity, and cross-connection
  tests.
- Added this continuity record.
- Preserved the exact project credit in the root README.
- Extended `tests/platform_flow.rs` to exercise mandatory legal access denial
  before consent, grant after acceptance, renewed denial after withdrawal, and
  recovery of both bot identity and legal history after closing and reopening
  the SQLite stores. The test uses a unique temporary directory and removes
  only that test-owned directory.
- Fixed unauthorized access to bot ownership audit history. Reads now require
  the current owner and check ownership inside the same store lock/read
  transaction as retrieving records. Co-owners and unrelated actors are
  denied. Records remain stored after bot deletion but are inaccessible through
  the current registry API after the identity is removed.
- Current legal-store schema work modifies `src/legal/mod.rs` and adds three
  isolated migration tests in `tests/legal.rs`; `README.md` and
  `docs/architecture.md` now describe the transactional versioned initialization.
- Legal schema hardening is now implemented locally in `src/legal/mod.rs`:
  SQLite foreign keys are enabled before the migration transaction, and schema
  objects plus the `legal_store` component version are committed atomically.
  Existing unversioned records survive upgrade; failed DDL rolls back; negative
  or unsupported future versions are rejected.
- Added public `LegalAction`, `LegalAuthorizationRequest`, `LegalAuthorizer`,
  and `LegalPrincipal` contracts. Legal user registration/lookups, document
  registration/activation, consent writes/history, required-document queries,
  and access checks now require the supplied principal policy. Runtime/context
  access checks require it too. Authorization errors are sanitized. The crate
  delegates real principal authentication and policy to consuming applications;
  no shared user directory or production authorization provider is implemented.
- Added `tests/common/mod.rs` with a test-only policy and regression coverage
  rejecting cross-user consent forgery, unauthorized history reads, and
  unauthorized document activation. This fixture verifies contract use, not a
  real external identity integration.
- Updated README and architecture docs to define the delegated legal identity
  boundary and the actual implemented/pending scope.
- Files changed for the legal authorization contract: `src/legal/mod.rs`,
  `src/core/context.rs`, `src/core/runtime.rs`, `src/lib.rs`,
  `tests/common/mod.rs` (created), `tests/core.rs`, `tests/legal.rs`, and
  `tests/platform_flow.rs`; docs: `README.md`, `docs/architecture.md`, and
  this ledger. No files were deleted.
- Wired `ModuleManager` into `MatterRuntime` through shared synchronized state.
  Consumers register modules before initialization and invoke runtime
  initialize/start/shutdown. Errors propagate; lifecycle callback panics are
  converted to errors and failed initialization attempts cleanup. The
  cross-module flow exercises success, and a second integration test verifies
  start-error propagation plus cleanup.
- Files changed for runtime lifecycle integration: `src/core/runtime.rs`,
  `src/modules/mod.rs`, `tests/platform_flow.rs`, `README.md`,
  `docs/architecture.md`, and this ledger. No files were deleted.
- Modified public API files `src/legal/mod.rs`, `src/core/context.rs`,
  `src/core/runtime.rs`, and `src/lib.rs` to require a principal policy for
  sensitive legal operations. Updated `tests/core.rs`, `tests/legal.rs`, and
  `tests/platform_flow.rs`; added the shared test policy at
  `tests/common/mod.rs`.
- Added a migration-failure regression for the SQLite bot registry. An injected
  conflicting schema object forces migration to fail; the test verifies the
  schema version and identity columns remain at version 1, legacy rows remain
  present, and a clean retry completes to version 3.
- Added component-scoped schema version 1 for the legal SQLite store. Schema
  creation, indexes, immutability triggers, and the version row are now one
  immediate transaction. Existing unversioned legal tables/data are preserved;
  failed DDL rolls back, negative/future versions fail closed, and retry after
  resolving a test-injected schema conflict succeeds.

### Files created, modified, and deleted

- Created: `Cargo.lock` (generated by Cargo during dependency resolution),
  `docs/DEVELOPMENT_STATUS.md`.
- Modified for documentation: `INFO.md`, `README.md`, `docs/architecture.md`,
  `docs/i18n.md`, `docs/logger.md`, `docs/services.md`.
- Modified for behavior and tests: `src/security/key_manager.rs`,
  `src/security/authenticated_provider.rs`, `tests/core.rs`, and
  `tests/authenticated_key_provider.rs`; `src/legal/mod.rs` also uses
  immediate transactions for registration and decision writes; provider
  assertion timestamp validation changed in
  `src/security/provider_authenticator.rs` and
  `tests/provider_authenticator.rs`; provider list limits changed in
  `src/security/key_manager.rs` and `tests/key_provider.rs`; legal access checks were consolidated in
  one SQL statement in `src/legal/mod.rs`; caller conversion handling changed
  in `src/events/mod.rs` and `src/i18n/mod.rs`; callback behavior is documented
  in `README.md` and `docs/architecture.md`.
- Added for persistent replay protection: `src/security/provider_replay_store.rs`
  and `tests/provider_replay_store.rs`; exported the contract and both stores
  from `src/security/mod.rs`.
- Modified by `cargo fmt --all` (formatting only):
  `src/core/config.rs`, `src/core/context.rs`, `src/core/logger.rs`,
  `src/core/mod.rs`, `src/core/runtime.rs`, `src/lib.rs`, `src/modules/mod.rs`,
  `src/security/encryption.rs`, `src/security/key_provider.rs`,
  `src/security/mod.rs`, `src/security/provider_binding_store.rs`,
  `src/security/provider_identity.rs`,
  `src/services/mod.rs`, and test Rust files other than
  `tests/core.rs`, `tests/authenticated_key_provider.rs`, and
  `tests/provider_authenticator.rs`, `src/security/provider_replay_store.rs`,
  and `tests/provider_replay_store.rs`.
- Deleted: none.

## Verification results

After formatting, the following commands completed successfully:

- `cargo fmt --all`
- `cargo check --all-targets`
- Initial baseline `cargo test` — 143 tests passed, 0 failed; doc tests: 0.
- Targeted persistence recovery test — passed.
- Targeted independent-connection withdrawal concurrency test — passed.
- Targeted provider Vault-switch regression test — passed.
- Provider assertion extreme-timestamp regression test — passed.
- EventBus/Translator panicking-conversion regression test — passed.
- EventBus callback reentrant-subscription test — passed.
- Oversized assertion and provider key-list tests — passed.
- Provider replay-store integration tests — 4 passed, 0 failed.
- Full `cargo test` after resource-limit changes — **151 passed, 0 failed**;
  doc tests: 0 passed, 0 failed.
- Full `cargo test` with SQLite replay-store implementation — **155 passed, 0 failed**;
  doc tests: 0 passed, 0 failed. `cargo check --all-targets` passed;
  `git diff --check` passed.
- Full `cargo test` after lock-poison recovery — **158 passed, 0 failed**;
  doc tests: 0 passed, 0 failed. `cargo check --all-targets` passed.
- Full `cargo test` after checked event dispatch — **159 passed, 0 failed**;
  doc tests: 0 passed, 0 failed. `cargo check --all-targets` passed.
- Full `cargo test` after legal-store concurrency and rollback tests — **161 passed, 0 failed**;
  doc tests: 0 passed, 0 failed. `cargo check --all-targets` passed.
- `git diff --check`
- Root README link audit — 1 README reviewed; all relative links resolve.
- Full `cargo test` after envelope-size limits — **163 passed, 0 failed**;
  doc tests: 0 passed, 0 failed. `cargo check --all-targets` passed.
- Full `cargo test` after adding `ModuleManager` — **166 passed, 0 failed**;
  doc tests: 0 passed, 0 failed. `cargo check --all-targets` passed.
- Full `cargo test` after global replay-store expiry cleanup — **167 passed, 0 failed**;
  doc tests: 0 passed, 0 failed. `cargo check --all-targets` passed.
- Full `cargo test` after provider error sanitization — **167 passed, 0 failed**;
  doc tests: 0 passed, 0 failed. `cargo check --all-targets` passed.
- Full `cargo test` after owner/co-owner enforcement — **179 passed, 0 failed**;
  doc tests: 0 passed, 0 failed. `cargo check --all-targets` passed.
- Targeted independent-connection owner-transfer race — one winner; persisted
  owner and one transfer audit event verified.
- Local Markdown-link scan — all relative links resolved.
- Secret-pattern scan of the tracked diff — no matches. This is a pattern
  scan, not a comprehensive secret detector.

- Full `cargo test` with persistent identity and cross-module flow — **175 passed, 0 failed**;
  doc tests: 0 passed, 0 failed. `cargo check --all-targets` passed.
- Current local verification after legal grant/withdrawal and dual-store
  recovery flow — focused `cargo test --test platform_flow`: **1 passed**;
  `cargo fmt --all`, `cargo check --all-targets`, and full `cargo test`:
  **182 passed, 0 failed**; doc tests: 0 passed, 0 failed. `git diff --check`
  passed.
- Current audit-fix verification: focused `cargo test --test bot_registry` —
  **12 passed**; `cargo fmt --all`, `cargo check --all-targets`, full
  `cargo test` — **182 passed, 0 failed**; doc tests: 0 passed, 0 failed;
  `git diff --check` passed.
- Current bot migration rollback verification: focused test passed;
  `cargo fmt --all`, `cargo check --all-targets`, and full `cargo test` —
  **183 passed, 0 failed**; doc tests: 0 passed, 0 failed; `git diff --check`
  passed.
- Current legal schema migration verification: the three focused migration
  tests passed; `cargo fmt --all`, `cargo check --all-targets`, and full
  `cargo test` — **186 passed, 0 failed**; doc tests: 0 passed, 0 failed;
  `git diff --check` passed.
- Current legal schema migration verification: 3 focused migration tests passed;
  `cargo fmt --all`, `cargo check --all-targets`, full `cargo test` —
  **186 passed, 0 failed**; doc tests: 0 passed, 0 failed;
  `git diff --check` passed.
- Current legal authorization verification after denied cross-user consent,
  denied user-data lookup, denied document registration/activation, and generic
  error checks: focused legal/core/platform flow tests passed;
  `cargo fmt --all`, `cargo check --all-targets`, full `cargo test` —
  **187 passed, 0 failed**; doc tests: 0 passed, 0 failed;
  `git diff --check` passed.
- Current runtime-owned lifecycle verification: focused callback-panic cleanup
  and cross-module runtime success/start-failure tests passed; `cargo fmt --all`,
  `cargo check --all-targets`, full `cargo test` — **189 passed, 0 failed**;
  doc tests: 0 passed, 0 failed; `git diff --check` passed.

These results verify only the code that exists. They do not verify absent bot
platform capabilities or a live external integration.

## README audit

- README files found: **1** (`README.md` at repository root).
- README files reviewed: **1**.
- README files updated for the prior replay-store checkpoint: **1**.
- README files reviewed for the lock-recovery checkpoint: **1**.
- README files with no changes for the lock-recovery checkpoint: **1**.
- README files reviewed for checked event dispatch: **1**.
- README files updated for checked event dispatch: **1**.
- README files with no changes for checked event dispatch: **0**.
- README files reviewed for legal-store resilience tests: **1**.
- README files updated for legal-store resilience tests: **0**.
- README files with no changes for legal-store resilience tests: **1**.
- README files reviewed for envelope-size hardening: **1**.
- README files updated for envelope-size hardening: **1**.
- README files with no changes for envelope-size hardening: **0**.
- README files reviewed for lifecycle coordination: **1**.
- README files updated for lifecycle coordination: **1**.
- README files with no changes for lifecycle coordination: **0**.
- README files reviewed for lifecycle cleanup refinement: **1**.
- README files updated for lifecycle cleanup refinement: **0**.
- README files with no changes for lifecycle cleanup refinement: **1**.
- README files reviewed for global replay-store expiry cleanup: **1**.
- README files updated for global replay-store expiry cleanup: **1**.
- README files with no changes for global replay-store expiry cleanup: **0**.
- README files reviewed for provider error sanitization: **1**.
- README files updated for provider error sanitization: **1**.
- README files with no changes for provider error sanitization: **0**.
- README files reviewed for bot identity registry: **1**.
- README files updated for bot identity registry: **1**.
- README files with no changes for bot identity registry: **0**.
- README files reviewed for transactional identity schema migrations: **1**.
- README files updated for transactional identity schema migrations: **1**.
- README files with no changes for transactional identity schema migrations: **0**.
- README files reviewed for cross-module flow test: **1**.
- README files updated for cross-module flow test: **0**.
- README files with no changes for cross-module flow test: **1**.
- README files reviewed for architecture wording correction: **1**.
- README files updated for architecture wording correction: **0**.
- README files with no changes for architecture wording correction: **1**.
- README files reviewed for persisted identity validation: **1**.
- README files updated for persisted identity validation: **0**.
- README files with no changes for persisted identity validation: **1**.
- README files reviewed for runtime registry injection: **1**.
- README files updated for runtime registry injection: **1**.
- README files reviewed for bot ownership and co-owner policy: **1**.
- README files updated for bot ownership and co-owner policy: **1**.
- README files with no changes for bot ownership and co-owner policy: **0**.
- README files reviewed for ownership example/roadmap correction: **1**.
- README files updated for ownership example/roadmap correction: **1**.
- README files with no changes for ownership example/roadmap correction: **0**.
- README files reviewed for co-owner code example and auth boundary: **1**.
- README files updated for co-owner code example and auth boundary: **1**.
- README files with no changes for co-owner code example and auth boundary: **0**.
- README files reviewed for owner-transfer concurrency test: **1**.
- README files updated for owner-transfer concurrency test: **0**.
- README files with no changes for owner-transfer concurrency test: **1**.
- README files reviewed for legal-consent and dual-store recovery flow: **1**.
- README files updated for legal-consent and dual-store recovery flow: **0**.
- README files with no changes for legal-consent and dual-store recovery flow: **1**.
- README files reviewed for owner-only audit-history reads: **1**.
- README files updated for owner-only audit-history reads and API example: **1**.
- README files with no changes for owner-only audit-history reads: **0**.
- README files reviewed for bot-registry migration rollback coverage: **1**.
- README files updated for bot-registry migration rollback coverage: **0**.
- README files with no changes for bot-registry migration rollback coverage: **1**.
- README files reviewed for legal schema versioning and DDL rollback: **1**.
- README files updated for legal schema versioning and DDL rollback: **1**.
- README files with no changes for legal schema versioning and DDL rollback: **0**.
- README files reviewed for the legal principal/action/resource contract: **1**.
- README files updated for the legal principal/action/resource contract: **1**.
- README files with no changes for the legal principal/action/resource contract: **0**.
- README files reviewed for legal principal enforcement across runtime/store APIs: **1**.
- README files updated for legal principal enforcement across runtime/store APIs: **1**.
- README files with no changes for that API enforcement checkpoint: **0**.
- README files reviewed for runtime-owned module lifecycle: **1**.
- README files updated for runtime initialize/start/shutdown: **1**.
- README files with no changes for runtime lifecycle integration: **0**.
- README files reviewed for transactional/versioned legal-store initialization: **1**.
- README files updated for transactional/versioned legal-store initialization: **1**.
- README files with no changes for transactional/versioned legal-store initialization: **0**.
- README files reviewed for the added Vault and WhatsApp/Telegram/Discord
  integration scope: **1**.
- README files updated for that scope: **0**.
- README files with no changes for that scope: **1**.
- README files with no changes for runtime registry injection: **0**.
- README files reviewed for lifecycle cleanup refinement: **1**; unchanged.
- README files reviewed for replay expiry cleanup: **1**, updated **1**, unchanged **0**.
- Problems found: documented JavaScript/Node project structure and `npm`
  commands did not match the Rust Cargo crate; future bot/provider and other
  systems were presented as available; repository links and component claims
  were stale; the required credit text was missing.
- Problems corrected: documented current Rust crate, actual commands and APIs,
  implemented/pending boundaries, security persistence limitations, exact
  required credits, authenticated Vault revalidation, timestamp validation,
  replay-store configuration and its SQLite persistence boundary, synchronous
  event callback behavior, explicit panic-isolated dispatch API, and provider
  input size limits. The Matterssmith
  banner/heading identity has been retained.
- Problems pending: component-by-component README examples cannot cover the
  absent subsystems; comprehensive release documentation waits for their
  implementation. No README was skipped.

## Stages and remaining work

- H: **In progress.** The initial inventory and gap classification are done;
  a confirmed provider Vault identity check gap was corrected with a
  regression test; legal duplicate-check/write sequences now use immediate
  transactions, access checks use one snapshot, and the independent-connection
  withdrawal race is covered.
  Provider assertion time arithmetic now rejects malformed extremes without
  overflow; replay tracking now supports SQLite persistence when configured.
  The default constructor remains process-local. Caller string
  conversions now run outside EventBus/Translator write locks, and reentrant
  callback behavior is tested. Provider inputs have bounded size/count.
  EventBus, ServiceRegistry and Translator now recover from poisoned internal
  locks; dedicated lock-poison tests pass. These subsystems still need broader
  concurrency review. Legacy `publish` propagates callback panics;
  `publish_checked` isolates unwinding panics and reports a dispatch error.
  The source pass found no `todo!`, `unimplemented!`, or production
  `panic!`; lock `.expect` calls were removed from production event, service,
  and translation registries. The rest of the module audit remains. A caller-owned `ModuleManager` was added with duplicate/name/version
  validation, initialization order, reverse shutdown, start-failure cleanup,
  and stop-before-start cleanup. Replay storage removes expired entries across
  scopes and has per-scope plus global capacity limits. `stop_all` also releases modules
  that initialized but never started, and retains failed-cleanup state for retry.
  `EventBus::publish_checked` continues notifying listeners after an individual
  unwinding panic and returns an error summary; default `publish` behavior
  remains backward compatible. SQLite replay storage rejects duplicate
  assertions across independent connections and verifier restarts; four focused
  tests pass. Legal store tests cover concurrent idempotent document
  registration and rollback after a simulated SQLite write failure. Bot identity
  registration persists family/release/instance identity and atomically resolves
  display-name collisions; owner/co-owner controls are scoped and audited.
  Audit-history reads now require the current owner. Envelope payloads
  are capped at 16 MiB before encryption or Base64 decoding. Errors
  from key, binding, and proof providers are sanitized at module boundaries; a
  regression test confirms an upstream secret-like diagnostic is not returned.
- I: **Not complete.** Added runtime legal-store close/reopen recovery,
  changing-provider-identity tests, and a cross-module identity/runtime/service/
  event/module lifecycle flow including legal grant/withdrawal and recovery of
  bot identity plus legal consent after reopening SQLite. Broad
  authorization/socket/service fault-injection tests remain blocked by absent
  subsystems.
- J: **Not complete.** Existing security-focused tests pass; general
  authorization, socket/plugin isolation, operational auditing, and an
  independent security review remain unavailable or absent.
- K: **Not complete.** Rust contracts and envelope schema exist; public API
  compatibility/versioning and language-neutral contracts are not defined.
- L: **Not complete.** The listed local checks passed, but H–K acceptance,
  dependency/license/secret review, integration coverage, and release criteria
  remain outstanding.
- Matter-Bot: **Blocked by unmet Matter-Base criteria. Do not start.**

Checkpoint `b788929235653a829f007f1ef38407b1d39ff4a9` bounded replay storage across scopes. Checkpoint `5c2b1db6e1c082212892852e20924b44aedac763` refined module cleanup.
Checkpoint `cd351b5c2267fb5ace70fa51b163a2dcb64b07ab` added the module lifecycle
coordinator. Earlier checkpoint `ba100c45433f145d9863e8aeffbb5bce29775b18`
added 16 MiB envelope payload bounds; `e48ce509add1e08eaed63dfda3917b3f5d14accd`
added checked event dispatch; `9c77cff43889028606eb384afc708eab0e597ed2`
added legal-store concurrency/rollback tests. Earlier checkpoints added lock
recovery and persistent replay storage. These commits were pushed to
`origin/matter-beta` over authenticated SSH and confirmed by fetch. No promotion to `matter` was made. Matter-Vault live
verification is the only specifically identified external dependency; most
remaining blockers are missing code and contracts in this repository, not
external services.

## Latest checkpoint (local, not yet published)

- Branch: `matter-beta`; after safe stash, fetch, reset, and stash reapply,
  both `HEAD` and `origin/matter-beta` were
  `b9ca1d0e46c3f63428d9545da7e9923f6b86b129`. No conflicts; ignored
  `target/` was preserved.
- Added a module shutdown retry test and a cloned-runtime concurrent-start
  integration test. They verify retained cleanup state and serialized lifecycle
  callbacks.
- Files changed: `src/modules/mod.rs`, `tests/platform_flow.rs`, and this
  ledger. No files created or deleted. README audit: 1 found, 1 reviewed,
  0 updated, 1 unchanged. No README issue applies to these test-only changes.
- Verification passed: `cargo fmt --all`, `cargo check --all-targets`,
  `cargo test` (191 tests, 0 failed; 0 doc tests), and `git diff --check`.
- This checkpoint is not yet committed or pushed. Update this record with its
  published SHA after the push and fetch confirmation.

## Exact next task to resume

1. Finish the resilience checkpoint: inspect the full diff, commit to
   `matter-beta`, push, fetch, and confirm that local and remote SHAs match.
   Do not promote to `matter`.
2. Continue H by auditing legal authorization denial/error paths and remaining
   provider, key, and bot-registry corruption cases. Add tests for confirmed
   defects.
3. Implement persistent shared application users with explicit, authorized
   external-identity links and bot-scoped user data. Authentication must enter
   through an explicit trusted-principal contract; never infer identity links
   from matching platform identifiers. Enforce bot scope at data access
   boundaries and verify persistence, validation, concurrency, and recovery.
4. Continue I with cross-module resilience around runtime, bot ownership,
   provider failures, legal-store errors, and the user registry. The current
   suite has 191 tests; router, sockets, plugins, general resource
   authorization, and live services remain uncovered.
5. Complete J, K, and L in order. Reassess capabilities from actual code and
   keep Matter-Bot blocked until all readiness criteria are satisfied.
6. At each checkpoint, review every README, update this ledger, commit and push
   to `matter-beta`, then fetch and verify the exact published SHA. Never claim
   Vault or platform integrations work without real verification.

## Additional requested ecosystem scope

The user added the following required scope; it is a goal, not an implemented
or verified capability:

1. Complete Matter-Base stages H–L and meet the existing readiness criteria.
2. Integrate Matter-Vault through its independent repository/service contract
   and verify a real integration when that service is available. Matter-Base
   must not store platform credentials or WhatsApp sessions.
3. Build and verify a functional WhatsApp adapter in Matter-Baileys, including
   account linking (QR/code where supported), receive/send, filters, activation
   controls, instance isolation, reconnect, and persistence. Live verification
   must use a secondary test account; do not overwrite existing sessions.
4. Then add Telegram (Matter-Telegraf) and Discord (Matter-Discord) adapters
   with equivalent capabilities where each platform supports them, against
   Matter-Base's stable platform-neutral contracts.
5. Keep provider-specific logic in the adapter repositories. Do not start
   adapter development before Matter-Base is ready, and do not claim a live
   integration based only on mocks.

These repositories and services must each be inspected for their real branch,
state, instructions, and API before work. At this checkpoint the only verified
repository is Matter-Base; the availability and state of Matter-Vault and the
three adapter repositories have not yet been inspected.

For later work, re-fetch the development branch before verification or code
changes, using the safe stash/fetch/reset/reapply procedure from the project
instructions. Never infer publication from local commit state alone.

Bot identity/ownership implementation in checkpoint `1a4750ca90572701192451d9cd635d6c3e9eb495`: `src/bots/mod.rs`,
`src/lib.rs`, and `tests/bot_registry.rs`, plus cross-module behavior in
`tests/platform_flow.rs`. The registry provides memory and SQLite stores,
validation, generated instance IDs, atomic unique names, persistence recovery,
transactional component-scoped schema migrations, future-version rejection,
cross-connection concurrency, owner-only changes/deletion, bounded co-owner
authority, legacy owner recovery hooks, persistent ownership audit records, and a
  separate-connection owner-transfer race with one winner.
Owner/co-owner operations use point-of-write authorization and transactional
audit records; legacy unowned records require an injected recovery authorizer.
Owner IDs must come from an authenticated application principal; this crate does
not authenticate platform users or verify owner IDs against a shared user store.
Component-scoped schema versioning is transactional and rejects unsupported
future versions. The full suite passed
with 180 tests; `cargo check --all-targets` and `git diff --check` passed. The
README audit found 1 README, reviewed 1, updated 1, unchanged 0; all relative
links resolve.
Owner/co-owner checkpoint `1a4750ca90572701192451d9cd635d6c3e9eb495` was pushed to
`origin/matter-beta`; a subsequent fetch confirmed the same SHA. No promotion
to `matter` occurred.
