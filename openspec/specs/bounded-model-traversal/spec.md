# bounded-model-traversal Specification

## Purpose

Compose generic fast neighbour lookups into deterministic multi-hop paths under an explicit typed policy without anticipating future query routes or interpreting user intent. Trace: KH-US-28; KH-DR-03/04/07/12/15/19/21/22; ontology §§8/21.7; file-storage §7.6.

## Requirements

### Requirement: Declarative typed traversal policies
Traversal SHALL accept exact starts, a pinned selection, target predicates and finite typed/directed association policies or ordered steps with finite alternatives. It SHALL validate selectors/types and reject unsupported plans explicitly. Ontology compatibility SHALL NOT create absent model edges.

#### Scenario: Follow mixed directions across several families
- **WHEN** a caller supplies inverse formalises/satisfies followed by supported forward/property steps
- **THEN** traversal follows only actual eligible facts and can return targets beyond the legacy four-hop limit through the additive operation

#### Scenario: Future query uses an existing association type
- **WHEN** a new permitted step plan is submitted for already stored facts
- **THEN** it executes through generic neighbour lookups without a query-specific storage index

### Requirement: Deterministic state-aware expansion
Execution SHALL use stable breadth-first frontier ordering and track exact subject plus applicable policy position/state. Reaching a subject in another required state SHALL NOT be discarded as already visited. Model cycles SHALL terminate under finite bounds without changing recorded facts.

#### Scenario: Alternative routes revisit a subject
- **WHEN** two valid plan positions reach the same exact subject
- **THEN** both eligible continuations are evaluated and deterministic target membership is preserved

#### Scenario: Graph contains a cycle
- **WHEN** a repeated allowed-step policy encounters a cycle
- **THEN** visited-state tracking and work bounds terminate evaluation without inventing edges or mutating the model

### Requirement: Exact witnesses and complete requested target discovery
Results SHALL retain exact assertion/property origins for each reported path and distinguish endpoint deduplication from evidence deduplication. Finding one nearer target SHALL NOT suppress other requested reachable targets. Selected witness limits SHALL be explicit and SHALL NOT imply enumeration of every possible path.

#### Scenario: Near and farther tables both match
- **WHEN** a permitted target is reached at depth three and another at depth six within bounds
- **THEN** both are returned when the query requests all reachable targets, with their separate origin paths

#### Scenario: Parallel assertions support one target
- **WHEN** distinct assertions connect the same endpoint pair
- **THEN** their origins remain available according to the witness budget even when the target record appears once

### Requirement: Finite total evaluation budgets
Requests SHALL receive or declare finite hop, plan-state, start, visited-state, examined-association, target, witness and projected-byte bounds. Work counts SHALL include filtered and duplicate examinations. Results SHALL identify reached bounds separately from exhaustion and ordinary output pagination.

#### Scenario: High fan-out exceeds examined work
- **WHEN** many edges are filtered out before enough targets are found
- **THEN** those examinations still consume the budget and the result is explicitly truncated rather than falsely exhausted

### Requirement: One-view continuations and current authority
Traversal and every continuation SHALL bind exact selection, policy, parameters, projection, actor and scope. Later head movement SHALL NOT alter retained membership. Cursor lifetime/capacity SHALL be finite and current disclosure SHALL be rechecked. External references SHALL remain explicit without automatic fetch.

#### Scenario: Writer changes a path between pages
- **WHEN** a query continues after a successor association is committed
- **THEN** it continues the original exact view and refuses incompatible plan or actor reuse

#### Scenario: External target cannot be resolved locally
- **WHEN** an eligible association names an unsupported external binding
- **THEN** the result identifies that limitation without silently accessing a remote source or substituting another record

### Requirement: Indexed local work and compatibility
Ready-view traversal SHALL compose batched local neighbour lookups rather than rescan all graph associations per subject. The additive operation SHALL preserve existing Query/Page contracts and SHALL NOT perform autonomous planning, execute stored code or infer causal impact.

#### Scenario: Unrelated model grows
- **WHEN** the same finite neighbourhood is traversed while unrelated facts/history increase
- **THEN** instrumentation reports neighbourhood and lookup-path work rather than repeated full graph/history loads, while old Query results remain unchanged

