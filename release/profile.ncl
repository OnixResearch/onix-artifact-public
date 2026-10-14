let Repository = import "../config/repository.ncl" in
{
  schema = "artifact.release-profile.v1",
  project = Repository.name,
  repository = Repository.repository,
  license = Repository.license,
  packages = Repository.packages,
  package_dependencies = Repository.package_dependencies,
  required_checks = Repository.release_checks,
  source_cohort = Repository.source_cohort,
  owner_authorization = Repository.owner_authorization,
  distribution = {
    channel = "immutable-git-revision",
    crates_io = false,
    sibling_paths = false,
    floating_revisions = false,
  },
  claim_boundary = Repository.non_claims,
}
