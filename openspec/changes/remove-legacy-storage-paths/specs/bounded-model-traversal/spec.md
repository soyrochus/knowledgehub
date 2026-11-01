## MODIFIED Requirements

### Requirement: Declarative typed traversal policies
Traversal SHALL accept exact starts, a pinned selection, target predicates and finite typed/directed association policies or ordered steps with finite alternatives. It SHALL validate selectors/types and reject unsupported plans explicitly. Ontology compatibility SHALL NOT create absent model edges.

#### Scenario: Follow mixed directions across several families
- **WHEN** a caller supplies inverse formalises/satisfies followed by supported forward/property steps
- **THEN** traversal follows only actual eligible facts and can return targets within the current documented finite plan/hop bounds

#### Scenario: Future query uses an existing association type
- **WHEN** a new permitted step plan is submitted for already stored facts
- **THEN** it executes through generic neighbour lookups without a query-specific storage index


## REMOVED Requirements

### Requirement: Indexed local work and compatibility
**Reason**: Current indexed work remains required; preserving the superseded Query/Page ordering is explicitly removed by this cutover.
**Migration**: Use current deterministic selection/traversal semantics and finite paging.

## ADDED Requirements

### Requirement: Indexed current local work
Ready-view traversal SHALL compose batched local neighbour lookups rather than rescan all graph associations per subject. Current Query/Page expansion SHALL use that engine. Reads SHALL NOT perform autonomous planning, execute stored code or infer causal impact.

#### Scenario: Unrelated model grows
- **WHEN** the same finite neighbourhood is traversed while unrelated facts/history increase
- **THEN** instrumentation reports neighbourhood and lookup-path work rather than repeated full graph/history loads under the current bounded query semantics
