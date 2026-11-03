# Spec Delta

## Purpose

Let developers and analysts use their coding-agent environment to produce and independently verify a source-bound KnowledgeHub model without entering Modelarium. Trace: MM-SL-01/C4, first feature user story, ontology §§4/14/17/21.10 and the practical design manual's slice/review method.

## ADDED Requirements

### Requirement: Portable maintained skills
KnowledgeHub SHALL distribute knowledgehub-import and knowledgehub-verify with reusable modelling references and thin host-specific invocation guidance for supported local Codex, Claude Code and GitHub Copilot execution modes. Core instructions SHALL use the same versioned native tools and contracts without depending on a Modelarium install or its AI module.

#### Scenario: Run in another supported agent host
- **WHEN** an analyst installs the maintained skills in a supported local execution mode
- **THEN** its host can invoke the documented native workflow with explicit source, destination and authority inputs
- **AND** host differences do not create divergent ontology or compiler semantics

#### Scenario: Host cannot execute the native tools
- **WHEN** the selected environment lacks the required local execution capability or current native toolchain
- **THEN** guidance reports the unmet prerequisite and does not claim that an unexecuted or simulated path passed

### Requirement: Source-first reviewed interpretation
The import skill SHALL inventory source authority and scope before modelling, preserve stable source IDs and distinguish original statements from derived interpretations. Material ambiguity SHALL be raised in the external agent conversation or retained as an explicit unresolved decision. Source omissions SHALL NOT be silently repaired with invented domain facts, artifacts or human review.

#### Scenario: Narrative and working UI disagree
- **WHEN** sources give incompatible definitions of a consequential interaction
- **THEN** the workflow records the conflict, exact support and authority decision or unresolved gap before reporting its interpretation as settled

#### Scenario: Discovery was simulated
- **WHEN** supplied research describes simulated rather than actual stakeholder evidence
- **THEN** its captured provenance preserves that status and no imported Interpretation is marked human-approved merely because the agent generated it

### Requirement: Complete typed authoring workflow
The import skill SHALL produce source/authority manifests, a reviewed typed plan and source-specific coverage expectations; invoke native preparation/assessment/publication; and invoke independent verification before reporting completion. It SHALL preserve exact provenance, direct lifecycle subjects, logical targets, subtype semantics, UI bindings and valid gaps rather than emitting only uploaded documents or untyped nodes.

#### Scenario: Analyst requests a full model
- **WHEN** an analyst supplies a rich design package and working UI definition
- **THEN** the skill drives the complete supported path to a persisted native model, baseline, receipts and coverage/gap report
- **AND** any failed assessment, unresolved full-coverage obligation or unexhausted verification remains visible instead of a completion claim

### Requirement: Verification is independent of the generation claim
The verify skill SHALL inspect the persisted exact baseline and retained input/mapping/query manifests with deterministic tooling. It SHALL compare expected source obligations and representative native traces, not trust the generating agent's prose, prior chat state or structural completeness alone. It SHALL distinguish unreviewed interpretation from executed checks and human acceptance.

#### Scenario: Generator says every story was modelled
- **WHEN** native readback finds an expected story, scenario or consequential control missing
- **THEN** verification reports the omission and full-model coverage fails regardless of the generator's narrative or native zero-gap result

### Requirement: Recover rather than restart uncertain publication
Skill instructions SHALL preserve the recorded run and original operation identities, and use native reconciliation when publication is interrupted or its response is lost. They SHALL NOT reset a destination, change operation intent, edit backend files or retry with a new identity merely to obtain apparent success.

#### Scenario: Agent session ends after dispatch
- **WHEN** another authorized session resumes the retained generation run
- **THEN** it resolves the original native outcome before continuing and returns the original receipt without duplicating the model

### Requirement: Truthful standalone handover
A completed skill run SHALL return the native directory, contract/profile, exact head, named baseline, original-source/coverage manifests and generation receipts. It SHALL separately report semantic coverage, native validity/completeness, unresolved interpretation, implementation evidence and human acceptance. Neither imported rules nor described application workflows SHALL execute as part of this handover.

#### Scenario: Handover is opened outside the agent session
- **WHEN** a developer receives only the generated Hub and retained handover artifacts
- **THEN** standalone verification can reopen it and reproduce the reported native/source checks without Modelarium or the original chat
- **AND** later Modelarium registration is an independent MM-SL-02 action
