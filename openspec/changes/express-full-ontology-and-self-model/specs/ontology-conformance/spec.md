## Purpose

Let authors express every declared ontology family with meaningful typed content,
exact provenance and coherent graph rules through the governed public API.
Trace: KH-DR-01–09/12/15–18; KH-US-19/20; ontology §§5–17, 21.4–21.6.

## ADDED Requirements

### Requirement: Schema-conforming values
Every element and assertion SHALL conform to its selected exact type definition. Values SHALL distinguish absent fields, explicit null, text, booleans, numbers, enumerations, collections, structured values, exact references and artifact references as declared by that schema. Validation SHALL identify the subject, property path and violated definition without mutating accepted state.

#### Scenario: Missing and null are distinct
- **WHEN** a required non-null field is absent in one candidate and explicitly null in another
- **THEN** validation reports the respective absence and nullability violations against the same pinned property definition

#### Scenario: Invalid typed value
- **WHEN** an author supplies text where a boolean, enumeration member or exact reference is required
- **THEN** the library reports the property's type violation and preserves the accepted head

### Requirement: Typed kernel subjects
Kernel identities, exact element revisions, relationship assertions, source anchors and representation bindings SHALL be addressable through public exact references. Relationships whose declared endpoints use these kernel kinds SHALL resolve them directly. The library SHALL NOT require shadow elements that duplicate a native subject's identity or provenance.

#### Scenario: Anchor supports an assertion
- **WHEN** a source anchor supports an identified satisfies assertion through supportsAssertion
- **THEN** both exact kernel endpoints and the assertion's own revision/provenance resolve after commit and reopen

#### Scenario: Conflicting assertions remain inspectable
- **WHEN** one identified relationship assertion contradicts another
- **THEN** the relationship resolves exact assertion endpoints without confusing them with ordinary element endpoints

#### Scenario: Represent an exact element revision
- **WHEN** a RepresentationBinding represents a Requirement revision
- **THEN** it identifies that actual revision and its exact representation fragment, rather than a fabricated ElementRevision copy

### Requirement: Relationship constraints
An assertion SHALL meet its pinned declaration's endpoint kinds or roles, direction, qualifiers and multiplicities in the selected model/baseline. Graph-wide constraints, including absence-dependent rules, SHALL be checked at guarded publication. Invalid endpoints SHALL NOT become accepted dangling links.

#### Scenario: Wrong satisfies endpoint
- **WHEN** UseCase satisfies targets an EventDefinition revision
- **THEN** validation rejects the endpoint even though both records individually satisfy their own schemas

#### Scenario: Concurrent root selection
- **GIVEN** two candidates select different aggregate roots from the same base
- **WHEN** one publishes before the other is admitted
- **THEN** the other conflicts and cannot create an aggregate with two roots

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

### Requirement: Explicit completeness and applicability
Validation SHALL distinguish record/graph integrity from purpose-specific completeness. Draft models SHALL retain explicit permitted coverage gaps. Gaps SHALL NOT waive invalid types, unresolved required references, competing ownership or immutable-revision checks. A complete result SHALL meet every applicable obligation; human adequacy assessment SHALL remain separate.

#### Scenario: Incomplete aggregate draft
- **WHEN** a draft profile permits an aggregate with no root and records that missing obligation
- **THEN** the candidate can remain explicitly incomplete, while a complete aggregate assessment requires exactly one root and no profile accepts two roots

#### Scenario: Families do not apply to this library
- **WHEN** the self-model records reasoned UI/runtime-AI inapplicability
- **THEN** its completeness report preserves those reasons and does not remove those families from the supported ontology

### Requirement: Exact original artifacts and resolving locators
The library SHALL retain original binary and UTF-8 source/artifact bytes with independent immutable digests, media types and provenance. Anchor kinds SHALL identify their exact source revision and resolving locator. Derived text or previews SHALL identify their original revision and derivation, and SHALL NOT replace the original digest.

#### Scenario: Retain the ontology PDF and Markdown
- **WHEN** an author captures both supplied source files and binds a PDF page anchor and Markdown byte-range quote
- **THEN** both original byte streams remain exactly retrievable with separate digests and source bindings; the library does not infer equivalence from filenames

#### Scenario: Derived quotation is inconsistent
- **WHEN** a PDF quotation does not resolve against its retained source-bound extraction artifact
- **THEN** validation reports the anchor failure instead of treating an arbitrary quote as verified provenance

#### Scenario: Accepted artifact is damaged
- **WHEN** required accepted binary content is missing or its digest fails on reopen
- **THEN** the library reports integrity failure without inventing empty content, promoting staging or establishing operation absence

### Requirement: One owner per normative property
Each normative property, assertion and representation binding SHALL have one declared owner within the selected scope. Document-owned values SHALL bind exact fragments; graph-owned values SHALL conform to their schema. Derived extractions SHALL be identified as derived and SHALL NOT establish a competing editable definition.

#### Scenario: Narrative and structured properties coexist
- **WHEN** a function's explanation is document-owned and its input-schema reference is graph-owned
- **THEN** a coherent candidate can commit both owners and exact provenance without copying the explanation into a competing normative graph field

### Requirement: Definitions evidence and acceptance remain separate
The library SHALL distinguish intended definitions, implementation references, observed executions, review assessments and acceptance records. Evidence/review records SHALL bind exact scope and revisions. A stored claim, imported source, passing structural validation or previous revision's assessment SHALL NOT manufacture a successful execution or human acceptance.

#### Scenario: Planned case has no run
- **WHEN** a TestCase exists without observed execution evidence
- **THEN** the model can describe the intended check but cannot report it as a passed TestRun

#### Scenario: New definition revision
- **WHEN** a successor replaces a previously reviewed definition in the selected model
- **THEN** the old review remains attached to its original exact revision and does not automatically assess the successor

### Requirement: Atomic enlarged changes retain original safeguards
New typed content, ontology bindings, revision selections, baselines, artifacts, history and original receipts SHALL publish together under current policy and expected-head guards. Exact retries SHALL resolve their original actor-bound intent before stale-base checks. Interrupted or cancelled publication SHALL remain recoverable through the original operation identity.

#### Scenario: Lost response while publishing typed graph and artifacts
- **WHEN** a complete ontology-backed change publishes but loses its response
- **THEN** authoritative recovery returns its original receipt with the same graph, artifact, baseline and exact revision selections, without another mutation
