## MODIFIED Requirements

### Requirement: Schema-conforming values
Every element and assertion SHALL conform to its selected exact type definition. Values SHALL distinguish absent fields, explicit null, text, booleans, numbers, enumerations, collections, structured values, exact references and artifact references as declared by that schema. Validation SHALL identify the subject, property path and violated definition without mutating accepted state.

#### Scenario: Missing and null are distinct
- **WHEN** a required non-null field is absent in one candidate and explicitly null in another
- **THEN** validation reports the respective absence and nullability violations against the same pinned property definition

#### Scenario: Invalid typed value
- **WHEN** an author supplies text where a boolean, enumeration member or exact reference is required
- **THEN** the library reports the property's type violation and preserves the accepted head

#### Scenario: Descendant fields remain mandatory
- **WHEN** a subtype instance omits an inherited required field or violates its own declared field constraint
- **THEN** assessment applies the resolved inherited and local schema with exact declaration/path diagnostics


### Requirement: Relationship constraints
An assertion SHALL meet its pinned declaration's assignable endpoint types or explicit governed roles, direction, qualifiers and multiplicities in the selected model/baseline. Graph-wide constraints, including absence-dependent rules, SHALL be checked at guarded publication. Invalid endpoints SHALL NOT become accepted dangling links.

#### Scenario: Wrong satisfies endpoint
- **WHEN** UseCase satisfies targets an EventDefinition revision
- **THEN** validation rejects the endpoint even though both records individually satisfy their own schemas

#### Scenario: Concurrent root selection
- **GIVEN** two candidates select different aggregate roots from the same base
- **WHEN** one publishes before the other is admitted
- **THEN** the other conflicts and cannot create an aggregate with two roots

#### Scenario: Both endpoints use declared assignability
- **WHEN** a signature selects ancestor types and its source and target are valid transitive descendants
- **THEN** the same assignability rule used by reference fields accepts both endpoints
- **AND** reversed base-for-descendant assignments and unrelated endpoints remain invalid


### Requirement: Specialised structural semantics
Validation SHALL enforce the deterministic obligations in `ontology-coverage.md`: aggregate roots, declared acyclic decomposition, behaviour contracts, lifecycle/process membership, event/schema references, transformation contracts, UI bindings, architecture mappings and exact evidence/review scope. Modelling a function or process SHALL NOT imply that the Hub executes it.

#### Scenario: Invalid decomposition and legitimate process loop
- **WHEN** an author creates a cycle in an acyclic BusinessFunction decomposition and a separately declared allowed process loop
- **THEN** the first is rejected and the second is assessed by its process rules rather than a universal no-cycles rule

#### Scenario: Function has only a name
- **WHEN** a FunctionDefinition is assessed as complete without its selected input/output, guard, effect, error, body and limit obligations
- **THEN** completeness reports those obligations instead of treating the type label as an executable contract

#### Scenario: Mismatched process state
- **WHEN** a Transition uses a state from another lifecycle or process
- **THEN** validation reports the membership violation and preserves the earlier model

#### Scenario: A differently named subtype cannot escape structural checks
- **WHEN** descendants of aggregate, lifecycle or UI-binding definitions violate an applicable inherited integrity constraint
- **THEN** validation enforces that constraint alongside descendant constraints rather than dispatching only on the concrete type name

#### Scenario: Inspect stored business code without executing it
- **WHEN** a Body or native rule artifact declares language, entrypoint, conditions, effects and source-bound limits
- **THEN** capture, assessment, commit, reopen and ordinary inspection preserve that content and links without invoking its business code, timers, external actions or workflow
- **AND** the report labels structural model checks separately from unexecuted business predicates


### Requirement: Explicit completeness and applicability
Validation SHALL distinguish record/graph integrity from purpose-specific completeness. Draft models SHALL retain explicit permitted coverage gaps. Gaps SHALL NOT waive invalid types, unresolved required references, competing ownership or immutable-revision checks. A complete result SHALL meet every applicable obligation; human adequacy assessment SHALL remain separate.

#### Scenario: Incomplete aggregate draft
- **WHEN** a draft profile permits an aggregate with no root and records that missing obligation
- **THEN** the candidate can remain explicitly incomplete, while a complete aggregate assessment requires exactly one root and no profile accepts two roots

#### Scenario: Families do not apply to this library
- **WHEN** the self-model records reasoned UI/runtime-AI inapplicability
- **THEN** its completeness report preserves those reasons and does not remove those families from the supported ontology

#### Scenario: Exact sourced omission is valid but incomplete
- **WHEN** a gap-permitting profile sees a genuinely required top-level property omitted with a resolving source-bound gap at that exact subject and path and a nonblank reason
- **THEN** assessment and the persisted/reopened baseline retain valid-but-incomplete status and that reason

#### Scenario: Invalid gap claim
- **WHEN** gap permission is absent or a gap has the wrong subject/path, no reason, or unresolved provenance
- **THEN** the missing obligation remains invalid and publication leaves the head unchanged

#### Scenario: A supplied malformed value is never excused
- **WHEN** an attached gap accompanies a wrong type/endpoint, forbidden null, empty positive-minimum list, unresolved artifact or undeclared property
- **THEN** assessment rejects the supplied malformed value without invented defaults, artifacts or evidence


## ADDED Requirements

### Requirement: Planned and implemented realization are distinct
Module and Adapter definitions SHALL explicitly distinguish planned from implemented realization. A planned definition SHALL be able to be complete without implementation artifacts where its contract permits that absence. An implementation claim SHALL require its real artifact bindings or remain explicitly incomplete only where a selected profile permits a genuine sourced omission.

#### Scenario: Honest planned architecture
- **WHEN** an author describes a planned module and adapter with their required design contracts and no implementation files
- **THEN** the planned definitions can assess complete without fabricated files or evidence

#### Scenario: Implementation claim lacks an artifact
- **WHEN** an implemented definition omits its required artifact binding
- **THEN** the result is invalid without a permitted exact sourced gap, or explicitly incomplete with one
- **AND** supplying an empty positive-minimum list or a nonresolving artifact is invalid even with a gap

#### Scenario: Presence is not execution
- **WHEN** an implemented definition binds a retained source or code artifact
- **THEN** the binding resolves exactly but does not establish code execution, policy enforcement, TestRun success or human acceptance

