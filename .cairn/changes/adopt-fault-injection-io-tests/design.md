## Goals

- Make every declared artifact I/O failure path of the shells triggerable from a test.
- Keep injection deterministic, annotated, and confined to test builds.
- Change no statement, geometry, or binding semantics.

## Mechanism

The `fallible!` macro decrements the process-global `FAULT_INJECT_COUNTER` atomic at each wrapped call site and returns an annotated `io::Error` when it reaches zero. The counter defaults to `u64::MAX`.

Wrapped sites cover the read surfaces behind statement production: source reads for digest computation, chunk reads for coverage, and signature or key file reads. The pure cores keep composing statements and geometry from supplied values and are not touched.

## Determinism controls

- Each fixture stores an explicit counter value and restores the default in a guard.
- Fixtures sharing the process-global counter run serially.
- `SLEEPINESS` stays zero in fixtures and CI.
- The trigger function records the injection site for assertions.

## Placement

Dev-dependency of the shell test targets only. The dependency catalog records transport `crates.io`, plane `validation`.

## Verification

Positive coverage: default counter completes verification and geometry checks unchanged.

Negative coverage: injected failures at each wrapped site, each asserting the declared bounded failure and that no statement is produced from the unreadable input.

Boundary coverage: annotation carries the expected crate, file, and line; parallel fixtures never observe a foreign counter; no fixture enables `SLEEPINESS`.
