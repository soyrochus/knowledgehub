# Design

## Context

The catalogue currently uses LifecycleDefinition.subject_type to reference an ontology ElementTypeDefinition, and EntityType.lifecycle as an inverse workaround. FieldDefinition.reference_targets likewise selects metatypes. Reference and endpoint roles use strings; supports_role matches concrete kinds rather than the TypeDefinition.parents graph. fields_for merges ancestors, but semantic dispatch by concrete name can let descendants escape built-in structural checks. Module.artifacts and Adapter.implementation require artifacts even for planned architecture.

C2 is a native KnowledgeHub refinement following C1. It changes the general ontology, not Memento's domain model. All named files below are in this repository. Planning, executed checks and human interpretation acceptance remain separate.

## Goals / Non-Goals

**Goals:** exact direct lifecycle subjects; one shared assignability definition; deterministic inherited fields and constraints; explicit logical constructors; honest planned realization; coherent guarded publication and reopen under one new contract.

**Non-Goals:** Modelarium bridge/controls, registration into a web application, business-rule evaluation, timers or lifecycle execution, generic programming-language type checking, old-format support, source inference, canonical hub regeneration or new backends.

## Decisions

### 1. Select actual application definitions directly

Replace LifecycleDefinition.subject_type with required singular subject: an exact reference selecting EntityType or AggregateDefinition, including their valid descendants. Remove the authoritative EntityType.lifecycle field. Derive inverse navigation by indexing the selected direct subject references; do not persist a second authoritative binding. A subject can have multiple lifecycle definitions; each lifecycle has exactly one subject. State and transition membership remains owned by its lifecycle.

An ElementTypeDefinition describes a modelling kind and cannot stand in for the application definition. Qualified subject references resolve within the selected model/baseline with ordinary external-reference disclosure. Missing required subject can only be an explicit valid/incomplete draft under the existing gap permission rules; wrong or supplied unresolved subjects are invalid. A coherent subject/lifecycle successor changes every required current binding in one guarded candidate; baseline A keeps its original exact subjects/states. Do not silently rewrite pinned evidence.

### 2. Replace string type roles with exact selectors

Use a public tagged TypeSelector with Type(ExactRef) and Role(SubjectRole). Reference value schemas and relationship from/to declarations use these selectors instead of kind-name strings. SubjectRole has explicit AnySubject and ElementRevision variants: the former covers the addressable native subject union; the latter only exact element revisions, not assertions/anchors by resemblance. Native payload validation stays governed by its actual native record variant. Application schema inheritance cannot manufacture native payloads or registration authority.

Allocate exact catalogue type references before sealing mutually referring declarations. A Type selector accepts the selected type itself and any valid descendant in its exact parent closure. Names are diagnostic labels only. Never fall back from unresolved identity to a string match. Native definitions that describe governed payload kinds cannot acquire arbitrary application parents or be used as parents to impersonate those payloads. Document the permitted extension boundary explicitly.

Resolve TypeDefinition.parents into an immutable, snapshot-bound DAG during registration and selected-schema assessment. Every ancestor reference must resolve exactly under the selected ontology/model scope. Repeated inheritance of one exact declaration origin deduplicates it; different origins competing for a field name are ambiguous even when their shapes look alike. Reject ambiguity, child shadowing, cycles and unresolved parents. Allow unambiguous multiple inheritance. Combine all applicable ancestor and descendant constraints, deduplicated by exact declaration origin, rather than choosing a parent by traversal order.

Replace concrete-name-only semantic dispatch with resolved applicable constraint execution. Existing aggregate, lifecycle, process, UI-binding, architecture and evidence checks must apply to their descendants as well. This evaluates governed model integrity only, never user-supplied rule bodies. A field selector, either relationship endpoint, lookup and subtype-enabled query all call the same matcher. C3 consumes the same public assessment path rather than reimplementing it.

### 3. Make query intent explicit and bounded

Expose exact versus include_subtypes selection explicitly, defaulting to exact in current constructors. Apply the matcher to query, neighbour and impact endpoint filters wherever a typed inclusive filter is supported; unsupported combinations fail explicitly. Snapshot the selected parent graph with the query intent and continuation. Current disclosure policy still governs each page. Existing stable order, finite budgets, exhaustion/truncation and continuation lifetime/capacity remain in force.

Prepared changes bind selected ontology/profile/type revisions and their transitive schema dependencies. Guards protect the selected schema context and model base; no latest-schema lookup may reinterpret the prepared candidate. Immutable historical parents remain usable for their own baseline, not silently replaced by current revisions.

### 4. Give logical constructors explicit generic fields

Keep the existing logical primitive kinds and add distinct date, timestamp and duration kinds. Add constructor-specific properties to DataType:

| Constructor | Required structured information |
| --- | --- |
| enum | enum_values: nonempty canonical set of text members |
| record | record_schema: exact DataSchema reference |
| list / set | item_type: exact DataType reference |
| reference | reference_targets: nonempty set selecting exact EntityType, ValueObjectType, DataSchema or AggregateDefinition application definitions |
| primitive / date / timestamp / duration | supplied format/unit/representation policy as resolving source-bound description |

Reject constructor-incompatible supplied fields and duplicate enum members. A permitted omission creates an exact sourced gap, not an invented enum/schema. Keep representation Body for source policies beyond these bounded constructors; do not interpret it as executable type code.

FieldDefinition.reference_targets now names application definitions. For a direct reference DataType, an absent field restriction means its DataType targets; a present restriction is nonempty and each target must be assignable to an allowed DataType target, so it narrows rather than widens. Collection targets are described through item_type, not a competing field-level collection shortcut; reject an incompatible field restriction. Logical reference targets are model data, distinct from the TypeSelector that validates the metamodel's own reference properties. Do not validate runtime application instances or execute predicates.

### 5. Express planned architecture without fictional files

Add required generic implementation_status with planned/implemented values to Module and Adapter. Their artifact/implementation field is optional at the property declaration, but nonempty and resolving when supplied. A structural obligation makes that field required for implemented status. Planned status permits absence as complete when other obligations are satisfied. A profile-permitted sourced gap can mark an omitted implemented binding valid/incomplete; supplied empty or invalid bindings remain invalid.

Status describes a design claim. Even a real code-artifact binding does not establish a successful run or human acceptance. Existing evidence/review distinctions remain authoritative.

### 6. Preserve precise gaps and non-execution

Apply omission gaps only to genuinely required top-level obligations at the exact subject/path, with permitted profile, reason and resolving provenance. Validate supplied values first: wrong types/endpoints, forbidden nulls, positive-minimum empty collections, undeclared fields and unresolved artifacts cannot be waived. Persist valid/incomplete assessments and their reasons in baseline/reopen reads.

Retain descriptions and native artifacts with source, language/entrypoint metadata where supplied and exact links to conditions/effects/scenarios/criteria. No capture, validation, commit or inspection invokes stored code, timers, policies or workflows. Public documentation and reports distinguish model-integrity execution from the described application's unexecuted rules.

### 7. Use one revised current contract

Update public model schemas/selectors, catalogue.json, registration/validation, read matching and affected codecs together. Allocate explicit new contract/catalogue/profile identities and update current-version metadata wherever these serialized shapes change. Regenerate disposable tests/examples and source-coverage/refinement inventories. Reuse the supported full KnowledgeHub API and native file backend, Rust 1.99.1 and host-owned async executor; introduce no service or compiler installation.

No string-selector aliases, old subject/inverse readers, dual-format branches, converters or migrations. Old input is rejected explicitly. Do not rewrite hub/ or old_hub/; current-contract revisions, baselines and receipts still remain immutable. Update current helper version pins if needed without rebuilding canonical stores or altering historical evidence.

## Acceptance trace

| Plan criteria | Native proof owned here; C3/C4 repeat through their boundary |
| --- | --- |
| G6.1-01 | Direct forward subject read and derived reverse lookup, with no inverse workaround |
| G6.1-02 | Cardinality, wrong kind/metatype, unresolved/missing binding diagnostics; no head advance |
| G6.1-03 | One authority, coherent successors, stale conflict and current/A reopen subjects/states |
| G6.1-04 | Shared native contract in reference and unrelated fixtures; Modelarium portion belongs to MM-SL-02 |
| G6.2-01 | Exact source/artifact/metadata/trace retention; sentinel business code never invoked |
| G6.2-02 | Bound/time/cancel definitions retained as data; no timers or actions |
| G6.2-03 | Executed reference/inheritance/ownership/structural checks labelled separately |
| G6.2-04 | No inferred TestRun, enforcement, execution or acceptance; no rule engine |
| G6.3-01 | Permitted exact sourced omission valid/incomplete through baseline/reopen |
| G6.3-02 | Denied/misbound/unreasoned/unresolving gap invalid without publication |
| G6.3-03 | Every listed malformed supplied-value class invalid despite attached gap |
| G6.3-04 | Complete planned versus required implemented artifact, genuine omissions versus malformed lists |
| G6.4-01 | Authorized registration, denied model-editor registration, no profile field injection |
| G6.4-02 | Exact/direct/transitive property acceptance; unrelated/reverse/same-name rejection |
| G6.4-03 | Both endpoints plus inherited/local fields and aggregate/lifecycle/UI constraints |
| G6.4-04 | Valid diamond/multiple parents; cycle/unresolved/conflict/shadow rejection and native boundaries |
| G6.4-05 | Same native matcher for assessment/reads; exact/inclusive difference and guarded selected parents; compiler repeats in C3, Modelarium in MM-SL-02 |
| G6.4-06 | Registered tokens/assertions/assessments/baselines reopen; fresh fixtures, obsolete input rejected |
| G6.4-07 | Generic extension and unrelated-domain fixtures; application content only in model data |

## Risks / Trade-offs

- Public selector and catalogue changes require coordinated native fixtures -> use one clean current contract and document the future consumer contract; no Modelarium changes here.
- A shared matcher alone would leave concrete-name semantic bypasses -> test inherited checks independently of endpoint acceptance.
- Multiple inheritance can obscure ownership -> origin-based merge rejects conflicts rather than overriding silently.
- Logical typing could expand into a runtime -> keep the constructor set bounded and retain additional policy as descriptions.
- Gap handling could falsely validate malformed data -> test absence and each supplied-invalid class separately.

## Cutover and scope gate

There is no migration. Use new-contract fresh destinations and current examples, preserving unrelated data. Reverting code uses separate prior stores, never a down-conversion of new stores. C3 follows these final contracts; C4 supplies the full sourced reference. All acceptance here uses the native library/tools. Abort if a required native outcome demands a Modelarium-owned edit or change; its later consumption is MM-SL-02.
