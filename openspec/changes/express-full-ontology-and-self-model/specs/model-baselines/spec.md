## Purpose

Let authors change sourced definitions while preserving exact historical meaning
and compare named coherent selections after restart.
Trace: KH-DR-01/02/04–06/08–13/17; KH-US-06–08/21; ontology §§5, 14, 21.4–21.6.

## ADDED Requirements

### Requirement: Immutable successor lineage
A revision request SHALL identify a stable semantic identity, exact predecessor and expected branch head. Admission SHALL create a new immutable revision with explicit lineage. Renaming a representation SHALL preserve semantic identity. Prior content and exact source/representation bindings SHALL remain retrievable.

#### Scenario: Revise the atomic-change explanation
- **WHEN** an authorised author revises the KnowledgeHub requirement and its document-owned fields against baseline A
- **THEN** the new revision retains its identity and exact predecessor while A still resolves the original narrative, provenance and relationships

### Requirement: Coherent representation and relationship changes
A change to document-owned content SHALL submit new authoritative bytes and all required revised bindings together. Relationship successors SHALL explicitly retain or change endpoint revision pins. The library SHALL reject unresolved or competing bindings rather than silently retargeting assertions to current revisions.

#### Scenario: Missing revised fragment binding
- **WHEN** a document successor invalidates a required selected narrative binding without a coherent replacement
- **THEN** validation rejects the candidate and preserves the earlier accepted selection

#### Scenario: Preserve an earlier assertion
- **WHEN** the author keeps an assertion pinned to its earlier endpoint revisions in historical baseline A while creating revised endpoints for baseline B
- **THEN** each baseline resolves its own explicit compatible selection without rewriting A or silently repinning its assertion

### Requirement: Exact named baseline membership
A baseline SHALL have stable identity, immutable revision, model scope, exact ontology/profile bindings and explicit member/supporting-artifact revisions. It SHALL resolve its required relationships and provenance coherently. Baseline labels SHALL NOT mean latest, and membership SHALL NOT be inferred from a timestamp.

#### Scenario: Create baseline A then move the head
- **WHEN** a permitted change creates A and later commits advance the branch
- **THEN** reading A returns its original exact membership and applicable artifacts, independently of current head contents

#### Scenario: Incompatible baseline selection
- **WHEN** a proposed baseline selects an assertion without its required exact endpoint revision or mixes incompatible ontology/profile bindings
- **THEN** admission reports the incompatibility and does not publish a partial baseline

### Requirement: Deterministic baseline comparison
A permitted comparison SHALL identify both exact baseline revisions and report added, removed and changed selected identities, exact before/after revisions, relationship changes and provenance/representation changes. Results SHALL use a documented stable order and SHALL NOT modify either baseline.

#### Scenario: Compare the two self-model explanations
- **WHEN** the maintainer compares A with B after revising the requirement and supporting assertion
- **THEN** the result identifies those exact successors, their changed narrative/provenance and any unchanged selected identities in stable order

### Requirement: Guarded changes and current disclosure
Revision and baseline requests SHALL retain current scoped policy, exact predecessor/expected-head guards, atomic publication and original-operation recovery. Reads and comparisons SHALL check current access to their full requested scope. A stale candidate SHALL NOT be silently rebased.

#### Scenario: Two revisions from one predecessor
- **WHEN** one concurrent revision commits and a second request from the same earlier head reaches admission
- **THEN** the second conflicts, and neither its document nor its baseline partially replaces accepted content

#### Scenario: Revoked baseline access
- **WHEN** access is revoked before a new baseline read or comparison
- **THEN** current policy refuses disclosure even if the caller holds an earlier baseline reference

### Requirement: Baseline and lineage recovery
Acknowledged baselines, exact revision lineage, provenance and their original receipts SHALL survive reopen within the supported file failure profile. An uncertain revision/baseline operation SHALL be resolved under its original identity before changing intent. Incomplete accepted history SHALL prevent an authoritative outcome claim.

#### Scenario: Lose the response after publishing baseline B
- **WHEN** the complete B change publishes before the process is interrupted without replying
- **THEN** recovery retrieves both complete A and B and the original B receipt, and exact retry performs no second revision
