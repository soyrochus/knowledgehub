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
Opening SHALL require a versioned workspace configuration identifying an existing native directory, exact model scope and trusted local actor/read grant. Model aliases SHALL select only configured entries. Requests and model content SHALL NOT override directories, actors, policies or capabilities.

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
Startup SHALL agree on the client protocol, library contract, record/request codecs, catalogue and finite limits before model reads. Incompatible versions, malformed messages and uncorrelated replies SHALL fail explicitly. Protocol output SHALL be separate from diagnostics.

#### Scenario: Incompatible host is supplied
- **WHEN** startup reports a different codec or unsupported protocol
- **THEN** the client refuses exploration with a compatibility diagnostic and performs no format conversion or mock fallback

### Requirement: Exact native value and error mapping
Transport results SHALL preserve qualified identities, opaque revisions, tagged values, missing-versus-Null, integer/decimal precision, original content identity, assertion origins and native error distinctions. Source ranges SHALL retain offsets and full-versus-partial verification status.

#### Scenario: Inspect values outside JavaScript numeric precision
- **WHEN** a definition contains a large Integer, exact Decimal, absent field and explicit Null
- **THEN** each arrives unchanged and remains distinguishable in JSON and inspection output

#### Scenario: Read a source excerpt
- **WHEN** an artifact range is returned
- **THEN** the response identifies the exact source, byte range and whether the full original content hash was verified

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
