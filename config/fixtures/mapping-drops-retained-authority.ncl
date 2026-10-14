let Contracts = import "../contracts.ncl" in
{
  schema = "artifact-auth.consumer-mapping.v1",
  consumer = "cairn",
  source_revision = "cc4639be24475d80844b091cdec605eb2fef9ff0",
  shared = ["domain"],
  extensions = [],
  retained_authority = [],
} | Contracts.ConsumerMapping "cairn" "cc4639be24475d80844b091cdec605eb2fef9ff0"
