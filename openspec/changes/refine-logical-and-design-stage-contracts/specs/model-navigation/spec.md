## MODIFIED Requirements

### Requirement: Exact typed selection
Public selection SHALL identify its resolved snapshot or exact baseline. It SHALL support exact type-definition and semantic-ID filters, an explicit subtype-inclusive type mode, and documented stable ordering. Returned records SHALL retain their declared types, exact revisions and provenance, including distinctions between absent and null values.

#### Scenario: Find the library's use cases
- **WHEN** a reader selects UseCase instances in the KnowledgeHub baseline
- **THEN** the result returns only instances of that selected exact type in stable qualified-ID/revision order and identifies the baseline revision

#### Scenario: Exact and inclusive selection differ
- **WHEN** a reader queries type T with exact mode and then with subtype-inclusive mode in the same baseline
- **THEN** exact mode returns only T while inclusive mode also returns its valid direct/transitive descendants under that baseline's exact parent graph
- **AND** each result reports its mode and exact selection

#### Scenario: Parent selection changes between pages
- **WHEN** a continuation's selected hierarchy differs from the current head
- **THEN** its pages retain the original exact hierarchy and membership, recheck current read authority and refuse changed query intent


## ADDED Requirements

### Requirement: Shared bounded type matching
Subtype-enabled selection, lookup and endpoint filters SHALL agree with native reference/assertion validation on qualified selected type identities and their exact parent closure. These reads SHALL preserve finite bounds, explicit truncation, stable ordering and external-reference disclosure; inheritance SHALL NOT silently widen authority or resolve external schemas.

#### Scenario: Query a registered extension
- **WHEN** a model containing a valid extension is assessed and then read with subtype-enabled lookup and endpoint filters
- **THEN** the same parent graph determines matching without bypassing read authority or treating a truncated page as complete

#### Scenario: Schema moves after preparation
- **WHEN** a prepared read intent or model candidate depends on a selected schema revision that later changes
- **THEN** the read stays bound to its original exact selection and the candidate's relevant publication guards prevent silent reinterpretation

