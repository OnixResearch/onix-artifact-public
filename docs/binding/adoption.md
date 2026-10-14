# Artifact binding adoption

<!-- r[verify artifact_binding.validation] -->

Use this sequence when a product adopts `artifact-binding-core`.

## 1. Pin the source

Pin one immutable Git revision in Cargo and Nix. Do not use a sibling path in durable product configuration.

Keep the package name `artifact-binding-core`. Do not import unrelated Artifact packages unless the adapter needs their separate mechanisms.

## 2. Keep product identifiers

Map product IDs into the typed core IDs. Set a byte limit for every conversion.

Do not replace a product's canonical artifact format with a new workspace-wide format. The core treats each ID as opaque text after validation.

## 3. Build one immutable snapshot

Load roots, edges, bindings, completeness facts, and attributions before you call the core. Give related facts the same `SnapshotId`.

Do not mix a current binding from one read with compatibility or attribution facts from another read. Mixed snapshots fail closed when the API carries both identities.

## 4. Separate checks from effects

Run `plan_transition` before storage mutation. If the plan passes, use the consumer's storage transaction to compare and swap the expected record.

The consumer must check the current record again inside its transaction. The pure plan does not lock storage and does not prove publication.

For rollback, publish a new successor revision that points to the retained older target. Never decrement the revision.

## 5. Keep resolution stable

Choose pinned or late-bound mode before a unit of work starts.

For late binding, retain the returned target, binding revision, and snapshot until the work ends. Do not perform another live lookup during that unit.

## 6. Declare every root class

List all product root classes that can retain a generation. Supply an explicit completeness observation for each class.

Examples include sessions, durable queues, timer callbacks, in-flight requests, operator pins, and deployment records. An omitted class makes retirement evidence unsafe.

## 7. Keep cleanup outside the core

Treat `Retired` as one input to product policy. Apply product retention, authorization, evidence, and deletion rules after classification.

The product must own failures, retries, storage consistency, cleanup receipts, and operator review.

## 8. Add positive and negative rails

At minimum, test these cases:

- Exact update and rollback successors.
- Stale, reversed, skipped, and overflowing revisions.
- Missing, rejected, and substituted compatibility observations.
- Missing, ambiguous, and mixed-snapshot resolution.
- Cycles, duplicate edges, and every named bound.
- Complete retirement, live scoped roots, shared artifacts, missing roots, and contradictory attribution.
- Host and supported `no_std` targets.

Use a repo-local Cairn change for the product integration. Record the exact Artifact revision and preserve product-specific non-claims.
