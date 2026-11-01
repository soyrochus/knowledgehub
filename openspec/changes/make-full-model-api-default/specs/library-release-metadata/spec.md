## Purpose

Identify the requested unpublished knowledgehub 0.10.0 implementation consistently in package, lock and observed source/build records. Trace KH-SL-014 and ontology §21.9–21.10.

## ADDED Requirements

### Requirement: Consistent 0.10.0 package identity
The package and its own lockfile entry SHALL declare version 0.10.0. Build/capture records that identify this implementation SHALL report that version from actual metadata. Publication SHALL remain disabled unless separately authorised.

#### Scenario: Observe release metadata
- **WHEN** the current crate is inspected or its source/build identity is captured
- **THEN** Cargo.toml, Cargo.lock and reported package identity agree on knowledgehub 0.10.0 with publish=false

### Requirement: Version identities remain independent
Crate, ontology, request/record codec and physical storage identities SHALL identify their actual contracts independently. A crate-version bump SHALL NOT add old-version readers or claim preservation of a superseded store.

#### Scenario: Payload contract changes
- **WHEN** the current receipt schema is simplified for 0.10.0
- **THEN** the storage identity distinguishes that contract and older instances need no migration support

