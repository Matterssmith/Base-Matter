# Authenticated inbound platform contracts

Matter-Base does not trust sender IDs or socket IDs embedded in network
messages. The application injects the following platform-specific contracts:

- InboundAuthenticator verifies provider signatures, session keys, socket
  ownership and external identity resolution, then returns VerifiedIngress.
- InboundAdmission validates authenticated actor, consent, session generation,
  group/destination policy and other platform rules on EVERY event.
- InboundGateway verifies bounds, rejects provider mismatch, requires both
  injected policies, claims an event's replay fingerprint and calls the
  existing ControlDispatcher. Control commands use the same admission gate.
- InboundReplayStore persists a keyed HMAC-SHA256 of the verified provider,
  socket and provider event ID. It never stores raw event identifiers, message
  text or the binding key. A single shared replay key is required for all
  processes; changing the key resets effective replay identification.
- Claims are transactional across SQLite connections, bounded to 100,000.
  Exceeding capacity denies admission until trusted pruning. Pruning reopens
  a replay window and requires an explicit provider retention policy.

Important: the replay claim is recorded before effects. A crash or failed
handler can leave an event claimed without successful execution; the host
must reconcile unknown outcomes, not blindly replay operations. A local
test authenticator does NOT prove real webhook signature verification.

No real WhatsApp, Telegram or Discord adapter or network login is bundled.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
