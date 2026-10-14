# Change: Consolidate artifact mechanisms into one workspace

## Why

Artifact authentication, transfer geometry, and versioned binding were extracted as separate repositories even though their reusable cores now share the same `no_std + alloc` posture, Rust toolchain, immutable-revision distribution model, and owner-authorized `MIT OR Apache-2.0` license. Keeping build, lint, lifecycle, and release infrastructure separate adds source and maintenance drift without adding an authority boundary that cannot be preserved at the package level.

## What Changes

- Establish `OnixResearch/artifact` from the existing artifact-auth history and retain the old artifact-auth revision as a compatibility source.
- Import `artifact-transfer-core` and its extraction/pilot evidence at an exact source revision under the owner-authorized workspace license.
- Move the active artifact-binding design package into the workspace while keeping the crate explicitly unimplemented until both consumer pilots pass.
- Share Rust, Cargo, Nix, Nickel, lint, license, and lifecycle infrastructure without introducing an umbrella crate or new cross-core dependencies.
- Add typed source, package, license, dependency, claim-boundary, and consumer-migration checks.

## Impact

- **Files**: workspace manifests, Nix/Nickel contracts, provenance, release metadata, documentation, transfer source, binding Cairn artifacts, and root workspace maps.
- **Compatibility**: existing Molten, Mantle, and Valence artifact-auth pins remain unchanged; each later source migration requires a repo-local Cairn change and generated Cargo/Nix locks.
- **Testing**: baseline and post-migration Rust tests, formatting, Clippy, host/Wasm `no_std` checks, Nickel positive/negative contracts, Cairn gates, and Nix checks.
