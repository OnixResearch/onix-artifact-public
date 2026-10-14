let Contracts = import "../contracts.ncl" in
{
  schema = "artifact-auth.consumer-mapping.v1",
  consumer = "molten",
  source_revision = "70590459a218dc8e66948ab6f305a7c54142b710",
  shared = ["domain", "purpose", "profile", "payload-ref", "public-key-ref", "verifier-context-ref", "generation", "currentness"],
  extensions = ["opaque-handle", "backend-class", "entropy-profile", "rotation-transition"],
  retained_authority = ["key-generation", "signing", "key-storage", "capability-authority", "federation-membership", "preserves", "iroh"],
} | Contracts.ConsumerMapping "molten" "70590459a218dc8e66948ab6f305a7c54142b710"
