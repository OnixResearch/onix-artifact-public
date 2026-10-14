let Contracts = import "../contracts.ncl" in
{
  name = "artifact-auth",
  repository = "https://github.com/OnixResearch/artifact-auth",
  license = "MIT OR Apache-2.0",
  packages = ["artifact-auth-core", "artifact-auth-ed25519"],
  schemas = [],
  source_cohort = [
    { consumer = "cairn", revision = "cc4639be24475d80844b091cdec605eb2fef9ff0", role = "typed-observation-boundary" },
    { consumer = "mantle", revision = "wrong-revision", role = "full-key-threshold-and-canonicalization" },
    { consumer = "molten", revision = "70590459a218dc8e66948ab6f305a7c54142b710", role = "purpose-generation-currentness" },
    { consumer = "valence", revision = "8eccac35fcd8aae8ef64b3d1021e7f1a53656180", role = "protocol-and-authorization-boundary" },
  ],
  owner_authorization = {
    owner_authorized = true,
    record = "provenance/SOURCE_GRANT.md",
    license_grant = "MIT OR Apache-2.0",
  },
  release_checks = [],
  non_claims = [],
} | Contracts.Repository
