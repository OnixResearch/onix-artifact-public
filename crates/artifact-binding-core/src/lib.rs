#![no_std]
#![forbid(unsafe_code)]
//! Product-neutral pure mechanics for versioned artifact bindings.
//!
//! The crate checks structural successor transitions, resolves exact targets
//! from one caller-loaded snapshot, traces bounded reachability, and classifies
//! retirement conservatively. It performs no I/O and owns no storage,
//! publication, atomic mutation, compatibility truth, root discovery,
//! authorization, evidence, retention, or deletion policy.

extern crate alloc;

mod id;
mod reachability;
mod resolution;
mod retirement;
mod transition;

pub use id::{
    ArtifactId, BindingKey, BindingRevision, ContractId, GenerationId, IdentifierError,
    ObservationId, RootClassId, RootId, SnapshotId,
};
pub use reachability::{
    GraphEdge, GraphRoot, PinPath, ReachabilityDiagnostic, ReachabilityError, ReachabilityLimits,
    ReachabilitySnapshot, ReachabilityTrace, trace_reachability,
};
pub use resolution::{
    BindingSnapshot, LateBoundRequest, PinnedRequest, Resolution, ResolutionError,
    ResolutionLimits, ResolutionRequest, resolve,
};
pub use retirement::{
    ArtifactAttribution, GenerationAttribution, RetirementClassification, RetirementCompleteness,
    RetirementDecision, RetirementError, RetirementIssue, RetirementLimits, RetirementRequest,
    RootClassCompleteness, classify_retirement,
};
pub use transition::{
    BindingRecord, CompatibilityObservation, ExpectedBinding, ProposedBinding,
    TargetReachabilityObservation, TransitionError, TransitionPlan, TransitionPolicy,
    TransitionRequest, plan_transition,
};
