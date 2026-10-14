# Durable publication pilot mappings

<!-- r[impl durable_file_publication.pilots.mantle] -->
<!-- r[impl durable_file_publication.pilots.onixos] -->
<!-- r[impl durable_file_publication.validation.losslessness] -->
<!-- r[verify durable_file_publication.validation.corpus] -->

Both disposable adapters use `fixtures/publication-candidate`. They retain the complete mechanical disposition next to the product interpretation.

## Mantle pilot

Revision: `688781f370e8f477d900e7b96efa1ad3266a392e`

Selected source: `src/remote_attempt_log_store.rs` and `src/linux_rename.rs`.

| Mantle fact | Candidate field or result | Owner after mapping |
|---|---|---|
| Immutable object file name | `destination_leaf` | Mantle |
| Canonical object byte count | `payload_bytes` | Mantle |
| Object size limit | `maximum_payload_bytes` | Mantle |
| Private create attempts | `collision_attempt_limit` | Mantle |
| Private/final mode | `final_mode` | Mantle |
| Linux `RENAME_NOREPLACE` | `NoReplace` | Candidate mechanism |
| File and parent synchronization | `DurabilityRequired` | Candidate mechanism |
| Existing exact bytes | `ExistingContent::Identical` | Mantle adapter |
| Existing different bytes | `ImmutableConflict` | Mantle adapter |
| Manifest, digest, receipt, retention | Not mapped | Mantle |

The candidate does not interpret existing bytes. Mantle can keep idempotent existing-content comparison after `DestinationExists`.

Mantle currently opens ambient paths inside its shell. A later adoption must open the authorized parent first and pass only that capability to the shared shell.

## OnixOS pilot

Revision: `628f0b8a7036ed344aa51c74612495e28299c75a`

Selected source: `cli/src/capability_fs.rs`, especially `atomic_write_with_policy`, `rename_within_noreplace`, and `sync_directory`.

| OnixOS fact | Candidate field or result | Owner after mapping |
|---|---|---|
| Admitted final leaf | `destination_leaf` | OnixOS |
| Exact payload count and adapter limit | Byte fields | OnixOS |
| Exact, existing, or create mode selection | One resolved `final_mode` | OnixOS adapter |
| `OverwritePolicy::Replace` | `Replace` | Candidate mechanism |
| `OverwritePolicy::Refuse` | `NoReplace` | Candidate mechanism |
| Existing destination | `DestinationExists` | OnixOS adapter |
| Current no-op directory sync | `VisibilityOnly` | Explicit non-claim |
| Required real directory sync | `DurabilityRequired` or unsupported preflight | Candidate mechanism |
| Root purpose, path admission, diagnostics | Not mapped | OnixOS |

The selected OnixOS revision has race-free no-replace on Linux and Android. Its non-Linux inspection-then-rename branch does not satisfy this candidate. A later adoption must return unsupported on those targets or add a race-free primitive.

The current `sync_directory` is an intentional no-op because the selected `cap-std` descriptor can be `O_PATH`. It maps only to visibility completion. A later durability-required adapter must supply a real synchronizable parent handle.

## Shared corpus

`fixtures/publication-candidate/src/corpus.rs` contains one 17-case corpus. Its source BLAKE3 is `d02379677341b7408b636906c4bee4910ef3835cc81261373194ef84678b27da`.

The shell and both adapters preserve these cases:

- Replace and no-replace success.
- Existing destination and destination race.
- Unsafe stage name and collision exhaustion.
- Stage-create, write, permission, payload-sync, rename, and parent-sync failure.
- Cleanup success and failure.
- Unsupported no-replace and unsupported parent synchronization.
- Payload-bound rejection before I/O.

The corpus does not prove either product has adopted the candidate. No Cargo or Nix dependency was added to Mantle or OnixOS.
