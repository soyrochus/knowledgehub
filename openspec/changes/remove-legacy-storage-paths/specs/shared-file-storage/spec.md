## REMOVED Requirements

### Requirement: Explicit format selection and legacy preservation
**Reason**: One current default backend replaces opt-in formats and legacy preservation.
**Migration**: Construct FileBackend::new for current full storage. Older stores remain unsupported; use a fresh demonstration.

## ADDED Requirements

### Requirement: One current default file format
FileBackend::new SHALL construct the sole current knowledgehub-files/4 backend. Older-format constructors, selectors, aliases and automatic conversion SHALL be absent. Open SHALL reject any other required format without modifying or recreating it.

#### Scenario: Construct without a format selector
- **WHEN** a host constructs the default backend and creates a full Hub
- **THEN** it uses the current complete implementation and its versioned metadata

#### Scenario: Open an older instance
- **WHEN** a caller passes a files/1, files/2 or files/3 store to the current backend
- **THEN** open refuses it explicitly and leaves its bytes unchanged

## MODIFIED Requirements

### Requirement: Shared content and change-proportional writes
The current file backend SHALL reuse unchanged record/content objects and persistent state structures. Repeated identical content SHALL NOT add another logical payload. A small definition change or baseline capture SHALL NOT write another complete graph/artifact copy; diagnostics SHALL distinguish new content from structural metadata.

#### Scenario: One explanation changes
- **WHEN** an author changes one narrative and its required bindings in a model containing many unrelated artifacts
- **THEN** the new state shares unchanged content and tree structures while the prior state remains exact

