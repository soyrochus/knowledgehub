# Spec Delta

## MODIFIED Requirements

### Requirement: Declared and checked file-backend support profile
The file backend SHALL declare its supported host/local-filesystem profiles, writer-coordination assumptions, format version and tested durability boundary. Supported targets SHALL be Linux x86_64/local ext4 and macOS arm64/local APFS, with ordinary files and staging/objects/references on one filesystem. It SHALL use stable branch lock files with nonblocking exclusive OS advisory locks, including coordination between processes and separately opened same-process instances. Scope creation SHALL be coordinated and exclusive. Busy locks SHALL return an explicit retryable busy result, distinct from stale-head conflict once a new operation reaches admission. It SHALL coordinate all supported writers and reject detected unsupported or incompatible stores explicitly. The host SHALL declare the support profile; a path alone SHALL not be treated as proof of hardware durability or the absence of unsupported external writers. It SHALL not claim arbitrary multi-host shared-folder writes or untested durability guarantees.

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

#### Scenario: Native APFS profile is checked
- **WHEN** a host declares macOS arm64/local APFS for a writable local APFS store with an ordinary same-filesystem layout
- **THEN** creation/opening succeeds under current authority and the normal coordination protocol
- **AND** admission does not depend on Linux mount metadata or a cast of Linux device identifiers

#### Scenario: Host declaration contradicts the detected location
- **WHEN** a host declares APFS on a non-APFS or nonlocal volume, ext4 on the Mac host, an uncoordinated arrangement, or a managed subtree crossing filesystem boundaries
- **THEN** the operation is explicitly refused without creating or changing accepted store content

#### Scenario: Independent readers and maintenance contend
- **GIVEN** a reader owns the supported shared read lease on either platform
- **WHEN** an independently opened backend attempts exclusive publication or compaction
- **THEN** it receives the established busy outcome until the reader releases its lease
- **AND** process death releases owned OS locks without deleting or replacing the stable lock identity

### Requirement: Durable current-format acknowledgement
Within each declared current support profile, success SHALL require durable complete content, original receipt and reference publication. The adapter SHALL sync required candidate files/directories, sync its temporary reference, atomically replace the same-filesystem head, then sync its directory before acknowledgement. Content and receipt SHALL recover together. Each profile SHALL use a documented native persistence sequence; unsupported required persistence or failed sync SHALL NOT be silently ignored or treated as a weaker successful commit.

#### Scenario: Reopen an acknowledged current commit
- **WHEN** a covered failure follows a successful current acknowledgement
- **THEN** the complete accepted content and exact original receipt recover together

#### Scenario: Persistence fails before acknowledgement
- **WHEN** the protocol cannot establish a durable outcome
- **THEN** it reports the established prior outcome or uncertainty without confirmed durable success

#### Scenario: Native persistence fails before reference replacement
- **WHEN** required object, pack, reference-file or directory persistence fails before head replacement on either platform
- **THEN** the prior accepted head remains unchanged and the caller receives an explicit failure

#### Scenario: Native persistence fails after reference replacement
- **WHEN** reference-directory persistence fails after possible publication on macOS or Linux
- **THEN** the caller receives uncertainty and resolves the original operation identity after reopening
- **AND** a recovered committed operation returns its original receipt without another generation

## ADDED Requirements

### Requirement: Full native runtime journey
Each supported native profile SHALL run the ordinary full-model create, commit, exact/range read, query, baseline, original-retry, reopen, compact and scrub operations under the existing authority, conflict, immutability and finite-query contracts. A successful build or an unsupported-platform rejection test SHALL NOT establish runtime support.

#### Scenario: Maintain a sourced model on the Mac
- **WHEN** an authorised maintainer registers the catalogue, captures original bytes, commits baseline A and a sourced successor B on supported APFS, then closes and reopens
- **THEN** exact A/B content, artifact ranges, history and original operation outcomes remain resolvable
- **AND** compaction, scrub and actual neighbour/text/impact queries preserve those values and their selection/policy bounds

#### Scenario: Invalid or denied native request
- **WHEN** a caller lacks current permission, submits invalid source/ownership bindings, reuses an operation identity for different intent, or reaches admission with a stale head
- **THEN** the corresponding denial, invalidity, operation-reuse or conflict result preserves the accepted state on both native profiles

### Requirement: Native recovery evidence per support profile
Completion SHALL require exact-source native evidence for both supported profiles, including process interruption, injected persistence failures, cancellation, original-result recovery and publication/maintenance coordination. Evidence SHALL identify actual compiler, OS, architecture, filesystem and tested failure boundary. Compilation, cross-compilation or another platform's log SHALL NOT substitute for a native execution.

#### Scenario: Stop at every publication and maintenance checkpoint
- **WHEN** the native fault batch kills writers at all eight current publication checkpoints and seven pack/cleanup checkpoints on each supported profile
- **THEN** reopening exposes the exact prior or complete new accepted state, original bytes and lifetime receipt history without accepting orphan content
- **AND** maintenance creates no model generation or changed original outcome

#### Scenario: Only one native platform is available
- **WHEN** Mac checks pass but the matching-source Linux regression batch is missing, or vice versa
- **THEN** the report records the missing evidence and the change remains technically incomplete
- **AND** process interruption is not reported as power-loss, kernel-crash, hardware durability or human acceptance evidence
