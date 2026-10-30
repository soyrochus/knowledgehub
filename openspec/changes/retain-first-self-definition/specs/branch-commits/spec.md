# Branch commits

## Purpose

Allow an authorised host to commit one coherent model-branch change and establish its original outcome, preserving exact intent, graph consistency and history across concurrent work, repeated requests and lost responses.

## ADDED Requirements

### Requirement: Explicit scoped admission and non-mutating validation
The library SHALL accept typed change intent identifying one model branch, expected head, operation identity and relevant exact revisions. It SHALL validate a candidate against a coherent base without mutating accepted content. Current scoped host authority and applicable policy SHALL be checked again when a new change is admitted for publication.

#### Scenario: Validating a candidate does not commit it
- **WHEN** a permitted caller validates the complete self-description seed against head A
- **THEN** the library returns a base-bound candidate or structured violations while head A and its accepted records remain unchanged

#### Scenario: Authority changes after validation
- **GIVEN** a seed candidate validated at A but the host now denies its caller write access
- **WHEN** the caller requests a new commit
- **THEN** the library denies admission and no definitions, source bindings, head change or successful receipt become accepted

### Requirement: Coherent atomic branch publication
The library SHALL publish all accepted document, definition, assertion, provenance, history, branch-head and operation-result changes for one change set atomically. Success SHALL identify the committed snapshot/head and exact affected revisions. Failure before commitment SHALL leave the accepted head unchanged; an uncertain response SHALL not be reported as confirmed failure.

#### Scenario: Complete seed commits together
- **WHEN** the valid seed is committed from head A
- **THEN** head B resolves all required seed content and provenance plus the original operation receipt, with no independently visible partial definition

#### Scenario: Candidate fails a late validation or storage check
- **WHEN** the new candidate cannot meet its required guards or fails before it is committed
- **THEN** head A remains the accepted model and no durable success is reported for a partial candidate

### Requirement: Publication guards preserve graph-wide consistency
The library SHALL check the expected head and relevant revision and graph-wide invariants at new-change publication. A stale candidate SHALL return an explicit conflict without silently rebasing or replacing accepted content. Competing changes SHALL not jointly violate constraints that depend on multiple records or relationship absence.

#### Scenario: Two writers start from the same head
- **GIVEN** two valid new operations were prepared from A
- **WHEN** one publishes B before the other is admitted
- **THEN** the other returns a stale-head conflict without mixing its candidate into B, and the caller can retain that candidate for explicit reconsideration

#### Scenario: An intervening relationship changes validation conditions
- **GIVEN** the selected profile constrains a relationship set and a candidate was valid under A
- **WHEN** an intervening commit changes that set before the candidate publishes
- **THEN** the candidate cannot publish using only its earlier record-level validation; it conflicts or fails the current applicable graph guards

### Requirement: Exact operation retries preserve original intent
After current access checks, the library SHALL resolve a committed operation by its identity, branch scope, actor and canonical request fingerprint before treating its original base as a new stale write. The exact retry SHALL return the original result without another mutation. Reuse of that operation identity for different intent SHALL be rejected explicitly.

#### Scenario: Original base is old but the operation already committed
- **GIVEN** operation O committed A to B and the branch later advanced to C
- **WHEN** its permitted actor retries O with its original scope and exact request fingerprint
- **THEN** the library returns O's original B receipt and creates no new revision or head change

#### Scenario: Operation identity reused for different content
- **GIVEN** O is already committed
- **WHEN** a request reuses O's full scoped operation identity with changed definition content, actor or other fingerprinted intent
- **THEN** the library rejects the incompatible reuse without returning another actor's result or committing the changed intent

#### Scenario: Retry cannot bypass revoked access
- **GIVEN** O previously committed but the caller no longer has access to its result scope
- **WHEN** it retries O or requests its stored result
- **THEN** the library applies current disclosure/admission policy before revealing the receipt or performing any change

### Requirement: Original operation outcome lookup
The library SHALL provide a permitted lookup of the original operation's status/result without requiring resubmission of a mutation. It SHALL distinguish an established committed receipt, established absence after authoritative recovery, and inability to establish an outcome. Cancellation or a missing response SHALL not authorise a fresh operation identity automatically.

#### Scenario: Lost acknowledgement after publication
- **GIVEN** O's complete snapshot and receipt committed but the caller lost the response
- **WHEN** the host reopens and queries O under current authority
- **THEN** it obtains the original receipt and exact committed head without re-executing O

#### Scenario: Storage cannot establish the outcome
- **WHEN** outcome lookup cannot inspect a valid authoritative recovered state
- **THEN** it reports an unavailable or uncertain outcome rather than claiming that O did not commit or directing a blind new submission

#### Scenario: Recovered operation is definitely absent
- **GIVEN** authoritative recovery has established a valid branch and complete required operation history with no committed O
- **WHEN** its permitted actor looks up O
- **THEN** the library can report that established absence, while preserving the distinction from an unreachable or incompletely recovered store
