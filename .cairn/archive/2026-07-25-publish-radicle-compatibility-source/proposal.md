# Publish the artifact-auth compatibility source through Radicle

## Why

Valence, Mantle, and Molten consume the immutable `artifact-auth` revision `799459346d5416fbd7b9f55840a7371441b55afa` through GitHub SSH. The existing Bounded Exec pilot proved one Radicle-backed dependency, but the forge still lacks a second independently governed source and multi-repository seed admission evidence.

## Outcome

Publish the exact historical Git object without changing package behavior, establish the existing three-delegate/two-signature governance boundary, converge Aspen and the desktop replica, and expose the repository through the upload-pack-only HTTPS adapter. Emit a typed BLAKE3 receipt that downstream consumers can validate.

## Scope

- Preserve commit `799459346d5416fbd7b9f55840a7371441b55afa` and source-archive BLAKE3 `246a7cad91e7e8a158e22da21f3bff3e61aa0431a58936b5a739178bc62064c7`.
- Keep both crates, APIs, tests, license metadata, and authority boundaries unchanged.
- Reuse the established author, Bonsai, and Pine delegate identities with threshold two.
- Publish only public Git source; do not grant seed, CI, or HTTPS services signing, review, release, deployment, or canonical-ref authority.

## Non-goals

This change does not migrate consumers, replace the unified `artifact` workspace, admit private repositories, enforce CI at protocol level, or claim source correctness or release readiness.
