## Context

The candidate sits between two existing boundaries:

- `cap-root` performs pure lexical admission and deliberately does not own filesystem publication or rollback.
- `artifact-binding-core` may validate a pure compare-and-swap plan but deliberately leaves persistence and atomic mutation to consumers.

A publication mechanism therefore must not be added to either package. It may deserve an independent repository only if a narrow mechanical shell can serve materially different products without absorbing their policy.

The initial source inventory is:

- Mantle `src/remote_attempt_log_store.rs`: bounded private staging, file synchronization, Linux `RENAME_NOREPLACE`, parent synchronization, cleanup, and immutable-content conflict handling.
- Mantle `src/linux_rename.rs`: centralized Linux no-replace syscall wrapper and explicit unsupported-platform result.
- OnixOS `cli/src/capability_fs.rs`: capability-rooted replace/refuse publication, permission selection, file synchronization, and BLAKE3-observed copy behavior; parent synchronization is currently an explicit durability non-claim.
- Nickel Export `crates/nickel-export/src/lib.rs`: generation-pointer publication plus recoverable sequential two-artifact transactions.
- Valence `crates/valence-cli/src/main.rs`: staged single-file replacement and bundle-specific multi-output rollback behavior.

Nickel Export and Valence are corroborating implementations, not initial adopters. Their stronger transaction and evidence semantics stay outside the first pilot.

## Decisions

### Decision: Artifact owns incubation evidence, not the candidate package

**Choice:** Keep feasibility artifacts and disposable pilots in this change without adding a Cargo workspace member. If the pilots pass, create a separate product-neutral repository with its own Cairn lifecycle and release identity.

**Rationale:** Artifact is the stack's current mechanism-extraction portfolio and can preserve the decision evidence, but its reusable packages are pure `no_std + alloc` cores. Durable publication necessarily includes a std filesystem shell and must not weaken Artifact workspace invariants.

**Rejected alternatives:**

- Add an `artifact-publication` crate immediately: this assumes the result before pilots and mixes a std mutation shell into a pure-core workspace.
- Put the shell in `cap-root`: lexical admission does not own I/O, atomicity, or durability.
- Put the shell in `artifact-binding-core`: binding transition validity is not filesystem mutation success.
- Keep expanding one product helper and ask other products to copy it: this preserves duplication and does not prove an independent API.

### Decision: Limit version one to one regular file

**Choice:** The candidate publishes one newly staged regular file to one validated leaf under the same caller-opened parent directory.

**Rationale:** Same-parent publication preserves same-filesystem rename semantics and is the intersection exercised by Mantle immutable attempt-log objects and OnixOS atomic file writes. Directory trees, multi-file commits, and generation pointers have materially stronger preparation and recovery requirements.

The candidate does not publish a symlink payload, recursively synchronize a directory tree, or claim atomicity across multiple names. Replace may replace an existing non-directory directory entry according to the platform rename primitive; destination type policy remains consumer-owned.

### Decision: Split a pure phase core from an imperative filesystem shell

**Choice:** A `no_std + alloc` core validates explicit requests and classifies finite observations over named phases such as planned, staged, payload-synchronized, renamed, and parent-synchronized. A std shell performs capability-relative file creation, bounded writes, permission application, synchronization, rename, and cleanup.

**Rationale:** The core can prove that reported observations form a valid state transition and can classify partial commits deterministically without files, clocks, processes, random sources, or platform calls. The shell stays thin and returns exact observations rather than embedding product policy.

The core does not claim that supplied observations are true. The shell does not construct product receipts or interpret payload identity.

### Decision: Require caller-owned authority and explicit bounds

**Choice:** The shell accepts a caller-opened destination-directory capability, one already-admitted leaf name, an explicit byte limit, an explicit permission plan, a replacement mode, a durability requirement, an injected staging-name source, and a named collision-attempt limit.

**Rationale:** Opening an ambient root, validating product paths, selecting permissions implicitly, deriving staging names from clocks or process ids, or retrying without a bound would import authority and hidden policy. The consumer remains responsible for opening and authorizing the parent and for proving the leaf belongs beneath its product root.

A pilot may adapt Mantle's current ambient path only by opening the already-authorized parent at the product shell boundary before invoking the candidate API.

### Decision: Make replacement semantics exact

**Choice:** Replacement mode is explicit:

- `replace` uses one same-filesystem atomic rename that may replace an existing non-directory entry.
- `no-replace` succeeds only through a race-free platform primitive such as Linux/Android `RENAME_NOREPLACE`.

If exact no-replace is unavailable, the shell returns unsupported before rename. It never implements no-replace as destination inspection followed by ordinary rename.

**Rationale:** Check-then-rename has a time-of-check/time-of-use race and is not lossless with Mantle's current safety property. An explicit unsupported result is safer than a compatibility fallback that changes semantics.

### Decision: Separate visibility from durability

**Choice:** Every request selects either visibility-only or durability-required publication. The shell synchronizes the complete staged file after permissions are applied and before rename. Durability-required success additionally requires successful parent-directory synchronization after rename.

**Rationale:** Atomic visibility and crash durability are different observations. OnixOS currently provides the first but explicitly cannot claim the second through its existing `cap-std` directory descriptor. The pilot must either obtain a genuinely synchronizable capability-relative parent handle or report durability unsupported; a no-op cannot satisfy the durability-required path.

The result vocabulary distinguishes at least:

- not committed;
- destination existed under no-replace;
- committed and visibility-only complete;
- committed and parent synchronized;
- committed but parent synchronization failed;
- primary failure plus cleanup failure.

A rename-success/parent-sync-failure result is a partial commit. The shell must not attempt an implicit rollback or return an error that hides destination visibility.

### Decision: Leave content and idempotency semantics with consumers

**Choice:** The candidate may report bounded byte counts and mechanical observations but does not define canonical bytes, a universal digest, immutable-object identity, or equality-on-existing behavior.

**Rationale:** Mantle interprets an existing identical immutable object as idempotent success, while OnixOS refusal treats destination existence as denial. The shared no-replace primitive can report destination existence; each adapter must apply its own content comparison and product result.

BLAKE3 remains the default for any future repository-owned fixture or source identity, but product content identities remain product-owned.

### Decision: Treat cleanup and crash recovery conservatively

**Choice:** Before rename, the shell attempts bounded cleanup of its private stage and reports cleanup failure separately. After rename, it never describes cleanup as rollback. Version one defines no persistent transaction marker, stale-stage scanner, automatic recovery, or multi-process lock.

**Rationale:** A generic cleanup scanner could delete another operation's stage, and transaction recovery requires product-owned identities, locks, and lifecycle policy. Nickel Export's marker protocol therefore remains outside this boundary.

### Decision: Use two materially different disposable pilots

**Choice:** Pilot the same candidate API against:

1. Mantle immutable remote-attempt-log object publication, preserving bounded bytes, private staging, race-free no-replace, parent synchronization, and consumer-owned identical-content handling.
2. OnixOS `CapabilityRoot::atomic_write_with_policy`, preserving capability confinement, replace/refuse behavior, explicit modes, and current visibility-versus-durability non-claims.

The pilots use exact recorded source revisions and a shared fault corpus but add no permanent dependency, generated lock update, or release claim to either consumer.

**Rationale:** Mantle stresses immutable no-replace and true parent synchronization; OnixOS stresses capability-relative replace/refuse and permission mapping. If one API cannot preserve both, the candidate is not ready.

### Decision: Promotion requires more than passing local tests

**Choice:** Independent repository promotion requires all of:

- two passing lossless pilots over the same API and fixture corpus;
- a source/provenance and copyright authorization map;
- no stack-product or evidence dependencies;
- an independent useful release surface consisting of the core, std shell, platform capability matrix, and conformance fixtures;
- explicit unsupported-platform behavior and rollback guidance;
- exact immutable candidate revision for later adoption;
- separate Mantle and OnixOS Cairns before permanent consumption.

**Rationale:** Similar helpers, one pilot, sibling-path builds, or a clean local test suite do not establish shared ownership or release value.

## Pilot Acceptance Matrix

| Property | Mantle pilot | OnixOS pilot | Shared requirement |
|---|---|---|---|
| Destination authority | Mantle opens its authorized log-object parent | Existing `CapabilityRoot` supplies the admitted parent | Candidate opens no ambient root |
| Replace | Manifest-like mutable pointer is out of initial pilot | Existing replace mode maps directly | Atomic same-parent rename |
| No-replace | Immutable object creation | Existing refuse mode | Race-free primitive or unsupported |
| Permissions | Private staged file and product final policy | Exact/existing/create mode selection | Explicit permission input; no ambient umask claim |
| Payload sync | Required before immutable object publication | Existing `sync_all` | Required before rename |
| Parent sync | Existing real directory sync | Currently unavailable through existing descriptor | Durable success only when actually performed |
| Existing destination | Consumer compares bounded bytes for idempotency | Consumer reports refusal | Shared shell reports destination exists only |
| Partial commit | Must expose post-rename sync failure | Must preserve destination visibility | Typed committed-but-durability-unknown outcome |

## Risks / Trade-offs

- **Core too trivial:** If phase classification adds no reusable safety beyond an enum, promotion should be blocked rather than manufacturing a core package.
- **Shell too platform-specific:** Race-free no-replace and capability-relative directory synchronization may require explicit platform modules. Unsupported targets are acceptable; silent semantic fallback is not.
- **Capability mismatch:** Mantle currently uses ambient `Path` in the selected shell. Its pilot must prove adaptation to a caller-opened parent without moving product path authorization into the candidate.
- **Partial-commit ergonomics:** Ordinary `Result<(), E>` encourages callers to discard whether rename happened. The candidate must make committed state impossible to ignore in error classification.
- **License mismatch:** Reviewed implementations may have licenses incompatible with a proposed repository. Use clean-room specification or explicit source grants; do not copy expression by default.
- **Overclaim through naming:** “Atomic” applies only to one rename visibility event. “Durable” applies only when the requested synchronization observations completed and does not prove hardware persistence or power-loss behavior.
- **Pilot contamination:** Disposable adapters must not become sibling-path product dependencies or release evidence.
