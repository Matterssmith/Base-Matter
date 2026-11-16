# Matter-Base development status

This is a development ledger. GitHub remains the source of truth. Its
synchronization entry records the published base of each working checkpoint;
verify the branch log for the current published commit.

## Recovery snapshot

- Repository: `/home/Sakura/Matter-Base`
- Working branch: `matter-beta`
- Previous synchronized base commit: `530efb81c645bd7c31e70abd7066048d947bfa73`
- Latest confirmed checkpoint commit: `0cc310f5f3483fc4ba3edd58175d8121f008e72c`
- Remote: `origin` (`https://github.com/Matterssmith/Matter-Base.git`)
- Synchronization: `git fetch origin` succeeded before and after the checkpoint
  push; local `HEAD` matched `origin/matter-beta` at
  `0cc310f5f3483fc4ba3edd58175d8121f008e72c`.
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

## Capability assessment from the synchronized code

| Capability | Status | Evidence / limitation |
| --- | --- | --- |
| Runtime and lifecycle | PARTIAL | Runtime opens the legal store and logs initialization; no coordinated startup/shutdown lifecycle. |
| Bot identity, family/release/instance IDs, registration | ABSENT | No implementation in `src/`. |
| Shared application users and bot data isolation | ABSENT | `legal_users` are legal-consent subjects, not a shared application user system. |
| Sessions and platform credential contracts | ABSENT | No session/credential subsystem; platform credentials must remain outside this crate. |
| Persistence and recovery | PARTIAL | SQLite persistence exists for legal documents/decisions only. No bot/config/socket/service persistence or migrations framework. |
| Events and messaging | PARTIAL | Named synchronous in-memory event callbacks exist; no durable events or messaging contract. |
| Router and command registry | ABSENT | No implementation. |
| Middleware, roles, permissions, authorization | ABSENT | Provider key permissions exist only in the security integration; no general resource authorization. |
| Control Plane, sockets, activation | ABSENT | No implementation; socket ownership/policies cannot be verified. |
| Services and connections | PARTIAL | Typed in-memory service registry only; no connection manager or lifecycle. |
| Plugins, extensions, sandbox | ABSENT | No manifest validation, isolation, or resource limits. |
| Application audit log | ABSENT | Legal decisions are append-only; no general administrative audit system. |
| Encryption/key/provider contracts | PARTIAL | Primitives and injection contracts have tests; no production key service integration or independent security audit. |
| Provider assertion replay protection | PARTIAL | Replay IDs are bounded and tracked only in one in-memory authenticator; no restart or multi-process replay store. |
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
  runtime and verifies user, consent history, and access survive.
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
- Added this continuity record.
- Preserved the exact project credit in the root README.

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
- Modified by `cargo fmt --all` (formatting only):
  `src/core/config.rs`, `src/core/context.rs`, `src/core/logger.rs`,
  `src/core/mod.rs`, `src/core/runtime.rs`, `src/lib.rs`, `src/modules/mod.rs`,
  `src/security/encryption.rs`, `src/security/key_provider.rs`,
  `src/security/mod.rs`, `src/security/provider_binding_store.rs`,
  `src/security/provider_identity.rs`,
  `src/services/mod.rs`, and test Rust files other than
  `tests/core.rs`, `tests/authenticated_key_provider.rs`, and
  `tests/provider_authenticator.rs`.
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
- Full `cargo test` after resource-limit changes — **151 passed, 0 failed**;
  doc tests: 0 passed, 0 failed.
- `git diff --check`
- Local Markdown-link scan — 7 Markdown files scanned, all relative links
  resolved.
- Secret-pattern scan of the tracked diff — no matches. This is a pattern
  scan, not a comprehensive secret detector.

These results verify only the code that exists. They do not verify absent bot
platform capabilities or a live external integration.

## README audit

- README files found: **1** (`README.md` at repository root).
- README files reviewed: **1**.
- README files updated for this checkpoint: **1**.
- README files with no changes: **0**.
- Problems found: documented JavaScript/Node project structure and `npm`
  commands did not match the Rust Cargo crate; future bot/provider and other
  systems were presented as available; repository links and component claims
  were stale; the required credit text was missing.
- Problems corrected: documented current Rust crate, actual commands and APIs,
  implemented/pending boundaries, security persistence limitations, exact
  required credits, authenticated Vault revalidation, timestamp validation,
  process-local replay tracking, synchronous event callback behavior, and
  provider input size limits. The Matterssmith banner/heading identity has
  been retained.
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
  overflow; replay tracking remains process-local and volatile. Caller string
  conversions now run outside EventBus/Translator write locks, and reentrant
  callback behavior is tested. Provider inputs have bounded size/count.
  These subsystems still need broader concurrency review. Callback panics
  propagate; no fallible callback contract exists.
  The first source/test pass found no `todo!`, `unimplemented!`, or production
  `panic!`, but did find lock `.expect` calls. The rest of the module audit
  remains.
- I: **Not complete.** Added runtime legal-store close/reopen recovery and
  changing-provider-identity tests. Broad bot/authorization/socket/service
  integration and resilience tests cannot pass until those subsystems exist.
- J: **Not complete.** Existing security-focused tests pass; general
  authorization, socket/plugin isolation, operational auditing, and an
  independent security review remain unavailable or absent.
- K: **Not complete.** Rust contracts and envelope schema exist; public API
  compatibility/versioning and language-neutral contracts are not defined.
- L: **Not complete.** The listed local checks passed, but H–K acceptance,
  dependency/license/secret review, integration coverage, and release criteria
  remain outstanding.
- Matter-Bot: **Blocked by unmet Matter-Base criteria. Do not start.**

Checkpoint `0cc310f5f3483fc4ba3edd58175d8121f008e72c` was pushed to
`origin/matter-beta` over authenticated SSH and then confirmed by `git fetch`.
The next H/J audit corrections are being prepared for a separate checkpoint.
No promotion to `matter` was made. Matter-Vault live verification is the only
specifically identified external dependency; most remaining blockers are
missing code and contracts in this repository, not external services.

## Exact next task to resume

1. Re-read this file and the root README.
2. Confirm the current branch, status, and remote tip. Preserve local work;
   do not discard `Cargo.lock` or `target/` without inspecting ownership.
3. Continue H by auditing and testing the existing implementations, beginning
   with event/service/i18n lock-poisoning and callback behavior, legal-store
   transaction/recovery behavior, and security-provider boundaries. Make
   targeted compatible corrections with behavior tests.
4. Run the complete verification sequence after the above targeted tests, then
   update the recorded results here. (Completed for this snapshot: 151 tests
   passed.) Continue auditing SQLite operation error paths, then
   events/services/i18n lock and callback behavior. Add tests and make
   compatible fixes where defects are confirmed.
5. Update this record after each completed stage. Do not mark H–L complete
   until the corresponding criteria are implemented and verified.

For later work, re-fetch the development branch before verification or code
changes, using the safe stash/fetch/reset/reapply procedure from the project
instructions. Never infer publication from local commit state alone.
