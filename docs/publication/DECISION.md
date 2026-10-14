# Durable file publication extraction decision

<!-- r[impl durable_file_publication.extraction.boundary] -->
<!-- r[impl durable_file_publication.extraction.promotion] -->
<!-- r[verify durable_file_publication.validation.evidence] -->

## Disposition

**EXTRACT-READY FOR A SEPARATE REPOSITORY.**

Artifact does not add a publication package. Mantle and OnixOS do not gain a dependency in this change.

The reviewed disposable candidate is Artifact commit `67f14770a82e24eb39d3a751285b908232566e6b`.

## Why the boundary is viable

One product-neutral request preserves the selected Mantle and OnixOS mechanics without product fields:

- One caller-opened parent capability.
- One validated regular-file destination leaf.
- Explicit payload and collision bounds.
- Explicit final permissions.
- Replace or race-free no-replace.
- Visibility-only or durability-required completion.
- Typed pre-commit failure, cleanup, destination-exists, durable commit, and partial-commit outcomes.

The pure core and shell are independently useful. They depend on `rustix`, not on Artifact, Mantle, OnixOS, Valence, Cairn, or another stack product.

The 17-case corpus passes through the shell and both adapters. The Linux integration case proves actual create-new, no-follow staging, exact permissions, `NOREPLACE`, destination preservation, payload sync, and parent `fsync`.

## Promotion gates

| Gate | Result | Evidence |
|---|---|---|
| Two materially different pilots | Pass | `docs/publication/PILOTS.md` |
| One shared positive and negative corpus | Pass | Corpus BLAKE3 `d02379677341b7408b636906c4bee4910ef3835cc81261373194ef84678b27da` |
| Pure deterministic state core | Pass | `fixtures/publication-candidate/src/core.rs` |
| Thin capability-relative shell | Pass on Linux/Android | `fixtures/publication-candidate/src/shell.rs` |
| Race-free no-replace | Pass on Linux/Android | `rustix::fs::renameat_with(NOREPLACE)` |
| Real parent synchronization | Pass with the caller-opened Linux/Android directory file | `rustix::fs::fsync` |
| Source and license inventory | Pass for clean-room pilot | `docs/publication/SOURCE-INVENTORY.md` |
| Acyclic dependency shape | Pass | Candidate is below both consumers and has no product dependency |
| Independent release value | Pass | Core, shell, platform matrix, corpus, and adapter contract stand alone |
| Permanent consumer adoption | Not performed | Requires later repo-local Cairns |

## Exact mismatches retained

### Mantle

Mantle owns ambient parent selection today. A later adapter must open that authorized parent before it calls the shared shell.

Mantle also owns identical-existing-content comparison. The shared API returns only `DestinationExists` and cleanup state.

### OnixOS

The selected OnixOS `sync_directory` is a no-op. Existing behavior maps to `VisibilityOnly`. It cannot map to durable success.

OnixOS has race-free no-replace on Linux and Android. Its non-Linux inspection-then-rename fallback is not admissible. A later adapter must return unsupported there.

The candidate Linux shell accepts a standard directory file. A permanent OnixOS adapter must supply an equivalent synchronizable capability handle. It must not extract an ambient path from `CapabilityRoot`.

### Corroborating systems

Nickel Export generation pointers and transaction markers stay out of scope. Valence two-output publication remains explicitly non-atomic and stays out of scope.

## Platform matrix

| Platform | Replace | No-replace | Parent sync | Candidate status |
|---|---|---|---|---|
| Linux | `renameat` | `renameat_with(NOREPLACE)` | Directory `fsync` | Pilot passed |
| Android | Same compiled primitive set | Same compiled primitive set | Directory `fsync` | Compiled design; not executed here |
| Other targets | No shared shell implementation | Unsupported | Unsupported | Blocked until exact primitives exist |

The `no_std` core passes `wasm32-unknown-unknown`. This does not make the filesystem shell available on WebAssembly.

## Rollback and recovery

Before rename, the shell attempts stage cleanup and reports cleanup failure. The consumer can retry cleanup under its own policy.

After rename, the candidate never claims rollback. If parent synchronization fails, the result is `CommittedDurabilityUnknown`. The consumer must treat the new destination as visible, inspect current state, and apply product recovery.

A consumer can retain its current implementation and dependency pin as the adoption rollback path. Adoption must not delete the old adapter until positive, negative, and partial-commit replay passes.

## False completion cases

These cases must never report ordinary success:

- Request or payload bounds fail before I/O.
- Stage name is unsafe.
- Exact no-replace is unsupported.
- Required parent synchronization is unsupported.
- Destination exists under no-replace.
- Write, permission, payload-sync, or rename fails.
- Cleanup fails after a pre-commit failure.
- Parent synchronization fails after rename.

## Non-claims

This decision does not prove filesystem correctness, storage-device persistence, power-loss survival, host integrity, payload truth, destination authorization, consumer integration, deployment success, or release eligibility.
