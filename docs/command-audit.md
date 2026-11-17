# Command audit contracts

`CommandAuditStore` is an application-owned SQLite store injected through
`CommandRouter::new_with_audit(prefixes, authorizer, audit)`. The default `new`
constructor preserves its unaudited behavior. No network, platform session,
credential, key material or Vault record is stored here.

## Admission and outcomes

After validating context/message limits, parsing and resolving a registered
canonical command, the router commits an attempt before any authorization,
middleware or handler callback. It records bot, actor, source, canonical command
and timestamp; it omits arguments, message bodies, responses and diagnostics.
The application must establish actor/source/bot context from authenticated ingress.

A single terminal row records `Denied`, `MiddlewareRejected`, `Failed` or
`Succeeded`. Callback errors and unwinding panics are covered. A missing outcome
is reported as `Pending`, including after reopening the database. Callbacks run
outside the audit connection lock and may make reentrant calls.

An attempt-write failure prevents all callbacks. An outcome-write failure
returns a sanitized persistence error but cannot roll back handler effects.
`Failed` may also follow partial effects. Neither status authorizes automatic
retry. There is no exactly-once guarantee, distributed transaction, automatic
replay or automatic reconciliation. Wall-clock timestamps are informational;
attempt IDs determine pagination order, not completion order.

Non-command, invalid and unregistered messages are excluded. When the router
is used through `ControlDispatcher`, power/policy/binding rejections happen
before routing and create no command attempt. Authorized control changes use
the activation store's atomic state/audit transaction instead. That administrative
transaction remains distinct from arbitrary command handler effects.

## Reading and persistence

`history(bot, actor, bots, after_id, limit)` requires the current bot owner,
filters by bot and returns increasing IDs after a nonnegative cursor. Page size
must be 1–1,000. A foreign owner cannot inspect records; ownership transfer
revokes the previous owner's reads. Records of deleted bots remain stored but
are inaccessible through this API. The application supplies the trusted bot
registry; actor IDs are not authenticated by the audit store itself.

Bot authorization and audit reads span separate stores, so revocation does not
cancel an admitted read. Each page uses a single SQLite read snapshot; a later
page is not a frozen snapshot. A pending attempt can acquire an outcome after
it was read; re-read its earlier page to reconcile it.

Schema creation, triggers and component version `command_audit=1` commit in
one transaction. Unsupported versions are rejected without rewriting them;
failed DDL rolls back. Independent connections allocate unique attempt IDs and
persist matching outcomes. Triggers reject updates, deletes and replacement
inserts on existing attempts/outcomes. Database administrators can remove
triggers or rewrite files; this is not cryptographic tamper evidence.

Parent directories must exist and storage access must be restricted by the
application. Storage grows with command attempts; archival/rotation and disk
monitoring are operational responsibilities. No automatic retention/deletion or
global audit aggregation is implemented. SQLite busy/disk failures propagate;
if initial audit persistence fails, execution is denied.

Tests exercise cross-module activation/router/audit integration, callback
failures, start/outcome write failures, restart recovery, owner isolation and
transfer, bounded pages, concurrent connections, immutable records and schema
rollback/future-version rejection. They do not verify a live platform or Vault.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
