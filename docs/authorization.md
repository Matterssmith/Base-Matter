# Scoped role authorization

Matter-Base defines opt-in, durable role rules independent of individual
messaging platforms. The implementation is in
[src/authorization.rs](../src/authorization.rs) and is exercised by
[tests/authorization.rs](../tests/authorization.rs).

- A trusted application provisions a scope and its owner after authenticating
  the underlying bot or resource ownership. Provisioning never transfers owner.
- That exact owner can define roles, add direct grants, revoke grants, install
  explicit allow or deny rules and read a bounded immutable audit.
- Role inheritance is an immutable parent chain with a maximum depth of 16.
  Each scope supports at most 128 roles; each user has at most 32 direct grants;
  each role supports at most 256 resource/action rules.
- Wildcard resource/action is exactly "*". Any deny matched by any direct or
  inherited role overrides all allows. No match means deny.
- Policy checks use a single SQLite read transaction, with no network service
  calls. Mutations and audit writes share one immediate transaction, so an
  audit failure rolls back the corresponding mutation.
- The adapter RoleCommandAuthorizer implements CommandAuthorizer by checking
  the normalized/canonical command name as "command:<name>" and "execute".
  Scope is the bot instance ID; subject is the authenticated actor ID supplied
  by the host adapter. Neither source/group permissions nor socket ownership
  can be inferred from that tuple.
- Scope owner status never bypasses command denial. Bot owner transfer does
  not automatically transfer authorization-scope ownership. Integration must
  handle that trusted transition explicitly before production.
- Role data is local SQLite policy state; no sensitive platform session,
  external secret or live Matter-Vault integration is provided.

Security boundaries: never invoke provision_scope with untrusted user inputs.
The application binds actor IDs and bot IDs to authenticated sessions before
routing commands, and applies SocketActivationStore/ControlDispatcher first.
This role module does not replace the platform authorization and consent
requirements.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
