let Contracts = import "../contracts.ncl" in
{
  name = "artifact-auth",
  repository = "https://github.com/OnixResearch/artifact-auth",
  license = "AGPL-3.0-only",
  packages = ["artifact-auth-core", "artifact-auth-ed25519"],
  schemas = [],
  source_cohort = [],
  owner_authorization = {
    owner_authorized = true,
    record = "provenance/SOURCE_GRANT.md",
    license_grant = "AGPL-3.0-only",
  },
  release_checks = [],
  non_claims = [],
} | Contracts.Repository
