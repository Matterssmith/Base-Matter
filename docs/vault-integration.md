# Matter-Vault integration evidence

## 2026-11-18 — Vault-Matter identity and scoped role roadmap

The real private repository is now named `Matterssmith/Vault-Matter`
(branch `matter`). Its `vault/manifest.json` was updated to name
Vault-Matter and reference `https://github.com/Matterssmith/Base-Matter`;
its README explicitly documents encrypted socket/group role persistence.
No production grants, encryption master keys or platform credentials were
created or uploaded.

The alpha controller now holds socket/group grants in an independent
locally persistent `RoleStore`. `user` has no explicit grant; setting a
new rank replaces the prior grant transactionally. The existing Base
`EncryptedControlOutbox::enqueue_role_snapshot` encrypts role snapshots
using XChaCha20-Poly1305 and zeroizes the plaintext; the new regression
test `tests/socket_vault_outbox.rs` exercises this with an in-memory
test key and fake sink, confirming the remote envelope contains no
plaintext subject identifier and that a return to `user` removes grants.

**Not implemented / not safe to claim:** verified remote GitHub
publication via a least-privilege authenticated sink, provider binding and
key loading, revision CAS, remote read/decrypt/recovery, conflict and
revocation handling, automated enqueue after each live role mutation.
A local acknowledgment from WhatsApp says `Vault-Matter aún no
sincronizado`. Do not publish assignments in clear or generate synthetic
production keys just to satisfy a release check.

# Matter-Vault integration evidence

The actual private repository was fetched over the environment's existing SSH
access and inspected at `072c60a288d0773020f3460a639e77f84bd76c4c` on `matter`.
The GitHub connector returned 404; SSH access succeeded. No Vault records,
credentials or keys were modified. There is no Cargo project or executable
service in that revision: it contains a manifest, version metadata and the
version-1 encrypted-envelope JSON schema.

The Base encryption module and Vault schema both describe XChaCha20-Poly1305,
URL-safe Base64 without padding, and roles/permissions/configuration/audit/
encrypted_key records. The opt-in repository test reads those real metadata
files and checks field/constant/type agreement plus ephemeral local crypto
roundtrips. It is not a complete JSON Schema validator, remote key-service test,
production-key recovery test or proof of authenticated administrative access.

On 2026-11-17, read-only `git ls-remote` confirmed that `matter` still points
to that exact revision and no `matter-beta` branch is advertised. The local
checkout is clean and contains only the README, ignore rules and those three
metadata files. The explicit repository test passed (1 test); it did not use
production keys, write Vault records or contact a key service. Live verification
requires the external trusted key/binding source and its authenticated access
contract; neither is provided by this repository snapshot.

Run after inspecting and synchronizing both repositories:

```sh
MATTER_VAULT_CHECKOUT=/home/Sakura/Matter-Vault cargo test --test vault_repository -- --ignored
```

No bootstrap keys or plaintext platform credentials may be stored in Vault.
Implement the existing BoundKeyProvider/ProviderBindingStore contracts using a
trusted external source before claiming live key-service integration. Preserve
the existing envelope/AAD contract and check migrations for any format changes.
Control commands operate in Base; they must not assume that successful metadata
inspection grants Vault permissions or authenticates platform users.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

## Encrypted outbox contract

`EncryptedControlOutbox` queues already encrypted roles/configuration/
permissions/audit payloads and exposes only `EncryptedEnvelope` to an external
`EncryptedEnvelopeSink`. Role snapshots are owner-scoped and serialized from
one database snapshot. Fingerprints use HMAC with the active key and bind the
scope and record type; they are not public raw hashes. Repeating identical
content under the same key skips publishing, and rotation triggers new
ciphertext. The sink must implement authenticated remote compare-and-swap,
idempotency by envelope ID and access isolation. This repository does not ship
a GitHub App transport, a production Vault key source or encrypted recovery.
Lost acknowledgements or concurrent publishers may require remote reconciliation;
no exactly-once claim is made.


## Lost-acknowledgment reconciliation

`EncryptedControlOutbox::reconcile` receives a trusted
`EncryptedEnvelopeReader` and checks the exact remote envelope (every
authenticated metadata/ciphertext field) against the local queued envelope.
If the data is identical, it stores the remote revision and marks the local
entry synchronized with an immutable audit record. Missing remote data
leaves it pending. Divergent or malformed remote ciphertext fails closed;
no automatic overwrite occurs. The remote reader must authenticate the
provider and enforce access isolation. This mechanism handles uncertain
publish acknowledgments, not complete remote state import, secret recovery,
or concurrent publisher arbitration.
