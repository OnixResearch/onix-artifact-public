# Artifact binding core design

<!-- r[impl artifact_binding.core.boundary] -->

`artifact-binding-core` is a pure `no_std + alloc` library. It accepts values that a consumer already loaded. It performs no I/O.

## Claim boundary

The core checks these structural facts:

- A binding revision is the exact checked successor of the current revision.
- Compare-and-swap facts match one current record and one immutable snapshot.
- Compatibility and reachability observations bind to exact targets, contracts, and that snapshot.
- Pinned resolution keeps an exact target.
- Late-bound resolution selects one active target from one immutable snapshot.
- Reachability terminates under named bounds, including cyclic graphs.
- Retirement is definitive only when root, edge, and attribution observations are complete.

The core does not own these facts:

- Artifact identity or byte integrity.
- Compatibility truth or reachability discovery.
- Storage, publication, locking, or atomic mutation.
- Root discovery, observation freshness, or attribution truth.
- Deployment, cleanup, retention, authorization, or deletion.

A successful plan is not proof that a consumer published it. A `Retired` result is not deletion authority.

## Functional core

The public API has four independent operations.

### Transition planning

`plan_transition` compares `ExpectedBinding` with one active `BindingRecord`. It then checks one proposed successor.

The successor revision must equal `current + 1`. Checked overflow, reversal, and skipped revisions fail with separate typed errors.

A rollback uses the same successor rule. Only the target points to an older immutable artifact. The revision never decreases.

When policy requires compatibility or reachability, the caller must supply exact observations. The core validates their target, contract, and snapshot bindings. It does not evaluate product compatibility or discover graph reachability.

### Resolution

`resolve` has explicit `Pinned` and `LateBound` modes.

Pinned mode keeps the supplied target, revision, and snapshot. Late-bound mode scans one bounded `BindingSnapshot`. Zero candidates, multiple active candidates, stale revisions, and mixed snapshots fail closed.

A consumer must retain the returned revision and snapshot for the full unit of work. It must not perform a second live lookup during that work.

### Reachability

`trace_reachability` normalizes roots and edges with ordered maps and sets. It uses a bounded breadth-first traversal.

Roots and neighbors use lexical identifier order. The selected pin path is therefore stable for equivalent input sets. Exact duplicate roots and edges produce typed diagnostics. Cycle-closing edges produce typed diagnostics and do not recurse.

The caller sets bounds for roots, edges, reachable nodes, path nodes, and diagnostics. The function returns no partial trace after a bound failure.

### Retirement

`classify_retirement` combines one reachability trace with caller-supplied completeness and attribution facts from the same snapshot.

A generation is `Live` when a generation-scoped root reaches work for it. It is also `Live` when a reachable artifact belongs only to that generation.

`GenerationAttribution` makes exclusive and shared ownership explicit. A shared value must contain at least two distinct generations.

An artifact shared by old and new generations does not keep the old generation live by itself. This rule prevents shared immutable code from blocking retirement forever.

Missing inventory produces `Incomplete`. Contradictory attribution produces `Unknown`. Only complete, coherent facts with no old-generation pin produce `Retired`.

## Stable output

The core sorts normalized identities, paths, and diagnostics. Input order does not change the result.

Diagnostics are finite. When issue counts exceed the caller's limit, one typed truncation issue reports the omitted count.

## Resource model

All identifiers use explicit byte bounds at construction. All graph and retirement operations use caller-selected count bounds.

The release profile sets one code-generation unit and size optimization for this package. Host, WebAssembly, and `thumbv7em-none-eabi` checks protect the small `no_std` surface.
