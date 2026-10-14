# Release evidence

A release candidate is admitted only when the checked repository/release Nickel exports are fresh, every implemented package passes host and Wasm no-std checks, positive and negative Rust tests pass, Clippy and formatting pass, Cairn lifecycle validation passes, authority audits find no forbidden shell dependencies, source provenance is complete, and the immutable Git revision is recorded.

Passing these checks proves bounded source, contract, build, and fixture facts only. It does not prove cryptographic library correctness, trust-root correctness, revocation freshness, consumer adapter correctness, artifact truth, transfer integrity, observation completeness, authorization, deletion authority, or product release eligibility.

Independent public Artifact source is available from `OnixResearch/onix-artifact-public`, rooted at audited parentless base `f379f2583b5d152cfd6d58919738652861793c90`. This is source-only publication and remains **NOT RELEASE-ADMITTED**: it does not establish that the checks above passed for any new publication revision. Original private Git ancestry is not published. Historical consumer pins to `artifact-auth` remain authoritative until each consumer completes its own reviewed migration; frozen Radicle receipts retain only their original historical scope.
