//! Explicit pinned and one-snapshot late-bound resolution.

use alloc::vec::Vec;

#[cfg(test)]
use alloc::vec;

use crate::{ArtifactId, BindingKey, BindingRecord, BindingRevision, SnapshotId};

/// Caller-selected bound for one snapshot resolution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolutionLimits {
    /// Maximum binding records admitted from one immutable snapshot.
    pub max_bindings: usize,
}

/// One immutable caller-loaded binding snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BindingSnapshot {
    /// Exact immutable snapshot identity.
    pub id: SnapshotId,
    /// Already-loaded binding records.
    pub bindings: Vec<BindingRecord>,
}

/// Exact pinned resolution request. No lookup occurs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PinnedRequest {
    /// Exact immutable target.
    pub target: ArtifactId,
    /// Binding revision retained by the caller's unit of work.
    pub binding_revision: BindingRevision,
    /// Snapshot retained by the caller's unit of work.
    pub snapshot: SnapshotId,
}

/// Late-bound resolution request against one supplied snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LateBoundRequest {
    /// Logical binding key.
    pub key: BindingKey,
    /// Exact snapshot that the caller intended to use.
    pub expected_snapshot: SnapshotId,
    /// Optional exact expected revision.
    pub expected_revision: Option<BindingRevision>,
}

/// Explicit resolution mode.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResolutionRequest {
    /// Retain an exact caller-pinned target.
    Pinned(PinnedRequest),
    /// Resolve one logical key from one immutable snapshot.
    LateBound(LateBoundRequest),
}

/// Successful exact resolution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Resolution {
    /// Exact immutable target.
    pub target: ArtifactId,
    /// Exact binding revision retained for the unit of work.
    pub binding_revision: BindingRevision,
    /// Exact snapshot identity retained for the unit of work.
    pub snapshot: SnapshotId,
    /// Logical key when late binding was used.
    pub binding_key: Option<BindingKey>,
}

/// Deterministic resolution denial.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResolutionError {
    /// The caller selected a zero record bound.
    InvalidLimits,
    /// Snapshot exceeds the caller-selected record bound.
    BindingLimitExceeded {
        /// Caller maximum.
        maximum_bindings: usize,
        /// Supplied record count.
        actual_bindings: usize,
    },
    /// Request and snapshot identities differ.
    SnapshotMismatch,
    /// One record claims a different snapshot.
    MixedSnapshotRecord {
        /// Ordered index in the supplied snapshot.
        index: usize,
    },
    /// No active candidate exists for the logical key.
    MissingTarget,
    /// More than one active candidate exists for the logical key.
    AmbiguousTarget {
        /// Number of active candidates.
        candidates: usize,
    },
    /// Caller expected another active revision.
    RevisionMismatch {
        /// Expected revision.
        expected: BindingRevision,
        /// Resolved revision.
        resolved: BindingRevision,
    },
}

fn validate_snapshot(
    snapshot: &BindingSnapshot,
    limits: ResolutionLimits,
) -> Result<(), ResolutionError> {
    if limits.max_bindings == 0 {
        return Err(ResolutionError::InvalidLimits);
    }
    if snapshot.bindings.len() > limits.max_bindings {
        return Err(ResolutionError::BindingLimitExceeded {
            maximum_bindings: limits.max_bindings,
            actual_bindings: snapshot.bindings.len(),
        });
    }
    for (index, binding) in snapshot.bindings.iter().enumerate() {
        if binding.snapshot != snapshot.id {
            return Err(ResolutionError::MixedSnapshotRecord { index });
        }
    }
    Ok(())
}

fn resolve_late_bound(
    request: &LateBoundRequest,
    snapshot: &BindingSnapshot,
    limits: ResolutionLimits,
) -> Result<Resolution, ResolutionError> {
    validate_snapshot(snapshot, limits)?;
    if request.expected_snapshot != snapshot.id {
        return Err(ResolutionError::SnapshotMismatch);
    }
    let candidates = snapshot
        .bindings
        .iter()
        .filter(|binding| binding.active && binding.key == request.key)
        .collect::<Vec<_>>();
    if candidates.is_empty() {
        return Err(ResolutionError::MissingTarget);
    }
    if candidates.len() != 1 {
        return Err(ResolutionError::AmbiguousTarget {
            candidates: candidates.len(),
        });
    }
    let binding = candidates.first().ok_or(ResolutionError::MissingTarget)?;
    if let Some(expected) = request.expected_revision {
        if expected != binding.revision {
            return Err(ResolutionError::RevisionMismatch {
                expected,
                resolved: binding.revision,
            });
        }
    }
    Ok(Resolution {
        target: binding.target.clone(),
        binding_revision: binding.revision,
        snapshot: snapshot.id.clone(),
        binding_key: Some(request.key.clone()),
    })
}

// r[impl artifact_binding.binding.resolution]
// r[impl artifact_binding.binding.snapshot_consistency]
/// Resolve one exact pinned target or one key from one immutable snapshot.
///
/// # Errors
///
/// Returns [`ResolutionError`] for invalid bounds, mixed snapshots, missing or
/// ambiguous targets, snapshot mismatch, or revision mismatch.
pub fn resolve(
    request: &ResolutionRequest,
    snapshot: Option<&BindingSnapshot>,
    limits: ResolutionLimits,
) -> Result<Resolution, ResolutionError> {
    match request {
        ResolutionRequest::Pinned(pinned) => Ok(Resolution {
            target: pinned.target.clone(),
            binding_revision: pinned.binding_revision,
            snapshot: pinned.snapshot.clone(),
            binding_key: None,
        }),
        ResolutionRequest::LateBound(late_bound) => {
            let snapshot = snapshot.ok_or(ResolutionError::MissingTarget)?;
            resolve_late_bound(late_bound, snapshot, limits)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ContractId;

    const ID_LIMIT: usize = 64;
    const SNAPSHOT_BINDING_LIMIT: usize = 8;
    const ACTIVE_REVISION: u64 = 3;
    const AMBIGUOUS_CANDIDATES: usize = 2;

    fn artifact(value: &str) -> ArtifactId {
        ArtifactId::try_new(value, ID_LIMIT)
            .unwrap_or_else(|error| unreachable!("fixture: {error}"))
    }

    fn key() -> BindingKey {
        BindingKey::try_new("service:search", ID_LIMIT)
            .unwrap_or_else(|error| unreachable!("fixture: {error}"))
    }

    fn snapshot_id(value: &str) -> SnapshotId {
        SnapshotId::try_new(value, ID_LIMIT)
            .unwrap_or_else(|error| unreachable!("fixture: {error}"))
    }

    fn binding(target: &str, snapshot: &SnapshotId) -> BindingRecord {
        BindingRecord {
            key: key(),
            revision: BindingRevision::new(ACTIVE_REVISION),
            target: artifact(target),
            contract: ContractId::try_new("contract:v1", ID_LIMIT)
                .unwrap_or_else(|error| unreachable!("fixture: {error}")),
            snapshot: snapshot.clone(),
            active: true,
        }
    }

    fn limits() -> ResolutionLimits {
        ResolutionLimits {
            max_bindings: SNAPSHOT_BINDING_LIMIT,
        }
    }

    // r[verify artifact_binding.binding.resolution]
    #[test]
    fn pinned_and_late_bound_resolution_return_exact_facts() {
        let snapshot = snapshot_id("snapshot:a");
        let pinned = resolve(
            &ResolutionRequest::Pinned(PinnedRequest {
                target: artifact("artifact:pinned"),
                binding_revision: BindingRevision::new(ACTIVE_REVISION),
                snapshot: snapshot.clone(),
            }),
            None,
            limits(),
        );
        assert_eq!(
            pinned.as_ref().map(|resolution| resolution.target.as_str()),
            Ok("artifact:pinned")
        );

        let late = resolve(
            &ResolutionRequest::LateBound(LateBoundRequest {
                key: key(),
                expected_snapshot: snapshot.clone(),
                expected_revision: Some(BindingRevision::new(ACTIVE_REVISION)),
            }),
            Some(&BindingSnapshot {
                id: snapshot.clone(),
                bindings: vec![binding("artifact:a", &snapshot)],
            }),
            limits(),
        );
        assert_eq!(
            late.as_ref()
                .map(|resolution| (resolution.target.as_str(), resolution.snapshot.as_str(),)),
            Ok(("artifact:a", "snapshot:a"))
        );
    }

    #[test]
    fn missing_ambiguous_mixed_stale_and_oversized_resolution_fail_closed() {
        let snapshot = snapshot_id("snapshot:a");
        let request = ResolutionRequest::LateBound(LateBoundRequest {
            key: key(),
            expected_snapshot: snapshot.clone(),
            expected_revision: Some(BindingRevision::new(ACTIVE_REVISION)),
        });
        assert_eq!(
            resolve(
                &request,
                Some(&BindingSnapshot {
                    id: snapshot.clone(),
                    bindings: Vec::new(),
                }),
                limits(),
            ),
            Err(ResolutionError::MissingTarget)
        );
        assert_eq!(
            resolve(
                &request,
                Some(&BindingSnapshot {
                    id: snapshot.clone(),
                    bindings: vec![
                        binding("artifact:a", &snapshot),
                        binding("artifact:b", &snapshot),
                    ],
                }),
                limits(),
            ),
            Err(ResolutionError::AmbiguousTarget {
                candidates: AMBIGUOUS_CANDIDATES,
            })
        );

        let other = snapshot_id("snapshot:other");
        assert_eq!(
            resolve(
                &request,
                Some(&BindingSnapshot {
                    id: snapshot.clone(),
                    bindings: vec![binding("artifact:a", &other)],
                }),
                limits(),
            ),
            Err(ResolutionError::MixedSnapshotRecord { index: 0 })
        );

        let oversized_snapshot = BindingSnapshot {
            id: snapshot.clone(),
            bindings: vec![binding("artifact:a", &snapshot)],
        };
        assert_eq!(
            resolve(
                &request,
                Some(&oversized_snapshot),
                ResolutionLimits { max_bindings: 0 },
            ),
            Err(ResolutionError::InvalidLimits)
        );

        let stale = ResolutionRequest::LateBound(LateBoundRequest {
            key: key(),
            expected_snapshot: snapshot_id("snapshot:stale"),
            expected_revision: None,
        });
        assert_eq!(
            resolve(
                &stale,
                Some(&BindingSnapshot {
                    id: snapshot.clone(),
                    bindings: vec![binding("artifact:a", &snapshot)],
                }),
                limits(),
            ),
            Err(ResolutionError::SnapshotMismatch)
        );
    }
}
