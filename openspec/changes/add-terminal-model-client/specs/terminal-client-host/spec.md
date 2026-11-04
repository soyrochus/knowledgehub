# Terminal client host

## Purpose

Provide local terminal consumers with permitted, exact native KnowledgeHub reads through a repository-owned persistent host, independent of Modelarium. Trace: KH-DR-02/04/07/12/15/17–22; KH-US-09/10/27/29; ontology §§21.2–21.3/21.7/21.9–21.10.

## ADDED Requirements

### Requirement: Repository-owned local host
The terminal client SHALL launch and manage a persistent local host containing the KnowledgeHub library. Model operations SHALL use the public library API. The delivered client SHALL require neither Modelarium files, configuration, SQL, accounts nor a running Modelarium process.

#### Scenario: Explore without the source applications
- **WHEN** a user opens a prepared model from an extracted distribution with both source repositories unavailable
- **THEN** the client starts its packaged host and reads the native model through the library

### Requirement: Explicit trusted model configuration
Opening SHALL require a versioned workspace identifying an existing directory, exact scope, trusted actor and explicit Open/Read grants. ReadHistory SHALL be a separate optional grant. Model aliases SHALL select only configured entries. Requests, model content and supplied baseline references SHALL NOT override directories, actors, policies or capabilities.

#### Scenario: Select a configured alias
- **WHEN** the reader selects the FruitTrucks alias
- **THEN** the host opens its configured directory and scope and reports their semantic identity

#### Scenario: A request attempts to grant itself access
- **WHEN** an ordinary query includes an actor, directory or permission override
- **THEN** the host rejects it without opening another scope or expanding authority

### Requirement: Semantic read-only boundary
The host SHALL admit only opening and documented exploration operations. It SHALL expose no model creation, ontology registration, candidate validation, commit, import, reset or deletion. Native recovery, locking and derived-index preparation SHALL remain library-managed and SHALL NOT be described as semantic model edits.

#### Scenario: Attempt publication through the exploration host
- **WHEN** a caller submits a commit or an unsupported write operation
- **THEN** the host refuses it and the model head, records, original artifacts and operation history remain unchanged

#### Scenario: Prepare missing derived navigation data
- **WHEN** a permitted lookup needs a bounded index preparation supported by the library
- **THEN** the host prepares it or reports readiness/capacity without publishing a model revision

### Requirement: Explicit compatible transport
Startup SHALL agree on the client protocol, library contract, record/request codecs, catalogue, native history projection/method support and finite limits before model reads. Incompatible versions, malformed messages and uncorrelated replies SHALL fail explicitly. Protocol output SHALL be separate from diagnostics.

#### Scenario: Incompatible host is supplied
- **WHEN** startup reports a different codec or unsupported protocol
- **THEN** the client refuses exploration with a compatibility diagnostic and performs no format conversion or mock fallback

#### Scenario: Equal package version lacks native history support
- **WHEN** a host reports version 0.10.0 but lacks the required history methods or knowledgehub-history/1 projection
- **THEN** startup reports incompatibility without substituting application audit data, actor-bound recovery or backend-file parsing

### Requirement: Exact native value and error mapping
Transport results SHALL preserve qualified identities, opaque revisions, tagged values, missing-versus-Null, integer/decimal precision, original content identity, assertion origins and native error distinctions. Source ranges SHALL retain offsets and full-versus-partial verification status.

#### Scenario: Inspect values outside JavaScript numeric precision
- **WHEN** a definition contains a large Integer, exact Decimal, absent field and explicit Null
- **THEN** each arrives unchanged and remains distinguishable in JSON and inspection output

#### Scenario: Read a source excerpt
- **WHEN** an artifact range is returned
- **THEN** the response identifies the exact source, byte range and whether the full original content hash was verified

#### Scenario: Preserve history counters and resource failures
- **WHEN** a history response contains original publication sequences, affected offsets or a native resource-limit failure
- **THEN** transport preserves exact values and typed resource/work data without JavaScript rounding, truncation or interpreting non-membership as a failure

### Requirement: Bounded exact-selection reads
Enumeration, neighbours, source reads, search and traversal SHALL enforce finite response/work limits and retain their exact selection and native coverage signals. Continuations SHALL remain bound to their original intent. A full-model state dump SHALL NOT be required for ordinary browsing or diagram generation.

#### Scenario: The model is larger than one response
- **WHEN** a browser requests a page of a large type group
- **THEN** the host returns a bounded page and continuation or explicit reached limit without loading the entire model into the client

### Requirement: Current read authority
The host SHALL enforce current trusted read authority on every library operation and continuation. A selected baseline, record reference or continuation SHALL confer no additional permission. External references SHALL remain unresolved unless the configured permitted scope supports them.

#### Scenario: Authority is withdrawn during exploration
- **WHEN** a later page or source request is made after the host closes the grant
- **THEN** it is denied rather than served from a stale client cache

### Requirement: Explicit current history authority
Publication lookup, discovery, affected pages, continuation and release SHALL require current scoped Read plus ReadHistory. Historical attribution SHALL NOT change the reader's identity or authority. History permission SHALL NOT grant mutation or actor-bound operation recovery. Cached history disclosure SHALL obey current permission.

#### Scenario: Ordinary read is insufficient for publication disclosure
- **WHEN** an Open/Read-only reader requests publication metadata or retained-baseline discovery
- **THEN** that request is denied before disclosure while ordinary permitted browsing remains usable

#### Scenario: Reader B inspects publisher A
- **WHEN** B has current Read/ReadHistory authority and resolves a known publication by A
- **THEN** original A attribution is retained without impersonation and operation-recovery requests remain outside the host's allowlist

#### Scenario: History is revoked while ordinary reading remains permitted
- **WHEN** ReadHistory is withdrawn or policy evaluation fails before a later page, release or cached history disclosure
- **THEN** the host returns denial or policy unavailability, discloses no history and retains only the ordinary browsing authority still granted

### Requirement: Original publication projections
The host SHALL expose original publication lookup, descending accepted discovery and exact affected-reference pages at an accepted snapshot anchor. It SHALL preserve original metadata, counts, vector order, repetitions, offsets, published status, completeness, initial boundary and work. A header SHALL NOT be presented as a complete receipt.

#### Scenario: Resolve a handover after later changes
- **WHEN** B resolves A's known operation at a later accepted anchor
- **THEN** original scope, actor, operation, fingerprint/version, previous/resulting heads, sequence and affected count remain unchanged and the anchor is separately identified

#### Scenario: Reach an initial or zero-affected boundary
- **WHEN** discovery reaches the initial no-receipt snapshot or a published operation has zero affected references
- **THEN** it reports the corresponding complete boundary without inventing a publication or treating an empty affected vector as non-membership

#### Scenario: Operation was published after the anchor
- **WHEN** the selected operation is absent from a verified anchor's ancestry
- **THEN** it returns explicit NotPublishedAt, or published=false for affected lookup, without claiming current/global operation absence

### Requirement: Truthful bounded history transport
History SHALL retain native item/node/storage/output limits and actual work counters, while enforcing separate finite wire and aggregate-action limits. Oversized indivisible values SHALL fail explicitly. Native output-byte accounting SHALL NOT be relabelled as wire size. Unknown anchors, unavailable or corrupt history and resource failures SHALL NOT become empty success.

#### Scenario: Native result fits but transport cannot encode it within its bound
- **WHEN** decimal-string mapping and envelope overhead exceed the selected wire limit
- **THEN** the host reports explicit capacity/resource failure without dropping fields, shortening identifiers or changing native limits

#### Scenario: Cold history needs unavailable derived pack offsets
- **WHEN** native history returns index-not-ready or a cold-read resource limit
- **THEN** the host preserves that outcome and work without hidden full-pack repair, model scanning or false non-membership

### Requirement: Authority-bound history continuation lifetime
History continuations SHALL retain reader, scope, exact anchor, operation kind/ID and complete limits/intent. They SHALL share the native finite cursor pool and expiry. Abandoned cursors SHALL be released under current authority or removed by handle drain. Process loss SHALL invalidate tokens without changing retained history.

#### Scenario: Cursor is changed, expired or moved
- **WHEN** a continuation's limits, anchor, kind, operation, actor or scope differ, or its token is tampered with, expired or released
- **THEN** the host returns the corresponding explicit authority, cursor or resource failure without history disclosure or a fallback to current head

#### Scenario: Append or packing occurs between pages
- **WHEN** the accepted head advances or supported packing changes physical locations
- **THEN** continuing the original pinned request retains its original sequence and attribution or fails explicitly without retargeting the anchor

#### Scenario: Host restarts during history exploration
- **WHEN** the process-local history token is lost
- **THEN** the client starts an explicitly fresh bounded query at the retained anchor and does not present the old token as reusable

### Requirement: Managed session lifetime
The client SHALL retain host handles during interactive exploration, release abandoned continuations, stop new work on exit and drain admitted reads before terminating the host. Cancellation, host loss and invalid cursors SHALL preserve the selected reference and give an explicit recovery path.

#### Scenario: Host exits during a source read
- **WHEN** the pipe closes before a complete response arrives
- **THEN** the interface reports the failed read without displaying partial bytes as complete and offers reopening the same selection

#### Scenario: Continue after restart
- **WHEN** a user resumes an old navigation position after reopening the host
- **THEN** the client starts a fresh bounded request against the retained exact selection instead of reusing an invalid cursor

### Requirement: Honest native storage opening
Missing directories, wrong scopes, incompatible stores, native integrity failures, unsupported filesystems and ownership conflicts SHALL produce distinct failures. Opening SHALL NOT create an empty replacement, decode historical codecs, bypass locks or silently switch stores.

#### Scenario: Historical store or busy destination
- **WHEN** the configured destination is incompatible or a required lock cannot be acquired
- **THEN** exploration stops with the corresponding diagnostic and leaves accepted model content unchanged
