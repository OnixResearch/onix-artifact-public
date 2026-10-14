# Disposable binding pilots

The pilots compile consumer-shaped mappings without permanent sibling dependencies. They are in `crates/artifact-binding-core/tests/pilots.rs`.

## Reviewed sources

The Molten pilot uses Aspen revision `935729eeb4a37b3e71e77a42d4c4e7417311fd33`.

The mapping was reviewed against these source surfaces:

- `crates/molten-core/src/system_extension/manifest.rs`: `AdmittedSystemExtensionManifest` carries `extension_id`, `service_id`, `implementation_ref`, state schemas, and `initial_generation`.
- `crates/molten-core/src/system_extension/dispatch.rs`: callback and typed-effect checks use the admitted manifest and active generation.
- `crates/molten-core/src/fabric/tier.rs`: `ExtensionTier::SystemExtension` retains product admission authority.

The OnixOS pilot uses revision `628f0b8a7036ed344aa51c74612495e28299c75a`.

The mapping was reviewed against these source surfaces:

- `cli/src/pure_domains/bpf_runtime_lifecycle.rs`: `BpfRuntimeInventory` carries active and retained generations.
- The same file uses `expected_predecessor` and `rollback_generation` in `BpfLifecyclePlanRequest`.
- `cli/src/cmd/bpf_runtime_agent.rs` loads retained rollback generations and retains adapter authority.

## Molten mapping

The pilot maps a system-extension service to `BindingKey`. It maps `implementation_ref` to `ArtifactId` and the state compatibility contract to `ContractId`.

An update uses an exact successor revision. One late-bound lookup returns the exact target, revision, and snapshot.

The retirement case maps a Molten runtime vat to a root. A root moved to the new generation allows the complete old generation to become `Retired`.

## OnixOS mapping

The pilot maps an active generation and a retained rollback generation to current and successor binding targets.

Rollback is a new successor revision that points to the retained target. An unreachable rollback target fails before any consumer effect.

## Evidence limit

The pilot proves that these selected DTO shapes can map into the pure core at the named revisions. It does not prove Aspen or OnixOS integration.

It does not prove runtime reachability, compatibility, atomic publication, deployment success, cleanup safety, or deletion authority. Each consumer needs a separate repo-local change before adoption.
