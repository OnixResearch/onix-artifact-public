# Release evidence

A release candidate is admitted only when the checked repository/release Nickel exports are fresh, every implemented package passes host and Wasm no-std checks, positive and negative Rust tests pass, Clippy and formatting pass, Cairn lifecycle validation passes, authority audits find no forbidden shell dependencies, source provenance is complete, and the immutable Git revision is recorded.

Passing these checks proves bounded source, contract, build, and fixture facts only. It does not prove cryptographic library correctness, trust-root correctness, revocation freshness, consumer adapter correctness, artifact truth, transfer integrity, observation completeness, authorization, deletion authority, or product release eligibility.

No immutable `artifact` workspace revision has been published yet. Historical consumer pins to `artifact-auth` remain authoritative until each consumer completes its own reviewed migration.
