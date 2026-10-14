# Typed evidence for the frozen artifact-auth Radicle publication.
let PublicationReceipt = {
  schema_version | Number,
  receipt_type | String,
  status | String,
  observed_date | String,
  repository
    | {
      rid | String,
      reviewed_commit | String,
      source_archive_blake3 | String,
      object_type | String,
      default_branch | String,
      history_rewritten | Bool,
      license | String,
    },
  governance
    | {
      identity_root | String,
      identity_revision | String,
      delegates | Array String,
      approval_threshold | Number,
      signed_main_delegates | Array String,
      offline_delegate | String,
      signed_refs_feature | String,
    },
  distribution
    | {
      onix_core_policy_revision | String,
      primary_native_seed | String,
      replica_native_seed | String,
      public_https_git | String,
      primary_client_nid | String,
      replica_client_nid | String,
      primary_commit | String,
      replica_commit | String,
      https_commit | String,
      primary_source_archive_blake3 | String,
      replica_source_archive_blake3 | String,
      https_source_archive_blake3 | String,
      undeclared_rid_rejected | Bool,
      receive_pack_rejected | Bool,
      github_fallback_configured | Bool,
    },
  authority_boundary
    | {
      seed_delegate_keys_absent | Bool,
      seed_forbidden_authorities_absent | Bool,
      ci_enabled | Bool,
      private_material_exported | Bool,
      exported_artifact | String,
    },
  non_claims | Array String,
  evidence | Array String,
}
in
let expected_reviewed_commit = "799459346d5416fbd7b9f55840a7371441b55afa" in
let expected_source_archive_blake3 = "246a7cad91e7e8a158e22da21f3bff3e61aa0431a58936b5a739178bc62064c7" in
let author_delegate = "did:key:z6MksnXbFoE8zkCkGWhHc8zuxpnEUhrJHv2KECRV4GSv9gkx" in
let bonsai_delegate = "did:key:z6MkjCqx5ksRqcDeNeuEnz53udbUHebRLHhddCxecWJu9koE" in
let pine_delegate = "did:key:z6MkmkA6sEzzMffaWqKEKJcDh8LjAzgrJLrTNi971KN3X6sh" in
let expected_approval_threshold = 2 in
let receipt | PublicationReceipt = {
  schema_version = 1,
  receipt_type = "artifact-auth.radicle-publication.v1",
  status = "accepted",
  observed_date = "2026-07-25",
  repository = {
    rid = "rad:z4JGYYW7WsesXUq7MXVdx16Fawu2f",
    reviewed_commit = expected_reviewed_commit,
    source_archive_blake3 = expected_source_archive_blake3,
    object_type = "commit",
    default_branch = "main",
    history_rewritten = false,
    license = "MIT OR Apache-2.0",
  },
  governance = {
    identity_root = "e95f12e9791861f072e9a60f7d0d75f08428d721",
    identity_revision = "c22900ae6b7b5637aa0e378fe00503cf02c6d1bf",
    delegates = [author_delegate, bonsai_delegate, pine_delegate],
    approval_threshold = expected_approval_threshold,
    signed_main_delegates = [author_delegate, bonsai_delegate],
    offline_delegate = pine_delegate,
    signed_refs_feature = "parent",
  },
  distribution = {
    onix_core_policy_revision = "a3f5a18b36a0c874c1fce0d47acee19932f4d931",
    primary_native_seed = "z6MkfpHAyrqSqhpiSGayy6AjB6L5UWkKLvsZvLh5hYD7XSu8@100.100.103.95:8776",
    replica_native_seed = "z6MkkQCj5EczNiVzDzCkX9ewHNJ7NDEXSKbuRiS1x7o72yeG@100.110.43.11:8776",
    public_https_git = "https://git.onix.computer/z4JGYYW7WsesXUq7MXVdx16Fawu2f.git",
    primary_client_nid = "z6MkkQEGLf4NPLDzwguTX7LWQYxQvWnbjFvh2mECYHzuPPvb",
    replica_client_nid = "z6Mku6MfTxgiBW2gtDpAJqk6Cf6FStVbNx4JrcyDjb9bUTqQ",
    primary_commit = expected_reviewed_commit,
    replica_commit = expected_reviewed_commit,
    https_commit = expected_reviewed_commit,
    primary_source_archive_blake3 = expected_source_archive_blake3,
    replica_source_archive_blake3 = expected_source_archive_blake3,
    https_source_archive_blake3 = expected_source_archive_blake3,
    undeclared_rid_rejected = true,
    receive_pack_rejected = true,
    github_fallback_configured = false,
  },
  authority_boundary = {
    seed_delegate_keys_absent = true,
    seed_forbidden_authorities_absent = true,
    ci_enabled = false,
    private_material_exported = false,
    exported_artifact = "verified-public-git-bundle",
  },
  non_claims = [
    "artifact-auth-source-correctness",
    "consumer-behavior-correctness",
    "live-delegate-replacement",
    "third-delegate-live-convergence",
    "private-repository-confidentiality",
    "automatic-https-failover",
    "protocol-enforced-mandatory-ci",
    "release-readiness",
    "whole-stack-github-independence",
  ],
  evidence = [
    "cairn/archive/2026-07-25-publish-radicle-compatibility-source/evidence/publication-and-admission-2026-07-25.md",
    "evidence/radicle/publication-v1.json",
    "onix-core:evidence/radicle/source-admission-v1.json",
  ],
}
in
receipt
