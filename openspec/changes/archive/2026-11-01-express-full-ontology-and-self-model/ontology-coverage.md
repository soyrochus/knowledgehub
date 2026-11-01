# Full ontology coverage inventory

Planning draft · 31 October 2026 · Proposed schemas, not an implementation result.

The catalogue is taken from source draft 0.2, PDF §§5–8 (pp. 3–8), with detailed
semantics in §§9–17. The Markdown counterpart supplies searchable anchors;
the two original files retain independent byte identities. The PDF's §6 table
was visually inspected and all 128 listed type names checked in extracted pages
5–6. Source files are unchanged. Schema field names, requiredness and encodings
below are explicit KH-D-11/12 refinements where the source does not prescribe them.

## Coverage denominator and acceptance

- K01–K17: all 17 kernel concepts, including exact public materialisation.
- T001–T128: all 128 types in all 14 source families, each with its own exact
  type definition and concrete schema; shared templates never merge identities.
- R01–R32: all 32 catalogue signatures (not 32 distinct verb spellings).
- P01–P06: additional UI signatures needed to preserve §12's prose/example;
  endpoint specialisations and nested composition are explicit refinements.
- V-01–V-17 and S01–S08: all validation categories and detailed source contracts.

Every row needs an exact registered definition, resolving source/refinement
binding, public discovery/readback and valid/invalid conformance evidence after
implementation. Kernel rows can be realised by native governed records as
specified, but remain addressable and typed. Test fixtures for each family
must demonstrate meaningful values and cross-record semantics; counts or a
name-only fixture cannot establish support. Record actual evidence paths and
checked revisions per row during implementation. All rows are currently planned.
No missing row may be reclassified as deferred to declare this change complete.

## Schema notation and shared fields

In v2, an opaque immutable revision token and its tagged canonical record digest
are separate values. Exact references use preallocated tokens; the manifest
binds each token to exactly one content digest. Cyclic reference graphs therefore
need no circular self-hashes. Original byte digests remain independent; the old
seed's digest-as-revision encoding is preserved by its original codec.

Every modelled family record has the common exact identity/revision/type/scope
and property-level authority/provenance envelope. `name: Text` and nonempty
`description: Narrative` are common properties. `Narrative` references exact
Markdown fragments unless the schema explicitly assigns graph text. Ontology
schemas and native kernel fields are graph-owned; each narrative field has one
owner. There is no simultaneous editable graph copy of a document-owned value.

Type-specific declarations below add to that common schema. Fields have cardinality
one unless `?` (absent allowed), `[]` (zero-or-more ordered list), or `[+]` (one-or-more).
Optional does not mean nullable; null is rejected unless the registered descriptor
explicitly declares it. `via predicate` means the authoritative assertion described
below, not a second normative property. Reference unions use `or`; `AnySubject` and
`AnyElementRevision` are explicit resolver roles, not unchecked strings. TypedValue
is the registered tagged value union, validated recursively. An empty declared list
can mean no applicable effects/guards; omitted required lists are distinguishable.

Reusable structured values are closed schemas:

| Value | Fields / constraints |
| --- | --- |
| ArtifactRef | Exact artifact ID/revision, original digest/length, media type; optional qualified native locator; captured versus external status explicit. |
| Body | Tagged Description(Narrative), NativeArtifact(ArtifactRef, language, entrypoint) or DecisionTable(exact table artifact, input/output schema refs). No implied execution. |
| Cardinality | min: nonnegative Int; max: nonnegative Int or explicit unbounded; max >= min. |
| Limits | timeout_ms?, active_jobs?, waiting_jobs?, max_results?, max_depth?, max_steps?; all finite nonnegative Int where applicable; applicability: Enum(bounded,not-applicable), with at least one bound or a required reason Narrative respectively. |
| Selector | Exact subject/schema reference plus declared field path or exact-ID selection. Cannot contain vendor query syntax. |
| BoundValue | Declared parameter/field reference plus schema-conforming TypedValue. No arbitrary property bags. |
| Compatibility | Exact schema/version bindings and either exact-match policy or explicit versioned compatibility mapping. |
| Delivery | Ordering, duplicate handling, retention and recovery narrative; declared schema and producer boundary references. |
| Handoff | from_actor/to_actor exact refs, information Narrative, waiting/completion Narrative and next-step exact ref. |
| StepExecution | Exact process step, admission time/guard observations, input/output exact refs, observed outcome and artifact evidence. |
| TraceMapping | Exact source/target revisions and typed field selectors; mapping-rule revision. |
| Timestamp | Offset-qualified timestamp retained as a validated explicit instant; original authored representation remains in its source artifact. |
| ClaimKind | observation, hypothesis, authored-requirement, design-decision, implementation-observation or validation-result; no generic approval flag. |

## Kernel materialisation

| Coverage ID | Kernel concept | Exact public materialisation / obligation |
| --- | --- | --- |
| K01 | `KnowledgeHub` | Native governed scope; hub ID, scope, participants and policy references. Runtime policy remains host-supplied. |
| K02 | `SystemModel` | Native model definition; model ID, purpose, scope owner, exact ontology/profile and baseline references. |
| K03 | `OntologyDefinition` | Registered catalogue record; version, source captures, exact type/relationship/constraint/profile membership. |
| K04 | `ElementTypeDefinition` | Registered schema record; exact type ID, parent definitions, property schemas, endpoint roles and constraints. |
| K05 | `RelationshipTypeDefinition` | Registered signature record; exact ID, source/target kinds or roles, multiplicities, qualifier schema and rules. |
| K06 | `ConstraintDefinition` | Registered rule record; primitive/composition ID, typed parameters, applicability, violation contract and source/refinement provenance. |
| K07 | `ProfileDefinition` | Registered selection record; vocabulary membership, completeness obligations, permitted draft gaps and applicability rules. |
| K08 | `ElementIdentity` | Native semantic identity; namespace/model/type ownership and stable ID. Exact public addressing does not require a duplicate element. |
| K09 | `ElementRevision` | Native immutable typed envelope; identity, predecessor, schema, properties/owners/provenance and semantic digest. Also an endpoint role. |
| K10 | `RelationshipAssertion` | Native identified assertion/revision; signature, exact endpoints, qualifiers, owners/provenance and lineage. Also a valid relationship endpoint. |
| K11 | `SourceArtifact` | Native source identity; owner, origin locator, media type, source status and exact captured revisions. |
| K12 | `SourceRevision` | Native capture; source identity, original digest/length/bytes, immutable locator/extraction manifests and capture provenance. |
| K13 | `SourceAnchor` | Native addressable locator; source revision, locator kind/parameters, resolving evidence and human-readable label. Direct supportsAssertion endpoint. |
| K14 | `Interpretation` | Native governed claim; exact subject/property, source/anchor, text, interpreter, method and separate interpretation/review status. |
| K15 | `ModelBaseline` | Native immutable explicit selection; ontology/profile, normative member revisions, supporting exact artifacts/sources, gaps and parentage. |
| K16 | `RepresentationBinding` | Native identified representation binding; exact subject, exact document/diagram/UI/schema/code artifact fragment, owner/derived role and provenance. |
| K17 | `ReviewDecision` | Governed assessment record; exact target revisions/baseline, assessor, decision/rationale/time and assessment provenance; no automatic runtime privilege. |

## Family type schemas

All rows inherit the shared schema above. Specific semantic checks follow in
the validation/specialisation sections; source family purposes retain their
original meanings. Exact schema refinements remain proposed pending review.

### Purpose and business (7 types)

| Coverage ID | Type | Additional declared schema / authoritative relationships |
| --- | --- | --- |
| T001 | `Stakeholder` | interest: Narrative; involvement: Text; accountable_scope: Ref(SystemModel) |
| T002 | `BusinessGoal` | intended_result: Narrative; measures: Ref(Measure)[] |
| T003 | `DesiredOutcome` | beneficiary: Ref(Actor)[]; result: Narrative; measures: Ref(Measure)[] |
| T004 | `BusinessCapability` | ability: Narrative; owner_role: Ref(BusinessRole)? |
| T005 | `BusinessFunction` | responsibility: Narrative; owner_role: Ref(BusinessRole)?; decomposition_rule: Enum(acyclic) |
| T006 | `BusinessRole` | responsibility: Narrative; authority_description: Narrative |
| T007 | `Measure` | definition: Narrative; unit: Text; target: Decimal?; measurement_method: Body |

### Requirements and reasoning (8 types)

| Coverage ID | Type | Additional declared schema / authoritative relationships |
| --- | --- | --- |
| T008 | `Requirement` | statement: Narrative; rationale: Ref(Rationale)[]; claim_kind: ClaimKind |
| T009 | `BusinessPolicy` | policy: Body; applicable_scope: Ref(SystemModel)[]; rationale: Ref(Rationale)[] |
| T010 | `BusinessRule` | rule: Body; applicability: Narrative; rationale: Ref(Rationale)[] |
| T011 | `Assumption` | statement: Narrative; basis: Narrative; resolution_status: Enum(open,confirmed,refuted); consequence: Narrative |
| T012 | `OpenQuestion` | question: Narrative; decision_owner: Text; consequence: Narrative; resolution: Ref(DesignDecision)? |
| T013 | `DesignDecision` | decision: Narrative; rationale: Ref(Rationale)[+]; alternatives: Narrative[]; decision_owner: Text; assessment_status: Enum(proposed,adopted,superseded) |
| T014 | `Rationale` | reason: Narrative; supporting_subjects: Ref(AnySubject)[] |
| T015 | `Constraint` | restriction: Body; applicability: Narrative; affected_subjects: Ref(AnySubject)[] |

### Experience (10 types)

| Coverage ID | Type | Additional declared schema / authoritative relationships |
| --- | --- | --- |
| T016 | `Actor` | responsibility: Narrative; actor_kind: Enum(human,system,host); system_model: Ref(SystemModel) |
| T017 | `Persona` | actor: Ref(Actor); characteristics: Narrative; evidence: Ref(SourceRevision)[] |
| T018 | `Journey` | actor: Ref(Actor); trigger: Narrative; completion: Narrative; alternatives: Narrative[]; interruptions: Narrative[] |
| T019 | `JourneyStep` | journey: Ref(Journey); task: Narrative; information: Narrative; authority: Narrative; result: Narrative; next_steps: Ref(JourneyStep)[]; alternatives: Ref(JourneyStep)[]; handoffs: Handoff[] |
| T020 | `UserActivity` | journey: Ref(Journey); objective: Narrative; tasks: Ref(UserTask)[+] |
| T021 | `UserTask` | activity: Ref(UserActivity); task: Narrative; completion: Narrative |
| T022 | `UserStory` | actor: Ref(Actor); need: Narrative; benefit: Narrative |
| T023 | `StoryMap` | journey: Ref(Journey); activities: Ref(UserActivity)[+]; cuts: Ref(OutcomeCut)[] |
| T024 | `OutcomeCut` | map: Ref(StoryMap); outcome: Narrative; stories: Ref(UserStory)[+]; prerequisites: Ref(OutcomeCut)[]; exclusions: Narrative[] |
| T025 | `Scenario` | preconditions: Narrative[]; trigger: Narrative; expected: Narrative[+]; preserved_on_failure: Narrative[] |

### Strategic DDD (6 types)

| Coverage ID | Type | Additional declared schema / authoritative relationships |
| --- | --- | --- |
| T026 | `Domain` | subject: Narrative; subdomains: Ref(Subdomain)[] |
| T027 | `Subdomain` | domain: Ref(Domain); responsibility: Narrative; classification: Enum(core,supporting,generic) |
| T028 | `BoundedContext` | responsibility: Narrative; model_owner: Text; language: Ref(UbiquitousLanguageTerm)[]; hypotheses: Ref(Assumption)[] |
| T029 | `ContextRelationship` | upstream: Ref(BoundedContext); downstream: Ref(BoundedContext); collaboration: Narrative; contract: Ref(PublishedLanguage)? |
| T030 | `UbiquitousLanguageTerm` | term: Text; meaning: Narrative; context: Ref(BoundedContext); distinctions: Narrative[] |
| T031 | `PublishedLanguage` | owner_context: Ref(BoundedContext); definitions: Ref(AnyElementRevision)[+]; compatibility_policy: Narrative |

### Tactical DDD (7 types)

| Coverage ID | Type | Additional declared schema / authoritative relationships |
| --- | --- | --- |
| T032 | `AggregateDefinition` | responsibility: Narrative; consistency_boundary: Narrative; members: Ref(EntityType)[+]; root via hasRoot; invariants via protects |
| T033 | `EntityType` | identity_contract: Ref(DataSchema); lifecycle: Ref(LifecycleDefinition)?; fields: Ref(FieldDefinition)[] |
| T034 | `ValueObjectType` | value_contract: Ref(DataSchema); equality: Body; fields: Ref(FieldDefinition)[] |
| T035 | `DomainService` | responsibility: Narrative; functions: Ref(FunctionDefinition)[+]; rules: Ref(Invariant)[] |
| T036 | `Invariant` | rule: Body; rationale: Narrative; valid_examples: Ref(Scenario)[]; invalid_examples: Ref(Scenario)[] |
| T037 | `RepositoryPort` | aggregate: Ref(AggregateDefinition); contract: Ref(Interface); consistency: Narrative |
| T038 | `LifecycleDefinition` | subject_type: Ref(ElementTypeDefinition); states: Ref(State)[+]; transitions: Ref(Transition)[]; initial_state: Ref(State); terminal_states: Ref(State)[] |

### Application behaviour (9 types)

| Coverage ID | Type | Additional declared schema / authoritative relationships |
| --- | --- | --- |
| T039 | `UseCase` | actor: Ref(Actor)[+]; trigger: Narrative; outcome: Narrative; preconditions: Ref(Precondition)[]; errors: Ref(ErrorDefinition)[]; functions via invokes |
| T040 | `CommandDefinition` | input: Ref(DataSchema); result: Ref(DataSchema); guards: Ref(Precondition)[]; errors: Ref(ErrorDefinition)[]; operation_identity: Narrative; concurrency: Narrative; handler via handledBy |
| T041 | `QueryDefinition` | input: Ref(DataSchema); result: Ref(DataSchema); snapshot_semantics: Narrative; ordering: Narrative; limits: Limits; disclosure: Narrative |
| T042 | `FunctionDefinition` | input: Ref(DataSchema); output: Ref(DataSchema); preconditions: Ref(Precondition)[]; postconditions: Ref(Postcondition)[]; rules: Ref(Invariant)[]; reads: Ref(AnySubject)[]; effects: Ref(Effect)[]; errors: Ref(ErrorDefinition)[]; determinism: Enum(deterministic,nondeterministic,conditional); limits: Limits; body: Body; verification: Ref(TestCase)[] |
| T043 | `Parameter` | datatype: Ref(DataType); direction: Enum(input,output); nullable: Bool; cardinality: Cardinality; unit: Text?; constraints: Ref(FieldConstraint)[] |
| T044 | `Precondition` | condition: Body; denied_result: Ref(ErrorDefinition)? |
| T045 | `Postcondition` | condition: Body; scope: Narrative |
| T046 | `Effect` | effect_kind: Enum(read,write,emit,external); target: Ref(AnySubject); condition: Body; commit_boundary: Narrative |
| T047 | `ErrorDefinition` | code: Text; condition: Body; outcome: Enum(rejected,unavailable,uncertain); preserved_state: Narrative; next_action: Narrative |

### Events and processes (11 types)

| Coverage ID | Type | Additional declared schema / authoritative relationships |
| --- | --- | --- |
| T048 | `EventDefinition` | meaning: Narrative; owner_context: Ref(BoundedContext); schema: Ref(DataSchema); producers: Ref(AnyElementRevision)[]; consumers: Ref(EventHandler)[]; disclosure: Narrative; compatibility: Compatibility; delivery: Delivery |
| T049 | `EventOccurrence` | definition: Ref(EventDefinition); subject: Ref(AnySubject); occurred_at: Timestamp; recorded_at: Timestamp; causation: Ref(AnySubject)[]; correlation: Text; producer: Ref(AnyElementRevision); execution: Ref(ProcessExecution)?; payload: ArtifactRef? |
| T050 | `EventHandler` | input_schema: Ref(DataSchema); function: Ref(FunctionDefinition); retry: Narrative; deduplication: Narrative; consumed events via consumes |
| T051 | `ProcessDefinition` | steps: Ref(ProcessStep)[+]; initial_steps: Ref(ProcessStep)[+]; states: Ref(State)[]; transitions: Ref(Transition)[]; allowed_loops: Ref(Transition)[]; outcomes: Narrative[+] |
| T052 | `ProcessStep` | process: Ref(ProcessDefinition); guards: Ref(Precondition)[]; next_steps: Ref(ProcessStep)[]; timers: Ref(Timer)[]; waits_for: Ref(EventDefinition)[]; compensations: Ref(Compensation)[]; invoked use cases via invokes |
| T053 | `ProcessExecution` | definition: Ref(ProcessDefinition); subjects: Ref(AnySubject)[]; admitted_steps: StepExecution[]; started_at: Timestamp; ended_at: Timestamp?; outcome: Enum(pending,succeeded,failed,uncertain); evidence: ArtifactRef[] |
| T054 | `State` | owner: Ref(LifecycleDefinition or ProcessDefinition); meaning: Narrative; terminal: Bool |
| T055 | `Transition` | owner: Ref(LifecycleDefinition or ProcessDefinition); from: Ref(State); to: Ref(State); guards: Ref(Precondition)[]; effects: Ref(Effect)[]; trigger via triggeredBy |
| T056 | `Timer` | owner: Ref(ProcessDefinition); schedule: Body; expiry_effect: Ref(Effect); correlation: Ref(CorrelationPolicy)? |
| T057 | `Compensation` | owner: Ref(ProcessDefinition); trigger: Body; operation: Ref(UseCase); preserved_state: Narrative |
| T058 | `CorrelationPolicy` | keys: Ref(FieldDefinition)[+]; matching: Body; scope: Narrative |

### Information and transformations (10 types)

| Coverage ID | Type | Additional declared schema / authoritative relationships |
| --- | --- | --- |
| T059 | `DataSchema` | fields: Ref(FieldDefinition)[]; identity_fields: Ref(FieldDefinition)[]; compatibility: Compatibility; native_artifact: ArtifactRef? |
| T060 | `FieldDefinition` | schema: Ref(DataSchema); datatype: Ref(DataType); nullable: Bool; cardinality: Cardinality; unit: Text?; constraints: Ref(FieldConstraint)[]; reference_targets: Ref(ElementTypeDefinition)[] |
| T061 | `DataType` | value_kind: Enum(text,boolean,integer,decimal,enumeration,reference,record,list,set,binary); representation: Body; value_constraints: Ref(FieldConstraint)[] |
| T062 | `FieldConstraint` | field: Ref(FieldDefinition or Parameter); predicate: Body; violation: Narrative |
| T063 | `ProjectionDefinition` | output_schema: Ref(DataSchema); mapping: Ref(TransformationDefinition); freshness: Narrative; derived inputs via derivesFrom |
| T064 | `TransformationDefinition` | kind: Enum(data,state,schema,model,artifact); mapping_rules: Ref(MappingRule)[+]; preconditions: Ref(Precondition)[]; postconditions: Ref(Postcondition)[]; effects: Ref(Effect)[]; errors: Ref(ErrorDefinition)[]; information_loss: Narrative; inverse_claim: Narrative?; input/output via hasInputContract/hasOutputContract |
| T065 | `TransformationRun` | definition: Ref(TransformationDefinition); inputs: Ref(AnySubject)[+]; parameters: BoundValue[]; environment: Ref(Environment); outputs: Ref(AnySubject)[]; result: Enum(succeeded,failed,uncertain); diagnostics: ArtifactRef[]; mappings: TraceMapping[] |
| T066 | `MappingRule` | source: Selector; target: Selector; rule: Body; missing_policy: Narrative; error_policy: Narrative |
| T067 | `MigrationDefinition` | source_contract: Ref(DataSchema or ProfileDefinition); target_contract: Ref(DataSchema or ProfileDefinition); transformation: Ref(TransformationDefinition); validation: Narrative; interruption: Narrative; recovery: Ref(RecoveryProcedure) |
| T068 | `DataStoreDefinition` | responsibility: Narrative; schemas: Ref(DataSchema)[+]; persistence: Narrative; retention: Ref(RetentionPolicy)[] |

### Interaction and visual design (12 types)

| Coverage ID | Type | Additional declared schema / authoritative relationships |
| --- | --- | --- |
| T069 | `ScreenDefinition` | purpose: Narrative; states: Ref(ViewState)[+]; layout: Ref(LayoutConstraint)[]; accessibility: Ref(AccessibilityRule)[]; composition/display via prose relationship refinements |
| T070 | `ViewState` | screen: Ref(ScreenDefinition); condition: Body; information: Narrative; actions: Ref(Interaction)[]; feedback: Narrative; focus: Narrative; recovery: Narrative |
| T071 | `ComponentDefinition` | contract: Ref(DataSchema); appearance: Narrative; interactions: Ref(Interaction)[]; tokens: Ref(DesignToken)[]; assets: Ref(Asset)[] |
| T072 | `ComponentPlacement` | component: Ref(ComponentDefinition); screen: Ref(ScreenDefinition); parameters: BoundValue[]; layout: Ref(LayoutConstraint)[]; bindings: Ref(DataBinding or ActionBinding)[]; UI composition/edits/uses/issue edges via prose refinements |
| T073 | `Interaction` | gesture: Text; condition: Body; effect: Ref(Effect)[]; feedback: Narrative; command via issues |
| T074 | `NavigationTransition` | from: Ref(ViewState); to: Ref(ViewState); trigger: Ref(Interaction); guards: Ref(Precondition)[]; focus: Narrative; return_state: Ref(ViewState)? |
| T075 | `DataBinding` | target: Selector; mapping: Ref(TransformationDefinition)?; condition: Body; query via reads |
| T076 | `ActionBinding` | interaction: Ref(Interaction); command: Ref(CommandDefinition); parameters: BoundValue[]; permission_condition: Body; validation: Ref(FieldConstraint)[] |
| T077 | `LayoutConstraint` | subject: Ref(ScreenDefinition or ComponentPlacement); rule: Body; responsive_conditions: Body[]; normative: Bool |
| T078 | `DesignToken` | token_kind: Text; datatype: Ref(DataType); value: TypedValue; unit: Text? |
| T079 | `Asset` | artifact: ArtifactRef; usage: Narrative; media_type: Text; accessibility_text: Narrative? |
| T080 | `AccessibilityRule` | scope: Ref(ScreenDefinition or ComponentDefinition or Interaction)[]; rule: Body; verification: Ref(TestCase)[] |

### Architecture and integration (11 types)

| Coverage ID | Type | Additional declared schema / authoritative relationships |
| --- | --- | --- |
| T081 | `ApplicationSystem` | purpose: Narrative; components: Ref(SoftwareComponent)[]; interfaces: Ref(Interface)[]; external_systems: Ref(ExternalSystem)[] |
| T082 | `SoftwareComponent` | responsibility: Narrative; modules: Ref(Module)[]; interfaces: Ref(Interface)[]; artifacts: ArtifactRef[]; implemented use cases/interfaces via implements |
| T083 | `Module` | responsibility: Narrative; component: Ref(SoftwareComponent); artifacts: ArtifactRef[+]; interfaces: Ref(Interface)[] |
| T084 | `Interface` | operations: Ref(CommandDefinition or QueryDefinition or FunctionDefinition)[+]; contract: Ref(DataSchema)[]; compatibility: Compatibility |
| T085 | `Port` | responsibility: Narrative; interface: Ref(Interface); owner_component: Ref(SoftwareComponent) |
| T086 | `Adapter` | responsibility: Narrative; port: Ref(Port); implementation: ArtifactRef[+]; translations via translates |
| T087 | `Endpoint` | interface: Ref(Interface); protocol: Text; address: Text; transport_policy: Narrative |
| T088 | `MessageChannel` | message_schema: Ref(DataSchema); producers: Ref(SoftwareComponent)[]; consumers: Ref(SoftwareComponent)[]; delivery: Delivery |
| T089 | `ExternalSystem` | owner: Text; boundary: Narrative; interfaces: Ref(IntegrationContract)[]; native_locator: Text? |
| T090 | `IntegrationContract` | native_identity: Narrative; schema: Ref(DataSchema); protocol: Text; authority: Narrative; translation: Ref(TransformationDefinition)[]; error_semantics: Narrative |
| T091 | `Dependency` | from: Ref(AnyElementRevision); to: Ref(AnyElementRevision); dependency_kind: Text; reason: Narrative; compatibility: Narrative |

### Security and trust (9 types)

| Coverage ID | Type | Additional declared schema / authoritative relationships |
| --- | --- | --- |
| T092 | `PrincipalType` | identity_contract: Ref(DataSchema); trust_assumptions: Narrative |
| T093 | `Role` | principal_types: Ref(PrincipalType)[]; responsibility: Narrative; permissions: Ref(Permission)[] |
| T094 | `Permission` | action: Text; resource_scope: Selector; conditions: Body |
| T095 | `AuthorizationPolicy` | rules: Body[+]; evaluation_failure: Enum(deny,unavailable); governed use cases/queries/resources via governs |
| T096 | `TrustBoundary` | inside: Ref(AnyElementRevision)[]; outside: Ref(AnyElementRevision)[]; crossing_contract: Narrative |
| T097 | `DataClassification` | criteria: Body; handling: Narrative; applicable_subjects: Ref(AnySubject)[] |
| T098 | `RetentionPolicy` | scope: Selector; retention: Body; deletion_conditions: Body; history_obligations: Narrative |
| T099 | `Threat` | subject: Ref(AnyElementRevision); stimulus: Narrative; consequence: Narrative; mitigations: Ref(Constraint or AuthorizationPolicy)[] |
| T100 | `AuditRequirement` | scope: Selector; retained_facts: Narrative[+]; attribution: Narrative; integrity: Narrative; retention: Ref(RetentionPolicy)? |

### Quality and operations (9 types)

| Coverage ID | Type | Additional declared schema / authoritative relationships |
| --- | --- | --- |
| T101 | `QualityScenario` | environment: Ref(Environment); stimulus: Narrative; response: Narrative; measure: Ref(Measure); target: Decimal |
| T102 | `ServiceObjective` | measure: Ref(Measure); target: Decimal; conditions: Narrative; assessment_window: Narrative |
| T103 | `Environment` | platform: Narrative; configuration: Ref(ConfigurationDefinition)[]; assumptions: Narrative[]; captured_facts: ArtifactRef[] |
| T104 | `DeploymentDefinition` | components: Ref(SoftwareComponent)[+]; environment: Ref(Environment); resources: Ref(Resource)[]; configuration: Ref(ConfigurationDefinition)[] |
| T105 | `DeploymentInstance` | definition: Ref(DeploymentDefinition); build: ArtifactRef; observed_at: Timestamp; native_locator: Text; evidence: ArtifactRef[+] |
| T106 | `Resource` | resource_kind: Text; capacity: BoundValue[]; constraints: Ref(Constraint)[] |
| T107 | `ConfigurationDefinition` | schema: Ref(DataSchema); defaults: BoundValue[]; secret_references: Text[]; ownership: Narrative |
| T108 | `ObservabilityRequirement` | subject: Ref(AnyElementRevision); signals: Narrative[+]; interpretation: Narrative; verification: Ref(TestCase)[] |
| T109 | `RecoveryProcedure` | failure_profile: Narrative; preconditions: Ref(Precondition)[]; steps: Body; preserved_state: Narrative; uncertain_outcome: Narrative; verification: Ref(TestCase)[] |

### AI participation (9 types)

| Coverage ID | Type | Additional declared schema / authoritative relationships |
| --- | --- | --- |
| T110 | `AgentRole` | responsibility: Narrative; capabilities: Ref(ToolContract)[]; authority: Ref(AuthorizationPolicy); human_boundary: Ref(HumanReviewBoundary) |
| T111 | `ModelConfiguration` | model_reference: Text; provider: Text; parameters: BoundValue[]; version_binding: ArtifactRef? |
| T112 | `PromptDefinition` | content: ArtifactRef; inputs: Ref(DataSchema); model: Ref(ModelConfiguration); evaluation: Ref(EvaluationCase)[] |
| T113 | `ToolContract` | input: Ref(DataSchema); output: Ref(DataSchema); effects: Ref(Effect)[]; permissions: Ref(Permission)[]; errors: Ref(ErrorDefinition)[]; limits: Ref(RunLimit) |
| T114 | `ContextPolicy` | selection: Body; sources: Ref(SourceArtifact)[]; disclosure: Narrative; retention: Ref(RetentionPolicy)? |
| T115 | `RetrievalPolicy` | selection: Body; snapshot_semantics: Narrative; ranking: Narrative; limits: Ref(RunLimit) |
| T116 | `RunLimit` | calls: Int?; duration_ms: Int?; concurrency: Int?; resource_budget: BoundValue[]; exceeded_outcome: Narrative; at least one applicable finite bound in a complete profile |
| T117 | `EvaluationCase` | inputs: ArtifactRef[+]; criterion: Ref(AcceptanceCriterion); expected: Narrative; evaluation_method: Body |
| T118 | `HumanReviewBoundary` | trigger: Body; scope: Selector; authorised_reviewer: Ref(Role)[]; decision_effect: Narrative |

### Delivery and assurance (10 types)

| Coverage ID | Type | Additional declared schema / authoritative relationships |
| --- | --- | --- |
| T119 | `DeliverySlice` | actor: Ref(Actor); trigger: Narrative; outcome: Narrative; complete_path: Narrative; safeguards: Ref(Requirement)[]; evidence_obligations: Ref(AcceptanceCriterion)[+]; prerequisites: Ref(DeliverySlice)[] |
| T120 | `ChangeDefinition` | intent: Narrative; affected_subjects: Ref(AnySubject)[+]; rationale: Ref(Rationale)[]; slice: Ref(DeliverySlice); specifications: ArtifactRef[+] |
| T121 | `ImplementationTask` | change: Ref(ChangeDefinition); work: Narrative; implementation_references: ArtifactRef[]; implementation_status: Enum(planned,implemented) |
| T122 | `ReleaseWorkPackage` | purpose: Narrative; slices: Ref(DeliverySlice)[]; obligations: Ref(AcceptanceCriterion)[+] |
| T123 | `Release` | baseline: Ref(ModelBaseline); packages: Ref(ReleaseWorkPackage)[]; implementation: ArtifactRef[+]; acceptance: Ref(AcceptanceRecord)[] |
| T124 | `AcceptanceCriterion` | promise: Narrative; scope: Ref(AnySubject)[+]; required_evidence: Narrative; decision_owner: Text |
| T125 | `TestCase` | inputs: ArtifactRef[]; steps: Body; expected: Narrative[+]; checked subjects via checks |
| T126 | `TestRun` | baseline: Ref(ModelBaseline); build: ArtifactRef; environment: Ref(Environment); started_at: Timestamp; result: Enum(passed,failed,inconclusive); observations: Narrative[+]; evidence: ArtifactRef[+]; exact case via executes |
| T127 | `EvidenceArtifact` | artifact: ArtifactRef; observed_scope: Ref(AnySubject)[+]; observation_kind: ClaimKind; captured_at: Timestamp; criteria via supportsCriterion |
| T128 | `AcceptanceRecord` | assessor: Text; assessed_scope: Ref(AnySubject)[+]; baseline: Ref(ModelBaseline); decision: Enum(accepted,conditional,rejected); rationale: Narrative; conditions: Narrative[]; assessed_at: Timestamp |

## Source relationship declarations

Every assertion has exactly one source and one target endpoint. These rows
retain §8's direction, endpoint unions and stated constraints. Proposed default
outgoing/incoming aggregate cardinalities are 0..many unless the source/profile
requires otherwise; they are explicit schema data, not implied by a label.
`hasRoot` is exactly one in a complete aggregate, at most one in a permitted draft.
`handledBy` is exactly one logical handler in a complete command-handling profile.
Inverse views use the same assertion identity. Qualifier schemas are closed and
empty unless a registered refinement explicitly adds a typed qualifier.

| Coverage ID | Source signature | Target type or role | Source meaning / constraints |
| --- | --- | --- | --- |
| R01 | `BusinessCapability contributesTo` | `BusinessGoal` | Connects an ability to its purpose; many-to-many. |
| R02 | `BusinessFunction realises` | `BusinessCapability` | Identifies supporting business responsibility; many-to-many. |
| R03 | `BusinessFunction decomposesInto` | `BusinessFunction` | Describes responsibility structure; declared decomposition must not form a containment cycle. |
| R04 | `Journey pursues` | `DesiredOutcome` | Identifies the participant outcome. |
| R05 | `Journey containsStep` | `JourneyStep` | Preserves step identity; sequence, alternatives and handoffs are explicit relationships, not inferred from list positions. |
| R06 | `UserStory supportsStep` | `JourneyStep` | Connects a need to the work being performed; many-to-many. |
| R07 | `Requirement formalises` | `UserStory` or `DesiredOutcome` | Connects a precise promise to its need; a requirement can support several stories and can exist without a user-story formulation. |
| R08 | `UseCase satisfies` | `Requirement` | Identifies behaviour intended to fulfil the promise; does not itself establish verification. |
| R09 | `Interaction issues` | `CommandDefinition` | Binds an interaction to an explicit request. |
| R10 | `CommandDefinition handledBy` | `UseCase` | Identifies the responsible logical handler in a command-handling profile; physical replicas are separate implementation objects. |
| R11 | `DataBinding reads` | `QueryDefinition` | Connects a displayed value or collection to an authorised query contract. |
| R12 | `UseCase invokes` | `FunctionDefinition` | Connects application orchestration to operations and calculations. |
| R13 | `FunctionDefinition operatesOn` | `AggregateDefinition` | Identifies affected domain state; functions without aggregate state need no such edge. |
| R14 | `AggregateDefinition hasRoot` | `EntityType` | Exactly one root within the aggregate definition. |
| R15 | `AggregateDefinition protects` | `Invariant` | Identifies its consistency obligations. |
| R16 | `ElementRevision ownedIn` | `BoundedContext` | Identifies domain ownership where applicable; cross-context similarity is represented by explicit mappings. |
| R17 | `FunctionDefinition declaresEvent` | `EventDefinition` | Declares a possible resulting fact, including the condition and commit boundary under which it can be published. |
| R18 | `EventHandler consumes` | `EventDefinition` | Declares a reaction to a supported event contract. |
| R19 | `ProcessStep invokes` | `UseCase` | Connects process coordination to application behaviour. |
| R20 | `Transition triggeredBy` | `CommandDefinition` or `EventDefinition` | Identifies a transition trigger; its states and guards belong to the declared lifecycle or process. |
| R21 | `TransformationDefinition hasInputContract` | `DataSchema` or `ProfileDefinition` | Declares accepted input structure or model shape. |
| R22 | `TransformationDefinition hasOutputContract` | `DataSchema` or `ProfileDefinition` | Declares the required output structure or model shape. |
| R23 | `ProjectionDefinition derivesFrom` | `DataSchema` or `EventDefinition` | Identifies inputs used to construct a read model. |
| R24 | `SoftwareComponent implements` | `UseCase` or `Interface` | Connects intended responsibility to implementation structure. |
| R25 | `Adapter translates` | `IntegrationContract` | Identifies the boundary at which provider-specific meaning is mapped. |
| R26 | `AuthorizationPolicy governs` | `UseCase`, `QueryDefinition` or `Resource` | Makes access and admission rules inspectable independently of a UI control. |
| R27 | `TestCase checks` | `Scenario`, `Requirement` or `Invariant` | Connects verification intent to the promise being checked. |
| R28 | `TestRun executes` | `TestCase` | Refers to an exact test version and its observed execution context. |
| R29 | `EvidenceArtifact supportsCriterion` | `AcceptanceCriterion` | Associates results with a specific acceptance obligation. |
| R30 | `RepresentationBinding represents` | `ElementRevision` | Connects a document, diagram, UI or code representation to the exact definition represented. |
| R31 | `SourceAnchor supportsAssertion` | `RelationshipAssertion` | Records the evidence for a semantic relationship. |
| R32 | `RelationshipAssertion contradicts` | `RelationshipAssertion` | Preserves an identified disagreement until it is resolved. |

## Prose relationships and additional contract coverage

§12 also uses semantic composition/display/edit/transform edges outside §8's
illustrative catalogue. Preserve them with explicit signature definitions below.
`ComponentPlacement` endpoints specialise the source's distinction between a
component definition and its use in a screen; P02 is a proposed nesting refinement.
All these edges carry independent assertion identity, revision and provenance.
They are not graph-layout drawing connections.

| ID | Signature | Meaning / constraints |
| --- | --- | --- |
| P01 | ScreenDefinition contains ComponentPlacement | Screen composition; each placement pins its component definition and screen; no containment cycle. |
| P02 | ComponentPlacement contains ComponentPlacement | Proposed nested composition; same screen and no containment cycle. |
| P03 | ScreenDefinition displays ProjectionDefinition | A displayed projection's exact output schema and freshness contract remain inspectable. |
| P04 | ComponentPlacement edits DataSchema | Form fields bind to the exact request schema; field constraints resolve. |
| P05 | ComponentPlacement uses TransformationDefinition | Transformation input/output contracts agree with the bound form/schema. |
| P06 | ComponentPlacement issues CommandDefinition | Control activation refers to an identified interaction/action binding and exact command, consistent with the existing Interaction issues declaration. |

Other typed reference properties retain declared schemas and property provenance;
a rendered property edge is derived and never a competing assertion owner.

| ID | Source detail | Required expression / meaningful conformance examples |
| --- | --- | --- |
| S01 | §§7,9 business/behaviour distinctions | BusinessFunction, UseCase and FunctionDefinition remain distinct; functions expose input/output, pre/postconditions, rules, reads/effects, errors/uncertainty, determinism, limits, body and verification. A name-only complete function fails. Decision logic can retain an exact native decision-table Body without inventing human review. |
| S02 | §10 definitions versus occurrences | EventDefinition/schema/delivery differs from EventOccurrence time/cause/correlation; ProcessDefinition differs from observed ProcessExecution. Match handlers to declared schema versions; do not infer event sourcing. |
| S03 | §§10–11 process and transitions | State membership is exact, triggers/guards/effects resolve, allowed process loops differ from prohibited containment cycles; timer/compensation/correlation rules are expressible. |
| S04 | §11 information and transformations | Schema fields carry type, ID role, unit, nullability, cardinality, references and retention. All five transformation kinds (data, state, schema, model, artifact) express contracts, rules, errors/loss and optional inverse claims; runs bind exact inputs/output/parameters/environment/diagnostics/trace mappings. No assumed reversibility. |
| S05 | §12 visual representation | Screen/state, component/placement, data/action binding, layout/token/asset/accessibility, focus/navigation and denied/conflict/recovery states are expressible. Normative UI layout differs from nonsemantic diagram coordinates. |
| S06 | §13 architecture/security/quality | Logical contexts differ from deployment components; integration mapping retains native identity/protocol/authority/error meaning. Policy records do not grant permission. Quality results bind actual build/environment/workload; secret configuration stores references, not copied secrets. |
| S07 | §13 AI where applicable | Agent/model/prompt/tool/context/retrieval/limit/evaluation/human-review boundaries expose contracts and evidence without requiring an AI engine or granting modelled agents authority. |
| S08 | §§14–15 delivery/evidence | Slice/change/task/release differ; specification versus execution versus acceptance remain separate. Current/target/observed selections are explicit baselines; replacing a revision does not inherit review or fabricate observed implementation. |

## Validation categories

For each source rule, implement the deterministic check indicated below and retain
purpose/human boundaries explicitly. The valid/invalid examples form acceptance
obligations; creating this inventory does not execute them.

| Rule | Deterministic obligation / evidence boundary |
| --- | --- |
| V-01 Identity | Scope-qualified identity uniqueness; aliases/labels/locations cannot replace identity. Valid rename preserves identity; colliding different concepts fail. |
| V-02 Type conformance | Every property/qualifier conforms to its exact schema, requiredness and permitted types; unknown normative fields fail. |
| V-03 Relationship endpoints | Every R/P signature has valid direction, endpoint role/type, qualifier and cardinality; invalid reverse/target/missing pins fail independently. |
| V-04 Revision integrity | Immutable content digests, explicit predecessor lineage and coherent baseline membership; no latest substitution or accepted rewrite. |
| V-05 Provenance | Exact source captures, resolving native/derived locator manifests, claim/interpretation status and ownership; invalid digest/locator fails. |
| V-06 Authority | One owner per field/assertion within selected scope; exact narrative fragments and derived roles; competing owner fails. |
| V-07 Domain ownership | Exact context ownership and qualified cross-context mappings; same label does not merge distinct meanings. |
| V-08 Aggregate composition | One selected root EntityType belongs to complete aggregate membership; no second root; protected invariants are explicit. |
| V-09 Behaviour contracts | Complete selected behaviour profiles have the S01 obligations and exact bindings; text/native algorithm representation is not execution proof. |
| V-10 State/process | S03 state/step membership, trigger/guard/effect references, declared cycle rules and initial/terminal selections are coherent. |
| V-11 Events | Producer/handler/schema/version and delivery/deduplication/correlation contracts agree; incompatibility is explicit, never guessed. |
| V-12 Transformations | S04 contracts and mapping selectors resolve; run input/output lineage and loss/inverse claims are explicit. |
| V-13 UI | S05 bindings resolve exact query/command/schema/policy subjects and layout/state/placement ownership; graph coordinates confer no semantic edge. |
| V-14 Architecture | Implements/translates and typed component/port/artifact references resolve exact responsibilities/contracts; implementation truth remains observed evidence. |
| V-15 Evidence scope | Actual run records include exact case/baseline/build/environment/observations and captured evidence; structural validation cannot establish real-world success. |
| V-16 Review/acceptance | Exact assessed scope, actor, outcome and provenance; successor review is explicit. Actual human acceptance is never inferred from validation. |
| V-17 Completeness | Separate structural validity, profile completeness and human adequacy. Every applicable obligation is met or an allowed draft gap/inapplicability claim is explicit; complete reports permit no unresolved applicable obligation. |

## Definition versus implementation evidence

At implementation time each K/T/R/P/V/S row must receive the exact registered
revision(s), specimen record/graph IDs, a meaningful successful example, independent
failure examples where the rule is rejectable, reopen results and evidence paths.
Human-only adequacy/acceptance obligations receive explicit assessment status,
not synthetic automatic-test success. Catalogue support must be complete even
where the library self-model records a family as inapplicable.

## Original source fingerprints

- `source/system-definition-ontology-proposal.pdf`: SHA-256 `5b933fd4a40bfde09acd933ec6c2c2143ea698c1bf0ed77f7385eba70003ebda`, 479613 original bytes.
- `source/system-definition-ontology-proposal.md`: SHA-256 `30cc6f283e2b3f5cea375ff3990c97e4716c918d239ceee1ad331cc57ac06620`, 74064 original bytes.
