# Resource authorization contracts

`RoleStore::open(path, Arc<BotRegistry>)` provides SQLite bot-scoped policy with
mandatory current-owner administration. The application authenticates actor and
subject IDs and supplies its trusted registry once at construction. There is no
global administrator bypass and no implicit permission from names such as
`owner`, `admin` or `global-owner`. Co-ownership alone cannot administer policy.

## Policy and integration

A policy contains named `RoleDefinition` values and explicit subject-to-role
assignments. Each `ResourcePermission` selects resource type, resource ID and
action by exact, case-sensitive identifier or whole-string `*`. Partial globs
are not interpreted. Every selector remains within the request's bot instance.
Requests must contain concrete identifiers. A matching deny in any direct or
inherited role wins over all matching allows; absent grants deny.

The store validates parent existence, duplicates, cycles and maximum depth on
writes and on loading policy. Removing a role removes direct assignments but
fails atomically if another role still inherits it. Repeated assignments and
revocations are state-idempotent and still leave an administrative audit event.

`RoleCommandAuthorizer` maps a canonical router invocation to resource type
`command`, resource ID equal to the canonical command name, and action `execute`.
Aliases cannot change that mapping. Inject it using either router constructor;
with `new_with_audit`, both successful and denied recognized attempts persist.
Use `ControlDispatcher` for inbound execution so policy does not bypass power,
binding and socket ownership checks. Role grants never confer platform API
permissions. Other resources require explicit `ResourceRequest` mappings;
legal/user stores retain their independent authorizer contracts.

## Ownership, recovery and atomicity

`provision_scope` creates an empty deny-by-default policy owned by the current
bot owner. Duplicate provisioning fails. Ordinary mutations require both
current bot ownership and a matching stored policy owner. Every authorization
rechecks that the stored owner ID still owns the bot. A different owner must
explicitly `reset_scope`, which clears roles and assignments, changes the pin
and records the previous administrator. It also recovers a corrupt policy body.
Only the current bot owner can reset; existing audit is retained.

The pin is an owner identity, not an ownership epoch: if ownership returns to
the same ID before a reset, its existing policy becomes applicable again.
Applications requiring a fresh policy for every ownership transition must
coordinate reset with that transition. No ownership events or distributed
transactions are implied. Bot authorization and policy transactions use separate
stores; already admitted operations may finish after revocation. Policy state,
its pin and assignments are read together from one SQLite row/snapshot.

Administrative writes use an immediate transaction that commits the updated
policy and audit together. Audit failure rolls back provisioning, updates,
assignments, removals and reset. Independent connections serialize updates,
including cycle checks, without losing assignments. Schema `resource_roles=1`
initialization is transactional and rejects unsupported versions without rewrite.
SQL triggers reject audit updates, deletes and replacement inserts. They do not
protect against a database administrator dropping triggers or rewriting files.

## Limits and inspection

- 64 roles per bot, 128 permissions and 8 distinct parents per role.
- Maximum inheritance depth 16; 1,024 subjects and 16 direct roles per subject.
- Role names: 1–64 ASCII letters/digits/underscore/hyphen/dot. Other identifiers:
  1–128 bytes, nonblank, without control characters.
- Complete serialized policy: at most 1 MiB, enforced before JSON loading.
  Role-change audit details: at most 64 KiB; over-limit changes roll back.
- `policy(bot, owner)` exposes policy only to its current matching owner.
  `audit_history(bot, owner, after_id, limit)` returns 1–100 records per page;
  a current new owner can inspect retained history before reset. Deleted bots'
  history remains stored but inaccessible through this API.

Do not use policy identifiers or details to store secrets, credentials, platform
sessions or message content. Policy is not encrypted by this local store and
is not automatically replicated to Vault. Protect its database at the application
boundary and arrange archival/rotation as needed; audit storage grows over time.
Missing scope, invalid/corrupt policy, stale owner and storage failures return
errors; callers must deny on errors. The router sanitizes authorization failures.

Tests exercise persistent bot/roles/control/command-audit composition, canonical
aliases, inheritance and deny precedence, isolation and ownership changes,
limits, malformed data, rollback, competing updates/cycles and schema recovery.
This does not establish live identity-provider, Vault or platform integration.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
