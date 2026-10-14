# Artifact

Product-neutral `no_std + alloc` cores for artifact authentication, transfer geometry, and versioned binding/reachability mechanics.

The workspace shares a Rust toolchain, license, lint policy, dependency resolution, lifecycle, and release checks. Workspace membership does not make its mechanisms semantically interchangeable and does not transfer consumer authority into these crates.

## Start here

1. Run `nix develop -c cargo test --workspace`.
2. Choose the narrow package matching the mechanism you need.
3. Read that package's claim boundary before writing an adapter.
4. Pin an immutable Git revision; never depend on a sibling path from a product repository.

## Packages

| Package | Status | Owns | Does not own |
|---|---|---|---|
| `artifact-auth-core` | Implemented | Canonical signed statements, typed references, explicit currentness/threshold observations, bounded authentication decisions | Trust discovery, signing, authorization, storage, lifecycle, release |
| `artifact-auth-ed25519` | Implemented | Pure Ed25519 verification over exact core-produced canonical bytes | Private keys, signing, trust, revocation collection, authorization |
| `artifact-transfer-core` | Implemented | Fixed, FastCDC-bound, and external ordered-part geometry; checked coverage/offsets; explicit BLAKE3/SHA-256 tags | Wire manifests, chunking/hashing bytes, transport, storage, integrity claims |
| `artifact-binding-core` | Implemented | Checked successor binding, one-snapshot resolution, deterministic reachability, stable pin paths, conservative retirement | Artifact identity, compatibility truth, persistence, atomic swaps, deployment, GC, deletion authority |

## Workspace invariants

<!-- r[impl artifact_workspace.substrate.uniform] r[impl artifact_workspace.dependencies.narrow] -->

- Every reusable core remains `#![no_std]` plus `alloc` and receives already-loaded values.
- `artifact-auth-ed25519` may depend on `artifact-auth-core`; the other cores remain independent unless a later reviewed Cairn change proves a narrower dependency.
- No umbrella crate re-exports the packages and no universal `ArtifactRef` erases algorithm, schema, or consumer semantics.
- Consumers retain canonical product formats, compatibility policy, authority, persistence, transport, evidence, deployment, retention, and release decisions.
- Positive and negative tests travel together, including malformed input, bounds, overflow, stale observations, and authority-widening cases.

## Authentication boundary

A passing authentication decision proves only that supplied public-key material verified exact canonical statements and that supplied purpose, generation, currentness, trust, and threshold observations satisfied the selected bounded policy.

It does not prove artifact truth, producer authorization, trust-root correctness, revocation freshness, provenance, build correctness, deployment safety, or release eligibility. See [`docs/security.md`](docs/security.md), [`docs/baseline-vectors.md`](docs/baseline-vectors.md), and [`docs/consumer-mappings.md`](docs/consumer-mappings.md).

## Transfer boundary

<!-- r[impl artifact_transfer.claims.bounded] -->

A passing geometry decision proves only that supplied ordered lengths cover a declared byte count under an explicit mechanism, bounds, and empty-artifact policy. It does not prove chunk bytes, hashes, external ordering, reconstruction, transfer completion, or integrity.

The extraction decision and disposable pilots remain available under [`docs/transfer/`](docs/transfer/).

## Binding boundary

<!-- r[impl artifact_binding.core.boundary] -->

The binding core classifies only caller-supplied binding, graph, root, attribution, and completeness facts. Incomplete root inventories cannot produce a definitive retired result. Retirement evidence never grants cleanup or deletion authority.

See the [design](docs/binding/design.md), [adoption guide](docs/binding/adoption.md), and [disposable pilots](docs/binding/pilot.md).

## Publication incubation

The durable one-file publication candidate remains a disposable fixture. It is not a workspace package and does not change Mantle or OnixOS dependencies.

See the [candidate API](docs/publication/API.md), [source inventory](docs/publication/SOURCE-INVENTORY.md), [pilot mappings](docs/publication/PILOTS.md), and [extraction decision](docs/publication/DECISION.md).

## Distribution and compatibility

<!-- r[impl artifact_workspace.provenance.imports] r[impl artifact_workspace.migration.receiver_local] -->

Packages are distributed from immutable Git revisions and set `publish = false`; crates.io is not a release channel. Product repositories must align Cargo and Nix to the same reviewed source identity.

### Independent public source (NOT RELEASE-ADMITTED)

Current source discovery uses [`OnixResearch/onix-artifact-public`](https://github.com/OnixResearch/onix-artifact-public). The audited independent base is `f379f2583b5d152cfd6d58919738652861793c90`, a parentless source export of historical Artifact snapshot `2a34ddd92a4eaf57d4d0de792dfa9aa4ea105121`. Only that export and scoped metadata descendants are published; original private repositories and their histories are not this mirror's ancestry.

For the audited base, use `github:OnixResearch/onix-artifact-public/f379f2583b5d152cfd6d58919738652861793c90` with `flake = false`, or the same exact revision from `https://github.com/OnixResearch/onix-artifact-public.git` for Cargo. The base deliberately retains its historical repository metadata. Current repository/release projections on metadata descendants identify this public mirror instead; they do not rewrite the extraction source cohort, grants, or frozen evidence.

Cargo and Nix consumers must select the same immutable revision and regenerate their own locks and current source-selection projections. A changed repository URL or profile digest is not interchangeable with a historical evidence-bound identity. Receiver-local migration and its focused contract checks remain required. Artifact's live Cairn input uses a separately pinned public assembly; Cairn can consume the audited non-flake base without following Artifact's lifecycle input.

This is source-only publication, **NOT RELEASE-ADMITTED**. Anonymous source access proves neither build/test success nor source correctness, consumer migration, license authority beyond the recorded grants, provenance authentication, or release eligibility. Historical Radicle receipts and public `did:key` identities remain unchanged and make only their original bounded claims.

The standalone `artifact-auth`, `artifact-transfer`, and `artifact-binding` repositories are deprecated. Keep their immutable revisions only for source history and rollback. All active development and new consumer pins must use this workspace.

Historical `artifact-auth` revision `799459346d5416fbd7b9f55840a7371441b55afa` remains a valid compatibility source for existing Molten, Mantle, and Valence pins. Moving a consumer to this workspace requires a repo-local Cairn change, exact source/package/license checks, generated Cargo/Nix locks, focused positive and negative tests, and rollback evidence.

## Validation

Fast feedback:

```console
nix develop -c cargo fmt --all -- --check
nix develop -c cargo test --workspace
nix develop -c cargo clippy --workspace --all-targets -- -D warnings
```

Core boundaries:

```console
nix develop -c cargo check -p artifact-auth-core --no-default-features --target wasm32-unknown-unknown
nix develop -c cargo check -p artifact-auth-ed25519 --no-default-features --target wasm32-unknown-unknown
nix develop -c cargo check -p artifact-transfer-core --no-default-features --target wasm32-unknown-unknown
nix develop -c cargo check -p artifact-binding-core --no-default-features --target wasm32-unknown-unknown
nix develop -c cargo check -p artifact-binding-core --no-default-features --target thumbv7em-none-eabi
nix develop -c cargo test --manifest-path fixtures/publication-candidate/Cargo.toml
```

Full repository gate:

```console
nickel typecheck config/repository.ncl
nickel typecheck release/profile.ncl
nix run --inputs-from . cairn#cairn -- validate --root . --policy cairn-policy/generated/cairn-policy.json
nix flake check path:$PWD -L
```

### Frozen Radicle compatibility source

Historical revision `799459346d5416fbd7b9f55840a7371441b55afa` is served unchanged at `https://git.onix.computer/z4JGYYW7WsesXUq7MXVdx16Fawu2f.git`. Its typed publication receipt is [`evidence/radicle/publication-v1.ncl`](evidence/radicle/publication-v1.ncl). This compatibility publication does not replace the unified `artifact` workspace or claim current-package equivalence.

## License

<!-- r[verify artifact_workspace.substrate.uniform] r[verify artifact_workspace.claims.bounded] -->

The workspace and its repository-owned packages are available under `MIT OR Apache-2.0`.

The authorization and exact imported source revisions are recorded in [`provenance/SOURCE_GRANT.md`](provenance/SOURCE_GRANT.md) and [`docs/source-inventory.md`](docs/source-inventory.md). Third-party dependencies retain their own licenses.

## References

- [Deprecated artifact-auth history](https://github.com/OnixResearch/artifact-auth) — immutable authentication source and compatibility revisions.
- [Deprecated artifact-transfer history](https://github.com/OnixResearch/artifact-transfer) — immutable transfer-geometry source and disposable consumer evidence imported at revision `507b10823c4513064710a70fe565f9a1ee592985`.
- [`../artifact-binding/`](../artifact-binding/) — deprecated local design history with no publication source.
- [PsychoLlama/ambient](https://github.com/PsychoLlama/ambient) — non-normative design reference for immutable implementations, explicit late binding, semantic handler identity, and generation retirement tracing.
- [Molten](https://github.com/brittonr/aspen), [Mantle](https://github.com/OnixResearch/mantle), [Valence](https://github.com/OnixResearch/valence), and [OnixOS](https://github.com/onixcomputer/onix-modules) — consumers and pilots that retain product authority.
