# Native model history delta

## Purpose

Let a currently authorised consumer inspect retained original publications and their exact affected revisions without impersonating their publisher or depending on backend layout. Trace C8, KH-DR-02/07/10–13/15/17/19/20 and KH-US-06–10/21/26; ontology §§5/21.3–21.4/21.6–21.7/21.10.

## ADDED Requirements

### Requirement: Separate current history authority
Publication-history disclosure SHALL require current scoped read and explicit history-read authority. The caller SHALL remain the current actor; historical actors and modelled roles SHALL confer no permission. These reads SHALL NOT grant mutation, review or operation-recovery authority.

#### Scenario: Reader inspects a different publisher
- **GIVEN** actor A published operation O and actor B currently holds read and history-read permission for that scope
- **WHEN** B reads O through the history interface
- **THEN** the result retains A as publisher while permission is evaluated for B, without impersonation or mutation

#### Scenario: Ordinary read is insufficient
- **WHEN** a caller with ordinary model-read permission but without history-read permission requests a publication or history page
- **THEN** disclosure is denied before revealing publication existence or metadata

#### Scenario: Authority is revoked during paging
- **GIVEN** a permitted reader received a history continuation
- **WHEN** either required permission is revoked or current policy evaluation fails before continuation
- **THEN** the request reports denial or policy unavailability respectively and discloses no further history

### Requirement: Original publication lookup at an exact snapshot
A permitted lookup SHALL identify one operation within one model-branch scope and accepted snapshot. It SHALL return original receipt metadata and the exact affected-reference count for a publication accepted at or before that snapshot, or authoritative non-membership at that snapshot. It SHALL NOT replace the original head or actor with current values.

#### Scenario: Resolve a handover after later edits
- **GIVEN** a handover identifies O, which published A to B, and the Hub later advanced to C
- **WHEN** another permitted reader resolves O at C after reopen
- **THEN** original scope, operation, actor, fingerprint/version, A, B, sequence and affected count are retained, with C identified only as the read anchor

#### Scenario: Publication is later than the anchor
- **GIVEN** O was accepted after snapshot A
- **WHEN** the reader resolves O at A
- **THEN** the result establishes only that O was not published in A's accepted ancestry, without claiming global operation absence or authorising a new mutation

### Requirement: Truthful failure and absence outcomes
History SHALL distinguish confirmed non-membership at a verified anchor from denied access, invalid requests, unavailable or unsupported selections, exhausted resources and integrity/recovery failures. An empty or incomplete response SHALL NOT imply confirmed absence. Unknown or unavailable anchors SHALL NOT silently resolve to current head.

#### Scenario: Storage cannot establish membership
- **WHEN** an original receipt or its accepted-history binding cannot be verified because of corruption, recovery uncertainty or storage failure
- **THEN** the request returns the corresponding failure rather than non-membership, an empty complete history or a fabricated receipt

#### Scenario: Unknown snapshot
- **WHEN** the requested snapshot is not an available accepted snapshot of the requested scope
- **THEN** the request reports an explicit selection failure without substituting another snapshot

### Requirement: Snapshot-pinned publication discovery
A permitted reader SHALL discover accepted native publications in descending native sequence order, pinned to one exact snapshot. Each page SHALL identify its scope, anchor, order, completeness and continuation. Only accepted publications in that ancestry SHALL appear; head movement SHALL NOT change an existing traversal.

#### Scenario: Discover publications without an application audit
- **GIVEN** a Hub contains pre-registration publications with no SQL journal or application audit rows
- **WHEN** a permitted consumer pages its native history
- **THEN** all publications within the completed requested ancestry are discoverable with original identities, independently of application activity

#### Scenario: Append while reading
- **GIVEN** the first page is pinned at B
- **WHEN** another actor commits C before subsequent pages are requested
- **THEN** continuation contains only B's original accepted ancestry in stable order, without duplicates or skipped publications caused by C

#### Scenario: Reach the initial boundary
- **WHEN** discovery reaches the earliest accepted snapshot with no native publication receipt
- **THEN** it reports the completed boundary without inventing a creator receipt, timestamp or application event

### Requirement: Bounded publication and affected-reference pages
History requests SHALL enforce documented item, work and encoded-output limits with actual work counters. Original affected references SHALL be separately pageable in their retained order with exact counts and completeness. A response SHALL NOT claim an entire receipt when references are omitted. Oversized indivisible values SHALL produce an explicit limit failure rather than silent truncation.

#### Scenario: Large publication and history
- **GIVEN** many later publications and an original receipt affecting more references than one page permits
- **WHEN** a reader requests its metadata and affected references using finite limits
- **THEN** bounded pages preserve the original references and order, disclose remaining work, and do not require full-state, all-receipt or artifact materialisation

#### Scenario: Budget cannot advance
- **WHEN** a request cannot fit the next indivisible result or minimum verification work within its permitted limits
- **THEN** it reports an explicit limit outcome without an endless non-advancing continuation or false completeness

### Requirement: Authority-bound finite continuations
History continuations SHALL be opaque, finite and bound to the calling actor, model-branch scope, exact anchor and complete query intent. Every use SHALL recheck current permission. Expiry, release, capacity and invalidation SHALL be explicit; a token SHALL confer no independent authority or silently restart at current head.

#### Scenario: Continuation is moved or altered
- **WHEN** a different actor or scope uses a continuation, its query intent is changed, or its lifetime has expired
- **THEN** continuation fails explicitly without returning a page from another actor, scope, query or snapshot

#### Scenario: Restart after cursor expiry
- **WHEN** a process-local cursor is lost after reopen
- **THEN** the consumer can start a new bounded request at the retained exact anchor, while the old token fails explicitly

### Requirement: Compose history with existing exact model reads
Native publication history SHALL preserve exact revision and baseline identities so permitted consumers can use existing historical reads, predecessor references and named-baseline comparison. A publication sequence SHALL NOT be treated as a baseline label, source revision, review decision, test execution or human acceptance.

#### Scenario: Recover original and edited meaning
- **GIVEN** baseline A pins an original definition and source while B selects a coherent successor
- **WHEN** a permitted reader resolves their publications, reads their exact definitions and compares A/B after reopen
- **THEN** each baseline retains its original content, predecessor and source bindings, with native publication distinct from source history, execution and review

### Requirement: Preserve original publications through maintenance
History reads SHALL be semantically non-mutating. Reopen and supported packing/index maintenance SHALL preserve original publication identities, metadata, affected order and exact selection meaning. Physical maintenance SHALL NOT create a native publication or erase accepted history.

#### Scenario: Reopen and compact
- **WHEN** a current-contract fixture is closed, reopened and compacted using supported maintenance
- **THEN** the same pinned history and affected-reference traversal returns equivalent original values and unchanged semantic heads, without maintenance events masquerading as publications

### Requirement: Preserve actor-bound mutation recovery
The history reader SHALL be separate from existing actor-bound operation-result lookup and retry admission. Original request fingerprints, recovery results and operation-reuse rules SHALL remain unchanged. Successful history disclosure SHALL NOT permit another actor to retry the publisher's operation as their own.

#### Scenario: Disclosure does not permit reuse
- **GIVEN** B can inspect A's publication O through current history permission
- **WHEN** B uses the actor-bound recovery API for O or submits B-bound mutation intent using O's key
- **THEN** the existing operation-reuse refusal remains enforced, while A's permitted exact retry still returns the original receipt without another publication
