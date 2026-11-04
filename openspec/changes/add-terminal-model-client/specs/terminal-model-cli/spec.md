# Terminal model CLI

## Purpose

Let people and agents explore configured native models through reproducible commands with the same semantic results as the terminal browser. Trace: KH-DR-01–07/12/15/17–22; KH-US-09/10/21/22/24/27/29; ontology §§5/8/17/21.3/21.7.

## ADDED Requirements

### Requirement: Discoverable exploration commands
The CLI SHALL provide help, version/capability information, configured model listing, model information, ontology/type inspection, typed listing, browsing views, record inspection, relationships, literal search, sources, gaps, lineage, baselines, comparison and visual export. Commands SHALL use the same exploration semantics as the TUI.

#### Scenario: An agent discovers the interface
- **WHEN** it requests top-level help and command-specific help
- **THEN** it obtains supported arguments, selection rules, limits, formats and failure conventions without launching the TUI

### Requirement: Explicit model and selection context
A model-reading command SHALL resolve exactly one configured scope and one snapshot or exact baseline before its dependent reads. Results SHALL identify that selection. Later commands SHALL accept the returned exact selection; defaulting to current head SHALL be explicit.

#### Scenario: Two commands inspect the same state
- **WHEN** an agent passes the snapshot returned by an earlier command after the branch head advances
- **THEN** the later result reads the original state rather than substituting current revisions

### Requirement: Schema-driven discovery and listing
Type and property inspection SHALL reflect exact registered definitions, inherited fields and permitted reference targets. Listing SHALL support exact type selection, explicit subtype inclusion and configured view filters without a hard-coded application vocabulary. Unsupported projection/filter combinations SHALL be refused.

#### Scenario: Explore an unrelated design
- **WHEN** the user lists a registered type not present in FruitTrucks
- **THEN** its declared definitions and permitted instances are available without changing client code

### Requirement: Unambiguous reference resolution
Inspection SHALL accept exact qualified references and semantic-ID lookup within the selected scope. Display names and shortcuts SHALL NOT replace identity. Ambiguous input SHALL return candidate exact references and require an explicit selection.

#### Scenario: Equal names in different contexts
- **WHEN** two definitions share the requested display name
- **THEN** neither is chosen implicitly and the response identifies both permitted candidates

### Requirement: Complete native record inspection
Record inspection SHALL expose native kinds, exact type/revision, predecessor, tagged properties, narrative bindings, provenance and selection status. Historical support SHALL be distinguishable from normative selected membership. Arbitrary native kinds SHALL have a generic inspection path.

#### Scenario: Open an identified assertion
- **WHEN** the user inspects the assertion reference behind a displayed relationship
- **THEN** the CLI returns its exact endpoints, qualifiers, provenance and revision independently of the endpoint records

### Requirement: Directional relationship and reference navigation
Relationship commands SHALL support incoming, outgoing and both directions, exact selectors and Association/Property categories. They SHALL retain distinct assertion and property occurrences, schema paths, exact targets and fact scope. They SHALL NOT infer links from labels or compatible endpoint types.

#### Scenario: Follow acknowledgement evidence
- **WHEN** incoming relationships are requested for the FruitTrucks acknowledgement requirement
- **THEN** the result includes the actual UseCase satisfies Requirement and Scenario illustrates Requirement assertions, with their exact directions and references

### Requirement: Literal search with explicit field coverage
Search SHALL expose the library's supported literal modes, type/field selectors, exact matching spans and finite coverage. No-match, ambiguity, incomplete search and unsupported query SHALL remain distinguishable. The CLI SHALL NOT imply semantic search or whole-artifact coverage.

#### Scenario: Search stops at its bound
- **WHEN** a phrase search reaches a candidate or byte limit
- **THEN** the response reports the reached bound and verified hits without claiming that unmatched records were searched

### Requirement: Exact sources and evidence status
Source commands SHALL resolve exact source revisions/anchors, show retained text excerpts and export original bytes explicitly. They SHALL preserve source/interpretation/review statuses and unresolved gaps. Binary or unsupported previews SHALL offer an original-content export rather than claim extracted text.

#### Scenario: Proposed expiry is mistaken for policy
- **WHEN** the user opens the fixture's five-minute offer-validity definition and its supporting source
- **THEN** the result identifies it as a proposed teaching choice and does not promote it to an accepted client requirement

### Requirement: Historical inspection and baseline comparison
The CLI SHALL enumerate exact baseline references, follow bounded predecessor lineage and compare two exact named baselines through the native comparison contract. It SHALL NOT present arbitrary snapshot or record diff as native baseline comparison, nor substitute later endpoints in historical views.

#### Scenario: Inspect A after successor B
- **WHEN** the reader selects A and compares A with B
- **THEN** original A definitions/sources remain inspectable and comparison identifies exact selected additions, removals and successors

### Requirement: Structured output and process outcomes
JSON mode SHALL emit one versioned result envelope without prompts, colour or progress text on stdout. It SHALL retain selection, data, coverage and typed failures. Human-readable diagnostics SHALL use stderr. Success, invalid usage, incomplete result, denial and operational failure SHALL have documented distinguishable exit outcomes.

#### Scenario: Use the CLI in an agent tool
- **WHEN** an agent requests JSON for a successful read or an ambiguous reference
- **THEN** stdout remains machine-readable in both cases and the envelope and exit status distinguish their outcomes

### Requirement: Finite command completion and cursor honesty
One-shot commands SHALL follow pages only within documented total work/output limits and report exhaustion or reached bounds. They SHALL NOT return a process-local cursor as a reusable continuation after their host exits. Recovery instructions SHALL retain exact selection and query intent.

#### Scenario: A one-shot listing cannot exhaust its result
- **WHEN** its aggregate budget is reached and the process exits
- **THEN** the result is explicitly partial and explains how to rerun the pinned query with narrower filters or permitted larger limits, without advertising a dead cursor

### Requirement: Read failures remain actionable
Missing records, denied access, unsupported inputs, incompatible stores, invalid selections, readiness/capacity failures and interrupted transport SHALL remain distinct from an empty successful result. Retrying SHALL preserve the original requested selection unless the user explicitly refreshes it.

#### Scenario: An index is not ready
- **WHEN** bounded native preparation cannot make the required lookup ready
- **THEN** the CLI reports readiness failure instead of reporting that the definition has no relationships
