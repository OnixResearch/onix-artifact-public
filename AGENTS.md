# Agent Notes

## Repository purpose

`artifact` is the shared Rust/Nix workspace for narrow product-neutral artifact mechanisms. It owns common toolchain, license, lint, dependency-resolution, lifecycle, and release-check infrastructure without merging the mechanisms' semantics or authority.

## Package boundaries

- `artifact-auth-core` and `artifact-auth-ed25519` own canonical authentication statements, explicit supplied observations, bounded policy evaluation, and pure Ed25519 verification. They do not own signing, private keys, trust discovery, authorization, storage, lifecycle, or release decisions.
- `artifact-transfer-core` owns explicit chunk geometry, checked coverage/offsets, consumer bounds, empty policy, and tagged BLAKE3/SHA-256 digest identity. It does not own wire formats, chunking/hashing bytes, transforms, transport, storage, or integrity claims.
- `artifact-binding-core` remains design-stage. Its pure core may validate explicit binding revisions, resolve one immutable snapshot, traverse supplied roots/edges, explain pin paths, and classify retirement conservatively. It does not own hashing, compatibility truth, persistence, atomic mutation, authority, receipts, deployment, retention, GC, or deletion.

## Workspace invariants

All reusable cores are `#![no_std] + alloc`, take already-loaded values, forbid unsafe code, and remain testable without filesystem, environment, clock, process, network, or ambient mutable state. `artifact-auth-ed25519 -> artifact-auth-core` is the only admitted cross-package dependency until a reviewed Cairn change proves another edge necessary.

Do not add an umbrella re-export crate, universal artifact identifier, product dependency, sibling-path consumer dependency, private-key API, or ambient default. Consumers must pin one immutable Git revision and align Cargo/Nix source identity.

Every behavior or validation change requires positive and negative tests. Include malformed values, empty/boundary collections, overflow, stale observations, algorithm confusion, incomplete inventories, and authority/claim widening as applicable.

Use BLAKE3 for workspace-owned content identity. Preserve explicitly labeled algorithms when interoperability requires SHA-256 or another standard.

## Primary checks

```console
nix develop -c cargo fmt --all -- --check
nix develop -c cargo test --workspace
nix develop -c cargo clippy --workspace --all-targets -- -D warnings
nix develop -c cargo check -p artifact-auth-core --no-default-features --target wasm32-unknown-unknown
nix develop -c cargo check -p artifact-auth-ed25519 --no-default-features --target wasm32-unknown-unknown
nix develop -c cargo check -p artifact-transfer-core --no-default-features --target wasm32-unknown-unknown
nickel typecheck config/repository.ncl
nickel typecheck release/profile.ncl
nix run path:../cairn#cairn -- validate --root . --policy cairn-policy/generated/cairn-policy.json
nix flake check path:$PWD -L
```

## Lifecycle

Native Cairn content lives under `cairn/`. Public compatibility, mutation, authority, evidence, distribution, or architecture changes require a Cairn change before implementation. Validate proposal, design, and tasks gates with the checked-in policy.

Do not edit `Cargo.lock` or `flake.lock` manually. Regenerate them only with Cargo or Nix. Do not claim consumer migration, immutable publication, or release readiness until the owning product and repository checks provide that evidence.
