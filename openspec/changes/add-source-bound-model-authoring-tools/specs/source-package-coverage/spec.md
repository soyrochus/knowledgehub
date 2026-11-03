# Spec Delta

## Purpose

Account for the complete supplied design package independently of model-integrity assessment, retaining original artifacts, source identities, authority and consequential UI behavior. Trace: MM-SL-01/C3, O-05/O-06, ontology §§4/14/17/21.4–21.10 and KH-DR-01–06/08/12/15–19.

## ADDED Requirements

### Requirement: Exact source inventory and authority
A source manifest SHALL inventory supplied artifacts with normalized path, stable source key, exact digest, byte length, media type, authority role and explicit exclusions. Original binary and UTF-8 bytes SHALL be retained through native capture. Duplicate renditions SHALL retain separate originals and explicit concept mappings rather than silently doubling or merging definitions.

#### Scenario: Markdown and PDF express one concept
- **WHEN** two captured renditions describe the same stable requirement identifier
- **THEN** one declared semantic identity can bind both exact originals with explicit ownership/equivalence interpretation
- **AND** differences remain reviewable instead of being erased by filename matching

#### Scenario: File changes after inventory
- **WHEN** supplied bytes differ from the pinned manifest during compilation or publication
- **THEN** the tool rejects that input revision without silently capturing changed content as the original source

### Requirement: Complete meaningful UI source retention
A UI directory designated as the working UI definition SHALL retain that authority irrespective of its name. Its relevant HTML, CSS, scripts, images and local dependencies SHALL be inventoried and captured. Source expectations SHALL cover consequential controls, states, bindings and effects, not just screen names or screenshots.

#### Scenario: Consequential interaction appears only in script
- **WHEN** a retained script defines a confirmation, condition, error state or command binding absent from the narrative overview
- **THEN** the source-to-model map accounts for that behavior with exact support or flags an unresolved interpretation
- **AND** the report does not declare UI coverage from surface counts alone

#### Scenario: Referenced local asset is missing
- **WHEN** a UI source references an unavailable local stylesheet, script or image
- **THEN** the manifest reports the missing dependency and any interpretation limit without substituting empty bytes or silently following an external URL

### Requirement: Independent source obligation accounting
Coverage SHALL compare a source-specific expected inventory with exact source-to-model mappings and native readback. It SHALL distinguish retained bytes, structured semantics, source omissions, permitted native gaps, interpretation review and unsupported/excluded material. Application IDs, counts, entities and UI controls SHALL come from input data rather than production rules.

#### Scenario: Structurally valid model omitted a requirement
- **WHEN** native assessment is valid/complete but an in-scope expected requirement or consequential UI interaction lacks its required model mapping
- **THEN** source coverage fails and the tool does not report a full import

#### Scenario: Source never supplied an implementation artifact
- **WHEN** a design refers to an absent external implementation or unresolved decision
- **THEN** the report identifies that source gap and planned status without inventing an artifact, implementation success or human approval

### Requirement: Exact resolving interpretation evidence
Mappings SHALL identify the exact model subject/property/assertion, source anchor and interpretation/review state. UTF-8 byte ranges SHALL resolve against captured bytes; non-text locators SHALL retain their extraction/locator provenance. Generated previews or inferred descriptions SHALL NOT replace originals or be labelled as approved human decisions.

#### Scenario: Quote or locator fails resolution
- **WHEN** a quoted byte range is not on valid UTF-8 boundaries or a PDF/UI locator does not resolve against its declared source-bound extraction
- **THEN** verification rejects the mapping and identifies its source/target path

### Requirement: Bounded accountable coverage reports
Reports SHALL identify exact baseline, manifest, plan and tool contracts, actual record/payload sizes, exercised read limits and unexhausted pages. Every supplied file and expected item SHALL have a mapped, unresolved, unsupported or explicitly excluded disposition. Truncation, exclusions and unresolved material SHALL remain visible in machine-readable and human-readable results.

#### Scenario: Oversized or unsupported input
- **WHEN** an input exceeds a finite configured bound or has no supported extraction path
- **THEN** the report preserves its inventory/disposition and does not silently omit it or claim complete semantic coverage

#### Scenario: Reproduce a report after reopening
- **WHEN** another local process receives the retained source/mapping manifests and native baseline
- **THEN** it can independently repeat source-byte, obligation and representative trace checks without agent chat history or application infrastructure
