# Positive and negative tests for the frozen Radicle publication receipt.
let publication = import "../lib/radicle-publication-receipt.ncl" in
let receipt = import "../evidence/radicle/publication-v1.ncl" in
let digest_a = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" in
let digest_b = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb" in
let missing_commit = "1111111111111111111111111111111111111111" in
let result = fun candidate => publication.validate candidate in
let accepted = fun candidate => result candidate |> match { 'Ok => true, 'Error _ => false } in
let rejected_as = fun code candidate =>
  result candidate |> match { 'Ok => false, 'Error error => error.code == code }
in
{
  tests =
    [
      accepted receipt,
      publication.is_blake3 digest_a,
      publication.is_git_oid receipt.repository.reviewed_commit,
      publication.is_rid receipt.repository.rid,
      rejected_as "publication-shape" {},
      rejected_as "publication-header" (receipt & { status | force = "pending" }),
      rejected_as "publication-repository" (receipt & { repository.rid | force = "rad:z3gqcJUoA1n9HaHKufZs5FCSGazv5" }),
      rejected_as "publication-repository" (receipt & { repository.reviewed_commit | force = missing_commit }),
      rejected_as "publication-repository" (receipt & { repository.source_archive_blake3 | force = digest_b }),
      rejected_as "publication-repository" (receipt & { repository.history_rewritten | force = true }),
      rejected_as "publication-governance" (receipt & { governance.approval_threshold | force = 1 }),
      rejected_as "publication-governance" (receipt & { governance.signed_refs_feature | force = "root" }),
      rejected_as "publication-governance" (receipt & { governance.signed_main_delegates | force = [] }),
      rejected_as "publication-distribution" (receipt & { distribution.onix_core_policy_revision | force = missing_commit }),
      rejected_as "publication-distribution" (receipt & { distribution.public_https_git | force = "https://github.com/OnixResearch/artifact-auth" }),
      rejected_as "publication-distribution" (receipt & { distribution.replica_commit | force = missing_commit }),
      rejected_as "publication-distribution" (receipt & { distribution.github_fallback_configured | force = true }),
      rejected_as "publication-authority" (receipt & { authority_boundary.seed_delegate_keys_absent | force = false }),
      rejected_as "publication-authority" (receipt & { authority_boundary.ci_enabled | force = true }),
      rejected_as "publication-non-claims" (receipt & { non_claims | force = [] }),
      rejected_as "publication-evidence" (receipt & { evidence | force = ["/run/secrets/private_key"] }),
      rejected_as "publication-overclaim" (receipt & { private_key = "must-not-leak" }),
      rejected_as "publication-overclaim" (receipt & { claims = ["release-ready"] }),
      !publication.is_blake3 "sha256:%{digest_a}",
      !publication.is_git_oid digest_a,
      !publication.is_rid "rad:../private",
    ]
    |> std.test.assert_all,
}
