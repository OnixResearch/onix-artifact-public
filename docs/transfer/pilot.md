# Disposable consumer pilot

Date: 2026-07-18

Standalone mechanism revision: `d0ccfd947b84`

The pilot used detached disposable worktrees and a local path to the clean standalone revision. No authoritative consumer checkout, dependency manifest, lockfile, release evidence, or publication state was changed.

## Baselines

| Consumer | Reference revision | Baseline evidence |
|---|---|---|
| Molten | `70590459a218dc8e66948ab6f305a7c54142b710` | `fixed_v1_chunking_has_stable_manifest_identity`: 1 passed, 0 failed; 1,166 library tests filtered |
| ChaosControl | `c6d8ec98c222f1a60b86a94f3a734e0d78676ff1` | `cargo test -p chaoscontrol-evidence snapshot_chunk`: 4 passed, 0 failed; 47 integration tests filtered |

The authoritative Molten checkout already had an unrelated untracked Cairn change. The authoritative ChaosControl checkout already had an unrelated `.gitignore` modification and unrelated untracked Cairn/OpenSpec changes. The pilot did not touch them.

## Molten fixed-size/BLAKE3 adapter

The disposable adapter replaced `validate_fixed_chunk_lengths` with `artifact_transfer_core::validate_geometry` using:

- `ChunkingAlgorithm::Fixed { chunk_bytes }`;
- `EmptyArtifactPolicy::Allow`;
- Molten's existing `MAX_CHUNK_STORE_CHUNKS` bound;
- ordered lengths adapted from Molten-owned `ChunkRef` values.

Molten retained Preserves schemas and canonical identity, `fixed_v1` labels, BLAKE3 domain separation, chunk/root refs, transform policy, protected commitments, capability filesystem access, redb indexes, Iroh behavior, receipts, and product diagnostics.

Evidence after migration:

- `cargo test fixed_v1`: 2 passed, including a new declared-total mismatch rejection; 0 failed.
- Hegel chunk-store determinism/range/resume/no-dangling property test: 1 passed with its configured 32 cases; 0 failed.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `cargo fmt --all -- --check`: passed.

The pilot improved overflow behavior: checked summation now rejects overflow instead of relying on iterator summation. Existing focused behavior remained green.

## ChaosControl external-parts/SHA-256 adapter

The disposable adapter added geometry validation to `SnapshotChunkManifest::validate_shape` using:

- `ChunkingAlgorithm::ExternalOrderedParts`, because the schema declares ordered parts but no fixed or content-defined boundary algorithm;
- `EmptyArtifactPolicy::Reject`, preserving the snapshot format's non-empty policy;
- ordered lengths adapted from ChaosControl-owned `SnapshotChunk` values.

ChaosControl retained JSON schema/version checks, local path policy, SHA-256 parsing and hashing, file metadata/hash verification, materialization and atomic rename, snapshot storage semantics, replay evidence, readiness, and anti-overclaim policy. Actual file lengths and aggregate SHA-256 remain checked by the imperative materialization shell; metadata geometry is not promoted into integrity evidence.

Evidence after migration:

- `cargo test -p chaoscontrol-evidence snapshot_chunk`: 5 passed, including a new declared-total mismatch rejection; 0 failed.
- `cargo clippy -p chaoscontrol-evidence --lib -- -D warnings`: passed.
- `cargo fmt --all -- --check`: passed.
- `cargo clippy -p chaoscontrol-evidence --all-targets -- -D warnings` reached one unrelated pre-existing test warning at `crates/chaoscontrol-evidence/src/kernel_bundle_initrd.rs:815` (`drop_non_drop`). The same `drop(writer)` is present in the authoritative source and is outside the migrated surface.

## Mantle boundary check

Mantle reference revision `26c6880c08cd730c9e9420f92ea91a87212e492d` uses FastCDC and BLAKE3, but its protocol negotiation and profile identity are not interchangeable with Molten's fixed manifest or ChaosControl's snapshot parts. The standalone FastCDC-bound geometry has positive and negative tests, but Mantle was not migrated merely to increase consumer count. A later Mantle adapter must start from an actual ordered-length manifest and retain FastCDC implementation/version or gear-table ownership, wire negotiation, closure planning, and transfer tallies in Mantle.

## Findings

The pilot supports a narrow extraction and falsifies a universal artifact-transfer manifest:

1. Fixed, FastCDC-bound, and external ordered-part geometry can share checked length/offset mechanics only when the mode is explicit.
2. Empty-artifact policy differs and must remain an adapter input.
3. BLAKE3 and SHA-256 identity must remain tagged; equal bytes are not equal digests across algorithms.
4. Geometry validation cannot replace byte hashing, reconstruction, transport, storage, authorization, evidence, or lifecycle checks.
5. Consumer diagnostics may wrap the typed shared error, but serialized schemas and product evidence remain consumer-owned.

Independent VibeThinker review was attempted after the pilot and returned `fetch failed`; standalone and consumer checks provide the recorded evidence.
