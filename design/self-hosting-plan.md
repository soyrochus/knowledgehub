# Bootstrapping KnowledgeHub's own definition

Draft 0.1 · 30 October 2026 · Proposed transition; no Hub instance or authority migration exists yet.

This plan owns the bootstrap sequence, representation authority and migration gates. It references the [domain analysis](domain-analysis.md), [slice plan](../delivery/slice-plan.md), [ontology proposal](../source/system-definition-ontology-proposal.md) and [Practical Design Manual](../source/practical-design-manual.md). The manual guides how we discover and specify the product; the ontology defines what the Hub can represent and preserve.

## Intended result

KnowledgeHub will be developed using its own governed system model. Begin with reviewable repository documents, implement a small complete library path, and progressively load the library's own definition through that public API. Later development changes start from the governed model and produce exact, version-bound specification representations. Implementation and execution remain in their appropriate code/build/test tools.

Uploading a directory of prose is only source capture. The self-model also needs identified concepts, typed relationships, ownership, revisions, provenance, selected baselines, explicit gaps and links from promises to actual evidence. The first seed can be small; it must already preserve those distinctions for its selected scope.

## One definition, several representations

| Material | During bootstrap | After an explicit scoped handover |
| --- | --- | --- |
| Original proposal and manual under `source/` | Immutable inputs to the analysis; retain original revisions and hashes. | Remain source evidence. A new source edition is a successor capture, never an edit of earlier evidence. |
| Domain analysis, journeys and slice definitions | Repository documents own these authored definitions while they are being reviewed. | Selected fields may remain document-owned or become graph-owned; the recorded authority map names exactly which. |
| OpenSpec specifications and change artifacts | Own the bounded behavioural delta used to direct an authorised implementation. | Bind each contract to a Hub model revision/baseline. Keep narrative document-owned where appropriate; graph-owned fields appear only as derived representations or references. |
| Graph records | Planned mappings only; no operative graph exists. | Own stable identities, typed assertions and the structured fields explicitly assigned to them. Derived extractions remain marked as derived. |
| Rust source, builds and tests | Git/build/test tools own implementation and actual execution evidence. | Continue to own them; the Hub retains exact versions, bindings and scoped evidence records. |
| Review and acceptance records | Actual user decisions must be recorded explicitly; current drafts are unassessed. | Hub review decisions bind exact scope. Actual acceptance and test results remain distinct from stored design status. |

Authority is per field, relationship and baseline scope. It need not move all at once, and moving definitions into the Hub does not require converting every paragraph to graph fields. A document-owned definition edited through the API must update its bindings coherently; a graph-owned field's Markdown rendering cannot become a competing editable definition.

## Bootstrap stages and gates

### B0 — Define the library outside the running library

Current stage. Keep original sources intact. Author domain language, responsibility boundaries, rules, journeys, slices and the first specification as documents. Record what is sourced, what is proposed, and which adoption choices remain open. Stable document identifiers referenced in the analysis will become import mappings, not filenames used as semantic identities.

Exit evidence is a reviewed planning set and resolved first-implementation decisions. Structural validation can show consistency of the documents; it cannot complete domain review or authorise implementation.

### B1 — Retain a small self-description through the public API

KH-SL-001 supplies the smallest end-to-end path. A versioned bootstrap profile declares `Requirement`, `UseCase` and `UseCase satisfies Requirement`, plus the shared identity/revision, document/provenance and model-scope structures they require. Only the source constraints applicable to this profile are required initially; unsupported types or profile versions are rejected explicitly.

The planned first seed has the following semantic roles. These are mapping labels, not an implemented schema, generated data fixture or a claim that records already exist.

| Seed role | Representation and source | Required meaning |
| --- | --- | --- |
| Library system model | Definition of the `knowledgehub` library; ontology §21.1. | Describes the library being built, separate from the Hub instance that stores it. |
| Atomic-change requirement | One `Requirement`; ontology §21.4. | An admitted change set must preserve a coherent all-or-nothing outcome and original operation result. |
| Commit-change-set use case | One `UseCase`; ontology §§21.3–21.4. | Describes intended behaviour; does not prove a working commit implementation. |
| Satisfaction assertion | One identified `UseCase satisfies Requirement` assertion; ontology §8 and the cited behaviour sections. | Exact endpoint revisions and assertion provenance remain inspectable. |
| Narrative definition | A small Markdown document explaining these records with stable fragment bindings. | Authoritative narrative fields and graph-owned typed bindings are declared separately. |
| Source capture and interpretation | Exact ontology Markdown bytes, SHA-256, meaningful section anchors and authored interpretation. | Source claim and its interpretation are distinguishable; no approval or runtime result is inferred. |

The calling host submits the entire seed under one authorised, guarded branch change. Opening, reading, concurrent rejection, retrying the original identity and recovery all use ordinary public operations. The bootstrap seed must not be installed by directly patching accepted snapshot files.

This is already limited use of KnowledgeHub for its own definition. It is not the complete self-model, a reviewed authority handover, or an exemption from its own consistency contract.

### B2 — Grow the governed self-description progressively

KH-SL-002–005 add coherent authoring, explicit baselines, impact navigation, scoped review and portable exchange. Introduce required profile types as these outcomes need them. Capture both source documents and any retained PDF bytes with independent original-content hashes and explicit representation relationships; similar filenames do not establish byte or semantic equivalence.

Map the authored model through the following families as needed:

| Project definition | Ontology family / type examples |
| --- | --- |
| Purpose, actors and scope | `BusinessGoal`, `DesiredOutcome`, `Actor`, `SystemModel`. |
| Domain language, responsibility and rules | `UbiquitousLanguageTerm`, `BoundedContext`, `AggregateDefinition`, `EntityType`, `Invariant`. |
| Journey and need | `Journey`, `JourneyStep`, `UserActivity`, `UserTask`, `UserStory`, `Requirement`, `Scenario`. |
| Library behaviour and contracts | `UseCase`, `CommandDefinition`, `QueryDefinition`, `FunctionDefinition`, `ErrorDefinition`. |
| Development choices and delivery | `DesignDecision`, `OpenQuestion`, `DeliverySlice`, `ChangeDefinition`, `ImplementationTask`, `ReleaseWorkPackage`. |
| Verification and acceptance | `AcceptanceCriterion`, `TestCase`, `TestRun`, `EvidenceArtifact`, `AcceptanceRecord`, `ReviewDecision`. |

The two supplied sources do not settle every type property or relationship in this mapping. The selected profile must define endpoint, multiplicity, ownership and completeness rules explicitly before it is used. Mark missing coverage as gaps rather than adding generic links and claiming the self-description is complete.

### B3 — Hand over a selected scope and use it for the next change

KH-SL-006 establishes this gate. Import and review a candidate baseline containing the selected library model and exact repository/source bindings. Compare its identities, content, relationships, provenance, reviews and gaps against the bootstrap documents. Export/reconstruct it independently, including the declared recovery history where the backup profile promises it.

Record an explicit handover decision naming:

1. The exact source repository revision and Hub baseline being compared.
2. Which fields, documents and relationships are document-owned or graph-owned.
3. Which earlier editable definitions become references or derived views.
4. The reviewer, current status, unresolved gaps and conditions for reversing or superseding the handover.
5. How to rebuild or restore the Hub, and which representation owns work during recovery.

After this gate, demonstrate one subsequent change: follow a need to its rule and slice, prepare exact behavioural deltas, review them, implement outside the Hub when authorised, attach actual build/test evidence, and record scoped acceptance. A newly stored requirement or a successful import cannot mark its behaviour implemented.

If the Hub is temporarily unavailable, restore the last verified backup or declare an explicit document-authority recovery branch. Reconcile it against the base before re-import. Editing a stale derived export and silently promoting it is outside the write protocol. A broken definition of the library does not change the already compiled recovery implementation.

## Decisions to resolve

All entries are open proposals, not adopted technology or governance decisions. The user owns product/adoption decisions; the library implementer should propose technical selections with supporting evidence for review. No public minimum Rust version or filesystem guarantee is invented here.

| ID | Decision and proposed direction | Consequence / gate |
| --- | --- | --- |
| KH-D-01 | Adopt the initial profile: kernel + `Requirement`, `UseCase` and `satisfies`, with declared property/endpoint rules. | Blocks KH-SL-001 implementation; scope cannot be inferred from arbitrary JSON. |
| KH-D-02 | Assign initial document/graph authority and the recoverable authoring route. Proposed: narrative Markdown fields document-owned, semantic IDs/typed assertions graph-owned. | Blocks the first coherent document/graph write and later handover. |
| KH-D-03 | Select Rust edition/MSRV and async executor policy. Edition 2024 is in the current manifest; MSRV, executor and bounded blocking-I/O policy are undecided. | Blocks final API/dependency and implementation design. |
| KH-D-04 | Specify one initial host/local-filesystem profile, writer coordination, file/directory persistence and recovery protocol. Single-host coordinated writers are the source boundary. | Blocks durable success claims and first file-backend implementation. Process interruption and power-loss evidence must be distinguished. |
| KH-D-05 | Define initial host actor/policy integration. Proposed: trusted composition supplies current scoped authority; no built-in account system, graph role privilege or mandatory human review in the seed. | Blocks admission/read/result-disclosure contracts in KH-SL-001. |
| KH-D-06 | Define bootstrap format/profile versioning, canonical request encoding, source-byte hashing and operation-ledger retention. | Blocks retry/fingerprint and recoverability implementation; API/ontology/file/bundle versions remain separate. |
| KH-D-07 | Define portable query operators, ordering, missing/null, cursors and traversal limits. | Needed for KH-SL-003; exact-ID snapshot reads are the smaller first-slice contract. |
| KH-D-08 | Define review roles, decision outcomes, policy effects and disputed assertions. | Needed for KH-SL-004; committing the seed does not approve its design. |
| KH-D-09 | Define model-exchange and recovery-backup import/export profiles, including identity collisions and replay-domain continuity. | Needed for KH-SL-005 and independent recovery before authority handover. |
| KH-D-10 | Adopt the full selected self-description profile and scoped handover criteria. | Needed for KH-SL-006; automatic total authority migration is not assumed. |

The first-slice gate consists of KH-D-01–06 plus explicit implementation authorisation. Planning artifacts can be complete while those choices remain open. Their resolution must be recorded before apply work begins; no code is generated by this planning set.
