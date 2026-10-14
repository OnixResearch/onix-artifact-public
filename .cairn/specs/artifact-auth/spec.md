# Artifact Authentication

## Purpose

Provide product-neutral canonical artifact statements, pure Ed25519 observations, explicit key-currentness and threshold policy decisions, and consumer conformance mappings without product authority.

## Requirements

### Requirement: Functional core is authority free

r[artifact_auth.core.authority_free] Both reusable packages MUST compile with `#![no_std]` plus `alloc` and MUST NOT read files, inspect environment state, spawn processes, print, access clocks, generate or store keys, contact networks or trust services, or perform persistence.

#### Scenario: Wasm no-std build passes

- GIVEN serialization and default features are disabled
- WHEN each package is checked for `wasm32-unknown-unknown`
- THEN both packages MUST compile without product or ambient-authority dependencies.

#### Scenario: Forbidden authority is introduced

- GIVEN a package source imports filesystem, environment, process, clock, network, randomness, signing-key, or persistence authority
- WHEN the authority audit runs
- THEN release validation MUST fail.

### Requirement: Canonical statement is injective and domain separated

r[artifact_auth.statement.canonical] The core MUST encode schema, domain, purpose, profile, producer, key label, full-key identity, subject, sorted unique parents, and verifier context through a versioned length-delimited preimage and MUST use BLAKE3 for standalone identities while retaining explicit interoperability algorithm labels.

#### Scenario: Parent order changes

- GIVEN two valid statements with identical typed values and different parent input order
- WHEN canonical bytes and identity are computed
- THEN both results MUST be identical.

#### Scenario: Bound field changes

- GIVEN a valid statement
- WHEN domain, purpose, profile, producer, key, subject, parent, or verifier context changes
- THEN canonical bytes and statement identity MUST change or malformed input MUST fail closed.

### Requirement: Ed25519 verification is pure and explicit

r[artifact_auth.ed25519.verify] The Ed25519 package MUST verify signatures only over core-produced canonical bytes using explicitly supplied public-key and signature bytes and MUST return deterministic success or stable failure observations without exposing signing or private-key APIs.

#### Scenario: Matching signature verifies

- GIVEN a valid canonical statement and matching Ed25519 key and signature bytes
- WHEN verification runs
- THEN the observation MUST bind the BLAKE3 identity of the full raw public-key bytes and report success.

#### Scenario: Malformed or mismatched input fails

- GIVEN malformed key or signature lengths, malformed key bytes, a changed statement, a wrong key, or a wrong signature
- WHEN verification runs
- THEN the observation MUST report one stable failure class without panic or secret-bearing diagnostics.

### Requirement: Policy consumes explicit currentness and counts full keys

r[artifact_auth.policy.explicit] The core MUST evaluate only explicitly supplied trusted-key, purpose, generation, currentness, and cryptographic observations; thresholds MUST count distinct verified full-key identities, and signer labels MUST NOT inflate or replace that count.

#### Scenario: Distinct admitted keys satisfy threshold

- GIVEN enough distinct verified full-key identities with matching producer/key bindings, purposes, generations, and current or verification-overlap observations
- WHEN policy evaluation runs
- THEN the decision MAY pass for the exact scope.

#### Scenario: Duplicate stale or revoked key fails

- GIVEN duplicate full-key evidence, stale generation, superseded/revoked/unknown currentness, wrong purpose, untrusted labels, failed cryptography, or insufficient distinct keys
- WHEN policy evaluation runs
- THEN the decision MUST fail closed with sorted stable issues.

### Requirement: Consumer mappings preserve product authority

r[artifact_auth.conformance.consumers] Cairn, Mantle, Molten, and Valence mapping profiles MUST preserve exact source identity, algorithm labels, compatibility identities, extensions, retained authority, positive cases, and negative mapping-loss cases without product or sibling-path runtime dependencies.

#### Scenario: Reviewed mappings are lossless

- GIVEN the fixed source cohort and representative valid observations
- WHEN conformance evaluation runs
- THEN shared fields and bounded non-claims MUST map without changing identity, currentness, threshold, or claim meaning.

#### Scenario: Mapping widens a claim

- GIVEN a mapping drops an algorithm, substitutes a signer label for full-key identity, promotes requested or stale currentness, or treats authentication as authorization
- WHEN conformance evaluation runs
- THEN the mapping MUST fail before consumer cutover.

### Requirement: Authentication claims remain bounded

r[artifact_auth.claims.bounded] Decisions and documentation MUST deny claims of artifact truth, verifier correctness, producer authorization, trust-root correctness, revocation freshness, provenance correctness, lifecycle conformance, build correctness, semantic equivalence, deployment safety, and release eligibility.

#### Scenario: Passing decision retains non-claims

- GIVEN a decision satisfies cryptographic and policy checks
- WHEN the decision is projected to a consumer
- THEN every required non-claim MUST remain present.

#### Scenario: Overclaim is introduced

- GIVEN a profile or adapter describes signature validity as proof of truth, authority, correctness, readiness, or universal trust
- WHEN release validation runs
- THEN the artifact MUST be rejected.
