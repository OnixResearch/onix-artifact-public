let OwnedPackageCount = 3 in
let OwnedPackages = std.contract.from_predicate (fun value =>
  std.is_array value
  && std.array.length value == OwnedPackageCount
  && std.array.elem "artifact-auth-core" value
  && std.array.elem "artifact-auth-ed25519" value
  && std.array.elem "artifact-transfer-core" value
) in
let StandaloneLicense = std.contract.from_predicate (fun value =>
  value == "MIT OR Apache-2.0"
) in
let OwnerAuthorized = std.contract.from_predicate (fun value => value == true) in
let SourceContract = {
  consumer | String,
  revision | String,
  role | String,
} in
let ExpectedSourceCohort = [
  { consumer = "cairn", revision = "cc4639be24475d80844b091cdec605eb2fef9ff0", role = "typed-observation-boundary" },
  { consumer = "mantle", revision = "26c6880c08cd730c9e9420f92ea91a87212e492d", role = "full-key-threshold-and-canonicalization" },
  { consumer = "molten", revision = "70590459a218dc8e66948ab6f305a7c54142b710", role = "purpose-generation-currentness" },
  { consumer = "valence", revision = "8eccac35fcd8aae8ef64b3d1021e7f1a53656180", role = "protocol-and-authorization-boundary" },
] in
let SourceCohort = std.contract.from_predicate (fun value =>
  value == ExpectedSourceCohort
) in
let ExpectedPackageDependencies = [
  { package = "artifact-auth-core", dependencies = [] },
  { package = "artifact-auth-ed25519", dependencies = ["artifact-auth-core"] },
  { package = "artifact-transfer-core", dependencies = [] },
] in
let PackageDependencies = std.contract.from_predicate (fun value =>
  value == ExpectedPackageDependencies
) in
let ExpectedNonClaims = [
  "Authentication does not prove artifact truth, verifier correctness, or producer authorization.",
  "Authentication does not prove trust-root correctness or revocation freshness.",
  "Authentication does not prove provenance, lifecycle, build, semantic, deployment, or release correctness.",
  "Transfer geometry does not prove chunk bytes, digest correctness, reconstruction, transport completion, or integrity.",
  "Binding and reachability decisions do not prove observation completeness, artifact correctness, deployment safety, retirement cleanup, or deletion authority.",
  "Workspace membership does not transfer authority or make distinct artifact mechanisms semantically interchangeable.",
] in
let NonClaims = std.contract.from_predicate (fun value =>
  value == ExpectedNonClaims
) in
let OwnerAuthorizationContract = {
  owner_authorized | OwnerAuthorized,
  record | String,
  license_grant | StandaloneLicense,
} in
let RepositoryContract = {
  name | String,
  repository | String,
  license | StandaloneLicense,
  packages | OwnedPackages,
  package_dependencies | PackageDependencies,
  mechanisms | Array String,
  schemas | Array String,
  source_cohort | SourceCohort,
  owner_authorization | OwnerAuthorizationContract,
  release_checks | Array String,
  non_claims | NonClaims,
} in
let Equal = fun expected =>
  std.contract.from_predicate (fun value => value == expected)
in
let NonEmptyStrings = std.contract.from_predicate (fun value =>
  std.is_array value
  && std.array.length value > 0
  && std.array.all std.is_string value
) in
let ConsumerMappingContract = fun expected_consumer expected_revision => {
  schema | Equal "artifact-auth.consumer-mapping.v1",
  consumer | Equal expected_consumer,
  source_revision | Equal expected_revision,
  shared | NonEmptyStrings,
  extensions | Array String,
  retained_authority | NonEmptyStrings,
} in
{
  ConsumerMapping = ConsumerMappingContract,
  OwnerAuthorization = OwnerAuthorizationContract,
  Repository = RepositoryContract,
  Source = SourceContract,
}
