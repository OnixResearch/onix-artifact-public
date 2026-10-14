# Artifact Workspace Specification

## Purpose

Defines the `artifact-workspace` capability.

## Requirements

### Requirement: Workspace substrate is uniform

r[artifact_workspace.substrate.uniform] The workspace MUST use Rust 1.88 with edition 2024, MUST license every repository-owned package as `MIT OR Apache-2.0`, MUST disable crates.io publication, and MUST apply shared unsafe-code, documentation, and Clippy policy while retaining package-specific behavior and specifications.

#### Scenario: Implemented packages conform

- GIVEN the workspace metadata and implemented package manifests
- WHEN Cargo and Nickel metadata checks run
- THEN every package MUST report the selected Rust version, edition, license, publication policy, and workspace lint inheritance.

#### Scenario: Package metadata drifts

- GIVEN a package declares an incompatible Rust version, license, edition, publication channel, or lint escape
- WHEN workspace validation runs
- THEN validation MUST fail before release evidence is produced.

### Requirement: Mechanism dependencies remain narrow

r[artifact_workspace.dependencies.narrow] The workspace MUST admit `artifact-auth-ed25519` depending on `artifact-auth-core`, MUST keep transfer and binding cores independent of authentication and each other, and MUST NOT publish an umbrella re-export crate or universal identifier that erases algorithm, schema, or consumer semantics.

#### Scenario: Allowed authentication edge is present

- GIVEN the workspace dependency graph
- WHEN package relationships are audited
- THEN the verifier MAY depend on the authentication core without gaining signing, trust, or authorization behavior.

#### Scenario: Cross-mechanism dependency is introduced

- GIVEN transfer or binding depends on an authentication package, another product repository, or an umbrella package
- WHEN the dependency audit runs
- THEN validation MUST fail unless a later accepted Cairn requirement explicitly replaces this allowlist.

### Requirement: Imported source and relicensing are explicit

r[artifact_workspace.provenance.imports] The workspace MUST record the exact artifact-auth history baseline, artifact-transfer import revision, binding design identity, owner-authorized license grant, and bounded material disposition, and MUST retain third-party license boundaries.

#### Scenario: Provenance is complete

- GIVEN imported source or design material appears in the workspace
- WHEN provenance validation runs
- THEN its source repository, revision or bounded working-copy identity, role, workspace license disposition, and non-claims MUST be reviewable.

#### Scenario: Source or grant is missing

- GIVEN imported material lacks an exact source identity or owner-authorized disposition
- WHEN release validation runs
- THEN the workspace MUST fail closed rather than infer permission from repository proximity.

### Requirement: Historical consumers remain compatible

r[artifact_workspace.migration.receiver_local] Existing immutable artifact-auth revisions MUST remain valid compatibility sources, and migration to an Artifact workspace revision MUST require receiver-local Cargo/Nix source alignment, package/license admission, positive and negative behavior checks, retained product authority, and rollback evidence.

#### Scenario: Consumer remains on historical source

- GIVEN Molten, Mantle, or Valence still pins its reviewed artifact-auth revision
- WHEN the Artifact workspace is assembled
- THEN that consumer MUST remain unchanged and MUST NOT be described as stale, migrated, or nonconforming solely because a new workspace exists.

#### Scenario: Workspace creation is treated as cutover

- GIVEN no immutable Artifact revision or receiver-local migration evidence exists
- WHEN release or adoption status is evaluated
- THEN consumer migration and authority admission MUST remain false.

### Requirement: Claims remain package bounded

r[artifact_workspace.claims.bounded] Workspace metadata, tests, builds, and package composition MUST NOT promote authentication into truth or authority, transfer geometry into byte integrity or completion, or binding reachability into observation truth, cleanup, or deletion authority.

#### Scenario: Bounded package result is reported

- GIVEN any package returns a passing result
- WHEN it is documented or projected by the workspace
- THEN the package's complete non-claims MUST remain present.

#### Scenario: Cross-mechanism overclaim appears

- GIVEN documentation or metadata treats workspace membership or one mechanism's result as proof supplied by another mechanism
- WHEN claim validation runs
- THEN release validation MUST fail.
