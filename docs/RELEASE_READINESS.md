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

## Clarificación obligatoria: WhatsApp y Bot-Matter

Vinculación WhatsApp exclusivamente por pairing code (nunca QR) sobre un
socket autenticado de su dueño. Confirmar conexión real y recuperación de
la sesión sin sobrescribir credenciales. El `ping` histórico no es el bot
global: los cambios administrativos usan `=/>` desde Base-Matter.
Bot-Matter no se desarrolla hasta cerrar todos los criterios de Base-Matter.

## Signed WhatsApp control integration checkpoint

The HMAC-validated owner-only global control frame and local replay checks
are unit-tested separately from actual WhatsApp pairing. Before promotion,
require live phone pairing via code, verified incoming owner messages,
dedicated per-socket keys, no socket crossover, revocation and crash tests.

## Proveedor primario elegido y créditos

Se continúa con **whatsapp-rust**, por código de vinculación únicamente. El
prototipo Node/Baileys no es el proveedor de Base-Matter y está congelado.
Los futuros comandos de control global se registran en Rust bajo `=/>`, con
controles del propietario y crédito requerido por AGENTS.md:

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## WhatsApp native ingress gate

The Rust native ingress module is covered by offline tests only. A verified single-process whatsapp-rust pairing-code session, actual self-chat messages, revocation and recovery are required to qualify the live provider for stable.

## Executable whatsapp-rust provider prerequisite

An isolated Rust provider crate was added at providers/whatsapp-rust, with pairing code only, SQLite session lease and owner control. Run its separate CI and perform controlled live device pairing, recovery and security review before promoting to stable. The upstream library generates QR concurrently internally; this provider does not display or expose QR callbacks.

## Native SQLite version conflict resolved by process separation (architecture)

whatsapp-rust 0.7 requires a different libsqlite3-sys major version from Base. They must not link into a single executable. Two Rust processes communicating over authenticated loopback remove the link collision, but do not themselves establish production readiness. Pairing code with an authorized phone, security checks, and receiver runtime tests are mandatory.

## Rust loopback command server

The new Base-Matter whatsapp-matter-control receiver requires CI, replay persistence and real linked-session tests, with distinct transport and replay secrets. It never enables Bot-Matter. Until actual phone linkage is verified, provider integration is not production-ready.

## Recent local Rust connection checkpoints (not a phone release)

- `fd2fb26363b622549185527274e36b43f2ddc4c9` passed the full
  GitHub Actions Rust/Linux and Windows verification on 2026-11-17, including
  a true subprocess TCP restart/replay integration test.
- `1c10f44e2ab3aef129c9964f8b64a5239e97bffe` passed the same
  base CI and a separate `whatsapp-rust` provider CI. The provider
  verifies the local controller listener before opening a session.
- Later self-chat confirmation code is an unverified development checkpoint
  until its exact SHA passes provider CI and an authorized real phone test.
  Self-chat delivery, pairing code, LID matching, revocation and recovery
  under real WhatsApp traffic are still unresolved release gates.
- Rust 1.89+ OS file leases allow crash recovery of v2 lockfiles. Legacy
  PID-only locks remain blocked until verified operator migration.
