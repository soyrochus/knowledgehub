# composite-model-queries Specification

## Purpose

Answer bounded trace questions in one deterministic search/traverse/project read, returning compact exact evidence and explicit ambiguity or insufficient traceability. Trace: KH-US-28–29; KH-DR-04/07/12/15/18/19/21/22; ontology §§11/17/21.3/21.7; file-storage §7.6–7.7.

## Requirements

### Requirement: Single pinned composite evaluation
A public composite request SHALL combine typed phrase or exact-ID start selection, traversal and projection against one pinned view. Callers SHALL NOT need one library/tool call per hop. Output pagination SHALL preserve that same view and complete query intent.

#### Scenario: Story phrase reaches a physical schema target
- **WHEN** a caller supplies a story phrase and a valid policy through recorded requirements, behaviour and explicit persistence mappings
- **THEN** one request returns matching starts, candidate targets and exact paths within its bounds without requiring intermediate external calls

### Requirement: Explicit match policy and grouped origins
Requests SHALL specify grouped-all or require-one start matching. Several verified starts SHALL remain distinguishable; the engine SHALL NOT choose one by relevance or inferred intent. No match and incomplete matching SHALL be explicit, and target grouping SHALL preserve each originating start.

#### Scenario: Ambiguous story phrase
- **WHEN** several stories match a complete require-one search
- **THEN** the response identifies ambiguity with permitted candidate references instead of traversing an arbitrarily selected story

#### Scenario: Several stories reach the same table
- **WHEN** grouped matching returns starts with a shared physical target
- **THEN** the target can be hydrated once while its separate start/path groups remain intact

### Requirement: Compact exact evidence projection
Results SHALL support requested target metadata, exact origin paths and bounded source-fragment references/snippets with deduplicated record hydration. They SHALL identify query/matching versions, resolved selection, budgets and omitted content. Full source artifacts SHALL NOT be included implicitly.

#### Scenario: Several paths share a large document
- **WHEN** a projection requests their supporting snippets
- **THEN** it returns exact bounded fragments and content references rather than repeated copies of the full document

#### Scenario: One projected item exceeds the byte cap
- **WHEN** a target contains more content than the permitted response size
- **THEN** the result returns its compact exact reference and explicit omission/follow-up status without an endless empty continuation

### Requirement: Honest dependency and coverage meaning
Reported impacts SHALL mean witnessed candidate dependencies under the supplied policy. Physical tables SHALL require explicit model classification/mappings. Missing mappings, declared gaps, external limits and truncation SHALL remain distinct from proven no-impact; a linked target SHALL NOT imply a required schema migration.

#### Scenario: Entity has no physical mapping
- **WHEN** traversal reaches a logical entity with no recorded persistence target
- **THEN** the response reports no witnessed table and insufficient/unknown traceability rather than asserting no database tables are affected

#### Scenario: Only story wording changes
- **WHEN** a changed story retains the same witnessed implementation dependencies
- **THEN** results identify candidate review targets without claiming that table structures must change

### Requirement: Deterministic non-agentic execution
The engine SHALL evaluate only the supplied closed plan, registered ontology facts and supported deterministic matching modes under current policy. It SHALL NOT call an LLM, execute modelled bodies, invent links, rewrite its own plan or mutate knowledge as part of the read.

#### Scenario: Same request after cache reconstruction
- **WHEN** the same view, query versions, parameters and authority are evaluated after indexes are rebuilt
- **THEN** target membership, ordering, witnesses and coverage statuses are identical without any agent or remote inference

### Requirement: Generic vocabulary and scoped demonstration
Composite execution SHALL operate on registered types/selectors rather than hardcoded business routes. The rent demonstration SHALL use isolated explicitly mapped physical-schema fixtures; it SHALL NOT introduce fictional rent concepts or database tables into KnowledgeHub's canonical self-description.

#### Scenario: Another model registers different associations
- **WHEN** a caller supplies its exact registered selectors and physical target type
- **THEN** the same engine evaluates those actual paths without adding model-specific traversal/index code

