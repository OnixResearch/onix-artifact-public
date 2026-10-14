# Security and claim boundary

## Owned facts

- injective versioned statement preimage;
- explicit typed digest algorithms and profiles;
- raw Ed25519 public-key BLAKE3 identity;
- pure Ed25519 verification;
- explicit producer/key/purpose/generation/currentness matching;
- distinct-full-key threshold evaluation;
- deterministic issues, decisions, and required non-claims.

## Excluded authority

The repository does not generate, sign with, store, rotate, discover, fetch, or revoke keys. It does not read clocks or files, contact networks, canonicalize product payloads, choose trust roots, collect currentness, authorize producers, evaluate capabilities, inspect registries, decide readiness, or release products.

## Failure policy

Malformed schemas, text, digests, key identities, statements, keys, signatures, duplicate references, duplicate trusted full-key identities, zero or stale generations, missing currentness, unauthorized purposes, untrusted labels, failed cryptography, and unsatisfied thresholds fail closed with stable classes.

Diagnostics contain field paths and stable issue codes only. They do not include raw public keys, signatures, private material, credentials, payload bytes, local paths, or host state.
