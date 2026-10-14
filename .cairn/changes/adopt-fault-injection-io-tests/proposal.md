## Why

Artifact owns product-neutral artifact mechanisms: authentication statements, transfer geometry, and binding or retirement mechanics. Its failure paths — failed source read while digesting, failed chunk read during coverage checks, failed signature or key file read — mark where a statement stops being provable. Today those paths are covered only incidentally.

The komora-io audit (2026-09-09) selected `fault-injection` (Apache-2.0, crates.io) as the stack-wide dev-dependency for deterministic `io::Error` injection with crate, file, and line annotation. The durable-file-publication pilot `adopt-fault-injection-io-tests` proves the fixture pattern first. This change reuses that pattern for the artifact shells.

## What Changes

- Add `fault-injection` as a pinned dev-dependency of the shell test targets.
- Wrap the shells' I/O call sites — source reads for digesting, chunk reads, signature and key file reads — with `fallible!` in test builds.
- Add negative fixtures for each wrapped site asserting the declared bounded failure and that no statement is produced from an unreadable input.
- Add one positive fixture proving unchanged verification and geometry behavior with the default counter.
- Record the dependency as transport `crates.io`, plane `validation`.

## Impact

Immediate outcome: every declared artifact I/O failure path becomes triggerable from a test. Durable capability contribution: fail-closed statement production becomes provable without corrupt fixtures. Maintenance owner: this repository. Repeatability evidence: fixtures set explicit counter values and assert classified outcomes from intact inputs.

## Dependencies

The durable-file-publication pilot `adopt-fault-injection-io-tests` must land first. This change copies its fixture pattern, determinism guard, and dependency-catalog entry shape.

## Non-goals

- Do not add `fault-injection` to any non-dev dependency edge.
- Do not enable `SLEEPINESS` random delays.
- Do not change statement, geometry, or binding semantics.
- Do not claim trust, transfer completion, or observation truth from injection coverage; authentication remains separate from trust and geometry from byte integrity.
