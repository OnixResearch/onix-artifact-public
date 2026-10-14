//! Disposable compile-time pilots for exact reviewed consumer revisions.
//!
//! These tests map consumer DTO shapes into `artifact-binding-core`. They do
//! not create permanent consumer dependencies or claim consumer integration.

use artifact_binding_core::{
    ArtifactAttribution, ArtifactId, BindingKey, BindingRecord, BindingRevision, BindingSnapshot,
    CompatibilityObservation, ContractId, ExpectedBinding, GenerationAttribution, GenerationId,
    GraphRoot, LateBoundRequest, ObservationId, ProposedBinding, ReachabilityLimits,
    ReachabilitySnapshot, ResolutionLimits, ResolutionRequest, RetirementClassification,
    RetirementCompleteness, RetirementLimits, RetirementRequest, RootClassCompleteness,
    RootClassId, RootId, SnapshotId, TargetReachabilityObservation, TransitionError,
    TransitionPolicy, TransitionRequest, classify_retirement, plan_transition, resolve,
};

const ASPEN_REVISION: &str = "935729eeb4a37b3e71e77a42d4c4e7417311fd33";
const ONIXOS_REVISION: &str = "628f0b8a7036ed344aa51c74612495e28299c75a";
const EXPECTED_REVISION_BYTES: usize = 40;
const ID_LIMIT: usize = 128;
const CURRENT_REVISION: u64 = 11;
const SUCCESSOR_REVISION: u64 = CURRENT_REVISION + 1;
const PILOT_BOUND: usize = 32;
const PILOT_DIAGNOSTIC_BOUND: usize = 16;

fn artifact(value: &str) -> ArtifactId {
    ArtifactId::try_new(value, ID_LIMIT).unwrap_or_else(|error| unreachable!("fixture: {error}"))
}

fn key(value: &str) -> BindingKey {
    BindingKey::try_new(value, ID_LIMIT).unwrap_or_else(|error| unreachable!("fixture: {error}"))
}

fn contract(value: &str) -> ContractId {
    ContractId::try_new(value, ID_LIMIT).unwrap_or_else(|error| unreachable!("fixture: {error}"))
}

fn snapshot(value: &str) -> SnapshotId {
    SnapshotId::try_new(value, ID_LIMIT).unwrap_or_else(|error| unreachable!("fixture: {error}"))
}

fn observation(value: &str) -> ObservationId {
    ObservationId::try_new(value, ID_LIMIT).unwrap_or_else(|error| unreachable!("fixture: {error}"))
}

fn generation(value: &str) -> GenerationId {
    GenerationId::try_new(value, ID_LIMIT).unwrap_or_else(|error| unreachable!("fixture: {error}"))
}

fn root_class(value: &str) -> RootClassId {
    RootClassId::try_new(value, ID_LIMIT).unwrap_or_else(|error| unreachable!("fixture: {error}"))
}

fn root_id(value: &str) -> RootId {
    RootId::try_new(value, ID_LIMIT).unwrap_or_else(|error| unreachable!("fixture: {error}"))
}

fn transition(
    binding_key: &str,
    old_target: &str,
    new_target: &str,
    contract_id: &str,
    snapshot_id: &str,
    reachable: bool,
) -> TransitionRequest {
    let snapshot = snapshot(snapshot_id);
    let current = BindingRecord {
        key: key(binding_key),
        revision: BindingRevision::new(CURRENT_REVISION),
        target: artifact(old_target),
        contract: contract(contract_id),
        snapshot: snapshot.clone(),
        active: true,
    };
    let successor = ProposedBinding {
        key: key(binding_key),
        revision: BindingRevision::new(SUCCESSOR_REVISION),
        target: artifact(new_target),
        contract: contract(contract_id),
        basis_snapshot: snapshot.clone(),
    };
    TransitionRequest {
        expected: ExpectedBinding {
            key: current.key.clone(),
            revision: current.revision,
            target: current.target.clone(),
            snapshot: snapshot.clone(),
        },
        observed_current: current.clone(),
        proposed_successor: successor.clone(),
        compatibility: Some(CompatibilityObservation {
            observation_id: observation("pilot:compatibility"),
            snapshot: snapshot.clone(),
            prior_target: current.target,
            successor_target: successor.target.clone(),
            prior_contract: current.contract,
            successor_contract: successor.contract,
            compatible: true,
        }),
        successor_reachability: Some(TargetReachabilityObservation {
            observation_id: observation("pilot:reachability"),
            snapshot,
            target: successor.target,
            reachable,
        }),
        rollback_target: None,
        policy: TransitionPolicy {
            compatibility_required: true,
            successor_reachability_required: true,
        },
    }
}

fn retirement_limits() -> RetirementLimits {
    RetirementLimits {
        reachability: ReachabilityLimits {
            max_roots: PILOT_BOUND,
            max_edges: PILOT_BOUND,
            max_nodes: PILOT_BOUND,
            max_path_nodes: PILOT_BOUND,
            max_diagnostics: PILOT_DIAGNOSTIC_BOUND,
        },
        max_attributions: PILOT_BOUND,
        max_root_classes: PILOT_BOUND,
        max_issues: PILOT_BOUND,
    }
}

#[test]
fn molten_system_extension_mapping_supports_transition_resolution_and_retirement() {
    assert_eq!(ASPEN_REVISION.len(), EXPECTED_REVISION_BYTES);
    let transition = transition(
        "molten-system-extension:search",
        "blake3:molten-extension-old",
        "blake3:molten-extension-new",
        "molten-system-extension-contract:v1",
        "molten-extension-snapshot:accepted",
        true,
    );
    let plan = plan_transition(&transition);
    assert_eq!(
        plan.as_ref().map(|plan| plan.successor.target.as_str()),
        Ok("blake3:molten-extension-new")
    );

    let resolved = resolve(
        &ResolutionRequest::LateBound(LateBoundRequest {
            key: transition.proposed_successor.key.clone(),
            expected_snapshot: transition.proposed_successor.basis_snapshot.clone(),
            expected_revision: Some(BindingRevision::new(SUCCESSOR_REVISION)),
        }),
        Some(&BindingSnapshot {
            id: transition.proposed_successor.basis_snapshot.clone(),
            bindings: vec![BindingRecord {
                key: transition.proposed_successor.key.clone(),
                revision: transition.proposed_successor.revision,
                target: transition.proposed_successor.target.clone(),
                contract: transition.proposed_successor.contract.clone(),
                snapshot: transition.proposed_successor.basis_snapshot.clone(),
                active: true,
            }],
        }),
        ResolutionLimits {
            max_bindings: PILOT_BOUND,
        },
    );
    assert_eq!(
        resolved
            .as_ref()
            .map(|resolution| resolution.target.as_str()),
        Ok("blake3:molten-extension-new")
    );

    let graph_snapshot = snapshot("molten-retirement-snapshot:accepted");
    let session_class = root_class("molten-runtime-vat");
    let old_generation = generation("molten-generation:old");
    let new_generation = generation("molten-generation:new");
    let retired = classify_retirement(
        &RetirementRequest {
            generation: old_generation.clone(),
            declared_root_classes: vec![session_class.clone()],
            graph: ReachabilitySnapshot {
                id: graph_snapshot.clone(),
                roots: vec![GraphRoot {
                    class: session_class.clone(),
                    id: root_id("molten-vat:search"),
                    target: artifact("blake3:molten-extension-new"),
                    generation_scope: Some(new_generation.clone()),
                }],
                edges: Vec::new(),
            },
            attributions: vec![ArtifactAttribution {
                snapshot: graph_snapshot.clone(),
                artifact: artifact("blake3:molten-extension-new"),
                ownership: GenerationAttribution::Exclusive(new_generation),
            }],
            completeness: RetirementCompleteness {
                snapshot: graph_snapshot,
                root_classes: vec![RootClassCompleteness {
                    class: session_class,
                    complete: true,
                }],
                edge_inventory_complete: true,
                attribution_inventory_complete: true,
            },
        },
        retirement_limits(),
    );
    assert_eq!(
        retired.as_ref().map(|decision| decision.classification),
        Ok(RetirementClassification::Retired)
    );
}

#[test]
fn onixos_active_rollback_descriptor_mapping_preserves_successor_rules() {
    assert_eq!(ONIXOS_REVISION.len(), EXPECTED_REVISION_BYTES);
    let mut rollback = transition(
        "onixos-generation:web",
        "blake3:onixos-active-generation",
        "blake3:onixos-rollback-generation",
        "onixos-generation-contract:v1",
        "onixos-generation-snapshot:accepted",
        true,
    );
    rollback.rollback_target = Some(artifact("blake3:onixos-rollback-generation"));
    let plan = plan_transition(&rollback);
    assert!(plan.as_ref().is_ok_and(|plan| plan.rollback_as_successor));
    assert_eq!(
        plan.as_ref().map(|plan| plan.successor.revision),
        Ok(BindingRevision::new(SUCCESSOR_REVISION))
    );

    let mut unreachable = rollback;
    if let Some(observation) = unreachable.successor_reachability.as_mut() {
        observation.reachable = false;
    }
    assert_eq!(
        plan_transition(&unreachable),
        Err(TransitionError::SuccessorUnreachable)
    );
}
