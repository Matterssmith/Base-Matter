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
