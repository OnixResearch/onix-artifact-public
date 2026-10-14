let Contracts = import "../contracts.ncl" in
{
  schema = "artifact-auth.consumer-mapping.v1",
  consumer = "valence",
  source_revision = "8eccac35fcd8aae8ef64b3d1021e7f1a53656180",
  shared = ["protocol-verified-key", "entry-subject", "parent-oids", "related-target", "profile-context"],
  extensions = ["radicle-entry-oid", "radicle-public-key", "git-parent-oid", "evidence-ir-identity"],
  retained_authority = ["radicle-verification", "actor-authorization", "evidence-ir", "cob-reduction", "git-storage", "ssh-agent", "lifecycle-handoff"],
} | Contracts.ConsumerMapping "valence" "8eccac35fcd8aae8ef64b3d1021e7f1a53656180"
