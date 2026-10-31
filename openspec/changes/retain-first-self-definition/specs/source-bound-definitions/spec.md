# Source-bound definitions

## Purpose

Allow a permitted caller to retain and retrieve a small system definition with explicit semantic identity, definition ownership and exact source provenance, beginning with the KnowledgeHub library's own proposed behaviour.

## ADDED Requirements

### Requirement: Asynchronous storage-independent consumer contract
The library SHALL expose I/O-bearing operations asynchronously using owned semantic request/result values. It SHALL distinguish denied, unavailable, incompatible, invalid, conflict, operation-reuse and storage/recovery outcomes through structured public errors. Consumers SHALL not require driver objects, mutable accepted-state references or knowledge of internal file layout.

#### Scenario: Public consumer distinguishes refused work
- **WHEN** the host performs the seed walkthrough through public contracts and receives a denied, invalid, conflict or uncertain-storage result
- **THEN** it can distinguish the relevant outcome and safe next action without parsing adapter-private error text or manipulating native storage resources

### Requirement: Explicit model scope and bootstrap profile
The library SHALL create or open an identified Hub/model branch bound to an explicit supported ontology/profile version and host policy context. It SHALL reject incompatible metadata or unsupported profile versions without treating them as an empty model. The initial supported profile SHALL be `knowledgehub-seed/1`, pinned to exact ontology/type/constraint revisions, with the required fields, cardinalities and single-owner bindings in adopted KH-D-01/02. It SHALL reject unsupported normative properties. A complete seed SHALL contain the atomic-change Requirement, Commit Change Set UseCase and their identified satisfaction assertion; an empty created scope SHALL not be reported as a completed seed.

#### Scenario: Create and reopen the seed scope
- **GIVEN** a currently permitted maintainer supplies a supported bootstrap profile and new model scope
- **WHEN** the host creates the scope and later reopens it
- **THEN** the library returns the same semantic Hub/model identities and exact profile binding, with the current branch head identified

#### Scenario: Existing scope is incompatible
- **GIVEN** stored metadata declares an unsupported profile or file-format version
- **WHEN** the host opens that scope
- **THEN** the library returns an explicit incompatibility and preserves the original stored content

### Requirement: Exact source capture and meaningful anchors
The library SHALL retain an identified immutable source revision, its original content digest and meaningful source anchors for each required seed citation. It SHALL distinguish an authored interpretation from the captured source and from review approval. Original bytes SHALL remain retrievable under permitted exact-source reads. SHA-256 SHALL hash the exact original bytes. Seed anchors SHALL identify valid UTF-8 byte ranges, expected quoted bytes and human-readable section labels; the range and quote SHALL resolve against the exact captured revision.

#### Scenario: Capture the ontology proposal fragment
- **WHEN** a permitted change captures the exact ontology Markdown and anchors its atomic-change definition to section 21.4
- **THEN** the source revision, original bytes, digest, anchor and authored interpretation are inspectable from the committed seed without implying approval or execution evidence

#### Scenario: Invalid source binding
- **WHEN** a candidate cites a missing required source revision, an anchor that cannot resolve, or a digest inconsistent with supplied original content
- **THEN** validation reports the binding violation and no part of that candidate becomes the accepted model

### Requirement: Typed seed definitions and relationship identity
The library SHALL validate the initial profile's required record properties and relationship direction, endpoint types and multiplicity against one pinned scope. Each committed seed element and assertion SHALL have a stable semantic identity and immutable revision. A `UseCase satisfies Requirement` assertion SHALL identify the exact compatible endpoint revisions.

#### Scenario: Commit the self-description seed
- **WHEN** a valid candidate defines the Commit Change Set use case, the atomic-change requirement and their identified satisfaction assertion
- **THEN** exact reads return both element revisions, the assertion revision, the pinned type definitions and the assertion's supporting source binding

#### Scenario: Wrong endpoint type or unresolved revision
- **WHEN** the satisfaction assertion targets another use case instead of a requirement, or cites a revision absent from its selected scope
- **THEN** validation identifies the endpoint violation and the accepted head remains unchanged

#### Scenario: Unsupported modelling type
- **WHEN** a caller submits a type outside the registered seed profile
- **THEN** the library rejects that unsupported type explicitly rather than storing an unchecked accepted property bag

#### Scenario: A required seed field is missing
- **WHEN** a candidate omits the use case actor, trigger or outcome, a required narrative fragment, or the assertion's supporting interpretation
- **THEN** validation identifies the missing obligation and preserves the accepted head

### Requirement: Coherent document and graph ownership
The library SHALL retain the seed Markdown definition with stable fragment bindings and an explicit authority map. Document-owned narrative fields and graph-owned semantic fields SHALL be committed with their required provenance and bindings as one change. Conflicting normative owners or inconsistent required bindings SHALL be rejected.

#### Scenario: Narrative and structured seed agree
- **WHEN** a candidate contains the authored Markdown definition and its declared graph/provenance bindings under one authority map
- **THEN** the committed snapshot identifies which representation owns each normative field and preserves all required bindings coherently

#### Scenario: Competing normative definition
- **WHEN** a candidate declares both independently editable Markdown and graph values authoritative for the same field in one scope
- **THEN** validation reports the ownership conflict and neither value replaces accepted content

### Requirement: Permitted snapshot-bound exact reads
The library SHALL check current scoped host authority for exact definition, source and snapshot reads. Each read SHALL resolve one explicit snapshot and return owned public values with exact identity, revision and provenance bindings. Consumer contracts SHALL not require native storage handles or knowledge of the adapter's internal file layout.

#### Scenario: A concurrent commit cannot mix an exact read
- **GIVEN** a read has selected snapshot A and another writer subsequently commits snapshot B
- **WHEN** the reader resolves the seed definition, assertion and source bindings for A
- **THEN** every returned binding resolves within A and the result identifies A, without mixing in B's revisions

#### Scenario: Access is revoked before a new read
- **GIVEN** the caller previously read the seed but no longer has permission for its scope
- **WHEN** it requests another exact definition or source read
- **THEN** the library refuses disclosure under the current policy without treating possession of an identity as permission

#### Scenario: Public read after restart
- **WHEN** a permitted host reopens the committed seed and reads it through public contracts
- **THEN** it obtains the original exact Markdown, element/assertion revisions, source bytes and bindings without opening adapter-private storage objects

### Requirement: Declared executor and bounded file work
The initial file backend SHALL use the host-owned Tokio runtime without creating a hidden runtime or exposing runtime resources in public semantic values. It SHALL support current-thread and multithreaded Tokio callers under the Rust 2024 / minimum Rust 1.85 target. Blocking I/O and unbounded CPU work SHALL run off async scheduler threads. Backend instances SHALL enforce finite active and waiting job limits, defaulting to 4 active and 32 waiting, configurable by the host. Capacity exhaustion SHALL yield an explicit unavailable/capacity result before job submission and mutation. Caller cancellation SHALL not release publication resources while an admitted publication continues. Orderly host shutdown SHALL drain admitted jobs before terminating the runtime.

#### Scenario: File job capacity is exhausted
- **GIVEN** the configured active and waiting file-job capacities are occupied
- **WHEN** another request needs a file job
- **THEN** it receives an explicit retryable capacity result without submitting another job or changing accepted state

Verification directive · 31 October 2026: the user requires existing Rust 1.99.1, no compiler installation and assumed compliant platforms. Actual checks use that compiler with both schedulers; minimum Rust execution is assumed, not claimed as evidence.

#### Scenario: Both supported executor configurations
- **WHEN** the public seed consumer runs on the minimum toolchain and current stable using either supported Tokio scheduler configuration
- **THEN** it can create, commit, reopen and read without blocking the async scheduler on file work or requiring a hidden library runtime

#### Scenario: Caller stops waiting during publication
- **GIVEN** a publication job owns its required resources
- **WHEN** its caller cancels the waiting future
- **THEN** the publication retains its lock and owned data until its publication/recovery boundary completes, and a later permitted original-operation lookup can establish the outcome
