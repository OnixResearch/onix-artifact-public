# Pure deterministic validator for the frozen artifact-auth publication receipt.
let _digest_hex_chars = 64 in
let _git_oid_hex_chars = 40 in
let _delegate_count = 3 in
let _approval_threshold = 2 in
let _blake3_regex = "^[0-9a-f]{%{std.to_string _digest_hex_chars}}$" in
let _git_oid_regex = "^[0-9a-f]{%{std.to_string _git_oid_hex_chars}}$" in
let _rid_regex = "^rad:z[1-9A-HJ-NP-Za-km-z]+$" in
let _did_regex = "^did:key:z[1-9A-HJ-NP-Za-km-z]+$" in
let _expected = {
  rid = "rad:z4JGYYW7WsesXUq7MXVdx16Fawu2f",
  commit = "799459346d5416fbd7b9f55840a7371441b55afa",
  source_archive_blake3 = "246a7cad91e7e8a158e22da21f3bff3e61aa0431a58936b5a739178bc62064c7",
  https_git = "https://git.onix.computer/z4JGYYW7WsesXUq7MXVdx16Fawu2f.git",
  delegates = [
    "did:key:z6MksnXbFoE8zkCkGWhHc8zuxpnEUhrJHv2KECRV4GSv9gkx",
    "did:key:z6MkjCqx5ksRqcDeNeuEnz53udbUHebRLHhddCxecWJu9koE",
    "did:key:z6MkmkA6sEzzMffaWqKEKJcDh8LjAzgrJLrTNi971KN3X6sh",
  ],
  signed_main_delegates = [
    "did:key:z6MksnXbFoE8zkCkGWhHc8zuxpnEUhrJHv2KECRV4GSv9gkx",
    "did:key:z6MkjCqx5ksRqcDeNeuEnz53udbUHebRLHhddCxecWJu9koE",
  ],
  offline_delegate = "did:key:z6MkmkA6sEzzMffaWqKEKJcDh8LjAzgrJLrTNi971KN3X6sh",
}
in
let _required_root_fields = [
  "schema_version",
  "receipt_type",
  "status",
  "observed_date",
  "repository",
  "governance",
  "distribution",
  "authority_boundary",
  "non_claims",
  "evidence",
]
in
let _required_non_claims = [
  "artifact-auth-source-correctness",
  "consumer-behavior-correctness",
  "live-delegate-replacement",
  "third-delegate-live-convergence",
  "private-repository-confidentiality",
  "automatic-https-failover",
  "protocol-enforced-mandatory-ci",
  "release-readiness",
  "whole-stack-github-independence",
]
in
let _has_fields = fun fields value =>
  std.is_record value && fields |> std.array.all (fun field => std.record.has_field field value)
in
let _is_blake3 = fun value => std.is_string value && std.string.is_match _blake3_regex value in
let _is_git_oid = fun value => std.is_string value && std.string.is_match _git_oid_regex value in
let _is_rid = fun value => std.is_string value && std.string.is_match _rid_regex value in
let _is_did = fun value => std.is_string value && std.string.is_match _did_regex value in
let _error = fun error_code error_message =>
  {
    code = error_code,
    message = error_message,
  }
in
let _first_error = fun errors =>
  errors
  |> std.array.filter_map (fun error => if error == null then 'None else 'Some error)
  |> match { [] => 'Ok, [error, ..] => 'Error error }
in
let _validate = fun receipt =>
  if !_has_fields _required_root_fields receipt then
    'Error (_error "publication-shape" "publication evidence is incomplete")
  else
    _first_error [
      if receipt.schema_version == 1
      && receipt.receipt_type == "artifact-auth.radicle-publication.v1"
      && receipt.status == "accepted"
      && receipt.observed_date == "2026-07-25" then
        null
      else
        _error "publication-header" "publication identity or status drifted",
      if _has_fields ["rid", "reviewed_commit", "source_archive_blake3", "object_type", "default_branch", "history_rewritten", "license"] receipt.repository
      && _is_rid receipt.repository.rid
      && receipt.repository.rid == _expected.rid
      && _is_git_oid receipt.repository.reviewed_commit
      && receipt.repository.reviewed_commit == _expected.commit
      && _is_blake3 receipt.repository.source_archive_blake3
      && receipt.repository.source_archive_blake3 == _expected.source_archive_blake3
      && receipt.repository.object_type == "commit"
      && receipt.repository.default_branch == "main"
      && !receipt.repository.history_rewritten
      && receipt.repository.license == "MIT OR Apache-2.0" then
        null
      else
        _error "publication-repository" "frozen repository identity or license drifted",
      if _has_fields ["identity_root", "identity_revision", "delegates", "approval_threshold", "signed_main_delegates", "offline_delegate", "signed_refs_feature"] receipt.governance
      && _is_git_oid receipt.governance.identity_root
      && _is_git_oid receipt.governance.identity_revision
      && std.array.length receipt.governance.delegates == _delegate_count
      && receipt.governance.delegates == _expected.delegates
      && receipt.governance.delegates
      |> std.array.all _is_did
      && receipt.governance.approval_threshold == _approval_threshold
      && receipt.governance.signed_main_delegates == _expected.signed_main_delegates
      && receipt.governance.offline_delegate == _expected.offline_delegate
      && receipt.governance.signed_refs_feature == "parent" then
        null
      else
        _error "publication-governance" "delegate, threshold, or signed-reference evidence drifted",
      if receipt.distribution.onix_core_policy_revision == "a3f5a18b36a0c874c1fce0d47acee19932f4d931"
      && receipt.distribution.public_https_git == _expected.https_git
      && receipt.distribution.primary_commit == _expected.commit
      && receipt.distribution.replica_commit == _expected.commit
      && receipt.distribution.https_commit == _expected.commit
      && receipt.distribution.primary_source_archive_blake3 == _expected.source_archive_blake3
      && receipt.distribution.replica_source_archive_blake3 == _expected.source_archive_blake3
      && receipt.distribution.https_source_archive_blake3 == _expected.source_archive_blake3
      && receipt.distribution.undeclared_rid_rejected
      && receipt.distribution.receive_pack_rejected
      && !receipt.distribution.github_fallback_configured then
        null
      else
        _error "publication-distribution" "deployment, endpoint, or exact-source evidence drifted",
      if receipt.authority_boundary.seed_delegate_keys_absent
      && receipt.authority_boundary.seed_forbidden_authorities_absent
      && !receipt.authority_boundary.ci_enabled
      && !receipt.authority_boundary.private_material_exported
      && receipt.authority_boundary.exported_artifact == "verified-public-git-bundle" then
        null
      else
        _error "publication-authority" "publication widened seed, CI, or private-material authority",
      if _required_non_claims |> std.array.all (fun claim => std.array.elem claim receipt.non_claims) then
        null
      else
        _error "publication-non-claims" "required non-claims are missing",
      if receipt.evidence |> std.array.all (fun path => std.is_string path && !(std.string.is_match "^/" path) && !(std.string.contains "private_key" path)) then
        null
      else
        _error "publication-evidence" "evidence path is absolute or secret-bearing",
      if !(std.record.has_field "private_key" receipt)
      && !(std.record.has_field "credentials" receipt)
      && !(std.record.has_field "claims" receipt) then
        null
      else
        _error "publication-overclaim" "receipt contains private material or unsupported claims",
    ]
in
{
  expected = _expected,
  required_non_claims = _required_non_claims,
  is_blake3 = _is_blake3,
  is_git_oid = _is_git_oid,
  is_rid = _is_rid,
  validate = _validate,
}
