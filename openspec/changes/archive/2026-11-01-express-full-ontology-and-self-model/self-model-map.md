# KnowledgeHub self-model: concrete loading and acceptance map

Planning draft · 31 October 2026 · Records below are intended public-consumer
output, not records already committed to a full-ontology Hub.

## The three subjects

| Subject | Stable semantic identity | Meaning |
| --- | --- | --- |
| Registered vocabulary | `sdo:ontology:system-definition` | The types, relationships, properties and constraints used to describe any system, including KnowledgeHub. |
| Governed Hub instance | `kh:hub:self-description` | The actual store containing this demonstration's model and exact registered vocabulary. |
| Described library | `urn:knowledgehub:library` | The system model of the `knowledgehub` library, distinct from the store and the vocabulary. |

Use declared namespaces and stable IDs; paths are artifact locators, not record
identity. Rename a file or module label without silently changing the semantic ID.

The existing seed's library/model, atomic-change requirement, commit use case,
satisfaction assertion, narrative document and ontology Markdown source keep their
original semantic IDs (`urn:knowledgehub:library`, `urn:requirement:atomic-change`,
`urn:use-case:commit-change-set`, `urn:assertion:commit-satisfies-atomic-change`,
`urn:document:knowledgehub-seed`, `urn:source:ontology`). Other `kh:` IDs below
identify additional concepts. The fresh v2 store has its own Hub/replay domain;
preserving semantic identities does not copy old committed operation keys or
reinterpret v1 revision tokens. Record explicit source/format lineage for the
new v2 representations.

## Inputs and claims

Capture original bytes and independent digests for the ontology PDF and Markdown,
the practical manual, the owning domain/slice/self-hosting documents, selected
OpenSpec contracts, actual Rust/Cargo files and completed native test logs.
Bind meaningful anchors: PDF pages 3–8/9–18/20–26 with retained extraction
manifests; Markdown/code/log byte ranges or resolved symbols for other sources.
PDF and Markdown are distinct `SourceArtifact` identities and exact revisions.

Git revision bindings are used only when they actually identify the captured
bytes. Existing work is uncommitted; a loader run must either capture its exact
working-tree snapshot and declare that fact, or use a real later Git revision.
Never label modified bytes as the earlier HEAD. New runs must identify the
actual checked source/build digests; earlier KH-SL-001 results remain historical.

The manual supplies modelling method, not Memento product requirements. Domain
boundaries and aggregate responsibilities in the analysis retain their proposed
status unless an actual review changes it. A source stating that a decision was
adopted can be cited as that recorded claim, without inventing a new human
assessment or acceptance event.

## Required record groups

Each row defines a required group, not permission to replace the whole group
with one generic text node. Implementation expands repeated groups to every
actual exposed operation/type/module/rule/slice in the selected exact source
snapshot and records expected IDs in a loader coverage manifest.

| Group / identity convention | Ontology types | Concrete content and source |
| --- | --- | --- |
| Purpose: `kh:goal:explain-system-definition` | Stakeholder, BusinessGoal, DesiredOutcome, BusinessCapability, Measure | Explain a system's intended/observed behaviour from versioned sources; maintainer, returning reader and host stakeholder responsibilities. Source proposal §§1–3; no invented measured user benefit. |
| Participants: `kh:actor:maintainer`, `kh:actor:host`, `kh:actor:reader` | Actor, BusinessRole, PrincipalType, Role | Authored actor responsibilities and trusted host identity/policy boundary; distinguish descriptions from runtime permissions. |
| Domain: `kh:domain:governed-system-definition` | Domain, Subdomain | Core coherent definitions/provenance/governance and supporting interchange; source `design/domain-analysis.md`. |
| Contexts: `kh:context:definition-modelling`, `kh:context:evidence-provenance`, `kh:context:change-review`, `kh:context:interchange` | BoundedContext, ContextRelationship, PublishedLanguage | Preserve candidate status and published collaboration contracts. Interchange is proposed supporting responsibility, not an implemented adapter claim. |
| Language: `kh:term:<declared-term>` | UbiquitousLanguageTerm | Every term in analysis §3, including identity versus revision, baseline versus head, interpretation versus approval, commit versus acceptance. |
| Consistency: `kh:aggregate:model-branch`, `kh:entity:model-branch`, `kh:invariant:<rule-id>` | AggregateDefinition, EntityType, Invariant | One proposed root role, current head/selected revisions, admission and exact-receipt invariants. The modelled aggregate is an analysis responsibility, not an invented Rust struct. |
| Other responsibilities: `kh:aggregate:element-identity`, `kh:aggregate:relationship-identity`, `kh:aggregate:source-artifact`, `kh:aggregate:model-baseline`, `kh:aggregate:ontology-definition`, `kh:aggregate:review-decision` | AggregateDefinition, EntityType, ValueObjectType, DomainService | Each applicable responsibility from analysis §5 has actual source status, root/composition and consistency explanation; capture uncertainty explicitly rather than claim implemented DDD objects. |
| Rules: `kh:rule:KH-DR-01` through all selected rule IDs | BusinessRule, Invariant, Constraint, Rationale | Exact owning rule statements and their relevant operations. Distinguish authored business/domain restrictions from ontology ConstraintDefinition validators. |
| Requirements: `kh:requirement:<contract-id>` | Requirement, Scenario, AcceptanceCriterion | Atomic publication, exact source/revision preservation, current policy, conflicts, retries/recovery, ownership, full ontology support and baseline/navigation promises; capture all selected behavioural delta requirements, preserving existing IDs. |
| Journey: `kh:journey:retain-sourced-definition`, `kh:journey:express-full-ontology`, `kh:journey:describe-library` | Journey, JourneyStep, UserActivity, UserTask, UserStory, StoryMap, OutcomeCut | Actor → source/scope selection → validation → guarded publication → inspection → reopen/recovery. Include denied, stale and interrupted alternatives from the owning slice plan. |
| Commands: `kh:command:commit-change-set`, plus each selected mutation | CommandDefinition, UseCase, Parameter, Precondition, Postcondition, Effect, ErrorDefinition | Registration, create/open admission where applicable, definition changes and named baseline creation. Typed input/result schemas, authority, guards, atomic effects and safe next actions for rejection/uncertainty. |
| Queries: `kh:query:read-exact`, `kh:query:operation-result`, plus each exposed read/discovery/navigation/comparison | QueryDefinition, UseCase, Parameter, ErrorDefinition | All actual public I/O operations in the implementation snapshot; exact scope/profile/baseline, disclosure and finite query limits. Convenience methods retain their public contract coverage. |
| Functions: `kh:function:validate-candidate`, `kh:function:admit-change`, `kh:function:publish-generation`, `kh:function:recover-history`, `kh:function:fingerprint-intent` | FunctionDefinition | Actual logical function contracts anchored to selected code symbols; input/output, guards, postconditions, reads/effects, errors, determinism, limits and body references. Extend to cover all described behaviours, not just five names. |
| Value/information contracts: `kh:schema:<public-contract-name>`, `kh:field:<contract>.<field>` | DataSchema, FieldDefinition, DataType, FieldConstraint, ValueObjectType | Every actual public semantic contract and field: scope, exact refs, ontology/profile/schema definitions, typed values, operations, guards, snapshots/baselines, artifacts, receipts, errors and query results. No native file handle/Tokio resource in semantic contracts. |
| Transformations: `kh:transformation:canonical-request`, `kh:transformation:semantic-revision`, `kh:transformation:snapshot-encoding` | TransformationDefinition, MappingRule, ProjectionDefinition | Exact canonical encodings, original-byte versus semantic hashing and versioned file mappings; errors/loss/inverse claims explicit. Captured source body/artifact does not imply a separately executed TransformationRun. |
| Component: `kh:component:knowledgehub-crate` | ApplicationSystem, SoftwareComponent | One Rust crate and its implemented responsibilities; actual Cargo version/build/source bindings. |
| Modules: `kh:module:application`, `kh:module:model`, `kh:module:contracts`, `kh:module:storage`, `kh:module:codec`, `kh:module:jobs`, `kh:module:lib` | Module | Current exact files in `src/` and their responsibilities. Loader inventories any later module additions/renames in the selected implementation; the list is not frozen to seven forever. |
| Ports/adapters: `kh:port:guarded-storage`, `kh:adapter:local-files`, `kh:interface:public-library` | Port, Adapter, Interface, IntegrationContract, Dependency, DataStoreDefinition | Private storage semantics, file publication mapping and owned public API; no placeholder database/service adapter. |
| Policy: `kh:policy:current-host-access`, `kh:boundary:trusted-host` | AuthorizationPolicy, Permission, TrustBoundary, AuditRequirement, RetentionPolicy | Default denial, qualified actor, registration/read/validate/commit/result permissions, admission/disclosure checks, retained original receipts and no expiry/pruning. |
| Decisions: `kh:decision:KH-D-01` and each selected decision | DesignDecision, Rationale, Assumption, OpenQuestion | Existing adopted first-slice decisions, user's no-compiler-install/platform direction and current proposed catalogue/refinement choices, each at its actual recorded status. |
| Operation: `kh:environment:verified-host`, `kh:recovery:file-publication` | Environment, Resource, ConfigurationDefinition, RecoveryProcedure, QualityScenario, ObservabilityRequirement | Actual Linux/ext4/kernel/compiler observations, bounded jobs, Busy/Capacity and draining, immutable generations, file/directory persistence and process-interruption limits. Targets remain targets unless measured. |
| Delivery: `kh:slice:KH-SL-001`, `kh:slice:KH-SL-007`, `kh:slice:KH-SL-008`, `kh:change:<openspec-name>` | DeliverySlice, ChangeDefinition, ImplementationTask, ReleaseWorkPackage | Completed versus proposed work, exact behavioural specs/tasks and evidence obligations; no generic status conflating verification/acceptance. Preserve original slice IDs in the revised sequence. |
| Cases: `kh:test:<stable-case-name>` | TestCase, Scenario, AcceptanceCriterion | Normal/denied/invalid/race/replay/cancellation/fault/restart/catalogue/lineage/baseline/navigation cases, anchored to actual test definitions. |
| Runs: `kh:run:<execution-id>` and `kh:evidence:<artifact-id>` | TestRun, EvidenceArtifact | A completed actual run, checked baseline/source/build/environment, observations and original log bytes. At minimum include original-operation replay and interruption after head replacement; current-code success requires current-code evidence. |
| Assessment: created only for an actual assessment | ReviewDecision, AcceptanceRecord | Store an exact actual assessed scope if evidence exists. Absence of human acceptance is explicit; no synthetic acceptance record is required for the loader to run. |

## Required connecting assertions

These concrete instances use source catalogue signatures. Additional references
such as module membership and task-to-change use their declared typed properties;
derived property views are not independent competing assertions.

| Concrete assertion | Purpose |
| --- | --- |
| `kh:capability:govern-definitions contributesTo kh:goal:explain-system-definition` | State why the capability exists. |
| `kh:journey:describe-library pursues kh:outcome:recover-library-explanation` | Connect the journey to its observable outcome. |
| `kh:command:commit-change-set handledBy urn:use-case:commit-change-set` | Identify the logical handler independently of physical implementation. |
| `urn:use-case:commit-change-set satisfies urn:requirement:atomic-change` | Retain the original seed's meaning under the full ontology. |
| `urn:use-case:commit-change-set invokes kh:function:admit-change` | Show the responsible logical operation and its contract. |
| `kh:function:admit-change operatesOn kh:aggregate:model-branch` | Explain branch-level consistency responsibility at the definition level. |
| `kh:aggregate:model-branch hasRoot kh:entity:model-branch` | Preserve exactly one declared root role. |
| `kh:aggregate:model-branch protects kh:invariant:atomic-publication` | Expose the protected invariant. |
| `kh:component:knowledgehub-crate implements urn:use-case:commit-change-set` | Connect intended behaviour to actual versioned implementation references. |
| `kh:adapter:local-files translates kh:integration:guarded-file-storage` | Retain the file mapping and native error/recovery meaning. |
| `kh:policy:current-host-access governs urn:use-case:commit-change-set` | Describe admission separately from the host policy that actually runs. |
| `kh:test:kill-after-head-replacement checks urn:requirement:atomic-change` | Name the promise tested by the real interruption example. |
| `kh:run:<actual-interruption-execution> executes kh:test:kill-after-head-replacement` | Bind an observed run to the exact test specification. |
| `kh:evidence:<actual-interruption-log> supportsCriterion kh:criterion:recover-original-receipt` | Keep observation, criterion and native log scope connected. |
| `kh:anchor:ontology-21.4 supportsAssertion urn:assertion:commit-satisfies-atomic-change` | Exercise first-class kernel-to-assertion provenance. |
| `kh:representation:admission-code represents <exact admission FunctionDefinition revision>` | Bind actual code bytes/symbol to the exact represented definition. |

Every arrow has its own stable assertion identity, exact type/endpoint revisions,
provenance and declared owner. Negative tests must reject the same assertion
with an incorrect endpoint/type/revision or competing owner. Context ownership
assertions use `ownedIn` for exact element revisions where applicable.

## Two exact baselines

1. A selects the complete sourced self-model and exact catalogue, all applicable
   definitions and actual evidence available before the revision demonstration.
2. Submit a coherent successor to the atomic-publication requirement's narrative
   explanation. Retain stable requirement/document/fragment IDs where their
   meaning persists; bind exact new bytes and explicitly revised assertion pins.
3. B selects the new explanation and complete compatible endpoints/supporting
   artifacts. A retains its original selected revision and exact source evidence.
4. Drain, close, reopen, retrieve A/B by exact baseline revision, compare them,
   and resolve/retry both original operation IDs without duplicate mutations.

The change is a new explanation, not an invented test success or a change to
historical run results. Existing tests and reviews stay attached to their actual
original scopes; a new execution is needed to claim verification of changed code.

## Questions the public walkthrough must answer

| ID | Public read/navigation question | Required observable answer |
| --- | --- | --- |
| Q01 | What vocabulary can this Hub express? | All K/T/R/P entries and exact schemas, constraints, source/refinement status; V/S coverage report. |
| Q02 | What is KnowledgeHub and who uses it? | Its SystemModel, purpose/outcomes, actors, supported boundary and exact originating definitions. |
| Q03 | What happens when I commit a change? | Command, use case and function input/output/guard/effect/error/uncertainty contracts, linked requirement and implementation component/artifacts. |
| Q04 | Why is publication coordinated by branch? | Sourced invariant, aggregate responsibility and rationale/decision at their actual assessment status. |
| Q05 | Where is that implemented? | Actual captured module/source/artifact/symbol bindings for application orchestration and file publication/recovery, tied to the selected build. |
| Q06 | What evidence supports recovery after a lost response? | Exact TestCase, completed TestRun, checked code/build/environment and original captured log; process interruption remains distinct from power loss. |
| Q07 | What changed between A and B? | Exact requirement/document/assertion successors and stable IDs; old/new narratives/provenance are independently retrievable after reopen. |
| Q08 | What is planned, absent, unverified or inapplicable? | Later adapters/services/UI/runtime-AI/authority handover and unresolved assessments explicitly distinguished from implemented/observed facts. |

## Applicability and authority

The self-model must explain all applicable current library responsibilities;
it must not stop at the atomic-change example. The example is the required
cross-family verification path through that larger model. Catalogue-wide fixtures
separately exercise every type/signature, including UI, AI and external-system
concepts that need no fabricated runtime instance in the library model.

There is no deployment success, human acceptance, full interchange capability or
repository-to-Hub authority handover implied by loading the model. Repository
definitions remain authoritative in their declared scopes. This model is sourced
and inspectable; routine authoring authority transfers only by KH-SL-006's explicit
decision naming exact scope, baseline and document/recovery consequences.
