# Validation evidence

Date: 2026-07-27

## Baseline

The pre-change workspace test rail passed in pueue task `632` at revision `2d98d1c37594d37aee926a953adbc4173eb0b4d2`.

## Rust core

The final focused package and workspace rails passed in task `752`:

- `cargo test -p artifact-binding-core`
- `cargo clippy -p artifact-binding-core --all-targets -- --deny warnings`

The package has 18 unit tests and two disposable integration pilots. Positive and negative cases cover transitions, resolution, cycles, bounds, retirement, malformed attribution, and mixed snapshots.

The earlier full local workspace rail passed in task `695`, and task `752` repeated it after the final conservative-classification correction:

- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- --deny warnings`
- `cargo fmt --all -- --check`

## Nix and no-std

The focused Nix check passed in tasks `703` and `758`. The final task ran package tests, strict Clippy, and these target checks:

- `wasm32-unknown-unknown`
- `thumbv7em-none-eabi`

The authority audit passed in task `693`. The Nix workspace test check passed in task `694`.

The complete `nix flake check -L` rail passed in final task `761`. Nix reported `all checks passed` for `x86_64-linux`. The flake did not evaluate the incompatible `aarch64-linux` outputs on this host.

## Consumer pilots

The Molten mapping uses Aspen revision `935729eeb4a37b3e71e77a42d4c4e7417311fd33`.

The OnixOS mapping uses revision `628f0b8a7036ed344aa51c74612495e28299c75a`.

The reviewed source paths and mapping limits are in `docs/binding/pilot.md`. These tests prove DTO mapping only. They do not prove permanent consumer integration.

## Cairn

Proposal, design, and tasks gates passed in tasks `711`, `713`, and `712`. The design gate passed again after delta-spec normalization in task `718`.

The accepted specification was synchronized with receipt `5d134d9482079205bbdfacf0dd940dd3f4e324bbfac20d85ae99bec33ac6329b`.

The completed change was archived with receipt `9a4a669d5de4d39baf9ab36a68440ab82549e578f5546e11ea5b1f397985a228`.

The focused traceability profile covers 10 of 10 requirements. Its final receipt is `3d72dab84a147d297cfa75ce5ff1b823e68f3066ed0159ace3ac75f9c9e53c1f`.

The default traceability profile covers 24 of 24 requirements. Its final receipt is `079c846e55124e738d65786610fd93aa0f1f1049ac214aba782a55122f1a460d`.

Repository validation passed in tasks `714`, `729`, `744`, and `758` with no substance issues.

## Corrected validation failures

The first focused Nix check in task `692` used a raw offline `runCommand`. Cargo could not resolve workspace package metadata for `serde`. The final check uses `rustPlatform.buildRustPackage`, and task `703` passed.

The first broad flake run in task `715` overlapped a source update. Nix rejected the changing source hash during evaluation. A stable rerun exposed one dangling trace marker, which was corrected. Final tasks `732` and `761` passed.

## Claim boundary

The evidence proves the selected repository checks passed for the recorded source tree. It does not prove compatibility truth, root discovery completeness, storage atomicity, consumer integration, deployment success, cleanup safety, or deletion authority.
