# Plugin trust and sandbox boundary

The plugin host is an opt-in platform-independent contract and **not an OS
sandbox implementation**. It includes a manifest loader, trusted installation
hook and per-invocation authorization and delegates untrusted execution to a
mandatory PluginSandbox provided by the application.

Manifest format fields: plugin_id, version, runtime (rust/cpp/go/python/node),
relative traversal-free entrypoint, command, aliases and capabilities. Input
is bounded to 16 KiB, 32 capabilities and 256 directory manifests.

Trust requirements:

- PluginTrust::approve_manifest must authenticate publisher identity,
  independently verify a pinned revision or signed artifact, enforce its
  capability allowance and reject revoked publishers.
- PluginTrust::authorize_execution must verify the authenticated actor, bot,
  session, legal consent and permissions on each command invocation.
- PluginSandbox::execute must start code only inside a real process/container
  or WASM boundary with explicit filesystem/network/process constraints,
  hard resource limits, cancellation, and no inherited credentials. Matter-Base
  does not yet ship or verify such a backend.
- DenyPluginSandbox is a secure default and denies execution.
- Commands run through CommandRouter's existing authorization and optional
  audit first; callbacks are outside registry locks. Errors are sanitized.

Directory loading pre-validates all manifests before installation, but host
router registration can partially succeed if a concurrent registration or
another later install fails. Deployment orchestrators must serialize plugin
installation and implement rollback/recovery or atomically swap registries.
Never advertise isolated execution from a test-only mock backend.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
