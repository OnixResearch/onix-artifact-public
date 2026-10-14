# Durable publication source inventory

<!-- r[impl durable_file_publication.extraction.provenance] -->

This inventory records behavior that informed the clean-room pilot. The pilot does not copy implementation text from the reviewed repositories.

## Reviewed revisions

| Repository | Revision | License surface | Reviewed files and functions | Product-owned semantics |
|---|---|---|---|---|
| Mantle | `688781f370e8f477d900e7b96efa1ad3266a392e` | `AGPL-3.0-or-later` in `Cargo.toml` | `src/remote_attempt_log_store.rs`: `publish_immutable_file_with_hook`, `write_identity_owned_temp`, `commit_manifest_with_hook`, `ensure_durable_dir`; `src/linux_rename.rs`: `rename_no_replace`, `rename_path_no_replace` | Immutable object identity, canonical JSON, bounded equality on existing content, manifests, receipts, retention |
| OnixOS | `628f0b8a7036ed344aa51c74612495e28299c75a` | `AGPL-3.0-or-later` in `cli/Cargo.toml` | `cli/src/capability_fs.rs`: `atomic_write_with_policy`, `rename_within_noreplace`, `publish_temporary_payload`, `sync_directory` | Root opening, path admission, root purpose, overwrite policy, existing/create mode selection, diagnostics |
| Nickel Export | `257fafc1c746f1faf156207043a4c826bfb16d49` | `AGPL-3.0-or-later` in the exact remote workspace `Cargo.toml` | `crates/nickel-export/src/lib.rs`: generation-pointer and recoverable transaction publication | Export identity, generation pointers, transaction markers, recovery protocol, evaluator policy |
| Valence | `6ab37aa34c9f81da812d6b42f2f06b7ac2d7e214` | CLI workspace `AGPL-3.0-or-later`; `valence-core` is `MPL-2.0` | `crates/valence-cli/src/main.rs`: `cap_atomic_write`, `cap_atomic_write_pair`, `stage_atomic_write` | Evidence bundle shape, output rollback, compatibility projections, non-atomic pair diagnostics |
| Cap Root | `8991e2572578c16e590c2f1c0915db6c46c5d9cb` | `AGPL-3.0-or-later` in `Cargo.toml` | Pure lexical root and leaf admission APIs | Capability opening, no-follow I/O, authorization, diagnostics, evidence |
| Artifact binding core | `25cd1235e25619290f744cffc9654d2414b2d08d` | `MIT OR Apache-2.0` | `crates/artifact-binding-core`: pure transition and snapshot checks | Persistence, atomic mutation, compatibility truth, deployment, retention, deletion |

The Nickel Export revision is also bound by Mantle's `config/nickel-export-core-source.ncl` and generated JSON source record.

## Copyright and reuse disposition

The reviewed product repositories publish the license terms listed above. This record does not prove that Artifact can relicense their implementation expression.

The disposable candidate was written from the normative Cairn contract and observed behavior. It uses `rustix` APIs instead of copied syscall wrappers. No reviewed function body, diagnostic text, staging-name format, product DTO, or receipt format is copied.

A separate repository must use terms compatible with any source expression it later imports. The handoff recommends `AGPL-3.0-or-later` unless every imported component has an explicit compatible grant.

## Boundary findings

- Mantle's existing-content equality is product policy. The shared outcome is only `DestinationExists`.
- OnixOS currently calls a directory-sync helper that is an intentional no-op. Its existing operation maps to visibility-only completion.
- OnixOS has a race-free Linux/Android no-replace path. Its non-Linux inspection-then-rename fallback cannot satisfy the candidate no-replace contract.
- Nickel Export and Valence multi-file behavior is outside the one-file scope.
- Cap Root and Artifact binding remain lower pure mechanisms. Neither owns publication I/O.
