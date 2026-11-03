# Spec Delta

## Purpose

Let a model author describe product-wide experience structure and exact requirement traces without duplicating story identities or inventing an umbrella journey. Trace: MM-SL-01/C1, O-01/O-02, KH-DR-01–06/08–13/15–19 and ontology §§5–8/14/17/21.4.

## ADDED Requirements

### Requirement: Multi-journey experience scope
A StoryMap SHALL declare one or more exact Journey revisions. Each selected activity SHALL declare its participating journeys within that map's scope. A complete map SHALL preserve explicit activity/task ownership without inventing a source journey or deriving ownership from screen order.

#### Scenario: Product map spans distinct journeys
- **WHEN** an author supplies one map spanning entry, collaboration and return journeys
- **THEN** its activities and tasks resolve against those exact journeys in one model
- **AND** no synthetic journey or duplicate story is required

#### Scenario: Activity escapes the map
- **WHEN** a map selects an activity whose declared journey is outside its scope
- **THEN** assessment reports the exact ownership/scope violation and publication leaves the accepted head unchanged

### Requirement: Identified story placements
Story placement SHALL be a versioned model definition connecting one StoryMap, one UserTask and one UserStory with explicit order. Its task SHALL belong to an activity selected by the map. Several placements SHALL be able to reference the same story while retaining independent placement identities and source bindings.

#### Scenario: One story participates in multiple tasks
- **WHEN** two placements in permitted tasks refer to the same UserStory revision
- **THEN** queries distinguish the placements and resolve one shared story identity

#### Scenario: Placement task has another owner
- **WHEN** a placement points to a task outside its selected map's activities
- **THEN** validation rejects the ownership mismatch instead of silently inserting or moving the task

### Requirement: Outcome cuts select placements
An OutcomeCut SHALL select exact placements belonging to its map and record its outcome, exclusions and prerequisite cuts. Cuts SHALL NOT create competing copies of story definitions. Prerequisite cuts SHALL belong to the same map and form an acyclic graph.

#### Scenario: Cumulative cuts reuse placed stories
- **WHEN** two outcome cuts select an overlapping placement set
- **THEN** each cut retains its exact selection and shared stories retain one identity

#### Scenario: Cyclic or foreign cut prerequisite
- **WHEN** a cut names a prerequisite from another map or introduces a prerequisite cycle
- **THEN** the candidate is rejected with the affected cut references and no partial publication

### Requirement: Direct scenario requirement traces
The ontology SHALL provide a qualified Scenario illustrates Requirement signature. Each assertion SHALL retain its exact endpoints, identity, source support and review status. Forward and reverse bounded reads SHALL expose the trace without creating a TestCase or claiming an execution.

#### Scenario: Read a promised behavior and its examples
- **WHEN** a reader follows illustrates in reverse from a Requirement
- **THEN** the result identifies its distinct Scenario assertions and preserved source fragments
- **AND** no successful TestRun or human acceptance is inferred

#### Scenario: Wrong illustrates target
- **WHEN** an author asserts that a Scenario illustrates a DataSchema through that signature
- **THEN** the endpoint violation is rejected even if both records are individually valid

### Requirement: Declared story prerequisites
A qualified UserStory dependsOn UserStory signature SHALL distinguish required from conditional prerequisites. Conditional assertions SHALL include a sourced condition. Self-dependencies and cycles formed by unconditional required assertions SHALL be rejected. The model SHALL NOT execute conditions or infer runtime sequencing.

#### Scenario: An alternative prerequisite remains conditional
- **WHEN** a source says a story consumes another story's result only under a named condition
- **THEN** the assertion retains that condition and is not converted into an unconditional prerequisite

#### Scenario: Required prerequisites form a cycle
- **WHEN** required story dependencies would make a story transitively require itself
- **THEN** assessment identifies the cycle and rejects the complete candidate

### Requirement: Generic vocabulary and source refinement
Experience schemas/signatures SHALL have domain-independent meanings and exact decision/source provenance. Domain names, IDs, catalogue counts, named phases and business fields SHALL remain model or fixture data. Source draft coverage SHALL remain distinguishable from newly authored schema refinements.

#### Scenario: Reuse the same contracts in another application
- **WHEN** an unrelated domain supplies different story, journey and task definitions
- **THEN** the same schemas and validators accept its valid structure without production-code branches or vocabulary changes

#### Scenario: A reference catalogue supplies application IDs
- **WHEN** reference verification checks the source's expected story identifiers and counts
- **THEN** those obligations come from its coverage input rather than built-in ontology rules

### Requirement: Exact guarded experience publication
Experience records, assertions, provenance and required selections SHALL publish coherently through the ordinary API under current authority and exact guards. Reopen and original-operation recovery SHALL preserve the exact accepted map/cut/trace. Revised contracts SHALL have explicit identities; obsolete payloads SHALL NOT be translated.

#### Scenario: Response is lost after publication
- **WHEN** the complete map change commits but the caller loses the response
- **THEN** recovery with the original actor and operation intent returns its original receipt without a duplicate map or placement

#### Scenario: Reopen an earlier current-contract baseline
- **WHEN** the map is revised after baseline A and the Hub is reopened
- **THEN** A still resolves its original placements, cuts and assertions without selecting newer endpoints

#### Scenario: Submit the removed single-journey payload
- **WHEN** a caller supplies obsolete map/activity fields to the revised schema
- **THEN** the request is rejected explicitly without a legacy reader, alias or automatic conversion
