# Design: Radicle publication of artifact-auth compatibility source

## Source identity

Publication starts from a clean worktree whose local `main` points at `799459346d5416fbd7b9f55840a7371441b55afa`. The Git object ID is the primary interoperability identity. A deterministic `git archive --format=tar` observation records BLAKE3 `246a7cad91e7e8a158e22da21f3bff3e61aa0431a58936b5a739178bc62064c7`; downstream checks must preserve both identities.

## Governance

The repository identity uses the established delegates:

- author: `did:key:z6MksnXbFoE8zkCkGWhHc8zuxpnEUhrJHv2KECRV4GSv9gkx`;
- Bonsai: `did:key:z6MkjCqx5ksRqcDeNeuEnz53udbUHebRLHhddCxecWJu9koE`;
- Pine: `did:key:z6MkmkA6sEzzMffaWqKEKJcDh8LjAzgrJLrTNi971KN3X6sh`.

The identity threshold is two. Delegate signing may occur offline from copied public repository storage. Plaintext delegate profiles remain temporary under `/dev/shm`; only signed public Git bundles may leave them.

## Publication shell

Radicle initialization, delegate signing, seed synchronization, HTTPS probing, and receipt writing are imperative effects. Exact revision, delegate-set, threshold, source-hash, ref-convergence, and non-claim checks are deterministic comparisons over captured observations.

Aspen and the desktop replica may store and serve only explicitly admitted RIDs. HTTPS exposes upload-pack for the exact repository route and does not expose receive-pack or generic repository discovery.

## Failure behavior

Publication fails closed if the reviewed commit is absent, archive identity changes, delegates or threshold differ, either seed lacks the exact ref, HTTPS resolves a different object, an undeclared RID becomes admitted, or authority checks show seed/CI signing or canonical-ref mutation capability.

## Evidence

A typed Nickel source exports deterministic JSON plus a BLAKE3 sidecar. The receipt binds repository ID, reviewed commit, source archive, delegate set, threshold, seed observations, HTTPS route, focused tests, negative probes, and bounded non-claims.
