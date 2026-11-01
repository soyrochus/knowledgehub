# model-navigation Specification

## Purpose

Let readers discover typed definitions and follow their exact relationships within
bounded, reproducible selections without knowing adapter layout or query syntax.
Trace: KH-DR-03/04/07/12/15/19; KH-US-09/10/22; ontology §§8, 21.3/21.7.

## Requirements

### Requirement: Exact typed selection
Public selection SHALL identify its resolved snapshot or exact baseline. It SHALL support exact type-definition and semantic-ID filters with documented stable ordering. Returned records SHALL retain their declared types, exact revisions and provenance, including distinctions between absent and null values.

#### Scenario: Find the library's use cases
- **WHEN** a reader selects UseCase instances in the KnowledgeHub baseline
- **THEN** the result returns only instances of that selected exact type in stable qualified-ID/revision order and identifies the baseline revision

### Requirement: Typed directional traversal
Traversal SHALL accept exact starting subjects, qualified relationship declarations, explicit direction and finite bounds. It SHALL return identified assertion revisions and exact endpoint references. Kernel subjects SHALL be traversable where their declared relationships permit them. Node deduplication SHALL NOT erase distinct assertions.

#### Scenario: Follow requirement to execution evidence
- **WHEN** a reader follows inverse satisfies and checks from the atomic-change requirement, then inverse executes from a selected TestCase
- **THEN** the result identifies the selected UseCase, distinct TestCase and actual TestRun records with their exact supporting assertions

#### Scenario: Follow source-anchor provenance
- **WHEN** the reader follows supportsAssertion from an exact source anchor
- **THEN** the result resolves the original identified assertion in the same selection

### Requirement: Finite traversal and explicit truncation
Requests SHALL declare or receive documented finite page, depth and visited-subject/edge bounds. Results SHALL distinguish exhaustion from truncation and identify the bound reached. Cycles SHALL terminate under those bounds. Unsupported filters, predicates or excessive limits SHALL fail explicitly.

#### Scenario: Traverse a cyclic process model
- **WHEN** a permitted reader traverses a valid loop with a finite depth limit
- **THEN** the response terminates, identifies its selection and reports truncation if further reachable content remains

### Requirement: Continuations remain bound to intent
A continuation SHALL bind exact selection, scope, actor, filters, direction and ordering. Later head movement SHALL NOT change its membership. A continuation used with different intent SHALL be refused. Every new request SHALL recheck current disclosure policy.

#### Scenario: Another writer commits between pages
- **WHEN** a reader continues a result selected at A after the branch advances to B
- **THEN** the next page continues A in the same stable order without mixing in B

#### Scenario: Reuse a cursor under another actor or predicate
- **WHEN** a caller changes the actor or predicate associated with a continuation
- **THEN** the library rejects the incompatible continuation without returning another actor's result

### Requirement: Current scope and reference disclosure
Selection and traversal SHALL use current scoped host read authority. Qualified external references SHALL remain identifiable as external without implicit fetching or traversal into an unauthorised model. Unsupported external resolution SHALL NOT appear as a resolved local revision.

#### Scenario: Revocation before continuation
- **WHEN** a previously permitted reader loses access before requesting the next page
- **THEN** the request is denied rather than relying on possession of its cursor

#### Scenario: Native implementation reference
- **WHEN** a result includes an exact repository code artifact reference
- **THEN** it reports the captured or qualified artifact binding without silently accessing a remote repository

### Requirement: Explicit continuation lifetime and capacity
Continuations SHALL have a documented finite lifetime and backend-instance capacity. The initial contract SHALL allow at most 64 active continuations, expire them after 30 minutes of inactivity and invalidate them when the backend closes. Release SHALL be explicit. Invalid or expired continuation SHALL be distinct from exhausted results; exact selection references SHALL remain durable.

#### Scenario: Continue after reopening
- **WHEN** a caller uses an earlier continuation after its backend was closed and reopened
- **THEN** the library reports an invalid continuation and permits a new query against the original exact snapshot or baseline

#### Scenario: Continuation capacity is full
- **WHEN** a new paginated selection would exceed the documented backend capacity
- **THEN** it reports capacity without accepting an unbounded cursor or mutating the model

