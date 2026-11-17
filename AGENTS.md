# Matterssmith — repository instructions

Read `docs/DEVELOPMENT_STATUS.md`, the root README and architecture before work.
Inspect actual code, Git status, branch, ignored files and applicable nested instructions.
Do not assume a past report is current. Preserve useful existing implementations.

## Identity

Preserve the project's visual identity and existing credits. Where credits apply:

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

Do not add a separate author line or attribute project authorship to automation.
Preserve upstream copyright/license notices when importing provider code.

## Git and verification

Work on `matter-beta`; `matter` is stable. GitHub is the source of truth.
Inspect all local differences first. When dirty, protect tracked and new
nonignored files with `git stash push -u`. Fetch origin, switch to matter-beta,
reset to origin/matter-beta, restore the stash and inspect conflicts. If fetch
or stash restoration fails, stop dependent modifications/verification, retain
recovery data and report the failure. Never use git clean or remove ignored
sessions, databases or credentials. Preserve unrelated stashes and user changes.

Run cargo fmt --all, cargo check --all-targets, cargo test and git diff --check
for implementation checkpoints. Report actual results. Do not change global
linker/toolchain configuration without demonstrated need.

The user has authorized committing and pushing verified development checkpoints
to matter-beta. Review EVERY repository README before EACH commit, report total,
reviewed, updated, unchanged, findings, fixes and pending issues. Update affected
README/code/contracts together; preserve correct content and design. Check new
files, full diff, examples, internal links, credits and accidental secrets.
Confirm the pushed SHA against origin. The owner has authorized promoting independently verified stable subsets to
`matter`, not a bulk merge of incomplete beta. Require green CI on the exact
candidate SHA and preserve divergent stable commits (including Cargo.lock
history). Leave all unverified work in matter-beta.

## Scope and continuity

Finish Matter-Base before starting Matter-Bot. Continue H audit/corrections,
I integration/resilience, J security, K contracts and L release readiness.
Missing earlier prerequisites must be implemented and tested; do not call
interfaces, mocks or isolated tests a completed external integration.
Record exact next work, changes, tests and blockers in docs/DEVELOPMENT_STATUS.md
at checkpoints and before ending a session. Preserve its historical entries.

Priority is Matter-Base with the real Matter-Vault contract and global control
commands. Vault is independent and must not store platform credentials/sessions.
Do not change cryptographic formats without compatibility/migration review.
Intercept `=` and `/>` control prefixes before the normal router. Only the socket
owner can power it on/off. A global owner label is not authority over another
socket; an explicit socket ID still needs authorization. Operational power is
independent of network connection. Powered-off sockets execute no normal commands.
Policy precedence when powered on: Group > Socket > Global > None (deny).
Preserve socket isolation and persist/audit administrative changes atomically.

## Current hard constraints — owner clarification 2026-11-17

- Every \`*-Matter\` repository is part of the Matter ecosystem; Bot-Matter
  will consume its components in a later, separate stage.
- DO NOT implement, refactor or launch Bot-Matter until Base-Matter is complete.
  Freeze separate platform prototypes until core readiness, unless the owner
  explicitly reauthorizes them.
- Historical WhatsApp pairing-code connection/ping tests were undertaken
  before this beta version. Do not erase that history. However, do not claim
  the current remote Base repository contains a verified live WhatsApp
  connection without actual source and an end-to-end test.
- WhatsApp linking MUST use pairing code only. QR must not be exposed,
  advertised or used as fallback. Never overwrite existing local sessions.
- The former ping is a test only; it is NOT a permanent global command.
  Global control is implemented by Base-Matter using canonical \`=/>\`,
  independent from the bot's own command prefix and even when the bot is off.
  Legacy \`=\` and \`/>\` aliases are retained until an intentional migration.
- Preserve every ecosystem repository's history and respect the fact that
  provider modules are ultimately consumed by Bot-Matter, not developed as
  an excuse to start Bot-Matter early.

## Lenguajes y WhatsApp — corrección expresa del propietario

- Lenguajes principales: Python, C++ y Rust. Mantener Rust como núcleo y proveedor de WhatsApp ya trabajado. C y Go son secundarios; JavaScript/Node.js es terciario y NO sustituye ni dirige la implementación `whatsapp-rust`.
- Recuperar el código histórico `src/providers/whatsapp/mod.rs` del commit `3c36d97bb` para continuidad; conservar licencias, datos de sesiones y progreso existente.
- Prioridad actual: completar comandos globales, seguridad, verificación y componentes pendientes de Base-Matter sobre Rust. No seguir desarrollando la sonda Baileys/Node ni Bot-Matter.
- La vinculación WhatsApp es exclusivamente por pairing code; el `ping` anterior era una prueba y se retira de comandos durables.
- Atribuir comandos/archivos con el crédito exacto: `Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].` (en comentarios o notas discretas), sin línea duplicada de Autor.
- El estado `powered` de un socket NO implica que esté habilitada su política de ejecución. Las pruebas deben mantener esa separación.

## Updated ecosystem direction (user instruction)

- Target name WhatsApp-Matter replaces Matter-Baileys.
- Target name Telegram-Matter replaces Matter-Telegraf.
- All other repositories follow <Component>-Matter, with Base-Matter and
  Vault-Matter first. Preserve Matterssmith credits and stable branches.
- Inspect and rename the actual GitHub repositories when ecosystem work begins;
  documenting these targets does not mean remote repositories were renamed.
- Each platform repository may contain multiple provider methods in separate
  directories, e.g. providers/baileys and providers/telegraf. Verify the official
  upstream source, pin its revision, preserve history/licenses/notices and keep
  local modifications traceable. Do not clone unverified lookalike repositories.
- Adapters implement universal Matter-Base contracts. Platform libraries and
  provider-specific changes remain in their platform repository, outside Base.
- Full ecosystem management does not imply bypassing a platform's permissions.
- Real WhatsApp verification uses a secondary test account and a single process
  owning its connection; never overwrite an existing session. Then verify
  equivalent Telegram and Discord functionality where supported.
- This scope does not relax Matter-Base readiness criteria. External service
  availability/account linking must be reported honestly when blocked.
