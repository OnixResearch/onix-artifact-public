# Artifact Binding Core Specification

## Purpose

Defines the `artifact-binding-core` capability.

## Requirements

### Requirement: The core is pure and product-neutral

r[artifact_binding.core.boundary] `artifact-binding-core` MUST be `no_std + alloc`, MUST evaluate already-loaded values without side effects, and MUST leave artifact hashing, canonical encoding, interface semantics, storage, atomic mutation, clocks, transport, authorization, policy, evidence, receipts, deployment, retention, garbage collection, and release decisions to consumers.

#### Scenario: Consumer supplies complete in-memory facts
- GIVEN a caller supplies validated identifiers, binding records, compatibility observations, roots, edges, and generation attribution in memory
- WHEN the core evaluates them
- THEN it MUST return deterministic decisions and diagnostics without filesystem, environment, process, clock, network, storage, or presentation effects.

#### Scenario: Core result is treated as product authority
- GIVEN a passing binding or retirement decision
- WHEN a consumer attempts to use that result as authorization, deployment, deletion, or release authority by itself
- THEN the integration MUST reject that claim because those decisions remain consumer-owned.

### Requirement: Binding revisions advance explicitly

r[artifact_binding.binding.transition] The core MUST validate compare-and-swap-style binding transitions against an expected current revision and target, MUST require a checked successor revision, and MUST represent rollback as a new successor revision that points to an older immutable target rather than decrementing history.

#### Scenario: Current binding advances to a new target
- GIVEN the expected current binding matches the supplied snapshot and the successor revision advances exactly once without overflow
- WHEN transition validation runs
- THEN the core MUST return a plan that preserves the prior binding and names the new exact target.

#### Scenario: Stale or overflowing transition is proposed
- GIVEN the expected current revision or target does not match, the successor skips or regresses, or successor arithmetic overflows
- WHEN transition validation runs
- THEN the core MUST deny with deterministic stale-transition, non-successor, or revision-overflow diagnostics.

### Requirement: Compatibility is an explicit observation

r[artifact_binding.binding.compatibility] The core MUST require caller-supplied compatibility observations when a binding contract requires them and MUST NOT infer interface, schema, ABI, semantic, policy, or state compatibility from names, matching bytes, or target-reference shape.

#### Scenario: Consumer supplies admitted compatibility
- GIVEN a transition policy requires a compatibility observation and the caller supplies one bound to the exact prior and successor targets and contract refs
- WHEN transition validation runs
- THEN the core MAY accept the observation as an input fact without claiming the observation is true.

#### Scenario: Same-looking target substitutes for compatibility
- GIVEN targets share a display name, reference prefix, schema label, or byte length but no exact compatibility observation is supplied
- WHEN the transition requires compatibility
- THEN the core MUST deny rather than infer compatibility.

### Requirement: Resolution modes are explicit

r[artifact_binding.binding.resolution] The core MUST distinguish exact pinned resolution from late-bound resolution by logical binding key, and every successful resolution MUST return one exact artifact target plus the binding revision and snapshot identity used.

#### Scenario: Pinned request names an exact target
- GIVEN a request carries a valid exact target ref
- WHEN pinned resolution runs
- THEN the result MUST retain that target without consulting mutable binding records.

#### Scenario: Late-bound request is ambiguous or absent
- GIVEN one immutable snapshot contains zero or multiple active candidates for a required binding key
- WHEN late-bound resolution runs
- THEN the core MUST deny with missing-target or ambiguous-target diagnostics.

### Requirement: Late-bound resolution uses one snapshot

r[artifact_binding.binding.snapshot_consistency] A late-bound resolution operation MUST evaluate exactly one immutable caller-supplied binding snapshot and MUST NOT perform a second lookup or silently combine revisions during the same resolution result.

#### Scenario: Binding changes after resolution
- GIVEN a unit of work resolves target A from snapshot S and a later snapshot points the same key to target B
- WHEN the existing unit continues
- THEN its resolution result MUST remain pinned to A and new resolution against the later snapshot MAY select B.

#### Scenario: Caller combines observations from different snapshots
- GIVEN candidates, expected revision, or target facts come from different snapshot identities
- WHEN resolution or transition validation runs
- THEN the core MUST deny with snapshot-mismatch diagnostics.

### Requirement: Reachability is deterministic and cycle-safe

r[artifact_binding.reachability.trace] The core MUST traverse caller-supplied roots and directed edges with bounded deterministic ordering, MUST terminate in the presence of cycles and duplicates, and MUST return reachable refs plus stable explanatory root-to-target paths when requested.

#### Scenario: Cyclic graph contains a pinned target
- GIVEN a complete graph contains a cycle and one root reaches an old-generation target through that cycle
- WHEN reachability tracing runs
- THEN traversal MUST terminate, classify the target reachable, and return a stable finite pin path.

#### Scenario: Equivalent input order changes
- GIVEN two inputs contain the same validated roots and edges in different orders with duplicate edges
- WHEN tracing runs
- THEN reachable sets, selected explanatory paths, duplicate diagnostics, and output ordering MUST be identical.

### Requirement: Retirement is conservative

r[artifact_binding.retirement.classification] The core MUST classify a generation as retired only when root inventory, edge inventory, and generation attribution are explicitly complete and no generation-scoped root or generation-exclusive artifact remains reachable. Incomplete inputs MUST produce `unknown` or `incomplete`, never `retired`.

#### Scenario: Complete inventory has no old-generation reachability
- GIVEN complete roots, edges, and generation attribution show no root reaches an old generation or any artifact exclusive to it
- WHEN retirement classification runs
- THEN the core MAY classify that generation retired and MUST return the facts supporting the classification.

#### Scenario: Root inventory is incomplete
- GIVEN runtime frames, sessions, tasks, cells, queues, timers, registries, rollback pins, remote pins, or another declared root class is unobserved
- WHEN retirement classification runs
- THEN the core MUST refuse a retired result and identify the incomplete root classes.

### Requirement: Shared artifacts do not create false generation pins

r[artifact_binding.retirement.attribution] Generation retirement input MUST distinguish generation-scoped roots and generation-exclusive artifact attribution from immutable artifact refs shared by multiple generations, and ambiguous attribution MUST fail conservatively.

#### Scenario: Unchanged artifact is shared by old and current snapshots
- GIVEN an immutable artifact ref is present in both generations and no old-generation-scoped root remains
- WHEN retirement classification runs with complete attribution
- THEN the shared ref alone MUST NOT keep the old generation live.

#### Scenario: Artifact attribution is missing
- GIVEN a reachable artifact may be exclusive to an old generation but attribution is absent or contradictory
- WHEN classification runs
- THEN the result MUST be incomplete rather than retired.

### Requirement: Diagnostics preserve bounded explanations

r[artifact_binding.diagnostics] Core diagnostics MUST use stable typed classes and bounded identifiers for stale transitions, snapshot mismatch, missing or ambiguous targets, missing compatibility observations, unreachable successor targets, cycles, duplicates, incomplete inventories, contradictory attribution, live pins, and arithmetic failure.

#### Scenario: Old generation remains pinned
- GIVEN a complete trace reaches an old-generation-exclusive artifact from a registered session root
- WHEN diagnostics are constructed
- THEN the result MUST identify the generation, root class and id, target ref, and stable pin path without claiming the session may be killed or the artifact deleted.

### Requirement: Validation includes positive and negative adoption evidence

r[artifact_binding.validation] The repository MUST include positive, negative, boundary, overflow, cyclic-graph, deterministic-order, incomplete-inventory, stale-transition, and property-style tests plus disposable Molten and OnixOS consumer pilots before permanent adoption is proposed.

#### Scenario: Only one consumer shape is proven
- GIVEN the core passes its own tests and one disposable consumer pilot
- WHEN permanent shared adoption is reviewed
- THEN the review MUST remain blocked until a materially distinct second pilot exercises the public boundary.

#### Scenario: Both pilots pass
- GIVEN Molten and OnixOS pilots map their product data into the same public core without transferring product authority
- WHEN adoption evidence is reviewed
- THEN the repository MAY publish an immutable candidate revision while each permanent consumer still requires its own Cairn change.
