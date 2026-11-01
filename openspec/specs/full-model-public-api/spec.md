# full-model-public-api Specification

## Purpose

Give every consumer the complete governed ontology/model API through the ordinary crate-root entry point. Trace KH-SL-014, KH-DR-01–13/15–19 and ontology §§4–8/17/21.2–21.4.

## Requirements

### Requirement: One full public entry point
The crate SHALL expose KnowledgeHub as the full typed model application API with its owned model/query contracts at the ordinary import boundary. The seed application and OntologyHub name SHALL NOT be retained as aliases, deprecated facades or feature-selected alternatives.

#### Scenario: Ordinary consumer imports
- **WHEN** a caller imports KnowledgeHub and full-model contract types from knowledgehub
- **THEN** it can create/open a full Hub and use current operations without opting into another application API

#### Scenario: Removed facade names
- **WHEN** a caller compiles against the previous seed application or OntologyHub alias
- **THEN** those obsolete names are unavailable rather than silently selecting another contract

### Requirement: Full-model scope and state
Requests, policy checks and storage publication SHALL use the actual model scope and a mandatory typed state. No synthetic seed profile or optional seed/full state union SHALL mediate the public contract. Ontology/profile selection SHALL remain governed model data.

#### Scenario: Inspect policy scope
- **WHEN** current host policy evaluates a full-model request
- **THEN** it receives that model's scope without fabricated bootstrap vocabulary

#### Scenario: Create an unrelated model
- **WHEN** an authorised host creates a model using supported registered vocabulary
- **THEN** the API does not require KnowledgeHub's atomic-change example records

### Requirement: One qualified operation receipt
A committed receipt SHALL bind the actual scope, actor, request fingerprint, original outcome/head and one collection of qualified exact affected revisions. Duplicate legacy and full affected collections SHALL NOT be required.

#### Scenario: Commit a typed successor
- **WHEN** a guarded current change succeeds
- **THEN** its receipt identifies exact qualified revisions and an exact retry returns that original receipt without another generation

### Requirement: Complete current model operations
The sole API SHALL retain registration, validation, exact reads, artifacts/ranges, revision/baseline/history operations, bounded selection, neighbours, traversal, text and composite reads. Current authority, finite continuations, explicit gaps and deterministic evidence semantics SHALL apply.

#### Scenario: Use current reads from the default API
- **WHEN** a caller performs typed selection and a phrase-to-target traversal
- **THEN** it receives bounded owned results, exact origins and explicit coverage without an older facade or backend mode

### Requirement: Current correctness remains enforced
The cutover SHALL preserve immutable accepted history, exact source/ownership, schema constraints, atomic guarded publication, original retries and recovery within the current implementation. API simplification SHALL NOT make a valid definition imply execution or acceptance.

#### Scenario: Read an older current-store baseline
- **WHEN** a current Hub advances after baseline A and is reopened
- **THEN** A still resolves its exact knowledge and original outcomes while current authority remains checked
