# Matter-Base development status

## 2026-11-17 — Reserved socket status control (pending CI)

- Built `=/>status [SOCKET_ID]` as a socket-owner-only introspection
  operation independent of Bot-Matter and available when bot is off/unbound.
- Reads policy, source and optional bot binding in a single stored admission
  snapshot. Never treats activated power as proof of network connectivity.
- Added three regressions covering power on/off, unauthorized actors,
  cross-socket targets, bot bindings and rejection of `=/>ping`.
- README audit: 1 root README reviewed and updated; activation, pairing
  contracts and stage status reviewed/updated. No actual WhatsApp
  credentials, session files or QR flow introduced.
- Stage I remains in progress; external pairing/connection and Vault
  integration are not complete. This checkpoint awaits CI validation.
  `matter` stable and Bot-Matter untouched.


## 2026-11-17 — Owner correction: pairing-code-only and global control boundary (pending CI)

- Owner clarifies the 22+ \`Matter\` repos form one ecosystem, but Bot-Matter
  must NOT receive development until Base-Matter is complete.
- Historical WhatsApp connection and ping verification are acknowledged.
  Current beta tree has no executable provider implementation. Do not confuse
  previously tested server work with a current live integrated GitHub build.
- WhatsApp pairing is strictly through a code derived from the operator's
  authorized phone number; QR is excluded. The previous ping is a test-only
  behavior and will not replace Base's global prefix \`=/>\`.
- Added PairingCodeService contract in Base with per-socket owner/host policy,
  phone and transient-code validation and zeroizing/redacted return.
  It has no QR API, no network adapter, no credential persistence, no
  automatic reconnect and no ping handler. Provider-specific implementation
  and authenticated end-to-end tests remain open.
- Added five tests covering access control, provider/code failure, session
  eligibility and global prefix isolation without bot binding.
- Existing legacy alias prefixes remain supported. All changes only in
  matter-beta; matter stable and Bot-Matter are untouched.
- README audit: 1 README reviewed/updated; AGENTS, architecture, session docs
  and release-readiness updated. CI for this checkpoint pending.


## 2026-11-17 — Verified stable promotion, naming and WhatsApp beta

- Promoted only standalone Rust lint/security/test hardening, MIT LICENSE,
  Cargo.lock and Linux/Windows CI from stable candidate through PR #3 into
  \`matter\`, merge SHA \`e509cc4f8c53eafbb1233359bc2ae63e0dde7f48\`.
  The post-merge stable run 44139981479 passed on Ubuntu and Windows.
- \`matter-beta\` retains incomplete integrations and diverged feature work;
  it was not mass-merged into stable. Last named beta CI run 44138531259
  passed; no older local server files were touched.
- Matter-Baileys has a separate matter-beta branch for the future
  WhatsApp-Matter adapter. Node dependency resolution, 8 offline test cases
  and upstream import check passed run 44139262152. This cannot establish a
  live WhatsApp connection or authorize production credentials/groups.
- Repo naming: 22 remotes require GitHub administrative rename; no repository
  rename was executed through this connector. Safe gh CLI rename plan and
  docs/ECOSYSTEM_RENAME.md are committed in beta.
- README audit: one root README inspected and updated. This checkpoint is
  documentation-only, with CI validation to follow.
- Stage I integration/resilience remains active. H audit closure and J–L
  release gates remain open. Do not start Matter-Bot until Base is ready.


## 2026-11-17 — Reversed ecosystem names and WhatsApp provider smoke

- User requested '<Component>-Matter' names across the Matterssmith
  ecosystem. Inventoried 22 prefixed repositories. Added
  docs/ECOSYSTEM_RENAME.md and safe Python gh CLI migration planner;
  default mode only previews, and --apply requires --only or --all.
- GitHub connector lacks repository rename administration, so no actual
  remote repository name has changed. The existing Beta and stable URLs
  remain authoritative until a separately authorized GitHub CLI rename.
- Provider repository Matterssmith/Matter-Baileys now has its own
  matter-beta branch with offline-tested Baileys QR/pairing-code smoke,
  fail-closed allowlisted #ping and seven tests (GitHub run 44138049692
  successful). No live QR pairing or authenticated Base bridge proved.
- Root README audited, reviewed and updated (1/1). AGENTS updated to the
  latest naming convention and strict verified-subset release policy;
  Rust package/crate names retained until compatibility validation.
- Added unit tests for all 22 proposed names. Stage I integration continues;
  Vault authentication, real provider binding, sandbox isolation,
  cross-store transitions and Termux remain unverified.


## 2026-11-17 — Eliminate Windows test-directory collision (pending CI)

- Windows run 44126198795 exposed a flaky duplicate temporary directory
  name in tests/control.rs. Clock-based nanosecond identifiers are not
  reliably unique on Windows under parallel test execution.
- Replaced process/time-derived directory IDs with 128-bit randomized IDs
  in tests/control.rs, tests/activation.rs, tests/platform_flow.rs,
  tests/bot_registry.rs and tests/core.rs. This changes test fixture
  allocation only, not production state or behavior.
- README audit: one root README fully reviewed, unchanged, no new broken
  links or design discrepancy. CI on Linux and Windows must pass again.
- Real Vault integration, signed platform adapters, isolated sandbox,
  crash-recoverable owner coordination and Termux verification remain open.
  Stable matter and Matter-Bot remain unchanged.


## 2026-11-17 — Verified beta checkpoint and actual remaining gates

- Verified GitHub Actions run **44125657053** on exact beta SHA
  `d4dcc7791fdf1641de74e755a2edef81f87531b8`:
  **success** on Ubuntu and Windows. Jobs included locked compilation,
  unit/integration tests, strict Clippy, formatting, docs, README link/credit
  checks, tracked-secret-name audit and whitespace.
- New since earlier stage H: scoped role authorization, owner handover,
  reserved set/unset role controls, durable session metadata, outgoing
  activation/role admission, connection/credential contracts, encrypted outbox
  and lost-ack reconciliation, plugin trust/sandbox interface and trusted
  inbound gateway/replay claims. These are verified local contracts, not
  evidence of live provider or Vault integration.
- `matter` stable remains at
  `c1ad3498381ba9f1f18d4aef3cf702686dcd285c`.
  Matter-Bot remains blocked. No changes to real user-local worktrees,
  ignored sessions, production keys, Vault or stable branch were made.
- README audit for this documentation-only commit: one README found, fully
  reviewed, **0 updated, 1 unchanged**. Its current project status and
  boundaries remain accurate; release-readiness document updated.
- **Critical unverified work:** authenticated private Vault key/remote
  publisher and restart/restore integration; real signed/authenticated
  WhatsApp, Telegram and Discord adapters and test accounts; an independently
  tested OS/WASM sandbox; coordinated atomic or crash-recoverable bot/socket/
  role ownership handover; Termux/Android tests and final security threat
  model/migrations. External secrets, provider test accounts and isolation
  infrastructure are unavailable in this GitHub-only execution context.
- Do not mark H–L or Matter-Base as complete. Next session must inspect actual
  beta head/CI status and continue from these exact missing integrations,
  without repeating passing modules or touching Matter-Bot.


## 2026-11-17 — Authenticated ingress and replay contract (pending CI)

- Previous Windows SQLite-handle fix passed GitHub Actions run 44124628703
  on both Ubuntu and Windows (compile, tests and strict Clippy).
- Added InboundGateway requiring external authenticated provider identity and
  mandatory authorization/consent policy before ControlDispatcher.
- Added bounded SQLite HMAC-keyed replay claims per verified platform/socket
  and provider event ID, without storing message text, raw IDs or replay key.
- Five regressions cover power gating, duplicate handling, denial, provider
  mismatch, durable replay isolation and invalid/oversized input.
- README audit: one root README reviewed and updated; ingress and architecture
  docs added. CI for this phase is pending.
- Production webhook signature verification, platform identity mapping and
  replay-key rotation still require real adapter validation.
  Stable matter/Matter-Bot untouched; H–L remain open.


## 2026-11-17 — Windows SQLite file-handle regression (pending CI)

- Run 44124073091: Linux checks passed, Windows compilation passed, but two
  tests in tests/users.rs failed with Windows OS code 32 because test-owned
  UserRegistry SQLite connections remained open during remove_file.
- Corrected both tests by explicitly dropping registries before deleting
  temporary database files, preserving production behavior and assertions.
- README audit: 1 root README found/reviewed/updated, with cross-platform
  verification limitations retained. Re-run both CI jobs before accepting.
- Real Vault/provider/sandbox/Termux tests remain open; matter and Matter-Bot
  unchanged.


## 2026-11-17 — Windows Rust CI lane (pending CI)

- Prior release and tracked-file hygiene passed GitHub Actions run 44123810438.
- Expanded matter-beta workflow with a Windows stable Rust lane running
  locked all-target compilation, unit/integration tests and strict Clippy.
- README audit: one root README reviewed and updated; release-readiness
  notes updated to distinguish Windows CI from missing Termux/platform tests.
- A passing result on Windows is pending. No production readiness claim,
  stable branch merge, Matter-Bot work or local session modifications.


## 2026-11-17 — Release hygiene and evidence gates (pending CI)

- Previous encrypted lost-ack reconciliation passed GitHub Actions run
  44123443517.
- Added MIT LICENSE matching Cargo.toml's declared license, without altering
  upstream provider licensing or incorporating external provider code.
- Added scripts/verify_repository.py to check all tracked README credits,
  relative links, tracked LICENSE presence and sensitive tracked filenames.
- CI now runs the tracked-file/document audit alongside Rust checks; new
  docs/RELEASE_READINESS.md lists independently verifiable release blockers
  instead of treating interfaces/mocks as completed integrations.
- README audit: one root README reviewed and updated; its relative links
  are intended to be checked in CI. Full release readiness is NOT declared.
- Test results for this commit await GitHub Actions. Stable matter and
  Matter-Bot unchanged.



## 2026-11-17 — Remote encrypted acknowledgment reconciliation (pending CI)

- Prior connection lifecycle checkpoint passed GitHub Actions run 44122884865.
- Added EncryptedEnvelopeReader and fail-closed outbox reconcile operation,
  verifying full envelope identity, metadata and ciphertext before updating
  the local revision/sync flag in the same transaction as immutable audit.
- Three regressions cover lost network acknowledgments, remote divergence,
  missing entries and ciphertext tampering. No retry or remote overwrite on
  conflict; no plaintext, token or master key stored in GitHub.
- README audit: one root README reviewed and updated; Vault docs updated.
  Real authenticated GitHub sink/source and full remote import remain open.
- This checkpoint awaits CI. No work on stable matter or Matter-Bot.


## 2026-11-17 — Connection lifecycle and credentials contract (pending CI)

- Previous scope-owner handover is verified by GitHub Actions run 44122169726.
- Added ConnectionService orchestrating connection state via SessionRegistry,
  mandatory authenticated ConnectionAdmission, zeroizing credential source,
  and external ConnectionDriver for connect/disconnect/liveness.
- Five deterministic regression tests cover owner checks, transient credentials,
  connection failures, health faulting, recovery and versioned transitions.
- No provider credentials, login sessions, master keys or network adapters
  are added to Matter-Base. Driver implementations require real external
  generation fencing and credential storage.
- README audit: one root README reviewed and updated; session/architecture
  contracts updated. CI for this checkpoint is pending.
- H–L remain in progress; independent provider integration and real isolated
  plugin execution are unverified. Stable matter untouched, Matter-Bot blocked.



## 2026-11-17 — Role scope owner handover (pending CI)

- Previous plugin contract checkpoint passed GitHub Actions run 44102970673.
- Added explicit RoleStore::transfer_scope_owner; caller must be the previous
  role scope owner and target must already be current owner in BotRegistry.
  Scope update and audit share one SQLite transaction, with fail-closed
  target checks and idempotent same-owner operation.
- Two regressions cover incorrect targets, revoked old administration, new
  owner-only history and audit-trigger rollback/recovery.
- README audit: 1 total, 1 reviewed and updated; corrected stale roadmap
  and documented cross-store non-atomic owner handover.
- This phase's CI is pending. Lost-owner recovery still requires a distinct
  authenticated authority; no production-atomic bot/role transfer is claimed.
  H–L remain open. Matter-Bot and stable matter remain unchanged.



## 2026-11-17 — Plugin trust and sandbox contracts (pending CI)

- Previous encrypted control-outbox checkpoint passed GitHub Actions run
  44101946067.
- Added plugin manifests with bounded parsing, traversal rejection,
  deterministic directory loading and duplicate-name protection.
- Added mandatory PluginTrust hooks for publisher and invocation checks,
  plus PluginSandbox contract. Default DenyPluginSandbox never executes
  untrusted code. Four deterministic tests cover manifest/path checks,
  policy rejection, canonical commands and invalid batches.
- README audit: one root README reviewed and updated; new docs/plugins.md
  and architecture map the actual boundary. No platform executable imported.
- CI for this checkpoint is pending. No production process isolation,
  external signature verification or runtime sandbox has been verified.
  H–L remain open; stable matter and Matter-Bot unchanged.


## 2026-11-17 — Encrypted control outbox (pending CI)

- Previous reserved role-control checkpoint passed GitHub Actions run
  44100755985.
- Added EncryptedControlOutbox with authenticated XChaCha20-Poly1305 envelopes,
  key-bound HMAC change fingerprints, SQLite pending ciphertext, immutable
  audit, compare-and-swap sink revision and restart-safe pending status.
- Added RoleStore::export_snapshot with one SQL snapshot, stable ordering
  and owner authorization; transient plaintext is zeroized after encryption.
- Regression tests cover encrypted-only persistence, retry on sink failure,
  role scope authorization and key-rotation resubmission.
- README audit: one README fully reviewed and updated; Vault docs clearly
  distinguish the contract from absent real GitHub and authenticated key
  service adapters. No key material, secret tokens or platform sessions added.
- CI validation for this checkpoint is pending. Cross-process conflicts,
  acknowledgement loss and remote recovery still need explicit reconciliation.
  H–L are not complete, stable matter unchanged, Matter-Bot blocked.


## 2026-11-17 — Reserved role control (pending CI)

- Previous outbound message-control integration passed GitHub Actions run
  44099969055.
- Added optional set/unset role operations on all reserved control prefixes,
  requiring current socket, bot and authorization-scope ownership.
- Added tests for powered-off operation, unauthorized actors, unknown roles,
  idempotence, revocation, missing role configuration and bot transfer.
- README audit: one root README reviewed and updated, plus activation docs.
  Cross-store owner-transfer serialization remains a necessary host contract.
- This checkpoint awaits CI verification. H–L are not fully complete.
  Stable matter and Matter-Bot remain untouched.



## 2026-11-17 — Outbound control-plane integration (pending CI)

- Previous session contracts passed GitHub Actions run 44099305219.
- Added ControlPlaneMessageAdmission bridging existing socket activation,
  group precedence, bot binding, current bot owner, matching session owner,
  persistent role rules and mandatory app-supplied legal/provider policy.
  A disconnected/off/unbound/revoked socket fails before transport.
- OutboundMessage now declares optional group_id explicitly; this avoids
  treating private destinations as group-policy resources. This is a pre-1.0
  API addition without a compatibility guarantee.
- Three integration tests exercise powered-off, group denial, role revocation,
  external policy, session owner mismatch and binding recovery.
- README audit: one README reviewed and updated; documented cross-store
  admission limitations. No secret data or platform sessions are persisted.
- New integration CI is pending; external providers and exactly-once semantics
  remain unavailable. H–L remain open; no Matter-Bot work.


## 2026-11-17 — Session state and transport contracts (pending CI)

- Previous authorization code passed GitHub Actions run 44098196776:
  formatting, all-target checks/tests, documentation tests and strict Clippy.
- Added SessionRegistry with bounded metadata, owner-only lifecycle management,
  optimistic revision checks, immutable audit and transactional rollback.
  Added MessageService, mandatory MessageAdmission policy and TransportDriver
  abstraction; it holds no provider secrets or private session data.
- Added six regression tests (restart, independent SQLite connections,
  audit rollback, admission/connection, transport errors and payload bounds).
  They exercise local contracts only, not external platform integration.
- README audit: one root README reviewed and updated. Architecture and
  docs/sessions.md describe the new boundary and actual missing providers.
- CI results for this new session checkpoint are pending.
  Keep H/I/J/K/L open; do not start Matter-Bot or modify the stable branch.


## 2026-11-17 — Scoped persistent role authorization (pending CI validation)

- Added opt-in SQLite role scopes, immutable parent inheritance, owner-only
  role/grant/rule administration, explicit deny precedence, default deny, a
  bounded immutable audit and atomic policy-plus-audit transactions.
- Added RoleCommandAuthorizer for the canonical CommandRouter permission
  check; authenticated ingress and independent socket gating remain required.
- Six integration regressions cover inheritance/deny, scope isolation and owner
  authorization, persistence/revoke, audit rollback, alias dispatch and corrupt
  role-chain refusal. They are not claims of live provider integration.
- README audit before this commit: one README fully reviewed and updated.
  New docs/authorization.md documents operational and trust limitations.
- CI validation for this new code is pending. All earlier code is preserved,
  matter remains unchanged and Matter-Bot is not started.
- Follow-up: inspect GitHub Actions, fix any build/test/lint faults, then
  extend integrations and resilience under stages H/I/J/K/L.


## 2026-11-17 — Strict CI lint remediation

- Retrieved GitHub Actions run `44095876210` and job logs: formatting,
  `cargo check --all-targets --locked`, all default tests and doc tests
  succeeded. Strict Clippy failed with exactly three diagnostics.
- Corrected `src/legal/mod.rs` by naming the existing SQLite tuple type,
  `src/security/encryption.rs` by using `div_ceil(3)`, and
  `src/users/mod.rs` by using inclusive range membership. These are
  targeted, behavior-preserving lint corrections, not a completed phase.
- Reviewed the sole root README and updated it to explain this CI gate.
  Source changes were published on `matter-beta`; no local worktree,
  stable branch, Matter-Vault or Matter-Bot files were changed.
- A successful CI run for the final commit is **not yet confirmed**.
  The previous run's passing tests do not verify these new commits.
  Check the final run and address any new diagnostics before proceeding
  with persistent resource authorization and H–L completion.
- README audit: 1 total, 1 reviewed, 1 updated, 0 unchanged.
  Full repository-wide release/security verification remains pending.


## 2026-11-17 — Remote CI groundwork (ChatGPT checkpoint)

- Continued from the actual remote `matter-beta` commit
  `665ae8c1aa2e1858993ac8e2ba29de3ced0bce7f`, without modifying
  `matter`, Matter-Bot, local worktrees, ignored sessions or Vault.
- Added `.github/workflows/rust-ci.yml` on `matter-beta`. It runs
  formatting, `cargo check --all-targets --locked`,
  `cargo test --all-targets --locked`, `cargo test --doc --locked`,
  `cargo clippy --all-targets --locked -- -D warnings`, and a parent-commit
  whitespace diff on push/PR or manual dispatch. Checkout uses depth two.
- The GitHub connector confirmed the file commits were published. **No
  successful CI run has yet been verified.** Combined commit statuses returned
  no status entries. GitHub API tools in this session cannot retrieve the
  Actions run list; do not interpret that as a passing build. These changes
  are CI groundwork, not a completed H/I/J/K/L phase.
- README audit: 1 README found and reviewed, 1 updated, 0 unchanged.
  Documented the new workflow and the unverified external Vault test.
  No source or crypto contracts were modified. Tests and Clippy were not run
  in this environment, so no test result is claimed.
- Next: confirm a successful GitHub Actions run, correct any workflow/build
  issues, then implement persistent general role/resource authorization with
  scoped owner administration, explicit deny precedence and command integration.
  Preserve prior work and audit the complete README before each commit.
  H–L remain open; Matter-Bot remains blocked.


This is a development ledger. GitHub remains the source of truth. Its
synchronization entry records the published base of each working checkpoint;
verify the branch log for the current published commit.

## 2026-11-17 — Socket handover and binding recovery

- Base: `dc9ae57d0dd6bfa982312c58a70c98f0ac9313fb`, pushed and fetched on
  `matter-beta` with identical origin SHA before this checkpoint.
- Added socket-owner-only transfer and binding release. Both atomically power
  off, remove local overrides/binding, and persist their audit. Transfer also
  changes ownership; the previous owner loses administrative/audit access.
  No global administrator bypass, bot-ownership takeover, platform credential
  transfer or implicit reactivation is introduced. Same-owner transfer and
  release of an absent binding are authorized no-ops.
- Unified dispatcher admission into one SQLite snapshot containing power,
  policy, socket owner and bot binding. This prevents old activation state from
  being combined with a new binding during concurrent handover. Previously
  admitted handlers may finish; subsequent admissions see the reset state.
- Schema 4 transactionally adds optional audit target IDs while preserving
  historical rows as NULL targets. New binding/release events name the bot;
  transfer names the recipient. Public audit records now expose actor/target.
  Strict decoding rejects corrupt persisted audit boolean values.
- Added five tests: complete handover and revoked-bot binding recovery through
  the dispatcher, independent-connection transfer race with one winner,
  complete rollback on audit failure, legacy v3 audit migration, and an
  in-flight handler overlapping transfer without admitting the new binding
  while off. Other sockets and bot ownership remain independent.
- Verification passed: `cargo fmt --all`, `cargo check --all-targets`, focused
  activation/control tests (19 passed), full `cargo test` (239 passed, 0 failed,
  1 opt-in Vault test ignored; 0 doc tests), and `git diff --check`.
- README audit: 1 total, 1 reviewed in full, 1 updated, 0 unchanged. Updated
  transfer/recovery, explicit reactivation and snapshot claims; documented
  schema/API compatibility in activation contracts. Examples match exercised
  APIs, relative links and credits are correct. Full diff/file inventory and
  secret-pattern scan reviewed; no new/deleted files or secret matches.
  Lost-owner recovery and actual platform/network operations remain pending.
- Next independent prerequisite: persistent bounded role/resource authorization
  with owner-scoped administration, role inheritance, deny precedence and command
  integration. Continue H/I/J implementation and tests; external Vault verification
  remains unavailable, but does not stop this local work. H–L remain open and
  Matter-Bot is not started.

## 2026-11-17 — Persistent global activation policy

- Resumed from verified `7ef49de36cdeae03446c8230bffe7a6aee76e18f` on
  `matter-beta` after fetch/switch/reset with a clean tree. The existing stash,
  ignored build output and stable branch were preserved.
- Added trusted one-time global-policy administrator provisioning, owner-only
  policy mutation and bounded owner-only audit pages. Provisioning and policy
  changes commit with their audit rows; audit failure rolls everything back.
- Schema `socket_activation=3` upgrades versions 1/2 transactionally while
  preserving existing socket state and bindings. No administrator is inferred.
- `resolve_persisted` and `ControlDispatcher::new_with_persistent_policy` read
  power plus Group > Socket > Global > deny in one SQLite snapshot per normal
  admission. Independent-connection changes become visible without rebuilding
  the dispatcher. Existing static-global APIs retain their previous behavior.
  Global administration grants no authority over another owner's socket power
  or overrides. Missing global policy denies fallback; corrupt values fail closed.
- Added five regressions for restart recovery, precedence and isolation,
  authorization/pagination, provisioning and mutation rollback, corrupt global
  values, concurrent writes/audit order, migration failure/retry, and actual
  dispatcher behavior with live policy changes. No external service is mocked
  or represented as verified by these tests.
- Checks passed: `cargo fmt --all`, `cargo check --all-targets`, focused
  activation/control tests (14 passed), full `cargo test` (234 passed, 0 failed,
  1 opt-in Vault test ignored; 0 doc tests), and `git diff --check`.
- README audit: 1 found, 1 fully reviewed, 1 updated, 0 unchanged. Added actual
  persisted-global API use and corrected roadmap scope; examples match compiled
  tests, relative links resolve and credits remain intact. Full diff and file
  inventory reviewed; no new/deleted files or secret-pattern matches. Remaining
  documentation gaps correspond to unimplemented systems, not this policy flow.
- Next: socket ownership transfer and owner-authorized binding recovery with
  atomic power-off, isolation and audit, then general resource authorization.
  Continue independent local prerequisites while the external Vault key/binding
  implementation and authenticated access contract remain unavailable. H–L
  are still open; no stable promotion or Matter-Bot work occurred.

## 2026-11-17 — Durable command audit integration

- Continued independent Base work after publishing and fetching control fix
  `402f1bec2432e6646e89242f57ee944d4b5eed0b`; local HEAD and origin matched.
  Worked only on `matter-beta`, preserving the legacy stash and ignored files.
- Added `CommandAuditStore`, owner-scoped bounded history, immutable SQLite
  attempts/outcomes and transactional schema version `command_audit=1`.
  `CommandRouter::new_with_audit` persists recognized canonical commands before
  callbacks and records authorization, middleware and handler results, including
  unwinding panics. Default `new` preserves its existing unaudited behavior.
- Audit start failure prevents callbacks; outcome-write failure returns a
  sanitized error and leaves a durable Pending attempt. Handler effects cannot
  be undone by this store. No automatic retry or exactly-once claim is made.
  Arguments, message bodies, responses and callback diagnostics are omitted.
- Added six integration tests covering callback failures/panics, SQLite start
  and completion failures, recovery, foreign/current-owner read authorization,
  ownership revocation, bounded pagination, canonical alias logging, payload
  omission, concurrent independent connections, immutable records (including
  replacement inserts), migration rollback and future-version rejection.
  One flow integrates actual persistent bot/activation/audit stores with the
  router and ControlDispatcher: off blocks normal commands before audit;
  power changes retain their separate atomic activation audit.
- Files created: `src/command_audit.rs`, `tests/command_audit.rs`,
  `docs/command-audit.md`. Modified router/public exports, README, architecture
  and this ledger. No dependencies, cryptographic formats or external
  repositories changed; no files deleted.
- Verification: `cargo fmt --all`, `cargo check --all-targets`, focused audit
  tests (6 passed), full `cargo test` (229 passed, 0 failed, 1 opt-in Vault test
  ignored; 0 doc tests), and `git diff --check` passed. The explicit Vault
  metadata test passed in the preceding checkpoint; no live-source verification
  has become available.
- README audit before commit: 1 total, 1 fully reviewed, 1 updated, 0 unchanged.
  Corrected previously absent audit claims, described the opt-in constructor,
  failure/recovery semantics and bounded owner-scoped reads, and linked the
  new contract. Existing examples and new API use match the compiled tests;
  internal links resolve, project identity/credits are preserved, full diff
  and all new files were reviewed, and secret-pattern scans found no matches.
  The scan is not an independent security audit. Broader ingress auditing,
  archival/retention operations and release documentation remain pending.
- H/I progress is verified at this scope; no phase is declared complete.
  The Vault implementation/access-contract question remains pending. A real
  source is needed to implement and verify authenticated key/binding reads,
  failure handling, rotation and recovery. Its metadata repository alone
  cannot fulfill that dependency.
- Exact next local work: persist authorized global activation policy and its
  audit without granting global owners power over sockets; add socket ownership
  recovery/transfer and general resource authorization. Then complete
  session/credential and connection/messaging contracts and implementations,
  effective plugin isolation, cross-module resilience, security review and
  public compatibility/release criteria. Keep Matter-Bot blocked and do not
  substitute isolated tests for an external integration. Read this latest
  entry and synchronize safely on resume; historical entries below are retained.

## 2026-11-17 — Resumed control audit and Vault prerequisite

- Synchronized `matter-beta` with `origin/matter-beta` at
  `cec9df539f5031af9381564545c0abce7880ccc8`. The working tree was clean;
  no new stash was necessary. The existing legacy bot-registry stash and
  ignored `target/` were preserved. Root `AGENTS.md` applies; no nested
  instructions were found. No local stable-branch changes were made.
- Recovered previously unrecorded progress from that backup commit: durable
  bot/socket bindings, ControlDispatcher, schema-v1 upgrade coverage, ecosystem
  naming instructions and the opt-in real Vault repository metadata test.
  These were inspected and retained, not reimplemented.
- Confirmed and corrected an admission-order defect: bot binding/ownership
  checks ran before reserved power commands, preventing a socket owner from
  powering off after bot ownership transfer or before binding. Controls now
  check the target socket owner directly through the transactional activation
  store. Normal messages check power/policy, binding and current bot authority
  before reaching the router. A new bot owner/global owner gains no authority
  over an independently owned socket. No network connection is needed to
  change operational power. Admission remains snapshot-based.
- Added regressions for all three reserved prefixes, unbound control, revoked
  bot authority, foreign/global owner denial and absence of handler effects.
  Added independent-connection concurrent power/audit recovery coverage and
  binding-audit failure rollback, idempotent binding and conflicting rebind
  rejection. Existing Group > Socket > Global > deny tests remain passing.
- Verified: `cargo fmt --all`, `cargo check --all-targets`, focused control and
  activation tests (9 passed), full `cargo test` (223 passed, 0 failed,
  1 opt-in Vault test ignored; 0 doc tests), and `git diff --check`.
  Explicit `MATTER_VAULT_CHECKOUT=/home/Sakura/Matter-Vault cargo test --test
  vault_repository -- --ignored` also passed (1 test). Formatting normalized
  the preexisting Vault test; no encryption format changed.
- README audit: 1 total, 1 reviewed in full, 1 updated, 0 unchanged.
  Findings/fixes: clarified power independence, linked real Vault metadata
  evidence, and corrected roadmap wording that implied all socket policy was
  still absent. README examples were reviewed against the actual APIs;
  repository-relative Markdown links resolve. Identity/credits are preserved.
  Full diff/new-file and added-line secret-pattern review found no accidental
  secrets; this is not a comprehensive secret audit. No new/deleted files.
  Release documentation for absent systems remains pending.
- Real external blocker: read-only remote inspection confirms Matter-Vault
  `matter` is still `072c60a288d0773020f3460a639e77f84bd76c4c`; no
  `matter-beta` exists there. Its clean checkout has only README/ignore rules,
  manifest, version metadata and envelope schema. There is no executable
  key/binding provider or authenticated service contract to exercise. The
  metadata test cannot substitute for real key retrieval, binding, rotation,
  recovery or authenticated administrative integration. Requested the location
  of that external implementation/contract without requesting secret values.
- H and I have advanced but remain incomplete; J–L are not complete. Missing
  local subsystems remain implementation work, not external blockers: durable
  command audit, general authorization, persistent global policy, socket
  recovery/transfer, session/credential interfaces, connections/messaging,
  plugin isolation and public contract/release readiness work. Matter-Bot and
  platform implementation remain unopened.
- Exact next work: obtain and inspect the real external key/binding source and
  authenticated access contract, implement the Base-side integration against
  it while preserving Vault independence/envelope compatibility, and verify
  failure/recovery with a real source. Continue local audit and the missing
  prerequisites above; do not mark H–L done from metadata or isolated tests.
  At resume, use the repository's safe synchronization procedure and inspect
  this checkpoint's actual published SHA before editing.

## Historical recovery snapshot

- Repository: `/home/Sakura/Matter-Base`
- Working branch: `matter-beta`
- Previous synchronized base commit: `ea36faf68944b0d9be1a8563ebb0bfbc8782bfb6`
- Latest confirmed published checkpoint: `916fdbbf61c7b6c1668f7d58f0c43df3eb323d09`
- Remote: `origin` (`https://github.com/Matterssmith/Matter-Base.git`)
- Synchronization: push and subsequent fetch confirmed local HEAD and
  origin/matter-beta at `916fdbbf61c7b6c1668f7d58f0c43df3eb323d09` for the
  persistent socket activation checkpoint. This ledger-only follow-up does not
  change the tested implementation.
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
| Bot identity, family/release/instance IDs, registration | PARTIAL | `src/bots` generates IDs, persists validated identities, and allocates unique names; owner-only transfer/removal, owner/co-owner-scoped identity reads and lists, co-owner operations bounded to 256, legacy recovery authorization, and audit history work. General role/resource authorization remains absent. |
| Shared application users and bot data isolation | PARTIAL | `src/users` provides in-memory/SQLite global IDs, explicit unique external links, and bot/user/key-scoped text data with authorization hooks. A consuming application's `UserAuthorizer` must authenticate principals and verify bot existence/ownership; user storage does not consult the bot registry or implement general roles. |
| Sessions and platform credential contracts | ABSENT | No session/credential subsystem; platform credentials must remain outside this crate. |
| Persistence and recovery | PARTIAL | SQLite persists bot identities, shared users/identity links/bot-scoped data, and legal documents/decisions. No config/network-connection/service persistence or ecosystem-wide migration coordinator; each persistent component has its own versioned schema. |
| Events and messaging | PARTIAL | Named synchronous in-memory event callbacks exist; no durable events or messaging contract. |
| Router and command registry | PARTIAL | `CommandRouter` supports bounded prefixes/names/aliases/arguments, mandatory injected authorization, middleware, and synchronous handlers; ControlDispatcher provides socket gating and reserved control-prefix interception; an injected CommandAuditStore persists recognized attempts/outcomes; default routing remains unaudited and platform-specific parsing remains pending. |
| Middleware, roles, permissions, authorization | PARTIAL | Bot owner/co-owner checks and injectable user/legal action/resource authorization contracts exist; no general role inheritance or socket/resource policy engine. |
| Control Plane, sockets, activation | PARTIAL | SQLite socket activation store provides owner checks, power, group/socket overrides, recovery and atomic audit. Durable bot binding and control-prefix interception/router gating exist. Global fallback persistence and single-snapshot dispatcher integration are implemented; owner-authorized transfer/binding recovery are implemented; network lifecycle and lost-owner recovery remain pending. |
| Services and connections | PARTIAL | Typed in-memory service registry only; no connection manager or lifecycle. |
| Plugins, extensions, sandbox | ABSENT | No manifest validation, isolation, or resource limits. |
| Application audit log | PARTIAL | Legal decisions, bot/socket administrative changes and optional recognized-command attempts/outcomes persist. A unified audit service and broader ingress coverage remain pending. |
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

## Previous published checkpoint

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
- Commit `348431451771822581505484b4d042c098e92cba` was pushed to
  `origin/matter-beta`; fetch confirmed the published SHA. No stable branch
  promotion was made.

## Latest checkpoint — shared user registry

- Implemented `UserRegistry`, `UserPrincipal`, `UserAuthorizer`, and a
  replaceable `UserStore`; provided in-memory and SQLite stores.
- Identity links are explicit and unique by provider plus external subject.
  Repeated links to the same user are idempotent; linking the same external
  identity to another user fails. Matching identifiers across providers remain
  distinct.
- Bot data is keyed by bot instance, shared user ID, and data key. Every
  create/link/unlink/resolve/read/write/delete operation requires the supplied
  authorization policy. The application must authenticate principal IDs and
  verify bot existence/ownership; that integration is not yet provided here.
- Added optional user-registry injection to `MatterContext` and
  `MatterRuntime::open_with_registries` / `with_stores_and_users`.
- Added tests for SQLite reopen recovery, bot isolation, denial sanitization,
  provider namespace separation, link ownership, and concurrent link claims.
- Files changed: `src/users/mod.rs` (new), `src/lib.rs`,
  `src/core/context.rs`, `src/core/runtime.rs`, `tests/users.rs` (new),
  `README.md`, `docs/architecture.md`, and this ledger. No files deleted.
- Verification before the recovery interruption: `cargo fmt --all`,
  `cargo check --all-targets`, full `cargo test` (195 passed, 0 failed; 0 doc
  tests), `git diff --check`, and focused user tests (5 passed after the runtime
  injection/recovery case). After restoring the working tree, the full sequence
  passed again: `cargo fmt --all`, `cargo check --all-targets`, `cargo test`
  (196 passed, 0 failed; 0 doc tests), and `git diff --check`. Platform flow
  passed (3); no live services were used.
- README audit for this work: 1 found, 1 reviewed, 1 updated, 0 unchanged.
- Commit `6b501997f377e8dd424ba78669b04796f6a2a989` was pushed to
  `origin/matter-beta`; a subsequent fetch confirmed the same SHA. The README
  audit found 1 README, reviewed 1, updated 1, unchanged 0; its relative links
  resolve. No stable-branch promotion was made.
- Recovery note: an earlier stash attempt failed while intent-to-add entries
  were present; a separate preserved bot-registry stash was partially applied
  and conflicted. It remains in `git stash list` and was not discarded or
  merged. Its files and conflict snapshots were copied to
  `/tmp/Matter-Base-recovery-20261116`. The user-registry work was restored from
  Git objects `d1f504f…` (tracked files) and `e213e3b…` (new source/tests).
  Review the preserved bot stash later; its alternative in-memory bot registry
  conflicts with the existing persistent bot registry and should not be merged
  without a deliberate architecture review.

## Current checkpoint — user-store resilience tests

- Added SQLite migration rollback/retry coverage, future-schema rejection
  without rewriting stored data, and rejection of writes to unregistered users
  or over-limit values.
- Changed files: `tests/users.rs` and this status ledger. No files created or
  deleted. No production behavior changed in this checkpoint.
- Verified after synchronizing with `origin/matter-beta` at
  `81b48f5a1a42c927650ea4e9b5e3b9230c615bea`: `cargo fmt --all`,
  `cargo check --all-targets`, focused user tests (8 passed), full `cargo test`
  (199 passed, 0 failed; 0 doc tests), and `git diff --check` all passed.
- README audit: 1 found, 1 reviewed, 0 updated, 1 unchanged; test-only changes
  do not affect README claims.
- Commit `7f385de9579094221ab55e52d3ac672969b846ed` was pushed to
  `origin/matter-beta`; a subsequent fetch confirmed the exact SHA. No stable
  branch promotion was made. The original separate bot-registry stash remains
  preserved.

## Current local work — bot co-owner resource bound

- Added a 256 co-owner cap to both in-memory and SQLite assignment paths.
  Persisted identities with larger lists fail validation; ownership transfer
  and removal also fail closed when they encounter an oversized list.
- Added coverage for both stores and a corrupted SQLite row.
- Updated README and architecture documentation to state the cap.
- Audited public legal-store entry points. Sensitive reads, writes, document
  administration, and consent paths invoke `LegalPrincipal` before storage
  access. Authorization diagnostics are sanitized; no omitted auth path was
  confirmed in this pass.
- Verified after syncing with `origin/matter-beta` at
  `34f31718da728822cde3d12aca69437fb96ec553`: `cargo fmt --all`,
  `cargo check --all-targets`, focused bot-registry tests (15 passed), full
  `cargo test` (201 passed, 0 failed; 0 doc tests), and `git diff --check`.
- README audit for this code checkpoint: 1 found, 1 reviewed, 1 updated,
  0 unchanged; relative links resolve.
- Commit `82e4562ca6b022c2d45a53744f92da87d0b7ad92` was pushed to
  `origin/matter-beta`; a subsequent fetch confirmed the same SHA. No stable
  branch promotion was made.

## Current local work — reject empty provider key sets

- Confirmed that a provider returning a successful but empty key list caused
  `KeyManager` to replace and erase its current keys. Empty results now fail
  closed, preserving the current key set and active key.
- Added a regression test. Updated README and architecture documentation.
- Verified after syncing with `origin/matter-beta` at
  `b1f286c982e580608d2f37be6610fc8b5ea479a4`: `cargo fmt --all`,
  `cargo check --all-targets`, focused key-provider tests (7 passed), full
  `cargo test` (202 passed, 0 failed; 0 doc tests), and `git diff --check`.
- README audit: 1 found, 1 reviewed, 1 updated, 0 unchanged; relative links
  resolve.
- Commit `b1afcc84490de64436a909072bf4f15ae699d912` was pushed to
  `origin/matter-beta`; a subsequent fetch confirmed the exact SHA. No stable
  branch promotion was made.

## Exact next task to resume

1. Continue H by auditing remaining provider/key and cross-module trust
   boundaries; add tests for confirmed defects. Authenticated key reads are
   Vault-scoped, and bot identity reads/lists now enforce owner/co-owner scope.
2. Continue I with cross-module resilience around bot ownership, provider
   failures, legal-store errors, and shared users. One bot-owner/shared-user
   isolation flow is now covered; router, sockets, plugins, general resource
   authorization, and live services remain uncovered.
3. Complete J, K, and L in order. Reassess capabilities from actual code and
   keep Matter-Bot blocked until all readiness criteria are satisfied.
4. At each checkpoint, review every README, update this ledger, commit and push
   to `matter-beta`, then fetch and verify the exact published SHA. Never claim
   Vault or platform integrations work without real verification.

## Documentation audit — service registry

- Corrected `docs/services.md`: its lock-poisoning note no longer matches the
  implementation. The registry recovers poisoned locks and has a regression
  test; lifecycle, health checks, persistence, and authorization remain
  unimplemented.
- README audit: 1 found, 1 reviewed, 0 updated, 1 unchanged. No README claims
  the service registry provides lifecycle or connection management.
- Documentation-only correction; no tests were run for this edit.
- Base before publication: `5ee4a71e562decc3e72b38fbe54e765b2736ed02`.

## Current local work — total key-manager capacity

- Added the 4,096-key limit to manual `insert` and `rotate`, matching the
  provider-load limit. At capacity, both operations fail without mutating
  key count or active-key state.
- Added a boundary regression test and updated README/architecture docs.
- Verified after syncing with `origin/matter-beta` at
  `6b7a7f8e75c5d5409b9e5b81cfdd1e97b4e06b1e`: `cargo fmt --all`,
  `cargo check --all-targets`, focused key-provider tests (8 passed), full
  `cargo test` (203 passed, 0 failed; 0 doc tests), and `git diff --check`.
- README audit: 1 found, 1 reviewed, 1 updated, 0 unchanged; relative links
  resolve.
- Commit `7ffe9c570033b0e50cd0ce4a00c902e3772d786a` was pushed to
  `origin/matter-beta`; a subsequent fetch confirmed the exact SHA. No stable
  branch promotion was made.

## Current local work — key-manager Vault isolation

- Confirmed that an `AuthenticatedProvider` could read from a `KeyManager`
  loaded from a different Vault when the manager handle was reused.
- Authenticated key ID, key material, and active-key reads now compare the
  manager's recorded Vault ID with the authenticated identity and reject a
  mismatch. In-memory manually inserted keys without a provider Vault remain
  usable by existing direct tests.
- Added a two-Vault regression and documented the scope check.
- Verified after syncing with `origin/matter-beta` at
  `40c3bcac140aaab8b91be8541a849293b6f0ec71`: `cargo fmt --all -- --check`,
  `cargo check --all-targets`, full `cargo test` (204 passed, 0 failed; 0 doc
  tests), and `git diff --check`.
- README audit: 1 found, 1 reviewed, 1 updated, 0 unchanged. The architecture
  note was updated too; the documented internal links still resolve.
- Commit `66b327616c05344305f3351195541869161c4e6b` was pushed to
  `origin/matter-beta`; a subsequent fetch confirmed the exact SHA. No stable
  branch promotion was made.

## Current local work — bot and shared-user integration

- Added a cross-module test that creates two bot instances and a shared user,
  persists them in separate SQLite stores, closes and reopens both stores,
  authorizes access through recovered bot-owner permissions, verifies user data
  remains isolated by instance, and rejects cross-owner reads and writes.
- After syncing with `origin/matter-beta` at
  `7b455090f1cf25ed2b6b1dc6a1cd25da57a1a5b3`, `cargo fmt --all -- --check`,
  `cargo check --all-targets`, the full suite (205 passed, 0 failed; 0 doc
  tests), and `git diff --check` passed.
- README audit: 1 found, 1 reviewed, 0 updated, 1 unchanged. This test adds no
  public behavior; README and architecture claims remain accurate.
- Commit `d1e9af11bad9785b137cb9018b4024e7ebd864ed` was pushed to
  `origin/matter-beta`; a subsequent fetch confirmed the exact SHA. H and I
  remain in progress; no stable branch promotion was made.

## Current local work — bot identity read authorization

- Confirmed that `BotRegistry::get` and `list` returned owner and co-owner
  identity data without an actor scope.
- Both methods and the `BotIdentityStore` contract now require an actor ID;
  reads return only records owned by or shared with that actor. Inaccessible
  single records appear absent, and list results are filtered. The registry
  repeats filtering for replaceable store implementations.
- Updated SQLite and in-memory stores, legacy migration tests, and call sites.
  Legacy unowned records stay hidden until successfully claimed through the
  recovery authorizer.
- README and architecture now show the actor-scoped signatures and the
  required authenticated-request boundary.
- After syncing with `origin/matter-beta` at
  `007d4e0e52cc42a9c629111cc0c0b33984078cf4`, format, all-target check, focused
  bot/platform integration tests, full suite (205 passed, 0 failed; 0 doc
  tests), and `git diff --check` passed.
- README audit: 1 found, 1 reviewed, 1 updated, 0 unchanged.
- Commit `ff2301abbb01bd4d7b0beca05f7bd26cce708cf0` was pushed to
  `origin/matter-beta`; a subsequent fetch confirmed the exact SHA. H remains
  in progress; no stable branch promotion was made.

## Current local work — universal command router

- Added and exported `CommandRouter`, `CommandInvocation`, and the mandatory
  `CommandAuthorizer` contract. It handles configured prefixes, case-insensitive
  canonical names and aliases, whitespace arguments, middleware order, and
  synchronous handler dispatch.
- Enforced limits for input bytes, prefixes, command names/count, aliases,
  middleware count, and argument count. Aliases resolve to canonical names
  before authorization. Authorization, middleware, and handler callbacks run
  outside the registry lock; callback errors/panics are returned as sanitized
  errors.
- Added unit coverage for alias authorization, middleware order/rejection,
  unauthorized dispatch, duplicate names, size limits, and callback panics, as
  well as an integration test exercising the documented root API.
- Added a cross-module flow test wiring `CommandRouter` authorization directly
  to the real bot registry: one owner executes against its instance, another
  owner's request is rejected before handler effects, and that owner can use
  its own separate instance.
- Command routing remains PARTIAL: durable audit records, platform-specific
  argument parsing, and Control Plane/socket activation are not integrated.
  The authorizer must verify authenticated actor, bot ownership, permissions,
  and operational activation.
- After syncing with `origin/matter-beta` at
  `9eb8943b281f7e8a35a981068d08406fcf7859ce`, format, all-target check, full
  suite (214 passed, 0 failed; 0 doc tests), and `git diff --check` passed.
- README audit: 1 found, 1 reviewed, 0 updated, 1 unchanged. The public example
  is compiled and exercised by `tests/commands.rs`.
- Router implementation commit `1c3225ba3e8ac93bd669203c914421c571dc3705` and
  integration commit `7212f10bf531f69de417b4a95970a868d92fa490` were pushed to
  `origin/matter-beta`; a subsequent fetch confirmed the exact SHA. H and I
  remain in progress; no stable branch promotion was made.

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

## 2026-11-16 — Persistent activation prerequisite for WhatsApp

- Branch: `matter-beta`; synchronized base: `77648da3658fed79cd6a18e7de10f4cadae70a13`.
- Stage H corrections and prerequisite implementation remain in progress; H–L
  and Matter-Base readiness are NOT complete.
- Added `src/activation.rs`, `tests/activation.rs`, and `docs/activation.md`;
  updated `src/lib.rs`, README, architecture and this ledger; no files deleted.
- Implemented SQLite activation ownership, default power-off, group/socket
  overrides, trusted global fallback, versioned transactional schema and atomic
  policy/audit writes. Audit failure rolls back the state mutation. Corrupt
  persisted policy values fail closed rather than interpreting any nonzero
  integer as enabled.
- Trusted application provisioning and actor authentication remain required.
  There is no network connection, session storage, bot/socket binding, ownership
  transfer, interchangeable backend or integrated command gate yet.
- Executed `cargo fmt --all`, `cargo check --all-targets`, focused activation
  tests (4 passed), full `cargo test` (218 passed, zero failed) and
  `git diff --check` successfully after synchronization.
- README audit: 1 found, 1 reviewed, 1 updated, 0 unchanged. Corrected stale
  claims that command routing and shared users were absent; added actual module
  paths and activation limitations. Internal links were checked. Credits preserved.
- Read-only GitHub inspection: Matterssmith/Matter-Baileys has only branch
  `matter`, at `57226de377c1dfeecd9de49415fb593c8ac01936`, with only README.md
  in its root. No runnable adapter is available there. No session was opened.
- Exact next task: add authenticated bot/socket binding and a dispatch boundary
  that intercepts control prefixes before normal routing and rejects normal
  commands while powered off. Test owner denial, concurrent changes, persistence
  and handler side effects before declaring that flow verified.
- Continue with session/credential contracts, connections/messaging, persistent
  command audit and remaining readiness criteria; real WhatsApp testing also
  requires implementing the external adapter and linking a secondary test account.
- To resume: read this ledger, inspect actual Git and instructions, preserve local
  changes and ignored data, fetch/synchronize matter-beta, then continue the exact
  next task. Existing legacy bot-registry stash is retained.

- Publication confirmed: activation checkpoint
  `916fdbbf61c7b6c1668f7d58f0c43df3eb323d09` is on origin/matter-beta.
  README re-review for this ledger-only commit: 1 found/reviewed, 0 updated,
  1 unchanged; no new documentation discrepancies. `git diff --check` passed.
