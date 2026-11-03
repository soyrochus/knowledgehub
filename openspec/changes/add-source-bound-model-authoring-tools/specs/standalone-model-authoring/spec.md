# Spec Delta

## Purpose

Let a developer compile a reviewed source-bound plan into a fresh native Hub, recover interrupted publication and inspect the exact result without Modelarium or an inference engine inside KnowledgeHub. Trace: MM-SL-01/C3, ontology §21.3–21.10 and KH-DR-01–19.

## ADDED Requirements

### Requirement: Discover and compile one selected contract
Standalone tools SHALL expose the actual selected catalogue/profile and compile a versioned typed neutral model plan into native records, assertions, artifacts, interpretations and gaps. Compilation SHALL use exact schema/parent identities and public native assessment, including direct lifecycle subjects and shared assignability. Unsupported or ambiguous input SHALL produce precise diagnostics rather than guessed fields or fallback semantics.

#### Scenario: Compile a reviewed rich plan
- **WHEN** a developer supplies source bindings and a typed plan against the current registered contracts
- **THEN** preparation resolves stable plan keys to exact native identities and reports schema, reference, ownership and completeness results
- **AND** the compiled subject and subtype semantics agree with native validation

#### Scenario: Plan contains an unknown or obsolete shape
- **WHEN** a plan contains an unsupported contract, undeclared field, ambiguous identity or inverse-only lifecycle subject
- **THEN** compilation fails at its plan/source path without converting an earlier payload or inventing a replacement definition

### Requirement: Explicit safe fresh destination
Creation SHALL require an explicit source root, new destination, work directory and host authority context. Existing destinations SHALL NOT be reset, overwritten or silently reinitialized. Publication SHALL use supported native APIs, current policy and exact guards; accepted backend files SHALL NOT be edited directly. The tool SHALL NOT read or alter unrelated stores.

#### Scenario: Destination already exists
- **WHEN** creation is requested for a nonempty or previously initialized destination
- **THEN** it refuses fresh creation and preserves its bytes, scope, baseline and operation outcomes
- **AND** only explicit recovery of the same recorded run can continue that run

#### Scenario: Required native assessment fails
- **WHEN** a supplied reference, inherited constraint, lifecycle subject or gap claim is invalid
- **THEN** the model change is not published and the tool reports the exact diagnostic
- **AND** it does not fabricate defaults, implementation files or successful evidence

### Requirement: Preserve original publication intent
Before each native dispatch, tooling SHALL durably retain the actor, scope, operation identity, exact guarded request and fingerprint needed for recovery. Retry SHALL reconcile the original outcome before stale-base assessment and SHALL NOT create a new identity to hide uncertainty. Changed intent and denied recovery SHALL remain explicit failures.

#### Scenario: Response lost after commit
- **WHEN** publication commits and the response is lost before the caller records success
- **THEN** resume resolves the original actor-bound operation and receipt without a duplicate mutation

#### Scenario: Incomplete run is resumed with another plan
- **WHEN** recovery is requested with different source bytes, plan, actor or operation intent
- **THEN** the tool refuses changed-intent reuse and preserves the original run for authorized reconciliation

### Requirement: Reopen and verify exact output
A ready result SHALL identify the destination scope, current contract/profile, exact resulting head, named baseline, source manifest and generation receipts. Verification SHALL drain/reopen the native Hub and check original bytes, resolving anchors and representative exact traces independently of compilation success. Read budgets and truncation SHALL be disclosed.

#### Scenario: Fresh process verifies the result
- **WHEN** verification reopens a generated Hub without the original agent conversation or Modelarium checkout
- **THEN** it retrieves the named baseline, original sources and requested native traces and resolves the known generation outcomes
- **AND** it distinguishes denied, unavailable, uncertain and positively absent results

#### Scenario: A read budget is exhausted
- **WHEN** a query cannot exhaust its relevant records within a configured finite budget
- **THEN** verification reports incomplete coverage and the reached bound rather than declaring the model complete

### Requirement: Honest partial creation and finite resources
Tools SHALL impose documented finite source, plan, payload and read limits and fail explicitly when exceeded. A partially initialized destination SHALL be labelled incomplete until its final model/baseline verification succeeds. The creation manifest SHALL enumerate its known native operations and exact receipts without pretending to discover arbitrary native history.

#### Scenario: Initialization stops between native commits
- **WHEN** catalogue/source initialization succeeds but final model publication is interrupted
- **THEN** the retained run identifies completed, pending and uncertain operations and does not present the destination as a ready model
- **AND** resume uses the recorded guards and original outcomes without deleting the destination

### Requirement: Description import never executes the application
Capture, compilation, assessment, commit and inspection SHALL retain source-bound rule/code metadata and design policies without invoking their business entrypoints, scripts, timers or external actions. Reports SHALL distinguish executed native integrity checks, interpretation review, implementation evidence and human acceptance. No rule execution engine or application runtime SHALL be added.

#### Scenario: Retained source contains an entrypoint
- **WHEN** a rule artifact includes runnable code and language/entrypoint metadata
- **THEN** the artifact and its condition/effect/scenario bindings remain exact after reopen while its entrypoint is never invoked by the import or verification tools
