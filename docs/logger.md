# Logging

`core::logger::init_logger(debug)` installs a `tracing-subscriber` formatter
with target information. It uses `RUST_LOG` when a valid environment filter
can be read; otherwise the default level is `debug` when `debug` is true and
`info` otherwise. Initialization returns an error if a global subscriber has
already been installed or subscriber setup fails.

Runtime startup emits a tracing info event. This crate does not yet define a
central redaction policy or audit log. Do not include credentials, tokens,
private keys, session contents, or other secret material in tracing fields.
