# Sessions and message delivery contracts

Matter-Base now owns durable **connection metadata**, not provider sessions.

- SessionRegistry stores socket ID, owner ID, platform name, lifecycle state,
  revision and owner-scoped immutable audit in SQLite.
- Trusted provisioning is idempotent for the same owner/platform and rejects
  takeover or changing the provider. Credentials and authenticated platform
  sessions are **never stored** by this module.
- State transitions require the socket owner and the expected revision, run
  in an immediate transaction and atomically append the audit. Stale writes
  and invalid state transitions fail closed, including across connections.
- Lifecycle states: Disconnected -> Connecting -> Connected. Connecting and
  Connected may fall back to Disconnected or Faulted; Faulted can recover
  through Connecting or Disconnected. Same-state updates are no-ops.
- ControlPlaneMessageAdmission checks operational socket power, group/socket/global
  policy, current bot binding/ownership, session owner consistency, and the
  scoped message:send / execute role. It additionally requires an application
  MessageAdmission implementation for identity, consent and provider rules.
  It is fail-closed but cannot make independent DB reads/network effects atomic.
  OutboundMessage.group_id distinguishes actual groups from private messages.
- MessageService requires a MessageAdmission implementation to check actor
  authentication, authorization, legal consent, socket activation and group
  policy. It only calls a TransportDriver for a connected session.
- TransportDriver is a **contract for an external platform implementation**.
  There is no built-in WhatsApp, Telegram or Discord connection, session
  credential loader, or network driver. Implementations own their credentials
  outside Matter-Base.
- The contract does not make an outbound operation atomic with a lifecycle
  transition. A send already admitted may race with disconnect. The driver
  must reject stale session revisions/generations at its own network boundary.
  No retries, idempotency claims or recipient-delivery guarantees exist.
- Transport errors are sanitized; message bodies are not inserted in
  SessionRegistry. The host must ensure its own logs do not expose content.

See [tests/sessions.rs](../tests/sessions.rs) for local deterministic
regressions. Real external credentials, platform sessions, network disconnects
and transport compatibility remain integration work.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## Authenticated connection lifecycle

The optional `connections` module provides ConnectionService on top of
SessionRegistry. The application supplies an authenticated ConnectionAdmission,
SessionCredentialSource returning `Zeroizing<Vec<u8>>`, and a separate
ConnectionDriver for the real platform. The orchestration owns no network
socket implementation and never stores external sessions or login credentials.

State changes are compared by revision. Connecting moves to Connected only
when the external driver reports success. Driver or credential-source failure
marks the record Faulted and suppresses private diagnostics. `check_health`
probes actual provider liveness and faults a disconnected session. An
uncertain state after a process crash can be marked Faulted explicitly.
Adapters must fence and revoke old connection generations across processes:
SQLite state changes are not atomic with a network connection. A real
credential source, socket activation, consent and destination authorization
remain mandatory in the consuming application. No network transport is
bundled with this crate.

## Pairing-code-only WhatsApp contract

Use `PairingCodeService` with authenticated session ownership, injected
`ConnectionAdmission` and a separate `PairingCodeProvider`. It returns a
redacted, zeroizing temporary code for an unlinked WhatsApp session. No QR
mode, pairing URL or ping test command is exposed. This remains a provider
contract rather than a live socket. See [pairing-code contract](PAIRING_CODE.md).
