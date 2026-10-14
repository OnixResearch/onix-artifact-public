# IO Fault Fixtures Specification

## Purpose

Give Artifact deterministic, annotated, test-only injection of `io::Error` so every declared read failure behind statement production is provable without corrupt fixtures.

## Requirements

### Requirement: Injection is test-only and pinned [r[artifact.io_fault.fixtures]]

The repository MUST confine `fault-injection` to dev-dependencies of the shell test targets. The pinned revision MUST be recorded as transport `crates.io`, plane `validation`.

#### Scenario: Runtime code stays unaware

- GIVEN the workspace dependency graph after adoption
- WHEN dependency classification runs
- THEN `fault-injection` MUST appear only on dev edges

#### Scenario: Read surfaces are wrapped

- GIVEN the shells' source, chunk, and signature or key read sites
- WHEN the test targets build
- THEN each site MUST pass through `fallible!`

### Requirement: Injection is deterministic [r[artifact.io_fault.determinism]]

Fixtures MUST store an explicit counter value before acting and MUST restore the default afterward. `SLEEPINESS` MUST stay zero.

#### Scenario: Fixture fires at a chosen site

- GIVEN a fixture stores a counter value that reaches zero at the source-read site
- WHEN digest computation runs
- THEN the read MUST return an injected `io::Error`
- AND no statement MUST be produced from the unreadable input

#### Scenario: Random delays stay disabled

- GIVEN any fixture or CI configuration
- WHEN the test suite runs
- THEN `SLEEPINESS` MUST be zero

### Requirement: Injection claims nothing beyond unit coverage [r[artifact.io_fault.boundary]]

Documentation MUST state that injection coverage grants no trust, transfer-completion, or observation-truth claim.

#### Scenario: Over-claim rejected

- GIVEN documentation claims transfer completion from injection fixtures
- WHEN boundary verification runs
- THEN the claim MUST fail verification

### Requirement: Failure paths stay covered [r[artifact.io_fault.verification]]

Positive, negative, and boundary fixtures MUST cover every wrapped site and the annotation contract.

#### Scenario: Complete matrix passes

- GIVEN fixtures for the unchanged path, each injected site, and annotation boundaries
- WHEN package, Wasm-target, workspace, Clippy, Cairn, and Nix checks run
- THEN each injected site MUST produce its declared classification
- AND the unchanged path MUST behave as before
