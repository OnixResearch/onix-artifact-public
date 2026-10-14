## Context

The stack already has several related but non-identical mechanisms:

- Molten has immutable artifact refs, name views, generations, system-extension fencing, retention pins, reference indexes, and upgrade sessions.
- OnixOS has active and rollback descriptor generations with pointer updates and target lifecycle evidence.
- Mantle has immutable store objects, roots, closures, and garbage collection.

Those products should not share policy, receipts, storage formats, or deployment semantics. They can share the smaller deterministic question: given explicit immutable refs, binding revisions, snapshots, roots, edges, attribution, and completeness observations, is a transition structurally valid, what exact ref resolves, what is reachable, and why is a generation still pinned?

Ambient demonstrates a coherent version of this mechanism through names that rebind to immutable hashes and runtime-owned retirement roots. This design adapts only that narrow mechanism; it does not adopt Ambient's language, VM, effect system, mobile-code path, default trust model, or claim scope.

## Decisions

### Decision: Create an independent narrow package

**Choice:** Establish `artifact-binding-core` as a `no_std + alloc` pure library package inside the shared Artifact workspace.

**Rationale:** Binding transitions and reachability are useful below Molten and OnixOS, but they do not belong to Kamacite's effect schemas, Valence's Evidence IR, artifact authentication, execution-graph's acyclic work DAGs, or artifact transfer geometry. A separate package and lifecycle capability preserve that claim boundary while sharing only workspace toolchain, license, lint, and release infrastructure.

**Rejected alternatives:**

- Keep all logic in Molten: this would duplicate transition and trace edge cases in OnixOS and make later extraction harder.
- Add it to execution-graph: retirement graphs may contain cycles and do not model execution readiness or task lifecycle.
- Add it to artifact-auth or Valence: reachability and rebinding prove neither authenticity nor evidence truth.

### Decision: Keep artifact refs opaque and consumer-owned

**Choice:** The public core accepts validated opaque artifact, contract, snapshot, binding, generation, root, and edge identifiers. It does not hash artifact bytes or define a universal serialized record.

**Rationale:** Molten uses canonical Preserves/BLAKE3 refs, OnixOS binds descriptors and activation evidence, and interoperability surfaces may carry other tagged refs. Consumers must remain authoritative for parsing, domain separation, canonical bytes, and typed schemas.

### Decision: Model publication as a pure compare-and-swap plan

**Choice:** Binding transition validation consumes the expected current binding and a proposed checked successor revision. It returns a plan or deterministic issues. Consumers perform load, compatibility admission, storage, atomic compare-and-swap, and receipts.

**Rationale:** Atomicity is an imperative store property. A pure crate can validate the preconditions and expected transition without falsely claiming a mutation occurred.

Rollback is a new successor revision that points to a prior immutable target. History never moves backward.

### Decision: Make resolution mode explicit

**Choice:** Public inputs distinguish pinned exact refs from late-bound binding keys. Late-bound resolution consumes one immutable snapshot and returns one exact target, revision, and snapshot identity.

**Rationale:** This prevents hidden ambient lookup and torn multi-read resolution. Consumers decide the unit of work—request, turn, callback pass, job, protocol session, or another explicit boundary—and pin the result below it.

### Decision: Require explicit compatibility observations

**Choice:** The core may require a caller-supplied compatibility observation bound to exact old/new targets and contract refs, but it never constructs or validates product semantics behind that observation.

**Rationale:** Interface, state, schema, ABI, effect, and policy compatibility belong to consumers. Structural similarity is not compatibility.

### Decision: Trace caller-supplied graphs deterministically

**Choice:** Reachability accepts finite root and edge inventories, uses deterministic ordered traversal with visited sets, tolerates duplicate and cyclic edges, and selects stable bounded explanatory paths.

**Rationale:** Runtime and storage ownership graphs naturally contain cycles. Stable path selection is necessary for repeatable diagnostics and evidence projections, while traversal limits remain explicit consumer/core inputs with named bounds.

### Decision: Retirement requires complete observations

**Choice:** `retired` is available only when root inventory, edge inventory, and generation attribution are explicitly complete. Any missing root class, stale snapshot, ambiguous ownership, contradictory attribution, or uninstrumented execution produces `incomplete` or `unknown`.

**Rationale:** False retirement can lead consumers toward unsafe cleanup. Conservative refusal is preferable to inferred absence.

Generation-scoped roots and generation-exclusive artifacts are distinct from immutable refs shared by multiple generations. A shared unchanged artifact does not by itself pin an older binding snapshot.

### Decision: Prove extraction with two disposable consumers

**Choice:** Before permanent adoption, build disposable mapping pilots for Molten system-extension/service generations and OnixOS active/rollback descriptor generations. Publish `PILOT.md` and `ADOPTION.md` with exact source revisions and mismatches.

**Rationale:** A shared crate is not justified by one consumer or by prose similarity. The second pilot must exercise the same public core through materially different product data.

## Risks / Trade-offs

- **False completeness:** A consumer may incorrectly mark roots complete. Mitigate by making completeness explicit per declared root class and documenting that the core does not prove observations true.
- **Over-general API:** Product-specific fields could leak into the core. Mitigate with opaque ids, caller observations, narrow enums, and two-consumer review.
- **Unsafe retirement promotion:** Consumers may treat retirement as deletion authority. Mitigate with API/docs non-claims and negative adoption tests.
- **Path explosion:** Reachability graphs can be large or cyclic. Mitigate with finite explicit bounds, visited sets, checked arithmetic, deterministic truncation/error classes, and no hidden defaults.
- **Attribution ambiguity:** Shared artifacts can create false old-generation pins or false retirement. Mitigate with explicit shared/exclusive attribution and conservative incomplete results.
- **Premature permanent dependency:** Mitigate by requiring immutable publication and repo-local Cairns after both disposable pilots pass.
