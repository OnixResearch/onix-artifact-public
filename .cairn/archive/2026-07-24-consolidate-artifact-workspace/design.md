# Design: Consolidate artifact mechanisms into one workspace

## Success contract

The consolidation succeeds when one local `artifact` Rust/Nix workspace builds and tests the implemented authentication and transfer packages under Rust 1.88 and `MIT OR Apache-2.0`, retains the binding design without claiming an implementation, preserves exact import provenance, enforces the intended package dependency graph, and leaves existing consumer pins valid and unchanged.

False completion includes copying code without source revision and license records, silently reinterpreting the old artifact-auth source, adding sibling paths to product repositories, publishing an umbrella artifact API, sharing an unqualified identifier across mechanisms, allowing transfer or binding results to imply authentication/authority, marking binding implemented, editing generated locks manually, or claiming consumer migration before repo-local evidence exists.

## Decisions

### Use artifact-auth history as the workspace lineage

The new repository starts from the mature artifact-auth history because it already supplies the strictest typed config, source grants, consumer mappings, Nix package-set assertions, Cairn policy, and three authoritative consumers. The intended remote becomes `OnixResearch/artifact`; the old artifact-auth repository and reviewed revision remain compatibility inputs during migration.

This local construction does not claim that the new remote exists or has published an immutable revision. Publication and downstream adoption remain separately authorized steps.

### Share infrastructure, not semantics

The root workspace owns Rust 1.88, edition 2024, `MIT OR Apache-2.0`, `publish = false`, lint policy, dependency resolution, Nix checks, typed repository metadata, and Cairn lifecycle. Authentication, transfer geometry, and binding/retirement remain separate packages, specifications, diagnostics, tests, and non-claims.

`artifact-auth-ed25519` may depend on `artifact-auth-core`. `artifact-transfer-core` remains independent. The planned `artifact-binding-core` remains independent. No umbrella crate or universal artifact reference is introduced.

### Record relicensing and import provenance explicitly

The owner-authorized transfer source is imported from revision `507b10823c4513064710a70fe565f9a1ee592985`. `provenance/SOURCE_GRANT.md`, the source inventory, README, package metadata, and generated repository/release profiles record the workspace license and bounded provenance. Third-party dependencies retain their existing licenses.

### Keep consumer migration receiver-local

Molten, Mantle, and Valence continue to consume the historical artifact-auth revision until each repository reviews an immutable Artifact workspace revision. A migration must align Cargo and Nix identities, admit only the intended package set and license, preserve product authority and old-source rollback, and run focused positive and negative behavior tests. The workspace cannot grant that acceptance.

### Keep binding design-stage

The active binding change moves into the workspace and is updated from “standalone repository” to “independent workspace package.” No crate is added during consolidation. Its existing two-pilot, incomplete-inventory, stable-path, and no-deletion-authority requirements remain prerequisites.

## Risks and mitigations

- **Workspace becomes an authority monolith:** enforce package-level specifications, dependency allowlists, and explicit non-claims.
- **Source identity breakage:** preserve old pins and require immutable receiver-local migration evidence.
- **License ambiguity:** record the exact transfer revision and owner direction; verify every package reports `MIT OR Apache-2.0`.
- **Accidental lockstep semantics:** package APIs and lifecycle capabilities remain independent even though the source revision and checks are shared.
- **Premature binding claims:** omit the package from workspace members until its active Cairn tasks and two pilots pass.

## Validation

Validation compares baseline authentication and transfer tests with unified workspace results, checks formatting and Clippy, compiles each implemented package for host and `wasm32-unknown-unknown` without default features, exports and compares typed Nickel records, runs negative metadata/authority fixtures, validates both active Cairn changes and their gates, and runs the broad Nix flake check when the generated lock and repository snapshot are current.
