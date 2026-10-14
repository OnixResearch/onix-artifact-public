# Artifact Transfer Core

## Purpose

Preserve the implemented pure chunk-geometry and digest-tagging boundary while moving it into the shared Artifact workspace.

## ADDED Requirements

### Requirement: Geometry mechanism is explicit

r[artifact_transfer.geometry.explicit] The core MUST distinguish fixed-size chunks, FastCDC-bound output, and externally segmented ordered parts; MUST validate only supplied ordered lengths under consumer bounds and empty policy; and MUST NOT claim a concrete FastCDC implementation or external boundary algorithm.

#### Scenario: Valid explicit geometry covers the artifact

- GIVEN valid ordered lengths, declared total bytes, explicit mechanism, finite limits, and empty policy
- WHEN geometry validation runs
- THEN it MUST return contiguous checked ranges whose total equals the declaration.

#### Scenario: Mechanism shape is malformed

- GIVEN a zero fixed size, invalid FastCDC bounds, zero chunk, wrong fixed length, out-of-bound FastCDC length, absent non-empty chunks, excess count/bytes, overflow, or total mismatch
- WHEN geometry validation runs
- THEN it MUST fail deterministically without panic or partial success.

### Requirement: Digest identity retains its algorithm

r[artifact_transfer.digest.tagged] The core MUST keep BLAKE3 and SHA-256 identities explicitly distinct even when their digest bytes are equal and MUST reject malformed digest lengths.

#### Scenario: Equal bytes use different algorithms

- GIVEN equal supported-length bytes tagged once as BLAKE3 and once as SHA-256
- WHEN typed digests are compared
- THEN they MUST remain unequal because algorithm identity is part of the value.

#### Scenario: Digest length is malformed

- GIVEN a digest byte sequence with a non-supported length
- WHEN typed digest construction runs
- THEN it MUST return a stable invalid-length error.

### Requirement: Transfer claims remain bounded

r[artifact_transfer.claims.bounded] Geometry validation MUST NOT parse or define consumer wire schemas, chunk or hash bytes, identify FastCDC gear tables, reconstruct artifacts, perform transforms or transport, access storage, authorize operations, or prove integrity, completion, evidence truth, readiness, or release eligibility.

#### Scenario: Geometry passes

- GIVEN supplied lengths satisfy the selected mechanism and bounds
- WHEN a consumer receives the validated ranges
- THEN only ordered declared-length coverage MAY be claimed.

#### Scenario: Geometry is promoted to integrity

- GIVEN documentation or an adapter treats validated lengths as proof of bytes, hashes, reconstruction, transfer, or authorization
- WHEN claim validation runs
- THEN the result MUST be rejected before adoption.
