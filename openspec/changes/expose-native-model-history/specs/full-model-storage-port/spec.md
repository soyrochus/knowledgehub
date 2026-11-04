# Full model storage port delta

## MODIFIED Requirements

### Requirement: Complete current backend operations
The current adapter SHALL implement all required exact, range, operation-outcome, native publication-history, neighbour and deterministic read operations. History metadata, accepted ancestry and affected references SHALL support bounded selective reads without full-state or all-receipt materialisation. Legacy-only unsupported defaults and full-history fallbacks SHALL be removed rather than used for current selective reads.

#### Scenario: Read a fixed neighbourhood
- **WHEN** unrelated current history and facts grow while the view is prepared
- **THEN** the operation follows current required paths and postings without an old-backend scan fallback


#### Scenario: Read a short native history page
- **GIVEN** a current store contains a long accepted history and large source artifacts
- **WHEN** a consumer requests one finite page at an exact accepted snapshot
- **THEN** storage returns original publication metadata within the declared work/output limits without loading full model states, all receipts, complete affected sets or source contents

#### Scenario: Page a large affected set
- **WHEN** an original receipt affects more exact references than one request permits
- **THEN** selective pages preserve the stored order and exact count without materialising the entire collection first

#### Scenario: Verify against accepted history
- **WHEN** receipt metadata is missing or inconsistent with its accepted generation, parent, scope or sequence
- **THEN** storage reports inability to establish a valid publication rather than authoritative absence or an invented result
