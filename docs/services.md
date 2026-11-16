# Service registry

`ServiceRegistry` is an in-memory, thread-safe registry keyed by Rust
`TypeId`. It accepts values that implement `Any + Send + Sync`, stores them in
an `Arc`, and returns typed `Arc<T>` handles. Registering another value of the
same concrete type replaces the prior value. `remove`, `contains`, `len`, and
`clear` operate on the registry.

`MatterContext` shares one registry through `Arc<ServiceRegistry>`.
Registration and retrieval are synchronized. If a lock is poisoned by an
unwinding panic, the registry recovers its inner map and remains usable; service
registration and removal mutate the map through single lock-protected
operations. The registry does not start, stop, health-check, persist, or
authorize services. Those behaviors are not implemented.

This is a core utility, not a provider or network connection manager. Avoid
putting platform credentials or secret material into generic services unless
the consuming application has an explicit protected secret-handling design.
