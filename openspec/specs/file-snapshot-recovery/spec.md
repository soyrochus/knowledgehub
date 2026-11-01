# file-snapshot-recovery Specification

## Purpose

Preserve the library's coherent commit and exact-read contract in a portable file backend, with a declared single-host support boundary and inspectable recovery after interrupted publication or lost acknowledgement.

## Requirements

### Requirement: Declared and checked file-backend support profile
The file backend SHALL declare its supported host/local-filesystem profile, writer-coordination assumptions, format version and tested durability boundary. The initial target SHALL be Linux x86_64 with ordinary files on local ext4 and staging/generations/references on one filesystem. It SHALL use stable branch lock files with nonblocking exclusive OS advisory locks, including coordination between processes and separately opened same-process instances. Scope creation SHALL be coordinated and exclusive. Busy locks SHALL return an explicit retryable busy result, distinct from stale-head conflict once a new operation reaches admission. It SHALL coordinate all supported writers and reject detected unsupported or incompatible stores explicitly. The host SHALL declare the support profile; a path alone SHALL not be treated as proof of hardware durability or the absence of unsupported external writers. It SHALL not claim arbitrary multi-host shared-folder writes or untested durability guarantees.

#### Scenario: Supported local scope is opened
- **WHEN** the host opens a compatible store under the declared supported profile
- **THEN** the backend identifies the format/profile and establishes the writer coordination required for that scope before admitting writes

#### Scenario: Concurrent supported writers
- **WHEN** two supported clients attempt publication against the same branch on the coordinated host
- **THEN** publication and expected-head checks are coordinated so that only a valid guarded outcome becomes accepted

#### Scenario: Unsupported format or writer arrangement
- **WHEN** the store format or requested writer arrangement falls outside the declared supported profile
- **THEN** the backend refuses that unsupported operation and preserves the existing accepted store rather than advertising broader support

#### Scenario: Writer lock is busy
- **GIVEN** another supported writer owns the branch publication lock
- **WHEN** a writer attempts nonblocking acquisition
- **THEN** it receives a retryable busy outcome without changing the head or consuming a blocking worker indefinitely waiting for the lock

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

#### Scenario: Damaged store is not recreated
- **GIVEN** existing Hub metadata or its accepted generation fails integrity checks
- **WHEN** a host attempts create/open at that location
- **THEN** the adapter reports the incompatibility/integrity failure and does not overwrite it with an empty store

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

### Requirement: Durable current-format acknowledgement
Within the declared current support profile, success SHALL require durable complete content, original receipt and reference publication. The adapter SHALL sync required candidate files/directories, sync its temporary reference, atomically replace the same-filesystem head, then sync its directory before acknowledgement. Content and receipt SHALL recover together.

#### Scenario: Reopen an acknowledged current commit
- **WHEN** a covered failure follows a successful current acknowledgement
- **THEN** the complete accepted content and exact original receipt recover together

#### Scenario: Persistence fails before acknowledgement
- **WHEN** the protocol cannot establish a durable outcome
- **THEN** it reports the established prior outcome or uncertainty without confirmed durable success

### Requirement: Current metadata and uncertain publication
Metadata SHALL pin the sole current file, request and profile/schema identities. A failure after possible head publication SHALL report uncertainty until original-operation recovery establishes the outcome. Required process-interruption/lost-response evidence SHALL NOT imply power-loss, kernel-crash or hardware guarantees.

#### Scenario: Directory sync fails after replacement
- **WHEN** the new complete head is replaced but reference-directory sync fails
- **THEN** the caller receives uncertainty and resolves its original operation rather than repeating new intent
