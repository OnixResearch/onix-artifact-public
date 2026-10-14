//! Conservative generation retirement classification.

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec;
use alloc::vec::Vec;

const SHARED_GENERATION_MINIMUM: usize = 2;

use crate::{
    ArtifactId, GenerationId, PinPath, ReachabilityError, ReachabilityLimits, ReachabilitySnapshot,
    ReachabilityTrace, RootClassId, RootId, SnapshotId, trace_reachability,
};

/// Completeness observation for one declared root class.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RootClassCompleteness {
    /// Declared class.
    pub class: RootClassId,
    /// Caller assertion that the class inventory is complete.
    pub complete: bool,
}

/// Explicit generation ownership shape for one immutable artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GenerationAttribution {
    /// Exactly one generation owns use of the artifact.
    Exclusive(GenerationId),
    /// Two or more generations share the immutable artifact.
    Shared(Vec<GenerationId>),
}

/// Caller-supplied generation attribution for one immutable artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactAttribution {
    /// Exact immutable snapshot used for this attribution.
    pub snapshot: SnapshotId,
    /// Exact immutable artifact.
    pub artifact: ArtifactId,
    /// Explicit exclusive or shared generation ownership.
    pub ownership: GenerationAttribution,
}

/// Explicit observation completeness facts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetirementCompleteness {
    /// Exact immutable snapshot covered by these completeness observations.
    pub snapshot: SnapshotId,
    /// Completeness for every caller-declared root class.
    pub root_classes: Vec<RootClassCompleteness>,
    /// Caller assertion that edge inventory is complete.
    pub edge_inventory_complete: bool,
    /// Caller assertion that attribution inventory is complete.
    pub attribution_inventory_complete: bool,
}

/// Explicit classification bounds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetirementLimits {
    /// Reachability bounds.
    pub reachability: ReachabilityLimits,
    /// Maximum caller-supplied attribution records.
    pub max_attributions: usize,
    /// Maximum declared root classes.
    pub max_root_classes: usize,
    /// Maximum retained classification issues.
    pub max_issues: usize,
}

/// Complete retirement request over one immutable graph snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetirementRequest {
    /// Generation being classified.
    pub generation: GenerationId,
    /// Root classes that the consumer says can retain work.
    pub declared_root_classes: Vec<RootClassId>,
    /// One immutable root/edge snapshot.
    pub graph: ReachabilitySnapshot,
    /// Caller-supplied attribution records.
    pub attributions: Vec<ArtifactAttribution>,
    /// Caller-supplied completeness observations.
    pub completeness: RetirementCompleteness,
}

/// Conservative generation disposition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetirementClassification {
    /// Complete facts show no old-generation pin.
    Retired,
    /// At least one explicit old-generation pin remains.
    Live,
    /// One or more required inventories are incomplete or missing.
    Incomplete,
    /// Contradictory attribution prevents a coherent decision.
    Unknown,
}

/// Stable bounded retirement explanation.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RetirementIssue {
    /// A declared root class has no complete observation.
    IncompleteRootClass {
        /// Incomplete class.
        class: RootClassId,
    },
    /// A supplied root or completeness fact uses an undeclared class.
    UndeclaredRootClass {
        /// Undeclared class.
        class: RootClassId,
    },
    /// Edge inventory is not asserted complete.
    EdgeInventoryIncomplete,
    /// Attribution inventory is not asserted complete.
    AttributionInventoryIncomplete,
    /// A reachable artifact has no complete attribution.
    MissingAttribution {
        /// Reachable artifact.
        artifact: ArtifactId,
    },
    /// One shared attribution has fewer than two distinct generations.
    MalformedSharedAttribution {
        /// Malformed artifact.
        artifact: ArtifactId,
    },
    /// Duplicate attribution records disagree.
    ContradictoryAttribution {
        /// Contradictory artifact.
        artifact: ArtifactId,
    },
    /// A generation-scoped root explicitly keeps the generation live.
    GenerationScopedRootLive {
        /// Root class.
        root_class: RootClassId,
        /// Root identity.
        root_id: RootId,
        /// Root target.
        target: ArtifactId,
    },
    /// A reachable artifact is exclusive to the generation.
    ExclusiveArtifactLive {
        /// Exclusive reachable artifact.
        artifact: ArtifactId,
        /// Stable selected pin path.
        path: PinPath,
    },
    /// More deterministic issues existed than the caller retained.
    IssuesTruncated {
        /// Number omitted.
        omitted: usize,
    },
}

/// Conservative classification and supporting pure trace.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetirementDecision {
    /// Conservative disposition.
    pub classification: RetirementClassification,
    /// Generation classified.
    pub generation: GenerationId,
    /// Underlying deterministic reachability trace.
    pub trace: ReachabilityTrace,
    /// Stable bounded paths that prove a `Live` classification.
    pub pin_paths: Vec<PinPath>,
    /// Sorted bounded supporting issues.
    pub issues: Vec<RetirementIssue>,
}

/// Retirement evaluation denial before a decision can be formed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RetirementError {
    /// One or more required bounds are zero.
    InvalidLimits,
    /// No root classes were declared.
    NoDeclaredRootClasses,
    /// Declared root-class input exceeds its bound.
    RootClassLimitExceeded {
        /// Caller maximum.
        maximum: usize,
        /// Supplied count.
        actual: usize,
    },
    /// Attribution input exceeds its bound.
    AttributionLimitExceeded {
        /// Caller maximum.
        maximum: usize,
        /// Supplied count.
        actual: usize,
    },
    /// Completeness facts refer to another snapshot.
    CompletenessSnapshotMismatch,
    /// One attribution refers to another snapshot.
    AttributionSnapshotMismatch {
        /// Ordered index in the supplied attribution list.
        index: usize,
    },
    /// Reachability failed under explicit bounds.
    Reachability(ReachabilityError),
    /// Checked issue arithmetic overflowed.
    IssueCountOverflow,
}

fn normalize_completeness(request: &RetirementRequest, issues: &mut Vec<RetirementIssue>) {
    let mut observed = BTreeMap::<RootClassId, BTreeSet<bool>>::new();
    for observation in &request.completeness.root_classes {
        observed
            .entry(observation.class.clone())
            .or_default()
            .insert(observation.complete);
    }
    let declared = request
        .declared_root_classes
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    for class in declared {
        let complete = observed
            .get(&class)
            .is_some_and(|values| values.len() == 1 && values.contains(&true));
        if !complete {
            issues.push(RetirementIssue::IncompleteRootClass { class });
        }
    }
    if !request.completeness.edge_inventory_complete {
        issues.push(RetirementIssue::EdgeInventoryIncomplete);
    }
    if !request.completeness.attribution_inventory_complete {
        issues.push(RetirementIssue::AttributionInventoryIncomplete);
    }
}

fn add_undeclared_root_classes(request: &RetirementRequest, issues: &mut Vec<RetirementIssue>) {
    let declared = request
        .declared_root_classes
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    for root in &request.graph.roots {
        if !declared.contains(&root.class) {
            issues.push(RetirementIssue::UndeclaredRootClass {
                class: root.class.clone(),
            });
        }
    }
    for observation in &request.completeness.root_classes {
        if !declared.contains(&observation.class) {
            issues.push(RetirementIssue::UndeclaredRootClass {
                class: observation.class.clone(),
            });
        }
    }
}

// r[impl artifact_binding.retirement.attribution]
fn normalize_attributions(
    request: &RetirementRequest,
    issues: &mut Vec<RetirementIssue>,
) -> BTreeMap<ArtifactId, BTreeSet<GenerationId>> {
    let mut observed = BTreeMap::<ArtifactId, Vec<BTreeSet<GenerationId>>>::new();
    for attribution in &request.attributions {
        let generations = match &attribution.ownership {
            GenerationAttribution::Exclusive(generation) => BTreeSet::from([generation.clone()]),
            GenerationAttribution::Shared(generations) => {
                let distinct = generations.iter().cloned().collect::<BTreeSet<_>>();
                if distinct.len() < SHARED_GENERATION_MINIMUM || distinct.len() != generations.len()
                {
                    issues.push(RetirementIssue::MalformedSharedAttribution {
                        artifact: attribution.artifact.clone(),
                    });
                    continue;
                }
                distinct
            }
        };
        observed
            .entry(attribution.artifact.clone())
            .or_default()
            .push(generations);
    }
    let mut normalized = BTreeMap::new();
    for (artifact, candidates) in observed {
        let distinct = candidates.into_iter().collect::<BTreeSet<_>>();
        if distinct.len() != 1 {
            issues.push(RetirementIssue::ContradictoryAttribution {
                artifact: artifact.clone(),
            });
            continue;
        }
        if let Some(generations) = distinct.into_iter().next() {
            normalized.insert(artifact, generations);
        }
    }
    normalized
}

fn add_generation_scoped_roots(request: &RetirementRequest, issues: &mut Vec<RetirementIssue>) {
    let mut roots = request.graph.roots.clone();
    roots.sort();
    roots.dedup();
    for root in roots {
        if root.generation_scope.as_ref() == Some(&request.generation) {
            issues.push(RetirementIssue::GenerationScopedRootLive {
                root_class: root.class,
                root_id: root.id,
                target: root.target,
            });
        }
    }
}

fn add_attribution_issues(
    request: &RetirementRequest,
    trace: &ReachabilityTrace,
    attributions: &BTreeMap<ArtifactId, BTreeSet<GenerationId>>,
    issues: &mut Vec<RetirementIssue>,
) {
    for artifact in &trace.reachable {
        let Some(generations) = attributions.get(artifact) else {
            if request.completeness.attribution_inventory_complete {
                issues.push(RetirementIssue::MissingAttribution {
                    artifact: artifact.clone(),
                });
            }
            continue;
        };
        if generations.is_empty() {
            issues.push(RetirementIssue::MissingAttribution {
                artifact: artifact.clone(),
            });
            continue;
        }
        if generations.len() == 1 && generations.contains(&request.generation) {
            if let Some(path) = trace.paths.iter().find(|path| path.target == *artifact) {
                issues.push(RetirementIssue::ExclusiveArtifactLive {
                    artifact: artifact.clone(),
                    path: path.clone(),
                });
            }
        }
    }
}

fn collect_pin_paths(issues: &[RetirementIssue], maximum: usize) -> Vec<PinPath> {
    let mut paths = Vec::new();
    for issue in issues {
        match issue {
            RetirementIssue::GenerationScopedRootLive {
                root_class,
                root_id,
                target,
            } => paths.push(PinPath {
                root_class: root_class.clone(),
                root_id: root_id.clone(),
                target: target.clone(),
                artifacts: vec![target.clone()],
            }),
            RetirementIssue::ExclusiveArtifactLive { path, .. } => paths.push(path.clone()),
            _ => {}
        }
    }
    paths.sort();
    paths.dedup();
    paths.truncate(maximum);
    paths
}

fn classify(issues: &[RetirementIssue]) -> RetirementClassification {
    let live = issues.iter().any(|issue| {
        matches!(
            issue,
            RetirementIssue::GenerationScopedRootLive { .. }
                | RetirementIssue::ExclusiveArtifactLive { .. }
        )
    });
    if live {
        return RetirementClassification::Live;
    }
    let malformed = issues
        .iter()
        .any(|issue| matches!(issue, RetirementIssue::MalformedSharedAttribution { .. }));
    if malformed {
        return RetirementClassification::Unknown;
    }
    if issues.is_empty() {
        RetirementClassification::Retired
    } else {
        RetirementClassification::Incomplete
    }
}

fn bound_issues(
    mut issues: Vec<RetirementIssue>,
    maximum: usize,
) -> Result<Vec<RetirementIssue>, RetirementError> {
    issues.sort();
    issues.dedup();
    if issues.len() <= maximum {
        return Ok(issues);
    }
    let retained = maximum
        .checked_sub(1)
        .ok_or(RetirementError::InvalidLimits)?;
    let omitted = issues
        .len()
        .checked_sub(retained)
        .ok_or(RetirementError::IssueCountOverflow)?;
    issues.truncate(retained);
    issues.push(RetirementIssue::IssuesTruncated { omitted });
    Ok(issues)
}

// r[impl artifact_binding.retirement.classification]
/// Conservatively classify one generation from complete caller observations.
///
/// A `Retired` result means only that the supplied complete snapshot contains
/// no generation-scoped root and no reachable artifact attributed exclusively
/// to the generation. It does not authorize deletion.
///
/// # Errors
///
/// Returns [`RetirementError`] when explicit input or traversal bounds fail.
pub fn classify_retirement(
    request: &RetirementRequest,
    limits: RetirementLimits,
) -> Result<RetirementDecision, RetirementError> {
    if limits.max_attributions == 0 || limits.max_root_classes == 0 || limits.max_issues == 0 {
        return Err(RetirementError::InvalidLimits);
    }
    if request.declared_root_classes.is_empty() {
        return Err(RetirementError::NoDeclaredRootClasses);
    }
    if request.declared_root_classes.len() > limits.max_root_classes {
        return Err(RetirementError::RootClassLimitExceeded {
            maximum: limits.max_root_classes,
            actual: request.declared_root_classes.len(),
        });
    }
    if request.attributions.len() > limits.max_attributions {
        return Err(RetirementError::AttributionLimitExceeded {
            maximum: limits.max_attributions,
            actual: request.attributions.len(),
        });
    }
    if request.completeness.snapshot != request.graph.id {
        return Err(RetirementError::CompletenessSnapshotMismatch);
    }
    for (index, attribution) in request.attributions.iter().enumerate() {
        if attribution.snapshot != request.graph.id {
            return Err(RetirementError::AttributionSnapshotMismatch { index });
        }
    }
    let trace = trace_reachability(&request.graph, limits.reachability)
        .map_err(RetirementError::Reachability)?;
    let mut issues = Vec::new();
    normalize_completeness(request, &mut issues);
    add_undeclared_root_classes(request, &mut issues);
    let attributions = normalize_attributions(request, &mut issues);
    add_generation_scoped_roots(request, &mut issues);
    add_attribution_issues(request, &trace, &attributions, &mut issues);
    let classification = classify(&issues);
    let pin_paths = collect_pin_paths(&issues, limits.max_issues);
    Ok(RetirementDecision {
        classification,
        generation: request.generation.clone(),
        trace,
        pin_paths,
        issues: bound_issues(issues, limits.max_issues)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GraphEdge, GraphRoot};

    const ID_LIMIT: usize = 64;
    const GRAPH_LIMIT: usize = 32;
    const DIAGNOSTIC_LIMIT: usize = 16;
    const RETIREMENT_LIMIT: usize = 32;
    const EXPECTED_PIN_PATH_NODES: usize = 2;

    fn artifact(value: &str) -> ArtifactId {
        ArtifactId::try_new(value, ID_LIMIT)
            .unwrap_or_else(|error| unreachable!("fixture: {error}"))
    }

    fn generation(value: &str) -> GenerationId {
        GenerationId::try_new(value, ID_LIMIT)
            .unwrap_or_else(|error| unreachable!("fixture: {error}"))
    }

    fn root_class() -> RootClassId {
        RootClassId::try_new("runtime-session", ID_LIMIT)
            .unwrap_or_else(|error| unreachable!("fixture: {error}"))
    }

    fn root(scope: Option<GenerationId>) -> GraphRoot {
        GraphRoot {
            class: root_class(),
            id: RootId::try_new("root:session", ID_LIMIT)
                .unwrap_or_else(|error| unreachable!("fixture: {error}")),
            target: artifact("artifact:entry"),
            generation_scope: scope,
        }
    }

    fn request() -> RetirementRequest {
        RetirementRequest {
            generation: generation("generation:old"),
            declared_root_classes: vec![root_class()],
            graph: ReachabilitySnapshot {
                id: SnapshotId::try_new("snapshot:retirement", ID_LIMIT)
                    .unwrap_or_else(|error| unreachable!("fixture: {error}")),
                roots: vec![root(None)],
                edges: vec![GraphEdge {
                    from: artifact("artifact:entry"),
                    to: artifact("artifact:shared"),
                }],
            },
            attributions: vec![
                ArtifactAttribution {
                    snapshot: SnapshotId::try_new("snapshot:retirement", ID_LIMIT)
                        .unwrap_or_else(|error| unreachable!("fixture: {error}")),
                    artifact: artifact("artifact:entry"),
                    ownership: GenerationAttribution::Exclusive(generation("generation:new")),
                },
                ArtifactAttribution {
                    snapshot: SnapshotId::try_new("snapshot:retirement", ID_LIMIT)
                        .unwrap_or_else(|error| unreachable!("fixture: {error}")),
                    artifact: artifact("artifact:shared"),
                    ownership: GenerationAttribution::Shared(vec![
                        generation("generation:old"),
                        generation("generation:new"),
                    ]),
                },
            ],
            completeness: RetirementCompleteness {
                snapshot: SnapshotId::try_new("snapshot:retirement", ID_LIMIT)
                    .unwrap_or_else(|error| unreachable!("fixture: {error}")),
                root_classes: vec![RootClassCompleteness {
                    class: root_class(),
                    complete: true,
                }],
                edge_inventory_complete: true,
                attribution_inventory_complete: true,
            },
        }
    }

    fn limits() -> RetirementLimits {
        RetirementLimits {
            reachability: ReachabilityLimits {
                max_roots: GRAPH_LIMIT,
                max_edges: GRAPH_LIMIT,
                max_nodes: GRAPH_LIMIT,
                max_path_nodes: GRAPH_LIMIT,
                max_diagnostics: DIAGNOSTIC_LIMIT,
            },
            max_attributions: RETIREMENT_LIMIT,
            max_root_classes: RETIREMENT_LIMIT,
            max_issues: RETIREMENT_LIMIT,
        }
    }

    // r[verify artifact_binding.retirement.classification]
    #[test]
    fn complete_shared_artifact_does_not_keep_old_generation_live() {
        let decision = classify_retirement(&request(), limits());
        assert_eq!(
            decision.as_ref().map(|decision| decision.classification),
            Ok(RetirementClassification::Retired)
        );
    }

    #[test]
    fn scoped_root_and_exclusive_artifact_keep_generation_live_with_paths() {
        let mut scoped = request();
        scoped.graph.roots = vec![root(Some(generation("generation:old")))];
        let scoped_decision = classify_retirement(&scoped, limits());
        assert_eq!(
            scoped_decision
                .as_ref()
                .map(|decision| decision.classification),
            Ok(RetirementClassification::Live)
        );

        let mut exclusive = request();
        let changed = exclusive
            .attributions
            .iter_mut()
            .find(|attribution| attribution.artifact.as_str() == "artifact:shared")
            .map(|attribution| {
                attribution.ownership =
                    GenerationAttribution::Exclusive(generation("generation:old"));
            });
        assert_eq!(changed, Some(()));
        let exclusive_decision = classify_retirement(&exclusive, limits());
        assert_eq!(
            exclusive_decision
                .as_ref()
                .map(|decision| decision.classification),
            Ok(RetirementClassification::Live)
        );
        assert!(
            exclusive_decision
                .as_ref()
                .is_ok_and(|decision| !decision.pin_paths.is_empty())
        );
        assert!(exclusive_decision.as_ref().is_ok_and(|decision| {
            decision.issues.iter().any(|issue| {
                matches!(
                    issue,
                    RetirementIssue::ExclusiveArtifactLive { path, .. }
                        if path.artifacts.len() == EXPECTED_PIN_PATH_NODES
                )
            })
        }));
    }

    #[test]
    fn incomplete_and_contradictory_inputs_never_classify_retired() {
        let mut incomplete = request();
        let changed = incomplete
            .completeness
            .root_classes
            .first_mut()
            .map(|observation| observation.complete = false);
        assert_eq!(changed, Some(()));
        assert_eq!(
            classify_retirement(&incomplete, limits())
                .as_ref()
                .map(|decision| decision.classification),
            Ok(RetirementClassification::Incomplete)
        );

        let mut missing = request();
        missing.attributions.clear();
        assert_eq!(
            classify_retirement(&missing, limits())
                .as_ref()
                .map(|decision| decision.classification),
            Ok(RetirementClassification::Incomplete)
        );

        let mut undeclared = request();
        let undeclared_class = RootClassId::try_new("undeclared-runtime-root", ID_LIMIT)
            .unwrap_or_else(|error| unreachable!("fixture: {error}"));
        if let Some(root) = undeclared.graph.roots.first_mut() {
            root.class = undeclared_class;
        }
        assert_eq!(
            classify_retirement(&undeclared, limits())
                .as_ref()
                .map(|decision| decision.classification),
            Ok(RetirementClassification::Incomplete)
        );

        let mut malformed_shared = request();
        let changed = malformed_shared
            .attributions
            .iter_mut()
            .find(|attribution| attribution.artifact.as_str() == "artifact:shared")
            .map(|attribution| {
                attribution.ownership = GenerationAttribution::Shared(vec![
                    generation("generation:old"),
                    generation("generation:old"),
                ]);
            });
        assert_eq!(changed, Some(()));
        assert_eq!(
            classify_retirement(&malformed_shared, limits())
                .as_ref()
                .map(|decision| decision.classification),
            Ok(RetirementClassification::Unknown)
        );

        let mut contradictory = request();
        contradictory.attributions.push(ArtifactAttribution {
            snapshot: SnapshotId::try_new("snapshot:retirement", ID_LIMIT)
                .unwrap_or_else(|error| unreachable!("fixture: {error}")),
            artifact: artifact("artifact:entry"),
            ownership: GenerationAttribution::Exclusive(generation("generation:old")),
        });
        assert_eq!(
            classify_retirement(&contradictory, limits())
                .as_ref()
                .map(|decision| decision.classification),
            Ok(RetirementClassification::Incomplete)
        );
    }

    #[test]
    fn retirement_is_invariant_under_root_edge_and_attribution_order() {
        let forward = request();
        let mut reversed = forward.clone();
        reversed.graph.roots.reverse();
        reversed.graph.edges.reverse();
        reversed.attributions.reverse();
        reversed.completeness.root_classes.reverse();
        assert_eq!(
            classify_retirement(&forward, limits()),
            classify_retirement(&reversed, limits())
        );
    }

    #[test]
    fn mixed_snapshot_facts_fail_closed() {
        let mut mixed_completeness = request();
        mixed_completeness.completeness.snapshot = SnapshotId::try_new("snapshot:mixed", ID_LIMIT)
            .unwrap_or_else(|error| unreachable!("fixture: {error}"));
        assert_eq!(
            classify_retirement(&mixed_completeness, limits()),
            Err(RetirementError::CompletenessSnapshotMismatch)
        );

        let mut mixed_attribution = request();
        if let Some(attribution) = mixed_attribution.attributions.first_mut() {
            attribution.snapshot = SnapshotId::try_new("snapshot:mixed", ID_LIMIT)
                .unwrap_or_else(|error| unreachable!("fixture: {error}"));
        }
        assert_eq!(
            classify_retirement(&mixed_attribution, limits()),
            Err(RetirementError::AttributionSnapshotMismatch { index: 0 })
        );
    }

    #[test]
    fn retirement_bounds_fail_before_classification() {
        let mut no_root_classes = request();
        no_root_classes.declared_root_classes.clear();
        assert_eq!(
            classify_retirement(&no_root_classes, limits()),
            Err(RetirementError::NoDeclaredRootClasses)
        );

        let mut invalid = limits();
        invalid.max_attributions = 0;
        assert_eq!(
            classify_retirement(&request(), invalid),
            Err(RetirementError::InvalidLimits)
        );

        let mut bounded = limits();
        bounded.max_attributions = 1;
        assert!(matches!(
            classify_retirement(&request(), bounded),
            Err(RetirementError::AttributionLimitExceeded { .. })
        ));
    }
}
