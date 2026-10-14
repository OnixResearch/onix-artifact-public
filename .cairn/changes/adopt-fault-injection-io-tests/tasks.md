## Fixture adoption

- [ ] [serial] Add `fault-injection` as a pinned dev-dependency and record transport `crates.io`, plane `validation` in the dependency catalog. r[artifact.io_fault.fixtures]
- [ ] [serial] Wrap source reads, chunk reads, and signature or key file reads with `fallible!` in test builds. r[artifact.io_fault.fixtures]
- [ ] [serial] Add a trigger function that records the injection site for fixture assertions. r[artifact.io_fault.fixtures]

## Determinism

- [ ] [serial] Add a setup guard storing an explicit counter value and restoring `u64::MAX` after each fixture. r[artifact.io_fault.determinism]
- [ ] [serial] Keep `SLEEPINESS` at zero in every fixture and in CI configuration. r[artifact.io_fault.determinism]

## Boundary

- [ ] [serial] Document that the dependency is test-only and that injection coverage grants no trust, transfer-completion, or observation-truth claim. r[artifact.io_fault.boundary]

## Verification

- [ ] [parallel] Add a positive fixture proving unchanged verification and geometry with the default counter. r[artifact.io_fault.verification]
- [ ] [parallel] Add negative fixtures firing at each wrapped site, asserting the declared bounded failure and absent statement production. r[artifact.io_fault.verification]
- [ ] [parallel] Add boundary fixtures asserting the annotation and serialized counter ownership. r[artifact.io_fault.verification]
- [ ] [serial] Run package, Wasm-target, workspace, Clippy, Cairn, and Nix checks; append the komora-io reference under `## References` in `README.md`; document non-claims. r[artifact.io_fault.verification]
