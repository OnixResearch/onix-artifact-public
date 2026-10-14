//! Pure compare-and-swap transition planning.

use crate::{ArtifactId, BindingKey, BindingRevision, ContractId, ObservationId, SnapshotId};

/// One caller-loaded logical binding record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BindingRecord {
    /// Logical name owned by the consumer.
    pub key: BindingKey,
    /// Monotonic revision owned by the consumer record.
    pub revision: BindingRevision,
    /// Exact immutable target.
    pub target: ArtifactId,
    /// Consumer-owned compatibility contract reference.
    pub contract: ContractId,
    /// Immutable snapshot from which this record was observed.
    pub snapshot: SnapshotId,
    /// Whether this record is active in its supplied snapshot.
    pub active: bool,
}

/// Proposed successor facts based on one immutable current snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposedBinding {
    /// Logical name owned by the consumer.
    pub key: BindingKey,
    /// Exact proposed successor revision.
    pub revision: BindingRevision,
    /// Exact proposed immutable target.
    pub target: ArtifactId,
    /// Consumer-owned compatibility contract reference.
    pub contract: ContractId,
    /// Immutable current snapshot used to construct the proposal.
    pub basis_snapshot: SnapshotId,
}

/// Expected compare-and-swap facts supplied by the caller.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpectedBinding {
    /// Expected logical key.
    pub key: BindingKey,
    /// Expected current revision.
    pub revision: BindingRevision,
    /// Expected current exact target.
    pub target: ArtifactId,
    /// Expected immutable snapshot.
    pub snapshot: SnapshotId,
}

/// Caller assertion about exact old/new compatibility.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompatibilityObservation {
    /// Consumer-owned observation identity.
    pub observation_id: ObservationId,
    /// Immutable snapshot for this observation.
    pub snapshot: SnapshotId,
    /// Exact prior target.
    pub prior_target: ArtifactId,
    /// Exact successor target.
    pub successor_target: ArtifactId,
    /// Contract used for the prior target.
    pub prior_contract: ContractId,
    /// Contract used for the successor target.
    pub successor_contract: ContractId,
    /// Caller assertion. The core does not prove its truth.
    pub compatible: bool,
}

/// Caller assertion that one exact successor is reachable in one snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetReachabilityObservation {
    /// Consumer-owned observation identity.
    pub observation_id: ObservationId,
    /// Immutable snapshot used for the observation.
    pub snapshot: SnapshotId,
    /// Exact target checked by the caller.
    pub target: ArtifactId,
    /// Caller assertion. The core does not discover reachability.
    pub reachable: bool,
}

/// Explicit structural transition policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransitionPolicy {
    /// Require one exact caller compatibility observation.
    pub compatibility_required: bool,
    /// Require the caller to report the successor target reachable.
    pub successor_reachability_required: bool,
}

/// Complete pure transition request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransitionRequest {
    /// Expected current compare-and-swap facts.
    pub expected: ExpectedBinding,
    /// Already-loaded current record.
    pub observed_current: BindingRecord,
    /// Proposed successor facts based on the current snapshot.
    pub proposed_successor: ProposedBinding,
    /// Optional exact compatibility observation.
    pub compatibility: Option<CompatibilityObservation>,
    /// Optional caller observation from its own reachability mechanism.
    pub successor_reachability: Option<TargetReachabilityObservation>,
    /// Optional older immutable target selected for rollback.
    pub rollback_target: Option<ArtifactId>,
    /// Structural policy.
    pub policy: TransitionPolicy,
}

/// Pure compare-and-swap plan. It does not claim publication occurred.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransitionPlan {
    /// Prior binding retained for history and rollback evidence.
    pub prior: BindingRecord,
    /// Exact checked successor proposal.
    pub successor: ProposedBinding,
    /// Accepted compatibility observation identity, when required or supplied.
    pub compatibility_observation: Option<ObservationId>,
    /// Accepted successor-reachability observation identity, when supplied.
    pub reachability_observation: Option<ObservationId>,
    /// True only when the successor targets the caller-selected older ref.
    pub rollback_as_successor: bool,
}

/// Structural transition denial.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransitionError {
    /// Expected and observed binding keys differ.
    StaleBindingKey,
    /// Expected and observed revisions differ.
    StaleRevision {
        /// Expected revision.
        expected: BindingRevision,
        /// Observed revision.
        observed: BindingRevision,
    },
    /// Expected and observed exact targets differ.
    StaleTarget,
    /// Expected and observed immutable snapshots differ.
    SnapshotMismatch,
    /// Current record is not active in the supplied snapshot.
    CurrentBindingInactive,
    /// Successor changed the logical binding key.
    SuccessorKeyMismatch,
    /// Successor revision arithmetic overflowed.
    RevisionOverflow,
    /// Proposed revision is not the exact checked successor.
    NonSuccessorRevision {
        /// Required successor revision.
        expected: BindingRevision,
        /// Proposed revision.
        proposed: BindingRevision,
    },
    /// Successor facts were mixed with another snapshot.
    SuccessorSnapshotMismatch,
    /// Policy requires a compatibility observation.
    CompatibilityMissing,
    /// Compatibility observation does not bind exact old/new facts.
    CompatibilityBindingMismatch,
    /// Caller explicitly rejected compatibility.
    CompatibilityRejected,
    /// Policy requires a successor-reachability observation.
    SuccessorReachabilityMissing,
    /// Reachability observation does not bind the exact target and snapshot.
    SuccessorReachabilityBindingMismatch,
    /// Policy requires a reachable successor and the caller reported otherwise.
    SuccessorUnreachable,
}

fn validate_expected(request: &TransitionRequest) -> Result<(), TransitionError> {
    let expected = &request.expected;
    let current = &request.observed_current;
    if expected.key != current.key {
        return Err(TransitionError::StaleBindingKey);
    }
    if expected.revision != current.revision {
        return Err(TransitionError::StaleRevision {
            expected: expected.revision,
            observed: current.revision,
        });
    }
    if expected.target != current.target {
        return Err(TransitionError::StaleTarget);
    }
    if expected.snapshot != current.snapshot {
        return Err(TransitionError::SnapshotMismatch);
    }
    if !current.active {
        return Err(TransitionError::CurrentBindingInactive);
    }
    Ok(())
}

fn validate_successor(request: &TransitionRequest) -> Result<(), TransitionError> {
    let current = &request.observed_current;
    let successor = &request.proposed_successor;
    if successor.key != current.key {
        return Err(TransitionError::SuccessorKeyMismatch);
    }
    if successor.basis_snapshot != current.snapshot {
        return Err(TransitionError::SuccessorSnapshotMismatch);
    }
    let expected_revision = current
        .revision
        .checked_successor()
        .ok_or(TransitionError::RevisionOverflow)?;
    if successor.revision != expected_revision {
        return Err(TransitionError::NonSuccessorRevision {
            expected: expected_revision,
            proposed: successor.revision,
        });
    }
    Ok(())
}

fn validate_reachability(
    request: &TransitionRequest,
) -> Result<Option<ObservationId>, TransitionError> {
    let Some(observation) = request.successor_reachability.as_ref() else {
        if request.policy.successor_reachability_required {
            return Err(TransitionError::SuccessorReachabilityMissing);
        }
        return Ok(None);
    };
    if observation.snapshot != request.observed_current.snapshot
        || observation.target != request.proposed_successor.target
    {
        return Err(TransitionError::SuccessorReachabilityBindingMismatch);
    }
    if !observation.reachable {
        return Err(TransitionError::SuccessorUnreachable);
    }
    Ok(Some(observation.observation_id.clone()))
}

fn validate_compatibility(
    request: &TransitionRequest,
) -> Result<Option<ObservationId>, TransitionError> {
    let Some(observation) = request.compatibility.as_ref() else {
        if request.policy.compatibility_required {
            return Err(TransitionError::CompatibilityMissing);
        }
        return Ok(None);
    };
    let current = &request.observed_current;
    let successor = &request.proposed_successor;
    if observation.snapshot != current.snapshot
        || observation.prior_target != current.target
        || observation.successor_target != successor.target
        || observation.prior_contract != current.contract
        || observation.successor_contract != successor.contract
    {
        return Err(TransitionError::CompatibilityBindingMismatch);
    }
    if !observation.compatible {
        return Err(TransitionError::CompatibilityRejected);
    }
    Ok(Some(observation.observation_id.clone()))
}

// r[impl artifact_binding.binding.transition]
// r[impl artifact_binding.binding.compatibility]
/// Validate a structural successor and return a publication-neutral plan.
///
/// # Errors
///
/// Returns a stable [`TransitionError`] for stale compare-and-swap facts,
/// non-successor revisions, overflow, snapshot mixing, unavailable
/// compatibility, rejected compatibility, or unreachable successors.
pub fn plan_transition(request: &TransitionRequest) -> Result<TransitionPlan, TransitionError> {
    validate_expected(request)?;
    validate_successor(request)?;
    let compatibility_observation = validate_compatibility(request)?;
    let reachability_observation = validate_reachability(request)?;
    let rollback_as_successor = request
        .rollback_target
        .as_ref()
        .is_some_and(|target| target == &request.proposed_successor.target);
    Ok(TransitionPlan {
        prior: request.observed_current.clone(),
        successor: request.proposed_successor.clone(),
        compatibility_observation,
        reachability_observation,
        rollback_as_successor,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const ID_LIMIT: usize = 64;
    const CURRENT_REVISION: u64 = 7;
    const SUCCESSOR_REVISION: u64 = CURRENT_REVISION + 1;
    const SKIPPED_REVISION: u64 = SUCCESSOR_REVISION + 1;

    fn artifact(value: &str) -> ArtifactId {
        ArtifactId::try_new(value, ID_LIMIT)
            .unwrap_or_else(|error| unreachable!("fixture: {error}"))
    }

    fn key() -> BindingKey {
        BindingKey::try_new("service:search", ID_LIMIT)
            .unwrap_or_else(|error| unreachable!("fixture: {error}"))
    }

    fn contract() -> ContractId {
        ContractId::try_new("contract:service-v1", ID_LIMIT)
            .unwrap_or_else(|error| unreachable!("fixture: {error}"))
    }

    fn snapshot() -> SnapshotId {
        SnapshotId::try_new("snapshot:current", ID_LIMIT)
            .unwrap_or_else(|error| unreachable!("fixture: {error}"))
    }

    fn observation_id() -> ObservationId {
        ObservationId::try_new("observation:compatibility", ID_LIMIT)
            .unwrap_or_else(|error| unreachable!("fixture: {error}"))
    }

    fn request() -> TransitionRequest {
        let current = BindingRecord {
            key: key(),
            revision: BindingRevision::new(CURRENT_REVISION),
            target: artifact("artifact:old"),
            contract: contract(),
            snapshot: snapshot(),
            active: true,
        };
        let successor = ProposedBinding {
            key: key(),
            revision: BindingRevision::new(SUCCESSOR_REVISION),
            target: artifact("artifact:new"),
            contract: contract(),
            basis_snapshot: snapshot(),
        };
        TransitionRequest {
            expected: ExpectedBinding {
                key: current.key.clone(),
                revision: current.revision,
                target: current.target.clone(),
                snapshot: current.snapshot.clone(),
            },
            observed_current: current.clone(),
            proposed_successor: successor.clone(),
            compatibility: Some(CompatibilityObservation {
                observation_id: observation_id(),
                snapshot: current.snapshot.clone(),
                prior_target: current.target.clone(),
                successor_target: successor.target.clone(),
                prior_contract: current.contract,
                successor_contract: successor.contract,
                compatible: true,
            }),
            successor_reachability: Some(TargetReachabilityObservation {
                observation_id: ObservationId::try_new("observation:reachability", ID_LIMIT)
                    .unwrap_or_else(|error| unreachable!("fixture: {error}")),
                snapshot: current.snapshot,
                target: successor.target,
                reachable: true,
            }),
            rollback_target: None,
            policy: TransitionPolicy {
                compatibility_required: true,
                successor_reachability_required: true,
            },
        }
    }

    // r[verify artifact_binding.binding.transition]
    #[test]
    fn exact_successor_and_rollback_as_successor_are_planned() {
        let update = plan_transition(&request());
        assert!(update.is_ok());
        assert!(!update.as_ref().is_ok_and(|plan| plan.rollback_as_successor));

        let mut rollback = request();
        rollback.proposed_successor.target = rollback.observed_current.target.clone();
        rollback.rollback_target = Some(rollback.observed_current.target.clone());
        if let Some(observation) = rollback.compatibility.as_mut() {
            observation.successor_target = rollback.proposed_successor.target.clone();
        }
        if let Some(observation) = rollback.successor_reachability.as_mut() {
            observation.target = rollback.proposed_successor.target.clone();
        }
        let plan = plan_transition(&rollback);
        assert!(plan.as_ref().is_ok_and(|plan| plan.rollback_as_successor));
        assert_eq!(
            plan.as_ref().map(|plan| plan.successor.revision),
            Ok(BindingRevision::new(SUCCESSOR_REVISION))
        );
    }

    #[test]
    fn stale_skipped_reversed_and_overflowing_revisions_fail_separately() {
        let mut stale = request();
        stale.expected.revision = BindingRevision::new(CURRENT_REVISION - 1);
        assert!(matches!(
            plan_transition(&stale),
            Err(TransitionError::StaleRevision { .. })
        ));

        let mut skipped = request();
        skipped.proposed_successor.revision = BindingRevision::new(SKIPPED_REVISION);
        assert!(matches!(
            plan_transition(&skipped),
            Err(TransitionError::NonSuccessorRevision { .. })
        ));

        let mut reversed = request();
        reversed.proposed_successor.revision = BindingRevision::new(CURRENT_REVISION - 1);
        assert!(matches!(
            plan_transition(&reversed),
            Err(TransitionError::NonSuccessorRevision { .. })
        ));

        let mut overflow = request();
        overflow.observed_current.revision = BindingRevision::new(u64::MAX);
        overflow.expected.revision = BindingRevision::new(u64::MAX);
        assert_eq!(
            plan_transition(&overflow),
            Err(TransitionError::RevisionOverflow)
        );
    }

    #[test]
    fn compatibility_snapshot_and_reachability_fail_closed() {
        let mut missing = request();
        missing.compatibility = None;
        assert_eq!(
            plan_transition(&missing),
            Err(TransitionError::CompatibilityMissing)
        );

        let mut substituted = request();
        if let Some(observation) = substituted.compatibility.as_mut() {
            observation.successor_target = artifact("artifact:substituted");
        }
        assert_eq!(
            plan_transition(&substituted),
            Err(TransitionError::CompatibilityBindingMismatch)
        );

        let mut mixed_snapshot = request();
        mixed_snapshot.proposed_successor.basis_snapshot =
            SnapshotId::try_new("snapshot:mixed", ID_LIMIT)
                .unwrap_or_else(|error| unreachable!("fixture: {error}"));
        assert_eq!(
            plan_transition(&mixed_snapshot),
            Err(TransitionError::SuccessorSnapshotMismatch)
        );

        let mut unreachable = request();
        if let Some(observation) = unreachable.successor_reachability.as_mut() {
            observation.reachable = false;
        }
        assert_eq!(
            plan_transition(&unreachable),
            Err(TransitionError::SuccessorUnreachable)
        );

        let mut substituted_reachability = request();
        if let Some(observation) = substituted_reachability.successor_reachability.as_mut() {
            observation.target = artifact("artifact:substituted");
        }
        assert_eq!(
            plan_transition(&substituted_reachability),
            Err(TransitionError::SuccessorReachabilityBindingMismatch)
        );

        let mut missing_reachability = request();
        missing_reachability.successor_reachability = None;
        assert_eq!(
            plan_transition(&missing_reachability),
            Err(TransitionError::SuccessorReachabilityMissing)
        );
    }
}
