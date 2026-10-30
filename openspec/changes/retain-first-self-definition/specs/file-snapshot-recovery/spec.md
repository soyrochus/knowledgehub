# File snapshot recovery

## Purpose

Preserve the library's coherent commit and exact-read contract in a portable file backend, with a declared single-host support boundary and inspectable recovery after interrupted publication or lost acknowledgement.

## ADDED Requirements

### Requirement: Declared and checked file-backend support profile
The file backend SHALL declare its supported host/local-filesystem profile, writer-coordination assumptions, format version and tested durability boundary. It SHALL coordinate all supported writers and reject unsupported or incompatible stores explicitly. It SHALL not claim arbitrary multi-host shared-folder writes or untested durability guarantees.

#### Scenario: Supported local scope is opened
- **WHEN** the host opens a compatible store under the declared supported profile
- **THEN** the backend identifies the format/profile and establishes the writer coordination required for that scope before admitting writes

#### Scenario: Concurrent supported writers
- **WHEN** two supported clients attempt publication against the same branch on the coordinated host
- **THEN** publication and expected-head checks are coordinated so that only a valid guarded outcome becomes accepted

#### Scenario: Unsupported format or writer arrangement
- **WHEN** the store format or requested writer arrangement falls outside the declared supported profile
- **THEN** the backend refuses that unsupported operation and preserves the existing accepted store rather than advertising broader support

### Requirement: Readers observe complete immutable snapshots
The file backend SHALL expose only complete accepted snapshots whose manifests resolve all required model, document, provenance and receipt content. A read SHALL remain bound to its selected immutable snapshot. Staging content and incomplete generations SHALL never be treated as accepted content, and reopening SHALL not create a new empty model over damaged metadata.

#### Scenario: Reader holds A while B is prepared
- **WHEN** a reader resolves A and a writer prepares B
- **THEN** all of the reader's exact bindings continue to resolve within complete A, independently of staged B

#### Scenario: Interrupted staging is found on reopen
- **GIVEN** B was staged but never became an accepted head
- **WHEN** the backend reopens the scope
- **THEN** the prior valid head remains usable and staged B is identifiable as unaccepted content, without merging it into that head

#### Scenario: Accepted content is missing or corrupt
- **WHEN** a required object of the accepted manifest is missing or fails its integrity check
- **THEN** opening or reading reports the integrity/recovery failure explicitly without inventing an empty scope or silently accepting unrelated staging content

### Requirement: Durable acknowledgement covers content and receipt
Within the declared supported durability profile, the file backend SHALL acknowledge success only after the complete snapshot, original operation result and branch publication meet its specified persistence protocol. File replacement alone SHALL not be presented as proof of multi-file durability. Acknowledged content and its receipt SHALL be recoverable together.

#### Scenario: Reopen after acknowledged commit
- **GIVEN** the host received a success receipt for B
- **WHEN** a failure covered by the supported profile occurs and the store reopens
- **THEN** B is complete, all exact seed content resolves and the same original operation receipt is retrievable

#### Scenario: Persistence fails before a successful acknowledgement
- **WHEN** the required persistence protocol cannot establish a durable outcome
- **THEN** the caller receives no confirmed durable success, and the backend reports the established prior state or an uncertain outcome suitable for original-operation lookup

### Requirement: Publication interruption preserves an established outcome
Within its supported failure profile, the file backend SHALL recover interruption at staging, content persistence, head publication and acknowledgement into a complete prior or new snapshot without mixed content. A recovered new head SHALL resolve its complete content and original receipt. Damage outside that guarantee SHALL be reported explicitly and SHALL not count as successful recovery evidence.

#### Scenario: Stop before head publication
- **WHEN** the process is interrupted after some candidate content is persisted but before a new head is committed
- **THEN** recovery retains the prior accepted snapshot and does not expose an incomplete candidate as the new head

#### Scenario: Stop around head publication
- **WHEN** the process is interrupted at the publication boundary
- **THEN** recovery establishes either the prior complete snapshot or the new complete snapshot with its original receipt, and never presents a mixed snapshot as valid

#### Scenario: Stop after publication before acknowledgement
- **GIVEN** B and its original receipt were committed
- **WHEN** the process stops before replying and then reopens
- **THEN** the permitted caller resolves B through the original operation identity, and an exact retry returns that receipt without another change
