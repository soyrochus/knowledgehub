## MODIFIED Requirements

### Requirement: Declarative typed traversal policies
Traversal SHALL accept exact starts, a pinned selection, target predicates and finite typed/directed association policies or ordered steps with finite alternatives. It SHALL validate selectors/types and reject unsupported plans explicitly. Ontology compatibility SHALL NOT create absent model edges.

#### Scenario: Follow mixed directions across several families
- **WHEN** a caller supplies inverse formalises/satisfies followed by supported forward/property steps
- **THEN** traversal follows only actual eligible facts and can return targets within the current documented finite plan/hop bounds

#### Scenario: Future query uses an existing association type
- **WHEN** a new permitted step plan is submitted for already stored facts
- **THEN** it executes through generic neighbour lookups without a query-specific storage index

