# Socket activation contracts

State: **Partially implemented**. `activation::SocketActivationStore` stores
policies in SQLite. It contains no credentials, platform sessions, or network
connection. Parent directories for the database must already exist.

The trusted application calls `provision(socket_id, authenticated_owner)` after
authenticating the request and validating its authority to create the socket.
Provisioning rejects duplicate socket IDs and begins with operational power off.
This is a trusted bootstrap API, not a platform identity authentication service.
Do not expose it directly as an untrusted endpoint.

`set_power`, `set_socket_policy`, `set_group_policy`, and `audit_history` require
the stored owner ID from the authenticated request boundary. Neither a global
owner label nor an explicit foreign socket ID grants access. State changes and
audit insertion commit together; audit failure rolls back the state change.

`resolve` is an internal runtime query. It denies powered-off sockets regardless
of overrides. When powered on, the first configured policy wins: group within
that socket, socket, global configuration supplied by the trusted runtime, then
deny. Removing an override with `None` restores fallback. This does not change
the network connection. The read uses one SQL statement for a coherent snapshot.
Missing sockets and storage failures return errors; callers must fail closed.

Schema initialization is transactional and versioned as `socket_activation=2`.
Version 1 upgrades preserve existing policies; existing sockets remain unbound
until explicitly associated with a bot.
Unknown versions are rejected. Audit records omit message bodies and credentials.
Tests cover reopening, precedence, isolation, authorization, duplicate provisioning,
audit-write rollback, corrupt policy rejection, and unsupported schema versions using isolated temporary DBs.

`bind_bot(socket, actor, bot_id, bots)` requires both socket and bot ownership.
Bindings are durable and immutable; a conflicting rebinding fails. The dispatcher
rechecks the stored socket owner's bot authority on every admission. A later bot
ownership transfer blocks that socket until a dedicated recovery/transfer flow
is implemented. Ownership checks across these two stores are not a distributed
transaction; revocation does not cancel requests already admitted.

`control::ControlDispatcher` accepts the receiving socket from a trusted adapter,
resolves its bot internally and intercepts control messages before normal routing.
Grammar: `=on [SOCKET_ID]`, `=off [SOCKET_ID]`, with `/>` and `=/>` also accepted.
Unknown control verbs fail without reaching registered handlers. Only the target
socket's owner may change its power. An explicit target grants no extra privilege.
The dispatcher returns `Inactive` for normal messages while power/policy denies
access. Otherwise the normal router still applies its command authorizer.
The application authenticates actors and establishes socket/source/group context;
message text must never determine those trusted identifiers.

Admission uses snapshots. Power-off prevents subsequent admissions; it does not
cancel callbacks already admitted or undo their effects. Use the dispatcher for
all inbound messages; calling the lower-level router directly skips socket gating.

Pending: interchangeable storage, ownership transfer, network connection lifecycle,
global-policy persistence, authenticated adapter ingress and production integration.
These remain prerequisites for the real WhatsApp activation test.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
