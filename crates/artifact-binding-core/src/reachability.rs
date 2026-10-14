//! Bounded deterministic cycle-safe graph reachability.

use alloc::collections::{BTreeMap, BTreeSet, VecDeque};
use alloc::vec;
use alloc::vec::Vec;

use crate::{ArtifactId, GenerationId, RootClassId, RootId, SnapshotId};

/// One caller-declared graph root.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GraphRoot {
    /// Declared root class.
    pub class: RootClassId,
    /// Stable root identity.
    pub id: RootId,
    /// Exact artifact reached directly by the root.
    pub target: ArtifactId,
    /// Optional generation scope owned by the caller.
    pub generation_scope: Option<GenerationId>,
}

/// One directed caller-supplied artifact edge.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GraphEdge {
    /// Source artifact.
    pub from: ArtifactId,
    /// Target artifact.
    pub to: ArtifactId,
}

/// One immutable caller-supplied graph snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReachabilitySnapshot {
    /// Exact snapshot identity.
    pub id: SnapshotId,
    /// Declared roots.
    pub roots: Vec<GraphRoot>,
    /// Directed edges. Cycles are allowed.
    pub edges: Vec<GraphEdge>,
}

/// Explicit traversal and diagnostic bounds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReachabilityLimits {
    /// Maximum supplied root records, before normalization.
    pub max_roots: usize,
    /// Maximum supplied edge records, before normalization.
    pub max_edges: usize,
    /// Maximum distinct reachable artifacts.
    pub max_nodes: usize,
    /// Maximum artifact nodes in one selected path.
    pub max_path_nodes: usize,
    /// Maximum diagnostics retained, including truncation notice.
    pub max_diagnostics: usize,
}

/// Stable finite traversal diagnostic.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ReachabilityDiagnostic {
    /// Exact duplicate roots were normalized.
    DuplicateRoot {
        /// Duplicate root identity.
        root_id: RootId,
        /// Number of supplied identical records.
        occurrences: usize,
    },
    /// Exact duplicate edges were normalized.
    DuplicateEdge {
        /// Duplicate source.
        from: ArtifactId,
        /// Duplicate target.
        to: ArtifactId,
        /// Number of supplied identical records.
        occurrences: usize,
    },
    /// Traversal observed a cycle-closing edge.
    CycleEdge {
        /// Source in the selected traversal path.
        from: ArtifactId,
        /// Earlier target in the same path.
        to: ArtifactId,
    },
    /// Additional deterministic diagnostics were omitted at the named bound.
    DiagnosticsTruncated {
        /// Number of omitted diagnostics.
        omitted: usize,
    },
}

/// Stable explanation from one root to one reachable target.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PinPath {
    /// Selected root class.
    pub root_class: RootClassId,
    /// Selected root identity.
    pub root_id: RootId,
    /// Target explained by this path.
    pub target: ArtifactId,
    /// Ordered root-target through target path.
    pub artifacts: Vec<ArtifactId>,
}

/// Deterministic reachability output.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReachabilityTrace {
    /// Exact input snapshot identity.
    pub snapshot: SnapshotId,
    /// Sorted distinct reachable artifacts.
    pub reachable: Vec<ArtifactId>,
    /// Stable selected explanation for each reachable artifact.
    pub paths: Vec<PinPath>,
    /// Sorted bounded normalization and cycle diagnostics.
    pub diagnostics: Vec<ReachabilityDiagnostic>,
}

/// Traversal denial before any partial output is accepted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReachabilityError {
    /// One or more required bounds are zero.
    InvalidLimits,
    /// Supplied roots exceed the caller-selected bound.
    RootLimitExceeded {
        /// Caller maximum.
        maximum: usize,
        /// Supplied count.
        actual: usize,
    },
    /// Supplied edges exceed the caller-selected bound.
    EdgeLimitExceeded {
        /// Caller maximum.
        maximum: usize,
        /// Supplied count.
        actual: usize,
    },
    /// Distinct reachable artifacts exceed the caller-selected bound.
    NodeLimitExceeded {
        /// Caller maximum.
        maximum: usize,
    },
    /// A selected path exceeds the caller-selected node bound.
    PathLimitExceeded {
        /// Caller maximum.
        maximum: usize,
    },
    /// Checked diagnostic arithmetic overflowed.
    DiagnosticCountOverflow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PendingPath {
    root_class: RootClassId,
    root_id: RootId,
    target: ArtifactId,
    artifacts: Vec<ArtifactId>,
}

fn validate_limits(limits: ReachabilityLimits) -> Result<(), ReachabilityError> {
    if limits.max_roots == 0
        || limits.max_edges == 0
        || limits.max_nodes == 0
        || limits.max_path_nodes == 0
        || limits.max_diagnostics == 0
    {
        return Err(ReachabilityError::InvalidLimits);
    }
    Ok(())
}

fn normalize_roots(
    roots: &[GraphRoot],
    diagnostics: &mut Vec<ReachabilityDiagnostic>,
) -> Vec<GraphRoot> {
    let mut counts = BTreeMap::<GraphRoot, usize>::new();
    for root in roots {
        let count = counts.entry(root.clone()).or_default();
        *count = count.saturating_add(1);
    }
    let mut normalized = Vec::with_capacity(counts.len());
    for (root, occurrences) in counts {
        if occurrences > 1 {
            diagnostics.push(ReachabilityDiagnostic::DuplicateRoot {
                root_id: root.id.clone(),
                occurrences,
            });
        }
        normalized.push(root);
    }
    normalized
}

fn normalize_edges(
    edges: &[GraphEdge],
    diagnostics: &mut Vec<ReachabilityDiagnostic>,
) -> BTreeMap<ArtifactId, BTreeSet<ArtifactId>> {
    let mut counts = BTreeMap::<GraphEdge, usize>::new();
    for edge in edges {
        let count = counts.entry(edge.clone()).or_default();
        *count = count.saturating_add(1);
    }
    let mut adjacency = BTreeMap::<ArtifactId, BTreeSet<ArtifactId>>::new();
    for (edge, occurrences) in counts {
        if occurrences > 1 {
            diagnostics.push(ReachabilityDiagnostic::DuplicateEdge {
                from: edge.from.clone(),
                to: edge.to.clone(),
                occurrences,
            });
        }
        adjacency.entry(edge.from).or_default().insert(edge.to);
    }
    adjacency
}

fn path_exists(
    start: &ArtifactId,
    sought: &ArtifactId,
    adjacency: &BTreeMap<ArtifactId, BTreeSet<ArtifactId>>,
    reachable: &BTreeSet<ArtifactId>,
) -> bool {
    let mut queue = VecDeque::from([start.clone()]);
    let mut visited = BTreeSet::new();
    while let Some(candidate) = queue.pop_front() {
        if &candidate == sought {
            return true;
        }
        if !visited.insert(candidate.clone()) {
            continue;
        }
        if let Some(neighbors) = adjacency.get(&candidate) {
            for neighbor in neighbors {
                if reachable.contains(neighbor) && !visited.contains(neighbor) {
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }
    false
}

fn cycle_edges(
    adjacency: &BTreeMap<ArtifactId, BTreeSet<ArtifactId>>,
    reachable: &BTreeSet<ArtifactId>,
) -> Vec<ReachabilityDiagnostic> {
    let mut diagnostics = Vec::new();
    for (from, neighbors) in adjacency {
        if !reachable.contains(from) {
            continue;
        }
        for to in neighbors {
            if reachable.contains(to) && path_exists(to, from, adjacency, reachable) {
                diagnostics.push(ReachabilityDiagnostic::CycleEdge {
                    from: from.clone(),
                    to: to.clone(),
                });
            }
        }
    }
    diagnostics
}

fn bound_diagnostics(
    mut diagnostics: Vec<ReachabilityDiagnostic>,
    maximum: usize,
) -> Result<Vec<ReachabilityDiagnostic>, ReachabilityError> {
    diagnostics.sort();
    diagnostics.dedup();
    if diagnostics.len() <= maximum {
        return Ok(diagnostics);
    }
    let retained = maximum
        .checked_sub(1)
        .ok_or(ReachabilityError::InvalidLimits)?;
    let omitted = diagnostics
        .len()
        .checked_sub(retained)
        .ok_or(ReachabilityError::DiagnosticCountOverflow)?;
    diagnostics.truncate(retained);
    diagnostics.push(ReachabilityDiagnostic::DiagnosticsTruncated { omitted });
    Ok(diagnostics)
}

// r[impl artifact_binding.reachability.trace]
// r[impl artifact_binding.diagnostics]
/// Trace one immutable finite root/edge snapshot with deterministic ordering.
///
/// # Errors
///
/// Returns [`ReachabilityError`] when explicit limits are invalid or exceeded.
pub fn trace_reachability(
    snapshot: &ReachabilitySnapshot,
    limits: ReachabilityLimits,
) -> Result<ReachabilityTrace, ReachabilityError> {
    validate_limits(limits)?;
    if snapshot.roots.len() > limits.max_roots {
        return Err(ReachabilityError::RootLimitExceeded {
            maximum: limits.max_roots,
            actual: snapshot.roots.len(),
        });
    }
    if snapshot.edges.len() > limits.max_edges {
        return Err(ReachabilityError::EdgeLimitExceeded {
            maximum: limits.max_edges,
            actual: snapshot.edges.len(),
        });
    }

    let mut diagnostics = Vec::new();
    let roots = normalize_roots(&snapshot.roots, &mut diagnostics);
    let adjacency = normalize_edges(&snapshot.edges, &mut diagnostics);
    let mut queue = VecDeque::new();
    for root in roots {
        queue.push_back(PendingPath {
            root_class: root.class,
            root_id: root.id,
            target: root.target.clone(),
            artifacts: vec![root.target],
        });
    }

    let mut visited = BTreeSet::new();
    let mut selected_paths = BTreeMap::<ArtifactId, PinPath>::new();
    while let Some(pending) = queue.pop_front() {
        if visited.contains(&pending.target) {
            continue;
        }
        if pending.artifacts.len() > limits.max_path_nodes {
            return Err(ReachabilityError::PathLimitExceeded {
                maximum: limits.max_path_nodes,
            });
        }
        visited.insert(pending.target.clone());
        if visited.len() > limits.max_nodes {
            return Err(ReachabilityError::NodeLimitExceeded {
                maximum: limits.max_nodes,
            });
        }
        selected_paths.insert(
            pending.target.clone(),
            PinPath {
                root_class: pending.root_class.clone(),
                root_id: pending.root_id.clone(),
                target: pending.target.clone(),
                artifacts: pending.artifacts.clone(),
            },
        );
        if let Some(neighbors) = adjacency.get(&pending.target) {
            for neighbor in neighbors {
                if pending.artifacts.contains(neighbor) {
                    continue;
                }
                if visited.contains(neighbor) {
                    continue;
                }
                let mut artifacts = pending.artifacts.clone();
                artifacts.push(neighbor.clone());
                queue.push_back(PendingPath {
                    root_class: pending.root_class.clone(),
                    root_id: pending.root_id.clone(),
                    target: neighbor.clone(),
                    artifacts,
                });
            }
        }
    }

    diagnostics.extend(cycle_edges(&adjacency, &visited));
    let paths = selected_paths.into_values().collect::<Vec<_>>();
    let reachable = visited.into_iter().collect::<Vec<_>>();
    Ok(ReachabilityTrace {
        snapshot: snapshot.id.clone(),
        reachable,
        paths,
        diagnostics: bound_diagnostics(diagnostics, limits.max_diagnostics)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const ID_LIMIT: usize = 64;
    const GRAPH_LIMIT: usize = 32;
    const DIAGNOSTIC_LIMIT: usize = 16;
    const TIGHT_BOUND: usize = 1;
    const EXPECTED_OMITTED_DIAGNOSTICS: usize = 3;

    fn artifact(value: &str) -> ArtifactId {
        ArtifactId::try_new(value, ID_LIMIT)
            .unwrap_or_else(|error| unreachable!("fixture: {error}"))
    }

    fn root(value: &str, target: &str) -> GraphRoot {
        GraphRoot {
            class: RootClassId::try_new("session", ID_LIMIT)
                .unwrap_or_else(|error| unreachable!("fixture: {error}")),
            id: RootId::try_new(value, ID_LIMIT)
                .unwrap_or_else(|error| unreachable!("fixture: {error}")),
            target: artifact(target),
            generation_scope: None,
        }
    }

    fn edge(from: &str, to: &str) -> GraphEdge {
        GraphEdge {
            from: artifact(from),
            to: artifact(to),
        }
    }

    fn snapshot(roots: Vec<GraphRoot>, edges: Vec<GraphEdge>) -> ReachabilitySnapshot {
        ReachabilitySnapshot {
            id: SnapshotId::try_new("snapshot:graph", ID_LIMIT)
                .unwrap_or_else(|error| unreachable!("fixture: {error}")),
            roots,
            edges,
        }
    }

    fn limits() -> ReachabilityLimits {
        ReachabilityLimits {
            max_roots: GRAPH_LIMIT,
            max_edges: GRAPH_LIMIT,
            max_nodes: GRAPH_LIMIT,
            max_path_nodes: GRAPH_LIMIT,
            max_diagnostics: DIAGNOSTIC_LIMIT,
        }
    }

    // r[verify artifact_binding.reachability.trace]
    #[test]
    fn cycles_duplicates_and_input_permutations_are_deterministic() {
        let roots = vec![root("root:a", "artifact:a"), root("root:a", "artifact:a")];
        let edges = vec![
            edge("artifact:a", "artifact:b"),
            edge("artifact:b", "artifact:c"),
            edge("artifact:c", "artifact:a"),
            edge("artifact:a", "artifact:b"),
        ];
        let forward = trace_reachability(&snapshot(roots.clone(), edges.clone()), limits());
        let mut reversed_roots = roots;
        reversed_roots.reverse();
        let mut reversed_edges = edges;
        reversed_edges.reverse();
        let reversed = trace_reachability(&snapshot(reversed_roots, reversed_edges), limits());
        assert_eq!(forward, reversed);
        let trace = forward.unwrap_or_else(|error| unreachable!("fixture: {error:?}"));
        assert_eq!(
            trace
                .reachable
                .iter()
                .map(ArtifactId::as_str)
                .collect::<Vec<_>>(),
            vec!["artifact:a", "artifact:b", "artifact:c"]
        );
        assert!(
            trace
                .diagnostics
                .iter()
                .any(|diagnostic| matches!(diagnostic, ReachabilityDiagnostic::CycleEdge { .. }))
        );
        let path = trace
            .paths
            .iter()
            .find(|path| path.target.as_str() == "artifact:c");
        assert_eq!(
            path.map(|path| path
                .artifacts
                .iter()
                .map(ArtifactId::as_str)
                .collect::<Vec<_>>()),
            Some(vec!["artifact:a", "artifact:b", "artifact:c"])
        );
    }

    #[test]
    fn bounded_rotation_exploration_preserves_canonical_trace() {
        let roots = vec![root("root:a", "artifact:a"), root("root:b", "artifact:b")];
        let edges = vec![
            edge("artifact:a", "artifact:c"),
            edge("artifact:b", "artifact:c"),
            edge("artifact:c", "artifact:d"),
            edge("artifact:d", "artifact:a"),
        ];
        let mut exact_limits = limits();
        exact_limits.max_roots = roots.len();
        exact_limits.max_edges = edges.len();
        exact_limits.max_nodes = edges.len();
        exact_limits.max_path_nodes = edges.len();
        let expected = trace_reachability(&snapshot(roots.clone(), edges.clone()), exact_limits);
        for root_rotation in 0..roots.len() {
            for edge_rotation in 0..edges.len() {
                let mut rotated_roots = roots.clone();
                let mut rotated_edges = edges.clone();
                rotated_roots.rotate_left(root_rotation);
                rotated_edges.rotate_left(edge_rotation);
                assert_eq!(
                    trace_reachability(&snapshot(rotated_roots, rotated_edges), exact_limits),
                    expected
                );
            }
        }
    }

    #[test]
    fn explicit_root_edge_node_and_path_bounds_fail_closed() {
        let graph = snapshot(
            vec![root("root:a", "artifact:a")],
            vec![edge("artifact:a", "artifact:b")],
        );
        let mut invalid = limits();
        invalid.max_roots = 0;
        assert_eq!(
            trace_reachability(&graph, invalid),
            Err(ReachabilityError::InvalidLimits)
        );

        let root_heavy = snapshot(
            vec![root("root:a", "artifact:a"), root("root:b", "artifact:b")],
            Vec::new(),
        );
        let mut root_bound = limits();
        root_bound.max_roots = TIGHT_BOUND;
        assert!(matches!(
            trace_reachability(&root_heavy, root_bound),
            Err(ReachabilityError::RootLimitExceeded { .. })
        ));

        let edge_heavy = snapshot(
            vec![root("root:a", "artifact:a")],
            vec![
                edge("artifact:a", "artifact:b"),
                edge("artifact:b", "artifact:c"),
            ],
        );
        let mut edge_bound = limits();
        edge_bound.max_edges = TIGHT_BOUND;
        assert!(matches!(
            trace_reachability(&edge_heavy, edge_bound),
            Err(ReachabilityError::EdgeLimitExceeded { .. })
        ));

        let mut node_bound = limits();
        node_bound.max_nodes = TIGHT_BOUND;
        assert_eq!(
            trace_reachability(&graph, node_bound),
            Err(ReachabilityError::NodeLimitExceeded {
                maximum: TIGHT_BOUND,
            })
        );

        let mut path_bound = limits();
        path_bound.max_path_nodes = TIGHT_BOUND;
        assert_eq!(
            trace_reachability(&graph, path_bound),
            Err(ReachabilityError::PathLimitExceeded {
                maximum: TIGHT_BOUND,
            })
        );
    }

    #[test]
    fn diagnostic_output_is_explicitly_bounded() {
        let duplicate_root = root("root:a", "artifact:a");
        let duplicate_edge = edge("artifact:a", "artifact:a");
        let graph = snapshot(
            vec![duplicate_root.clone(), duplicate_root],
            vec![duplicate_edge.clone(), duplicate_edge],
        );
        let mut bounded = limits();
        bounded.max_diagnostics = TIGHT_BOUND;
        let trace = trace_reachability(&graph, bounded);
        assert!(trace.as_ref().is_ok_and(|trace| {
            trace.diagnostics
                == vec![ReachabilityDiagnostic::DiagnosticsTruncated {
                    omitted: EXPECTED_OMITTED_DIAGNOSTICS,
                }]
        }));
    }
}
