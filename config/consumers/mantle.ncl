let Contracts = import "../contracts.ncl" in
{
  schema = "artifact-auth.consumer-mapping.v1",
  consumer = "mantle",
  source_revision = "26c6880c08cd730c9e9420f92ea91a87212e492d",
  shared = ["domain", "subject", "parent", "signer-label", "full-key-identity", "revocation", "distinct-key-threshold", "ed25519-verification"],
  extensions = ["nix-key-identity", "oci-manifest-sha256", "metadata-manifest-sha256", "required-signer-label"],
  retained_authority = ["repository-authorization", "registry-routing", "credentials", "signing", "oci", "build", "release"],
} | Contracts.ConsumerMapping "mantle" "26c6880c08cd730c9e9420f92ea91a87212e492d"
