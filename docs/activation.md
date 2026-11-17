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

Schema initialization is transactional and versioned as `socket_activation=4`.
Version 1/2/3 upgrades preserve existing policies, audit and bindings. Audit
rows gain an optional target ID; existing rows keep NULL, without inventing
historical targets. Version 1
sockets remain unbound until explicitly associated with a bot. No global
administrator or policy is inferred during migration.
Unknown versions are rejected. Audit records omit message bodies and credentials.
Tests cover reopening, precedence, isolation, authorization, duplicate provisioning,
audit-write rollback (including binding), corrupt policy rejection, concurrent
independent-connection power changes and unsupported schema versions using
isolated temporary DBs. Control tests verify unbound and ownership-revoked
sockets can still be powered off without executing normal handlers.

`bind_bot(socket, actor, bot_id, bots)` requires both socket and bot ownership.
Bindings are durable; a conflicting direct rebinding fails. Only the socket
owner can explicitly release a binding before establishing another one. The dispatcher
rechecks the stored socket owner's bot authority before normal command admission.
A later bot ownership transfer blocks normal commands until the socket owner
releases the old binding and binds an authorized bot, or transfers the socket
to a new owner who binds it explicitly. Socket owners retain power control even
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

Admission reads socket owner, bot binding, power and policy scopes together in
one SQL snapshot. It cannot combine old power with a newly transferred binding.
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

## Socket handover and binding recovery

`transfer_owner(socket, actor, new_owner)` requires the current socket owner
inside an immediate transaction. The application validates the intended
recipient identity. Transfer changes ownership, powers off, clears socket/group
overrides and releases the bot binding atomically with its audit row. Other
sockets, global policy and bot ownership are unaffected. The previous owner
immediately loses administrative and audit access; the new owner must bind an
owned bot, choose local policy if needed, and explicitly power on. Transfer to
the same owner is an authorized no-op. Concurrent transfers from the same owner
have one winner. A global administrator has no transfer bypass.

`release_binding(socket, actor)` requires only current socket ownership, so it
works even after bot ownership was revoked. It clears binding and local
overrides, powers off and records the released bot ID in one transaction.
No existing binding is an authorized no-op. This allows recovery without
claiming rights over the previous bot. Audit failure rolls back every affected
row for both operations.

Audit records expose `actor_id` and optional `target_id`: the bound/released bot
for binding operations, or new owner for transfer. Read targets together with
the action. Legacy rows retain absent targets. Adding public record fields may
require callers constructing struct literals to update; the 0.1 development
API is not yet declared stable. Earlier readers reject schema 4; do not downgrade
by rewriting its version. Keep a backup before deploying a schema upgrade.

Neither operation cancels already admitted callbacks, transfers platform
credentials, disconnects a network connection or proves control of a platform
account. Lost-owner recovery still requires an external authenticated recovery
policy; there is no global-owner or administrator shortcut.

Pending: interchangeable storage, network connection lifecycle,
authenticated adapter ingress and production integration.
These remain prerequisites for the real WhatsApp activation test.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## Reserved role controls

Use `ControlDispatcher::with_roles` to enable `=/>set role USER_ID ROLE_ID`
and `=/>unset role USER_ID ROLE_ID` (aliases `=` and `/>`). The authenticated
receiving socket owner must own the currently bound bot and role scope. The
role must exist. Commands work while the socket is off and never invoke the
normal router. Without a role store they fail closed. The host must serialize
owner transfers with independent role/sockets stores; no cross-store atomicity
is claimed.
