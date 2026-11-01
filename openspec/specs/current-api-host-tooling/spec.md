# current-api-host-tooling Specification

## Purpose

Make every maintained command, capture and source binding describe the sole current full API while isolating the user's retained old_hub. Trace KH-SL-016, KH-DR-01/02/05/07/12/13/15/18/23 and ontology §21.2–21.7.

## Requirements

### Requirement: Unambiguous current commands
Maintained authoring, catalogue, inspection, benchmark and maintenance commands SHALL use the full KnowledgeHub API and sole current backend by default. Old format selectors and old-constructor branches SHALL be absent rather than accepted aliases.

#### Scenario: Run an ordinary current command
- **WHEN** a user invokes the maintained inspector or authoring example without a format option
- **THEN** it uses only the current full implementation

#### Scenario: Supply an obsolete format flag
- **WHEN** a caller supplies an old files2/files3 selector
- **THEN** the command rejects the obsolete argument explicitly

### Requirement: Retained store isolation
Source capture/checkpoints SHALL exclude old_hub and generated stores/caches/outputs. Commands SHALL NOT rename, delete, import or modify old_hub. A rebuild target check SHALL refuse old_hub, other unrelated paths and symlink escapes.

#### Scenario: Keep the user's previous instance
- **WHEN** old_hub contains the renamed prior Hub while current sources are captured and fixtures verified
- **THEN** its bytes remain untouched and no old_hub content enters the source inventory

#### Scenario: Wrong reset destination
- **WHEN** a caller chooses old_hub or a path escaping the canonical target
- **THEN** the command refuses before modifying any store

### Requirement: Actual current source bindings
Capture, authoring and coverage audit SHALL agree on actual current public contracts/modules and package version. Archive-aware source resolution SHALL preserve current provenance identities. Unknown, duplicate or stale origins SHALL fail explicitly.

#### Scenario: Capture archived planning and current code
- **WHEN** current source capture includes archived change artifacts and the sole API's current modules
- **THEN** authoring resolves their declared origins without aliases to removed software contracts

#### Scenario: An origin is unknown
- **WHEN** a current file or public contract has no declared semantic binding
- **THEN** validation fails before creating the canonical Hub

### Requirement: First cutover requires an absent destination
The first 0.10.0 canonical regeneration SHALL require hub to be absent after the user's rename. A remaining older hub SHALL cause a preflight refusal without auto-renaming, decoding through an old backend or deleting it.

#### Scenario: User rename remains outstanding
- **WHEN** hub still holds the superseded instance when the cutover workflow starts
- **THEN** preflight reports that the destination is occupied and leaves both hub and old_hub untouched

### Requirement: Honest current and historical guidance
Current guides and examples SHALL describe one supported API/storage contract. Historical source books, archived plans, logs and captured models SHALL retain their original scope and SHALL NOT be rewritten as evidence that the new implementation ran.

#### Scenario: Inspect an older execution report
- **WHEN** a reader follows a retained historical native-run reference
- **THEN** its old version/scope remains explicit while current commands use the new API
