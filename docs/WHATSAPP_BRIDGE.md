# WhatsApp-Matter ↔ Base-Matter: owner-only control bridge v1

**Development contract, not proof of a linked WhatsApp session.** This
protocol is intentionally restricted to the Base-Matter global control
prefix \`=/>\`; no normal bot commands, test ping or QR are supported.

## Boundaries

- WhatsApp-Matter (provider) must authenticate itself using its own active
  paired WhatsApp session. It constructs frames ONLY for \`fromMe=true\`
  messages in the owner's exact configured private JID. Group messages,
  external senders, relayed LID/PN mismatches and unrecognized identities
  must never become global control commands.
- Base-Matter binds a 32-byte HMAC key to a known provider installation
  **and one socket**. No shared cross-socket keys, no default/fallback key,
  no env or private material in Git.
- The verified frame maps to a locally configured socket owner identity.
  Caller-provided phone and actor fields never override that identity.
- Base-Matter calls its own reserved ControlDispatcher after verifying
  identity, signature, time and persisted replay. The socket can be off;
  its bot need not be bound. Unknown reserved verbs fail closed.
- This is synchronous admission, not cancellation of previously started
  effects. No platform message is sent by Base-Matter itself.

## Wire format

JSON UTF-8 object, maximum 16,384 bytes:
\`version\` (number, 1), \`socket_id\`, \`actor_id\`, \`owner_jid\`,
\`chat_jid\`, \`message_id\`, \`text\`, \`from_me\` (boolean, true),
\`timestamp\` (signed Unix seconds) and \`signature\` (lowercase hex HMAC).
No additional fields accepted.

Signature input:
- prefix UTF-8 \`Base-Matter:WhatsApp-Control:v1\` and a zero byte;
- each of the seven strings (version as \`1\`, socket_id, actor_id,
  owner_jid, chat_jid, message_id, text) as a 32-bit big-endian byte length
  followed by UTF-8 bytes;
- one byte for \`from_me\` (1);
- signed timestamp as an eight-byte big-endian integer.

HMAC-SHA256 over these bytes, encoded as 64 lowercase hex chars.
Both adapters have a cross-language fixed test vector. The Rust verifier
uses constant-time HMAC verification.

## Outstanding production work

The current Rust library accepts signed frames, but no live provider IPC,
transport security configuration, credential storage, remote liveness
proof, pairing-code delivery or actual mobile linkage has been verified.
A user-controlled local TCP bridge with a dedicated listener and runtime
integration must be implemented and tested separately, including
restart/revocation and multi-process isolation.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
