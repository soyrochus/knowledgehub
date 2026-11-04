# Full model public API delta

## MODIFIED Requirements

### Requirement: Complete current model operations
The sole API SHALL retain registration, validation, exact reads, artifacts/ranges, revision/baseline/history operations, bounded selection, neighbours, traversal, text and composite reads. Current authority, finite continuations, explicit gaps and deterministic evidence semantics SHALL apply. Distinct native publication-history reads SHALL use current scoped read/history authority and owned contracts without changing actor-bound mutation recovery.

#### Scenario: Use current reads from the default API
- **WHEN** a caller performs typed selection and a phrase-to-target traversal
- **THEN** it receives bounded owned results, exact origins and explicit coverage without an older facade or backend mode


#### Scenario: Inspect original publications through the sole API
- **GIVEN** a current Hub contains publications made by another actor before consumer registration
- **WHEN** a currently permitted reader resolves known publications or requests bounded discovery
- **THEN** the default KnowledgeHub API supplies exact original attribution, pinned scope/selection, explicit coverage and finite continuations without another facade or storage parser

#### Scenario: History disclosure preserves mutation recovery
- **WHEN** a reader is granted publication-history access without the original publisher's identity
- **THEN** history can be disclosed under current policy while actor-bound operation lookup, canonical retries and operation-reuse refusals retain their existing meanings
