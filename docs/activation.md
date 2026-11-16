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

Schema initialization is transactional and versioned as `socket_activation=1`.
Unknown versions are rejected. Audit records omit message bodies and credentials.
Tests cover reopening, precedence, isolation, authorization, duplicate provisioning,
audit-write rollback, corrupt policy rejection, and unsupported schema versions using isolated temporary DBs.

Pending: interchangeable storage, authenticated socket-to-bot binding, ownership
transfer, connection lifecycle, global-policy persistence, runtime integration,
control-prefix interception (`=` and `/>`), and command dispatch gating. These
remain prerequisites for the real WhatsApp activation test.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
