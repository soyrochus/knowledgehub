# shared-file-storage Specification

## Purpose

Retain exact KnowledgeHub states and original operation results through shared immutable file objects, avoiding complete copies for small changes. Trace: KH-US-25–26; KH-DR-01–15/17/20; ontology §§5/21.4–21.7; file-storage §§3–9/11.

## Requirements

### Requirement: Lossless immutable record and artifact reconstruction
Stored projections SHALL reconstruct exact accepted records under their original semantic codec and verify their digests. Original artifact bytes, ordered values, null/absence, ownership, provenance and opaque references SHALL remain unchanged. Logical reference cycles SHALL NOT require circular content hashes.

#### Scenario: Factored baseline and source quotation
- **WHEN** an original ordered baseline and exact quoted anchor are stored, reopened and reconstructed
- **THEN** their public values and semantic digests match the accepted originals and the source bytes retain their original digest

#### Scenario: Mutually referring records
- **WHEN** valid records with preallocated revision tokens refer to each other
- **THEN** the store retains and resolves them without changing their tokens or solving a physical hash cycle

### Requirement: Shared content and change-proportional writes
The current file backend SHALL reuse unchanged record/content objects and persistent state structures. Repeated identical content SHALL NOT add another logical payload. A small definition change or baseline capture SHALL NOT write another complete graph/artifact copy; diagnostics SHALL distinguish new content from structural metadata.

#### Scenario: One explanation changes
- **WHEN** an author changes one narrative and its required bindings in a model containing many unrelated artifacts
- **THEN** the new state shares unchanged content and tree structures while the prior state remains exact

### Requirement: Distinct baseline and generation semantics
A baseline SHALL pin exact semantic selection, dependencies, ontology/profile and gap values. Each accepted change SHALL produce one generation; an exact retry or physical maintenance SHALL produce none. Accepted ancestry, unselected revisions and original outcomes SHALL be retained for the store lifetime.

#### Scenario: Baselines share unchanged knowledge
- **WHEN** A captures the original explanation and B captures its successor
- **THEN** each resolves its exact explanation and unchanged definitions share storage without implying review approval

### Requirement: Guarded atomic and durable publication
A new change SHALL publish its complete validated state, history and receipt under current policy, expected-head and revision guards. Pre-publication failure SHALL preserve the accepted head. A failure after possible publication SHALL report uncertainty until recovery establishes the outcome.

#### Scenario: Same-base race
- **WHEN** two distinct valid requests compete from the same accepted head
- **THEN** one complete change succeeds and the other receives a conflict without a mixed state

#### Scenario: Process ends near head replacement
- **WHEN** publication is interrupted at a persistence boundary
- **THEN** recovery exposes the complete old or complete new accepted state and never promotes an orphan candidate

### Requirement: Original operation and accepted-history resolution
Current-authorised exact retries SHALL return the original actor-bound receipt before stale-base rejection. Changed actor or intent SHALL be refused. Operation absence and historical membership SHALL be established from the recovered accepted state; missing content or failed recovery SHALL NOT establish absence.

#### Scenario: Retry after later commits
- **WHEN** a caller submits its original request after the head has advanced
- **THEN** it receives the original receipt and no additional generation

#### Scenario: Receipt content is missing
- **WHEN** the accepted operation registry binds an unreadable receipt
- **THEN** lookup reports integrity/unavailability rather than absent or a new successful execution

### Requirement: Bounded root-bound reads and complete explicit scrub
A point/range read SHALL remain bound to one accepted selection and verify required immutable paths without materialising unrelated snapshots/artifacts. Full owned reads SHALL preserve their public completeness contract. Full retained-history health SHALL be established by explicit scrub, not inferred from a successful small read.

#### Scenario: Unrelated history grows
- **WHEN** a fixed record is read with increasing unrelated retained generations and artifacts
- **THEN** required object reads follow its lookup/content paths rather than loading every historical graph, and reported work excludes no hidden full-history scan

#### Scenario: Current authority is revoked
- **WHEN** a caller loses required read or original-outcome authority before a new request
- **THEN** the request is denied despite possession of earlier references or receipts

### Requirement: One current default file format
FileBackend::new SHALL construct the sole current knowledgehub-files/4 backend. Older-format constructors, selectors, aliases and automatic conversion SHALL be absent. Open SHALL reject any other required format without modifying or recreating it.

#### Scenario: Construct without a format selector
- **WHEN** a host constructs the default backend and creates a full Hub
- **THEN** it uses the current complete implementation and its versioned metadata

#### Scenario: Open an older instance
- **WHEN** a caller passes a files/1, files/2 or files/3 store to the current backend
- **THEN** open refuses it explicitly and leaves its bytes unchanged
