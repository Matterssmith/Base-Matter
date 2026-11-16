# Matter-Base

Matter-Base is a Rust library under development for reusable infrastructure
in the Matterssmith ecosystem. Its current implementation and limitations are
documented in [`README.md`](README.md) and
[`docs/DEVELOPMENT_STATUS.md`](docs/DEVELOPMENT_STATUS.md).

Applications are intended to depend on Matter-Base through public contracts.
The crate currently provides a runtime/context foundation, in-memory events
and typed services, translation lookup, SQLite legal-consent storage, and
security primitives/provider contracts. It does not yet provide bot
registration, application users, roles/permissions, routing, sockets, plugin
execution, or sandbox isolation.

Repository: <https://github.com/Matterssmith/Matter-Base>

Development branch: `matter-beta`.
