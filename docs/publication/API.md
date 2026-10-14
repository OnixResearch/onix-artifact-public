# Durable one-file publication candidate API

<!-- r[impl durable_file_publication.core.state] -->
<!-- r[impl durable_file_publication.shell.authority] -->
<!-- r[impl durable_file_publication.shell.bounds] -->
<!-- r[impl durable_file_publication.publication.scope] -->
<!-- r[impl durable_file_publication.publication.replacement] -->
<!-- r[impl durable_file_publication.publication.durability] -->
<!-- r[impl durable_file_publication.publication.partial_commit] -->
<!-- r[impl durable_file_publication.publication.non_authority] -->

The disposable API is in `fixtures/publication-candidate`.

## Request

`PublicationRequest` contains only mechanical facts:

- One caller-admitted destination leaf.
- Exact payload byte count and maximum.
- A stage-name collision-attempt limit.
- Exact final Unix permission bits.
- `Replace` or `NoReplace`.
- `VisibilityOnly` or `DurabilityRequired`.

The API accepts payload bytes separately. Their length must equal the request count before any I/O starts.

The candidate opens no root. `PublicationIo` represents one parent that the consumer already opened and authorized. `StageNameSource` is injected and bounded by the request.

## Functional core

`classify_publication` validates one finite observation stream. It accepts only this successful order:

1. Private stage created.
2. Complete payload written.
3. Exact permissions applied.
4. Payload synchronized.
5. Rename committed.
6. Parent synchronized, when durability is required.

A failure before rename requires a cleanup observation when a stage exists. A parent-sync failure after rename produces `CommittedDurabilityUnknown`.

The core does not prove that observations are true. The shell supplies them, and consumer evidence can retain them.

## Imperative shell

`publish_one_file` completes request admission and platform preflight before stage creation. It then calls one capability-relative `PublicationIo` implementation.

The Linux/Android pilot wraps a caller-opened directory `File`. It uses:

- `openat` with create-new, no-follow, and close-on-exec flags.
- `write_all` for complete writes.
- Exact permission application after creation.
- File `sync_all` before rename.
- `renameat` for replace.
- `renameat_with(NOREPLACE)` for race-free no-replace.
- `unlinkat` for bounded stage cleanup.
- Directory `fsync` after rename when durability is required.

No-replace or parent synchronization returns unsupported during preflight when the implementation lacks the exact primitive.

## Outcomes

- `NotCommitted`: retains the primary failure and cleanup result.
- `DestinationExists`: retains cleanup state without content interpretation.
- `CommittedVisibilityOnly`: rename committed under an explicit durability non-claim.
- `CommittedAndParentSynchronized`: rename and the requested parent-sync call completed.
- `CommittedDurabilityUnknown`: rename committed, but parent synchronization failed.

These outcomes do not prove filesystem correctness, device persistence, or power-loss survival.

## Excluded behavior

The candidate does not own path admission, destination authorization, product identity, canonical encoding, equality on existing content, multi-file commits, generation pointers, locks, receipts, rollback, stale-stage recovery, retention, cleanup policy, deployment, or release decisions.

It does not accept payload symlinks or caller-selected staging paths. It does not publish directory trees.
