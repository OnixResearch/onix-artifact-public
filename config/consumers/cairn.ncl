let Contracts = import "../contracts.ncl" in
{
  schema = "artifact-auth.consumer-mapping.v1",
  consumer = "cairn",
  source_revision = "cc4639be24475d80844b091cdec605eb2fef9ff0",
  shared = ["domain", "purpose", "producer", "key-label", "subject", "parents", "ed25519-observation"],
  extensions = ["legacy-public-key-hex-blake3", "evidence-kind", "canonical-receipt-profile", "valence-integrity-receipt"],
  retained_authority = ["filesystem-measurement", "receipt-canonicalization", "lifecycle-policy", "readiness", "json", "process-exit"],
} | Contracts.ConsumerMapping "cairn" "cc4639be24475d80844b091cdec605eb2fef9ff0"
