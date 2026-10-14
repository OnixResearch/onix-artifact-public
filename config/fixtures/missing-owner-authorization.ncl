let Contracts = import "../contracts.ncl" in
{
  name = "artifact-auth",
  repository = "https://github.com/OnixResearch/artifact-auth",
  license = "MIT OR Apache-2.0",
  packages = ["artifact-auth-core", "artifact-auth-ed25519"],
  schemas = [],
  source_cohort = [],
  release_checks = [],
  non_claims = [],
} | Contracts.Repository
