# Radicle publication

## ADDED Requirements

### Requirement: Exact compatibility-source identity r[artifact_auth.radicle_publication.identity]

The publication MUST preserve Git commit `799459346d5416fbd7b9f55840a7371441b55afa` and its measured source archive without changing package behavior or authority boundaries.

#### Scenario: Reviewed source is preserved

- GIVEN the historical compatibility commit and its expected BLAKE3 archive identity
- WHEN the Radicle publication source is materialized
- THEN the commit MUST resolve exactly and the archive identity MUST match.

#### Scenario: Source drift is rejected

- GIVEN a different commit, changed archive, missing crate, or changed authority boundary
- WHEN publication admission runs
- THEN publication MUST fail without producing an accepted receipt.

### Requirement: Threshold delegate governance r[artifact_auth.radicle_publication.governance]

The Radicle project MUST name exactly the established author, Bonsai, and Pine delegates and MUST require two delegate signatures.

#### Scenario: Two delegates satisfy governance

- GIVEN exact signed refs from two configured delegates over the reviewed main object
- WHEN canonical convergence is evaluated
- THEN the repository MAY be accepted without requiring the uninvolved third delegate to rewrite its namespace.

#### Scenario: One or unknown delegate is insufficient

- GIVEN fewer than two configured signatures or any substituted delegate
- WHEN governance is evaluated
- THEN acceptance MUST fail.

### Requirement: Selective two-seed convergence r[artifact_auth.radicle_publication.convergence]

Aspen and the independently operated desktop replica MUST store and serve the exact repository while retaining no delegate, signing, release, deployment, backup, or canonical-ref authority.

#### Scenario: Both seeds expose the reviewed object

- GIVEN the governed repository has been explicitly admitted
- WHEN each seed is queried independently
- THEN each seed MUST expose the reviewed commit and configured delegate refs.

#### Scenario: Seed authority remains unavailable

- GIVEN seed service identities and storage
- WHEN signing, delegate, release, deployment, or canonical-ref mutation is attempted
- THEN the operation MUST remain unavailable or denied.

### Requirement: Bounded public Git transport r[artifact_auth.radicle_publication.transport]

The public adapter MUST expose upload-pack for only the exact admitted repository route and MUST NOT expose receive-pack or generic repository discovery.

#### Scenario: Exact HTTPS fetch succeeds

- GIVEN the admitted public repository route
- WHEN a fresh client fetches the reviewed commit
- THEN the object MUST resolve exactly without GitHub fallback.

#### Scenario: Undeclared route fails closed

- GIVEN an unadmitted RID or write-service request
- WHEN the public adapter is queried
- THEN the request MUST be rejected.

### Requirement: Typed bounded publication evidence r[artifact_auth.radicle_publication.evidence]

The repository MUST emit a typed Nickel/JSON publication receipt with a BLAKE3 sidecar binding source, governance, convergence, transport, tests, negative probes, and non-claims.

#### Scenario: Complete evidence passes

- GIVEN all required identities and observations agree
- WHEN the receipt validator runs
- THEN it MUST accept the receipt deterministically.

#### Scenario: Missing or widened claims fail

- GIVEN mismatched identity, missing observation, or weakened non-claim data
- WHEN validation runs
- THEN it MUST fail closed.
