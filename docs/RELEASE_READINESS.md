# Matter-Base release readiness — evidence gate

This document separates verified reusable contracts from production
capabilities. Passing CI alone is not a readiness declaration.

## Verified in the development branch

- Core runtime, event and typed service registries.
- Bot identities, co-owner and owner checks, persistent user links and legal
  consent contracts.
- Scoped roles, explicit denials, command authorization and audit.
- Socket activation/control, message admission and session metadata.
- Provider key authentication and XChaCha20-Poly1305 envelope compatibility.
- Encrypted control outbox, change detection and lost-ack reconciliation.
- Plugin manifest/trust contracts with the deny-by-default sandbox interface.
- External session/credential and transport-driver contracts.
- Test and lint verification by GitHub Actions on matter-beta.

## Hard blockers before claiming Matter-Base complete

1. **Authenticated Vault implementation.** Install a real provider binding,
   key source, secret rotation and GitHub/Vault remote sink+reader with least
   privilege. Run end-to-end tests using a separate nonproduction Vault and
   verify remote conflict, restart, missing acknowledgment and restore paths.
   Repository metadata tests or mock sinks do not close this item.
2. **Real platform adapters.** Build or link independent, pinned adapters for
   WhatsApp, Telegram and Discord, with verified upstream licenses and no
   shared credential leakage. Use only an authorized secondary test account,
   a single owner process per connection and failure/reconnect tests.
3. **Real plugin sandbox.** Supply a tested OS-process/container/WASM
   isolation backend with CPU, memory, time, filesystem and network limits,
   revocation, independent process ownership and no inherited host secrets.
   The in-process mock and DenyPluginSandbox are not production execution.
4. **Cross-store ownership transaction.** Serialize and recover transitions
   of bot, socket and role ownership. Validate role administration revocation
   and inability to continue sessions under the old owner, including crashes
   during transitions and multiple processes.
5. **Full platform security and privacy review.** Authenticate ingress,
   enforce consent at every sensitive action, threat-model ciphertext sync,
   prevent replay/double send, run secret scans and dependency vulnerability
   checks. Verify no sensitive local sessions or production keys enter Git.
6. **Cross-platform verification.** Run release tests on the supported
   Linux/Windows/Termux targets; test migrations, disk failures, timeouts,
   corruption and backups. Unit tests on one Ubuntu CI runner are insufficient.
7. **Release documentation and operator procedure.** Confirm official provider
   revision pins, permissions, recovery, key rotation, monitoring, logs,
   support boundaries, license notices and version compatibility.

## CI and rules

The workflow on matter-beta runs rustfmt, compilation, unit/integration and
documentation tests, strict Clippy, whitespace checks and tracked README/credit
and sensitive-filename verification. Results apply only to the exact tested
SHA. Check the current green run before accepting changes.

Do not merge automatically to matter. Do not start Matter-Bot until the above
blockers are implemented and verified. Keep repositories independent and do not
persist session secrets in Base or Vault.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

The matter-beta workflow includes Linux and Windows Rust lanes. Termux and
real platform/provider compatibility remain separate release gates.

The inbound gateway provides replay and authorization contracts only; actual
signed webhooks and authenticated platform sender bindings remain unverified.

## Most recent verified development checkpoint

GitHub Actions run `44125657053` on `matter-beta` SHA
`d4dcc7791fdf1641de74e755a2edef81f87531b8` passed both Linux
and Windows Rust lanes on November 17, 2026. This does not waive any of
the external integration blockers above. No release promotion was performed.

## Verified stable promotion and WhatsApp smoke

Stable branch `matter` merged PR #3 at `e509cc4f8c53eafbb1233359bc2ae63e0dde7f48`.
GitHub Actions 44139981479 passed Linux/Windows. The beta naming and
administrative rename plan passed CI 44138531259. WhatsApp platform repo
`Matter-Baileys`, beta, passed eight offline tests and pinned dependency
installation in 44139262152. These are not evidence of live WhatsApp pairing
or production Vault, OS sandbox or account connection. Keep these as hard
release blockers.
