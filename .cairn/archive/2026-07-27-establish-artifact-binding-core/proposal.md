## Why

Molten, OnixOS, Mantle, and other stack consumers already distinguish immutable artifact references from mutable names, generations, roots, and retention decisions, but the deterministic mechanics are product-local. Reimplementing successor binding checks, one-snapshot resolution, cyclic reachability, pin explanations, and conservative retirement in each consumer would create incompatible edge cases and duplicated safety logic.

A lower shared library is justified only if it remains narrower than every consumer. It must accept explicit facts, return deterministic plans and diagnostics, and leave canonical encoding, hashing, storage, atomic mutation, compatibility semantics, authority, evidence, deployment, retention, and deletion to product owners.

## What Changes

- Establish a standalone `artifact-binding-core` `no_std + alloc` functional core for versioned logical bindings over opaque immutable artifact references.
- Define explicit pinned resolution and late-bound resolution against one immutable caller-supplied snapshot; never introduce ambient or per-call name lookup.
- Define deterministic graph reachability over caller-supplied roots, edges, generation attribution, and completeness observations, including stable explanatory pin paths and cycles.
- Make definitive retirement impossible when root inventory, edge inventory, or generation attribution is incomplete.
- Define structured diagnostics for stale binding revisions, missing or ambiguous targets, incompatible caller observations, unreachable replacements, incomplete inventories, and still-pinned generations.
- Prove the extraction boundary with disposable Molten and OnixOS pilots before any permanent consumer dependency.

## Impact

- **Files**: workspace `README.md`, `AGENTS.md`, `Cargo.toml`, `crates/artifact-binding-core/**`, binding design/pilot/adoption docs, `cairn/**`, and shared Nix checks introduced by implementation.
- **Testing**: Cairn proposal/design/tasks gates now; implementation requires positive, negative, boundary, overflow, cyclic-graph, deterministic-order, incomplete-inventory, stale-transition, and property-style tests plus `no_std` target checks and two disposable consumer pilots.
