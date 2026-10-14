# Separate repository handoff

<!-- r[verify durable_file_publication.extraction.promotion] -->

Create a separate repository only after review accepts the extraction decision in `DECISION.md`.

## Repository boundary

The repository must contain:

- A `no_std + alloc` phase core.
- A std capability-relative shell.
- Explicit platform implementations and unsupported results.
- The shared positive and negative corpus.
- Consumer-neutral API and non-claim documentation.
- Its own Cairn lifecycle, release profile, and immutable revision.

It must not depend on Artifact, Mantle, OnixOS, Valence, Cairn, or another stack product.

## Source and license

Start from Artifact pilot commit `67f14770a82e24eb39d3a751285b908232566e6b`.

Keep a source map for each imported file. Use `AGPL-3.0-or-later` if any AGPL implementation expression is imported. Use another license only after explicit compatibility and copyright review.

Do not copy product DTOs, diagnostic text, staging-name formats, receipts, or lifecycle policy.

## Required release rails

1. Run core tests with no default features on host and WebAssembly.
2. Run the shell and full fault corpus on Linux.
3. Test actual no-replace destination races.
4. Test parent-sync failure after a committed rename.
5. Test cleanup failure after each pre-commit failure class.
6. Reject unsupported platforms before mutation.
7. Run strict formatting, Clippy, dependency, license, and Cairn checks.
8. Publish one immutable candidate revision.

## Consumer adoption

Mantle and OnixOS each require a later repo-local Cairn change.

Each adoption must pin the same immutable Cargo and Nix revision, map every request and outcome field, retain the existing adapter for rollback, and replay the shared corpus.

Mantle must keep immutable-content comparison, manifests, receipts, and retention outside the shared repository.

OnixOS must keep root opening, path admission, root purpose, permission selection, and diagnostics outside the shared repository. It must provide a real synchronizable parent capability for durability-required operations.

## Stop conditions

Stop adoption if any product field enters the shared API, any outcome loses commit state, no-replace falls back to check-then-rename, parent sync becomes a no-op, or a license grant is missing.
