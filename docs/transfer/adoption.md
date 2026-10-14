# Consumer adoption status

## Cutover gate

The configured repository `https://github.com/OnixResearch/artifact-transfer` is not yet a published immutable source. Consumers must not add ambient sibling path dependencies or claim an unpublished local commit as external dependency evidence.

Disposable Molten and ChaosControl adapters passed focused positive/negative tests; see [`PILOT.md`](PILOT.md). They were validation evidence only and were removed without modifying authoritative consumers.

After explicit publication authorization:

1. publish one reviewed immutable commit without changing the AGPL license;
2. pin `artifact-transfer-core` by exact Git revision and align Cargo/Nix source identities;
3. authoritatively migrate Molten's `validate_fixed_chunk_lengths` through `ChunkingAlgorithm::Fixed`, retaining Preserves schemas, BLAKE3 domain separation, transforms, root refs, content refs, storage, Iroh, retention, and receipts in Molten;
4. authoritatively migrate ChaosControl's ordered snapshot-part size coverage through `ChunkingAlgorithm::ExternalOrderedParts`, retaining JSON schema, path policy, SHA-256 hashing, materialization, atomic install, replay evidence, and readiness claims in ChaosControl;
5. evaluate Mantle adoption separately: retain `crunch-delta-core` protocol negotiation, FastCDC profile/version policy, BLAKE3 identities, closure planning, and transfer tallies while using the shared geometry only where real manifests expose ordered lengths;
6. rerun standalone tests and focused positive/negative consumer suites against the exact published revision;
7. remove duplicate geometry helpers only after product diagnostics and behavior match.

Cutover success means consumers share checked geometry mechanics without sharing a wire format or promoting geometry into transfer/integrity evidence. Publication and authoritative consumer commits remain separately authorized actions.
