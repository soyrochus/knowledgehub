## Purpose

Make KnowledgeHub itself an inspectable system model expressed through its full
ontology and ordinary governed API, with native evidence and explicit gaps.
Trace: KH-DR-01–19; KH-US-15–17/23/24; ontology §§4–17, 21.10.

## ADDED Requirements

### Requirement: Concrete sourced self-model
A repeatable public consumer SHALL create the KnowledgeHub system model using the records and obligations in `self-model-map.md`. It SHALL cover purpose, actors, requirements, domain language/rules, behaviour contracts, information, architecture, policy, operation/recovery, delivery and verification. Every claim SHALL retain exact source/provenance and its definition/observation status.

#### Scenario: Inspect what the library is and does
- **WHEN** a reader opens the consumer-created self-model
- **THEN** it can identify KnowledgeHub's scope, current supported file profile, host integration, owned responsibilities, commands/queries, consistency rules and actual module/artifact bindings

### Requirement: Ontology and described library remain distinct
The self-model SHALL distinguish the registered modelling vocabulary, definitions of the KnowledgeHub library and observed operation/test occurrences. A modelled aggregate, policy, process or agent SHALL NOT itself become a live implementation object or grant runtime capability.

#### Scenario: Inspect branch consistency responsibility
- **WHEN** a reader inspects the ModelBranch aggregate definition and its proposed root/invariants
- **THEN** the result identifies a system-definition claim and its analysis/review status separately from actual commit objects and Rust implementation references

### Requirement: End-to-end implementation and evidence trace
The self-model SHALL connect the atomic-change requirement to its use case, command, function contracts, responsible component, verification criteria, test specifications and retained observed runs. A reader SHALL retrieve these connections through public typed navigation without inspecting private storage files.

#### Scenario: Explain the original interruption result
- **WHEN** a reader starts from the atomic-change requirement and follows its behaviour, implementation and verification links
- **THEN** it reaches the relevant captured Rust source and actual interruption-run evidence with exact build/source digests and explicit failure-profile limits

### Requirement: Real evidence and explicit applicability
The consumer SHALL capture actual available implementation and run evidence with exact revision/build/environment provenance. It SHALL mark unavailable or stale observations as gaps rather than fabricate outcomes. Inapplicable ontology families SHALL retain explicit reasons in the self-model while remaining supported by the ontology catalogue.

#### Scenario: Existing evidence predates current code
- **WHEN** the previous KH-SL-001 run's captured source hashes differ from the current library build
- **THEN** the self-model retains that historical evidence at its actual scope and requires a new run before claiming the current code passed

#### Scenario: No runtime screen or AI engine exists
- **WHEN** applicability is assessed for this embedded library
- **THEN** no invented product screen, runtime agent or deployment success is added to make the catalogue look populated

### Requirement: Register load revise and reopen through public operations
The walkthrough SHALL register the complete ontology, load the self-model, establish named baseline A, revise one sourced definition coherently, establish B, close/reopen and compare both exact explanations. All writes SHALL use normal admission and original-operation recovery; copying private accepted files SHALL NOT count as the walkthrough.

#### Scenario: Recover both explanations
- **WHEN** the walkthrough reopens after committing the revised requirement/document/assertion selection in B
- **THEN** A still returns the original explanation, B returns its successor, the comparison identifies the exact changes and both original receipts remain resolvable

### Requirement: Report completeness and evidence separately
The walkthrough SHALL produce a source-catalogue coverage result and a separate self-model completeness result. Full ontology support SHALL require verified support for every catalogue entry; self-model completeness SHALL require its applicable obligations. Neither result SHALL imply human acceptance or document-authority handover.

#### Scenario: Self-model is valid but catalogue support is partial
- **WHEN** the self-model passes its selected profile but a declared ontology family has no implemented schema/conformance evidence
- **THEN** the report identifies incomplete ontology support and cannot label this change complete

#### Scenario: Successful walkthrough
- **WHEN** the walkthrough and required checks pass
- **THEN** the evidence records their exact scope while human acceptance and repository-to-Hub authority handover remain explicit separate decisions
