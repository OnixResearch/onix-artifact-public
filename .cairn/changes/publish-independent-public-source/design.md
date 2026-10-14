# Design: Independent public source

## Decisions

Use the audited parentless export as the immutable public base and add only scoped publication metadata descendants. Push explicit refs without force. Exact anonymous Git object access and Nix source fetches establish only source availability. Reject a source boundary that contains the original private snapshot commit or any private parent chain.

Update live repository discovery in the workspace manifest and `config/repository.ncl`; regenerate `config/repository.json` and `release/profile.json` with Nickel. Preserve the extraction `source_cohort`, grant anchors, package contract, and non-claims. The current profile digest necessarily changes with repository discovery; historical evidence-bound profiles and receipts do not become current by rewriting them.

Current projections are regenerated using reviewed `/nix/store/ck3a4k9az87z5rjwgy8nmpcs34xp9cc5-nickel-lang-rev-fixup-1.17.0/bin/nickel`, which reports `nickel-lang-cli nickel 1.17.0 (rev 1320a98)`. This records the actual evaluator used, not independent cohort acceptance or a fabricated historical receipt.

Cairn consumes the audited Artifact base as non-flake authentication source. Artifact consumes a finalized immutable public Cairn assembly for lifecycle checks. This finite source graph breaks the packaging cycle without aliases or private history. Generate `flake.lock` with Nix, retaining the existing root nixpkgs-follow edge.

## Rejected alternatives

Publishing private histories, changing private repository visibility, replacing historical identities with public exports, disabling profile checks, claiming old receipts as fresh, or modifying authentication bytes to fit metadata are not authorized.

## Verification and risks

Anonymous fetches disable Git global/system configuration, credentials, prompts, inherited auth tokens, and Nix access tokens. Check exact commit/tree and parentlessness, absence of original snapshot objects, current immutable source closure, and unchanged crate/grant/frozen receipt trees. The integration owner runs full Cargo/Nickel/Cairn/Nix gates once after coordinated source edits; no such gate is claimed here.

Public source and reviewed public `did:key` identifiers are not private keys. Publication retains license grants and historical evidence, including explicit historical-only bounds. Network observations do not prove implementation correctness, private confidentiality, consumer behavior, source equivalence beyond observed bytes, or release eligibility. Status remains **NOT RELEASE-ADMITTED**.
