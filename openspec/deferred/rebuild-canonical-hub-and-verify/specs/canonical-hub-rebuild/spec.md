## Purpose

Deferred by user direction on 1 November 2026. Retained historical contract; no rebuild completion is claimed.

Delete and regenerate the explicitly disposable canonical KnowledgeHub demonstration in files/3, using current checked sources and actual complete-suite/public-walkthrough results. Trace: KH-US-30; KH-DR-02/04–13/15/17–23; ontology §§17/21.10; file-storage §13.3.

## ADDED Requirements

### Requirement: Explicit narrowly scoped demonstration reset
Rebuild SHALL require explicit reset intent and validate the exact canonical demonstration path, expected scope and ordinary-directory boundaries. It SHALL coordinate with existing users and remove all prior content only under that target. Ordinary library create/open SHALL NOT perform this reset.

#### Scenario: Reset the generated canonical store
- **WHEN** the authorised rebuild reaches reset after successful preflight
- **THEN** the complete previous canonical content is deleted before fresh creation, without retaining a migration copy or removing unrelated paths

#### Scenario: Unsafe reset target
- **WHEN** the requested target is a repository root, parent/unrelated path, symlink escape or unexpected Hub scope
- **THEN** reset is refused before any deletion

### Requirement: Complete native suite on exact final inputs
Rebuild SHALL run all unit/integration/internal and target tests, doctests, formatting and Clippy against final implementation inputs. It SHALL retain actual output, exit codes, build/source hashes and observed counts. A failed required check SHALL stop before deleting the previous Hub.

#### Scenario: A regression test fails
- **WHEN** any required native test or check fails
- **THEN** its failure is reported and the previous canonical Hub remains intact

#### Scenario: Passing result belongs to changed code
- **WHEN** current checked inputs differ from a retained successful manifest
- **THEN** the result is refused as current verification and the required checks/capture must be refreshed

### Requirement: Fresh current source capture
Population SHALL use a newly verified current repository/source capture with exact PDF/Markdown/extraction and actual native evidence bindings. It SHALL reject stale or unknown source origins and exclude generated store/cache/output recursion. Historical evidence SHALL remain labelled as its own older run.

#### Scenario: New APIs and modules are implemented
- **WHEN** the current capture includes files/3, neighbour and query contracts
- **THEN** their declared stable origins and exact current bytes are represented rather than silently omitted or replaced by filename identities

### Requirement: New instance without migration continuity
After reset, ordinary public APIs SHALL create and populate a new files/3 Hub and replay domain with the full catalogue and applicable current self-model. Old snapshots, history and operation receipts SHALL NOT be imported or presented as recovered outcomes. New counts and draft gaps SHALL be measured.

#### Scenario: New A and B are generated
- **WHEN** the fresh loader creates original and successor explanations
- **THEN** they have this run's baselines, snapshot history and receipts, and their existence does not imply preservation of the deleted instance

### Requirement: Verified reopened canonical data
Rebuild completion SHALL require successful drain/reopen, full integrity scrub, exact new A/B narratives/comparison and original-outcome/retry checks for this run. Native code-test success alone SHALL NOT establish that the generated Hub is complete or readable.

#### Scenario: Population succeeds but reopening fails
- **WHEN** the post-population inspector or required integrity check fails
- **THEN** the rebuild remains failed/incomplete and no completed-success report is published

### Requirement: Public and indexed walkthrough coverage
Verification SHALL execute full-catalogue and supported neighbour/search/composite walkthroughs as well as actual self-model reads. Fictional rent/late-link examples SHALL use isolated fixtures or clones; the canonical self-description SHALL retain actual library scope and explicit gaps.

#### Scenario: Rent impact example is exercised
- **WHEN** its explicitly mapped table fixture passes traversal verification
- **THEN** the evidence records that isolated outcome without adding fictional rent tables to the canonical library model

### Requirement: Explicit post-reset failure and retry
An interruption after deletion SHALL report absent or incomplete new data honestly and retain attempt diagnostics outside the reset target. A later explicit reset/retry SHALL rebuild from repository inputs. It SHALL NOT silently reuse a partial store as verified or promise recovery of deleted history.

#### Scenario: Process ends during model loading
- **WHEN** loading stops after only part of the fresh demonstration is committed
- **THEN** the attempt is marked incomplete and a clean retry requires explicit reset intent for that same canonical target

### Requirement: New run evidence and honest storage accounting
Successful rebuild SHALL publish new test/walkthrough results, captured checkpoint, fresh domain/baseline/receipt identities and measured storage/read/count data. Older reports SHALL NOT be relabelled as current. Changed model inputs SHALL be distinguished from controlled same-input format comparisons.

#### Scenario: Fresh model contains more implementation definitions
- **WHEN** files/3 and query additions increase source/model coverage
- **THEN** the report records actual new counts and qualifies comparison with the older 116 MB demonstration rather than claiming migration equality

### Requirement: Verification remains distinct from acceptance
A fresh populated Hub SHALL preserve source status, actual test scope, declared applicability/gaps and repository definition ownership. Passing tests or regeneration SHALL NOT create human acceptance or authority-handover records.

#### Scenario: Structurally valid draft is rebuilt
- **WHEN** all required technical checks pass with remaining declared draft gaps
- **THEN** the result reports successful technical verification and those gaps without inventing purpose completeness or human approval
