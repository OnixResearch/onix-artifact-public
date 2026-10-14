## Why

Anonymous consumers need an independent public Artifact source without publishing private Git ancestry. The approved source export is parentless commit `f379f2583b5d152cfd6d58919738652861793c90`, derived from historical snapshot `2a34ddd92a4eaf57d4d0de792dfa9aa4ea105121`.

## What Changes

Publish only `OnixResearch/onix-artifact-public`, change current Cargo/Nickel repository discovery to that mirror, regenerate current repository/release JSON with Nickel, and select a fully public immutable Cairn lifecycle input with Nix-generated locks. Authentication, transfer, binding, license grants, extraction cohorts, and frozen Radicle evidence remain unchanged.

## Impact

The mirror supplies exact source for downstream `flake = false` consumption. Public reachability is a source-only observation, not receiver-local migration, semantic equivalence, or release admission. Historical profiles retain their old source identities; downstream current source selection must align Cargo/Nix and regenerate its own dependent projections.

## Out of Scope

No original repository visibility or history changes, force pushes, API changes, key removal, frozen receipt regeneration, inherited proof claims, package release, or release admission. This publication is **NOT RELEASE-ADMITTED**. Full repository and lifecycle checks are performed separately by the integration owner; this package does not claim they already passed.
