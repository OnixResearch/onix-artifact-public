## Why

Mantle, OnixOS, Nickel Export, and Valence independently stage regular files, synchronize payloads, rename them into place, and classify replacement or recovery outcomes. The implementations are similar enough to justify a bounded extraction experiment, but they do not currently expose one lossless mechanism or claim boundary:

- Mantle provides race-free Linux no-replace publication, payload and parent synchronization, private temporary files, and idempotent byte comparison for immutable attempt-log objects.
- OnixOS publishes through capability-backed directories with explicit replace/refuse policy and exact permission handling, but its non-Linux refuse fallback is not race-free and its current capability directory synchronization is intentionally a no-op.
- Nickel Export owns stronger generation-pointer and recoverable multi-file transaction semantics that cannot be described as one atomic file replacement.
- Valence has staged replacement and bundle-specific rollback behavior, including a two-output path that explicitly is not cross-file atomic.

Similar code is not enough to justify another repository. The stack needs disposable pilots proving that Mantle and OnixOS can consume one narrow single-file API without losing guarantees, transferring product authority, or widening claims.

## What Changes

- Record an extraction-feasibility contract for capability-scoped publication of one regular file beneath one caller-opened destination directory.
- Specify a `no_std + alloc` functional core that validates explicit publication requests and classifies supplied phase observations, plus a thin std shell that owns private staging, payload synchronization, atomic replace or race-free no-replace, parent synchronization, and cleanup attempts.
- Require explicit visibility-only versus durability-required operation and typed partial-commit outcomes; a successful rename followed by failed parent synchronization must never be reported as an ordinary failure or as durable success.
- Exclude directories, symlinks as payloads, multi-file transactions, generation pointers, transaction markers, crash recovery protocols, locks, authorization, receipts, rollback, retention, and product content identity from the first candidate boundary.
- Build disposable Mantle immutable-attempt-log and OnixOS capability-file pilots against exact recorded revisions without adding permanent product dependencies.
- Produce an extract-or-block decision from one shared conformance corpus, dependency-cycle review, source/license provenance, and an independent-release-value assessment.
- Keep this Artifact change as incubation evidence only. It adds no package or workspace member; a passing result requires a separate repository and later repo-local adoption Cairns.

## Success Criteria

- The same candidate request, state, outcome, and error vocabulary maps losslessly to Mantle and OnixOS without product-specific fields in the shared boundary.
- Positive and negative fixtures cover replace, no-replace, destination races, unsupported platform capability, permission/write/sync/rename failures, cleanup failure, and rename-success/parent-sync-failure.
- No-replace is implemented by one race-free primitive or rejected before publication; existence-check-then-rename is never accepted as equivalent.
- Durability-required publication reports success only after payload metadata and parent-directory synchronization complete, while visibility-only publication carries an explicit durability non-claim.
- Product authorization, path admission, content meaning, idempotent existing-content comparison, receipts, and lifecycle remain in consumer adapters.
- The final decision records exact consumer revisions, field mappings, mismatches, license provenance, dependency direction, rollback paths, and either independent repository promotion or an exact bounded blocker.

## Out of Scope

- Implementing or publishing the candidate repository in this change before both pilots pass.
- Changing Mantle or OnixOS production publication behavior during disposable pilots.
- General filesystem transactions, directory-tree publication, cross-file atomicity, journal recovery, generation-pointer semantics, or automatic stale-stage cleanup.
- Folding publication into `cap-root`, `artifact-binding-core`, Nickel Export, or another package with a different claim boundary.
- Claiming filesystem correctness, storage-device persistence, power-loss survival, payload truth, destination authorization, host compromise resistance, or release eligibility.

## Impact

- **Files:** this Cairn package, future feasibility notes under `docs/publication/`, and disposable pilot/conformance fixtures that remain outside Artifact workspace membership.
- **Consumers:** read-only source mapping and disposable adapters for exact Mantle and OnixOS revisions; permanent changes require separate consumer Cairns after immutable publication.
- **Dependencies:** no permanent dependency is introduced. A future repository must remain below both consumers and must not depend on stack product or evidence crates.
- **Licensing:** implementation source must not be copied or relicensed across repository license boundaries without explicit authorization and a recorded provenance map.
- **Validation:** Cairn proposal/design/tasks gates now; implementation requires one shared positive/negative fault corpus, both pilots, deterministic core tests, platform capability checks, and a documented extract-or-block decision.
