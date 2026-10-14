# Frozen legacy baselines

These vectors freeze exact pre-extraction behavior from the fixed source cohort. They are compatibility observations, not alternate standalone schemas and not claims that the legacy statement identities equal `artifact-auth.statement.v1` identities.

## Cairn authenticated input

Source: Cairn `cc4639be24475d80844b091cdec605eb2fef9ff0`, `crates/cairn-core/src/verified/authenticated_inputs.rs`.

The positive core fixture, evaluated with canonical subject `{"claim":"bounded"}`, produces the exact UTF-8 bytes in [`fixtures/baselines/cairn-signature-message.json`](../fixtures/baselines/cairn-signature-message.json). The file has no trailing newline.

- byte length: `988`
- BLAKE3 byte identity: `65268c8bf653065b42f916ef79212513fbee3cc690f18c80a3b975021d7367ef`
- legacy public-key input: lowercase hexadecimal text `456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123`
- legacy BLAKE3-of-hex-text key identity: `fd5a0f16f9af5f2739494a1bce29a4571d3845ea8a61c47e3e2e57b82975fe91`

Cairn sorts measurements by role/path and parent hashes lexicographically, then uses recursively key-sorted compact JSON. Its legacy key identity hashes the encoded lowercase text, not raw key bytes. The standalone adapter must dual-record this identity and the standalone raw-key identity rather than equating them.

Focused baseline commands passed:

```console
nix develop -c cargo test -p cairn-core authenticated_inputs
nix develop -c cargo test -p cairn-cli authenticated_inputs
```

The negative core corpus freezes measured digest mismatch, receipt identity drift, unsupported canonicalization, stale/placeholder policy, wrong producer, revoked key, wrong purpose, missing parent, and failed cryptographic observation issue classes.

## Mantle OCI trust statement

Source: Mantle `26c6880c08cd730c9e9420f92ea91a87212e492d`, `src/oci_registry.rs`.

The positive push fixture with `sha256:` plus 64 `a` digits for the manifest and 64 `b` digits for the metadata manifest produces the exact UTF-8 bytes in [`fixtures/baselines/mantle-trust-statement.json`](../fixtures/baselines/mantle-trust-statement.json). The file has no trailing newline.

- byte length: `339`
- BLAKE3 compatibility identity of those bytes: `d6adc008cce03ea5b39560b0a5586dc6523479e70cb1f7e93d57ec6ee230fbb4`
- trust domain: `onix-kernel-bundle`
- signature suite: `ed25519-detached-v1`
- fixed report fixture public-key identity: `cad153db7d4f564bbfab4e10b6f31460ec672cdd5f296ab1a33d19371f028fab`

The statement itself uses canonical JSON and explicitly labeled OCI SHA-256 identities. Mantle's public-key identity uses BLAKE3 key derivation under `mantle/oci/registry-public-key/v1` over encoded key material, so it remains compatibility metadata beside the standalone raw-key identity.

Focused baseline commands passed:

```console
nix develop -c cargo test -p crunch-action-result-core
nix develop -c cargo test -p crunch-attestation-core
nix develop -c cargo test -p crunch-build signing::tests
```

The focused Mantle signing filter ran fifteen tests. Negative behavior freezes unknown/revoked keys, same-label distinct-key handling, duplicate detached signatures, wrong trust domain, immutable digest drift, non-canonical signature ordering, threshold failure, and unsupported statement metadata.

## Standalone enforcement

`artifact-auth-core/tests/conformance.rs` verifies both baseline files remain compact canonical JSON with the exact BLAKE3 identities above. The same test recomputes Cairn's legacy key identity. Standalone conformance vectors separately freeze the new length-delimited statement identity and fail-closed decisions.
