# Consumer migration

1. Pin one immutable `OnixResearch/onix-artifact-public` Git revision in Cargo and Nix. Audited base `f379f2583b5d152cfd6d58919738652861793c90` is available as `github:OnixResearch/onix-artifact-public/f379f2583b5d152cfd6d58919738652861793c90` with `flake = false`; Cargo uses `https://github.com/OnixResearch/onix-artifact-public.git` at that same revision. Existing historical `artifact-auth` pins remain rollback/compatibility identities, not aliases for the export.
2. Confirm Cargo and Nix resolve the same revision and no workspace-relative override is active.
3. Measure and canonicalize product payloads in the existing shell.
4. Produce standalone statements and observations through a pure adapter.
5. Dual-run legacy and standalone decisions over identical observations.
6. Classify every preimage, key-identity, decision, issue, and non-claim difference.
7. Keep the legacy path authoritative for unexplained differences.
8. Admit the standalone path only after positive and negative consumer fixtures pass.
9. Retain a bounded rollback release, then remove duplicate canonical ownership.

A standalone passing result remains one bounded input to product policy. It never authorizes publication, deployment, lifecycle completion, or release.

Regenerate Cargo/Nix locks and consumer-owned current source-selection/profile projections with their owning tools. The audited base retains historical Artifact repository metadata; current metadata descendants identify the public mirror and therefore have a different profile digest. Preserve historical evidence-bound hashes rather than replacing them with a current profile hash. Neither anonymous source access nor URL migration grants release admission: the independent mirror remains **NOT RELEASE-ADMITTED**.
