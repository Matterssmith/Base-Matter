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
