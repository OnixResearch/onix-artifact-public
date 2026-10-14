# Durable File Publication Extraction Specification

## ADDED Requirements

### Requirement: Extraction remains an evidence-gated decision

r[durable_file_publication.extraction.boundary] This change MUST remain an incubation and pilot package, MUST NOT add a publication crate to the Artifact workspace, and MUST NOT create or permanently adopt an independent repository until two materially different consumer pilots prove one lossless boundary.

#### Scenario: Similar implementations are discovered
- GIVEN Mantle, OnixOS, Nickel Export, and Valence contain staging, synchronization, and rename helpers
- WHEN extraction readiness is evaluated
- THEN implementation similarity alone MUST NOT authorize a package, repository, consumer dependency, or shared correctness claim.

#### Scenario: Both selected pilots preserve one boundary
- GIVEN the Mantle and OnixOS pilots pass the same conformance corpus and preserve every selected consumer guarantee without product fields in the shared API
- WHEN promotion is reviewed
- THEN a separate repository MAY be proposed subject to provenance, licensing, dependency, release-value, and consumer-local lifecycle gates.

### Requirement: The first candidate scope is one regular file

r[durable_file_publication.publication.scope] The candidate MUST limit one operation to one newly staged regular file, one validated destination leaf, and one caller-opened parent directory on the same filesystem, and MUST exclude directory trees, symlink payloads, multiple destination names, generation pointers, transaction markers, and cross-file atomicity.

#### Scenario: One staged file targets one leaf
- GIVEN a caller supplies one opened parent capability, one admitted leaf, bounded bytes, and explicit publication settings
- WHEN the candidate publishes the file
- THEN the operation MAY use one same-parent rename and MUST report only the observations for that one destination name.

#### Scenario: Caller requests a two-file transaction
- GIVEN a caller asks the candidate to publish a payload and sidecar, replace a generation pointer, or recover a transaction marker as one atomic operation
- WHEN request admission runs
- THEN the candidate MUST reject the request as outside version-one scope rather than sequence multiple renames under an atomic claim.

### Requirement: Publication state has a pure functional core

r[durable_file_publication.core.state] A `no_std + alloc` core MUST validate explicit bounded requests and classify caller-supplied observations through finite named phases without filesystem, environment, process, network, clock, randomness, serialization, presentation, or ambient mutable state.

#### Scenario: Valid observations reach durable completion
- GIVEN a valid request and ordered observations for private staging, complete payload synchronization, rename, and parent synchronization
- WHEN the core classifies the operation
- THEN it MUST return a deterministic durably-completed mechanical disposition and the exact completed phase set.

#### Scenario: Impossible observation order is supplied
- GIVEN observations claim parent synchronization or durable completion before rename, or rename before complete payload synchronization
- WHEN the core validates them
- THEN it MUST reject them with a stable invalid-transition class.

### Requirement: Filesystem authority stays in the consumer shell

r[durable_file_publication.shell.authority] The std shell MUST accept a caller-opened destination-directory capability and one already-admitted leaf, MUST keep staging beneath that same parent, and MUST NOT open an ambient root, interpret product paths, authorize a destination, derive product policy, or follow a caller-supplied staging path.

#### Scenario: Consumer supplies an admitted parent and leaf
- GIVEN the product has already authorized and opened a destination parent and validated one leaf
- WHEN the shell stages and publishes
- THEN every filesystem mutation MUST remain relative to that parent and the product MUST retain authority for the root and leaf admission.

#### Scenario: Request supplies an ambient absolute path
- GIVEN no caller-opened parent capability exists or the request attempts to choose an arbitrary staging path
- WHEN shell request admission runs
- THEN the shell MUST reject before creating or mutating a file.

### Requirement: Staging and resource bounds are explicit

r[durable_file_publication.shell.bounds] The shell MUST use exclusive private regular-file creation, explicit byte and collision-attempt bounds, checked arithmetic, an injected staging-name source, explicit permissions, and complete writes; it MUST NOT rely on ambient umask, process identifiers, clocks, unbounded retries, or silent short writes.

#### Scenario: Bounded private stage is written
- GIVEN safe stage-name material, an available exclusive name, bounded bytes, and an explicit permission plan
- WHEN staging runs
- THEN the shell MUST create one private no-follow regular file, write exactly the admitted bytes, apply permissions, and synchronize the complete file before rename.

#### Scenario: Stage names collide until the bound is exhausted
- GIVEN every generated stage leaf already exists through the caller-selected collision limit
- WHEN staging runs
- THEN the shell MUST fail with collision-exhausted before rename and MUST NOT increase the limit implicitly.

### Requirement: Replacement semantics are exact

r[durable_file_publication.publication.replacement] Every request MUST select replace or no-replace explicitly. Replace MUST use one same-filesystem atomic rename. No-replace MUST use one race-free platform primitive or return unsupported before rename; destination-check-then-ordinary-rename MUST NOT satisfy no-replace.

#### Scenario: Race-free no-replace is supported
- GIVEN the platform and directory capability support an atomic no-replace primitive and the destination is absent
- WHEN no-replace publication runs
- THEN exactly one rename operation MAY make the staged file visible without overwriting a concurrently created destination.

#### Scenario: Platform lacks race-free no-replace
- GIVEN the platform exposes only destination inspection plus ordinary replacing rename
- WHEN no-replace publication is requested
- THEN the shell MUST return unsupported before rename instead of using a time-of-check/time-of-use fallback.

### Requirement: Visibility and durability are distinct

r[durable_file_publication.publication.durability] Every request MUST choose visibility-only or durability-required behavior. The staged file MUST be synchronized after final permissions and before rename. Durability-required success MUST additionally require successful parent-directory synchronization after rename, while visibility-only completion MUST carry an explicit crash-durability non-claim.

#### Scenario: Required synchronization completes
- GIVEN the file synchronization, rename, and real parent-directory synchronization all succeed
- WHEN durability-required publication completes
- THEN the result MUST identify the destination as committed and parent-synchronized.

#### Scenario: Parent synchronization is unavailable
- GIVEN the caller requests durability but the capability-backed parent cannot provide a genuine synchronization operation
- WHEN the shell preflights the request
- THEN it MUST return durability-unsupported rather than treating a no-op as success.

### Requirement: Partial commits cannot be hidden

r[durable_file_publication.publication.partial_commit] Outcome and error types MUST preserve whether rename committed the destination. A parent-sync failure after successful rename MUST be reported as committed with durability unknown, and cleanup failures MUST remain distinguishable from the primary failure.

#### Scenario: Rename succeeds and parent sync fails
- GIVEN the staged file was synchronized and rename made it visible but parent-directory synchronization returns an error
- WHEN the shell returns
- THEN the result MUST state that the destination committed, MUST NOT claim durability, and MUST NOT imply the old destination remains current.

#### Scenario: Write fails and cleanup also fails
- GIVEN a pre-rename write or file-sync failure occurs and removal of the private stage also fails
- WHEN the shell returns
- THEN it MUST preserve both the primary failure class and the cleanup-failure observation without claiming any destination commit.

### Requirement: Product semantics remain consumer-owned

r[durable_file_publication.publication.non_authority] The candidate MUST leave path admission, destination authorization, payload meaning, canonical encoding, product content identity, existing-content equivalence, locks, transaction protocols, receipts, rollback, retention, cleanup policy, deployment, and release decisions to consumers.

#### Scenario: Mantle finds an identical existing immutable object
- GIVEN shared no-replace reports that the destination exists
- WHEN the Mantle adapter performs its bounded product-owned byte comparison
- THEN Mantle MAY classify identical content as idempotent success without adding that rule to the candidate API.

#### Scenario: Mechanical publication succeeds
- GIVEN the candidate reports a committed and parent-synchronized file
- WHEN a consumer evaluates product completion
- THEN it MUST still apply its own authorization, content, evidence, lifecycle, and release gates and MUST NOT infer payload truth or whole-product durability from the mechanical result.

### Requirement: One corpus proves both consumer pilots

r[durable_file_publication.validation.corpus] The feasibility package MUST define one shared positive and negative corpus covering request validation, staging, replacement, synchronization, cleanup, partial commit, bounds, and unsupported-platform behavior, and both pilots MUST execute or losslessly map every applicable case.

#### Scenario: Both adapters preserve a negative case
- GIVEN a fixture injects rename success followed by parent-sync failure
- WHEN the core, std shell, Mantle adapter, and OnixOS adapter classify it
- THEN every layer MUST preserve committed visibility and unknown durability without collapsing it to an ordinary failure.

#### Scenario: Adapter silently drops a fixture field or outcome
- GIVEN a consumer cannot represent a required request field, phase observation, or error class
- WHEN losslessness is reviewed
- THEN extraction MUST remain blocked and the mismatch MUST be recorded explicitly.

### Requirement: Mantle pilot preserves immutable-object mechanics

r[durable_file_publication.pilots.mantle] The disposable Mantle pilot MUST map immutable remote-attempt-log object publication at one exact source revision while preserving bounded bytes, private staging, complete file synchronization, race-free no-replace, real parent synchronization, and Mantle-owned identical-content conflict handling.

#### Scenario: Immutable object is absent
- GIVEN a valid bounded attempt-log object and an absent destination
- WHEN the Mantle pilot publishes through the candidate
- THEN the candidate MUST preserve no-replace and synchronization mechanics while Mantle retains object identity and manifest semantics.

#### Scenario: Existing immutable object conflicts
- GIVEN no-replace reports an existing destination with different bytes
- WHEN the Mantle adapter evaluates the product result
- THEN Mantle MUST retain its immutable-object conflict classification and the candidate MUST NOT interpret the bytes.

### Requirement: OnixOS pilot preserves capability mechanics

r[durable_file_publication.pilots.onixos] The disposable OnixOS pilot MUST map `CapabilityRoot::atomic_write_with_policy` at one exact source revision while preserving root confinement, replace/refuse selection, permission behavior, and explicit visibility-versus-durability outcomes.

#### Scenario: Capability-relative replace succeeds
- GIVEN OnixOS supplies an admitted parent and leaf, explicit permissions, bounded bytes, and replace mode
- WHEN the pilot publishes through the candidate
- THEN the file MUST remain beneath the opened root and OnixOS MUST retain path and purpose authority.

#### Scenario: Current parent descriptor cannot be synchronized
- GIVEN the selected OnixOS capability directory is unsuitable for real parent synchronization
- WHEN durability-required publication is requested
- THEN the pilot MUST expose durability-unsupported or obtain a proven synchronizable descendant handle; it MUST NOT preserve the current no-op as durable success.

### Requirement: Losslessness is reviewed explicitly

r[durable_file_publication.validation.losslessness] The final comparison MUST map every candidate request field, phase, outcome, error, platform capability, and non-claim to both consumers and MUST classify omitted, widened, narrowed, or product-specific semantics as explicit mismatches.

#### Scenario: Both pilots use the same boundary
- GIVEN every selected consumer property maps without silent dropping and all shared fixtures pass
- WHEN the comparison is rendered
- THEN it MUST identify the exact common API and separately list all consumer-owned behavior.

#### Scenario: Common API requires a product-specific enum
- GIVEN the candidate needs a Mantle log disposition or OnixOS root-purpose value to preserve behavior
- WHEN boundary review runs
- THEN extraction MUST remain blocked until the field is made product-neutral without loss or removed from shared ownership.

### Requirement: Provenance and licensing gate implementation reuse

r[durable_file_publication.extraction.provenance] The feasibility record MUST bind every reviewed implementation and pilot to exact source revisions, MUST record repository license and copyright authority, and MUST NOT copy or relicense implementation expression into a candidate repository without compatible terms or explicit authorization.

#### Scenario: Clean-room API pilot uses behavior only
- GIVEN the pilot reimplements the documented mechanical contract without copying protected expression
- WHEN provenance is recorded
- THEN the record MUST identify the source behaviors consulted, author, license, and exact revision while distinguishing specification from copied code.

#### Scenario: Candidate copies incompatible source
- GIVEN implementation text comes from a repository whose terms are incompatible with the proposed candidate license and no grant exists
- WHEN promotion is reviewed
- THEN promotion MUST be blocked regardless of test results.

### Requirement: Promotion requires independent release value

r[durable_file_publication.extraction.promotion] A passing decision MUST require two lossless pilots, an acyclic dependency shape below both consumers, source and license authorization, useful standalone core/shell/conformance artifacts, explicit platform support, rollback guidance, and later repo-local adoption Cairns pinned to one immutable candidate revision.

#### Scenario: All promotion gates pass
- GIVEN both pilots and the shared corpus pass, provenance is authorized, dependencies are acyclic, and the package is useful without a stack product
- WHEN the extraction decision is finalized
- THEN it MAY recommend a separate repository while leaving permanent consumer adoption to later Cairns.

#### Scenario: Only local package tests pass
- GIVEN the candidate core and shell tests pass but one consumer pilot, license authorization, independent release value, or immutable adoption path is absent
- WHEN promotion is reviewed
- THEN the disposition MUST remain blocked or incubating rather than extraction-ready.

### Requirement: Validation records bounded evidence and non-claims

r[durable_file_publication.validation.evidence] Validation MUST record selected commands, exact revisions, fixture identities, platform capabilities, passing and failing cases, and unavailable evidence, and MUST state that mechanical publication checks do not prove filesystem correctness, storage-device persistence, power-loss survival, host integrity, payload truth, authorization, or release eligibility.

#### Scenario: Selected platform checks pass
- GIVEN the core, shell, shared corpus, and both adapters pass on recorded platforms
- WHEN evidence is written
- THEN the evidence MUST scope the result to those revisions, filesystems, platform primitives, fixtures, and requested synchronization observations.

#### Scenario: A required platform or consumer check cannot run
- GIVEN the pilot cannot exercise a required syscall, capability handle, fault point, or exact consumer revision
- WHEN evidence is finalized
- THEN that item MUST be recorded as blocked or unavailable and MUST NOT be normalized to passing.
