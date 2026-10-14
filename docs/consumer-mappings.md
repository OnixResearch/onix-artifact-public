# Consumer mapping profiles

Mappings are one-way compatibility adapters. Typed sources live in `config/consumers/*.ncl`; checked JSON projections live in `fixtures/consumers/*.json`. Positive and negative checks require every consumer profile to preserve its explicit retained-authority set. Mappings do not transfer payload canonicalization, trust selection, authorization, or release authority.

## Cairn

- Domain: `cairn.authenticated-evidence.v1` plus evidence kind maps to standalone domain/purpose.
- Subject: recomputed receipt BLAKE3 maps to a typed Cairn receipt reference.
- Parents: measured non-primary artifacts, measured policy export, and pinned Valence receipt map to typed parents/context.
- Key: configured raw Ed25519 bytes produce the standalone full-key identity; Cairn's legacy BLAKE3-of-hex identity is retained as compatibility metadata.
- Retained: filesystem measurement, canonical JSON receipt reconstruction, lifecycle policy, Valence pin freshness, readiness composition, JSON, and exit behavior.

## Mantle

- OCI trust domain and immutable manifest pair map to domain/purpose/subject/parent references.
- Nix-format signer labels map to producer/key labels only.
- Raw Ed25519 key bytes produce the standalone identity; Mantle's domain-separated encoded-key identity remains compatibility metadata.
- `minimum_signatures` maps to standalone distinct-key threshold.
- Revoked full-key identities and required signer labels remain explicit policy observations.
- Retained: OCI descriptors, repository authorization, registry routing, credentials, signing, pull ordering, build/release policy, and receipts.

## Molten

- Signature domain ID/version, purpose, payload ref, signer public ref, and verifier-context ref map directly to statement fields.
- Key generation and `Current`/`Overlap`/`Superseded`/`Revoked` map to explicit generation/currentness observations.
- `Overlap` is verification-only; signing admission remains Molten-owned.
- Retained: key backends, entropy, opaque handles, rotation writes, capability authority, federation membership, Preserves codecs, and Iroh transport.

## Valence

- A Radicle protocol-verified entry may map its actor key, entry identity, parents, related target, and profile identity into standalone observations.
- Radicle owns the signed bytes and cryptographic verification; artifact-auth does not reconstruct that protocol.
- Retained: Evidence IR identity/roles, actor authorization, COB reduction, Git object measurement, SSH-agent signing, special refs, concurrency, lifecycle handoff, and release interpretation.

## Mapping failure policy

Adapters fail closed when a required typed field has no lossless source, an algorithm label is discarded, a key label substitutes for full key identity, a requested/currentness fact is promoted to enforcement, or standalone success would widen product claims.
