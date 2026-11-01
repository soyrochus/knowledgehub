## REMOVED Requirements

### Requirement: Durable acknowledgement covers content and receipt
**Reason**: The original requirement explicitly mandated superseded files/1 metadata. Current-only durability and schema checks replace that contract.
**Migration**: Use the current backend's persistence/recovery protocol and a fresh current store; no older receipt decoder is provided.

## ADDED Requirements

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


