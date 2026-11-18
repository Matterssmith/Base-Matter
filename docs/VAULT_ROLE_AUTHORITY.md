# Vault-Matter as the single role authority

**Status:** approved architecture direction; authenticated Vault runtime is not yet
implemented. This document does not assert that GitHub contains live role
records or that WhatsApp/Telegram/Discord write operations work.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## Non-negotiable authority boundary

- **Vault-Matter is the only authority for socket/group role assignments.**
  Base-Matter must not create authoritative role records, grant/revoke local
  roles, or publish directly to the Vault repository. Local RoleStore/outbox
  fixtures remain offline tests of serialization and cryptography only.
- Base-Matter may request **read-only, authenticated authorization decisions**
  from the Vault service; no mutable role copy is installed in Base.
  A grant is valid only when Vault confirms it is current for the authenticated
  actor, socket, group, action, and policy revision. No response, unknown actor,
  expired response, revoked role, mismatched scope or bad signature means deny.
- Socket **ownership** is a distinct, trusted control identity; it is not a
  self-assigned `owner` grant. Project owners/co-owners/staff, bot ranks,
  socket ranks and group ranks cannot be used interchangeably.
- `user` is implicit and does not create a role record. To change a subject
  back to `user`, Vault atomically revokes the explicit rank and increases the
  policy revision; Base must not continue using old permission decisions.
- No cleartext role records, platform identifiers, tokens, encryption keys,
  session databases or WhatsApp session content are written to GitHub.

## Administrative write flow (messaging gateway)

1. An authorized person sends a command to the **Vault-Matter gateway** over
   WhatsApp, Telegram or Discord. Each gateway is a platform adapter, not
   independent role storage or permission logic.
2. Vault verifies the authenticated platform identity and a previously
   established account link. A display name, phone number typed in a message,
   forwarded message, bot relay or claimed group administrator status alone
   are not authorization.
3. Vault validates the target socket/group, actor capabilities, current
   owner and policy revision. Sensitive owner/co-owner changes require an
   explicit confirmation bound to a one-use nonce and short expiry.
4. Vault commits one atomic, audited role change in its own policy engine,
   increments the revision, encrypts the updated role state with an
   externally supplied scoped key (XChaCha20-Poly1305), and publishes only
   the authenticated envelope to its private repository using its own
   least-privilege GitHub App identity and conditional revision update.
5. Vault replies on the same authenticated platform with success only
   **after** the authoritative mutation and required remote durability are
   confirmed. A failed GitHub publication must not be reported as durable
   success; pending/uncertain commits require reconciliation.
6. Base-Matter never receives write credentials or direct admin API access.
   A Base-Matter command may display the intended operation or direct the
   operator to Vault's own channel, but cannot impersonate the operator
   or use an unauthenticated WhatsApp relay as a write authorization.

Ordinary writes can be restricted to Vault's authenticated messaging
gateways. Operational disaster recovery still needs a documented offline
procedure for outages, account bans, token rotation and lost devices; such
recovery must be separately authenticated and audited rather than a hidden
Base-Matter write backdoor.

## Read-only authorization contract (to implement)

An authenticated Base service sends an authorization query with:

- exact `socket_id`, authenticated `actor_id` and linked platform ID;
- optional provider-verified `group_id`, requested `resource` and `action`;
- unique request nonce, timestamp and expected policy scope.

Vault responds with an authenticated, integrity-protected decision bound to
all those request fields, the definitive scope, a monotonically increasing
policy revision, an expiry, and the verifying Vault service identity.
A negative or unavailable response denies the capability. A transport-level
success without cryptographically authenticated binding is insufficient.
For high-impact actions, Base requests a fresh decision; it never treats an
old local grant snapshot as the source of truth. A short-lived read-through
optimization may be considered only if it cannot survive revocation or a
Vault outage in a way that grants stale authority. The default is no
authorization cache.

A production implementation should use an authenticated local IPC or
mTLS service connection between Base and Vault, with a **Vault-owned signing
key** for verifiable decisions. Public verification keys in Base are
read-only. Do not use a shared MAC key that would let Base forge a Vault
authorization response. GitHub encrypted envelopes are durable storage,
not a low-latency permission API.

## Recovery and branch policy

- On restart, Vault reads encrypted repository state, authenticates the
  revision, decrypts with its externally managed key provider and reconciles
  the monotonic policy revision **before** issuing positive decisions.
  Without this verification, it denies grants.
- Base's verified socket owner can still inspect its socket and stop the
  connection without depending on a delegated role. This is an operational
  safety control, not a Vault role override.
- Develop protocol and offline tests on `alpha`. Promote reviewed,
  independently validated changes to `beta` and then `stable` by PR;
  never merge untested role engines or silently mix role databases.
- Current alpha live WhatsApp controller intentionally leaves socket/group
  role changes **disabled** until the authenticated Vault authority exists.
  Its local activation store contains socket power and group policy only;
  that is not a role registry. A local role SQLite file created by an earlier
  alpha prototype must not be imported as an authoritative Vault grant.
