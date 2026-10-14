# Durable publication validation evidence

<!-- r[verify durable_file_publication.validation.evidence] -->

Date: 2026-07-27

## Candidate identity

The reviewed disposable candidate is Artifact commit `fbf9b81d02c8889ca61ed7e4e916aedd312c86b6`.

The shared 17-case corpus source BLAKE3 is `d02379677341b7408b636906c4bee4910ef3835cc81261373194ef84678b27da`.

## Exact source revisions

- Mantle: `688781f370e8f477d900e7b96efa1ad3266a392e`
- OnixOS: `628f0b8a7036ed344aa51c74612495e28299c75a`
- Nickel Export: `257fafc1c746f1faf156207043a4c826bfb16d49`
- Valence: `6ab37aa34c9f81da812d6b42f2f06b7ac2d7e214`
- Cap Root: `8991e2572578c16e590c2f1c0915db6c46c5d9cb`
- Artifact binding core: `25cd1235e25619290f744cffc9654d2414b2d08d`

## Candidate checks

Task `812` passed these checks:

- Candidate formatting.
- Six positive and negative tests.
- Strict Clippy for all targets.
- `no_std` core check on `wasm32-unknown-unknown`.
- Focused Nix check `checks.x86_64-linux.publication-incubation`.

The test suite runs the shared corpus through the pure core, fault-injected shell, Mantle mapping, and OnixOS mapping. It also runs a real Linux directory-capability integration case.

Task `825` passed the root Artifact workspace tests, strict Clippy, and formatting check.

## Recorded platform capability

The executed host was `x86_64-linux`.

The real integration case used `openat` with create-new and no-follow flags, `renameat_with(NOREPLACE)`, file synchronization, parent-directory `fsync`, exact mode checks, and destination preservation.

Android uses the same compiled primitive branch but was not executed. It remains unavailable evidence.

Other targets have no candidate std shell. Exact no-replace and parent synchronization are unsupported there until a reviewed platform implementation exists.

## Cairn and repository checks

Proposal, tasks, and design gates passed in tasks `821`, `823`, and `824`. The normalized delta design gate and dry-run synchronization passed in task `805`.

Accepted specification synchronization receipt: `6d270c35742aaa5851fe6b7469977122bec72f13ea00000bb42be256b6138ac1`.

Archive execution receipt: `8ea43912a3ae207d016083770359a1c8e8ae5dde18ae80c0b4053b68f2cf156e`.

The focused traceability profile covers 16 of 16 requirements. Its accepted-spec receipt is `019198221e93800921b704d2c566b6825e24403ddef38e3834628a39879525b8`.

The default traceability profile covers 40 of 40 requirements. Its receipt is `d1f5e4a6ace6689413ec661d3651101f186381f9d419385d1837d2ac5f4c6b9c`.

Repository Cairn validation passed after synchronization in task `810`.

The final task `812` passed the completed tasks gate, Cairn validation, and `nix flake check -L`. Nix reported `all checks passed` for `x86_64-linux` and omitted incompatible `aarch64-linux` outputs on this host.

## Consumer pilot limits

The pilots compile consumer-shaped adapters at the exact revisions. They do not add or build a production dependency in Mantle or OnixOS.

The OnixOS pilot records two exact gaps:

- Current parent-directory synchronization is a no-op and maps only to visibility completion.
- Current non-Linux refuse behavior uses inspection followed by rename and cannot map to exact no-replace.

The Mantle pilot retains identical-existing-content comparison in its adapter and keeps product manifests, identities, receipts, and retention outside the candidate.

## Corrected failures

The initial corpus test compile rejected equality checks that included `std::io::Error`, which has no `PartialEq`. The test now checks successful values without comparing error objects.

The first direct `b3sum` attempt was unavailable in the dev shell. The candidate now computes the corpus BLAKE3 through a Rust example with the pinned Cargo dependency.

The first full flake run in task `826` occurred before accepted-spec synchronization. Default traceability correctly rejected publication markers without accepted requirements. Synchronization and default-profile evidence links corrected that lifecycle ordering failure.

## Non-claims

These checks do not prove filesystem correctness, storage-device persistence, power-loss survival, host integrity, payload truth, destination authorization, consumer integration, deployment success, or release eligibility.
