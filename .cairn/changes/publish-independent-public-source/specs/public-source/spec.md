# Independent Public Source

## ADDED Requirements

### Requirement: Public source identity is independent and bounded

r[artifact_workspace.public_source.independent] Current public distribution MUST use immutable revisions of `OnixResearch/onix-artifact-public` descending only from audited parentless base `f379f2583b5d152cfd6d58919738652861793c90`, MUST preserve authentication/transfer/binding source, license grants and frozen historical evidence, and MUST remain source-only and NOT RELEASE-ADMITTED.

#### Scenario: Anonymous exact source succeeds

- GIVEN the independently published audited base and a scoped metadata descendant
- WHEN an anonymous consumer fetches the exact revision and its current source input graph
- THEN the selected public source MUST be available without credentials and current repository projections MUST identify the public mirror without relabeling historical evidence.

#### Scenario: Original ancestry or claim widening is rejected

- GIVEN source that includes original private history, altered frozen evidence, changed mechanism bytes, or inherited release claims
- WHEN evaluating publication scope
- THEN that source MUST NOT be admitted as this independent source-only publication.
