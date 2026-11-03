# Spec Delta

## Purpose

Let a lifecycle identify the exact application definition whose states and transitions it describes, without confusing that definition with its ontology metatype. Trace: MM-SL-01/C2, G6.1-01–04, ontology §§5–7/10/21.4 and KH-DR-01–08/12/15–18.

## ADDED Requirements

### Requirement: Direct exact lifecycle subject
A LifecycleDefinition SHALL have exactly one authoritative subject reference to an EntityType or AggregateDefinition application definition, including valid subtypes. Forward reads SHALL resolve that exact definition directly. An ontology ElementTypeDefinition SHALL NOT substitute for the subject.

#### Scenario: Resolve the actual application subject
- **WHEN** a reader selects a lifecycle describing an application's entity definition
- **THEN** its subject resolves directly to that exact entity definition and reverse navigation identifies the same binding
- **AND** neither an inverse entity property nor an invented shadow record is needed

#### Scenario: Invalid subject binding
- **WHEN** the required subject is absent without a permitted sourced gap, has multiple values, is unresolved, or refers to an ontology metatype or unrelated application kind
- **THEN** assessment identifies the subject/path violation and publication preserves the accepted head

### Requirement: One subject authority across revisions
The direct lifecycle subject SHALL own the binding; inverse views SHALL be derived. A guarded subject/lifecycle edit SHALL retain coherent exact states and transitions. Stale required subject selections SHALL conflict, and historical baselines SHALL retain their original subjects independently of current edits.

#### Scenario: Edit and reopen a lifecycle
- **WHEN** an authorized coherent successor is published after baseline A and the Hub reopens
- **THEN** current reads resolve the successor's exact subject/states while A resolves its original subject/states and transitions
- **AND** the inverse view agrees with the selected direct reference in each baseline

#### Scenario: Subject changed after preparation
- **WHEN** the required subject or selected schema revision changes before a prepared edit is admitted
- **THEN** the original guards reject the stale candidate without silently rebinding its subject

### Requirement: Generic standalone subject navigation
The native API and standalone authoring tools SHALL share the direct-subject contract and finite forward/reverse navigation. The contract SHALL work for unrelated application definitions without entity-name, attribute-name or state-name branches in production schemas or code.

#### Scenario: Two different designs use the same subject contract
- **WHEN** the rich reference fixture and an unrelated-domain fixture publish lifecycles with different application vocabulary
- **THEN** both resolve through the same subject validation and bounded native read paths after reopen
- **AND** the verification result does not depend on installing or invoking Modelarium
