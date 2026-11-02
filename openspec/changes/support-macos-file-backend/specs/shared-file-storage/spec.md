# Spec Delta

## MODIFIED Requirements

### Requirement: One current default file format
FileBackend::new SHALL construct the sole current knowledgehub-files/4 backend on every supported native profile. Platform support SHALL be selected through host support configuration, not another backend, format mode or semantic API. Older-format constructors, selectors, aliases and automatic conversion SHALL be absent. Open SHALL reject any other required format without modifying or recreating it.

#### Scenario: Construct without a format selector
- **WHEN** a host constructs the default backend and creates a full Hub
- **THEN** it uses the current complete implementation and its versioned metadata

#### Scenario: Open an older instance
- **WHEN** a caller passes a files/1, files/2 or files/3 store to the current backend
- **THEN** open refuses it explicitly and leaves its bytes unchanged

#### Scenario: Select native file support
- **WHEN** a host supplies its supported Linux/ext4 or Mac/APFS configuration to the ordinary default backend
- **THEN** it uses the same full-model operations, current record/request/object codecs and storage diagnostics
- **AND** model operations require no Mac-specific request or filesystem transaction value

## ADDED Requirements

### Requirement: Recognised creation profile and checked current location
Current-format metadata SHALL retain a recognised creation-profile identity. Create SHALL record the checked current profile; open SHALL preserve that field and independently check the present host/filesystem. Linux-origin and Mac-origin files/4 stores SHALL be recognised by the updated implementation. Unknown profiles or unsupported current locations SHALL be refused before store mutation.

#### Scenario: Create on local APFS
- **WHEN** a host successfully creates a current store under checked Mac/APFS support
- **THEN** metadata records the Mac creation profile and the existing current-format/codec/scope identities

#### Scenario: Reopen a Linux-origin fixture on the Mac
- **GIVEN** a complete files/4 fixture was closed on supported Linux and copied offline to supported local APFS
- **WHEN** the updated implementation opens it under a checked Mac host declaration
- **THEN** its recognised Linux creation profile is retained byte-for-byte and does not substitute for checking the current APFS location
- **AND** the existing scope, replay domain, head, baselines, record hashes and original receipts are retained

#### Scenario: Reject an unrecognised profile
- **WHEN** metadata declares an unknown creation profile, or a recognised fixture resides on an unsupported current location
- **THEN** opening fails explicitly without rewriting metadata, recreating the Hub or changing accepted content

### Requirement: Exact offline current-format continuity
A complete, quiescent current-format fixture copied between supported native profiles SHALL retain exact semantic IDs, revisions, source bytes, accepted ancestry, baselines, replay domain and original outcomes. Subsequent guarded use SHALL append ordinary history without conversion, implicit rebasing or metadata rewriting. Live copying and simultaneous multi-host access SHALL remain outside the supported procedure.

#### Scenario: Compare both offline copy directions
- **WHEN** independently verified Linux-origin and Mac-origin fixtures are closed, copied offline to the other supported native profile and reopened with the updated implementation
- **THEN** exact snapshots, source digests, baselines, queries and original-operation lookups agree with the originating fixtures
- **AND** an exact original retry adds no generation, while a valid new guarded change adds only its ordinary successor
- **AND** the isolated source copy remains unchanged

#### Scenario: Maintenance on the copied fixture
- **WHEN** the authorised destination host compacts a copied current fixture and rebuilds disposable indexes
- **THEN** full scrub, exact reads and queries preserve accepted semantic content, history and original receipts
- **AND** physical maintenance changes neither the creation-profile metadata nor the model head

#### Scenario: An incomplete copy is presented
- **WHEN** a copied fixture lacks required accepted content or its metadata/objects fail integrity checks
- **THEN** opening or the required read/scrub reports incompatibility or integrity failure without treating missing knowledge as absence or an empty model
