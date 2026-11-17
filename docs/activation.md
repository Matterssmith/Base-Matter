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

Schema initialization is transactional and versioned as `socket_activation=3`.
Version 1/2 upgrades preserve existing policies, audit and bindings. Version 1
sockets remain unbound until explicitly associated with a bot. No global
administrator or policy is inferred during migration.
Unknown versions are rejected. Audit records omit message bodies and credentials.
Tests cover reopening, precedence, isolation, authorization, duplicate provisioning,
audit-write rollback (including binding), corrupt policy rejection, concurrent
independent-connection power changes and unsupported schema versions using
isolated temporary DBs. Control tests verify unbound and ownership-revoked
sockets can still be powered off without executing normal handlers.

`bind_bot(socket, actor, bot_id, bots)` requires both socket and bot ownership.
Bindings are durable and immutable; a conflicting rebinding fails. The dispatcher
rechecks the stored socket owner's bot authority before normal command admission.
A later bot ownership transfer blocks normal commands until a dedicated
recovery/transfer flow is implemented. Socket owners retain power control even
without a bot binding or after bot ownership changes. Powering on alone does
not authorize normal execution; powering off always closes normal admission.
Ownership checks across these two stores are not a distributed transaction; revocation does not cancel requests already admitted.

`control::ControlDispatcher` accepts the receiving socket from a trusted adapter,
intercepts control messages before consulting bot bindings or normal routing.
For normal messages it applies power/policy gating, then resolves its bot internally.
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

## Persistent global fallback

Trusted provisioning calls `provision_global(authenticated_admin)` once. This
creates only global policy administration authority, never socket ownership.
The initial policy is absent (deny where neither group nor socket overrides
apply). Duplicate provisioning fails; the application must authenticate and
authorize bootstrap and must not expose it directly to untrusted callers.

`set_global_policy(actor, Option<bool>)` requires the stored global administrator.
The state mutation and audit insert share an immediate transaction. `None`
removes fallback. A global administrator cannot change socket power, socket
policy or group overrides without separately owning the target socket.
`global_audit_history(actor, after_id, limit)` checks that administrator in the
same read transaction as retrieving 1–1,000 records per page.

`resolve_persisted(socket, group)` reads power, group, socket and global policy
in a single SQL snapshot. Missing global configuration is equivalent to no
fallback; malformed stored values fail closed. The dispatcher constructor
`new_with_persistent_policy(activation, bots, router)` uses that query on each
normal admission, so changes through independent connections take effect
without rebuilding the dispatcher. Already admitted handlers are not cancelled.

`resolve(..., global_default)` and `ControlDispatcher::new(..., global_default)`
retain their explicit static-policy semantics and do not use persisted global
policy. Choose one mode deliberately in trusted application setup. No inbound
message may choose the policy source. Global updates use the administrative
API; the reserved message grammar remains exclusively power on/off.

Pending: interchangeable storage, ownership transfer, network connection lifecycle,
authenticated adapter ingress and production integration.
These remain prerequisites for the real WhatsApp activation test.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
