# ontology-neighbour-lookup Specification

## Purpose

Let permitted readers retrieve actual typed incoming/outgoing neighbours quickly in one exact model view, including relationships added after their endpoints and schema-declared reference occurrences. Trace: KH-US-27; KH-DR-03/04/07/12/15/17/19/21; ontology §§5/8/21.7; file-storage §7.4–7.5.

## Requirements

### Requirement: Generic batch direct-neighbour retrieval
The library SHALL support multiple exact starting subjects, incoming/outgoing/both directions and explicit association/type/category filters. Results SHALL identify opposite exact endpoints, selectors and origins in stable order. Legal ontology endpoint types alone SHALL NOT create neighbours.

#### Scenario: Follow inverse formalisation
- **WHEN** a reader requests incoming formalises associations for an exact UserStory
- **THEN** it receives only recorded eligible requirement links with their assertion revisions and exact endpoints

#### Scenario: Discover actual relationship kinds
- **WHEN** the selector filter is omitted for one subject
- **THEN** lookup enumerates that subject's actual permitted selector groups without requiring a list of every ontology relationship

### Requirement: Associations can arrive after endpoints
A later valid commit SHALL expose added associations in both directions without recreating endpoints or designing a query-specific index. Removing/unselecting an association SHALL affect the new selected view without altering prior views.

#### Scenario: Requirement is written months later
- **WHEN** a new Requirement formalises an already accepted UserStory
- **THEN** the new snapshot returns the link from both ends while the old baseline still has no such link

### Requirement: Registered reference fields and exact occurrences
Lookup SHALL support actual tagged reference fields permitted by registered schemas, including nested structures and collections. Each property result SHALL retain owning record/type, schema path and occurrence path. New registered types SHALL NOT require model-specific indexing code or string-ID inference.

#### Scenario: New type has nested reference lists
- **WHEN** an author registers a valid type with nested reference-bearing fields and creates instances
- **THEN** their actual forward/reverse occurrences are retrievable with distinct paths through the generic mechanism

#### Scenario: Repeated or parallel references
- **WHEN** two assertions or property occurrences link the same exact endpoints
- **THEN** both origins remain distinguishable even if endpoint hydration is deduplicated

### Requirement: Exact view and fact-scope isolation
Every lookup SHALL pin one selection, ontology/profile and explicit fact scope. Default facts SHALL use selected assertions/reference owners; supporting-history traversal SHALL be explicit. Exact historical endpoint pins SHALL NOT be repointed to latest. Provenance and administrative membership SHALL NOT enter ordinary model traversal implicitly.

#### Scenario: Current head advances during paging
- **WHEN** a neighbour continuation selected at A is read after B changes a relationship
- **THEN** its membership and ordering remain A's exact facts

#### Scenario: Selected assertion pins an older endpoint
- **WHEN** a valid eligible assertion targets a retained earlier revision
- **THEN** lookup returns that exact revision and separately indicates its selection status rather than returning a successor

### Requirement: Complete derived-index readiness
An index used for a complete answer SHALL match the exact view and extractor/schema versions. Missing, corrupt, incomplete or stale derived data SHALL trigger bounded correct reconstruction or explicit readiness/capacity/integrity failure. It SHALL NOT produce false empty or complete results. Authoritative corruption SHALL remain an integrity failure.

#### Scenario: Cache belongs to another baseline
- **WHEN** lookup encounters an index built for a different selection
- **THEN** it rebuilds or refuses readiness explicitly instead of serving that index as current

#### Scenario: Commit succeeds but cache installation fails
- **WHEN** accepted graph publication succeeds and subsequent derived-index persistence fails
- **THEN** the original committed receipt remains valid and a later lookup prepares its correct view or reports not-ready explicitly

### Requirement: Bounded permitted owned results
Lookup SHALL enforce finite starts, examined-posting, page, projection-byte and cursor limits, distinguish exhaustion from truncation, and recheck current authority on each request. Unsupported modes SHALL fail explicitly. Results SHALL be owned semantic values without physical adapter paths.

#### Scenario: High fan-out exceeds its budget
- **WHEN** a subject has more eligible neighbours than the request can examine or return
- **THEN** the result identifies the reached bound and pinned continuation state without claiming complete enumeration

#### Scenario: Access revoked before continuation
- **WHEN** the host revokes disclosure before the next page
- **THEN** possession of a cursor does not disclose the retained result

### Requirement: Local lookup work and linear index growth
With a ready index, lookup SHALL enumerate the relevant endpoint/selector neighbourhood rather than scan every model association or retained generation per subject. Index entry counts SHALL grow with actual facts, not all possible paths. Work measurements SHALL separate first preparation, cold reads and warm lookups.

#### Scenario: Many unrelated edges are added
- **WHEN** unrelated graph size grows while a queried neighbourhood remains fixed
- **THEN** measured posting/record reads stay tied to that neighbourhood plus lookup-path overhead, without hidden full graph/history scans

### Requirement: Unified current navigation semantics
Maintained Query/Page selection and traversal SHALL use current full-model/indexed semantics. Enumeration, type/ID filtering and finite paging SHALL remain available. The old assertion-order/four-hop implementation and compatibility-only traversal dispatch SHALL NOT remain.

#### Scenario: Use current selection and traversal
- **WHEN** a caller enumerates typed records or follows recorded relationships through the full API
- **THEN** it receives deterministic exact-view bounded results without a separate legacy traversal engine
