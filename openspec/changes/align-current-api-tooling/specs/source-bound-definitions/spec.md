## MODIFIED Requirements

### Requirement: Asynchronous storage-independent consumer contract
The library SHALL expose I/O-bearing operations asynchronously using owned semantic request/result values. It SHALL distinguish denied, unavailable, incompatible, invalid, conflict, operation-reuse and storage/recovery outcomes through structured public errors. Consumers SHALL not require driver objects, mutable accepted-state references or knowledge of internal file layout.

#### Scenario: Public consumer distinguishes refused work
- **WHEN** the host performs the full-model walkthrough through public contracts and receives a denied, invalid, conflict or uncertain-storage result
- **THEN** it can distinguish the relevant outcome and safe next action without parsing adapter-private error text or manipulating native storage resources

### Requirement: Exact source capture and meaningful anchors
The library SHALL retain an identified immutable source revision, its original content digest and meaningful source anchors for each required model citation. It SHALL distinguish an authored interpretation from the captured source and from review approval. Original bytes SHALL remain retrievable under permitted exact-source reads. SHA-256 SHALL hash the exact original bytes. Byte-range anchors SHALL identify valid UTF-8 byte ranges, expected quoted bytes and human-readable section labels; the range and quote SHALL resolve against the exact captured revision.

#### Scenario: Capture the ontology proposal fragment
- **WHEN** a permitted change captures the exact ontology Markdown and anchors its atomic-change definition to section 21.4
- **THEN** the source revision, original bytes, digest, anchor and authored interpretation are inspectable from the committed model without implying approval or execution evidence

#### Scenario: Invalid source binding
- **WHEN** a candidate cites a missing required source revision, an anchor that cannot resolve, or a digest inconsistent with supplied original content
- **THEN** validation reports the binding violation and no part of that candidate becomes the accepted model

### Requirement: Coherent document and graph ownership
The library SHALL retain authored Markdown definitions with stable fragment bindings and an explicit authority map. Document-owned narrative fields and graph-owned semantic fields SHALL be committed with their required provenance and bindings as one change. Conflicting normative owners or inconsistent required bindings SHALL be rejected.

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
- **WHEN** the reader resolves the selected definition, assertion and source bindings for A
- **THEN** every returned binding resolves within A and the result identifies A, without mixing in B's revisions

#### Scenario: Access is revoked before a new read
- **GIVEN** the caller previously read the seed but no longer has permission for its scope
- **WHEN** it requests another exact definition or source read
- **THEN** the library refuses disclosure under the current policy without treating possession of an identity as permission

#### Scenario: Public read after restart
- **WHEN** a permitted host reopens the committed model and reads it through public contracts
- **THEN** it obtains the original exact Markdown, element/assertion revisions, source bytes and bindings without opening adapter-private storage objects

### Requirement: Declared executor and bounded file work
The initial file backend SHALL use the host-owned Tokio runtime without creating a hidden runtime or exposing runtime resources in public semantic values. It SHALL support current-thread and multithreaded Tokio callers under the Rust 2024 / minimum Rust 1.85 target. Blocking I/O and unbounded CPU work SHALL run off async scheduler threads. Backend instances SHALL enforce finite active and waiting job limits, defaulting to 4 active and 32 waiting, configurable by the host. Capacity exhaustion SHALL yield an explicit unavailable/capacity result before job submission and mutation. Caller cancellation SHALL not release publication resources while an admitted publication continues. Orderly host shutdown SHALL drain admitted jobs before terminating the runtime.

#### Scenario: File job capacity is exhausted
- **GIVEN** the configured active and waiting file-job capacities are occupied
- **WHEN** another request needs a file job
- **THEN** it receives an explicit retryable capacity result without submitting another job or changing accepted state

Verification directive · 31 October 2026: the user requires existing Rust 1.99.1, no compiler installation and assumed compliant platforms. Actual checks use that compiler with both schedulers; minimum Rust execution is assumed, not claimed as evidence.

#### Scenario: Both supported executor configurations
- **WHEN** the public full-model consumer runs on the minimum toolchain and current stable using either supported Tokio scheduler configuration
- **THEN** it can create, commit, reopen and read without blocking the async scheduler on file work or requiring a hidden library runtime

#### Scenario: Caller stops waiting during publication
- **GIVEN** a publication job owns its required resources
- **WHEN** its caller cancels the waiting future
- **THEN** the publication retains its lock and owned data until its publication/recovery boundary completes, and a later permitted original-operation lookup can establish the outcome

